//! Prompt User Interface

use std::path::PathBuf;

use inquire::{MultiSelect, Select};

use super::aesthetic::get_render_config;
use crate::error::VsmRuntimeFault;
use crate::utils::extract_filename;

/// Wrapper around the inquire library
#[derive(Debug)]
pub struct UserPromptRenderer {
    /// Displayed at the bottom of each prompt
    help_message: String,
}

impl UserPromptRenderer {
    /// Initializes the global render theme, and holds prompt information.
    pub fn new() -> Self {
        inquire::set_global_render_config(get_render_config());
        Self {
            help_message: "↑/↓ or k/j to move, enter to select, type to filter".to_owned(),
        }
    }
    /// Presents the user with a single selection list of all installed
    /// variations of vim found on the system.
    ///
    /// # Arguments
    ///     * vim_variations A vector of the installed vim variations that were found on the system.
    ///
    /// # Errors
    ///     * VsmRuntimeFault::SelectionFailure
    pub fn vim_variant(&self, vim_variations: Vec<String>) -> Result<String, VsmRuntimeFault> {
        println!();
        match Select::new("Which variant would you like to use?", vim_variations)
            .with_vim_mode(true)
            .with_help_message(self.help_message.as_str())
            .prompt()
        {
            Ok(choice) => Ok(choice),
            Err(e) => Err(VsmRuntimeFault::SelectionFailure { msg: e.to_string() }),
        }
    }

    /// Presents the user with a single selection list of all vim session files
    /// found at the VIM_SESSIONS directory.
    ///
    /// # Arguments
    ///     * sessions: A collection of stripped session file names.
    ///
    /// # Errors
    ///     * VsmRuntimeFault::SelectionFailure
    pub fn session_open(&self, sessions: &Vec<PathBuf>) -> Result<String, VsmRuntimeFault> {
        println!();
        let cleaned_file_names = extract_filename(sessions);
        match Select::new("Which session would you like to open?", cleaned_file_names)
            .with_vim_mode(true)
            .with_help_message(self.help_message.as_str())
            .prompt()
        {
            Ok(choice) => Ok(choice),
            Err(e) => Err(VsmRuntimeFault::SelectionFailure { msg: e.to_string() }),
        }
    }

    /// Presents the user with a multi-selection list of all vim session files
    /// found at the VIM_SESSIONS directory.
    ///
    /// # Arguments
    ///     * sessions: A collection of stripped session file names.
    ///
    /// # Errors
    ///     * VsmRuntimeFault::SelectionFailure
    pub fn session_remove(&self, sessions: &Vec<PathBuf>) -> Result<Vec<String>, VsmRuntimeFault> {
        println!();
        let cleaned_file_names = extract_filename(sessions);
        match MultiSelect::new(
            "Which session(s) would you like to remove?",
            cleaned_file_names,
        )
        .with_vim_mode(true)
        .with_help_message("↑/↓ or k/j to move, space to select, type to filter")
        .prompt()
        {
            Ok(selected_sessions) => Ok(selected_sessions),
            Err(e) => Err(VsmRuntimeFault::SelectionFailure { msg: e.to_string() }),
        }
    }
}
