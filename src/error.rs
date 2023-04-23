//! Provides custom errors using the this error crate

use thiserror::Error;

/// Custom error handling
#[derive(Error, Debug)]
pub enum VsmRuntimeFault {
    /// Used in the `Config` loader, if the user has not defined the
    /// `$VIM_SESSIONS` environment variable
    #[error("Env Error: {msg}")]
    EnvironmentVariable {
        /// custom message
        msg: String,
    },
    /// Used in the utils shell crate. Reports the exact error upon shell
    /// command execution failure.
    #[error("CommandExecutor Error: {msg}")]
    CommandExecutor {
        /// custom message
        msg: String,
    },
    /// Used in the utils::fs crate. Error is used when reading/serializing a
    /// configure toml file fails.
    #[error("Toml Read Error: {msg}")]
    TomlConfigFileRead {
        /// custom message
        msg: String,
    },

    /// Used in the utils::fs crate. Error can be returned if the configuration
    /// file is not found
    #[error("Toml Write Error: {msg}")]
    TomlConfigFileWrite {
        /// custom message
        msg: String,
    },
    /// used in app.rs, Error can be returned if no vim variant that is
    /// supported if found to be installed in the users path.
    #[error("None of the supported vim variants were found on your system => {msg:?}")]
    NoSupportedVimVariantFound {
        /// custom message
        msg: String,
    },
    /// used in utils/ui.rs. Consumes Inquire crate errors
    #[error("Selection failure => {msg}")]
    SelectionFailure {
        /// custom message
        msg: String,
    },
    /// used in utils/fs.rs. Consumes the input output errors,
    ///
    #[error("Failure to delete session => {msg}")]
    SessionFileRemoval {
        /// custom message
        msg: String,
    },
}
