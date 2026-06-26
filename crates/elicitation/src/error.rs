//! Error types for elicitation operations.

use derive_more::{Display, Error, From};

/// RMCP error wrapper.
#[derive(Debug, Clone, Display, Error, derive_getters::Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[display("RMCP error: {} at {}:{}", source, file, line)]
pub struct RmcpError {
    /// The underlying rmcp error.
    #[error(not(source))]
    source: String,
    /// Line number where the error occurred.
    line: u32,
    /// File where the error occurred.
    #[cfg_attr(feature = "serde", serde(skip))]
    file: &'static str,
}

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
#[derive(Debug, Clone, Display, Error, derive_getters::Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[display("JSON error: {} at {}:{}", source, file, line)]
pub struct JsonError {
    /// The underlying serde_json error.
    #[error(not(source))]
    source: String,
    /// Line number where the error occurred.
    line: u32,
    /// File where the error occurred.
    #[cfg_attr(feature = "serde", serde(skip))]
    file: &'static str,
}

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

/// URL parsing error wrapper.
#[cfg(feature = "url")]
#[derive(Debug, Clone, Display, Error, derive_getters::Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[display("URL error: {message} at {file}:{line}")]
pub struct UrlError {
    /// The underlying URL parsing error.
    #[cfg_attr(feature = "serde", serde(skip))]
    source: Option<std::sync::Arc<url::ParseError>>,
    /// The underlying URL parsing error message.
    message: String,
    /// Line number where the error occurred.
    line: u32,
    /// File where the error occurred.
    #[cfg_attr(feature = "serde", serde(skip))]
    file: &'static str,
}

#[cfg(feature = "url")]
impl UrlError {
    /// Creates a new URL error with caller location.
    #[track_caller]
    #[tracing::instrument(skip(source), level = "debug")]
    pub fn new(source: url::ParseError) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            message: source.to_string(),
            source: Some(std::sync::Arc::new(source)),
            line: loc.line(),
            file: loc.file(),
        }
    }
}

#[cfg(feature = "url")]
impl From<url::ParseError> for UrlError {
    #[track_caller]
    fn from(source: url::ParseError) -> Self {
        Self::new(source)
    }
}

