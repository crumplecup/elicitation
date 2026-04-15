//! Disclosure requirement propositions.
//!
//! Each proposition represents a specific footnote or supplemental disclosure
//! that GAAP requires when applicable. These complement the recognition and
//! measurement props in the ASC series modules.
//!
//! Source: FASB ASC §50 disclosure subsections across the codification

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

    // ── General and cross-cutting disclosures ─────────────────────────────────

    /// Summary of significant accounting policies note is included.
    ///
    /// Source: ASC 235-10-50-1 — Notes to Financial Statements
    pub struct SignificantAccountingPoliciesDisclosed;

    /// All related-party transactions are identified and disclosed.
    ///
    /// Source: ASC 850-10-50-1 — Related Party Disclosures
    pub struct RelatedPartyTransactionsDisclosed;

    /// Subsequent events are reviewed through the issuance date and material events disclosed.
    ///
    /// Source: ASC 855-10-50-2 — Subsequent Events Disclosure
    pub struct SubsequentEventsDisclosed;

    /// Commitments and contingencies are disclosed in the balance sheet caption and notes.
    ///
    /// Source: ASC 440-10-50 — Commitments Disclosure; ASC 450-20-50 — Contingencies
    pub struct CommitmentsAndContingenciesDisclosed;

    /// Recently issued accounting standards and their expected impact are disclosed.
    ///
    /// Source: ASC 250-10-50-1 — New Accounting Standards
    pub struct NewAccountingStandardsDisclosed;

    /// Concentrations of credit risk are disclosed for all significant counterparties.
    ///
    /// Source: ASC 825-10-50-21 — Concentrations of Credit Risk
    pub struct ConcentrationRisksDisclosed;

    /// Liquidity risk, available funding sources, and any going concern indicators are disclosed.
    ///
    /// Source: ASC 205-40-50 — Going Concern; ASC 275-10-50 — Liquidity Risks
    pub struct LiquidityRisksDisclosed;

    // ── Revenue and contract disclosures ──────────────────────────────────────

    /// Revenue recognition policy, including the nature of performance obligations, is disclosed.
    ///
    /// Source: ASC 606-10-50-1 — Revenue Recognition Policy
    pub struct RevenueRecognitionPolicyNote;

    /// Revenue is disaggregated into categories that depict economic factors.
    ///
    /// Source: ASC 606-10-50-5 — Disaggregation of Revenue
    pub struct RevenueDisaggregationNote;

    /// Contract asset and liability opening and closing balances are disclosed.
    ///
    /// Source: ASC 606-10-50-8 — Contract Balances
    pub struct ContractBalanceNote;

    /// Remaining performance obligations and expected recognition timing are disclosed.
    ///
    /// Source: ASC 606-10-50-13 — Remaining Performance Obligations
    pub struct RemainingPerformanceObligationNote;

    // ── Asset and investment disclosures ──────────────────────────────────────

    /// Goodwill rollforward by reportable segment is disclosed.
    ///
    /// Source: ASC 350-20-50-1 — Goodwill Rollforward
    pub struct GoodwillRollforwardDisclosed;

    /// Intangible assets subject to amortization and indefinite-lived are separately disclosed.
    ///
    /// Source: ASC 350-30-50-1 — Intangible Assets Disclosure
    pub struct IntangibleAssetsDisclosed;

    /// Depreciation method and range of useful lives for each PP&E class are disclosed.
    ///
    /// Source: ASC 360-10-50-1 — PP&E Disclosure
    pub struct PpeDepreciationPolicyDisclosed;

    // ── Debt and equity disclosures ───────────────────────────────────────────

    /// Debt covenant terms, required ratios, and compliance status are disclosed.
    ///
    /// Source: ASC 470-10-50-1 — Debt Covenants
    pub struct DebtCovenantsDisclosed;

    /// Aggregate annual maturities of long-term debt for the next five years are disclosed.
    ///
    /// Source: ASC 470-10-50-1 — Debt Maturity Schedule
    pub struct DebtMaturityScheduleDisclosed;

    /// Preferred stock terms (liquidation preference, dividend rate, conversion rights) are disclosed.
    ///
    /// Source: ASC 505-10-50-4 — Preferred Stock Disclosures
    pub struct PreferredStockDisclosures;

    // ── Income tax disclosures ────────────────────────────────────────────────

    /// Deferred tax asset and liability components are disclosed.
    ///
    /// Source: ASC 740-10-50-2 — Deferred Tax Components
    pub struct DeferredTaxComponentsDisclosed;

    /// Effective tax rate reconciliation from statutory rate to reported rate is disclosed.
    ///
    /// Source: ASC 740-10-50-12 — Effective Tax Rate Reconciliation
    pub struct EffectiveTaxRateReconciliationDisclosed;

    /// Unrecognized tax benefits and the potential impact on the effective tax rate are disclosed.
    ///
    /// Source: ASC 740-10-50-15 — Uncertain Tax Positions Disclosure
    pub struct UncertainTaxBenefitsDisclosed;

    /// Material tax jurisdictions subject to examination are disclosed.
    ///
    /// Source: ASC 740-10-50-15 — Tax Jurisdictions
    pub struct TaxJurisdictionsDisclosed;

    // ── Pension and post-retirement benefit disclosures ───────────────────────

    /// Pension and OPEB benefit obligations and plan assets are disclosed.
    ///
    /// Source: ASC 715-20-50 — Defined Benefit Plan Disclosures
    pub struct PensionObligationDisclosed;

    /// Net periodic benefit cost components are disclosed.
    ///
    /// Source: ASC 715-20-50-1(h) — Net Periodic Benefit Cost
    pub struct NetPeriodicBenefitCostDisclosed;

    // ── Derivative and hedging disclosures ────────────────────────────────────

    /// Derivative instruments, hedging strategy, and fair value amounts are disclosed.
    ///
    /// Source: ASC 815-10-50-1 — Derivatives and Hedging Disclosures
    pub struct DerivativeAndHedgingDisclosed;

    /// Tabular disclosure of derivatives' fair value and gain/loss by category is included.
    ///
    /// Source: ASC 815-10-50-1A — Quantitative Derivative Disclosures
    pub struct DerivativeFairValueTableDisclosed;

    // ── Lease disclosures ─────────────────────────────────────────────────────

    /// Lease supplemental quantitative disclosures (cost, cash paid, ROU assets) are included.
    ///
    /// Source: ASC 842-20-50-4 — Quantitative Lease Disclosures
    pub struct LeaseQuantitativeDisclosed;

    /// Future undiscounted lease payments reconciled to the lease liability are disclosed.
    ///
    /// Source: ASC 842-20-50-6 — Maturity Analysis of Lease Liabilities
    pub struct LeaseLiabilityMaturityDisclosed;

    // ── Stock compensation disclosures ────────────────────────────────────────

    /// Stock compensation plan description, assumptions, and expense are disclosed.
    ///
    /// Source: ASC 718-10-50-1 — Stock Compensation Disclosures
    pub struct StockCompensationPlanDisclosed;

    /// Unrecognized compensation cost and expected recognition period are disclosed.
    ///
    /// Source: ASC 718-10-50-2(i) — Unrecognized Compensation Cost
    pub struct UnrecognizedCompensationCostDisclosed;

    // ── Segment disclosures ───────────────────────────────────────────────────

    /// Revenue, profit or loss, and total assets by reportable segment are disclosed.
    ///
    /// Source: ASC 280-10-50-22 — Segment Information
    pub struct SegmentInformationDisclosed;

    /// Entity-wide disclosures (products, geographic areas, major customers) are included.
    ///
    /// Source: ASC 280-10-50-38 — Entity-Wide Disclosures
    pub struct EntityWideDisclosuresIncluded;

    // ── Fair value disclosures ────────────────────────────────────────────────

    /// Valuation techniques and inputs used for recurring and nonrecurring FV measurements are disclosed.
    ///
    /// Source: ASC 820-10-50-2 — Fair Value Measurement Disclosures
    pub struct FairValueMeasurementMethodsDisclosed;

    /// Rollforward of Level 3 fair value measurements is disclosed.
    ///
    /// Source: ASC 820-10-50-2(d) — Level 3 Rollforward
    pub struct Level3FairValueRollforwardDisclosed;

    // ── Interim disclosures ───────────────────────────────────────────────────

    /// Material changes from the prior annual report are disclosed in interim financial statements.
    ///
    /// Source: ASC 270-10-50 — Interim Disclosures
    pub struct InterimSignificantChangesDisclosed;

    structural_prop!(
        SignificantAccountingPoliciesDisclosed,
        "SignificantAccountingPoliciesDisclosed"
    );
    structural_prop!(
        RelatedPartyTransactionsDisclosed,
        "RelatedPartyTransactionsDisclosed"
    );
    structural_prop!(SubsequentEventsDisclosed, "SubsequentEventsDisclosed");
    structural_prop!(
        CommitmentsAndContingenciesDisclosed,
        "CommitmentsAndContingenciesDisclosed"
    );
    structural_prop!(
        NewAccountingStandardsDisclosed,
        "NewAccountingStandardsDisclosed"
    );
    structural_prop!(ConcentrationRisksDisclosed, "ConcentrationRisksDisclosed");
    structural_prop!(LiquidityRisksDisclosed, "LiquidityRisksDisclosed");
    structural_prop!(RevenueRecognitionPolicyNote, "RevenueRecognitionPolicyNote");
    structural_prop!(RevenueDisaggregationNote, "RevenueDisaggregationNote");
    structural_prop!(ContractBalanceNote, "ContractBalanceNote");
    structural_prop!(
        RemainingPerformanceObligationNote,
        "RemainingPerformanceObligationNote"
    );
    structural_prop!(GoodwillRollforwardDisclosed, "GoodwillRollforwardDisclosed");
    structural_prop!(IntangibleAssetsDisclosed, "IntangibleAssetsDisclosed");
    structural_prop!(
        PpeDepreciationPolicyDisclosed,
        "PpeDepreciationPolicyDisclosed"
    );
    structural_prop!(DebtCovenantsDisclosed, "DebtCovenantsDisclosed");
    structural_prop!(
        DebtMaturityScheduleDisclosed,
        "DebtMaturityScheduleDisclosed"
    );
    structural_prop!(PreferredStockDisclosures, "PreferredStockDisclosures");
    structural_prop!(
        DeferredTaxComponentsDisclosed,
        "DeferredTaxComponentsDisclosed"
    );
    structural_prop!(
        EffectiveTaxRateReconciliationDisclosed,
        "EffectiveTaxRateReconciliationDisclosed"
    );
    structural_prop!(
        UncertainTaxBenefitsDisclosed,
        "UncertainTaxBenefitsDisclosed"
    );
    structural_prop!(TaxJurisdictionsDisclosed, "TaxJurisdictionsDisclosed");
    structural_prop!(PensionObligationDisclosed, "PensionObligationDisclosed");
    structural_prop!(
        NetPeriodicBenefitCostDisclosed,
        "NetPeriodicBenefitCostDisclosed"
    );
    structural_prop!(
        DerivativeAndHedgingDisclosed,
        "DerivativeAndHedgingDisclosed"
    );
    structural_prop!(
        DerivativeFairValueTableDisclosed,
        "DerivativeFairValueTableDisclosed"
    );
    structural_prop!(LeaseQuantitativeDisclosed, "LeaseQuantitativeDisclosed");
    structural_prop!(
        LeaseLiabilityMaturityDisclosed,
        "LeaseLiabilityMaturityDisclosed"
    );
    structural_prop!(
        StockCompensationPlanDisclosed,
        "StockCompensationPlanDisclosed"
    );
    structural_prop!(
        UnrecognizedCompensationCostDisclosed,
        "UnrecognizedCompensationCostDisclosed"
    );
    structural_prop!(SegmentInformationDisclosed, "SegmentInformationDisclosed");
    structural_prop!(
        EntityWideDisclosuresIncluded,
        "EntityWideDisclosuresIncluded"
    );
    structural_prop!(
        FairValueMeasurementMethodsDisclosed,
        "FairValueMeasurementMethodsDisclosed"
    );
    structural_prop!(
        Level3FairValueRollforwardDisclosed,
        "Level3FairValueRollforwardDisclosed"
    );
    structural_prop!(
        InterimSignificantChangesDisclosed,
        "InterimSignificantChangesDisclosed"
    );
}

