use crate::config::AppConfig;

pub struct Website {
    pub id: String,
    pub content_processor: String,
    pub processor_root: String,
    pub webroot: String,
    pub index: bool,
    // pub git_repo
}

impl Website {
    pub fn new(id:String, config: &AppConfig) -> Result<Website, Box<dyn std::error::Error>> {
        let website_config = config.websites.iter().find(|w| w.id == id);
        if let Some(website_config) = website_config {
            Ok(Website {
                id: website_config.id.clone(),
                content_processor: website_config.content_processor.clone(),
                processor_root: website_config.processor_root.clone(),
                webroot: format!("/var/www/{}", website_config.processor_root),
                index: website_config.index,
            })
        } else {
            Err(format!("Website with id '{}' not found in config", id).into())
        }
    }
}