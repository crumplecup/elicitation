//! Path byte-level validation foundation.
//!
//! This module provides validated path byte sequences with platform-specific
//! constraints. It forms the foundation for path contract types.

use super::ValidationError;
use super::utf8::Utf8Bytes;

// ============================================================================
// Unix Path Constraints
// ============================================================================
//
// Valid Unix Paths:
//   - Valid UTF-8 encoding (reuse Utf8Bytes!)
//   - No null bytes (\0)
//   - Any other byte allowed
//   - Path separator: /
//   - Absolute: starts with /
//   - Relative: does not start with /
//
// Special paths:
//   - "/" - root directory
//   - "." - current directory
//   - ".." - parent directory
//   - "~" - home directory (shell expansion, not path itself)

// ============================================================================
// PathBytes (Unix)
// ============================================================================

#[cfg(unix)]
/// A validated Unix path (UTF-8 + no null bytes).
///
/// Generic over maximum length for bounded verification.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathBytes<const MAX_LEN: usize = 4096> {
    utf8: Utf8Bytes<MAX_LEN>,
}

#[cfg(unix)]
impl<const MAX_LEN: usize> PathBytes<MAX_LEN> {
    /// Create from byte slice (Kani-friendly, no Vec allocation).
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::InvalidUtf8` if not valid UTF-8.
    /// Returns `ValidationError::TooLong` if exceeds MAX_LEN.
    /// Returns `ValidationError::PathContainsNull` if contains null bytes.
    pub fn from_slice(bytes: &[u8]) -> Result<Self, ValidationError> {
        let len = bytes.len();

        if len > MAX_LEN {
            return Err(ValidationError::TooLong {
                max: MAX_LEN,
                actual: len,
            });
        }

        // Copy to fixed array (Kani's native domain!)
        let mut fixed = [0u8; MAX_LEN];
        fixed[..len].copy_from_slice(bytes);

        // Validate UTF-8 (reuse existing foundation!)
        let utf8 = Utf8Bytes::new(fixed, len)?;

        // Then check for null bytes
        if has_null_byte(utf8.as_str()) {
            return Err(ValidationError::PathContainsNull);
        }

        Ok(Self { utf8 })
    }

    /// Create from Vec (user-facing API, delegates to from_slice).
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::InvalidUtf8` if not valid UTF-8.
    /// Returns `ValidationError::TooLong` if exceeds MAX_LEN.
    /// Returns `ValidationError::PathContainsNull` if contains null bytes.
    pub fn new(bytes: Vec<u8>) -> Result<Self, ValidationError> {
        Self::from_slice(&bytes)
    }

    /// Get the path as a string slice.
    pub fn as_str(&self) -> &str {
        self.utf8.as_str()
    }

    /// Get the path as bytes.
    pub fn as_bytes(&self) -> &[u8] {
        self.utf8.as_str().as_bytes()
    }

    /// Get the length in bytes.
    pub fn len(&self) -> usize {
        self.utf8.len()
    }

    /// Check if the path is empty.
    pub fn is_empty(&self) -> bool {
        self.utf8.is_empty()
    }

    /// Check if this is an absolute path (starts with /).
    pub fn is_absolute(&self) -> bool {
        is_absolute(self.as_str())
    }

    /// Check if this is a relative path (does not start with /).
    pub fn is_relative(&self) -> bool {
        !self.is_absolute()
    }

    /// Check if this is the root directory (/).
    pub fn is_root(&self) -> bool {
        self.as_str() == "/"
    }

    /// Convert to String.
    pub fn to_string(&self) -> String {
        self.utf8.to_string()
    }
}

// ============================================================================
// Validation Functions
// ============================================================================

/// Check if string contains null bytes.
pub fn has_null_byte(s: &str) -> bool {
    let bytes = s.as_bytes();
    let len = bytes.len();

    // Manual loop with explicit bound to help Kani
    let mut i = 0;
    while i < len {
        if bytes[i] == 0 {
            return true;
        }
        i += 1;
    }
    false
}

