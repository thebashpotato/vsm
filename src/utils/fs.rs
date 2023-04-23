//! Wrapper around the standard file-system module.

use std::path::{Path, PathBuf};
use std::{fs, io};

use derive_getters::Getters;
use log::debug;
use serde::Serialize;

use crate::config::TomlConfigFile;
use crate::error::VsmRuntimeFault;

/// Provides a simplified constrained interface to locations on disk and actions
/// for directories and files that vsm requires to work.
#[derive(Debug, Getters)]
pub struct FilesystemManager {
    /// Absolute path to configuration file storage directory
    config_dir: String,
    /// Absolute path to configure file
    config_file: String,
    /// Absolute path to session storage directory
    vim_session_dir: String,
}

impl FilesystemManager {
    /// Builds a new FilesystemManager object, relies on the static ENVIRONMENT
    /// state object to get parameters.
    ///
    /// # Arguments
    ///     * config_dir Absolute path to the configuration directory.
    ///     * config_file Absolute path to the configuration file.
    ///     * vim_session_dir Absolute path to the vim sessions directory.
    pub fn new(config_dir: &str, config_file: &str, vim_session_dir: &str) -> Self {
        Self {
            config_dir: String::from(config_dir),
            config_file: String::from(config_file),
            vim_session_dir: String::from(vim_session_dir),
        }
    }

    /// Check if the configuration directory exists on disk.
    ///
    /// # Returns
    ///     * true if the directory exists.
    ///     * false if the path points to a broken symlink,
    ///        or the program does not have proper permissions.
    fn config_dir_exists(&self) -> bool {
        Path::new(self.config_dir()).is_dir()
    }

    /// Check if the configuration file exists on disk.
    ///
    /// # Returns
    ///     * true if the file exists.
    ///     * false if the path points to a broken symlink, or the
    ///        program does not have proper permissions.
    pub fn config_file_exists(&self) -> bool {
        Path::new(self.config_file()).is_file()
    }

    /// Check if the vim_sessions directory exists on disk.
    ///
    /// # Returns
    ///     * true if the directory exists.
    ///     * false if the path points to a broken symlink,
    ///        or the program does not have proper permissions.
    pub fn vim_session_dir_exists(&self) -> bool {
        Path::new(self.vim_session_dir()).is_dir()
    }

    /// Collects all vim session files into a Vector of Path Buffers.
    ///
    /// # Returns
    ///     * Option<Vec<PathBuf>> the vector
    ///       of paths is an option in-case the directory is empty,
    ///       this is how we can tell if we have session files, to list,
    ///       or remove or open.
    ///
    /// # Errors
    ///     * io::Error
    pub fn load_vim_session_files(&self) -> Result<Option<Vec<PathBuf>>, io::Error> {
        // if the sessions directory doesn't exist, create it and all parent directories before
        // it, and return a result of None, since we know there aren't any session files to load from a
        // directory we just created.
        if !self.vim_session_dir_exists() {
            fs::create_dir_all(self.vim_session_dir())?;
            return Ok(None);
        }

        // the directory exists, so lets read it.
        let mut session_files = fs::read_dir(self.vim_session_dir())?
            .map(|res| res.map(|session| session.path()))
            .collect::<Result<Vec<PathBuf>, io::Error>>()?;

        // this gaurds against the case where the directory existed already,
        // but there were no session files found.
        if session_files.is_empty() {
            return Ok(None);
        }

        // Now we know we have sessions.
        // `session_files` is just a vector of PathBuf's loaded from the VIM_SESSIONS directory.
        // We can't be sure that the user hasn't put files other than .vim in there, or
        // created directories, the existence of either would create unwanted bugs when opening
        // sessions with vim variants. Therefore, we only keep paths that are files, and have a "vim" file extension.
        // anything else gets dropped.
        session_files.retain(|path| {
            if path.is_file() {
                path.extension().map_or_else(|| false, |ext| ext == "vim")
            } else {
                false
            }
        });

        session_files.sort();
        Ok(Some(session_files))
    }

    /// Serializes a config structure and writes it to disk as .. Does not check
    /// if the configure file already exists. Assumes it is being used in tandem
    /// with config_file_exists().
    ///
    /// # Arguments
    ///     * config_struct Any type which implements the
    ///       serde::Serialize + Sized traits.
    ///
    /// # Returns
    ///     * Result<()> on success.
    ///
    /// # Errors
    ///     * VsmRuntimeFault::TomlConfigFileWrite captures all
    ///        possible errors during io and serialization.
    pub fn write_config<T>(&self, config_struct: T) -> Result<(), VsmRuntimeFault>
    where
        T: Serialize + Sized,
    {
        if !self.config_dir_exists() {
            if let Err(e) = fs::create_dir_all(self.config_dir()) {
                return Err(VsmRuntimeFault::TomlConfigFileWrite { msg: e.to_string() });
            }
        }
        match toml::to_string(&config_struct) {
            Ok(serialized_string) => {
                debug!("Writing config file => {}", self.config_file());
                if let Err(e) = fs::write(self.config_file(), serialized_string) {
                    Err(VsmRuntimeFault::TomlConfigFileWrite { msg: e.to_string() })
                } else {
                    Ok(())
                }
            }
            Err(e) => Err(VsmRuntimeFault::TomlConfigFileWrite { msg: e.to_string() }),
        }
    }

    /// Deserializes the config.toml from disk.
    ///
    /// # Returns
    ///     * Ok(TomlConfigFile) if nothing went wrong.
    ///
    /// # Errors
    ///     * Err(VsmRuntimeFault::TomlConfigFileRead) containing
    ///        the error message generated from either the serde or toml
    ///        libraries respectively.
    pub fn read_config(&self) -> Result<TomlConfigFile, VsmRuntimeFault> {
        debug!("Reading {}", self.config_file());
        match fs::read_to_string(self.config_file()) {
            Ok(contents) => {
                let mut error_string = String::from("");
                let config_option: Option<TomlConfigFile> = match toml::from_str(&contents) {
                    Ok(c) => Some(c),
                    Err(e) => {
                        error_string = e.to_string();
                        None
                    }
                };
                config_option.map_or(
                    Err(VsmRuntimeFault::TomlConfigFileRead { msg: error_string }),
                    Ok,
                )
            }
            Err(e) => Err(VsmRuntimeFault::TomlConfigFileRead { msg: e.to_string() }),
        }
    }

    /// Wrapper around the built in filesystem remove file function.
    ///
    /// # Errors
    ///     * VsmRuntimeFault::SessionFileRemoval if fs::remove_file fails.
    pub fn remove_file(&self, session: &PathBuf) -> Result<(), VsmRuntimeFault> {
        if let Err(e) = fs::remove_file(session) {
            let msg = format!("Failed to remove {}\n{}", session.to_string_lossy(), e);
            return Err(VsmRuntimeFault::SessionFileRemoval { msg });
        }
        Ok(())
    }
}
