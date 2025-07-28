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
                website_config.processor_root.clone(),
                website_config.index,
                website_config.git.clone_id.clone(),
                website_config.git.repo_name.clone(),
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
    
    pub fn get_website(&self, id: &str) -> Option<&Website> {
        self.websites.iter().find(|w| w.id == id)
    }

    pub fn get_website_by_repo_name_and_branch_ref(&self, repo_name: String, branch_ref: String)  -> Option<&Website> {
        self.websites.iter().find(|w| w.git_repo.repo_name == repo_name && w.git_repo.branch_ref() == branch_ref)
    }

}