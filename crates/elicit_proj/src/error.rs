//! Error types for `elicit_proj`.

/// The underlying cause of a [`ProjTransformError`].
#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display)]
pub enum ProjTransformErrorKind {
    /// PROJ could not create the transformation object.
    #[display("PROJ creation error: {}", _0)]
    Create(String),
    /// A coordinate operation (convert, project, etc.) failed.
    #[display("PROJ operation error: {}", _0)]
    Operation(String),
}

/// An error from a PROJ coordinate transformation.
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("Proj error: {} at {}:{}", kind, file, line)]
pub struct ProjTransformError {
    /// The underlying cause.
    pub kind: ProjTransformErrorKind,
    /// Source file where the error originated.
    pub file: &'static str,
    /// Source line where the error originated.
    pub line: u32,
}

impl ProjTransformError {
    /// Create a new creation-failure error.
    #[track_caller]
    pub fn create(msg: impl std::fmt::Display) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            kind: ProjTransformErrorKind::Create(msg.to_string()),
            file: loc.file(),
            line: loc.line(),
        }
    }

    /// Create a new operation-failure error.
    #[track_caller]
    pub fn operation(msg: impl std::fmt::Display) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            kind: ProjTransformErrorKind::Operation(msg.to_string()),
            file: loc.file(),
            line: loc.line(),
        }
    }
}

/// A [`Result`] type for fallible PROJ coordinate operations.
pub type ProjResult<T> = Result<T, ProjTransformError>;
