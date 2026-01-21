//! PathBuf contract types with runtime filesystem validation.

use crate::{ElicitClient, ElicitResult, Elicitation, Prompt};
use super::ValidationError;
use std::path::PathBuf;

// PathBufExists - Paths that exist on the filesystem
/// A PathBuf that is guaranteed to exist on the filesystem (runtime check).
///
/// **Note:** This is a runtime validation, not compile-time.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathBufExists(PathBuf);

impl PathBufExists {
    /// Create a new PathBufExists, validating the path exists.
    pub fn new(path: PathBuf) -> Result<Self, ValidationError> {
        if path.exists() {
            Ok(Self(path))
        } else {
            Err(ValidationError::PathDoesNotExist(
                path.display().to_string(),
            ))
        }
    }

    /// Get the inner PathBuf.
    pub fn get(&self) -> &PathBuf {
        &self.0
    }

    /// Unwrap into the inner PathBuf.
    pub fn into_inner(self) -> PathBuf {
        self.0
    }
}

impl Prompt for PathBufExists {
    fn prompt() -> Option<&'static str> {
        Some("Please provide a path that exists on the filesystem:")
    }
}

impl Elicitation for PathBufExists {
    type Style = <PathBuf as Elicitation>::Style;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting PathBufExists");
        loop {
            let path = PathBuf::elicit(client).await?;
            match Self::new(path) {
                Ok(valid) => {
                    tracing::debug!(path = ?valid.0, "Valid existing path");
                    return Ok(valid);
                }
                Err(e) => {
                    tracing::warn!(error = %e, "Path does not exist, re-prompting");
                    continue;
                }
            }
        }
    }
}

// PathBufReadable - Paths that are readable
/// A PathBuf that is guaranteed to be readable (runtime check).
///
/// **Note:** This is a runtime validation checking metadata access.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathBufReadable(PathBuf);

impl PathBufReadable {
    /// Create a new PathBufReadable, validating the path is readable.
    pub fn new(path: PathBuf) -> Result<Self, ValidationError> {
        // Try to read metadata as a proxy for readability
        match path.metadata() {
            Ok(_) => Ok(Self(path)),
            Err(_) => Err(ValidationError::PathNotReadable(
                path.display().to_string(),
            )),
        }
    }

    /// Get the inner PathBuf.
    pub fn get(&self) -> &PathBuf {
        &self.0
    }

    /// Unwrap into the inner PathBuf.
    pub fn into_inner(self) -> PathBuf {
        self.0
    }
}

impl Prompt for PathBufReadable {
    fn prompt() -> Option<&'static str> {
        Some("Please provide a readable path:")
    }
}

impl Elicitation for PathBufReadable {
    type Style = <PathBuf as Elicitation>::Style;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting PathBufReadable");
        loop {
            let path = PathBuf::elicit(client).await?;
            match Self::new(path) {
                Ok(valid) => {
                    tracing::debug!(path = ?valid.0, "Valid readable path");
                    return Ok(valid);
                }
                Err(e) => {
                    tracing::warn!(error = %e, "Path not readable, re-prompting");
                    continue;
                }
            }
        }
    }
}

// PathBufIsDir - Paths that are directories
/// A PathBuf that is guaranteed to be a directory (runtime check).
///
/// **Note:** Path must exist for this check to work.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathBufIsDir(PathBuf);

impl PathBufIsDir {
    /// Create a new PathBufIsDir, validating the path is a directory.
    pub fn new(path: PathBuf) -> Result<Self, ValidationError> {
        if path.is_dir() {
            Ok(Self(path))
        } else if path.exists() {
            Err(ValidationError::PathNotDirectory(
                path.display().to_string(),
            ))
        } else {
            Err(ValidationError::PathDoesNotExist(
                path.display().to_string(),
            ))
        }
    }

    /// Get the inner PathBuf.
    pub fn get(&self) -> &PathBuf {
        &self.0
    }