/// Check if path is absolute (starts with /).
#[cfg(unix)]
pub fn is_absolute(path: &str) -> bool {
    let bytes = path.as_bytes();
    !bytes.is_empty() && bytes[0] == b'/'
}

/// Check if path is relative (does not start with /).
#[cfg(unix)]
pub fn is_relative(path: &str) -> bool {
    !is_absolute(path)
}

// ============================================================================
// Contract Types
// ============================================================================

#[cfg(unix)]
/// A Unix path guaranteed to be absolute (starts with /).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathAbsolute<const MAX_LEN: usize = 4096>(PathBytes<MAX_LEN>);

#[cfg(unix)]
impl<const MAX_LEN: usize> PathAbsolute<MAX_LEN> {
    /// Create from byte slice (Kani-friendly).
    ///
    /// # Errors
    ///
    /// Returns validation errors from PathBytes, plus:
    /// Returns `ValidationError::PathNotAbsolute` if path doesn't start with /.
    pub fn from_slice(bytes: &[u8]) -> Result<Self, ValidationError> {
        let path = PathBytes::from_slice(bytes)?;

        if !path.is_absolute() {
            return Err(ValidationError::PathNotAbsolute(path.to_string()));
        }

        Ok(Self(path))
    }

    /// Create from Vec (user-facing API).
    ///
    /// # Errors
    ///
    /// Returns validation errors from PathBytes, plus:
    /// Returns `ValidationError::PathNotAbsolute` if path doesn't start with /.
    pub fn new(bytes: Vec<u8>) -> Result<Self, ValidationError> {
        Self::from_slice(&bytes)
    }

    /// Get the underlying PathBytes.
    pub fn get(&self) -> &PathBytes<MAX_LEN> {
        &self.0
    }

    /// Get the path as a string slice.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Unwrap into the underlying PathBytes.
    pub fn into_inner(self) -> PathBytes<MAX_LEN> {
        self.0
    }
}

#[cfg(unix)]
/// A Unix path guaranteed to be relative (does not start with /).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathRelative<const MAX_LEN: usize = 4096>(PathBytes<MAX_LEN>);

#[cfg(unix)]
impl<const MAX_LEN: usize> PathRelative<MAX_LEN> {
    /// Create from byte slice (Kani-friendly).
    ///
    /// # Errors
    ///
    /// Returns validation errors from PathBytes, plus:
    /// Returns `ValidationError::PathNotRelative` if path starts with /.
    pub fn from_slice(bytes: &[u8]) -> Result<Self, ValidationError> {
        let path = PathBytes::from_slice(bytes)?;

        if !path.is_relative() {
            return Err(ValidationError::PathNotRelative(path.to_string()));
        }

        Ok(Self(path))
    }

    /// Create from Vec (user-facing API).
    ///
    /// # Errors
    ///
    /// Returns validation errors from PathBytes, plus:
    /// Returns `ValidationError::PathNotRelative` if path starts with /.
    pub fn new(bytes: Vec<u8>) -> Result<Self, ValidationError> {
        Self::from_slice(&bytes)
    }

    /// Get the underlying PathBytes.
    pub fn get(&self) -> &PathBytes<MAX_LEN> {
        &self.0
    }

    /// Get the path as a string slice.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Unwrap into the underlying PathBytes.
    pub fn into_inner(self) -> PathBytes<MAX_LEN> {
        self.0
    }
}

#[cfg(unix)]
/// A Unix path guaranteed to be non-empty.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathNonEmpty<const MAX_LEN: usize = 4096>(PathBytes<MAX_LEN>);

#[cfg(unix)]
impl<const MAX_LEN: usize> PathNonEmpty<MAX_LEN> {
    /// Create from byte slice (Kani-friendly).
    ///
    /// # Errors
    ///
    /// Returns validation errors from PathBytes, plus:
    /// Returns `ValidationError::EmptyString` if path is empty.
    pub fn from_slice(bytes: &[u8]) -> Result<Self, ValidationError> {
        let path = PathBytes::from_slice(bytes)?;

        if path.is_empty() {
            return Err(ValidationError::EmptyString);
        }

        Ok(Self(path))
    }

