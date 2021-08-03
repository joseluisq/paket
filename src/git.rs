use std::fs;
use std::path::{Path, PathBuf};

use crate::helpers::Command;
use crate::result::Result;

/// Git commands set interface.
pub struct Git {
    /// Base directory is usually current parent directory.
    pub base_dir: PathBuf,
    /// Current directory when a command is performed.
    pub current_dir: PathBuf,
}

impl Git {
    pub fn new<P: AsRef<Path>>(base_dir: P) -> Result<Self>
    where
        PathBuf: From<P>,
    {
        let base_dir = PathBuf::from(base_dir).canonicalize()?;
        Ok(Self {
            base_dir: base_dir.to_owned(),
            current_dir: base_dir,
        })
    }

    fn get_remote_endpoint(user_repo_name: &str, provider: &str) -> Result<String> {
        let provider = match provider {
            "github" => "github.com",
            "bitbucket" => "bitbucket.org",
            "gitlab" => "gitlab.com",
            _ => bail!(
                "git host provider not supported. use a full url endpoint with `add` command instead"
            ),
        };

        Ok(["https://", provider, "/", user_repo_name, ".git"].concat())
    }

    fn exec_name(&self) -> &'static str {
        "git"
    }

    /// Clone a Git repository.
    pub fn clone(
        &self,
        user_repo_name: &str,
        branch_tag: Option<&str>,
        git_provider: &str,
    ) -> Result<String> {
        let endpoint = Git::get_remote_endpoint(user_repo_name, git_provider)?;

        let out_dir = self.base_dir.join(user_repo_name);
        if !out_dir.exists() {
            fs::create_dir_all(&out_dir)?;
        }

        let branch_tag = branch_tag.unwrap_or("master");
        let branch_str = ["--branch=", branch_tag].concat();

        Command::new(self.exec_name(), Some(&self.current_dir))
            .arg("clone")
            .arg("--depth=1")
            .arg(branch_str)
            .arg(&endpoint)
            .arg(out_dir)
            .execute()
    }

    /// Fetch a Git branch or tag.
    pub fn fetch(&mut self, user_repo_name: &str, branch_tag: Option<&str>) -> Result<String> {
        let branch_tag = branch_tag.unwrap_or("master");
        let cwd = self.base_dir.join(user_repo_name).canonicalize()?;

        Command::new(self.exec_name(), Some(&cwd))
            .arg("fetch")
            .arg("--depth=1")
            .arg("origin")
            .arg(branch_tag)
            .execute()
    }

    /// Checkout to an specific Git branch or tag.
    pub fn checkout(&mut self, user_repo_name: &str, branch: Option<&str>) -> Result<String> {
        if branch.is_none() {
            bail!("provide a branch to switch to.");
        }

        let branch = branch.unwrap();
        let cwd = self.base_dir.join(user_repo_name).canonicalize()?;

        Command::new(self.exec_name(), Some(&cwd))
            .arg("checkout")
            .arg(branch)
            .execute()
    }

    /// Pull Git repository changes.
    pub fn pull(&mut self, user_repo_name: &str) -> Result<String> {
        let repo_dir = self.base_dir.join(user_repo_name);
        if !repo_dir.exists() {
            bail!("repository `{}` was not found", user_repo_name);
        }

        Command::new(self.exec_name(), Some(&repo_dir))
            .arg("pull")
            .arg("origin")
            .arg("master")
            .execute()
    }

    /// Check if given directory is a valid Git repository returning `Ok` if so or an error otherwise.
    pub fn check_valid_repo(&mut self, repo_dir: &Path) -> Result<()> {
        match Command::new(self.exec_name(), Some(&repo_dir.to_path_buf()))
            .arg("rev-parse")
            .arg("--is-inside-work-tree")
            .execute()
        {
            Ok(s) => {
                if s.trim() == "true" {
                    Ok(())
                } else {
                    bail!("package working directory is not inside the repository's work tree")
                }
            }
            Err(err) => {
                bail!(err)
            }
        }
    }
}
