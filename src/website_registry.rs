use crate::config::AppConfig;
use crate::git::GitRepository;
use crate::website::Website;

#[derive(Clone)]
pub struct WebsiteRegistry {
    pub source_root: String,
    pub docs_root: String,
    pub websites: Vec<Website>,
}

impl WebsiteRegistry {
    pub fn new(config: &AppConfig) -> Self {
        let mut registry = WebsiteRegistry {
            source_root: config.source_root.clone(),
            docs_root: config.docs_root.clone(),
            websites: Vec::new(),
        };
        for website_config in &config.websites {
            let git_repo = GitRepository {
                clone_id: website_config.git.clone_id.clone(),
                branch_name: website_config.git.branch.clone(),
                working_dir: std::path::Path::new(&registry.source_root).join(&website_config.id).display().to_string(),
            };

            let website_processor_root = std::path::Path::new(&registry.source_root).join(&website_config.id).join(&website_config.processor_root);

            let website = Website::new(
                website_config.id.clone(),
                website_config.content_processor.clone(),
                website_processor_root.display().to_string(),
                registry.docs_root.clone(),
                website_config.index,
                git_repo,
            );
            registry.websites.push(website);
        }
        registry
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
        self.websites.iter().for_each(|website| match self.process_website(website) {
            Ok(_) => log::info!("Website {} processed successfully", website.id),
            Err(e) => log::error!("Failed to process website '{}': {}", website.id, e),
        });
        Ok(())
    }
}
