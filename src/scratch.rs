use crate::website_registry::WebsiteRegistry;

pub fn scratch(registry: WebsiteRegistry) {
    log::debug!("Running Scratch");
    log::debug!("Processing registry with {} websites", registry.count());
    log::debug!("Scratch completed successfully");
}
