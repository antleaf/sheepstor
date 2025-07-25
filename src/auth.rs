use crate::server::HmacSha256;
use axum::http::{HeaderMap};
use hmac::Mac;
use secrecy::SecretString;
use std::env;
use std::env::VarError;

pub fn get_secret_from_env(key: String) -> Result<SecretString, VarError> {
    match env::var(key.clone()) {
        Ok(value) => Ok(SecretString::from(value)),
        Err(e) => {
            log::error!("Error reading ENV variable with key: {} - {}", key, e);
            Err(e)
        }
    }
}

pub fn validate_github_secret3(secret: &str, headers: HeaderMap, payload: String) -> Result<(), String> {
    if let Some(in_sig) = headers.get("x-hub-signature-256") {
        let signature = in_sig.to_str().unwrap();
        match HmacSha256::new_from_slice(secret.as_bytes()) {
            Ok(mut mac) => {
                mac.update(payload.as_bytes());
                if let Some(sig_sep) = signature.strip_prefix("sha256=") {
                    match hex::decode(sig_sep) {
                        Ok(decoded) => match mac.verify_slice(&decoded) {
                            Ok(()) => Ok(()),
                            Err(_) => Err(String::from("Failed to verify signature")),
                        },
                        Err(_) => Err(String::from("Couldn't decode secret")),
                    }
                } else {
                    Err(String::from("could not parse x-hub-signature-256 as str"))
                }
            }
            Err(_) => Err(String::from("could not build mac")),
        }
    } else {
        Err(String::from("missing x-hub-signature-256"))
    }
}

