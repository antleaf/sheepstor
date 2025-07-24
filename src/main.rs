use clap::Parser;
use sheepstor::cli::{Cli, Commands};
use sheepstor::logging::configure_flexi_logger;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    configure_flexi_logger(cli.global_opts.debug)?;
    log::info!("Starting Process");

    match &cli.command {
        Commands::Server { port } => {
            log::info!("Running Server on port: {}...", port);
        },
        Commands::Update { sites } => {
            log::info!("Updating site(s): {}...", sites);
        }
    }
    log::info!("Process Completed");
    Ok(())
}
