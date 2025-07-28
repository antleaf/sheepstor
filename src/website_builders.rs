use std::path::PathBuf;
use crate::website::Website;

pub fn build_with_hugo(website: Website,target_folder_for_build: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let hugo_build_command_output = std::process::Command::new("hugo")
        .arg("--quiet")
        .arg("--ignoreCache")
        .arg("--source")
        .arg(&website.processor_root)
        .arg("--destination")
        .arg(target_folder_for_build)
        .output()?;
    if hugo_build_command_output.status.success() {
        Ok(())
    } else {
        let error_message = String::from_utf8_lossy(&hugo_build_command_output.stderr);
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, error_message)))
    }
}

pub fn build_with_verbatim_copy(website: Website,target_folder_for_build: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub fn build_index(website: Website,target_folder_for_build: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let index_build_command_output = std::process::Command::new("pagefind")
        .arg("--site")
        .arg(target_folder_for_build)
        .output()?;
    if index_build_command_output.status.success() {
        Ok(())
    } else {
        let error_message = String::from_utf8_lossy(&index_build_command_output.stderr);
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, error_message)))
    }
}