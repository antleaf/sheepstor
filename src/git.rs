
#[derive(Clone)]
pub struct GitRepository {
    pub clone_id: String,
    pub repo_name: String,
    pub branch_name: String,
    pub working_dir: String,
}

impl GitRepository {
    pub fn new(clone_id: String, repo_name: String, branch_name: String, working_dir: String) -> GitRepository {
        GitRepository {
            clone_id,
            repo_name,
            branch_name,
            working_dir,
        }
    }

    pub fn branch_ref(&self) -> String {
        format!("refs/heads/{}", self.branch_name)
    }

    pub fn git_pull(&self) -> Result<(), Box<dyn std::error::Error>> {
        log::debug!("Pulling latest changes for repository {} at branch {}", self.clone_id, self.branch_name);
        let output = std::process::Command::new("git")
            .arg("-C")
            .arg(&self.working_dir)
            .arg("pull")
            .output()?;

        if output.status.success() {
            log::debug!("Repository pulled successfully");
            Ok(())
        } else {
            let error_message = String::from_utf8_lossy(&output.stderr);
            Err(Box::new(std::io::Error::other(error_message)))
        }
    }

    pub fn git_clone(&self) -> Result<(), Box<dyn std::error::Error>> {
        log::debug!("Cloning repository {} at branch {} into {}", self.clone_id, self.branch_name, self.working_dir);
        let output = std::process::Command::new("git")
            .arg("clone")
            .arg("-b")
            .arg(&self.branch_name)
            .arg(&self.clone_id)
            .arg(&self.working_dir)
            .output()?;

        if output.status.success() {
            log::debug!("Repository cloned successfully");
            Ok(())
        } else {
            let error_message = String::from_utf8_lossy(&output.stderr);
            Err(Box::new(std::io::Error::other(error_message)))
        }
    }
}