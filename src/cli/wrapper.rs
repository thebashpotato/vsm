//! A wrapper around the clap library

use std::fmt::{Display, Formatter, Result};

use clap::{
    crate_authors, crate_description, crate_name, crate_version, Arg, ArgAction, ArgMatches,
    Command,
};
use derive_getters::Getters;

use super::commands::{ActiveCommand, Argument, OptionalCommandName, SubCommandName};

/// Wrapper around the clap ArgMatches object
#[derive(Debug, Getters)]
pub struct Cli {
    /// Holds the users passed command
    active_command: ActiveCommand,
    /// Holds optional command line value for debug mode
    debug_mode: bool,
}

impl Display for Cli {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.active_command)
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}

impl Cli {
    /// Builds a new Cli object
    #[must_use]
    pub fn new() -> Self {
        let arg_matches: ArgMatches = Command::new(crate_name!())
            .author(crate_authors!())
            .version(crate_version!())
            .about(crate_description!())
            .subcommand_required(true)
            .arg(
                Arg::new(OptionalCommandName::DEBUG)
                    .required(false)
                    .help("Show debugging messages")
                    .short('d')
                    .long(OptionalCommandName::DEBUG)
                    .action(ArgAction::SetTrue),
            )
            .subcommand(
                Command::new(SubCommandName::LIST)
                    .arg_required_else_help(false)
                    .about("List all available vim session files"),
            )
            .subcommand(
                Command::new(SubCommandName::OPEN)
                    .arg_required_else_help(false)
                    .about("Load a session file"),
            )
            .subcommand(
                Command::new(SubCommandName::REMOVE)
                    .arg_required_else_help(false)
                    .about("Remove a session file"),
            )
            .subcommand(
                Command::new(SubCommandName::VARIANT)
                    .arg_required_else_help(false)
                    .about("Change the variation of vim you want to open sessions with"),
            )
            .get_matches();

        Self::build_active_command(&arg_matches)
    }

    /// Private helper function to build the proper active command.
    ///
    /// # Arguments
    ///     - matches clap::ArgMatches object
    #[allow(clippy::unreachable)]
    fn build_active_command(matches: &ArgMatches) -> Self {
        let active_command: ActiveCommand = match matches.subcommand() {
            Some((SubCommandName::LIST, _)) => {
                ActiveCommand::new(SubCommandName::LIST, Argument::default())
            }
            Some((SubCommandName::OPEN, _)) => {
                ActiveCommand::new(SubCommandName::OPEN, Argument::default())
            }
            Some((SubCommandName::REMOVE, _)) => {
                ActiveCommand::new(SubCommandName::REMOVE, Argument::default())
            }
            Some((SubCommandName::VARIANT, _)) => {
                ActiveCommand::new(SubCommandName::VARIANT, Argument::default())
            }
            _ => unreachable!(),
        };

        Self {
            active_command,
            debug_mode: matches.get_flag(OptionalCommandName::DEBUG),
        }
    }

    /// Returns true if the active sub-command is list
    #[must_use]
    pub fn list(&self) -> bool {
        self.active_command.command() == SubCommandName::LIST
    }

    /// Returns true if the active sub-command is open
    #[must_use]
    pub fn open(&self) -> bool {
        self.active_command.command() == SubCommandName::OPEN
    }

    /// Returns true if the active sub-command is remove
    #[must_use]
    pub fn remove(&self) -> bool {
        self.active_command.command() == SubCommandName::REMOVE
    }

    /// Returns true if the active sub-command is update
    #[must_use]
    pub fn variant(&self) -> bool {
        self.active_command.command() == SubCommandName::VARIANT
    }
}
