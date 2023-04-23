//! Provide a crate wide configuration singleton. As of now, data is sources
//! from environment variables and file paths.

use crate::error::VsmRuntimeFault;
use derive_getters::Getters;
use log::warn;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::{env, fmt};

/// Configuration for env variables
#[derive(Deserialize, Debug, Getters)]
pub struct Variables {
    /// $HOME
    home: String,
    /// $VIM_SESSIONS
    vim_sessions: String,
}

impl fmt::Display for Variables {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Users home directory: {}\nVim session file location: {}",
            self.home(),
            self.vim_sessions()
        )
    }
}

impl Default for Variables {
    fn default() -> Self {
        env::var("HOME").map_or_else(
            |_| Self {
                home: "~/".to_owned(),
                vim_sessions: "~/.config/vim_sessions".to_owned(),
            },
            |h| Self {
                home: h.clone(),
                vim_sessions: format!("{}/.config/vim_sessions", h),
            },
        )
    }
}

/// Holds hard coded paths
#[derive(Deserialize, Debug, Getters)]
pub struct Paths {
    /// Root directory path
    vsm_config_dir: String,
    /// Path including file name
    vsm_config_file: String,
}

impl fmt::Display for Paths {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Vsm config dir: {}\nVsm config file location: {}",
            self.vsm_config_dir(),
            self.vsm_config_file(),
        )
    }
}

impl Default for Paths {
    fn default() -> Self {
        let config_dir = format!("{}/.config/vsm", Variables::default().home());
        Self {
            vsm_config_dir: config_dir.clone(),
            vsm_config_file: format!("{}/config.toml", config_dir),
        }
    }
}

/// The root Environment object that holds all above configuration structs
#[derive(Deserialize, Debug, Default, Getters)]
pub struct Environment {
    /// Environment variable conifg structure
    var: Variables,
    /// Holds all the hard-coded paths
    path: Paths,
}

impl Environment {
    /// Builds a Environment object
    pub fn new() -> Result<Self, VsmRuntimeFault> {
        envy::from_env::<Variables>().map_or_else(
            |_| {
                Err(VsmRuntimeFault::EnvironmentVariable {
                    msg: String::from("VIM_SESSIONS is not defined"),
                })
            },
            |var| {
                Ok(Self {
                    var,
                    path: Paths::default(),
                })
            },
        )
    }
}

/// Public access to parsed configuration.
pub static ENVIRONMENT: Lazy<Environment> = Lazy::new(|| {
    Environment::new().unwrap_or_else(|e| {
        warn!("{}", e);
        warn!("Defaulting to {}", Variables::default().vim_sessions);
        Environment::default()
    })
});

#[cfg(test)]
mod tests {
    use super::{Environment, Variables};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_create_env_config() {
        let env_vars = Variables {
            home: "~/".to_owned(),
            vim_sessions: "~/.config/vim_sessions".to_owned(),
        };
        assert_eq!(env_vars.vim_sessions(), "~/.config/vim_sessions");
        assert_eq!(env_vars.home(), "~/");
    }

    #[test]
    fn test_create_root_config() {
        let home = std::env::var("HOME").expect("Users $HOME is not defined");
        let env = Environment::new().unwrap_or_default();
        assert_eq!(
            env.var().vim_sessions(),
            &format!("{}/.config/vim_sessions", home)
        );
        assert_eq!(env.var().home(), &home);
    }
}
