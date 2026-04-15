//! `GaapIcfrFactory` — ICFR assessment factory (Role 1a).
//! `GaapIcfrMeta`    — ICFR status reporter (Role 2).

use futures::future::BoxFuture;

use crate::gaap::errors::GaapResult;
use crate::gaap::internal_controls::{
    IcfrDesignAdequate, IcfrOperatingEffective, ManagementIcfrAssessmentCompleted,
    SegregationOfDutiesEnforced,
};
use crate::gaap::types::{
    ControlTestDescriptor, EntityId, IcfrDescriptor, ManagementAssertionDescriptor,
};
use elicitation::Established;

// ── Role 1a: ICFR assessment factory ─────────────────────────────────────────

/// Factory for Internal Control over Financial Reporting (ICFR) assessments.
///
/// Source: SOX §404; PCAOB AS 2201; COSO Internal Control—Integrated Framework.
pub trait GaapIcfrFactory: Send + Sync {
    // ── Design assessment ─────────────────────────────────────────────────────

    /// Assess control design for adequacy.
    ///
    /// Returns `IcfrDesignAdequate`.
    ///
    /// Source: SOX §404(a); PCAOB AS 2201.03.
    fn assess_icfr_design(
        &self,
        icfr: IcfrDescriptor,
    ) -> GaapResult<(IcfrDescriptor, Established<IcfrDesignAdequate>)>;

    // ── Operating effectiveness ────────────────────────────────────────────────

    /// Test control operating effectiveness.
    ///
    /// Requires `IcfrDesignAdequate` (design assessment must precede
    /// operating test per PCAOB AS 2201.52).
    /// Returns `IcfrOperatingEffective`.
    ///
    /// Source: PCAOB AS 2201.52.
    fn test_icfr_operating_effectiveness(
        &self,
        design_token: Established<IcfrDesignAdequate>,
        tests: Vec<ControlTestDescriptor>,
    ) -> GaapResult<Established<IcfrOperatingEffective>>;

    // ── Management assertion ──────────────────────────────────────────────────

    /// Prepare and assert the SOX management assertion on ICFR.
    ///
    /// Requires both design and operating-effectiveness proofs.
    /// Returns `ManagementIcfrAssessmentCompleted`.
    ///
    /// Source: SOX §302; SOX §404(a).
    fn assert_management_icfr(
        &self,
        design_token: Established<IcfrDesignAdequate>,
        operating_token: Established<IcfrOperatingEffective>,
        assertion: ManagementAssertionDescriptor,
    ) -> GaapResult<(
        ManagementAssertionDescriptor,
        Established<ManagementIcfrAssessmentCompleted>,
    )>;

    // ── Segregation of duties ─────────────────────────────────────────────────

    /// Verify that recording, custody, and authorization duties are segregated.
    ///
    /// Returns `SegregationOfDutiesEnforced`.
    ///
    /// Source: COSO Internal Control—Integrated Framework; PCAOB AS 2201.
    fn verify_segregation_of_duties(
        &self,
        entity: EntityId,
    ) -> GaapResult<Established<SegregationOfDutiesEnforced>>;
}

// ── Role 2: ICFR status reporter ─────────────────────────────────────────────

/// Orthogonal ICFR assessment status reporter.
pub trait GaapIcfrMeta: Send + Sync {
    /// Return ICFR descriptors for all entities pending assessment.
    fn pending_icfr_assessments(&self) -> BoxFuture<'_, GaapResult<Vec<IcfrDescriptor>>>;

    /// Return ICFR descriptors for all entities with completed assessments.
    fn completed_icfr_assessments(&self) -> BoxFuture<'_, GaapResult<Vec<IcfrDescriptor>>>;
}
