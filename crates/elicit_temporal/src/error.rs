//! Error types for `elicit_temporal`.

use derive_more::{Display, Error};

use crate::SerializationProfile;

/// Specific error conditions for temporal trait operations.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
pub enum TemporalErrorKind {
    /// Input text does not conform to the requested serialization profile.
    #[display("Parse rejected under {}: {}", _0, _1)]
    ParseRejected(SerializationProfile, String),
    /// A neutral descriptor violates the required contract shape.
    #[display("Invalid temporal descriptor: {}", _0)]
    InvalidDescriptor(&'static str),
    /// The requested operation is unsupported by this backend.
    #[display("Unsupported temporal operation: {}", _0)]
    Unsupported(&'static str),
    /// The operation would require explicit authority for a lossy conversion.
    #[display("Lossy conversion requires explicit authority")]
    LossyConversionRequiresAuthority,
    /// A local timestamp is ambiguous at a zone transition.
    #[display("Ambiguous local timestamp: {}", _0)]
    AmbiguousLocalTimestamp(String),
    /// A named-zone annotation is inconsistent with the represented instant.
    #[display("Named-zone inconsistency: {}", _0)]
    NamedZoneInconsistency(String),
}

/// Temporal trait-layer error with source location.
#[derive(Debug, Clone, Display, Error)]
#[display("{} at {}:{}", kind, file, line)]
pub struct TemporalError {
    /// Specific error kind.
    pub kind: TemporalErrorKind,
    /// File where the error was created.
    pub file: &'static str,
    /// Line number where the error was created.
    pub line: u32,
}

impl TemporalError {
    /// Create a new [`TemporalError`] capturing the call-site location.
    #[track_caller]
    pub fn new(kind: TemporalErrorKind) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            kind,
            file: loc.file(),
            line: loc.line(),
        }
    }
}

/// Result type for temporal operations.
pub type TemporalResult<T> = Result<T, TemporalError>;
