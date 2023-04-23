//! Logs to standard out

use std::env;

use env_logger::Builder;
use log::LevelFilter;

use super::format;

/// Simple colorized console logger which leverages `env_logger`, and colored.
#[derive(Debug, Default)]
pub struct StdoutLog;

impl StdoutLog {
    /// Builds and initializes a console logger at a specified level
    pub fn init(level: LevelFilter) {
        let mut builder = Builder::new();
        builder.format(format::format);
        builder.filter(None, level);
        if let Ok(rust_log) = env::var("RUST_LOG") {
            builder.parse_filters(&rust_log);
        }
        builder.init();
    }
}
