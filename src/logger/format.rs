//! Console colors
use std::io::{Error, Write};

use colored::Colorize;
use env_logger::fmt::Formatter;
use log::{Level, Record};

/// Colorizes a message with a color based on the level
fn level_color(level: log::Level, msg: &str) -> String {
    match level {
        Level::Error => msg.bright_red(),
        Level::Warn => msg.bright_yellow(),
        Level::Info => msg.bright_green(),
        Level::Debug => msg.bright_blue(),
        Level::Trace => msg.bright_magenta(),
    }
    .bold()
    .to_string()
}

/// Returns a single token which represents a logging level
const fn level_token(level: Level) -> &'static str {
    match level {
        Level::Error => "x",
        Level::Warn => "!",
        Level::Info => "✓",
        Level::Debug => "D",
        Level::Trace => "T",
    }
}

/// Prefix colored braces and colored level token
fn prefix_token(level: Level) -> String {
    format!(
        "{}{}{}",
        "[".bright_white().bold(),
        level_color(level, level_token(level)),
        "]".bright_white().bold()
    )
}

/// Format the entire log message
pub fn format(buf: &mut Formatter, record: &Record<'_>) -> Result<(), Error> {
    let sep = format!("\n{} ", " | ".white().bold());
    writeln!(
        buf,
        "{} {}",
        prefix_token(record.level()),
        format!("{}", record.args()).replace('\n', &sep),
    )
}
