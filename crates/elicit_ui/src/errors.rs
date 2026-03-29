/// Error types for UI verification.

use crate::ElementId;
use derive_more::{Display, Error};

/// Specific verification error conditions.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
pub enum VerificationErrorKind {
    /// Element missing required accessible label.
    #[display("Element {} missing accessible label", _0)]
    MissingLabel(ElementId),

    /// Element label is empty.
    #[display("Element {} has empty label", _0)]
    EmptyLabel(ElementId),

    /// Element has invalid ARIA role.
    #[display("Element {} has invalid role: {:?}", _0, _1)]
    InvalidRole(ElementId, accesskit::Role),

    /// Interactive element below minimum touch target size (44x44).
    #[display(
        "Element {} below minimum target size: {}x{} (minimum 44x44)",
        _0,
        _1,
        _2
    )]
    BelowMinTargetSize(ElementId, u32, u32),

    /// Element overflows viewport boundaries.
    #[display(
        "Element {} overflows viewport: position ({}, {}) + size ({}x{}) exceeds viewport ({}x{})",
        _0,
        _1,
        _2,
        _3,
        _4,
        _5,
        _6
    )]
    OverflowsViewport(ElementId, i32, i32, u32, u32, u32, u32),

    /// Element not keyboard accessible.
    #[display("Element {} not keyboard accessible (missing focusable role)", _0)]
    NotKeyboardAccessible(ElementId),

    /// AccessKit tree structure error.
    #[display("Tree error: {}", _0)]
    TreeError(String),

    /// Node not found in tree.
    #[display("Node {} not found in tree", _0)]
    NodeNotFound(ElementId),
}

/// Verification error with location tracking.
#[derive(Debug, Clone, Display, Error)]
#[display("Verification error: {} at {}:{}", kind, file, line)]
pub struct VerificationError {
    /// The specific verification error
    pub kind: VerificationErrorKind,
    /// Source file where error was created
    pub file: &'static str,
    /// Line number where error was created
    pub line: u32,
}

impl VerificationError {
    /// Create a new verification error.
    #[track_caller]
    pub fn new(kind: VerificationErrorKind) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            kind,
            file: loc.file(),
            line: loc.line(),
        }
    }

    /// Create error for missing label.
    #[track_caller]
    pub fn missing_label(element_id: ElementId) -> Self {
        Self::new(VerificationErrorKind::MissingLabel(element_id))
    }

    /// Create error for empty label.
    #[track_caller]
    pub fn empty_label(element_id: ElementId) -> Self {
        Self::new(VerificationErrorKind::EmptyLabel(element_id))
    }

    /// Create error for invalid role.
    #[track_caller]
    pub fn invalid_role(element_id: ElementId, role: accesskit::Role) -> Self {
        Self::new(VerificationErrorKind::InvalidRole(element_id, role))
    }

    /// Create error for below minimum target size.
    #[track_caller]
    pub fn below_min_target_size(element_id: ElementId, width: u32, height: u32) -> Self {
        Self::new(VerificationErrorKind::BelowMinTargetSize(
            element_id,
            width,
            height,
        ))
    }

    /// Create error for viewport overflow.
    #[track_caller]
    pub fn overflows_viewport(
        element_id: ElementId,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        viewport_width: u32,
        viewport_height: u32,
    ) -> Self {
        Self::new(VerificationErrorKind::OverflowsViewport(
            element_id,
            x,
            y,
            width,
            height,
            viewport_width,
            viewport_height,
        ))
    }

    /// Create error for not keyboard accessible.
    #[track_caller]
    pub fn not_keyboard_accessible(element_id: ElementId) -> Self {
        Self::new(VerificationErrorKind::NotKeyboardAccessible(element_id))
    }

    /// Create tree error.
    #[track_caller]
    pub fn tree_error(message: impl Into<String>) -> Self {
        Self::new(VerificationErrorKind::TreeError(message.into()))
    }

    /// Create node not found error.
    #[track_caller]
    pub fn node_not_found(element_id: ElementId) -> Self {
        Self::new(VerificationErrorKind::NodeNotFound(element_id))
    }
}

/// Verification report containing all errors found during validation.
#[derive(Debug, Clone, Default)]
pub struct VerificationReport {
    /// All verification errors found
    pub errors: Vec<VerificationError>,
}

impl VerificationReport {
    /// Create a new empty report.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an error to the report.
    pub fn add_error(&mut self, error: VerificationError) {
        self.errors.push(error);
    }

    /// Check if the report has any errors.
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Get the number of errors.
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }
}
