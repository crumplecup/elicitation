//! Error types for `elicit_db`.

use derive_more::{Display, Error};

/// Specific error conditions for database operations.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
pub enum DbErrorKind {
    /// Connection to the database failed.
    #[display("Connection failed: {}", _0)]
    ConnectionFailed(String),
    /// Query execution failed.
    #[display("Query failed: {}", _0)]
    QueryFailed(String),
    /// Transaction state error.
    #[display("Transaction error: {}", _0)]
    TransactionError(String),
    /// Schema manipulation error.
    #[display("Schema error: {}", _0)]
    SchemaError(String),
    /// Permission was denied for the requested operation.
    #[display("Permission denied: {}", _0)]
    PermissionDenied(String),
    /// Requested resource was not found.
    #[display("Not found: {}", _0)]
    NotFound(String),
    /// Constraint violation detected.
    #[display("Constraint violation: {}", _0)]
    ConstraintViolation(String),
    /// Serialization or deserialization error.
    #[display("Serialization error: {}", _0)]
    Serialization(String),
    /// Operation timed out.
    #[display("Timeout: {}", _0)]
    Timeout(String),
    /// Operation is not supported by this backend.
    #[display("Unsupported operation: {}", _0)]
    Unsupported(String),
}

/// Database operation error with source location.
#[derive(Debug, Clone, Display, Error)]
#[display("{} at {}:{}", kind, file, line)]
pub struct DbError {
    /// Specific error kind.
    pub kind: DbErrorKind,
    /// Line number where the error was created.
    pub line: u32,
    /// File where the error was created.
    pub file: &'static str,
}

impl DbError {
    /// Create a new [`DbError`] capturing the call site location.
    #[track_caller]
    pub fn new(kind: DbErrorKind) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            kind,
            line: loc.line(),
            file: loc.file(),
        }
    }
}

/// Convenience alias for database operation results.
pub type DbResult<T> = Result<T, DbError>;
