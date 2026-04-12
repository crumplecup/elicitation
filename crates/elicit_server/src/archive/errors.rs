//! Error types for the archive module.

use derive_more::{Display, Error};

/// Specific error conditions that can occur during archive operations.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
pub enum ArchiveErrorKind {
    /// A referenced schema does not exist in the database.
    #[display("schema not found: {}", _0)]
    SchemaNotFound(String),

    /// A referenced table does not exist in the given schema.
    #[display("table not found: {}.{}", _0, _1)]
    TableNotFound(String, String),

    /// A referenced index does not exist.
    #[display("index not found: {}", _0)]
    IndexNotFound(String),

    /// A query failed at the database layer.
    #[display("query failed: {}", _0)]
    QueryFailed(String),

    /// A display mode string was not recognised for the requested type.
    #[display("unknown display mode '{}' for type '{}'", _0, _1)]
    UnknownDisplayMode(String, String),

    /// A spatial operation was attempted on a non-spatial column.
    #[display("column '{}' in '{}.{}' is not a spatial type", _0, _1, _2)]
    NotSpatialColumn(String, String, String),

    /// The backend kind could not be detected from the connection URL.
    #[display("unrecognised backend in connection URL")]
    UnknownBackend,

    /// Failed to establish a database connection.
    #[display("connection failed: {}", _0)]
    Connection(String),

    /// A live database query or metadata fetch failed.
    #[display("query error: {}", _0)]
    Query(String),

    /// A frontend rendering or I/O error.
    #[display("frontend error: {}", _0)]
    Frontend(String),
}

/// Top-level archive error wrapping a kind with source location.
#[derive(Debug, Clone, Display, Error)]
#[display("archive error: {} at {}:{}", kind, file, line)]
pub struct ArchiveError {
    /// The specific error condition.
    pub kind: ArchiveErrorKind,
    /// Source line number where the error was created.
    pub line: u32,
    /// Source file where the error was created.
    pub file: &'static str,
}

impl ArchiveError {
    /// Creates a new `ArchiveError` recording the caller's location.
    #[track_caller]
    pub fn new(kind: ArchiveErrorKind) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            kind,
            line: loc.line(),
            file: loc.file(),
        }
    }

    /// Convenience constructor for `SchemaNotFound`.
    #[track_caller]
    pub fn schema_not_found(schema: impl Into<String>) -> Self {
        Self::new(ArchiveErrorKind::SchemaNotFound(schema.into()))
    }

    /// Convenience constructor for `TableNotFound`.
    #[track_caller]
    pub fn table_not_found(schema: impl Into<String>, table: impl Into<String>) -> Self {
        Self::new(ArchiveErrorKind::TableNotFound(schema.into(), table.into()))
    }

    /// Convenience constructor for `QueryFailed`.
    #[track_caller]
    pub fn query_failed(msg: impl Into<String>) -> Self {
        Self::new(ArchiveErrorKind::QueryFailed(msg.into()))
    }

    /// Convenience constructor for `UnknownDisplayMode`.
    #[track_caller]
    pub fn unknown_display_mode(mode: impl Into<String>, ty: impl Into<String>) -> Self {
        Self::new(ArchiveErrorKind::UnknownDisplayMode(mode.into(), ty.into()))
    }

    /// Convenience constructor for `NotSpatialColumn`.
    #[track_caller]
    pub fn not_spatial(
        col: impl Into<String>,
        schema: impl Into<String>,
        table: impl Into<String>,
    ) -> Self {
        Self::new(ArchiveErrorKind::NotSpatialColumn(
            col.into(),
            schema.into(),
            table.into(),
        ))
    }
}

/// Result type for archive operations.
pub type ArchiveResult<T> = Result<T, ArchiveError>;
