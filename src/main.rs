use clap::Parser;
use sheepstor::auth::get_secret_from_env;
use sheepstor::cli::{Cli, Commands};
use sheepstor::logging::configure_flexi_logger;
use sheepstor::server::run_http_server;
use sheepstor::website_registry;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    configure_flexi_logger(cli.global_opts.debug).expect("Failed to configure logger - quitting");
    log::info!("Starting process");
    let config = sheepstor::config::AppConfig::load(cli.global_opts.config).expect("Failed to load configuration - quitting");
    config.initialise().expect("Failed to initialise the configuration - quitting");
    let registry = website_registry::WebsiteRegistry::new(&config);
    log::info!("Loaded {} websites", registry.count());

    match &cli.commands {
        Commands::Server { port } => {
            log::info!("Running Server on port: {}", port);
            let secret = get_secret_from_env(config.github_webhook_secret_env_key).expect("Failed to get secret from env - quitting");
            run_http_server(*port, secret, registry).await;
        }
        Commands::Update { sites } => {
            log::info!("Updating site(s): {}", sites);
            match sites.as_str() {
                "all" => {
                    log::info!("Updating all sites");
                    match registry.process_all_websites() {
                        Ok(_) => log::info!("All websites updated successfully"),
                        Err(e) => log::error!("Failed to update all websites: {}", e),
                    }
                }
                _ => {
                    log::info!("Updating specific site(s): {}", sites);
                    let site_list: Vec<&str> = sites.split(',').collect();
                    for site_id in site_list {
                        let website = registry.get_website_by_id(site_id);
                        match website {
                            Some(website) => {
                                log::info!("Processing website: {}", site_id);
                                match registry.process_website(website.clone()) {
                                    Ok(_) => log::info!("Website '{}' updated successfully", site_id),
                                    Err(e) => log::error!("Failed to update website '{}': {}", site_id, e),
                                }
                            }
                            None => {
                                log::warn!("Website '{}' not found in registry", site_id);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        Commands::Scratch {} => {
            log::info!("Running Scratch");
            sheepstor::scratch::scratch(registry);
        }
    }
    log::info!("Process Completed");
}
