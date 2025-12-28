//! Error types for elicitation operations.

use derive_more::{Display, From};

/// MCP error wrapper.
#[derive(Debug, Clone, Display, derive_getters::Getters)]
#[display("MCP error: {}", source)]
pub struct PmcpError {
    /// The underlying pmcp error.
    source: String,
    /// Line number where the error occurred.
    line: u32,
    /// File where the error occurred.
    file: &'static str,
}

impl std::error::Error for PmcpError {}

impl PmcpError {
    /// Creates a new MCP error with caller location.
    #[track_caller]
    pub fn new(source: pmcp::Error) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source: source.to_string(),
            line: loc.line(),
            file: loc.file(),
        }
    }
}

impl From<pmcp::Error> for PmcpError {
    #[track_caller]
    fn from(source: pmcp::Error) -> Self {
        Self::new(source)
    }
}

/// JSON parsing error wrapper.
#[derive(Debug, Clone, Display, derive_getters::Getters)]
#[display("JSON error: {}", source)]
pub struct JsonError {
    /// The underlying serde_json error.
    source: String,
    /// Line number where the error occurred.
    line: u32,
    /// File where the error occurred.
    file: &'static str,
}

impl std::error::Error for JsonError {}

impl JsonError {
    /// Creates a new JSON error with caller location.
    #[track_caller]
    pub fn new(source: serde_json::Error) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source: source.to_string(),
            line: loc.line(),
            file: loc.file(),
        }
    }
}

impl From<serde_json::Error> for JsonError {
    #[track_caller]
    fn from(source: serde_json::Error) -> Self {
        Self::new(source)
    }
}

/// Specific error conditions during elicitation.
#[derive(Debug, Clone, Display, From)]
pub enum ElicitErrorKind {
    /// MCP error.
    #[display("{}", _0)]
    #[from]
    Mcp(PmcpError),

    /// JSON parsing error.
    #[display("{}", _0)]
    #[from]
    Json(JsonError),

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

/// Macro to generate bridge From implementations for external errors.
///
/// This creates the conversion chain: ExternalError → WrapperError → ElicitErrorKind
///
/// # Example
/// ```ignore
/// bridge_error!(pmcp::Error => PmcpError);
/// // Generates:
/// // impl From<pmcp::Error> for ElicitErrorKind {
/// //     #[track_caller]
/// //     fn from(err: pmcp::Error) -> Self {
/// //         PmcpError::from(err).into()
/// //     }
/// // }
/// ```
macro_rules! bridge_error {
    ($external:ty => $wrapper:ty) => {
        impl From<$external> for ElicitErrorKind {
            #[track_caller]
            fn from(err: $external) -> Self {
                <$wrapper>::from(err).into()
            }
        }
    };
}

// Bridge From implementations to chain external errors through wrappers
bridge_error!(pmcp::Error => PmcpError);
bridge_error!(serde_json::Error => JsonError);

/// Elicitation error with location tracking.
///
/// This type wraps all error conditions and provides automatic conversion
/// from underlying error types through the `?` operator.
#[derive(Debug, Clone, Display)]
#[display("Elicit error: {}", _0)]
pub struct ElicitError(Box<ElicitErrorKind>);

impl std::error::Error for ElicitError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &*self.0 {
            ElicitErrorKind::Mcp(e) => Some(e),
            ElicitErrorKind::Json(e) => Some(e),
            _ => None,
        }
    }
}

impl ElicitError {
    /// Returns a reference to the underlying error kind.
    pub fn kind(&self) -> &ElicitErrorKind {
        &self.0
    }

    /// Create a new error with location tracking.
    #[track_caller]
    pub fn new(kind: ElicitErrorKind) -> Self {
        tracing::error!(error_kind = %kind, "Error created");
        Self(Box::new(kind))
    }
}

/// Macro to implement From<SourceError> for ElicitError.
///
/// This creates the full conversion chain: SourceError → ElicitErrorKind → ElicitError
/// with proper location tracking and error logging.
///
/// # Example
/// ```ignore
/// error_from!(pmcp::Error);
/// // Generates:
/// // impl From<pmcp::Error> for ElicitError {
/// //     #[track_caller]
/// //     fn from(err: pmcp::Error) -> Self {
/// //         let kind = ElicitErrorKind::from(err);
/// //         tracing::error!(error_kind = %kind, "Error created");
/// //         Self(Box::new(kind))
/// //     }
/// // }
/// ```
macro_rules! error_from {
    ($source:ty) => {
        impl From<$source> for ElicitError {
            #[track_caller]
            fn from(err: $source) -> Self {
                let kind = ElicitErrorKind::from(err);
                tracing::error!(error_kind = %kind, "Error created");
                Self(Box::new(kind))
            }
        }
    };
}

// Implement From<ElicitErrorKind> for ElicitError
impl From<ElicitErrorKind> for ElicitError {
    #[track_caller]
    fn from(kind: ElicitErrorKind) -> Self {
        tracing::error!(error_kind = %kind, "Error created");
        Self(Box::new(kind))
    }
}

// Implement From for all external error types
error_from!(pmcp::Error);
error_from!(serde_json::Error);

/// Convenience alias for elicitation results.
pub type ElicitResult<T> = Result<T, ElicitError>;
