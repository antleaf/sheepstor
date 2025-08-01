use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use crate::errors::CustomSheepstorError;
use hmac::{Hmac, Mac};
use secrecy::{ExposeSecret, SecretString};
use sha2::Sha256;
use crate::website_registry::WebsiteRegistry;

pub type HmacSha256 = Hmac<Sha256>;

#[derive(Clone)]
pub struct ApplicationState {
    pub secret: SecretString,
    pub registry: WebsiteRegistry,
}

pub fn validate_github_secret(secret: &str, headers: HeaderMap, payload: String) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(in_sig) = headers.get("x-hub-signature-256") {
        let signature = in_sig.to_str()?;
        match HmacSha256::new_from_slice(secret.as_bytes()) {
            Ok(mut mac) => {
                mac.update(payload.as_bytes());
                if let Some(sig_sep) = signature.strip_prefix("sha256=") {
                    match hex::decode(sig_sep) {
                        Ok(decoded) => {
                            match mac.verify_slice(&decoded) {
                                Ok(()) => Ok(()),
                                Err(e) => Err(Box::new(e)),
                            }
                        },
                        Err(e) => Err(Box::new(e)),
                    }
                } else {
                    Err(Box::new(CustomSheepstorError::new("could not parse x-hub-signature-256 as str")))
                }
            }
            Err(e) => Err(Box::new(e)),
        }
    } else {
        Err(Box::new(CustomSheepstorError::new("missing x-hub-signature-256")))
    }
}

pub async fn process_github_webhook(State(state): State<ApplicationState>, headers: HeaderMap, Path(website_id): Path<String>, body: String) -> Response {
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