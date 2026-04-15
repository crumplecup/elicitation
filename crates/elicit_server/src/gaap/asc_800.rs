//! ASC 800 series — Business Combinations, Consolidations, Derivatives,
//! Fair Value, Financial Instruments, Foreign Currency, Interest, and Leases.
//!
//! Covers ASC 805, 810, 815, 820, 825, 830, 835, and 842.
//!
//! Source: FASB ASC 800 series — <https://asc.fasb.org/>

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

    // ── ASC 805 — Business Combinations ──────────────────────────────────────

    /// The acquisition method is applied to all business combinations.
    ///
    /// Source: ASC 805-10-25-1 — Acquisition Method
    pub struct AcquisitionMethodApplied;

    /// The acquisition date is properly identified as the date control is obtained.
    ///
    /// Source: ASC 805-10-25-6 — Acquisition Date
    pub struct AcquisitionDateIdentified;

    /// All identifiable assets acquired and liabilities assumed are recognized at fair value.
    ///
    /// Source: ASC 805-20-25-1 — Identifiable Assets and Liabilities
    pub struct IdentifiableAssetsRecognized;

    /// Goodwill or a bargain purchase gain is recognized correctly.
    ///
    /// Source: ASC 805-30-25-1 — Goodwill and Bargain Purchase
    pub struct GoodwillOrBargainPurchaseRecognized;

    /// Contingent consideration is measured at fair value at the acquisition date.
    ///
    /// Source: ASC 805-30-25-5 — Contingent Consideration
    pub struct ContingentConsiderationMeasured;

    /// Measurement period adjustments are recorded as of the acquisition date (retrospective).
    ///
    /// Source: ASC 805-10-25-14 — Measurement Period Adjustments
    pub struct MeasurementPeriodAdjustmentsRecorded;

    // ── ASC 810 — Consolidation ───────────────────────────────────────────────

    /// Consolidation criteria are evaluated for all entities in which the reporting entity has an interest.
    ///
    /// Source: ASC 810-10-15 — Consolidation Scope
    pub struct ConsolidationCriteriaEvaluated;

    /// VIE consolidation analysis is performed; primary beneficiary is determined.
    ///
    /// Source: ASC 810-10-25-38 — Variable Interest Entity Consolidation
    pub struct VieConsolidationAssessed;

    /// Voting interest model is applied for entities that are not VIEs.
    ///
    /// Source: ASC 810-10-15-8 — Voting Interest Model
    pub struct VotingInterestModelApplied;

    /// Noncontrolling interest is recognized and measured at fair value at the acquisition date.
    ///
    /// Source: ASC 810-10-45-16 — Noncontrolling Interest Recognition
    pub struct NoncontrollingInterestRecognized;

    // ── ASC 815 — Derivatives and Hedging ─────────────────────────────────────

    /// All derivatives are recognized on the balance sheet at fair value.
    ///
    /// Source: ASC 815-10-25-1 — Derivatives Recognition
    pub struct DerivativeRecognizedAtFairValue;

    /// Hedge designation and risk management strategy are formally documented at inception.
    ///
    /// Source: ASC 815-20-25-3 — Hedge Documentation
    pub struct HedgeDesignationDocumented;

    /// Fair value hedge gains and losses are recognized in current-period earnings.
    ///
    /// Source: ASC 815-20-35-1 — Fair Value Hedge Accounting
    pub struct FairValueHedgeAccountedFor;

    /// Cash flow hedge gains and losses on the effective portion are recognized in OCI.
    ///
    /// Source: ASC 815-30-35-1 — Cash Flow Hedge Accounting
    pub struct CashFlowHedgeAccountedFor;

    /// Hedge effectiveness is assessed both prospectively and retrospectively.
    ///
    /// Source: ASC 815-20-25-3(g) — Hedge Effectiveness Assessment
    pub struct HedgeEffectivenessAssessed;

    /// Net investment hedge of a foreign operation is accounted for correctly.
    ///
    /// Source: ASC 815-35-25-1 — Net Investment Hedge
    pub struct NetInvestmentHedgeApplied;

    // ── ASC 820 — Fair Value Measurement ──────────────────────────────────────

    /// The exit price notion (sale or transfer to market participants) is applied consistently.
    ///
    /// Source: ASC 820-10-20 — Fair Value Definition
    pub struct FairValueExitPriceApplied;

    /// The fair value hierarchy (Level 1, 2, 3) is applied and inputs are prioritized appropriately.
    ///
    /// Source: ASC 820-10-35-37 — Fair Value Hierarchy
    pub struct FairValueHierarchyApplied;

    /// Level 3 inputs, valuation techniques, and unobservable input sensitivities are disclosed.
    ///
    /// Source: ASC 820-10-50-2(bbb) — Level 3 Disclosures
    pub struct Level3InputsDisclosed;

    /// Recurring fair value measurements are performed each reporting period.
    ///
    /// Source: ASC 820-10-50-1 — Recurring Fair Value Disclosures
    pub struct FairValueOnRecurringBasis;

    /// Nonrecurring fair value measurements are performed when a triggering event occurs.
    ///
    /// Source: ASC 820-10-50-1 — Nonrecurring Fair Value Disclosures
    pub struct FairValueOnNonrecurringBasis;

    // ── ASC 825 — Financial Instruments ──────────────────────────────────────

    /// Fair value of all financial instruments is disclosed in the notes.
    ///
    /// Source: ASC 825-10-50-10 — Financial Instruments Fair Value Disclosure
    pub struct FinancialInstrumentFairValueDisclosed;

    /// Fair value option election is documented at inception and the rationale disclosed.
    ///
    /// Source: ASC 825-10-25-1 — Fair Value Option
    pub struct FairValueOptionElectionDocumented;

    // ── ASC 830 — Foreign Currency Matters ────────────────────────────────────

    /// Functional currency is determined for each entity based on the economic environment.
    ///
    /// Source: ASC 830-10-45-2 — Functional Currency Determination
    pub struct FunctionalCurrencyDetermined;

    /// Remeasurement is applied for entities whose books are not in the functional currency.
    ///
    /// Source: ASC 830-10-45-17 — Remeasurement Method
    pub struct RemeasurementApplied;

    /// Translation adjustment from functional to reporting currency is recognized in OCI.
    ///
    /// Source: ASC 830-30-45-12 — Translation Adjustment in OCI
    pub struct TranslationAdjustmentInOci;

    /// Foreign currency transaction gain or loss is recognized in current-period earnings.
    ///
    /// Source: ASC 830-20-35-1 — Transaction Gains and Losses
    pub struct ForeignCurrencyTransactionGainLossRecognized;

    // ── ASC 835 — Interest ────────────────────────────────────────────────────

    /// Interest is capitalized during the construction period of a qualifying asset.
    ///
    /// Source: ASC 835-20-25-1 — Capitalization of Interest
    pub struct InterestCapitalized;

    /// Effective interest method is applied to amortize debt discount, premium, and issuance costs.
    ///
    /// Source: ASC 835-30-35-2 — Effective Interest Method (Debt)
    pub struct EffectiveInterestMethodApplied;

    // ── ASC 842 — Leases ──────────────────────────────────────────────────────

    /// An arrangement has been assessed to determine whether it contains a lease.
    ///
    /// Source: ASC 842-10-15 — Identifying a Lease
    pub struct LeaseIdentified;

    /// Lease is classified as operating or finance (lessee) or operating, sales-type, or direct-finance (lessor).
    ///
    /// Source: ASC 842-20-25 — Lessee Classification; ASC 842-30-25 — Lessor Classification
    pub struct LeaseClassified;

    /// Right-of-use asset is recognized for all operating and finance leases.
    ///
    /// Source: ASC 842-20-25-1 — Right-of-Use Asset Recognition
    pub struct RouAssetRecognized;

    /// Lease liability is recognized at the present value of future lease payments.
    ///
    /// Source: ASC 842-20-30-1 — Lease Liability Measurement
    pub struct LeaseLiabilityRecognized;

    /// The rate implicit in the lease or the lessee's incremental borrowing rate is used as the discount rate.
    ///
    /// Source: ASC 842-20-30-3 — Discount Rate for Leases
    pub struct LeaseDiscountRateDetermined;

    /// Lease term includes renewal and termination option periods that are reasonably certain.
    ///
    /// Source: ASC 842-20-30-1 — Lease Term
    pub struct LeaseTermDetermined;

    /// Variable lease payments not based on an index or rate are excluded from the lease liability.
    ///
    /// Source: ASC 842-20-25-6 — Variable Lease Payments
    pub struct VariableLeasePmtAccountedFor;

    /// Short-term lease exemption (12 months or less) is applied consistently by asset class.
    ///
    /// Source: ASC 842-20-25-2 — Short-Term Lease Exemption
    pub struct ShortTermLeaseExemptionApplied;

    structural_prop!(AcquisitionMethodApplied, "AcquisitionMethodApplied");
    structural_prop!(AcquisitionDateIdentified, "AcquisitionDateIdentified");
    structural_prop!(IdentifiableAssetsRecognized, "IdentifiableAssetsRecognized");
    structural_prop!(
        GoodwillOrBargainPurchaseRecognized,
        "GoodwillOrBargainPurchaseRecognized"
    );
    structural_prop!(
        ContingentConsiderationMeasured,
        "ContingentConsiderationMeasured"
    );
    structural_prop!(
        MeasurementPeriodAdjustmentsRecorded,
        "MeasurementPeriodAdjustmentsRecorded"
    );
    structural_prop!(
        ConsolidationCriteriaEvaluated,
        "ConsolidationCriteriaEvaluated"
    );
    structural_prop!(VieConsolidationAssessed, "VieConsolidationAssessed");
    structural_prop!(VotingInterestModelApplied, "VotingInterestModelApplied");
    structural_prop!(
        NoncontrollingInterestRecognized,
        "NoncontrollingInterestRecognized"
    );
    structural_prop!(
        DerivativeRecognizedAtFairValue,
        "DerivativeRecognizedAtFairValue"
    );
    structural_prop!(HedgeDesignationDocumented, "HedgeDesignationDocumented");
    structural_prop!(FairValueHedgeAccountedFor, "FairValueHedgeAccountedFor");
    structural_prop!(CashFlowHedgeAccountedFor, "CashFlowHedgeAccountedFor");
    structural_prop!(HedgeEffectivenessAssessed, "HedgeEffectivenessAssessed");
    structural_prop!(NetInvestmentHedgeApplied, "NetInvestmentHedgeApplied");
    structural_prop!(FairValueExitPriceApplied, "FairValueExitPriceApplied");
    structural_prop!(FairValueHierarchyApplied, "FairValueHierarchyApplied");
    structural_prop!(Level3InputsDisclosed, "Level3InputsDisclosed");
    structural_prop!(FairValueOnRecurringBasis, "FairValueOnRecurringBasis");
    structural_prop!(FairValueOnNonrecurringBasis, "FairValueOnNonrecurringBasis");
    structural_prop!(
        FinancialInstrumentFairValueDisclosed,
        "FinancialInstrumentFairValueDisclosed"
    );
    structural_prop!(
        FairValueOptionElectionDocumented,
        "FairValueOptionElectionDocumented"
    );
    structural_prop!(FunctionalCurrencyDetermined, "FunctionalCurrencyDetermined");
    structural_prop!(RemeasurementApplied, "RemeasurementApplied");
    structural_prop!(TranslationAdjustmentInOci, "TranslationAdjustmentInOci");
    structural_prop!(
        ForeignCurrencyTransactionGainLossRecognized,
        "ForeignCurrencyTransactionGainLossRecognized"
    );
    structural_prop!(InterestCapitalized, "InterestCapitalized");
    structural_prop!(
        EffectiveInterestMethodApplied,
        "EffectiveInterestMethodApplied"
    );
    structural_prop!(LeaseIdentified, "LeaseIdentified");
    structural_prop!(LeaseClassified, "LeaseClassified");
    structural_prop!(RouAssetRecognized, "RouAssetRecognized");
    structural_prop!(LeaseLiabilityRecognized, "LeaseLiabilityRecognized");
    structural_prop!(LeaseDiscountRateDetermined, "LeaseDiscountRateDetermined");
    structural_prop!(LeaseTermDetermined, "LeaseTermDetermined");
    structural_prop!(VariableLeasePmtAccountedFor, "VariableLeasePmtAccountedFor");
    structural_prop!(
        ShortTermLeaseExemptionApplied,
        "ShortTermLeaseExemptionApplied"
    );
}