pub use emit_impls::{
    CommitmentsAndContingenciesDisclosed, ConcentrationRisksDisclosed, ContractBalanceNote,
    DebtCovenantsDisclosed, DebtMaturityScheduleDisclosed, DeferredTaxComponentsDisclosed,
    DerivativeAndHedgingDisclosed, DerivativeFairValueTableDisclosed,
    EffectiveTaxRateReconciliationDisclosed, EntityWideDisclosuresIncluded,
    FairValueMeasurementMethodsDisclosed, GoodwillRollforwardDisclosed, IntangibleAssetsDisclosed,
    InterimSignificantChangesDisclosed, LeaseLiabilityMaturityDisclosed,
    LeaseQuantitativeDisclosed, Level3FairValueRollforwardDisclosed, LiquidityRisksDisclosed,
    NetPeriodicBenefitCostDisclosed, NewAccountingStandardsDisclosed, PensionObligationDisclosed,
    PpeDepreciationPolicyDisclosed, PreferredStockDisclosures, RelatedPartyTransactionsDisclosed,
    RemainingPerformanceObligationNote, RevenueDisaggregationNote, RevenueRecognitionPolicyNote,
    SegmentInformationDisclosed, SignificantAccountingPoliciesDisclosed,
    StockCompensationPlanDisclosed, SubsequentEventsDisclosed, TaxJurisdictionsDisclosed,
    UncertainTaxBenefitsDisclosed, UnrecognizedCompensationCostDisclosed,
};
