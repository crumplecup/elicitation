//! Error type generators for testing.
//!
//! Sometimes you need to test error handling without triggering actual failures.
//! This module provides Generator implementations for common error types,
//! allowing agents to create mock errors for testing.
//!
//! # Use Case: Testing Error Handlers
//!
//! ```rust,ignore
//! use std::io;
//! use elicitation::{IoErrorGenerationMode, IoErrorGenerator, Generator};
//!
//! // Create an error generator for testing
//! let mode = IoErrorGenerationMode::NotFound("config.toml".to_string());
//! let generator = IoErrorGenerator::new(mode);
//!
//! // Generate error for test
//! let error = generator.generate();
//!
//! // Test your error handler
//! fn handle_error(e: io::Error) -> String {
//!     format!("Error: {}", e)
//! }
//! let result = handle_error(error);
//! assert!(result.contains("config.toml"));
//! ```

use crate::{
    ElicitClient, ElicitError, ElicitErrorKind, ElicitResult, Elicitation, Generator, Prompt,
    Select, mcp,
};
use std::io;

// ============================================================================
// std::io::Error Generator
// ============================================================================

/// Generation mode for std::io::Error.
///
/// Allows creating IO errors for testing without actual IO failures.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IoErrorGenerationMode {
    /// File/directory not found error.
    NotFound(String),
    /// Permission denied error.
    PermissionDenied(String),
    /// Connection refused error.
    ConnectionRefused(String),
    /// Connection reset error.
    ConnectionReset(String),
    /// Broken pipe error.
    BrokenPipe(String),
    /// Already exists error.
    AlreadyExists(String),
    /// Invalid input error.
    InvalidInput(String),
    /// Timeout error.
    TimedOut(String),
    /// Unexpected EOF error.
    UnexpectedEof(String),
    /// Generic "other" error.
    Other(String),
}

impl IoErrorGenerationMode {
    /// Get the error kind for this mode.
    pub fn error_kind(&self) -> io::ErrorKind {
        match self {
            IoErrorGenerationMode::NotFound(_) => io::ErrorKind::NotFound,
            IoErrorGenerationMode::PermissionDenied(_) => io::ErrorKind::PermissionDenied,
            IoErrorGenerationMode::ConnectionRefused(_) => io::ErrorKind::ConnectionRefused,
            IoErrorGenerationMode::ConnectionReset(_) => io::ErrorKind::ConnectionReset,
            IoErrorGenerationMode::BrokenPipe(_) => io::ErrorKind::BrokenPipe,
            IoErrorGenerationMode::AlreadyExists(_) => io::ErrorKind::AlreadyExists,
            IoErrorGenerationMode::InvalidInput(_) => io::ErrorKind::InvalidInput,
            IoErrorGenerationMode::TimedOut(_) => io::ErrorKind::TimedOut,
            IoErrorGenerationMode::UnexpectedEof(_) => io::ErrorKind::UnexpectedEof,
            IoErrorGenerationMode::Other(_) => io::ErrorKind::Other,
        }
    }

    /// Get the message for this mode.
    pub fn message(&self) -> &str {
        match self {
            IoErrorGenerationMode::NotFound(msg)
            | IoErrorGenerationMode::PermissionDenied(msg)
            | IoErrorGenerationMode::ConnectionRefused(msg)
            | IoErrorGenerationMode::ConnectionReset(msg)
            | IoErrorGenerationMode::BrokenPipe(msg)
            | IoErrorGenerationMode::AlreadyExists(msg)
            | IoErrorGenerationMode::InvalidInput(msg)
            | IoErrorGenerationMode::TimedOut(msg)
            | IoErrorGenerationMode::UnexpectedEof(msg)
            | IoErrorGenerationMode::Other(msg) => msg,
        }
    }
}

