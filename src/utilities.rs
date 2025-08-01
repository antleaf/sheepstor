use secrecy::SecretString;
use std::env;
use std::env::VarError;

pub fn get_secret_from_env(key: String) -> Result<SecretString, VarError> {
    match env::var(&key) {
        Ok(value) => Ok(SecretString::from(value)),
        Err(e) => {
            log::error!("Error reading ENV variable with key: {key} - {e}");
            Err(e)
        }
    }
}
