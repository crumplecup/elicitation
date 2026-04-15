//! ASC 200 series — Presentation of Financial Statements.
//!
//! Covers ASC 205–280: balance sheet presentation, comprehensive income,
//! income statement, cash flows, accounting changes, EPS, interim reporting,
//! and segment reporting.
//!
//! Source: FASB ASC 200 series — <https://asc.fasb.org/>

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

    // ── ASC 205 — Presentation of Financial Statements ───────────────────────

    /// Balance sheet classifies assets and liabilities as current vs. noncurrent.
    ///
    /// Source: ASC 210-10-45 — Balance Sheet: Current vs. Noncurrent Classification
    pub struct BalanceSheetClassified;

    /// Comparative prior-period financial statements are presented.
    ///
    /// Source: ASC 205-10-45-2 — Comparative Financial Statements
    pub struct ComparativePeriodPresented;

    /// Going concern evaluation has been performed through the issuance date.
    ///
    /// Source: ASC 205-40-50 — Going Concern
    pub struct GoingConcernEvaluated;

    /// A complete set of financial statements is presented (BS, IS, CF, equity, OCI notes).
    ///
    /// Source: ASC 205-10-45-1 — Complete Set of Financial Statements
    pub struct FinancialStatementsComplete;

    // ── ASC 210 — Balance Sheet ───────────────────────────────────────────────

    /// Asset is properly classified as current (realizable within one year or operating cycle).
    ///
    /// Source: ASC 210-10-45-1 — Current Assets
    pub struct CurrentAssetClassification;

    /// Liability is properly classified as current (due within one year or operating cycle).
    ///
    /// Source: ASC 210-10-45-8 — Current Liabilities
    pub struct CurrentLiabilityClassification;

    /// Assets and liabilities are not netted unless a right of offset legally exists.
    ///
    /// Source: ASC 210-20-45 — Offsetting of Amounts Related to Contracts
    pub struct OffsettingProhibited;

    // ── ASC 220 — Comprehensive Income ───────────────────────────────────────

    /// Other comprehensive income items are presented distinctly from net income.
    ///
    /// Source: ASC 220-10-45 — Presentation of Other Comprehensive Income
    pub struct OciPresentedSeparately;

    /// Total comprehensive income is reported for the period.
    ///
    /// Source: ASC 220-10-45-1 — Comprehensive Income
    pub struct ComprehensiveIncomeReported;

    // ── ASC 225 — Income Statement ────────────────────────────────────────────

    /// Income from continuing operations is shown distinctly.
    ///
    /// Source: ASC 225-10-45 — Income Statement Presentation
    pub struct IncomeFromContinuingOperationsDisclosed;

    /// Discontinued operations are presented separately from continuing operations.
    ///
    /// Source: ASC 205-20-45 — Discontinued Operations
    pub struct DiscontinuedOperationsSeparated;

    /// Unusual or infrequently occurring items are presented within continuing operations.
    ///
    /// Source: ASC 225-20-45-2 — Unusual or Infrequent Items
    pub struct UnusualItemsInContinuingOperations;

    // ── ASC 230 — Cash Flows ──────────────────────────────────────────────────

    /// A statement of cash flows is included in the financial statement set.
    ///
    /// Source: ASC 230-10-45-1 — Statement of Cash Flows Required
    pub struct CashFlowStatementPresented;

    /// Cash receipts and payments are classified as operating activities.
    ///
    /// Source: ASC 230-10-45-12 — Operating Activities
    pub struct OperatingActivitiesClassified;

    /// Cash flows from investing activities are correctly classified.
    ///
    /// Source: ASC 230-10-45-12 — Investing Activities
    pub struct InvestingActivitiesClassified;

    /// Cash flows from financing activities are correctly classified.
    ///
    /// Source: ASC 230-10-45-15 — Financing Activities
    pub struct FinancingActivitiesClassified;

    /// The direct or indirect method for operating activities is disclosed.
    ///
    /// Source: ASC 230-10-45-25 — Indirect Method; ASC 230-10-45-24 — Direct Method
    pub struct CashFlowMethodDisclosed;

    /// Significant noncash investing and financing activities are disclosed in supplemental schedules.
    ///
    /// Source: ASC 230-10-50-3 — Noncash Activities
    pub struct NonCashActivitiesDisclosed;

    // ── ASC 250 — Accounting Changes and Error Corrections ───────────────────

    /// A change in accounting principle is justified by preferability evidence.
    ///
    /// Source: ASC 250-10-45-2 — Change in Accounting Principle
    pub struct AccountingChangeJustified;

    /// Voluntary accounting principle change is applied retrospectively to all periods.
    ///
    /// Source: ASC 250-10-45-5 — Retrospective Application
    pub struct RetrospectiveApplicationApplied;

    /// Mandated accounting principle change is applied prospectively per transition guidance.
    ///
    /// Source: ASC 250-10-45-14 — Prospective Application
    pub struct ProspectiveApplicationApplied;

    /// Prior-period error is corrected by restating the previously issued financial statements.
    ///
    /// Source: ASC 250-10-45-23 — Error Corrections
    pub struct ErrorCorrectionRestated;

    // ── ASC 260 — Earnings Per Share ─────────────────────────────────────────

    /// Basic earnings per share is computed and presented.
    ///
    /// Source: ASC 260-10-45-2 — Basic EPS
    pub struct BasicEpsDeclared;

    /// Diluted earnings per share is computed when dilutive securities exist.
    ///
    /// Source: ASC 260-10-45-16 — Diluted EPS
    pub struct DilutedEpsDeclared;

    /// Weighted-average shares outstanding are correctly computed for EPS denominators.
    ///
    /// Source: ASC 260-10-45-10 — Weighted-Average Shares
    pub struct EpsWeightedAverageSharesCorrect;

    // ── ASC 270 — Interim Reporting ───────────────────────────────────────────

    /// Each interim period is treated as an integral part of the annual period.
    ///
    /// Source: ASC 270-10-45-2 — Integral Approach to Interim Reporting
    pub struct InterimPeriodIntegral;

    /// Seasonal fluctuations in revenue or expenses are disclosed in interim reports.
    ///
    /// Source: ASC 270-10-50-6 — Seasonal Disclosures
    pub struct SeasonalRevenueDisclosed;

    /// Interim income tax expense uses the annualized estimated effective tax rate.
    ///
    /// Source: ASC 740-270-30-5 — Interim Tax Rate
    pub struct InterimTaxRateAnnualized;

    // ── ASC 280 — Segment Reporting ───────────────────────────────────────────

    /// All reportable segments are identified using the 10% quantitative thresholds.
    ///
    /// Source: ASC 280-10-50-12 — Reportable Segments
    pub struct SegmentIdentificationComplete;

    /// Segment totals reconcile to the consolidated financial statement totals.
    ///
    /// Source: ASC 280-10-50-30 — Segment Reconciliation
    pub struct SegmentReconcilesTotal;

    /// Segments are defined by the chief operating decision maker's view of the business.
    ///
    /// Source: ASC 280-10-50-1 — Management Approach
    pub struct ManagementApproachApplied;

    structural_prop!(BalanceSheetClassified, "BalanceSheetClassified");
    structural_prop!(ComparativePeriodPresented, "ComparativePeriodPresented");
    structural_prop!(GoingConcernEvaluated, "GoingConcernEvaluated");
    structural_prop!(FinancialStatementsComplete, "FinancialStatementsComplete");
    structural_prop!(CurrentAssetClassification, "CurrentAssetClassification");
    structural_prop!(
        CurrentLiabilityClassification,
        "CurrentLiabilityClassification"
    );
    structural_prop!(OffsettingProhibited, "OffsettingProhibited");
    structural_prop!(OciPresentedSeparately, "OciPresentedSeparately");
    structural_prop!(ComprehensiveIncomeReported, "ComprehensiveIncomeReported");
    structural_prop!(
        IncomeFromContinuingOperationsDisclosed,
        "IncomeFromContinuingOperationsDisclosed"
    );
    structural_prop!(
        DiscontinuedOperationsSeparated,
        "DiscontinuedOperationsSeparated"
    );
    structural_prop!(
        UnusualItemsInContinuingOperations,
        "UnusualItemsInContinuingOperations"
    );
    structural_prop!(CashFlowStatementPresented, "CashFlowStatementPresented");
    structural_prop!(
        OperatingActivitiesClassified,
        "OperatingActivitiesClassified"
    );
    structural_prop!(
        InvestingActivitiesClassified,
        "InvestingActivitiesClassified"
    );
    structural_prop!(
        FinancingActivitiesClassified,
        "FinancingActivitiesClassified"
    );
    structural_prop!(CashFlowMethodDisclosed, "CashFlowMethodDisclosed");
    structural_prop!(NonCashActivitiesDisclosed, "NonCashActivitiesDisclosed");
    structural_prop!(AccountingChangeJustified, "AccountingChangeJustified");
    structural_prop!(
        RetrospectiveApplicationApplied,
        "RetrospectiveApplicationApplied"
    );
    structural_prop!(
        ProspectiveApplicationApplied,
        "ProspectiveApplicationApplied"
    );
    structural_prop!(ErrorCorrectionRestated, "ErrorCorrectionRestated");
    structural_prop!(BasicEpsDeclared, "BasicEpsDeclared");
    structural_prop!(DilutedEpsDeclared, "DilutedEpsDeclared");
    structural_prop!(
        EpsWeightedAverageSharesCorrect,
        "EpsWeightedAverageSharesCorrect"
    );
    structural_prop!(InterimPeriodIntegral, "InterimPeriodIntegral");
    structural_prop!(SeasonalRevenueDisclosed, "SeasonalRevenueDisclosed");
    structural_prop!(InterimTaxRateAnnualized, "InterimTaxRateAnnualized");
    structural_prop!(
        SegmentIdentificationComplete,
        "SegmentIdentificationComplete"
    );
    structural_prop!(SegmentReconcilesTotal, "SegmentReconcilesTotal");
    structural_prop!(ManagementApproachApplied, "ManagementApproachApplied");
}

pub use emit_impls::{
    AccountingChangeJustified, BalanceSheetClassified, BasicEpsDeclared, CashFlowMethodDisclosed,
    CashFlowStatementPresented, ComparativePeriodPresented, ComprehensiveIncomeReported,
    CurrentAssetClassification, CurrentLiabilityClassification, DilutedEpsDeclared,
    DiscontinuedOperationsSeparated, EpsWeightedAverageSharesCorrect, ErrorCorrectionRestated,
    FinancialStatementsComplete, FinancingActivitiesClassified, GoingConcernEvaluated,
    IncomeFromContinuingOperationsDisclosed, InterimPeriodIntegral, InterimTaxRateAnnualized,
    InvestingActivitiesClassified, ManagementApproachApplied, NonCashActivitiesDisclosed,
    OciPresentedSeparately, OffsettingProhibited, OperatingActivitiesClassified,
    ProspectiveApplicationApplied, RetrospectiveApplicationApplied, SeasonalRevenueDisclosed,
    SegmentIdentificationComplete, SegmentReconcilesTotal, UnusualItemsInContinuingOperations,
};
