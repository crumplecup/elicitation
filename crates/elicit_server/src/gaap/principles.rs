//! Core GAAP principle propositions.
//!
//! Each proposition is a zero-cost proof marker establishing compliance with a
//! specific Generally Accepted Accounting Principle. All ASC references are to
//! the FASB Accounting Standards Codification.
//!
//! Source: FASB Accounting Standards Codification — <https://asc.fasb.org/>

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

    // ── Core assumptions (ASC 105) ────────────────────────────────────────────

    /// Every transaction records equal and opposite entries; Assets = Liabilities + Equity.
    ///
    /// Pre-ASC foundational requirement. Required by FASB for GAAP compliance.
    pub struct DoubleEntryBookkeeping;

    /// Transactions are recorded when they occur, not when cash changes hands.
    ///
    /// Source: ASC 606-10-25-1 — Revenue Recognition
    pub struct AccrualBasis;

    /// All transactions are measured in a single stable monetary unit (e.g., USD cents).
    ///
    /// Source: ASC 105 — Generally Accepted Accounting Principles
    pub struct MonetaryUnitAssumption;

    /// The reporting entity is separate and distinct from its owners.
    ///
    /// Source: ASC 105 — Generally Accepted Accounting Principles
    pub struct EconomicEntityAssumption;

    /// Financial statements are prepared for a discrete time period.
    ///
    /// Source: ASC 105, ASC 270 — Interim Reporting
    pub struct TimePeriodAssumption;

    /// Financial statements are prepared assuming the entity will continue operating.
    ///
    /// Source: ASC 105, ASC 205-40 — Going Concern
    pub struct GoingConcernAssumption;

    // ── Recognition and measurement principles ────────────────────────────────

    /// Revenue and associated expenses are recognized in the same accounting period.
    ///
    /// Source: ASC 606-10-25-23 — Revenue Recognition
    pub struct MatchingPrinciple;

    /// Assets are recorded at original acquisition cost, not current market value.
    ///
    /// Source: ASC 820-10 — Fair Value Measurement (defines exceptions)
    pub struct HistoricalCostPrinciple;

    /// All information that could affect financial decisions is disclosed.
    ///
    /// Source: ASC 235 — Notes to Financial Statements
    pub struct FullDisclosurePrinciple;

    /// Revenue is recognized when earned and realizable.
    ///
    /// Source: ASC 606-10-25 — Revenue Recognition
    pub struct RevenueRecognitionPrinciple;

    // ── Qualitative principles ────────────────────────────────────────────────

    /// When uncertain, choose options resulting in lower income or smaller asset values.
    ///
    /// Source: ASC 250, ASC 450 — Contingencies
    pub struct ConservatismPrinciple;

    /// Significant items affecting financial decisions are reported.
    ///
    /// Source: ASC 250 — Accounting Changes and Error Corrections; SEC SAB 99
    pub struct MaterialityPrinciple;

    /// Accounting methods are applied consistently across reporting periods.
    ///
    /// Source: ASC 250-10-45 — Accounting Changes and Error Corrections
    pub struct ConsistencyPrinciple;

    /// Financial information is free from bias and represents economic reality.
    ///
    /// Source: FASB Concepts Statement No. 8 — Conceptual Framework
    pub struct NeutralityPrinciple;

    /// Economic substance governs accounting treatment, not legal form.
    ///
    /// Source: FASB Concepts Statement No. 8 — Conceptual Framework
    pub struct SubstanceOverFormPrinciple;

    structural_prop!(DoubleEntryBookkeeping, "DoubleEntryBookkeeping");
    structural_prop!(AccrualBasis, "AccrualBasis");
    structural_prop!(MonetaryUnitAssumption, "MonetaryUnitAssumption");
    structural_prop!(EconomicEntityAssumption, "EconomicEntityAssumption");
    structural_prop!(TimePeriodAssumption, "TimePeriodAssumption");
    structural_prop!(GoingConcernAssumption, "GoingConcernAssumption");
    structural_prop!(MatchingPrinciple, "MatchingPrinciple");
    structural_prop!(HistoricalCostPrinciple, "HistoricalCostPrinciple");
    structural_prop!(FullDisclosurePrinciple, "FullDisclosurePrinciple");
    structural_prop!(RevenueRecognitionPrinciple, "RevenueRecognitionPrinciple");
    structural_prop!(ConservatismPrinciple, "ConservatismPrinciple");
    structural_prop!(MaterialityPrinciple, "MaterialityPrinciple");
    structural_prop!(ConsistencyPrinciple, "ConsistencyPrinciple");
    structural_prop!(NeutralityPrinciple, "NeutralityPrinciple");
    structural_prop!(SubstanceOverFormPrinciple, "SubstanceOverFormPrinciple");
}

pub use emit_impls::{
    AccrualBasis, ConservatismPrinciple, ConsistencyPrinciple, DoubleEntryBookkeeping,
    EconomicEntityAssumption, FullDisclosurePrinciple, GoingConcernAssumption,
    HistoricalCostPrinciple, MatchingPrinciple, MaterialityPrinciple, MonetaryUnitAssumption,
    NeutralityPrinciple, RevenueRecognitionPrinciple, SubstanceOverFormPrinciple,
    TimePeriodAssumption,
};
