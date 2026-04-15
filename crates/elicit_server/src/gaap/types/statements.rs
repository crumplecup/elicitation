//! Financial statement presentation descriptor types.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::gaap::types::{FinancialPeriod, MonetaryAmount};

// ── Balance sheet ─────────────────────────────────────────────────────────────

/// Top-level balance sheet descriptor.
///
/// The presentation factory asserts `BalanceSheetClassified` when current
/// assets and liabilities are separately presented and the accounting equation
/// holds at period-end.
///
/// Source: ASC 210 — Balance Sheet; ASC 205-10.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BalanceSheetDescriptor {
    /// The as-of date (i.e. `period.end_date`).
    pub period: FinancialPeriod,
    /// Total current assets.
    pub current_assets: MonetaryAmount,
    /// Total non-current assets.
    pub noncurrent_assets: MonetaryAmount,
    /// Total assets (current + non-current).
    pub total_assets: MonetaryAmount,
    /// Total current liabilities.
    pub current_liabilities: MonetaryAmount,
    /// Total non-current liabilities.
    pub noncurrent_liabilities: MonetaryAmount,
    /// Total liabilities.
    pub total_liabilities: MonetaryAmount,
    /// Total stockholders' equity.
    pub total_equity: MonetaryAmount,
    /// Whether a comparative prior-period column is presented.
    pub is_comparative: bool,
    /// Whether the balance sheet uses the classified format (current/non-current split).
    pub is_classified: bool,
}

// ── Income statement ──────────────────────────────────────────────────────────

/// Top-level income statement (statement of operations) descriptor.
///
/// The presentation factory asserts `ComprehensiveIncomeReported` when
/// net income and OCI are both presented.
///
/// Source: ASC 220 — Comprehensive Income; ASC 225 — Income Statement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct IncomeStatementDescriptor {
    /// The reporting period.
    pub period: FinancialPeriod,
    /// Total net revenues.
    pub revenue: MonetaryAmount,
    /// Cost of goods sold / cost of revenues.
    pub cost_of_revenue: MonetaryAmount,
    /// Gross profit (revenue − cost of revenue).
    pub gross_profit: MonetaryAmount,
    /// Total operating expenses (below gross profit line).
    pub operating_expenses: MonetaryAmount,
    /// Operating income.
    pub operating_income: MonetaryAmount,
    /// Net income from continuing operations.
    pub net_income_continuing: MonetaryAmount,
    /// Discontinued operations net of tax, if any.
    pub discontinued_operations: Option<MonetaryAmount>,
    /// Net income (bottom line).
    pub net_income: MonetaryAmount,
    /// Other comprehensive income/(loss), net of tax.
    pub oci: MonetaryAmount,
    /// Comprehensive income (net income + OCI).
    pub comprehensive_income: MonetaryAmount,
    /// Whether a comparative prior-period column is presented.
    pub is_comparative: bool,
}

// ── Cash flow statement ───────────────────────────────────────────────────────

/// Presentation method for the statement of cash flows.
///
/// Source: ASC 230-10-45-1 — Direct or Indirect Method.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum CashFlowMethod {
    /// Direct method: major classes of gross cash receipts and payments.
    Direct,
    /// Indirect method: begins with net income and adjusts for non-cash items.
    Indirect,
}

/// Top-level cash flow statement descriptor.
///
/// The presentation factory asserts `CashFlowStatementPresented` when all three
/// activity sections are presented and the net change reconciles.
///
/// Source: ASC 230 — Statement of Cash Flows.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CashFlowDescriptor {
    /// The reporting period.
    pub period: FinancialPeriod,
    /// Net cash from operating activities.
    pub operating_activities: MonetaryAmount,
    /// Net cash from investing activities.
    pub investing_activities: MonetaryAmount,
    /// Net cash from financing activities.
    pub financing_activities: MonetaryAmount,
    /// Net change in cash (sum of the three sections).
    pub net_change_in_cash: MonetaryAmount,
    /// Beginning cash balance.
    pub beginning_cash: MonetaryAmount,
    /// Ending cash balance.
    pub ending_cash: MonetaryAmount,
    /// Presentation method chosen.
    pub method: CashFlowMethod,
    /// Whether significant non-cash investing/financing activities are disclosed.
    pub noncash_activities_disclosed: bool,
}

// ── EPS descriptor ────────────────────────────────────────────────────────────

/// Earnings per share computation descriptor.
///
/// The presentation factory asserts `BasicEpsDeclared` and optionally
/// `DilutedEpsDeclared`.
///
/// Source: ASC 260 — Earnings Per Share.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct EpsDescriptor {
    /// The reporting period.
    pub period: FinancialPeriod,
    /// Net income attributable to common shareholders.
    pub net_income_to_common: MonetaryAmount,
    /// Weighted-average basic shares outstanding.
    pub weighted_average_basic_shares: f64,
    /// Basic EPS (net_income_to_common / weighted_average_basic_shares).
    pub basic_eps: f64,
    /// Weighted-average diluted shares outstanding, if presented.
    pub weighted_average_diluted_shares: Option<f64>,
    /// Diluted EPS, if presented.
    pub diluted_eps: Option<f64>,
}
