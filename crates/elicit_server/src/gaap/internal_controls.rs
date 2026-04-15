//! Internal controls and audit trail propositions.
//!
//! Covers SOX 302/404 ICFR requirements, audit trail integrity,
//! segregation of duties, and access controls relevant to financial reporting.
//!
//! Source: Sarbanes-Oxley Act §302, §404; PCAOB AS 2201;
//!         COSO Internal Control — Integrated Framework (2013)

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    macro_rules! structural_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural */ }
                }
            }
        };
    }

    // ── SOX 302/404 — ICFR Assessment ─────────────────────────────────────────

    /// ICFR design has been evaluated and deemed adequate to prevent material misstatement.
    ///
    /// Source: SOX §404(a); COSO Internal Control Framework — Control Environment
    pub struct IcfrDesignAdequate;

    /// ICFR is operating effectively over the assessment period.
    ///
    /// Source: SOX §404(a); PCAOB AS 2201 — Operating Effectiveness
    pub struct IcfrOperatingEffective;

    /// Any identified material weakness in ICFR is disclosed in the annual report.
    ///
    /// Source: SOX §404(a); PCAOB AS 2201.09 — Material Weakness Disclosure
    pub struct MaterialWeaknessIdentified;

    /// Any significant deficiency is evaluated and communicated to the audit committee.
    ///
    /// Source: PCAOB AS 2201.69 — Significant Deficiency Communication
    pub struct SignificantDeficiencyEvaluated;

    /// Management's assessment of ICFR effectiveness (SOX 302 or 404) is completed.
    ///
    /// Source: SOX §302 — Management Certification; SOX §404(a) — Annual Assessment
    pub struct ManagementIcfrAssessmentCompleted;

    /// External auditor attestation on ICFR effectiveness is issued (large accelerated filers).
    ///
    /// Source: SOX §404(b); PCAOB AS 2201 — Auditor Attestation
    pub struct AuditorIcfrOpinionIssued;

    /// Disclosure controls and procedures are effective as of the period-end evaluation date.
    ///
    /// Source: SOX §302(a)(4) — Disclosure Controls and Procedures
    pub struct DisclosureControlsEffective;

    // ── Audit trail integrity ─────────────────────────────────────────────────

    /// Every journal entry links to an authorizing source document.
    ///
    /// Source: COSO — Control Activities; IRS Rev. Proc. 98-25 — Electronic Records
    pub struct AuditTrailComplete;

    /// Audit trail records are tamper-evident; modifications are logged with timestamp and user identity.
    ///
    /// Source: PCAOB AS 2110 — Identifying and Assessing Risks; SOX §802 — Record Retention
    pub struct AuditTrailTamperEvident;

    /// Journal entries are retained for the required retention period (minimum 7 years).
    ///
    /// Source: SOX §802 — Record Retention Requirements
    pub struct RecordsRetentionCompliant;

    // ── Segregation of duties ─────────────────────────────────────────────────

    /// Authorization, custody, and recording functions are performed by separate individuals.
    ///
    /// Source: COSO — Control Activities: Segregation of Duties
    pub struct SegregationOfDutiesEnforced;

    /// All transactions above the materiality threshold require documented authorization.
    ///
    /// Source: COSO — Control Activities: Authorization
    pub struct TransactionAuthorizationRequired;

    // ── Reconciliation and review controls ────────────────────────────────────

    /// Balance sheet account reconciliations are performed and reviewed within the close calendar.
    ///
    /// Source: COSO — Control Activities: Reconciliations
    pub struct ReconciliationPerformed;

    /// Manual journal entries are reviewed by a person independent of the preparer.
    ///
    /// Source: COSO — Control Activities: Supervisory Reviews; PCAOB AS 2110
    pub struct JournalEntryReviewCompleted;

    /// Period-end close controls checklist is completed before financial statements are issued.
    ///
    /// Source: COSO — Monitoring Activities: Ongoing Evaluations
    pub struct PeriodEndCloseControlsApplied;

    // ── Access and IT controls ────────────────────────────────────────────────

    /// Logical access controls restrict financial system access to authorized personnel only.
    ///
    /// Source: COSO — Control Activities: IT General Controls (ITGC)
    pub struct AccessControlsImplemented;

    /// Privileged access (admin, super-user) is logged, monitored, and periodically reviewed.
    ///
    /// Source: COSO — Control Activities: ITGC; SOX §404 IT Controls
    pub struct PrivilegedAccessMonitored;

    /// Change management controls govern modifications to financial reporting systems.
    ///
    /// Source: COSO — Control Activities: ITGC Change Management
    pub struct ChangeManagementControlsApplied;

    structural_prop!(IcfrDesignAdequate, "IcfrDesignAdequate");
    structural_prop!(IcfrOperatingEffective, "IcfrOperatingEffective");
    structural_prop!(MaterialWeaknessIdentified, "MaterialWeaknessIdentified");
    structural_prop!(
        SignificantDeficiencyEvaluated,
        "SignificantDeficiencyEvaluated"
    );
    structural_prop!(
        ManagementIcfrAssessmentCompleted,
        "ManagementIcfrAssessmentCompleted"
    );
    structural_prop!(AuditorIcfrOpinionIssued, "AuditorIcfrOpinionIssued");
    structural_prop!(DisclosureControlsEffective, "DisclosureControlsEffective");
    structural_prop!(AuditTrailComplete, "AuditTrailComplete");
    structural_prop!(AuditTrailTamperEvident, "AuditTrailTamperEvident");
    structural_prop!(RecordsRetentionCompliant, "RecordsRetentionCompliant");
    structural_prop!(SegregationOfDutiesEnforced, "SegregationOfDutiesEnforced");
    structural_prop!(
        TransactionAuthorizationRequired,
        "TransactionAuthorizationRequired"
    );
    structural_prop!(ReconciliationPerformed, "ReconciliationPerformed");
    structural_prop!(JournalEntryReviewCompleted, "JournalEntryReviewCompleted");
    structural_prop!(
        PeriodEndCloseControlsApplied,
        "PeriodEndCloseControlsApplied"
    );
    structural_prop!(AccessControlsImplemented, "AccessControlsImplemented");
    structural_prop!(PrivilegedAccessMonitored, "PrivilegedAccessMonitored");
    structural_prop!(
        ChangeManagementControlsApplied,
        "ChangeManagementControlsApplied"
    );
}

pub use emit_impls::{
    AccessControlsImplemented, AuditTrailComplete, AuditTrailTamperEvident,
    AuditorIcfrOpinionIssued, ChangeManagementControlsApplied, DisclosureControlsEffective,
    IcfrDesignAdequate, IcfrOperatingEffective, JournalEntryReviewCompleted,
    ManagementIcfrAssessmentCompleted, MaterialWeaknessIdentified, PeriodEndCloseControlsApplied,
    PrivilegedAccessMonitored, ReconciliationPerformed, RecordsRetentionCompliant,
    SegregationOfDutiesEnforced, SignificantDeficiencyEvaluated, TransactionAuthorizationRequired,
};
