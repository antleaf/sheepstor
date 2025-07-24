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
    pub fn new(id: String, cp: String, pr: String, wr: String, index: bool) -> Website {
        let web_root = std::path::Path::new(&wr).join(&id);
        Website {
            id,
            content_processor: cp,
            processor_root: pr,
            webroot: web_root.to_str().unwrap().to_string(),
            index,
        }
    }
}

pub fn load_websites(config: &AppConfig) -> Result<Vec<Website>, Box<dyn std::error::Error>> {
    let mut websites = Vec::new();
    for website_config in &config.websites {
        let website = Website::new(
            website_config.id.clone(),
            website_config.content_processor.clone(),
            website_config.processor_root.clone(),
            website_config.processor_root.clone(),
            website_config.index,
        );
        websites.push(website);
    }
    Ok(websites)
}
