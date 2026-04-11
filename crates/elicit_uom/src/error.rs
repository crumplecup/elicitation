//! Error types for `elicit_uom`.

use derive_more::{Display, Error};
use uuid::Uuid;

/// Specific error condition within a uom operation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
pub enum UomErrorKind {
    /// Unknown unit string for a given quantity registration.
    #[display("unknown unit '{}' for '{}'", unit, registration)]
    UnknownUnit {
        /// The unrecognised unit string.
        unit: String,
        /// The registration name (e.g. `"length"`).
        registration: String,
    },
    /// Quantity UUID not found in any registry.
    #[display("quantity '{}' not found", id)]
    NotFound {
        /// The UUID that was not found.
        id: Uuid,
    },
    /// Arithmetic operands belong to different registrations where same is required.
    #[display("operation requires same registration, got '{}' and '{}'", lhs, rhs)]
    HomogeneousRequired {
        /// LHS registration name.
        lhs: String,
        /// RHS registration name.
        rhs: String,
    },
    /// Dimension derivation for an operation has no known result.
    #[display(
        "dimension derivation failed: '{}' {} '{}' has no known result",
        lhs,
        op,
        rhs
    )]
    UnknownDerivation {
        /// LHS registration name (or operand description).
        lhs: String,
        /// Operation symbol (`"×"`, `"÷"`, `"√"`, `"^n"`).
        op: String,
        /// RHS registration name.
        rhs: String,
    },
    /// A JSON serialisation error occurred.
    #[display("serialisation error: {}", msg)]
    Serialisation {
        /// The serialisation error message.
        msg: String,
    },
}

/// Wrapper error carrying kind, source file, and line number.
#[derive(Debug, Clone, Display, Error)]
#[display("uom error: {} at {}:{}", kind, file, line)]
pub struct UomError {
    /// Specific error kind.
    pub kind: UomErrorKind,
    /// Source line number (via `#[track_caller]`).
    pub line: u32,
    /// Source file path (via `#[track_caller]`).
    pub file: &'static str,
}

impl UomError {
    /// Create a new `UomError` with caller location automatically captured.
    #[track_caller]
    pub fn new(kind: UomErrorKind) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            kind,
            line: loc.line(),
            file: loc.file(),
        }
    }
}