impl Select for IoErrorGenerationMode {
    fn options() -> &'static [Self] {
        // Can't return non-static Self with String fields
        // Options will be constructed from labels
        &[]
    }

    fn labels() -> &'static [&'static str] {
        &[
            "NotFound",
            "PermissionDenied",
            "ConnectionRefused",
            "ConnectionReset",
            "BrokenPipe",
            "AlreadyExists",
            "InvalidInput",
            "TimedOut",
            "UnexpectedEof",
            "Other",
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        // Message will be elicited separately
        match label {
            "NotFound" => Some(IoErrorGenerationMode::NotFound(String::new())),
            "PermissionDenied" => Some(IoErrorGenerationMode::PermissionDenied(String::new())),
            "ConnectionRefused" => Some(IoErrorGenerationMode::ConnectionRefused(String::new())),
            "ConnectionReset" => Some(IoErrorGenerationMode::ConnectionReset(String::new())),
            "BrokenPipe" => Some(IoErrorGenerationMode::BrokenPipe(String::new())),
            "AlreadyExists" => Some(IoErrorGenerationMode::AlreadyExists(String::new())),
            "InvalidInput" => Some(IoErrorGenerationMode::InvalidInput(String::new())),
            "TimedOut" => Some(IoErrorGenerationMode::TimedOut(String::new())),
            "UnexpectedEof" => Some(IoErrorGenerationMode::UnexpectedEof(String::new())),
            "Other" => Some(IoErrorGenerationMode::Other(String::new())),
            _ => None,
        }
    }
}

crate::default_style!(IoErrorGenerationMode => IoErrorGenerationModeStyle);

impl Prompt for IoErrorGenerationMode {
    fn prompt() -> Option<&'static str> {
        Some("Select the type of IO error to create for testing:")
    }
}

impl Elicitation for IoErrorGenerationMode {
    type Style = IoErrorGenerationModeStyle;

    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Select an option:"),
            Self::labels(),
        );

        let result = client
            .peer()
            .call_tool(rmcp::model::CallToolRequestParams {
                meta: None,
                name: mcp::tool_names::elicit_select().into(),
                arguments: Some(params),
                task: None,
            })
            .await?;

        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;

        let selected = Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(
                "Invalid IO error kind".to_string(),
            ))
        })?;

        // Elicit error message
        let message = String::elicit(client).await?;

        // Create mode with the message
        let mode = match selected {
            IoErrorGenerationMode::NotFound(_) => IoErrorGenerationMode::NotFound(message),
            IoErrorGenerationMode::PermissionDenied(_) => {
                IoErrorGenerationMode::PermissionDenied(message)
            }
            IoErrorGenerationMode::ConnectionRefused(_) => {
                IoErrorGenerationMode::ConnectionRefused(message)
            }
            IoErrorGenerationMode::ConnectionReset(_) => {
                IoErrorGenerationMode::ConnectionReset(message)
            }
            IoErrorGenerationMode::BrokenPipe(_) => IoErrorGenerationMode::BrokenPipe(message),
            IoErrorGenerationMode::AlreadyExists(_) => {
                IoErrorGenerationMode::AlreadyExists(message)
            }
            IoErrorGenerationMode::InvalidInput(_) => IoErrorGenerationMode::InvalidInput(message),
            IoErrorGenerationMode::TimedOut(_) => IoErrorGenerationMode::TimedOut(message),
            IoErrorGenerationMode::UnexpectedEof(_) => {
                IoErrorGenerationMode::UnexpectedEof(message)
            }
            IoErrorGenerationMode::Other(_) => IoErrorGenerationMode::Other(message),
        };

        Ok(mode)
    }
}

/// Generator for creating std::io::Error instances for testing.
///
/// Allows deterministic creation of IO errors without actual IO failures.
#[derive(Debug, Clone)]
pub struct IoErrorGenerator {
    mode: IoErrorGenerationMode,
}

impl IoErrorGenerator {
    /// Create a new IO error generator.
    pub fn new(mode: IoErrorGenerationMode) -> Self {
        Self { mode }
    }

