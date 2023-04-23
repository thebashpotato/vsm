//! Expose public configure interfaces

mod loader;
mod vsm_cf;

pub use loader::ENVIRONMENT;
pub use vsm_cf::{TomlConfigFile, VimVariant, SUPPORTED_VIM_VARIATIONS};
