//! `GaapPeriodFactory`  — accrual/deferral recording factory (Role 1a).
//! `GaapPeriodReporter` — period-close status reporter (Role 2).

use futures::future::BoxFuture;

use crate::gaap::errors::GaapResult;
use crate::gaap::internal_controls::PeriodEndCloseControlsApplied;
use crate::gaap::temporal::{
    AccrualRecordedAtPeriodEnd, DeferralReleasedInEarnedPeriod, TransactionCutoffRespected,
};
use crate::gaap::types::{AccrualDescriptor, DeferralDescriptor, FinancialPeriod, PpeDescriptor};
use elicitation::Established;

// ── Role 1a: accrual/deferral factory ────────────────────────────────────────

/// Factory for accrual-basis period-end adjustments.
///
/// Source: ASC 420 / accrual-basis accounting (ASC 105-10-05-2);
///         ASC 270 — Interim Reporting; ASC 360-10-35 — Depreciation.
pub trait GaapPeriodFactory: Send + Sync {
    /// Record a period-end accrual.
    ///
    /// Returns `AccrualRecordedAtPeriodEnd` — the accrual date falls within
    /// the covered period.
    ///
    /// Source: ASC 420; accrual accounting §ASC 105-10-05-2.
    fn record_accrual(
        &self,
        accrual: AccrualDescriptor,
    ) -> GaapResult<(AccrualDescriptor, Established<AccrualRecordedAtPeriodEnd>)>;

    /// Record a deferral and verify that the earned-date falls in the correct period.
    ///
    /// Returns `DeferralReleasedInEarnedPeriod`.
    ///
    /// Source: accrual-basis deferral model; ASC 606 deferred-revenue treatment.
    fn record_deferral(
        &self,
        deferral: DeferralDescriptor,
    ) -> GaapResult<(
        DeferralDescriptor,
        Established<DeferralReleasedInEarnedPeriod>,
    )>;

    /// Enforce transaction cut-off for a period.
    ///
    /// Returns `TransactionCutoffRespected` — all transactions are recorded in the
    /// period they occurred, with none shifted across the period boundary.
    ///
    /// Source: accrual-basis cut-off; ASC 250 — Accounting Changes and Error Corrections.
    fn enforce_cutoff(
        &self,
        period: FinancialPeriod,
    ) -> GaapResult<Established<TransactionCutoffRespected>>;

    /// Compute and record period depreciation for a PP&E asset.
    ///
    /// Returns `DepreciationAccumulatesCorrectly` wrapped in the prop token.
    ///
    /// Source: ASC 360-10-35 — Depreciation.
    fn record_depreciation(
        &self,
        asset: PpeDescriptor,
        period: FinancialPeriod,
    ) -> GaapResult<PpeDescriptor>;

    /// Close the period.
    ///
    /// Returns `PeriodEndCloseControlsApplied` — all sub-ledgers are posted and
    /// the period-close control checklist has been applied.
    ///
    /// Source: period-close procedures; ASC 205-10-45-1; PCAOB AS 2201.
    fn close_period(
        &self,
        period: FinancialPeriod,
    ) -> GaapResult<Established<PeriodEndCloseControlsApplied>>;
}

// ── Role 2: period-close status reporter ─────────────────────────────────────

/// Orthogonal period-close status reporter.
///
/// Queries the backend to determine which periods are open, closed, or pending.
pub trait GaapPeriodReporter: Send + Sync {
    /// Return all periods whose close process has not been completed.
    fn open_periods(&self) -> BoxFuture<'_, GaapResult<Vec<FinancialPeriod>>>;

    /// Return all periods that have been closed.
    fn closed_periods(&self) -> BoxFuture<'_, GaapResult<Vec<FinancialPeriod>>>;
}
