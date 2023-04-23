//! A wrapper around std::process::Command, this module constrains specific
//! shell commands that requires to work. This module makes no attempt to work
//! with Windows. Windows support is planned for the future.

use derive_getters::Getters;
use log::{debug, error};

use crate::error::VsmRuntimeFault;

/// A posix compliant wrapper around std::process
#[derive(Debug, Getters)]
pub struct CommandExecutor {
    /// Holds an instance of the users shell
    user_shell: String,
}

impl Default for CommandExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandExecutor {
    /// Builds a new command executor object based on the users SHELL
    /// environment variable. If it is not defined, shell defaults to the built
    /// in sh shell.
    #[must_use]
    pub fn new() -> Self {
        std::env::var("SHELL").map_or_else(
            |_| Self {
                user_shell: "/bin/sh".to_owned(),
            },
            |user_shell| Self { user_shell },
        )
    }

    /// Uses the POSIX compliant command -v to identify if a program is
    /// installed on the system.
    ///
    /// # Arguments
    ///     * program The name of the installed program
    ///
    /// # Returns
    ///     * true if program is installed
    ///     * false if it is not
    pub fn is_installed(&self, program: &str) -> bool {
        let cmd = format!("command -v {}", program);
        debug!("Executing {}", &cmd);
        let exit_status = std::process::Command::new(&self.user_shell)
            .arg("-c")
            .arg(cmd)
            .stdout(std::process::Stdio::null())
            .status();

        match exit_status {
            Ok(status) => status.success(),
            Err(e) => {
                error!("{}", e.to_string());
                false
            }
        }
    }

    /// Uses the POSIX compliant command -v to identify if a program is
    /// installed on the system.
    ///
    /// # Arguments
    ///     * vim_variant Variant of vim
    ///     * shell_command required shell command the variant needs to open a session file
    ///     * session_file absolute path to the session file to open
    ///
    /// # Errors
    ///     * Consumes all process error messages, and returns a single VsmRuntimeFault error
    pub fn open_editor_with_session(
        &self,
        vim_variant: &String,
        shell_command: &String,
        session_file: &String,
    ) -> Result<(), VsmRuntimeFault> {
        debug!(
            "Executing: {} {} {}",
            vim_variant, shell_command, session_file
        );
        let spawned_process = std::process::Command::new(vim_variant)
            .args(shell_command.split_whitespace())
            .arg(session_file)
            .spawn();

        match spawned_process {
            Ok(mut process) => match process.wait() {
                Ok(_) => Ok(()),
                Err(e) => Err(VsmRuntimeFault::CommandExecutor { msg: e.to_string() }),
            },
            Err(e) => Err(VsmRuntimeFault::CommandExecutor { msg: e.to_string() }),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::CommandExecutor;

    #[test]
    fn test_is_program_installed_with_installed_program() {
        let shell = CommandExecutor::default();
        let programs = [
            "ls", "cat", "less", "more", "man", "mkdir", "cp", "sh", "bash",
        ];
        for program in programs {
            assert_eq!(shell.is_installed(program), true);
        }
    }

    #[test]
    fn test_is_program_installed_with_non_existent_program() {
        let shell = CommandExecutor::new();
        assert_eq!(shell.is_installed("non_existant_program"), false);
    }
}
