use crate::auth::validate_github_secret3;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::{
    Router,
    http::StatusCode,
    http::header::HeaderMap,
    routing::{get, post},
};
use hmac::Hmac;
use secrecy::{ExposeSecret, SecretString};
use sha2::Sha256;

#[derive(Clone)]
struct HookState {
    secret: SecretString,
}

pub type HmacSha256 = Hmac<Sha256>;

pub fn create_router(secret: SecretString) -> Router {
    let state = HookState { secret };
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health", get(|| async { "OK" }))
        .route("/update", post(post_process_github_webhook))
        .with_state(state)
}

pub async fn run_http_server(port: u16, secret: SecretString) {
    let router = create_router(secret);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn post_process_github_webhook(State(state): State<HookState>, headers: HeaderMap, body: String) -> Response {
    match validate_github_secret3(state.secret.expose_secret(), headers, body) {
        Ok(_) => {
            log::debug!("Successfully verified signature");
        }
        Err(e) => {
            log::error!("Failed to verify signature: {}", e);
            return (StatusCode::UNAUTHORIZED, "Invalid secret").into_response();
        }
    }
    (StatusCode::OK, "OK").into_response()
}
