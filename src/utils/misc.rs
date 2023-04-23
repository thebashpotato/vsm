//! Miscellaneous helper functions
use std::path::PathBuf;

/// Takes a vector of Path Buffers, stripes the path down to just the name of
/// the filename with no extension. Returns the filename in a Vector of string.
pub fn extract_filename(sessions: &Vec<PathBuf>) -> Vec<String> {
    let mut cleaned_sessions: Vec<String> = vec![];
    for session in sessions {
        if let Some(file) = session.file_stem() {
            cleaned_sessions.push(file.to_string_lossy().to_string());
        }
    }
    cleaned_sessions
}
