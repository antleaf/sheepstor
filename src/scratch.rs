use std::error;

// Change the alias to use `Box<dyn error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub fn scratch() -> Result<()> {
    log::debug!("Running Scratch");

    log::debug!("Scratch completed successfully");
    Ok(())
}
