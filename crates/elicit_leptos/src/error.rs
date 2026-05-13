//! Error types for elicit_leptos.

use derive_more::{Display, Error};

/// Specific error conditions for Leptos MCP tools.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
pub enum LeptosErrorKind {
    /// Signal not found in registry.
    #[display("Signal not found: {}", _0)]
    SignalNotFound(String),
    /// Memo not found in registry.
    #[display("Memo not found: {}", _0)]
    MemoNotFound(String),
    /// Action not found in registry.
    #[display("Action not found: {}", _0)]
    ActionNotFound(String),
    /// Component not found in registry.
    #[display("Component not found: {}", _0)]
    ComponentNotFound(String),
    /// Route not found in registry.
    #[display("Route not found: {}", _0)]
    RouteNotFound(String),
    /// App not found in registry.
    #[display("App not found: {}", _0)]
    AppNotFound(String),
    /// Server function not found in registry.
    #[display("Server function not found: {}", _0)]
    ServerFnNotFound(String),
    /// Invalid operation for given state.
    #[display("Invalid operation: {}", _0)]
    InvalidOperation(String),
    /// Serialization failure.
    #[display("Serialization error: {}", _0)]
    Serialization(String),
}

/// Wrapper error carrying source location.
#[derive(Debug, Clone, Display, Error)]
#[display("{} at {}:{}", kind, file, line)]
pub struct LeptosError {
    /// The underlying error kind.
    pub kind: LeptosErrorKind,
    /// Source line number.
    pub line: u32,
    /// Source file path.
    pub file: &'static str,
}

impl LeptosError {
    /// Create a new error with automatic caller location.
    #[track_caller]
    pub fn new(kind: LeptosErrorKind) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            kind,
            line: loc.line(),
            file: loc.file(),
        }
    }
}

/// Result type for Leptos MCP tool operations.
pub type LeptosResult<T> = Result<T, LeptosError>;
