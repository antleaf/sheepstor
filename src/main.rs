use clap::Parser;
use sheepstor::auth::get_secret_from_env;
use sheepstor::cli::{Cli, Commands};
use sheepstor::logging::configure_flexi_logger;
use sheepstor::website::load_websites;
use sheepstor::server::run_http_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    configure_flexi_logger(cli.global_opts.debug)?;
    log::info!("Starting process");
    let config = sheepstor::config::AppConfig::load(cli.global_opts.config)?;
    config.initialise()?;
    let websites = load_websites(&config)?;
    log::info!("Loaded {} websites", websites.len());

    match &cli.commands {
        Commands::Server { port } => {
            log::info!("Running Server on port: {}", port);
            let secret = get_secret_from_env(config.github_webhook_secret_env_key)?;
            run_http_server(*port, secret).await;
        },
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
