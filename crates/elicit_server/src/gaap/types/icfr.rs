//! Internal control descriptor types — ICFR assessment and control testing.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::gaap::types::{EntityId, FinancialPeriod, PeriodDate};

// ── ICFR assessment ───────────────────────────────────────────────────────────

/// Descriptor for an Internal Control over Financial Reporting (ICFR) assessment.
///
/// The factory asserts `IcfrDesignAdequate` after design assessment and
/// `IcfrOperatingEffective` after operating-effectiveness testing.
///
/// Source: SOX §404; PCAOB AS 2201; COSO Internal Control—Integrated Framework.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct IcfrDescriptor {
    /// Legal entity subject to the assessment.
    pub entity: EntityId,
    /// Fiscal period covered.
    pub period: FinancialPeriod,
    /// Internal control frameworks referenced (e.g. `["COSO 2013"]`).
    pub frameworks: Vec<String>,
    /// Whether design adequacy has been assessed.
    pub design_assessed: bool,
    /// Whether operating effectiveness has been tested.
    pub operating_tested: bool,
    /// Whether any material weaknesses were identified.
    pub material_weakness_found: bool,
}

// ── Control test ──────────────────────────────────────────────────────────────

/// Descriptor for a single ICFR control test execution.
///
/// The factory uses this together with `Established<IcfrDesignAdequate>` to
/// assert `IcfrOperatingEffective`.
///
/// Source: PCAOB AS 2201 — Auditing ICFR Integrated with Financial Statement Audit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ControlTestDescriptor {
    /// Unique control reference (e.g. `"CTRL-AP-001"`).
    pub control_id: String,
    /// Description of the control activity.
    pub description: String,
    /// Date the test was performed (ISO 8601).
    pub test_date: PeriodDate,
    /// Number of items in the test sample.
    pub sample_size: u32,
    /// Number of exceptions or deviations found.
    pub exceptions: u32,
    /// Whether the exception rate is within the tolerable deviation rate.
    pub within_tolerance: bool,
}

// ── Management assertion ──────────────────────────────────────────────────────

/// Descriptor for a SOX 302 or SOX 404 management assertion.
///
/// The factory asserts `ManagementIcfrAssessmentCompleted` when the assertion
/// has been signed and dated by the appropriate certifying officers.
///
/// Source: SOX §302; SOX §404(a).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ManagementAssertionDescriptor {
    /// Legal entity making the assertion.
    pub entity: EntityId,
    /// Period covered.
    pub period: FinancialPeriod,
    /// ISO 8601 date of the assertion.
    pub assertion_date: PeriodDate,
    /// SOX section number (`"302"` or `"404"`).
    pub sox_section: String,
    /// Whether the CEO has signed.
    pub ceo_signed: bool,
    /// Whether the CFO has signed.
    pub cfo_signed: bool,
}
