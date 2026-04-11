//! Perform WCAG accessibility audits.

use elicitation::Established;

use crate::{
    AccessibleAA, ContrastViolation, SufficientContrast, UiResult, VerificationReport, WidgetA11y,
    WidgetId,
};

/// Perform WCAG accessibility audits.
pub trait UiAccessibilityAuditor: Send + Sync {
    /// Audit against WCAG Level A.
    fn audit_wcag_a(&self) -> UiResult<(VerificationReport, Established<AccessibleAA>)>;

    /// Audit against WCAG Level AA.
    fn audit_wcag_aa(&self) -> UiResult<(VerificationReport, Established<AccessibleAA>)>;

    /// Check all color pairs for WCAG 1.4.3 contrast compliance.
    ///
    /// Returns the list of violations and a proof token if there are none.
    fn audit_contrast(
        &self,
    ) -> UiResult<(
        Vec<ContrastViolation>,
        Option<Established<SufficientContrast>>,
    )>;

    /// Get accessibility status for a specific widget.
    fn widget_accessibility(&self, id: WidgetId) -> UiResult<WidgetA11y>;
}
