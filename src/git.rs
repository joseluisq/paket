use std::fs;
use std::path::{Path, PathBuf};

use crate::helpers;
use crate::result::Result;

pub struct Git {
    pub base_dir: PathBuf,
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

    fn get_remote_endpoint(user_repo_name: &str) -> String {
        // TODO: GitHub support for now
        let mut origin = String::from("https://github.com/");
        origin.push_str(user_repo_name);
        origin.push_str(".git");
        origin
    }

    fn exec_name(&self) -> &'static str {
        "git"
    }

    pub fn clone(&self, user_repo_name: &str, branch_tag: Option<&str>) -> Result<String> {
        let endpoint = Git::get_remote_endpoint(user_repo_name);

        let mut out_dir = self.base_dir.clone();
        out_dir.push(user_repo_name);

        if !out_dir.exists() {
            fs::create_dir_all(&out_dir)?;
        }

        let branch_tag = branch_tag.unwrap_or("master");
        let mut branch_str = String::from("--branch=");
        branch_str.push_str(branch_tag);

        let mut cmd = helpers::cmd::Cmd::new(self.exec_name(), &self.current_dir);
        cmd.arg("clone")
            .arg("--depth=1")
            .arg(branch_str)
            .arg(&endpoint)
            .arg(out_dir);
        cmd.execute()
    }

    pub fn fetch(&mut self, user_repo_name: &str, branch_tag: Option<&str>) -> Result<String> {
        let branch_tag = branch_tag.unwrap_or("master");
        let cwd = self.base_dir.join(user_repo_name).canonicalize()?;

        let mut cmd = helpers::cmd::Cmd::new(self.exec_name(), &cwd);
        cmd.arg("fetch")
            .arg("--depth=1")
            .arg("origin")
            .arg(branch_tag);
        cmd.execute()
    }

    pub fn checkout(&mut self, user_repo_name: &str, branch: Option<&str>) -> Result<String> {
        if branch.is_none() {
            bail!("provide a branch to switch to.");
        }

        let branch = branch.unwrap();
        let cwd = self.base_dir.join(user_repo_name).canonicalize()?;

        let mut cmd = helpers::cmd::Cmd::new(self.exec_name(), &cwd);
        cmd.arg("checkout").arg(branch);
        cmd.execute()
    }

    pub fn pull(&mut self, user_repo_name: &str) -> Result<String> {
        let repo_dir = self.base_dir.join(user_repo_name);
        if !repo_dir.exists() {
            bail!("repository `{}` was not found", user_repo_name);
        }

        let mut cmd = helpers::cmd::Cmd::new(self.exec_name(), &repo_dir);
        cmd.arg("pull").arg("origin").arg("master");
        cmd.execute()
    }
}
