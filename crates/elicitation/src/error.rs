//! Error types for elicitation operations.

use derive_more::{Display, Error};

/// Specific error conditions during elicitation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
pub enum ElicitErrorKind {
    /// Invalid format received from MCP tool.
    #[display("Invalid format: expected {expected}, received {received}")]
    InvalidFormat {
        /// Expected format description.
        expected: String,
        /// Received value description.
        received: String,
    },

    /// Value out of valid range.
    #[display("Out of range: value must be between {min} and {max}")]
    OutOfRange {
        /// Minimum valid value.
        min: String,
        /// Maximum valid value.
        max: String,
    },

    /// MCP tool call failed.
    #[display("MCP tool error: {}", _0)]
    ToolError(String),

    /// User cancelled the elicitation.
    #[display("User cancelled elicitation")]
    Cancelled,

    /// Missing required field in survey.
    #[display("Missing required field: {}", _0)]
    MissingField(String),

    /// Invalid selection option.
    #[display("Invalid option: '{value}' not in [{options}]")]
    InvalidOption {
        /// Value provided by user.
        value: String,
        /// Valid options as comma-separated string.
        options: String,
    },
}

/// Elicitation error with location tracking.
#[derive(Debug, Clone, Display, Error)]
#[display("Elicit error: {} at {}:{}", kind, file, line)]
pub struct ElicitError {
    /// The specific error condition.
    pub kind: ElicitErrorKind,
    /// Line number where error occurred.
    pub line: u32,
    /// Source file where error occurred.
    pub file: &'static str,
}

impl ElicitError {
    /// Create a new error with location tracking.
    #[track_caller]
    pub fn new(kind: ElicitErrorKind) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            kind,
            line: loc.line(),
            file: loc.file(),
        }
    }
}

/// Convenience alias for elicitation results.
pub type ElicitResult<T> = Result<T, ElicitError>;

// Error conversion: ElicitErrorKind → ElicitError
impl From<ElicitErrorKind> for ElicitError {
    #[track_caller]
    fn from(kind: ElicitErrorKind) -> Self {
        tracing::error!(error_kind = %kind, "Error created");
        Self::new(kind)
    }
}

// Bridge pmcp errors through ToolError variant
impl From<pmcp::Error> for ElicitErrorKind {
    fn from(err: pmcp::Error) -> Self {
        ElicitErrorKind::ToolError(err.to_string())
    }
}

// Complete conversion chain: pmcp::Error → ElicitError
impl From<pmcp::Error> for ElicitError {
    #[track_caller]
    fn from(err: pmcp::Error) -> Self {
        let kind = ElicitErrorKind::from(err);
        tracing::error!(error_kind = %kind, "Error created from pmcp::Error");
        Self::new(kind)
    }
}
