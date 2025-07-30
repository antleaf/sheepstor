use crate::server::HmacSha256;
use axum::http::{HeaderMap};
use hmac::Mac;
use secrecy::SecretString;
use std::env;
use std::env::VarError;
use crate::errors::CustomSheepstorError;

pub fn get_secret_from_env(key: String) -> Result<SecretString, VarError> {
    match env::var(key.clone()) {
        Ok(value) => Ok(SecretString::from(value)),
        Err(e) => {
            log::error!("Error reading ENV variable with key: {key} - {e}");
            Err(e)
        }
    }
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

