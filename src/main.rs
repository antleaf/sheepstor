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

    let test_website_id = "www.antleaf.com";
    if let Some(website) = registry.get_website(test_website_id) {
        log::debug!("Selected website: {}", website.id);
        log::debug!("Source root for website: {} are at: {}",website.id, website.git_repo.working_dir);
        match website.update_sources() {
            Ok(_) => log::info!("Sources updated for website: {}", website.id),
            Err(e) => log::error!("Failed to update sources for website '{}': {}", website.id, e),
        }
    } else {
        log::debug!("Website with ID '{}' not found", test_website_id);
    }

    match &cli.commands {
        Commands::Server { port } => {
            log::info!("Running Server on port: {}", port);
            let secret = get_secret_from_env(config.github_webhook_secret_env_key).expect("Failed to get secret from env - quitting");
            run_http_server(*port, secret).await;
        }
        Commands::Update { sites } => {
            log::info!("Updating site(s): {}", sites);
        }
        Commands::Scratch {} => {
            log::info!("Running Scratch");
            sheepstor::scratch::scratch().expect("Failed to run scratch - quitting");
        }
    }
    log::info!("Process Completed");
}
