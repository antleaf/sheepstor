use crate::auth::validate_github_secret;
use crate::website_registry::WebsiteRegistry;
use axum::extract::{Path, State};
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
struct ApplicationState {
    secret: SecretString,
    registry: WebsiteRegistry,
}

pub type HmacSha256 = Hmac<Sha256>;

pub fn create_router(secret: SecretString, registry: WebsiteRegistry) -> Router {
    let state = ApplicationState { secret, registry };
    Router::new()
        .route("/", get(|| async { "Sheepstor" }))
        .route("/health", get(|| async { "OK" }))
        .route("/update/{website_id}", post(process_github_webhook))
        .with_state(state)
}

pub async fn run_http_server(port: u16, secret: SecretString, registry: WebsiteRegistry) {
    let router = create_router(secret, registry);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await.expect("Failed to bind to port");
    axum::serve(listener, router).await.expect("Failed to start http server");
}

async fn process_github_webhook(State(state): State<ApplicationState>, headers: HeaderMap, Path(website_id): Path<String>, body: String) -> Response {
    match validate_github_secret(state.secret.expose_secret(), headers, body) {
        Ok(_) => {
            log::debug!("Successfully verified signature");
        }
        Err(e) => {
            log::error!("Failed to verify signature: {e}");
            return (StatusCode::UNAUTHORIZED, "Invalid secret").into_response();
        }
    }
    let website = state.registry.get_website_by_id(&website_id);
    match website {
        Some(website) => {
            log::info!("Processing website: {}", website.id);
            match state.registry.process_website(website) {
                Ok(_) => log::info!("Website '{}' updated successfully", website.id),
                Err(e) => log::error!("Failed to update website '{}': {}", website.id, e),
            }
        }
        None => {
            log::warn!("Website with id: {website_id} not found in registry");
        }
    }
    (StatusCode::OK, "OK").into_response()
}
