use crate::git::GitRepository;
use crate::website_builders::{build_index, build_with_hugo, build_with_verbatim_copy};
use std::fs;
use std::fs::create_dir_all;
use std::os::unix::fs::symlink;
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Clone, EnumString, Debug)]
pub enum ContentProcessor {
    Unknown,
    Hugo,
    None,
    // Add other content processors as needed e.g.. Jekyll, MkDocs, etc.
}

#[derive(Clone)]
pub struct Website {
    pub id: String,
    pub content_processor: ContentProcessor,
    pub processor_root: String,
    pub webroot: String,
    pub index: bool,
    pub git_repo: GitRepository,
}

impl Website {
    pub fn new(id: String, sr: String, cp: String, pr: String, wr: String, index: bool, clone_id: String, repo_name: String, branch_name: String) -> Website {
        let web_root = std::path::Path::new(&wr).join(&id);
        let source_path = std::path::Path::new(&sr).join(&id);
        let git_repo = GitRepository::new(clone_id, repo_name, branch_name, source_path.display().to_string());
        Website {
            id,
            content_processor: ContentProcessor::from_str(cp.as_str()).unwrap_or(ContentProcessor::Unknown),
            processor_root: source_path.join(pr).display().to_string(),
            webroot: web_root.display().to_string(),
            index,
            git_repo,
        }
    }

    pub fn build(&self) -> Result<(), Box<dyn std::error::Error>> {
        create_dir_all(std::path::Path::new(&self.webroot).join("logs"))?;
        let mut target_folder_for_build = std::path::Path::new(&self.webroot).join("public_1");
        let target_folder_symlink_path = std::path::Path::new(&self.webroot).join("public");
        match fs::read_link(&target_folder_symlink_path) {
            Ok(path) => {
                if path == target_folder_for_build {
                    target_folder_for_build = std::path::Path::new(&self.webroot).join("public_2");
                }
            }
            Err(_e) => {
                log::debug!("No symlink found at: {}, creating new one", &target_folder_symlink_path.display());
            }
        }
        if target_folder_for_build.exists() {
            fs::remove_dir_all(&target_folder_for_build)?;
        }
        create_dir_all(&target_folder_for_build)?;

        match self.content_processor {
            ContentProcessor::Hugo => {
                log::debug!("Building website: {} using Hugo", self.id);
                build_with_hugo(self, &target_folder_for_build)?;
            }
            ContentProcessor::None => {
                log::debug!("Building website: {} without processor (using verbatim copy)", self.id);
                build_with_verbatim_copy(self, &target_folder_for_build)?;
            }
            ContentProcessor::Unknown => {
                log::error!("Unrecognised content processor for website: {}", self.id);
                return Err("Unrecognised content processor for website".into());
            }
        }
        if self.index {
            log::debug!("Building index for website: {}...", self.id);
            build_index(&target_folder_for_build)?;
        }
        if target_folder_symlink_path.exists() {
            fs::remove_file(&target_folder_symlink_path)?;
        }
        symlink(target_folder_for_build, target_folder_symlink_path)?;
        Ok(())
    }

    pub fn update_sources(&self) -> Result<(), Box<dyn std::error::Error>> {
        log::debug!("Updating sources for website: {}", self.id);
        let git_repo_path = std::path::Path::new(&self.git_repo.working_dir).join(".git");
        // Check if the git repository exists, if not, clone it
        if !git_repo_path.exists() {
            log::debug!("Cloning repository for website: {}", self.id);
            self.git_repo.git_clone()?;
            log::debug!("Repository cloned for website: {}", self.id);
            return Ok(());
        }
        self.git_repo.git_pull()?;
        log::debug!("Sources updated for website: {}", self.id);
        Ok(())
    }
}
