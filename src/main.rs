use clap::Parser;
use sheepstor::auth::get_secret_from_env;
use sheepstor::cli::{Cli, Commands};
use sheepstor::logging::configure_flexi_logger;
use sheepstor::server::run_http_server;
use sheepstor::website_registry;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    configure_flexi_logger(cli.global_opts.debug)?;
    log::info!("Starting process");
    let config = sheepstor::config::AppConfig::load(cli.global_opts.config)?;
    config.initialise()?;
    let registry = website_registry::WebsiteRegistry::new(&config);
    log::info!("Loaded {} websites", registry.count());

    let test_website_id = "www.antleaf.com";
    if let Some(website) = registry.get_website(test_website_id) {
        log::debug!("Selected website: {}", website.id);
        log::debug!("Source root for website: {} are at: {}",website.id, website.git_repo.working_dir);
        website.update_sources()?;
    } else {
        log::debug!("Website with ID '{}' not found", test_website_id);
    }

    match &cli.commands {
        Commands::Server { port } => {
            log::info!("Running Server on port: {}", port);
            let secret = get_secret_from_env(config.github_webhook_secret_env_key)?;
            run_http_server(*port, secret).await;
        }
        Commands::Update { sites } => {
            log::info!("Updating site(s): {}", sites);
        }
        Commands::Scratch {} => {
            log::info!("Running Scratch");
            sheepstor::scratch::scratch()?;
        }
    }
    log::info!("Process Completed");
    Ok(())
}