pub use emit_impls::{
    AcquisitionDateIdentified, AcquisitionMethodApplied, CashFlowHedgeAccountedFor,
    ConsolidationCriteriaEvaluated, ContingentConsiderationMeasured,
    DerivativeRecognizedAtFairValue, EffectiveInterestMethodApplied, FairValueExitPriceApplied,
    FairValueHedgeAccountedFor, FairValueHierarchyApplied, FairValueOnNonrecurringBasis,
    FairValueOnRecurringBasis, FairValueOptionElectionDocumented,
    FinancialInstrumentFairValueDisclosed, ForeignCurrencyTransactionGainLossRecognized,
    FunctionalCurrencyDetermined, GoodwillOrBargainPurchaseRecognized, HedgeDesignationDocumented,
    HedgeEffectivenessAssessed, IdentifiableAssetsRecognized, InterestCapitalized, LeaseClassified,
    LeaseDiscountRateDetermined, LeaseIdentified, LeaseLiabilityRecognized, LeaseTermDetermined,
    Level3InputsDisclosed, MeasurementPeriodAdjustmentsRecorded, NetInvestmentHedgeApplied,
    NoncontrollingInterestRecognized, RemeasurementApplied, RouAssetRecognized,
    ShortTermLeaseExemptionApplied, TranslationAdjustmentInOci, VariableLeasePmtAccountedFor,
    VieConsolidationAssessed, VotingInterestModelApplied,
};
