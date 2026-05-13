//! `GaapTaxFactory` — income-tax accounting factory (Role 1a).

use crate::gaap::asc_700::{
    DeferredTaxAssetRecognized, DeferredTaxLiabilityRecognized, IntraperiodTaxAllocationApplied,
    ValuationAllowanceAssessed,
};
use crate::gaap::errors::GaapResult;
use crate::gaap::mathematical::TaxRateReconciles;
use crate::gaap::types::{DeferredTaxDescriptor, MonetaryAmount, TaxRateDescriptor};
use elicitation::Established;

// ── Role 1a: income-tax factory ───────────────────────────────────────────────

/// Factory for computing and asserting income-tax accounting positions.
///
/// Source: ASC 740 — Income Taxes.
pub trait GaapTaxFactory: Send + Sync {
    // ── Deferred taxes ────────────────────────────────────────────────────────

    /// Recognize a deferred tax asset from a deductible temporary difference.
    ///
    /// Returns `DeferredTaxAssetRecognized`.
    ///
    /// Source: ASC 740-10-25-2.
    fn recognize_deferred_tax_asset(
        &self,
        deferred_tax: DeferredTaxDescriptor,
    ) -> GaapResult<(
        DeferredTaxDescriptor,
        Established<DeferredTaxAssetRecognized>,
    )>;

    /// Recognize a deferred tax liability from a taxable temporary difference.
    ///
    /// Returns `DeferredTaxLiabilityRecognized`.
    ///
    /// Source: ASC 740-10-25-2.
    fn recognize_deferred_tax_liability(
        &self,
        deferred_tax: DeferredTaxDescriptor,
    ) -> GaapResult<(
        DeferredTaxDescriptor,
        Established<DeferredTaxLiabilityRecognized>,
    )>;

    /// Assess and record a valuation allowance against a deferred tax asset.
    ///
    /// Returns `ValuationAllowanceAssessed`.
    ///
    /// Source: ASC 740-10-30-5 — More-Likely-Than-Not Criterion.
    fn assess_valuation_allowance(
        &self,
        deferred_tax: DeferredTaxDescriptor,
    ) -> GaapResult<(
        DeferredTaxDescriptor,
        Established<ValuationAllowanceAssessed>,
    )>;

    // ── Rate reconciliation ───────────────────────────────────────────────────

    /// Verify the effective-tax-rate reconciliation from statutory to effective rate.
    ///
    /// Returns `TaxRateReconciles`.
    ///
    /// Source: ASC 740-10-50-12 — Rate Reconciliation Disclosure.
    fn verify_tax_rate_reconciliation(
        &self,
        rate: TaxRateDescriptor,
    ) -> GaapResult<(TaxRateDescriptor, Established<TaxRateReconciles>)>;

    // ── Intraperiod allocation ────────────────────────────────────────────────

    /// Perform intraperiod tax allocation — allocate income tax expense among
    /// continuing operations, OCI, and discontinued operations.
    ///
    /// Returns `IntraperiodTaxAllocationApplied`.
    ///
    /// Source: ASC 740-20-45 — Intraperiod Tax Allocation.
    fn allocate_intraperiod_tax(
        &self,
        continuing_ops_tax: MonetaryAmount,
        oci_tax: MonetaryAmount,
        discontinued_ops_tax: Option<MonetaryAmount>,
    ) -> GaapResult<Established<IntraperiodTaxAllocationApplied>>;
}
