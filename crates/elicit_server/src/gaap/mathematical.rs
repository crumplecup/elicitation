//! Mathematical invariant propositions.
//!
//! Each proposition represents a quantitative relationship that must hold
//! across the financial statements at every period-end. These are the
//! logical skeleton of double-entry bookkeeping — the expressions most
//! amenable to formal verification.
//!
//! Source: Pre-ASC foundational arithmetic requirements; FASB Concepts Statements

// ── Fundamental identities ────────────────────────────────────────────────

/// Assets = Liabilities + Stockholders' Equity at every period-end.
///
/// Source: Double-entry bookkeeping foundational equation; ASC 210 — Balance Sheet
#[derive(elicitation::Prop)]
pub struct AccountingEquationHolds;

/// The sum of all debits equals the sum of all credits in every journal entry.
///
/// Source: Double-entry bookkeeping — debit/credit symmetry
#[derive(elicitation::Prop)]
pub struct DebitEqualsCreditPerEntry;

/// The trial balance total debits equal total credits before adjustments.
///
/// Source: Double-entry bookkeeping — trial balance
#[derive(elicitation::Prop)]
pub struct TrialBalanceBalances;

// ── Income and equity rollforwards ────────────────────────────────────────

/// RE_end = RE_begin + Net Income − Dividends Declared.
///
/// Source: ASC 505-10 — Retained Earnings
#[derive(elicitation::Prop)]
pub struct RetainedEarningsRollforward;

/// Net income equals the sum of revenue minus all expenses and tax.
///
/// Source: ASC 225 — Income Statement aggregation
#[derive(elicitation::Prop)]
pub struct NetIncomeAggregation;

/// Ending AOCI = Beginning AOCI + Current OCI − Reclassifications to Income.
///
/// Source: ASC 220-10 — Comprehensive Income
#[derive(elicitation::Prop)]
pub struct OciRollforward;

// ── Cash flow reconciliation ──────────────────────────────────────────────

/// Net change in cash per the statement of cash flows equals the change in the cash balance on the balance sheet.
///
/// Source: ASC 230-10-45 — Cash Flows Reconciliation
#[derive(elicitation::Prop)]
pub struct CashFlowReconciles;

// ── Earnings per share invariants ─────────────────────────────────────────

/// Basic EPS numerator equals net income attributable to common stockholders after preferred dividends.
///
/// Source: ASC 260-10-45-11 — EPS Numerator
#[derive(elicitation::Prop)]
pub struct EpsNumeratorCorrect;

/// Basic EPS denominator equals the correctly computed weighted-average shares outstanding.
///
/// Source: ASC 260-10-45-10 — EPS Denominator
#[derive(elicitation::Prop)]
pub struct EpsDenominatorCorrect;

/// Diluted EPS ≤ Basic EPS (anti-dilution constraint prevents increasing EPS by adding dilutive securities).
///
/// Source: ASC 260-10-45-17 — Anti-Dilution Constraint
#[derive(elicitation::Prop)]
pub struct DilutedEpsNoMoreThanBasic;

// ── Asset rollforwards ────────────────────────────────────────────────────

/// Ending Inventory = Beginning Inventory + Purchases − Cost of Goods Sold.
///
/// Source: ASC 330-10 — Inventory rollforward identity
#[derive(elicitation::Prop)]
pub struct InventoryRollforward;

/// AR_end = AR_begin + Credit Sales − Collections − Write-Offs.
///
/// Source: ASC 310-10 — Accounts receivable rollforward identity
#[derive(elicitation::Prop)]
pub struct ReceivablesRollforward;

/// Allowance_end = Allowance_begin + Provision − Write-Offs + Recoveries.
///
/// Source: ASC 326-20 — Allowance for credit losses rollforward
#[derive(elicitation::Prop)]
pub struct AllowanceForCreditLossRollforward;

/// Goodwill_end = Goodwill_begin + Acquisitions − Impairment − Disposals ± FX.
///
/// Source: ASC 350-20-50 — Goodwill rollforward
#[derive(elicitation::Prop)]
pub struct GoodwillRollforward;

// ── Debt and lease invariants ─────────────────────────────────────────────

/// Lease liability equals the present value of future minimum lease payments discounted at the lease rate.
///
/// Source: ASC 842-20-30-1 — Lease Liability Present Value
#[derive(elicitation::Prop)]
pub struct LeaseLiabilityPvCorrect;

/// The amortization schedule advances the carrying value by exactly the effective interest each period.
///
/// Source: ASC 835-30-35-2 — Effective Interest Amortization Identity
#[derive(elicitation::Prop)]
pub struct AmortizationScheduleCorrect;

/// Accumulated depreciation never exceeds the depreciable cost basis of the asset.
///
/// Source: ASC 360-10-35 — PP&E Depreciation Accumulation Bound
#[derive(elicitation::Prop)]
pub struct DepreciationAccumulatesCorrectly;

// ── Tax invariants ────────────────────────────────────────────────────────

/// The statutory rate plus reconciling items equals the reported effective tax rate.
///
/// Source: ASC 740-10-50-12 — Effective Tax Rate Reconciliation Identity
#[derive(elicitation::Prop)]
pub struct TaxRateReconciles;

/// DTA and DTL are presented net only when they arise from the same tax jurisdiction and entity.
///
/// Source: ASC 740-10-45-6 — Deferred Tax Netting Constraint
#[derive(elicitation::Prop)]
pub struct DeferredTaxNetPresentable;

// ── Segment reconciliation ────────────────────────────────────────────────

/// Sum of reportable segment revenues reconciles to consolidated revenue.
///
/// Source: ASC 280-10-50-30 — Segment Reconciliation
#[derive(elicitation::Prop)]
pub struct SegmentRevenueSumsToConsolidated;