/// Specific error conditions during elicitation.
#[derive(Debug, Clone, Display, Error, From)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

    /// URL parsing error.
    #[cfg(feature = "url")]
    #[display("{}", _0)]
    #[from]
    Url(UrlError),

    /// HTTP construction/parsing error.
    #[cfg(feature = "reqwest")]
    #[display("{}", _0)]
    #[from]
    Http(HttpError),

    /// Invalid HTTP header value error.
    #[cfg(feature = "reqwest")]
    #[display("{}", _0)]
    #[from]
    InvalidHeaderValue(InvalidHeaderValueError),

    /// Invalid HTTP header name error.
    #[cfg(feature = "reqwest")]
    #[display("{}", _0)]
    #[from]
    InvalidHeaderName(InvalidHeaderNameError),

    /// Reqwest runtime error.
    #[cfg(feature = "reqwest")]
    #[display("{}", _0)]
    #[from]
    Reqwest(ReqwestError),

    /// Invalid format received from MCP tool.
    #[display("Invalid format: expected {expected}, received {received}")]
    InvalidFormat {
        /// Expected format description.
        #[error(not(source))]
        expected: String,
        /// Received value description.
        #[error(not(source))]
        received: String,
    },

    /// Value out of valid range.
    #[display("Out of range: value must be between {min} and {max}")]
    OutOfRange {
        /// Minimum valid value.
        #[error(not(source))]
        min: String,
        /// Maximum valid value.
        #[error(not(source))]
        max: String,
    },

    /// User cancelled the elicitation.
    #[display("User cancelled elicitation")]
    Cancelled,

    /// Missing required field in survey.
    #[display("Missing required field: {}", _0)]
    MissingField(#[error(not(source))] String),

    /// Invalid selection option.
    #[display("Invalid option: '{value}' not in [{options}]")]
    InvalidOption {
        /// Value provided by user.
        #[error(not(source))]
        value: String,
        /// Valid options as comma-separated string.
        #[error(not(source))]
        options: String,
    },

    /// Invalid selection label.
    #[display("Invalid selection: {}", _0)]
    InvalidSelection(#[error(not(source))] String),

    /// Parse error for text input.
    #[display("Parse error: {}", _0)]
    ParseError(#[error(not(source))] String),

    /// Contract validation error.
    #[display("Validation failed: {}", _0)]
    Validation(#[error(not(source))] String),

    /// Recursion depth exceeded during elicitation.
    #[display("Recursion depth exceeded: maximum depth is {}", _0)]
    RecursionDepthExceeded(#[error(not(source))] usize),
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
#[derive(Debug, Clone, Display, Error, derive_getters::Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[display("Service error: {} at {}:{}", source, file, line)]
pub struct ServiceError {
    /// The underlying service error message.
    #[error(not(source))]
    source: String,
    /// Line number where the error occurred.
    line: u32,
    /// File where the error occurred.
    #[cfg_attr(feature = "serde", serde(skip))]
    file: &'static str,
}

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

/// HTTP error wrapper with caller location.
#[cfg(feature = "reqwest")]
#[derive(Debug, Clone, Display, derive_more::Error, derive_getters::Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[display("HTTP error: {message} at {file}:{line}")]
pub struct HttpError {
    /// The underlying HTTP error.
    #[cfg_attr(feature = "serde", serde(skip))]
    source: Option<std::sync::Arc<http::Error>>,
    /// The underlying HTTP error message.
    message: String,
    /// Line number where the error occurred.
    line: u32,
    /// File where the error occurred.
    #[cfg_attr(feature = "serde", serde(skip))]
    file: &'static str,
}

#[cfg(feature = "reqwest")]
impl HttpError {
    /// Creates a new HTTP error with caller location.
    #[track_caller]
    #[tracing::instrument(skip(source), level = "debug")]
    pub fn new(source: http::Error) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            message: source.to_string(),
            source: Some(std::sync::Arc::new(source)),
            line: loc.line(),
            file: loc.file(),
        }
    }
}

#[cfg(feature = "reqwest")]
impl From<http::Error> for HttpError {
    #[track_caller]
    fn from(source: http::Error) -> Self {
        Self::new(source)
    }
}

/// Invalid HTTP header value error wrapper with caller location.
#[cfg(feature = "reqwest")]
#[derive(Debug, Clone, Display, derive_more::Error, derive_getters::Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[display("Invalid HTTP header value: {message} at {file}:{line}")]
pub struct InvalidHeaderValueError {
    /// The underlying invalid header value error.
    #[cfg_attr(feature = "serde", serde(skip))]
    source: Option<std::sync::Arc<http::header::InvalidHeaderValue>>,
    /// The underlying invalid header value error message.
    message: String,
    /// Line number where the error occurred.
    line: u32,
    /// File where the error occurred.
    #[cfg_attr(feature = "serde", serde(skip))]
    file: &'static str,
}

#[cfg(feature = "reqwest")]
impl InvalidHeaderValueError {
    /// Creates a new invalid header value error with caller location.
    #[track_caller]
    #[tracing::instrument(skip(source), level = "debug")]
    pub fn new(source: http::header::InvalidHeaderValue) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            message: source.to_string(),
            source: Some(std::sync::Arc::new(source)),
            line: loc.line(),
            file: loc.file(),
        }
    }
}

#[cfg(feature = "reqwest")]
impl From<http::header::InvalidHeaderValue> for InvalidHeaderValueError {
    #[track_caller]
    fn from(source: http::header::InvalidHeaderValue) -> Self {
        Self::new(source)
    }
}

/// Invalid HTTP header name error wrapper with caller location.
#[cfg(feature = "reqwest")]
#[derive(Debug, Clone, Display, derive_more::Error, derive_getters::Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[display("Invalid HTTP header name: {message} at {file}:{line}")]
pub struct InvalidHeaderNameError {
    /// The underlying invalid header name error.
    #[cfg_attr(feature = "serde", serde(skip))]
    source: Option<std::sync::Arc<http::header::InvalidHeaderName>>,
    /// The underlying invalid header name error message.
    message: String,
    /// Line number where the error occurred.
    line: u32,
    /// File where the error occurred.
    #[cfg_attr(feature = "serde", serde(skip))]
    file: &'static str,
}

#[cfg(feature = "reqwest")]
impl InvalidHeaderNameError {
    /// Creates a new invalid header name error with caller location.
    #[track_caller]
    #[tracing::instrument(skip(source), level = "debug")]
    pub fn new(source: http::header::InvalidHeaderName) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            message: source.to_string(),
            source: Some(std::sync::Arc::new(source)),
            line: loc.line(),
            file: loc.file(),
        }
    }
}

#[cfg(feature = "reqwest")]
impl From<http::header::InvalidHeaderName> for InvalidHeaderNameError {
    #[track_caller]
    fn from(source: http::header::InvalidHeaderName) -> Self {
        Self::new(source)
    }
}

