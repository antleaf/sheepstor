use std::error;
use std::process::Command;

// Change the alias to use `Box<dyn error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub fn scratch() -> Result<()> {
    log::debug!("Running Scratch");
    // let command_git_clone = Command::new("zsh")
    //     .arg("-c")
    //     .arg("git clone git@github.com:antleaf/www.antleaf.com.git /opt/data/sheepstor_rust/data/sources/www.antleaf.com")
    //     .output();
    // match command_git_clone {
    //     Ok(output) => {
    //         if output.status.success() {
    //             log::debug!("Command executed successfully");
    //             Ok(())
    //         } else {
    //             log::error!("Command failed with status: {}", output.status);
    //             Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Command execution failed")))
    //         }
    //     },
    //     Err(e) => {
    //         log::error!("Failed to execute command: {}", e);
    //         Err(Box::new(e))
    //     }
    // }}
    log::debug!("Scratch completed successfully");
    Ok(())
}
