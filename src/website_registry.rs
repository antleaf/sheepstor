use crate::config::AppConfig;
use crate::website::Website;



#[derive(Clone)]
pub struct WebsiteRegistry {
    pub websites: Vec<Website>,
}

impl WebsiteRegistry {
    pub fn new(config: &AppConfig) -> Self {
        // Load websites from the configuration
        let mut websites = Vec::new();
        for website_config in &config.websites {
            let website = Website::new(
                website_config.id.clone(),
                config.source_root.clone(),
                website_config.content_processor.clone(),
                website_config.processor_root.clone(),
                config.docs_root.clone(),
                website_config.index,
                website_config.git.clone_id.clone(),
                website_config.git.branch.clone(),
            );
            websites.push(website);
        }
        WebsiteRegistry {
            websites,
        }
    }
    pub fn count(&self) -> u8 {
        self.websites.len() as u8
    }
    
    pub fn get_website_by_id(&self, id: &str) -> Option<&Website> {
        self.websites.iter().find(|w| w.id == id)
    }

    pub fn process_website(&self, website: &Website) -> Result<(), Box<dyn std::error::Error>> {
        log::debug!("Processing website: {}...", website.id);
        website.update_sources()?;
        website.build()?;
        Ok(())
    }

    pub fn process_all_websites(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.websites.iter().for_each(|website| {
            match self.process_website(website) {
                Ok(_) => log::info!("Website {} processed successfully", website.id),
                Err(e) => log::error!("Failed to process website '{}': {}", website.id, e),
            }
        });
        Ok(())
    }

}