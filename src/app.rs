//! Consumes all crates to create the application. if you want to know how `vsm`
//! works, read this file.

use crate::cli::Cli;
use crate::config::{TomlConfigFile, VimVariant, ENVIRONMENT, SUPPORTED_VIM_VARIATIONS};
use crate::error::VsmRuntimeFault;
use crate::logger::StdoutLog;
use crate::ui::UserPromptRenderer;
use crate::utils::{CommandExecutor, FilesystemManager};
use log::{debug, error, info, warn, LevelFilter};
use std::path::PathBuf;

/// Wrap the entire execution process into an application object
#[derive(Debug)]
pub struct VimSessionManager {
    /// Holds an internal instance of the Command Line Interface
    cli: Cli,
    /// Holds an internal instance of the configure file structure
    config_file_struct: TomlConfigFile,
    /// Holds the File system manager to write and read configs
    fs: FilesystemManager,
    /// Holds Instance of users shell
    shell: CommandExecutor,
    /// Holds the inquire prompts
    prompt: UserPromptRenderer,
    /// Is this the programs first run
    first_run: bool,
}

impl Default for VimSessionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl VimSessionManager {
    /// Builds the application
    #[must_use]
    pub fn new() -> Self {
        let cli = Cli::new();
        // Set up the stdout logger eagerly before main components
        // are loaded so we can have logging.
        let level_filter: LevelFilter = if *cli.debug_mode() {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        };
        StdoutLog::init(level_filter);

        Self {
            cli,
            config_file_struct: TomlConfigFile::default(),
            fs: FilesystemManager::new(
                ENVIRONMENT.path().vsm_config_dir(),
                ENVIRONMENT.path().vsm_config_file(),
                ENVIRONMENT.var().vim_sessions(),
            ),
            shell: CommandExecutor::new(),
            prompt: UserPromptRenderer::new(),
            first_run: true,
        }
    }

    /// Run the application. All errors that are propagated to this level are
    /// considered un-recoverable.
    ///
    /// # Errors
    ///     - VsmRuntimeFault variations.
    pub fn run(&mut self) -> Result<(), VsmRuntimeFault> {
        self.setup()?;
        self.subcommand_dispatcher()?;
        Ok(())
    }

    /// Perform the application set up process, calls all otherwise setup
    /// functions
    ///
    /// # Errors
    ///     - VsmRuntimeFault.
    fn setup(&mut self) -> Result<(), VsmRuntimeFault> {
        // Set up disk configuration
        if self.fs.config_file_exists() {
            // The config file was found on disk, we load it into the config struct
            self.first_run = false;
            self.config_file_struct = self.fs.read_config()?;
        } else {
            // This is the first run of the program, so prompt the user.
            warn!("No config file detected");
            self.select_vim_variation()?
        }
        Ok(())
    }

    /// Allows the user to select their preferred variant of vim, writes the
    /// users selection to disk to be used for all other sessions. Called if
    /// there is no configuration file found, or if the user calls the update
    /// sub-command.
    ///
    /// # Errors
    ///     - VsmRuntimeFault.
    fn select_vim_variation(&mut self) -> Result<(), VsmRuntimeFault> {
        let mut installed_variations: Vec<String> = vec![];
        let mut variants_not_installed_error_msg = String::new();
        for variant in SUPPORTED_VIM_VARIATIONS.keys() {
            if self.shell.is_installed(variant) {
                installed_variations.push((*variant).to_owned());
            } else {
                // build a nice error string in the case that installed_variations is empty
                variants_not_installed_error_msg.push_str(format!("{}, ", variant).as_str());
            }
        }

        // The user does not have any of the supported variants of vim installed or in the $PATH
        if installed_variations.is_empty() {
            return Err(VsmRuntimeFault::NoSupportedVimVariantFound {
                msg: variants_not_installed_error_msg,
            });
        }

        // If this isn't the first run of the program, show the user their current
        // active vim variant.
        if !self.first_run {
            info!(
                "Current active variant is => {}",
                self.config_file_struct.vim_variant().active_variant()
            );
        }

        // Show the prompt of available vim variants for user selection.
        // Update the config struct and save it to disk.
        match self.prompt.vim_variant(installed_variations) {
            Ok(choice) => {
                // Small optimization, if the user selects the same variant as they already
                // have, we won't bother updating and serializing the new selection to disk.
                if self.first_run
                    || *self.config_file_struct.vim_variant().active_variant() != choice
                {
                    let shell_command = SUPPORTED_VIM_VARIATIONS
                        .get(choice.as_str())
                        .expect("Failed to retrieve shell_command value from Lazy loaded hashmap");
                    self.config_file_struct =
                        TomlConfigFile::new(VimVariant::new(choice, String::from(*shell_command)));
                    self.fs.write_config(&self.config_file_struct)?;
                }
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /// Executes the proper code based on which sub-command was used
    fn subcommand_dispatcher(&mut self) -> Result<(), VsmRuntimeFault> {
        if !self.cli.variant() {
            match self.fs.load_vim_session_files() {
                Ok(maybe_empty_sessions) => maybe_empty_sessions.map_or_else(
                    || warn!("No session files found"),
                    |sessions| {
                        if self.cli.list() {
                            self.list(&sessions);
                        }
                        if self.cli.open() {
                            if let Err(e) = self.open(&sessions) {
                                error!("{}", e);
                            }
                        }
                        if self.cli.remove() {
                            if let Err(e) = self.remove(&sessions) {
                                error!("{}", e);
                            }
                        }
                    },
                ),
                Err(e) => error!("{}", e.to_string()),
            }
        } else if !self.first_run {
            self.variant()?;
        }
        Ok(())
    }

    /// Executes sub-command list
    fn list(&mut self, sessions: &Vec<PathBuf>) {
        debug!("Listing all sessions");
        for session in sessions {
            if let Some(file) = session.file_stem() {
                info!("{}", file.to_string_lossy());
            }
        }
    }

    /// Executes sub-command open
    fn open(&mut self, sessions: &Vec<PathBuf>) -> Result<(), VsmRuntimeFault> {
        debug!("Opening a session");
        match self.prompt.session_open(sessions) {
            Ok(choice) => {
                for session in sessions {
                    if let Some(file) = session.file_stem() {
                        if choice == file.to_string_lossy() {
                            self.shell.open_editor_with_session(
                                self.config_file_struct.vim_variant().active_variant(),
                                self.config_file_struct.vim_variant().shell_command(),
                                &session.to_string_lossy().to_string(),
                            )?;
                        }
                    }
                }
            }
            Err(e) => return Err(e),
        }
        Ok(())
    }

    /// Executes sub-command remove
    fn remove(&mut self, sessions: &Vec<PathBuf>) -> Result<(), VsmRuntimeFault> {
        debug!("Removing stale sessions");
        match self.prompt.session_remove(sessions) {
            Ok(selected_sessions) => {
                // TODO: Optimize, this is O(n^2)
                for session in sessions {
                    for selected in &selected_sessions {
                        if let Some(s) = session.file_stem() {
                            if s.to_string_lossy() == *selected {
                                info!("Removing => {}", s.to_string_lossy());
                                self.fs.remove_file(session)?
                            }
                        }
                    }
                }
            }
            Err(e) => return Err(e),
        }

        Ok(())
    }

    /// Executes sub-command update
    fn variant(&mut self) -> Result<(), VsmRuntimeFault> {
        debug!("Updating users vim variant selection");
        self.select_vim_variation()?;
        info!("Succesfully updated the active vim variant");
        Ok(())
    }
}
