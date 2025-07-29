use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub source_root: String,
    pub docs_root: String,
    pub github_webhook_secret_env_key: String,
    pub websites: Vec<WebsiteConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct GitRepoConfig {
    pub clone_id: String,
    pub repo_name: String,
    pub branch: String,
}

#[derive(Serialize, Deserialize)]
pub struct WebsiteConfig {
    pub id: String,
    pub content_processor: String,
    pub processor_root: String,
    pub index: bool,
    pub git: GitRepoConfig,
}

impl AppConfig {
    pub fn load(config_file_path: String) -> Result<AppConfig, Box<dyn std::error::Error>> {
        let path = std::path::Path::new(&config_file_path);
        let file = match std::fs::File::open(path) {
            Ok(file) => {
                file
            }
            Err(err) => {
                log::error!("Couldn't open {}: {}", path.display(), err);
                return Err(err.into());
            }
        };
        let config: AppConfig = match serde_yaml::from_reader(file) {
            Ok(config) => {
                log::info!("Loaded config from file at {}",path.display());
                config
            }
            Err(err) => {
                log::error!("Error deserializing YAML from config file at {}: {}",path.display(), err);
                return Err(err.into());
            }
        };
        Ok(config)
    }

    pub fn initialise(&self)-> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(self.source_root.clone())?;
        fs::create_dir_all(self.docs_root.clone())?;
        Ok(())
    }
}