use std::{fs, io};
use std::path::{Path, PathBuf};
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
    let source_path = PathBuf::from(&website.processor_root);
    let dest_path = PathBuf::from(target_folder_for_build);
    copy_dir_all(source_path,dest_path)?;
    Ok(())
}

pub fn build_index(target_folder_for_build: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
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

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}