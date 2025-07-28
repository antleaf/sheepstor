use crate::git::GitRepository;

pub struct Website {
    pub id: String,
    pub content_processor: String,
    pub processor_root: String,
    pub webroot: String,
    pub index: bool,
    pub git_repo: GitRepository,
}

impl Website {
    pub fn new(id: String, sr: String, cp: String, pr: String, wr: String, index: bool,clone_id: String, repo_name: String, branch_name: String) -> Website {
        let web_root = std::path::Path::new(&wr).join(&id);
        let source_path = std::path::Path::new(&sr).join(&id);
        let git_repo = GitRepository::new(
            clone_id,
            repo_name,
            branch_name,
            source_path.to_str().unwrap().to_string(),
        );
        Website {
            id,
            content_processor: cp,
            processor_root: pr,
            webroot: web_root.to_str().unwrap().to_string(),
            index,
            git_repo,
        }
    }

    pub fn update_sources(&self) -> Result<(), Box<dyn std::error::Error>> {
        log::debug!("Updating sources for website: {}", self.id);
        let git_repo_path = std::path::Path::new(&self.git_repo.working_dir).join(".git");
        // Check if the git repository exists, if not, clone it
        if !git_repo_path.exists() {
            log::debug!("Cloning repository for website: {}", self.id);
            self.git_repo.clone()?;
            log::debug!("Repository cloned for website: {}", self.id);
            return Ok(());
        }
        self.git_repo.pull()?;
        log::debug!("Sources updated for website: {}", self.id);
        Ok(())
    }


}