    /// Get the generation mode.
    pub fn mode(&self) -> &IoErrorGenerationMode {
        &self.mode
    }
}

impl Generator for IoErrorGenerator {
    type Target = io::Error;

    fn generate(&self) -> Self::Target {
        io::Error::new(self.mode.error_kind(), self.mode.message())
    }
}

// Elicitation for io::Error itself
crate::default_style!(io::Error => IoErrorStyle);

impl Prompt for io::Error {
    fn prompt() -> Option<&'static str> {
        Some("Create an IO error for testing:")
    }
}

impl Elicitation for io::Error {
    type Style = IoErrorStyle;

    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting io::Error for testing");

        // Elicit generation mode
        let mode = IoErrorGenerationMode::elicit(client).await?;

        // Create generator and generate error
        let generator = IoErrorGenerator::new(mode);
        Ok(generator.generate())
    }
}

// ============================================================================
// serde_json::Error Generator
// ============================================================================

#[cfg(feature = "serde_json")]
mod json_error {
    use super::*;

    /// Generation mode for serde_json::Error.
    ///
    /// Creates real JSON parsing errors by attempting to parse invalid JSON.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum JsonErrorGenerationMode {
        /// Syntax error (invalid JSON).
        SyntaxError,
        /// EOF while parsing (incomplete JSON).
        EofWhileParsing,
        /// Invalid number.
        InvalidNumber,
        /// Invalid escape sequence.
        InvalidEscape,
        /// Invalid Unicode code point.
        InvalidUnicode,
    }

    impl Select for JsonErrorGenerationMode {
        fn options() -> &'static [Self] {
            &[
                JsonErrorGenerationMode::SyntaxError,
                JsonErrorGenerationMode::EofWhileParsing,
                JsonErrorGenerationMode::InvalidNumber,
                JsonErrorGenerationMode::InvalidEscape,
                JsonErrorGenerationMode::InvalidUnicode,
            ]
        }

        fn labels() -> &'static [&'static str] {
            &[
                "Syntax Error",
                "EOF While Parsing",
                "Invalid Number",
                "Invalid Escape",
                "Invalid Unicode",
            ]
        }

        fn from_label(label: &str) -> Option<Self> {
            match label {
                "Syntax Error" => Some(JsonErrorGenerationMode::SyntaxError),
                "EOF While Parsing" => Some(JsonErrorGenerationMode::EofWhileParsing),
                "Invalid Number" => Some(JsonErrorGenerationMode::InvalidNumber),
                "Invalid Escape" => Some(JsonErrorGenerationMode::InvalidEscape),
                "Invalid Unicode" => Some(JsonErrorGenerationMode::InvalidUnicode),
                _ => None,
            }
        }
    }

    crate::default_style!(JsonErrorGenerationMode => JsonErrorGenerationModeStyle);

    impl Prompt for JsonErrorGenerationMode {
        fn prompt() -> Option<&'static str> {
            Some("Select the type of JSON error to create for testing:")
        }
    }

    impl Elicitation for JsonErrorGenerationMode {
        type Style = JsonErrorGenerationModeStyle;

        async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
            let params = mcp::select_params(
                Self::prompt().unwrap_or("Select an option:"),
                Self::labels(),
            );

            let result = client
                .peer()
                .call_tool(rmcp::model::CallToolRequestParams {
                    meta: None,
                    name: mcp::tool_names::elicit_select().into(),
                    arguments: Some(params),
                    task: None,
                })
                .await?;

            let value = mcp::extract_value(result)?;
            let label = mcp::parse_string(value)?;

            Self::from_label(&label).ok_or_else(|| {
                ElicitError::new(ElicitErrorKind::ParseError(
                    "Invalid JSON error kind".to_string(),
                ))
            })
        }
    }

    /// Generator for creating serde_json::Error instances for testing.
    ///
    /// Creates real JSON errors by parsing intentionally invalid JSON.
    #[derive(Debug, Clone, Copy)]
    pub struct JsonErrorGenerator {
        mode: JsonErrorGenerationMode,
    }

    impl JsonErrorGenerator {
        /// Create a new JSON error generator.
        pub fn new(mode: JsonErrorGenerationMode) -> Self {
            Self { mode }
        }

        /// Get the generation mode.
        pub fn mode(&self) -> JsonErrorGenerationMode {
            self.mode
        }
    }

    impl Generator for JsonErrorGenerator {
        type Target = serde_json::Error;

        fn generate(&self) -> Self::Target {
            // Create real JSON errors by parsing invalid JSON
            let invalid_json = match self.mode {
                JsonErrorGenerationMode::SyntaxError => "{invalid}",
                JsonErrorGenerationMode::EofWhileParsing => "{\"key\":",
                JsonErrorGenerationMode::InvalidNumber => "{\"num\": 1e999999}",
                JsonErrorGenerationMode::InvalidEscape => r#"{"str": "\x"}"#,
                JsonErrorGenerationMode::InvalidUnicode => r#"{"str": "\uDEAD"}"#,
            };

            // Parse will fail, giving us a real serde_json::Error
            serde_json::from_str::<serde_json::Value>(invalid_json)
                .expect_err("Invalid JSON should always error")
        }
    }

    // Elicitation for serde_json::Error itself
    crate::default_style!(serde_json::Error => JsonErrorStyle);

    impl Prompt for serde_json::Error {
        fn prompt() -> Option<&'static str> {
            Some("Create a JSON parsing error for testing:")
        }
    }

    impl Elicitation for serde_json::Error {
        type Style = JsonErrorStyle;

        async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
            tracing::debug!("Eliciting serde_json::Error for testing");

            // Elicit generation mode
            let mode = JsonErrorGenerationMode::elicit(client).await?;

            // Create generator and generate error
            let generator = JsonErrorGenerator::new(mode);
            Ok(generator.generate())
        }
    }
}

