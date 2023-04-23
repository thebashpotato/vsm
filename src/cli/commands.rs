//! Sub-command declarations for the command line wrapper and helper structures

use std::fmt::{Display, Formatter, Result};

use derive_getters::Getters;

/// All accepted sub-commands are defined here
#[derive(Debug)]
pub struct SubCommandName;

impl<'scmd> SubCommandName {
    /// Lists all the current vim session files
    pub const LIST: &'scmd str = "list";
    /// Opens a vim session
    pub const OPEN: &'scmd str = "open";
    /// Removes a vim session
    pub const REMOVE: &'scmd str = "remove";
    /// Changes the users vim variant selection
    pub const VARIANT: &'scmd str = "variant";
}

/// Global optional commands are defined here. Optional commands such as
/// --version, or --debug must be given before sub-commands
#[derive(Debug)]
pub struct OptionalCommandName;

impl<'ocmd> OptionalCommandName {
    /// Runs the app in debug mode. used as Boolean flag
    pub const DEBUG: &'ocmd str = "debug";
}

/// Helps distinguish betwixt arguments that have values, and arguments that
/// don't. At this point in the application, no sub-commands have arguments.
/// Argument exists to future-proof the application.
#[derive(Debug, Clone, Default, Getters)]
pub struct Argument {
    /// The value of the argument
    value: Option<String>,
    /// The name of the argument
    name: Option<String>,
}

impl Display for Argument {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Argument Name: {:?}\nValue: {:?}",
            self.name(),
            self.value()
        )
    }
}

/// Returns the user specified command and the argument structure that goes with
/// it.
#[derive(Debug, Getters)]
pub struct ActiveCommand {
    /// Sub command name
    command: String,
    /// Argument structure
    arg: Argument,
}

impl Display for ActiveCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Active subcommand: {}\n{}", self.command(), self.arg())
    }
}

impl ActiveCommand {
    /// Builds a new ActiveCommand.
    ///
    /// # Arguments
    ///     - command Name of the sub-command.
    ///     - arg Arguments that go along with the command.
    pub fn new(command: &str, arg: Argument) -> Self {
        Self {
            command: String::from(command),
            arg,
        }
    }
}
