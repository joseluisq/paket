use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

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

    fn get_current_dir(&self) -> &PathBuf {
        &self.current_dir
    }

    fn set_current_dir(&mut self, dirpath: &PathBuf) {
        self.current_dir = dirpath.to_owned();
    }

    pub fn command(&self) -> Command {
        let mut cmd = Command::new(self.exec_name());
        cmd.current_dir(self.get_current_dir());
        cmd.stdin(Stdio::null());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        cmd
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

        execute_command(
            self.command()
                .arg("clone")
                .arg("--depth=1")
                .arg(branch_str)
                .arg(&endpoint)
                .arg(out_dir),
        )
    }

    pub fn fetch(&mut self, user_repo_name: &str, branch_tag: Option<&str>) -> Result<String> {
        let branch_tag = branch_tag.unwrap_or("master");

        let mut cwd = self.base_dir.clone();
        cwd.push(user_repo_name);
        self.set_current_dir(&cwd);

        execute_command(
            self.command()
                .arg("fetch")
                .arg("--depth=1")
                .arg("origin")
                .arg(branch_tag),
        )
    }

    pub fn checkout(&mut self, user_repo_name: &str, branch: Option<&str>) -> Result<String> {
        if branch.is_none() {
            bail!("provide a branch to switch to.");
        }

        let branch = branch.unwrap();

        let mut cwd = self.base_dir.clone();
        cwd.push(user_repo_name);
        self.set_current_dir(&cwd);

        execute_command(self.command().arg("checkout").arg(branch))
    }

    pub fn pull(&mut self, user_repo_name: &str) -> Result<String> {
        let mut repo_dir = self.base_dir.clone();
        repo_dir.push(user_repo_name);

        if !repo_dir.exists() {
            bail!("repository `{}` was not found", user_repo_name);
        }

        self.set_current_dir(&repo_dir);
        execute_command(self.command().arg("pull").arg("origin").arg("master"))
    }
}

fn execute_command(command: &mut Command) -> Result<String> {
    match command.output() {
        Ok(out) => {
            let success = out.status.success();

            let out = if success { out.stdout } else { out.stderr };
            let res = String::from_utf8(out).map_err(|err| anyhow!(err.to_string()));

            if !success {
                bail!("{}", res?);
            }

            res
        }
        Err(err) => {
            // unexpected error when executing a command
            bail!("{}", err)
        }
    }
}
