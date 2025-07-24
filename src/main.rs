use clap::Parser;
use sheepstor::cli::{Cli, Commands};
use sheepstor::logging::configure_flexi_logger;
use sheepstor::website::load_websites;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        },
        Commands::Update { sites } => {
            log::info!("Updating site(s): {}", sites);
        }
    }
    log::info!("Process Completed");
    Ok(())
}
