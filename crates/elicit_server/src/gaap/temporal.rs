//! Temporal and period-boundary propositions.
//!
//! Each proposition governs when a transaction or measurement must be
//! recorded relative to accounting period boundaries. These complement
//! the mathematical invariants by capturing the time-dimension of
//! accrual accounting correctness.
//!
//! Source: ASC 250, ASC 270, ASC 606, ASC 830, ASC 842;
//!         FASB Concepts Statement No. 5 — Recognition and Measurement
// ── Period cut-off ────────────────────────────────────────────────────────

/// Every transaction is recorded in the accounting period in which it occurred.
///
/// Source: ASC 250-10 — Accounting Changes and Errors; accrual basis cut-off principle
#[derive(elicitation::Prop)]
pub struct TransactionCutoffRespected;

/// Revenue is recognized only in the period in which it is earned and the performance obligation is satisfied.
///
/// Source: ASC 606-10-25 — Revenue Recognition Timing
#[derive(elicitation::Prop)]
pub struct RevenueEarnedInPeriod;

/// Period-end accruals are recorded before the financial statement close.
///
/// Source: ASC 250-10; accrual accounting — period-end recognition
#[derive(elicitation::Prop)]
pub struct AccrualRecordedAtPeriodEnd;

/// Deferred revenue or prepaid expense is released in the period when earned or incurred.
///
/// Source: ASC 606-10-45 — Contract Liabilities Release; ASC 430-10 — Deferred Revenue
#[derive(elicitation::Prop)]
pub struct DeferralReleasedInEarnedPeriod;

// ── Amortization and depreciation timing ──────────────────────────────────

/// Depreciation and amortization reflect the full period's allocation (not partial if in service all period).
///
/// Source: ASC 360-10-35 — PP&E Depreciation; ASC 350-30-35 — Intangible Amortization
#[derive(elicitation::Prop)]
pub struct DepreciationComputedForFullPeriod;

/// Interest accrual is computed through the last calendar day of the reporting period.
///
/// Source: ASC 835-10-25 — Interest Accrual
#[derive(elicitation::Prop)]
pub struct InterestAccruedThroughPeriodEnd;

// ── Dividends and corporate actions ───────────────────────────────────────

/// Dividends are recorded in the period in which the board declaration occurs.
///
/// Source: ASC 505-10-25-1 — Dividend Declaration Date
#[derive(elicitation::Prop)]
pub struct DividendsDeclaredInCorrectPeriod;

// ── Foreign currency translation timing ───────────────────────────────────

/// Balance sheet monetary items are translated at the period-end spot exchange rate.
///
/// Source: ASC 830-10-45-17 — Closing Rate Translation
#[derive(elicitation::Prop)]
pub struct FxTranslationAtClosingRate;

/// Income statement items are translated at the period-average exchange rate.
///
/// Source: ASC 830-30-45-3 — Average Rate Translation
#[derive(elicitation::Prop)]
pub struct FxTranslationAtAverageRate;

// ── Deferred tax timing ───────────────────────────────────────────────────

/// Temporary differences originate and reverse in their correct tax periods.
///
/// Source: ASC 740-10-25 — Deferred Tax Timing Differences
#[derive(elicitation::Prop)]
pub struct TemporaryDifferenceTimingCorrect;

/// The taxable year end is aligned with the financial reporting period, or the difference is noted.
///
/// Source: ASC 740-270 — Interim-Period Tax Accounting
#[derive(elicitation::Prop)]
pub struct TaxPeriodAligned;

// ── Interim period timing ─────────────────────────────────────────────────

/// Interim period accruals are based on the same method as the annual estimate.
///
/// Source: ASC 270-10-45-6 — Interim Accrual Consistency
#[derive(elicitation::Prop)]
pub struct InterimAccrualMethodConsistent;

/// The subsequent event evaluation period ends on a specific identifiable issuance date.
///
/// Source: ASC 855-10-25-1 — Subsequent Event Period Boundary
#[derive(elicitation::Prop)]
pub struct SubsequentEventDateBound;

// ── Stock awards and lease timing ─────────────────────────────────────────

/// The grant date for a stock award is the date on which mutual understanding of the terms is reached.
///
/// Source: ASC 718-10-25-5 — Grant Date Definition
#[derive(elicitation::Prop)]
pub struct StockOptionGrantDateCorrect;

/// The lease commencement date (not the signing date) determines when recognition begins.
///
/// Source: ASC 842-20-25-1 — Lease Commencement Date
#[derive(elicitation::Prop)]
pub struct LeaseCommencementDateCorrect;

// ── Revenue transfer-of-control timing ───────────────────────────────────

/// The point-in-time transfer date is the date on which control passes to the customer.
///
/// Source: ASC 606-10-25-30 — Point-in-Time Transfer
#[derive(elicitation::Prop)]
pub struct RevenueTransferDateCorrect;

/// Expense is matched to the same period as the revenue it helps to generate.
///
/// Source: FASB Concepts Statement No. 5 — Matching Principle
#[derive(elicitation::Prop)]
pub struct ExpenseMatchedToPeriod;
