//! ASC 700 series — Stock Compensation and Income Taxes.
//!
//! Covers ASC 718 (share-based payment awards) and ASC 740 (income taxes,
//! deferred tax, uncertain tax positions).
//!
//! Source: FASB ASC 718, 740 — <https://asc.fasb.org/>

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

    // ── ASC 718 — Compensation — Stock Compensation ───────────────────────────

    /// Grant-date fair value of a share-based award is measured using an appropriate valuation model.
    ///
    /// Source: ASC 718-10-30-3 — Grant-Date Fair Value Measurement
    pub struct StockCompensationFairValueMeasured;

    /// Compensation cost is recognized over the requisite service period (vesting period).
    ///
    /// Source: ASC 718-10-35-2 — Recognition of Compensation Cost
    pub struct StockCompensationRecognizedOverVesting;

    /// Graded vesting tranches are treated separately when the election is made.
    ///
    /// Source: ASC 718-10-35-8 — Graded Vesting
    pub struct GradedVestingApplied;

    /// Forfeitures are estimated at grant date (or recognized when they occur under the election).
    ///
    /// Source: ASC 718-10-35-3 — Forfeiture Accounting
    pub struct ForfeitureAccountingApplied;

    /// Performance condition probability is assessed each reporting period to determine compensation expense.
    ///
    /// Source: ASC 718-10-25-20 — Performance Conditions
    pub struct PerformanceConditionAssessed;

    /// Market condition is incorporated into the grant-date fair value (not reassessed).
    ///
    /// Source: ASC 718-10-25-20 — Market Conditions
    pub struct MarketConditionIncludedInFairValue;

    /// Modification of a share-based award triggers measurement of incremental fair value.
    ///
    /// Source: ASC 718-20-35-3 — Award Modifications
    pub struct AwardModificationAccountedFor;

    /// Share-based award tax effects are recognized through income tax expense in the income statement.
    ///
    /// Source: ASC 718-740-45-7 — Tax Effects of Share-Based Awards
    pub struct StockAwardTaxEffectInEarnings;

    // ── ASC 740 — Income Taxes ────────────────────────────────────────────────

    /// Deferred tax asset is recognized for deductible temporary differences and carryforwards.
    ///
    /// Source: ASC 740-10-25-2 — Deferred Tax Assets
    pub struct DeferredTaxAssetRecognized;

    /// Deferred tax liability is recognized for taxable temporary differences.
    ///
    /// Source: ASC 740-10-25-3 — Deferred Tax Liabilities
    pub struct DeferredTaxLiabilityRecognized;

    /// Valuation allowance is assessed against the deferred tax asset when realization is not more likely than not.
    ///
    /// Source: ASC 740-10-30-18 — Valuation Allowance Assessment
    pub struct ValuationAllowanceAssessed;

    /// The "more likely than not" standard is applied to determine DTA realizability.
    ///
    /// Source: ASC 740-10-30-5 — More-Likely-Than-Not Standard
    pub struct MoreLikelyThanNotStandardApplied;

    /// Uncertain tax positions are evaluated using the recognition and measurement thresholds.
    ///
    /// Source: ASC 740-10-25-6 — Uncertain Tax Positions
    pub struct UncertainTaxPositionEvaluated;

    /// Effective tax rate reconciliation is disclosed, including all material reconciling items.
    ///
    /// Source: ASC 740-10-50-12 — Effective Tax Rate Reconciliation
    pub struct EffectiveTaxRateDisclosed;

    /// Deferred tax assets and liabilities are classified as noncurrent on the balance sheet.
    ///
    /// Source: ASC 740-10-45-4 — Noncurrent Classification of Deferred Taxes
    pub struct DeferredTaxNoncurrentClassified;

    /// Intraperiod tax allocation is applied to allocate income tax among continuing operations, OCI, and equity.
    ///
    /// Source: ASC 740-20-45 — Intraperiod Tax Allocation
    pub struct IntraperiodTaxAllocationApplied;

    structural_prop!(
        StockCompensationFairValueMeasured,
        "StockCompensationFairValueMeasured"
    );
    structural_prop!(
        StockCompensationRecognizedOverVesting,
        "StockCompensationRecognizedOverVesting"
    );
    structural_prop!(GradedVestingApplied, "GradedVestingApplied");
    structural_prop!(ForfeitureAccountingApplied, "ForfeitureAccountingApplied");
    structural_prop!(PerformanceConditionAssessed, "PerformanceConditionAssessed");
    structural_prop!(
        MarketConditionIncludedInFairValue,
        "MarketConditionIncludedInFairValue"
    );
    structural_prop!(
        AwardModificationAccountedFor,
        "AwardModificationAccountedFor"
    );
    structural_prop!(
        StockAwardTaxEffectInEarnings,
        "StockAwardTaxEffectInEarnings"
    );
    structural_prop!(DeferredTaxAssetRecognized, "DeferredTaxAssetRecognized");
    structural_prop!(
        DeferredTaxLiabilityRecognized,
        "DeferredTaxLiabilityRecognized"
    );
    structural_prop!(ValuationAllowanceAssessed, "ValuationAllowanceAssessed");
    structural_prop!(
        MoreLikelyThanNotStandardApplied,
        "MoreLikelyThanNotStandardApplied"
    );
    structural_prop!(
        UncertainTaxPositionEvaluated,
        "UncertainTaxPositionEvaluated"
    );
    structural_prop!(EffectiveTaxRateDisclosed, "EffectiveTaxRateDisclosed");
    structural_prop!(
        DeferredTaxNoncurrentClassified,
        "DeferredTaxNoncurrentClassified"
    );
    structural_prop!(
        IntraperiodTaxAllocationApplied,
        "IntraperiodTaxAllocationApplied"
    );
}

pub use emit_impls::{
    AwardModificationAccountedFor, DeferredTaxAssetRecognized, DeferredTaxLiabilityRecognized,
    DeferredTaxNoncurrentClassified, EffectiveTaxRateDisclosed, ForfeitureAccountingApplied,
    GradedVestingApplied, IntraperiodTaxAllocationApplied, MarketConditionIncludedInFairValue,
    MoreLikelyThanNotStandardApplied, PerformanceConditionAssessed, StockAwardTaxEffectInEarnings,
    StockCompensationFairValueMeasured, StockCompensationRecognizedOverVesting,
    UncertainTaxPositionEvaluated, ValuationAllowanceAssessed,
};
