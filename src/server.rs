use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::{
    Router,
    body::{Body, Bytes},
    extract::{Extension, Json, Path, Query, Request},
    http::header::HeaderMap,
    http::{StatusCode, header},
    routing::{get, post},
};
use hmac::{Hmac, Mac};
use secrecy::{ExposeSecret, SecretString};
use sha2::Sha256;

#[derive(Clone)]
struct HookState {
    secret: SecretString,
}

pub type HmacSha256 = Hmac<Sha256>;

pub fn create_router(secret: SecretString) -> Router {
    let state = HookState { secret };
    Router::new().route("/", get(|| async { "Hello, World!" }))
        .route("/health", get(|| async { "OK" }))
        .route("/update", post(post_process_github_webhook))
        .with_state(state)
}

pub async fn run_http_server(port: u16, secret: SecretString) {
    let router = create_router(secret);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn post_process_github_webhook(
    State(state): State<HookState>,
    headers: HeaderMap,
    body: String,
) -> Response {
    if let Some(in_sig) = headers.get("x-hub-signature-256") {
        let signature = in_sig.to_str().unwrap();
        match HmacSha256::new_from_slice(state.secret.expose_secret().as_bytes()) {
            Ok(mut mac) => {
                mac.update(body.as_bytes());
                if let Some(sig_sep) = signature.strip_prefix("sha256=") {
                    match hex::decode(sig_sep) {
                        Ok(decoded) => match mac.verify_slice(&decoded) {
                            Ok(()) => {
                                log::debug!("Successfully verified signature");
                                (StatusCode::OK, "OK").into_response()
                            }
                            Err(_) => {
                                log::error!("Failed to verify signature");
                                (StatusCode::UNAUTHORIZED, "Invalid secret").into_response()
                            }
                        },
                        Err(_) => {
                            log::error!("Couldn't decode secret");
                            (StatusCode::UNAUTHORIZED, "Couldn't decode secret").into_response()
                        }
                    }
                } else {
                    log::error!("could not parse x-hub-signature-256 as str");
                    (
                        StatusCode::BAD_REQUEST,
                        "could not parse x-hub-signature-256 as str",
                    )
                        .into_response()
                }
            }
            Err(e) => {
                log::error!("could not build mac: {e}");
                tracing::error!("could not build mac: {e}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An error occured validating payload",
                )
                    .into_response()
            }
        }
    } else {
        log::error!("missing x-hub-signature-256");
        (StatusCode::UNAUTHORIZED, "missing x-hub-signature-256").into_response()
    }
}
