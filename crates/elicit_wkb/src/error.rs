//! Error wrappers for the upstream `wkb::error` module.

/// Error enum mirroring the public upstream `wkb::error::WkbError` variants.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    derive_more::Display,
)]
pub enum WkbError {
    /// Incorrect type was passed to an operation.
    #[display("Incorrect type passed to operation: {}", _0)]
    IncorrectType(String),
    /// Returned when functionality is not yet available.
    #[display("Not yet implemented: {}", _0)]
    NotYetImplemented(String),
    /// General error.
    #[display("General error: {}", _0)]
    General(String),
    /// I/O error raised by the writer layer.
    #[display("IO error: {}", _0)]
    IOError(String),
    /// Overflow when writing a size or type code.
    #[display("Overflow error: {}", _0)]
    OverflowError(String),
}

/// Crate-local result alias.
pub type WkbResult<T> = Result<T, WkbError>;

impl From<wkb::error::WkbError> for WkbError {
    fn from(value: wkb::error::WkbError) -> Self {
        match value {
            wkb::error::WkbError::IncorrectType(message) => {
                Self::IncorrectType(message.into_owned())
            }
            wkb::error::WkbError::NotYetImplemented(message) => Self::NotYetImplemented(message),
            wkb::error::WkbError::General(message) => Self::General(message),
            wkb::error::WkbError::IOError(error) => Self::IOError(error.to_string()),
            wkb::error::WkbError::OverflowError(error) => Self::OverflowError(error.to_string()),
            _ => Self::General("Unsupported upstream WkbError variant".to_string()),
        }
    }
}
