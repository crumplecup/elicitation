//! Error types for elicitation operations.

use derive_more::{Display, From};

/// RMCP error wrapper.
#[derive(Debug, Clone, Display, derive_getters::Getters)]
#[display("RMCP error: {}", source)]
pub struct RmcpError {
    /// The underlying rmcp error.
    source: String,
    /// Line number where the error occurred.
    line: u32,
    /// File where the error occurred.
    file: &'static str,
}

impl std::error::Error for RmcpError {}

impl RmcpError {
    /// Creates a new RMCP error with caller location.
    #[track_caller]
    pub fn new(source: rmcp::ErrorData) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source: source.to_string(),
            line: loc.line(),
            file: loc.file(),
        }
    }
}

impl From<rmcp::ErrorData> for RmcpError {
    #[track_caller]
    fn from(source: rmcp::ErrorData) -> Self {
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
    /// RMCP error.
    #[display("{}", _0)]
    #[from]
    Rmcp(RmcpError),

    /// Service error.
    #[display("{}", _0)]
    #[from]
    Service(ServiceError),

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

    /// Invalid selection label.
    #[display("Invalid selection: {}", _0)]
    InvalidSelection(String),

    /// Parse error for text input.
    #[display("Parse error: {}", _0)]
    ParseError(String),

    /// Recursion depth exceeded during elicitation.
    #[display("Recursion depth exceeded: maximum depth is {}", _0)]
    RecursionDepthExceeded(usize),
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

/// RMCP ServiceError wrapper for error conversion.
#[derive(Debug, Clone, Display, derive_getters::Getters)]
#[display("Service error: {}", source)]
pub struct ServiceError {
    /// The underlying service error message.
    source: String,
    /// Line number where the error occurred.
    line: u32,
    /// File where the error occurred.
    file: &'static str,
}

impl std::error::Error for ServiceError {}

impl ServiceError {
    /// Creates a new service error with caller location.
    #[track_caller]
    pub fn new(source: rmcp::service::ServiceError) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source: source.to_string(),
            line: loc.line(),
            file: loc.file(),
        }
    }
}

impl From<rmcp::service::ServiceError> for ServiceError {
    #[track_caller]
    fn from(source: rmcp::service::ServiceError) -> Self {
        Self::new(source)
    }
}

// Bridge From implementations to chain external errors through wrappers
bridge_error!(rmcp::ErrorData => RmcpError);
bridge_error!(rmcp::service::ServiceError => ServiceError);
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
            ElicitErrorKind::Rmcp(e) => Some(e),
            ElicitErrorKind::Service(e) => Some(e),
            ElicitErrorKind::Json(e) => Some(e),
            _ => None,
        }
    }
}

impl ElicitError {
    /// Returns a reference to the underlying error kind.
    #[tracing::instrument(skip(self), level = "trace")]
    pub fn kind(&self) -> &ElicitErrorKind {
        &self.0
    }

    /// Create a new error with location tracking.
    #[track_caller]
    #[tracing::instrument(skip(kind), level = "debug")]
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
error_from!(rmcp::ErrorData);
error_from!(rmcp::service::ServiceError);
error_from!(serde_json::Error);

/// Convenience alias for elicitation results.
pub type ElicitResult<T> = Result<T, ElicitError>;