    /// Create from Vec (user-facing API).
    ///
    /// # Errors
    ///
    /// Returns validation errors from PathBytes, plus:
    /// Returns `ValidationError::EmptyString` if path is empty.
    pub fn new(bytes: Vec<u8>) -> Result<Self, ValidationError> {
        Self::from_slice(&bytes)
    }

    /// Get the underlying PathBytes.
    pub fn get(&self) -> &PathBytes<MAX_LEN> {
        &self.0
    }

    /// Get the path as a string slice.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Unwrap into the underlying PathBytes.
    pub fn into_inner(self) -> PathBytes<MAX_LEN> {
        self.0
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    #[test]
    fn test_valid_unix_path() {
        let bytes = b"/home/user/file.txt".to_vec();
        let path = PathBytes::<4096>::new(bytes);
        assert!(path.is_ok());
        assert_eq!(path.unwrap().as_str(), "/home/user/file.txt");
    }

    #[test]
    fn test_path_with_null_rejected() {
        let bytes = b"/home/\0user/file.txt".to_vec();
        let path = PathBytes::<4096>::new(bytes);
        assert!(path.is_err());
    }

    #[test]
    fn test_invalid_utf8_rejected() {
        let bytes = vec![0xFF, 0xFE]; // Invalid UTF-8
        let path = PathBytes::<4096>::new(bytes);
        assert!(path.is_err());
    }

    #[test]
    fn test_absolute_path_detection() {
        let bytes = b"/home/user".to_vec();
        let path = PathBytes::<4096>::new(bytes).unwrap();
        assert!(path.is_absolute());
        assert!(!path.is_relative());
    }

    #[test]
    fn test_relative_path_detection() {
        let bytes = b"home/user".to_vec();
        let path = PathBytes::<4096>::new(bytes).unwrap();
        assert!(path.is_relative());
        assert!(!path.is_absolute());
    }

    #[test]
    fn test_root_path() {
        let bytes = b"/".to_vec();
        let path = PathBytes::<4096>::new(bytes).unwrap();
        assert!(path.is_root());
        assert!(path.is_absolute());
    }

    #[test]
    fn test_path_absolute_construction() {
        let bytes = b"/home/user".to_vec();
        let abs = PathAbsolute::<4096>::new(bytes);
        assert!(abs.is_ok());

        let bytes = b"home/user".to_vec();
        let abs = PathAbsolute::<4096>::new(bytes);
        assert!(abs.is_err());
    }

    #[test]
    fn test_path_relative_construction() {
        let bytes = b"home/user".to_vec();
        let rel = PathRelative::<4096>::new(bytes);
        assert!(rel.is_ok());

        let bytes = b"/home/user".to_vec();
        let rel = PathRelative::<4096>::new(bytes);
        assert!(rel.is_err());
    }

    #[test]
    fn test_path_nonempty_construction() {
        let bytes = b"/home".to_vec();
        let nonempty = PathNonEmpty::<4096>::new(bytes);
        assert!(nonempty.is_ok());

        let bytes = b"".to_vec();
        let nonempty = PathNonEmpty::<4096>::new(bytes);
        assert!(nonempty.is_err());
    }

    #[test]
    fn test_special_paths() {
        // Current directory
        let bytes = b".".to_vec();
        let path = PathBytes::<4096>::new(bytes).unwrap();
        assert!(path.is_relative());

        // Parent directory
        let bytes = b"..".to_vec();
        let path = PathBytes::<4096>::new(bytes).unwrap();
        assert!(path.is_relative());

        // Relative with ..
        let bytes = b"../parent".to_vec();
        let path = PathBytes::<4096>::new(bytes).unwrap();
        assert!(path.is_relative());
    }
}
