use async_stream::try_stream;
use futures::{Stream, StreamExt};
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

use crate::config::{Config, ProviderKind};
use crate::language::Language;

#[derive(Debug, Deserialize, Default)]
struct StreamChunk {
    #[serde(default)]
    choices: Vec<StreamChoice>,
}

#[derive(Debug, Deserialize, Default)]
struct StreamChoice {
    #[serde(default)]
    delta: Delta,
}

#[derive(Debug, Deserialize, Default)]
struct Delta {
    content: Option<String>,
}

pub struct ChatInput {
    pub language: Language,
    pub system_prompt: String,
    pub history: Vec<(String, String)>,
    pub user_message: String,
}

pub fn stream_reply(
    client: Client,
    config: &Config,
    provider: ProviderKind,
    input: ChatInput,
) -> impl Stream<Item = Result<String, String>> + Send + 'static {
    let endpoint = match provider {
        ProviderKind::Ollama => format!(
            "{}/chat/completions",
            config.ollama_base_url.trim_end_matches('/')
        ),
        ProviderKind::Cloud => format!(
            "{}/chat/completions",
            config.cloud_base_url.trim_end_matches('/')
        ),
        ProviderKind::Mock => String::new(),
    };
    let model = match provider {
        ProviderKind::Ollama => config.ollama_model.clone(),
        ProviderKind::Cloud => config.cloud_model.clone(),
        ProviderKind::Mock => "mock".to_owned(),
    };
    let api_key = if provider == ProviderKind::Cloud {
        config.cloud_api_key.clone()
    } else {
        None
    };

    let mut messages = vec![json!({ "role": "system", "content": input.system_prompt })];
    for (role, content) in input.history {
        messages.push(json!({ "role": role, "content": content }));
    }
    messages.push(json!({ "role": "user", "content": input.user_message }));

    let body = json!({
        "model": model,
        "messages": messages,
        "stream": true,
        "temperature": 0.8,
    });
    let mock_text = mock_reply(input.language);

    try_stream! {
        if provider == ProviderKind::Mock {
            for word in mock_text.split_inclusive(' ') {
                yield word.to_owned();
            }
            return;
        }

        if provider == ProviderKind::Cloud && api_key.is_none() {
            Err("Cloud provider selected but CLOUD_API_KEY is not set".to_owned())?;
        }

        let mut request = client.post(&endpoint).json(&body);
        if let Some(key) = api_key.as_deref() {
            request = request.bearer_auth(key);
        }

        let response = request.send().await.map_err(|e| format!("Provider request failed: {e}"))?;
        if !response.status().is_success() {
            let status = response.status();
            let detail = response.text().await.unwrap_or_default();
            Err(format!("Provider returned {status}: {}", truncate(&detail)))?;
            return;
        }

        let mut byte_stream = response.bytes_stream();
        let mut buffer = String::new();
        while let Some(chunk) = byte_stream.next().await {
            let chunk = chunk.map_err(|e| format!("Stream read failed: {e}"))?;
            buffer.push_str(&String::from_utf8_lossy(&chunk));

            while let Some(index) = buffer.find('\n') {
                let line: String = buffer.drain(..=index).collect();
                let Some(data) = line.trim().strip_prefix("data:") else { continue; };
                let data = data.trim();
                if data == "[DONE]" {
                    return;
                }
                let Ok(parsed) = serde_json::from_str::<StreamChunk>(data) else { continue; };
                for choice in parsed.choices {
                    if let Some(text) = choice.delta.content {
                        if !text.is_empty() {
                            yield text;
                        }
                    }
                }
            }
        }
    }
}

pub fn text_stream(text: &str) -> impl Stream<Item = Result<String, String>> + Send + 'static {
    let owned = text.to_owned();
    try_stream! {
        for word in owned.split_inclusive(' ') {
            yield word.to_owned();
        }
    }
}

fn mock_reply(language: Language) -> &'static str {
    match language {
        Language::Ru => "Я рядом. Расскажи, что сейчас тяжелее всего — мы разберёмся понемногу.",
        Language::En => {
            "I'm here. Tell me what feels heaviest right now — we'll take it one step at a time."
        }
    }
}

fn truncate(text: &str) -> String {
    text.chars().take(300).collect()
}
