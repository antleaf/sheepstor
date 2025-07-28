use std::error;
use std::process::Command;
use crate::website_registry::WebsiteRegistry;

// Change the alias to use `Box<dyn error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub fn scratch(registry: WebsiteRegistry) {
    log::debug!("Running Scratch");
    let test_website_id = "www.paulwalk.net";
    if let Some(website) = registry.get_website_by_id(test_website_id) {
        log::debug!("Selected website: {}", website.id);
        log::debug!("Source root for website: {} are at: {}", website.id, website.git_repo.working_dir);
        log::debug!("Webroot for built website: {} is at: {}", website.id, website.webroot);
        log::debug!("website content processor: {:?}", website.content_processor);
        match website.update_sources() {
            Ok(_) => log::info!("Sources updated for website: {}", website.id),
            Err(e) => log::error!("Failed to update sources for website '{}': {}", website.id, e),
        }
        match website.build() {
            Ok(_) => log::info!("Website: {} built", website.id),
            Err(e) => log::error!("Failed to build website '{}': {}", website.id, e),
        }
    } else {
        log::debug!("Website with ID '{}' not found", test_website_id);
    }

    log::debug!("Scratch completed successfully");
}