/// Reqwest error wrapper with caller location.
#[cfg(feature = "reqwest")]
#[derive(Debug, Clone, Display, derive_more::Error, derive_getters::Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[display("Reqwest error: {message} at {file}:{line}")]
pub struct ReqwestError {
    /// The underlying reqwest error.
    #[cfg_attr(feature = "serde", serde(skip))]
    source: Option<std::sync::Arc<reqwest::Error>>,
    /// The underlying reqwest error message.
    message: String,
    /// Line number where the error occurred.
    line: u32,
    /// File where the error occurred.
    #[cfg_attr(feature = "serde", serde(skip))]
    file: &'static str,
}

#[cfg(feature = "reqwest")]
impl ReqwestError {
    /// Creates a new reqwest error with caller location.
    #[track_caller]
    #[tracing::instrument(skip(source), level = "debug")]
    pub fn new(source: reqwest::Error) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            message: source.to_string(),
            source: Some(std::sync::Arc::new(source)),
            line: loc.line(),
            file: loc.file(),
        }
    }
}

#[cfg(feature = "reqwest")]
impl From<reqwest::Error> for ReqwestError {
    #[track_caller]
    fn from(source: reqwest::Error) -> Self {
        Self::new(source)
    }
}

// Bridge From implementations to chain external errors through wrappers
bridge_error!(rmcp::ErrorData => RmcpError);
bridge_error!(rmcp::service::ServiceError => ServiceError);
bridge_error!(serde_json::Error => JsonError);
#[cfg(feature = "url")]
bridge_error!(url::ParseError => UrlError);
#[cfg(feature = "reqwest")]
bridge_error!(http::Error => HttpError);
#[cfg(feature = "reqwest")]
bridge_error!(http::header::InvalidHeaderValue => InvalidHeaderValueError);
#[cfg(feature = "reqwest")]
bridge_error!(http::header::InvalidHeaderName => InvalidHeaderNameError);
#[cfg(feature = "reqwest")]
bridge_error!(reqwest::Error => ReqwestError);

/// Elicitation error with location tracking.
///
/// This type wraps all error conditions and provides automatic conversion
/// from underlying error types through the `?` operator.
#[derive(Debug, Clone, Display, Error)]
#[display("Elicit error: {}", _0)]
pub struct ElicitError(Box<ElicitErrorKind>);

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
#[cfg(feature = "url")]
error_from!(url::ParseError);
#[cfg(feature = "reqwest")]
error_from!(http::Error);
#[cfg(feature = "reqwest")]
error_from!(http::header::InvalidHeaderValue);
#[cfg(feature = "reqwest")]
error_from!(http::header::InvalidHeaderName);
#[cfg(feature = "reqwest")]
error_from!(reqwest::Error);

// Add conversion for ValidationError
impl From<crate::verification::types::ValidationError> for ElicitError {
    #[track_caller]
    fn from(err: crate::verification::types::ValidationError) -> Self {
        let kind = ElicitErrorKind::Validation(err.to_string());
        tracing::error!(error_kind = %kind, "Error from ValidationError");
        Self(Box::new(kind))
    }
}

// Add conversion for rmcp::service::ElicitationError
impl From<rmcp::service::ElicitationError> for ElicitError {
    #[track_caller]
    fn from(err: rmcp::service::ElicitationError) -> Self {
        use rmcp::service::ElicitationError as EE;
        let kind = match err {
            EE::Service(se) => ElicitErrorKind::Service(ServiceError::from(se)),
            EE::UserDeclined | EE::UserCancelled => ElicitErrorKind::InvalidFormat {
                expected: "user response".to_string(),
                received: "cancelled".to_string(),
            },
            EE::ParseError { error, .. } => ElicitErrorKind::Json(JsonError::from(error)),
            EE::CapabilityNotSupported => ElicitErrorKind::InvalidFormat {
                expected: "elicitation capability".to_string(),
                received: "unsupported".to_string(),
            },
            EE::NoContent => ElicitErrorKind::InvalidFormat {
                expected: "elicitation response".to_string(),
                received: "no content".to_string(),
            },
            _ => ElicitErrorKind::InvalidFormat {
                expected: "elicitation response".to_string(),
                received: "unknown error".to_string(),
            },
        };
        tracing::error!(error_kind = %kind, "Error from ElicitationError");
        Self(Box::new(kind))
    }
}

/// Convenience alias for elicitation results.
pub type ElicitResult<T> = Result<T, ElicitError>;