#[cfg(feature = "serde_json")]
pub use json_error::{JsonErrorGenerationMode, JsonErrorGenerator};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_io_error_generation() {
        let mode = IoErrorGenerationMode::NotFound("config.toml".to_string());
        let generator = IoErrorGenerator::new(mode);
        let error = generator.generate();

        assert_eq!(error.kind(), io::ErrorKind::NotFound);
        assert!(error.to_string().contains("config.toml"));
    }

    #[test]
    fn test_io_error_kinds() {
        let modes = vec![
            IoErrorGenerationMode::PermissionDenied("test".to_string()),
            IoErrorGenerationMode::ConnectionRefused("test".to_string()),
            IoErrorGenerationMode::BrokenPipe("test".to_string()),
        ];

        for mode in modes {
            let generator = IoErrorGenerator::new(mode.clone());
            let error = generator.generate();
            assert_eq!(error.kind(), mode.error_kind());
        }
    }

    #[cfg(feature = "serde_json")]
    #[test]
    fn test_json_error_generation() {
        let mode = JsonErrorGenerationMode::SyntaxError;
        let generator = JsonErrorGenerator::new(mode);
        let error = generator.generate();

        // Error should be a real serde_json::Error with non-empty message
        assert!(!error.to_string().is_empty());
    }

    #[cfg(feature = "serde_json")]
    #[test]
    fn test_json_error_kinds() {
        let modes = vec![
            JsonErrorGenerationMode::SyntaxError,
            JsonErrorGenerationMode::EofWhileParsing,
            JsonErrorGenerationMode::InvalidNumber,
        ];

        for mode in modes {
            let generator = JsonErrorGenerator::new(mode);
            let error = generator.generate();
            // All should produce real errors
            assert!(!error.to_string().is_empty());
        }
    }
}
