use std::convert::Infallible;
use std::pin::Pin;
use std::sync::Arc;

use async_stream::stream;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::sse::Event;
use axum::response::{IntoResponse, Response, Sse};
use futures::{Stream, StreamExt};
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

use crate::AppState;
use crate::config::ProviderKind;
use crate::language::Language;
use crate::prompts;
use crate::providers::{self, ChatInput};
use crate::safety;
use crate::session::{Character, ChatTurn, Role};

pub struct ApiError {
    status: StatusCode,
    message: String,
}

impl ApiError {
    fn new(status: StatusCode, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, message)
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, message)
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, message)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.status, Json(json!({ "error": self.message }))).into_response()
    }
}

pub async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

pub async fn get_config(State(state): State<Arc<AppState>>) -> Json<Value> {
    Json(json!({
        "default_provider": state.config.default_provider.as_str(),
        "providers": ["ollama", "cloud", "mock"],
        "characters": [Character::Girl.as_str(), Character::Boy.as_str()],
        "languages": [Language::Ru.as_str(), Language::En.as_str()],
        "models": {
            "ollama": state.config.ollama_model,
            "cloud": state.config.cloud_model
        },
        "cloud_configured": state.config.cloud_api_key.is_some(),
    }))
}

#[derive(Debug, Deserialize)]
pub struct CreateSessionRequest {
    #[serde(default)]
    pub language: Option<Language>,
    #[serde(default)]
    pub character: Option<Character>,
    #[serde(default)]
    pub provider: Option<ProviderKind>,
}

pub async fn create_session(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateSessionRequest>,
) -> Result<Json<Value>, ApiError> {
    let id = Uuid::new_v4().to_string();
    let session = crate::session::Session::new(
        id.clone(),
        payload.language.unwrap_or(Language::Ru),
        payload.character.unwrap_or(Character::Girl),
        payload.provider.unwrap_or(state.config.default_provider),
    );
    let mut sessions = state
        .sessions
        .lock()
        .map_err(|_| ApiError::internal("session store unavailable"))?;
    sessions.insert(id.clone(), session);
    Ok(Json(json!({ "session_id": id })))
}

#[derive(Debug, Deserialize)]
pub struct SessionRequest {
    pub session_id: String,
}

pub async fn clear_history(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SessionRequest>,
) -> Result<Json<Value>, ApiError> {
    let mut sessions = state
        .sessions
        .lock()
        .map_err(|_| ApiError::internal("session store unavailable"))?;
    let session = sessions
        .get_mut(&payload.session_id)
        .ok_or_else(|| ApiError::not_found("session not found"))?;
    session.history.clear();
    Ok(Json(json!({ "cleared": true })))
}

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub session_id: String,
    pub message: String,
}

pub async fn chat(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ChatRequest>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, ApiError> {
    let message = payload.message.trim().to_owned();
    if message.is_empty() {
        return Err(ApiError::bad_request("message must not be empty"));
    }
    if message.chars().count() > 4000 {
        return Err(ApiError::bad_request("message is too long"));
    }

    let (language, system_prompt, history, provider, crisis_reply) = {
        let mut sessions = state
            .sessions
            .lock()
            .map_err(|_| ApiError::internal("session store unavailable"))?;
        let session = sessions
            .get_mut(&payload.session_id)
            .ok_or_else(|| ApiError::not_found("session not found"))?;

        let language = session.language;
        let crisis_reply =
            safety::crisis_reply(Language::detect(&message), &message).map(str::to_owned);
        let history = session
            .history
            .iter()
            .map(|turn| {
                let role = match turn.role {
                    Role::User => "user",
                    Role::Assistant => "assistant",
                };
                (role.to_owned(), turn.content.clone())
            })
            .collect();

        session.history.push(ChatTurn::user(message.clone()));
        trim_history(session, state.config.max_history);

        (
            language,
            prompts::system_prompt(language, session.character),
            history,
            session.provider,
            crisis_reply,
        )
    };

    let reply_stream: Pin<Box<dyn Stream<Item = Result<String, String>> + Send>> =
        if let Some(crisis_text) = crisis_reply {
            providers::text_stream(&crisis_text).boxed()
        } else {
            providers::stream_reply(
                state.client.clone(),
                &state.config,
                provider,
                ChatInput {
                    language,
                    system_prompt,
                    history,
                    user_message: message,
                },
            )
            .boxed()
        };

    let stream_state = Arc::clone(&state);
    let stream_session_id = payload.session_id.clone();
    let stream = stream! {
        let mut full_reply = String::new();
        let mut failed = false;

        for await piece in reply_stream {
            match piece {
                Ok(text) => {
                    full_reply.push_str(&text);
                    yield Ok(Event::default().data(text));
                }
                Err(error) => {
                    failed = true;
                    yield Ok(Event::default().event("error").data(error));
                    break;
                }
            }
        }

        if !failed && !full_reply.trim().is_empty() {
            if let Ok(mut sessions) = stream_state.sessions.lock() {
                if let Some(session) = sessions.get_mut(&stream_session_id) {
                    session.history.push(ChatTurn::assistant(full_reply));
                    trim_history(session, stream_state.config.max_history);
                }
            }
        }

        yield Ok(Event::default().data("[DONE]"));
    };

    Ok(Sse::new(stream))
}

fn trim_history(session: &mut crate::session::Session, max_history: usize) {
    let limit = max_history.saturating_mul(2);
    if session.history.len() > limit {
        session.history.drain(..session.history.len() - limit);
    }
}
