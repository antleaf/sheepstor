use crate::auth::validate_github_secret;
use crate::website_registry::WebsiteRegistry;
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
        .route("/update", post(post_process_github_webhook))
        .with_state(state)
}

pub async fn run_http_server(port: u16, secret: SecretString, registry: WebsiteRegistry) {
    let router = create_router(secret, registry);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await.expect("Failed to bind to port");
    axum::serve(listener, router).await.expect("Failed to start http server");
}

async fn post_process_github_webhook(State(state): State<ApplicationState>, headers: HeaderMap, body: String) -> Response {
    let json_body: serde_json::Value = match serde_json::from_str(&body) {
        Ok(json) => json,
        Err(e) => {
            log::error!("Failed to parse JSON body: {e}");
            return (StatusCode::BAD_REQUEST, "Invalid JSON").into_response();
        }
    };
    match validate_github_secret(state.secret.expose_secret(), headers, body) {
        Ok(_) => {
            log::debug!("Successfully verified signature");
        }
        Err(e) => {
            log::error!("Failed to verify signature: {e}");
            return (StatusCode::UNAUTHORIZED, "Invalid secret").into_response();
        }
    }
    let repo_name = json_body.get("repository").and_then(|repo| repo.get("full_name")).and_then(|name| name.as_str()).unwrap_or("");
    let branch_ref = json_body.get("ref").and_then(|r| r.as_str()).unwrap_or("");
    let website = state.registry.get_website_by_repo_name_and_branch_ref(String::from(repo_name), String::from(branch_ref));
    match website {
        Some(website) => {
            log::info!("Processing website: {}", website.id);
            match state.registry.process_website(&website) {
                Ok(_) => log::info!("Website '{}' updated successfully", website.id),
                Err(e) => log::error!("Failed to update website '{}': {}", website.id, e),
            }
        }
        None => {
            log::warn!("Website with repo_name: {} and branch_ref:{} not found in registry", String::from(repo_name), String::from(branch_ref));
        }
    }
    (StatusCode::OK, "OK").into_response()
}
