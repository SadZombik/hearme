mod api;
mod config;
mod language;
mod prompts;
mod providers;
mod safety;
mod session;

use std::sync::Arc;

use axum::Router;
use axum::routing::{get, post};
use reqwest::Client;
use tower_http::services::{ServeDir, ServeFile};
use tracing_subscriber::EnvFilter;

use crate::config::Config;
use crate::session::Sessions;

pub struct AppState {
    pub config: Config,
    pub client: Client,
    pub sessions: Sessions,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "hearme=info".into()))
        .init();

    let config = Config::from_env();
    let bind_addr = config.bind_addr.clone();
    tracing::info!(
        "provider={} binding={}",
        config.default_provider.as_str(),
        bind_addr
    );

    let state = Arc::new(AppState {
        config,
        client: Client::builder().build().expect("HTTP client must build"),
        sessions: std::sync::Mutex::new(std::collections::HashMap::new()),
    });

    let static_files = ServeDir::new("static").fallback(ServeFile::new("static/index.html"));
    let app = Router::new()
        .route("/api/health", get(api::health))
        .route("/api/config", get(api::get_config))
        .route("/api/session", post(api::create_session))
        .route("/api/session/clear", post(api::clear_history))
        .route("/api/chat", post(api::chat))
        .fallback_service(static_files)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .expect("address must be bindable");
    tracing::info!("HEARME is listening on http://{bind_addr}");
    axum::serve(listener, app).await.expect("server must run");
}
