//! `GaapFairValueFactory` — fair value measurement factory (Role 1a).
//! `GaapLeaseFactory`     — ASC 842 lease accounting factory (Role 1a).
//! `GaapDerivativeFactory` — ASC 815 derivative factory (Role 1a).

use crate::gaap::asc_800::{
    DerivativeRecognizedAtFairValue, FairValueExitPriceApplied, FairValueHierarchyApplied,
    HedgeDesignationDocumented, LeaseClassified, LeaseLiabilityRecognized, RouAssetRecognized,
};
use crate::gaap::errors::GaapResult;
use crate::gaap::types::{DerivativeDescriptor, FairValueDescriptor, LeaseDescriptor};
use elicitation::Established;

// ── Role 1a: fair value factory ───────────────────────────────────────────────

/// Factory for fair value measurements under ASC 820.
///
/// Source: ASC 820 — Fair Value Measurement.
pub trait GaapFairValueFactory: Send + Sync {
    /// Apply the exit-price principle and classify within the fair value hierarchy.
    ///
    /// Returns `FairValueExitPriceApplied` and `FairValueHierarchyApplied`.
    ///
    /// Source: ASC 820-10-35-2 (exit price); ASC 820-10-35-37 (hierarchy).
    fn measure_fair_value(
        &self,
        measurement: FairValueDescriptor,
    ) -> GaapResult<(
        FairValueDescriptor,
        Established<FairValueExitPriceApplied>,
        Established<FairValueHierarchyApplied>,
    )>;
}

// ── Role 1a: lease factory ────────────────────────────────────────────────────

/// Factory for lessee accounting under ASC 842.
///
/// The factory encodes the sequential recognition steps for a lease: the
/// right-of-use asset and lease liability must both be established before the
/// lease can be presented as a balance-sheet item.
///
/// Source: ASC 842 — Leases.
pub trait GaapLeaseFactory: Send + Sync {
    /// Step 1: Classify the lease as finance, operating, or short-term.
    ///
    /// Returns `LeaseClassified`.
    ///
    /// Source: ASC 842-20-25-2 — Classification Criteria.
    fn classify_lease(
        &self,
        lease: LeaseDescriptor,
    ) -> GaapResult<(LeaseDescriptor, Established<LeaseClassified>)>;

    /// Step 2: Recognize the lease liability at present value of future payments.
    ///
    /// Requires `LeaseClassified` (Step 1 token).
    /// Returns `LeaseLiabilityRecognized`.
    ///
    /// Source: ASC 842-20-30-1.
    fn recognize_lease_liability(
        &self,
        classification_token: Established<LeaseClassified>,
        lease: LeaseDescriptor,
    ) -> GaapResult<(LeaseDescriptor, Established<LeaseLiabilityRecognized>)>;

    /// Step 3: Recognize the right-of-use asset.
    ///
    /// Requires `LeaseLiabilityRecognized` (Step 2 token).
    /// Returns `RouAssetRecognized`.
    ///
    /// Source: ASC 842-20-30-1.
    fn recognize_rou_asset(
        &self,
        liability_token: Established<LeaseLiabilityRecognized>,
        lease: LeaseDescriptor,
    ) -> GaapResult<(LeaseDescriptor, Established<RouAssetRecognized>)>;
}

// ── Role 1a: derivative factory ───────────────────────────────────────────────

/// Factory for derivative and hedging accounting under ASC 815.
///
/// Source: ASC 815 — Derivatives and Hedging.
pub trait GaapDerivativeFactory: Send + Sync {
    /// Recognize a derivative at fair value on the balance sheet.
    ///
    /// Returns `DerivativeRecognizedAtFairValue`.
    ///
    /// Source: ASC 815-10-35-1.
    fn recognize_derivative(
        &self,
        derivative: DerivativeDescriptor,
    ) -> GaapResult<(
        DerivativeDescriptor,
        Established<DerivativeRecognizedAtFairValue>,
    )>;

    /// Document hedge designation for a qualifying hedging relationship.
    ///
    /// Requires `DerivativeRecognizedAtFairValue` as a precondition.
    /// Returns `HedgeDesignationDocumented`.
    ///
    /// Source: ASC 815-20-55-1 — Designation and Documentation.
    fn document_hedge_designation(
        &self,
        derivative_token: Established<DerivativeRecognizedAtFairValue>,
        derivative: DerivativeDescriptor,
    ) -> GaapResult<(
        DerivativeDescriptor,
        Established<HedgeDesignationDocumented>,
    )>;
}