    /// Unwrap into the inner PathBuf.
    pub fn into_inner(self) -> PathBuf {
        self.0
    }
}

impl Prompt for PathBufIsDir {
    fn prompt() -> Option<&'static str> {
        Some("Please provide a directory path:")
    }
}

impl Elicitation for PathBufIsDir {
    type Style = <PathBuf as Elicitation>::Style;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting PathBufIsDir");
        loop {
            let path = PathBuf::elicit(client).await?;
            match Self::new(path) {
                Ok(valid) => {
                    tracing::debug!(path = ?valid.0, "Valid directory path");
                    return Ok(valid);
                }
                Err(e) => {
                    tracing::warn!(error = %e, "Path not directory, re-prompting");
                    continue;
                }
            }
        }
    }
}

// PathBufIsFile - Paths that are files
/// A PathBuf that is guaranteed to be a file (runtime check).
///
/// **Note:** Path must exist for this check to work.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathBufIsFile(PathBuf);

impl PathBufIsFile {
    /// Create a new PathBufIsFile, validating the path is a file.
    pub fn new(path: PathBuf) -> Result<Self, ValidationError> {
        if path.is_file() {
            Ok(Self(path))
        } else if path.exists() {
            Err(ValidationError::PathNotFile(path.display().to_string()))
        } else {
            Err(ValidationError::PathDoesNotExist(
                path.display().to_string(),
            ))
        }
    }

    /// Get the inner PathBuf.
    pub fn get(&self) -> &PathBuf {
        &self.0
    }

    /// Unwrap into the inner PathBuf.
    pub fn into_inner(self) -> PathBuf {
        self.0
    }
}

impl Prompt for PathBufIsFile {
    fn prompt() -> Option<&'static str> {
        Some("Please provide a file path:")
    }
}

impl Elicitation for PathBufIsFile {
    type Style = <PathBuf as Elicitation>::Style;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting PathBufIsFile");
        loop {
            let path = PathBuf::elicit(client).await?;
            match Self::new(path) {
                Ok(valid) => {
                    tracing::debug!(path = ?valid.0, "Valid file path");
                    return Ok(valid);
                }
                Err(e) => {
                    tracing::warn!(error = %e, "Path not file, re-prompting");
                    continue;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::env;

    #[test]
    fn test_path_exists_valid() {
        // Use Cargo.toml (guaranteed to exist in project root)
        let mut path = env::current_dir().unwrap();
        path.push("Cargo.toml");
        let result = PathBufExists::new(path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_path_exists_invalid() {
        let path = PathBuf::from("/this/path/does/not/exist/hopefully");
        let result = PathBufExists::new(path);
        assert!(result.is_err());
    }

    #[test]
    fn test_path_readable_valid() {
        let mut path = env::current_dir().unwrap();
        path.push("Cargo.toml");
        let result = PathBufReadable::new(path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_path_is_dir_valid() {
        // Use src directory
        let mut path = env::current_dir().unwrap();
        path.push("src");
        let result = PathBufIsDir::new(path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_path_is_dir_file() {
        let mut path = env::current_dir().unwrap();
        path.push("Cargo.toml");
        let result = PathBufIsDir::new(path);
        assert!(result.is_err());
    }

    #[test]
    fn test_path_is_file_valid() {
        let mut path = env::current_dir().unwrap();
        path.push("Cargo.toml");
        let result = PathBufIsFile::new(path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_path_is_file_dir() {
        let mut path = env::current_dir().unwrap();
        path.push("src");
        let result = PathBufIsFile::new(path);
        assert!(result.is_err());
    }

    #[test]
    fn test_path_into_inner() {
        let mut original = env::current_dir().unwrap();
        original.push("Cargo.toml");
        let exists = PathBufExists::new(original.clone()).unwrap();
        assert_eq!(exists.into_inner(), original);
    }
}
