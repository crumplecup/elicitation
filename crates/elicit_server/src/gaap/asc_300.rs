//! ASC 300 series — Assets.
//!
//! Covers ASC 310–360: receivables, investments in debt and equity securities,
//! inventory, intangibles/goodwill, and property, plant, and equipment.
//!
//! Source: FASB ASC 300 series — <https://asc.fasb.org/>

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

    // ── ASC 310 — Receivables ─────────────────────────────────────────────────

    /// Receivable is carried at amortized cost.
    ///
    /// Source: ASC 310-20-35 — Amortized Cost
    pub struct ReceivableRecordedAtAmortizedCost;

    /// Allowance for credit losses is estimated under the CECL model.
    ///
    /// Source: ASC 326-20-30 — Current Expected Credit Loss (CECL)
    pub struct AllowanceForCreditLossEstimated;

    /// Troubled debt restructuring is identified and accounted for under ASC 310-40.
    ///
    /// Source: ASC 310-40-15 — Troubled Debt Restructuring by Creditors
    pub struct TroubledDebtRestructuringIdentified;

    /// Factoring transaction is properly determined to be a sale or secured borrowing.
    ///
    /// Source: ASC 860-10-40 — Transfers and Servicing — Sale vs. Secured Borrowing
    pub struct FactoredReceivableSaleAccountedFor;

    /// Loan origination fees and costs are deferred and amortized using the effective interest method.
    ///
    /// Source: ASC 310-20-35-2 — Loan Fees and Costs
    pub struct LoanOriginationFeesDeferred;

    // ── ASC 320 — Investments — Debt Securities ───────────────────────────────

    /// Debt security is classified as trading, available-for-sale, or held-to-maturity.
    ///
    /// Source: ASC 320-10-25 — Classification of Debt Securities
    pub struct DebtSecurityClassified;

    /// Trading security is carried at fair value with unrealized gains/losses in earnings.
    ///
    /// Source: ASC 320-10-35-1 — Trading Securities
    pub struct TradingSecurityAtFairValue;

    /// Available-for-sale security is carried at fair value with unrealized gains/losses in OCI.
    ///
    /// Source: ASC 320-10-35-1 — Available-for-Sale Securities
    pub struct AfsSecurityAtFairValue;

    /// Held-to-maturity security is carried at amortized cost (intent and ability to hold).
    ///
    /// Source: ASC 320-10-35-2 — Held-to-Maturity Securities
    pub struct HtmSecurityAtAmortizedCost;

    /// Impairment review is performed on AFS and HTM securities each reporting period.
    ///
    /// Source: ASC 320-10-35-34 — Impairment of Investments
    pub struct InvestmentImpairmentReviewed;

    // ── ASC 321 — Investments — Equity Securities ─────────────────────────────

    /// Equity investment is measured at fair value through earnings.
    ///
    /// Source: ASC 321-10-35-1 — Equity Securities at Fair Value
    pub struct EquitySecurityAtFairValue;

    /// Equity method is applied when the investor has significant influence (generally 20–50%).
    ///
    /// Source: ASC 323-10-15-8 — Equity Method Investments
    pub struct EquityMethodApplied;

    /// All equity method investees are identified and not omitted.
    ///
    /// Source: ASC 323-10-15 — Scope of Equity Method
    pub struct EquityMethodInvesteeIdentified;

    /// Equity method investment is tested for impairment when indicators exist.
    ///
    /// Source: ASC 323-10-35-32 — Impairment of Equity Method Investments
    pub struct EquityMethodImpairmentAssessed;

    // ── ASC 330 — Inventory ───────────────────────────────────────────────────

    /// Inventory is carried at the lower of cost or net realizable value.
    ///
    /// Source: ASC 330-10-35-1 — Lower of Cost or Net Realizable Value
    pub struct InventoryAtLowerOfCostOrNrv;

    /// The cost flow assumption (FIFO, LIFO, weighted-average) is disclosed.
    ///
    /// Source: ASC 330-10-30-9 — Cost Flow Assumptions
    pub struct CostFlowAssumptionDisclosed;

    /// Write-down to NRV is recognized in the period the decline occurs and is not reversed.
    ///
    /// Source: ASC 330-10-35-14 — Inventory Write-Downs
    pub struct InventoryWriteDownRecognized;

    /// LIFO reserve is disclosed when LIFO is used.
    ///
    /// Source: ASC 330-10-50-1 — LIFO Reserve Disclosure
    pub struct LifoReserveDisclosed;

    // ── ASC 350 — Intangibles — Goodwill and Other ────────────────────────────

    /// Goodwill is tested for impairment at least annually or when indicators exist.
    ///
    /// Source: ASC 350-20-35-28 — Goodwill Impairment Testing
    pub struct GoodwillAnnuallyTested;

    /// Goodwill impairment loss is recognized in the period identified.
    ///
    /// Source: ASC 350-20-35-8a — Goodwill Impairment Recognition
    pub struct GoodwillImpairmentRecognized;

    /// Indefinite-lived intangible assets are tested for impairment at least annually.
    ///
    /// Source: ASC 350-30-35-18 — Indefinite-Lived Intangibles Impairment
    pub struct IndefiniteLifeIntangibleTested;

    /// Finite-lived intangible assets are amortized over their estimated useful lives.
    ///
    /// Source: ASC 350-30-35-6 — Amortization of Finite-Lived Intangibles
    pub struct FiniteLifeIntangibleAmortized;

    /// Intangible asset useful life is reassessed each reporting period.
    ///
    /// Source: ASC 350-30-35-9 — Reassessment of Useful Life
    pub struct IntangibleUsefulLifeReassessed;

    /// Internal-use software development costs in the application development stage are capitalized.
    ///
    /// Source: ASC 350-40-25-1 — Internal-Use Software
    pub struct InternalUseSoftwareCostCapitalized;

    // ── ASC 360 — Property, Plant, and Equipment ──────────────────────────────

    /// PP&E is carried at cost less accumulated depreciation.
    ///
    /// Source: ASC 360-10-30-1 — Initial Measurement of PP&E
    pub struct PpeCarriedAtCost;

    /// Depreciation method is disclosed for each major class of PP&E.
    ///
    /// Source: ASC 360-10-50-1 — Depreciation Method Disclosure
    pub struct DepreciationMethodDisclosed;

    /// Useful life is estimated for each PP&E class and applied consistently.
    ///
    /// Source: ASC 360-10-35-4 — Useful Life Estimation
    pub struct UsefulLifeEstimated;

    /// Long-lived asset is tested for recoverability when impairment indicators exist.
    ///
    /// Source: ASC 360-10-35-17 — Impairment of Long-Lived Assets
    pub struct LongLivedAssetImpairmentTested;

    /// Gain or loss on disposal of PP&E is recognized in the period of disposal.
    ///
    /// Source: ASC 360-10-40-5 — Disposal of Long-Lived Assets
    pub struct DisposalGainLossRecognized;

    /// Asset retirement obligation is recognized at fair value when incurred (see also ASC 410).
    ///
    /// Source: ASC 410-20-25-1 — Asset Retirement Obligations
    pub struct PpeAroRecognized;

    structural_prop!(
        ReceivableRecordedAtAmortizedCost,
        "ReceivableRecordedAtAmortizedCost"
    );
    structural_prop!(
        AllowanceForCreditLossEstimated,
        "AllowanceForCreditLossEstimated"
    );
    structural_prop!(
        TroubledDebtRestructuringIdentified,
        "TroubledDebtRestructuringIdentified"
    );
    structural_prop!(
        FactoredReceivableSaleAccountedFor,
        "FactoredReceivableSaleAccountedFor"
    );
    structural_prop!(LoanOriginationFeesDeferred, "LoanOriginationFeesDeferred");
    structural_prop!(DebtSecurityClassified, "DebtSecurityClassified");
    structural_prop!(TradingSecurityAtFairValue, "TradingSecurityAtFairValue");
    structural_prop!(AfsSecurityAtFairValue, "AfsSecurityAtFairValue");
    structural_prop!(HtmSecurityAtAmortizedCost, "HtmSecurityAtAmortizedCost");
    structural_prop!(InvestmentImpairmentReviewed, "InvestmentImpairmentReviewed");
    structural_prop!(EquitySecurityAtFairValue, "EquitySecurityAtFairValue");
    structural_prop!(EquityMethodApplied, "EquityMethodApplied");
    structural_prop!(
        EquityMethodInvesteeIdentified,
        "EquityMethodInvesteeIdentified"
    );
    structural_prop!(
        EquityMethodImpairmentAssessed,
        "EquityMethodImpairmentAssessed"
    );
    structural_prop!(InventoryAtLowerOfCostOrNrv, "InventoryAtLowerOfCostOrNrv");
    structural_prop!(CostFlowAssumptionDisclosed, "CostFlowAssumptionDisclosed");
    structural_prop!(InventoryWriteDownRecognized, "InventoryWriteDownRecognized");
    structural_prop!(LifoReserveDisclosed, "LifoReserveDisclosed");
    structural_prop!(GoodwillAnnuallyTested, "GoodwillAnnuallyTested");
    structural_prop!(GoodwillImpairmentRecognized, "GoodwillImpairmentRecognized");
    structural_prop!(
        IndefiniteLifeIntangibleTested,
        "IndefiniteLifeIntangibleTested"
    );
    structural_prop!(
        FiniteLifeIntangibleAmortized,
        "FiniteLifeIntangibleAmortized"
    );
    structural_prop!(
        IntangibleUsefulLifeReassessed,
        "IntangibleUsefulLifeReassessed"
    );
    structural_prop!(
        InternalUseSoftwareCostCapitalized,
        "InternalUseSoftwareCostCapitalized"
    );
    structural_prop!(PpeCarriedAtCost, "PpeCarriedAtCost");
    structural_prop!(DepreciationMethodDisclosed, "DepreciationMethodDisclosed");
    structural_prop!(UsefulLifeEstimated, "UsefulLifeEstimated");
    structural_prop!(
        LongLivedAssetImpairmentTested,
        "LongLivedAssetImpairmentTested"
    );
    structural_prop!(DisposalGainLossRecognized, "DisposalGainLossRecognized");
    structural_prop!(PpeAroRecognized, "PpeAroRecognized");
}

pub use emit_impls::{
    AfsSecurityAtFairValue, AllowanceForCreditLossEstimated, CostFlowAssumptionDisclosed,
    DebtSecurityClassified, DepreciationMethodDisclosed, DisposalGainLossRecognized,
    EquityMethodApplied, EquityMethodImpairmentAssessed, EquityMethodInvesteeIdentified,
    EquitySecurityAtFairValue, FactoredReceivableSaleAccountedFor, FiniteLifeIntangibleAmortized,
    GoodwillAnnuallyTested, GoodwillImpairmentRecognized, HtmSecurityAtAmortizedCost,
    IndefiniteLifeIntangibleTested, IntangibleUsefulLifeReassessed,
    InternalUseSoftwareCostCapitalized, InventoryAtLowerOfCostOrNrv, InventoryWriteDownRecognized,
    InvestmentImpairmentReviewed, LifoReserveDisclosed, LoanOriginationFeesDeferred,
    LongLivedAssetImpairmentTested, PpeAroRecognized, PpeCarriedAtCost,
    ReceivableRecordedAtAmortizedCost, TradingSecurityAtFairValue,
    TroubledDebtRestructuringIdentified, UsefulLifeEstimated,
};
