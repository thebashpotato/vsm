//! This module represents the `vsm's` `config.toml` as a `struct`.

use std::collections::HashMap;

use derive_getters::Getters;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

/// Holds the current supported vim variants, and the associated shell command
/// they require to open a vim session file
pub static SUPPORTED_VIM_VARIATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("vim", "-S");
    m.insert("nvim", "-S");
    m.insert("neovide", "-- -S");
    m.insert("gvim", "-S");
    m
});

/// Holds the users preferred variation of vim, currently defaults to `neovim`.
#[derive(Debug, Deserialize, Serialize, Getters)]
pub struct VimVariant {
    /// Active vim variant
    active_variant: String,
    /// shell command needed to open a vim session file that is specific to the
    /// variant
    shell_command: String,
}

impl Default for VimVariant {
    fn default() -> Self {
        let variant_pair = SUPPORTED_VIM_VARIATIONS
            .get_key_value("nvim")
            .expect("Failed to get vim variant from lazy loaded hashmap");
        Self {
            active_variant: String::from(*variant_pair.0),
            shell_command: String::from(*variant_pair.1),
        }
    }
}

impl VimVariant {
    /// Builds a new Vim Variant
    pub const fn new(active_variant: String, shell_command: String) -> Self {
        Self {
            active_variant,
            shell_command,
        }
    }
}

/// The `struct` is a composition of all above `structs`, this will be populated
/// by the `config.toml`, or written to disk to create the `config.toml`
#[derive(Debug, Default, Deserialize, Serialize, Getters)]
pub struct TomlConfigFile {
    /// Holds above vim variant structure
    vim_variant: VimVariant,
}

impl TomlConfigFile {
    /// Used when no configuration file is found on disk, denoting the first run
    /// of the program, the user is prompted to select their desired vim
    /// variation from a supported versions found installed on the system.
    pub const fn new(vim_variant: VimVariant) -> Self {
        Self { vim_variant }
    }
}
