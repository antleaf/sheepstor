// mod cli;
// mod logging;

use clap::Parser;
use sheepstor::cli::Args;
use sheepstor::logging::configure_flexi_logger;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    configure_flexi_logger(args.debug)?;

    log::info!("Starting Process");
    log::info!("Process Completed");
    Ok(())
}
