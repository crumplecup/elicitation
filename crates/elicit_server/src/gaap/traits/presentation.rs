//! `GaapPresentationFactory` — financial statement section factory (Role 1b).
//!
//! This is a **section factory** (Role 1b in the three-role taxonomy): it
//! requires evidence bundles of prior proof tokens as preconditions and mints
//! aggregate proof tokens for each completed financial statement section.
//! No presentation can be completed without first constructing the underlying
//! accounting entries and balance proofs.

use crate::gaap::asc_200::{
    BalanceSheetClassified, BasicEpsDeclared, CashFlowMethodDisclosed, CashFlowStatementPresented,
    ComparativePeriodPresented, ComprehensiveIncomeReported, CurrentAssetClassification,
    CurrentLiabilityClassification, FinancialStatementsComplete, FinancingActivitiesClassified,
    IncomeFromContinuingOperationsDisclosed, InvestingActivitiesClassified,
    NonCashActivitiesDisclosed, OciPresentedSeparately, OperatingActivitiesClassified,
};
use crate::gaap::errors::GaapResult;
use crate::gaap::mathematical::{
    AccountingEquationHolds, CashFlowReconciles, NetIncomeAggregation, TrialBalanceBalances,
};
use crate::gaap::types::{
    BalanceSheetDescriptor, CashFlowDescriptor, EpsDescriptor, IncomeStatementDescriptor,
};
use elicitation::Established;

// ── Evidence bundles for section factories ────────────────────────────────────

/// Evidence required to present a classified balance sheet.
///
/// All underlying accounting propositions must be established before the
/// section factory can assert `BalanceSheetClassified`.
///
/// Source: ASC 210 — Balance Sheet; ASC 205-10.
pub struct BalanceSheetEvidence {
    /// Accounting equation A = L + E holds at period-end.
    pub accounting_equation: Established<AccountingEquationHolds>,
    /// Trial balance is in balance.
    pub trial_balance: Established<TrialBalanceBalances>,
    /// Current assets have been separately classified.
    pub current_assets_classified: Established<CurrentAssetClassification>,
    /// Current liabilities have been separately classified.
    pub current_liabilities_classified: Established<CurrentLiabilityClassification>,
    /// A comparative prior-period column is presented.
    pub comparative_period: Established<ComparativePeriodPresented>,
}

/// Evidence required to present the income statement with comprehensive income.
///
/// Source: ASC 220 — Comprehensive Income; ASC 225 — Income Statement.
pub struct IncomeStatementEvidence {
    /// Net income has been correctly aggregated.
    pub net_income: Established<NetIncomeAggregation>,
    /// OCI is presented separately from net income.
    pub oci_separate: Established<OciPresentedSeparately>,
    /// Income from continuing operations is disclosed.
    pub continuing_ops_disclosed: Established<IncomeFromContinuingOperationsDisclosed>,
}

/// Evidence required to present the statement of cash flows.
///
/// Source: ASC 230 — Statement of Cash Flows.
pub struct CashFlowEvidence {
    /// Operating activities section is classified.
    pub operating_classified: Established<OperatingActivitiesClassified>,
    /// Investing activities section is classified.
    pub investing_classified: Established<InvestingActivitiesClassified>,
    /// Financing activities section is classified.
    pub financing_classified: Established<FinancingActivitiesClassified>,
    /// The presentation method (direct or indirect) is disclosed.
    pub method_disclosed: Established<CashFlowMethodDisclosed>,
    /// Non-cash investing/financing activities are disclosed.
    pub noncash_disclosed: Established<NonCashActivitiesDisclosed>,
    /// Net change in cash reconciles to the beginning/ending balances.
    pub reconciles: Established<CashFlowReconciles>,
}

/// Evidence required to assert that the full set of financial statements is complete.
///
/// Source: ASC 205-10-45 — Presentation of Financial Statements.
pub struct FullFinancialStatementsEvidence {
    /// Balance sheet has been presented and proven complete.
    pub balance_sheet: Established<BalanceSheetClassified>,
    /// Income statement with comprehensive income has been presented.
    pub income_statement: Established<ComprehensiveIncomeReported>,
    /// Statement of cash flows has been presented.
    pub cash_flows: Established<CashFlowStatementPresented>,
}

// ── Role 1b: financial statement section factory ──────────────────────────────

/// Section factory for assembling financial statement presentations.
///
/// Each method takes an evidence bundle of upstream proof tokens plus the raw
/// descriptor and returns the same descriptor with a section-level proof token.
/// Evidence bundles make it impossible to call a presentation method without
/// first establishing all required underlying accounting propositions.
///
/// Source: ASC 205 — Presentation of Financial Statements.
pub trait GaapPresentationFactory: Send + Sync {
    /// Assemble and assert a classified balance sheet.
    ///
    /// Requires `BalanceSheetEvidence` (accounting equation, trial balance,
    /// current classifications, comparative period).
    /// Returns `BalanceSheetClassified`.
    ///
    /// Source: ASC 210 — Balance Sheet.
    fn present_balance_sheet(
        &self,
        evidence: BalanceSheetEvidence,
        balance_sheet: BalanceSheetDescriptor,
    ) -> GaapResult<(BalanceSheetDescriptor, Established<BalanceSheetClassified>)>;

    /// Assemble and assert the income statement with comprehensive income.
    ///
    /// Requires `IncomeStatementEvidence` (net income, OCI separation,
    /// continuing-operations disclosure).
    /// Returns `ComprehensiveIncomeReported`.
    ///
    /// Source: ASC 220-10-45 — Presenting Comprehensive Income.
    fn present_income_statement(
        &self,
        evidence: IncomeStatementEvidence,
        income_statement: IncomeStatementDescriptor,
    ) -> GaapResult<(
        IncomeStatementDescriptor,
        Established<ComprehensiveIncomeReported>,
    )>;

    /// Assemble and assert the statement of cash flows.
    ///
    /// Requires `CashFlowEvidence` (all three activity sections, method
    /// disclosure, non-cash disclosure, reconciliation).
    /// Returns `CashFlowStatementPresented`.
    ///
    /// Source: ASC 230 — Statement of Cash Flows.
    fn present_cash_flows(
        &self,
        evidence: CashFlowEvidence,
        cash_flows: CashFlowDescriptor,
    ) -> GaapResult<(CashFlowDescriptor, Established<CashFlowStatementPresented>)>;

    /// Compute and present earnings per share.
    ///
    /// Returns `BasicEpsDeclared`.  If diluted data is provided, also asserts
    /// correctness of the diluted EPS calculation.
    ///
    /// Source: ASC 260 — Earnings Per Share.
    fn present_eps(
        &self,
        eps: EpsDescriptor,
    ) -> GaapResult<(EpsDescriptor, Established<BasicEpsDeclared>)>;

    /// Assert that the full set of financial statements is complete.
    ///
    /// Requires `FullFinancialStatementsEvidence` — balance sheet, income
    /// statement, and cash flows must all have been individually proven.
    /// Returns `FinancialStatementsComplete`.
    ///
    /// Source: ASC 205-10-45-1.
    fn assert_financial_statements_complete(
        &self,
        evidence: FullFinancialStatementsEvidence,
    ) -> GaapResult<Established<FinancialStatementsComplete>>;
}
