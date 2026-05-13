//! Internal controls and audit trail propositions.
//!
//! Covers SOX 302/404 ICFR requirements, audit trail integrity,
//! segregation of duties, and access controls relevant to financial reporting.
//!
//! Source: Sarbanes-Oxley Act §302, §404; PCAOB AS 2201;
//!         COSO Internal Control — Integrated Framework (2013)
// ── SOX 302/404 — ICFR Assessment ─────────────────────────────────────────

/// ICFR design has been evaluated and deemed adequate to prevent material misstatement.
///
/// Source: SOX §404(a); COSO Internal Control Framework — Control Environment
#[derive(elicitation::Prop)]
pub struct IcfrDesignAdequate;

/// ICFR is operating effectively over the assessment period.
///
/// Source: SOX §404(a); PCAOB AS 2201 — Operating Effectiveness
#[derive(elicitation::Prop)]
pub struct IcfrOperatingEffective;

/// Any identified material weakness in ICFR is disclosed in the annual report.
///
/// Source: SOX §404(a); PCAOB AS 2201.09 — Material Weakness Disclosure
#[derive(elicitation::Prop)]
pub struct MaterialWeaknessIdentified;

/// Any significant deficiency is evaluated and communicated to the audit committee.
///
/// Source: PCAOB AS 2201.69 — Significant Deficiency Communication
#[derive(elicitation::Prop)]
pub struct SignificantDeficiencyEvaluated;

/// Management's assessment of ICFR effectiveness (SOX 302 or 404) is completed.
///
/// Source: SOX §302 — Management Certification; SOX §404(a) — Annual Assessment
#[derive(elicitation::Prop)]
pub struct ManagementIcfrAssessmentCompleted;

/// External auditor attestation on ICFR effectiveness is issued (large accelerated filers).
///
/// Source: SOX §404(b); PCAOB AS 2201 — Auditor Attestation
#[derive(elicitation::Prop)]
pub struct AuditorIcfrOpinionIssued;

/// Disclosure controls and procedures are effective as of the period-end evaluation date.
///
/// Source: SOX §302(a)(4) — Disclosure Controls and Procedures
#[derive(elicitation::Prop)]
pub struct DisclosureControlsEffective;

// ── Audit trail integrity ─────────────────────────────────────────────────

/// Every journal entry links to an authorizing source document.
///
/// Source: COSO — Control Activities; IRS Rev. Proc. 98-25 — Electronic Records
#[derive(elicitation::Prop)]
pub struct AuditTrailComplete;

/// Audit trail records are tamper-evident; modifications are logged with timestamp and user identity.
///
/// Source: PCAOB AS 2110 — Identifying and Assessing Risks; SOX §802 — Record Retention
#[derive(elicitation::Prop)]
pub struct AuditTrailTamperEvident;

/// Journal entries are retained for the required retention period (minimum 7 years).
///
/// Source: SOX §802 — Record Retention Requirements
#[derive(elicitation::Prop)]
pub struct RecordsRetentionCompliant;

// ── Segregation of duties ─────────────────────────────────────────────────

/// Authorization, custody, and recording functions are performed by separate individuals.
///
/// Source: COSO — Control Activities: Segregation of Duties
#[derive(elicitation::Prop)]
pub struct SegregationOfDutiesEnforced;

/// All transactions above the materiality threshold require documented authorization.
///
/// Source: COSO — Control Activities: Authorization
#[derive(elicitation::Prop)]
pub struct TransactionAuthorizationRequired;

// ── Reconciliation and review controls ────────────────────────────────────

/// Balance sheet account reconciliations are performed and reviewed within the close calendar.
///
/// Source: COSO — Control Activities: Reconciliations
#[derive(elicitation::Prop)]
pub struct ReconciliationPerformed;

/// Manual journal entries are reviewed by a person independent of the preparer.
///
/// Source: COSO — Control Activities: Supervisory Reviews; PCAOB AS 2110
#[derive(elicitation::Prop)]
pub struct JournalEntryReviewCompleted;

/// Period-end close controls checklist is completed before financial statements are issued.
///
/// Source: COSO — Monitoring Activities: Ongoing Evaluations
#[derive(elicitation::Prop)]
pub struct PeriodEndCloseControlsApplied;

// ── Access and IT controls ────────────────────────────────────────────────

/// Logical access controls restrict financial system access to authorized personnel only.
///
/// Source: COSO — Control Activities: IT General Controls (ITGC)
#[derive(elicitation::Prop)]
pub struct AccessControlsImplemented;

/// Privileged access (admin, super-user) is logged, monitored, and periodically reviewed.
///
/// Source: COSO — Control Activities: ITGC; SOX §404 IT Controls
#[derive(elicitation::Prop)]
pub struct PrivilegedAccessMonitored;

/// Change management controls govern modifications to financial reporting systems.
///
/// Source: COSO — Control Activities: ITGC Change Management
#[derive(elicitation::Prop)]
pub struct ChangeManagementControlsApplied;
