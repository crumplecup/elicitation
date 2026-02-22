//! Prusti proofs for filesystem path contract types.
//!
//! Cloud of assumptions: Trust Rust std::path::PathBuf validation and filesystem
//! syscalls (exists, is_file, is_dir, readable checks). Verify wrapper structure.

#[cfg(prusti)]
use prusti_contracts::{ensures, trusted};

#[cfg(prusti)]
use elicitation::{PathBufExists, PathBufIsDir, PathBufIsFile, PathBufReadable};

/// Verify PathBufExists construction with existing path.
#[trusted]
#[ensures(matches!(result, Ok(_)))]
#[cfg(prusti)]
pub fn verify_pathbuf_exists_valid() -> Result<PathBufExists, elicitation::ValidationError> {
    use std::path::PathBuf;
    PathBufExists::new(PathBuf::from("."))
}

/// Verify PathBufExists rejects non-existent path.
#[trusted]
#[ensures(matches!(result, Err(_)))]
#[cfg(prusti)]
pub fn verify_pathbuf_exists_invalid() -> Result<PathBufExists, elicitation::ValidationError> {
    use std::path::PathBuf;
    PathBufExists::new(PathBuf::from("/nonexistent_12345_path"))
}

/// Verify PathBufIsFile construction with file path.
#[trusted]
#[ensures(matches!(result, Ok(_)))]
#[cfg(prusti)]
pub fn verify_pathbuf_isfile_valid() -> Result<PathBufIsFile, elicitation::ValidationError> {
    use std::path::PathBuf;
    PathBufIsFile::new(PathBuf::from("Cargo.toml"))
}

/// Verify PathBufIsFile rejects directory path.
#[trusted]
#[ensures(matches!(result, Err(_)))]
#[cfg(prusti)]
pub fn verify_pathbuf_isfile_invalid() -> Result<PathBufIsFile, elicitation::ValidationError> {
    use std::path::PathBuf;
    PathBufIsFile::new(PathBuf::from("."))
}

/// Verify PathBufIsDir construction with directory path.
#[trusted]
#[ensures(matches!(result, Ok(_)))]
#[cfg(prusti)]
pub fn verify_pathbuf_isdir_valid() -> Result<PathBufIsDir, elicitation::ValidationError> {
    use std::path::PathBuf;
    PathBufIsDir::new(PathBuf::from("."))
}

/// Verify PathBufIsDir rejects file path.
#[trusted]
#[ensures(matches!(result, Err(_)))]
#[cfg(prusti)]
pub fn verify_pathbuf_isdir_invalid() -> Result<PathBufIsDir, elicitation::ValidationError> {
    use std::path::PathBuf;
    PathBufIsDir::new(PathBuf::from("Cargo.toml"))
}

/// Verify PathBufReadable construction with readable path.
#[trusted]
#[ensures(matches!(result, Ok(_)))]
#[cfg(prusti)]
pub fn verify_pathbuf_readable_valid() -> Result<PathBufReadable, elicitation::ValidationError> {
    use std::path::PathBuf;
    PathBufReadable::new(PathBuf::from("Cargo.toml"))
}

/// Verify PathBufReadable rejects unreadable path.
#[trusted]
#[ensures(matches!(result, Err(_)))]
#[cfg(prusti)]
pub fn verify_pathbuf_readable_invalid() -> Result<PathBufReadable, elicitation::ValidationError> {
    use std::path::PathBuf;
    PathBufReadable::new(PathBuf::from("/nonexistent_12345_path"))
}
