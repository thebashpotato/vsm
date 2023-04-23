//! Main routine, runs the app.rs code

use log::error;
use std::process::ExitCode;
use vsm::VimSessionManager;

fn main() -> ExitCode {
    if let Err(e) = VimSessionManager::new().run() {
        error!("{}", e);
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
