use std::path::PathBuf;
use std::process::{Child, Command as StdCommand, Stdio};

use crate::result::Result;

/// Wrapper around `std::process::Command`
pub struct Command {
    inner: StdCommand,
}

impl Command {
    /// Creates a new instance of `Cmd`
    pub fn new(exec_name: &str, cwd: Option<&PathBuf>) -> Self {
        let mut inner = StdCommand::new(exec_name);

        if let Some(cwd) = cwd {
            inner.current_dir(cwd);
        }

        inner.stdin(Stdio::null());
        inner.stdout(Stdio::piped());
        inner.stderr(Stdio::piped());
        Self { inner }
    }

    /// Adds an argument to pass to the program to execute.
    pub fn arg<S: AsRef<std::ffi::OsStr>>(&mut self, arg: S) -> &mut Self {
        self.inner.arg(arg.as_ref());
        self
    }

    /// Executes a given command with its arguments
    pub fn execute(&mut self) -> Result<String> {
        match self.inner.output() {
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

    /// Executes the command as a child process, returning a handle to it.
    pub fn spawn(&mut self) -> Result<Child> {
        Ok(self.inner.spawn()?)
    }
}
