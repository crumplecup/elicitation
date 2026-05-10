//! ASC 700 series — Stock Compensation and Income Taxes.
//!
//! Covers ASC 718 (share-based payment awards) and ASC 740 (income taxes,
//! deferred tax, uncertain tax positions).
//!
//! Source: FASB ASC 718, 740 — <https://asc.fasb.org/>
// ── ASC 718 — Compensation — Stock Compensation ───────────────────────────

/// Grant-date fair value of a share-based award is measured using an appropriate valuation model.
///
/// Source: ASC 718-10-30-3 — Grant-Date Fair Value Measurement
#[derive(elicitation::Prop)]
pub struct StockCompensationFairValueMeasured;

/// Compensation cost is recognized over the requisite service period (vesting period).
///
/// Source: ASC 718-10-35-2 — Recognition of Compensation Cost
#[derive(elicitation::Prop)]
pub struct StockCompensationRecognizedOverVesting;

/// Graded vesting tranches are treated separately when the election is made.
///
/// Source: ASC 718-10-35-8 — Graded Vesting
#[derive(elicitation::Prop)]
pub struct GradedVestingApplied;

/// Forfeitures are estimated at grant date (or recognized when they occur under the election).
///
/// Source: ASC 718-10-35-3 — Forfeiture Accounting
#[derive(elicitation::Prop)]
pub struct ForfeitureAccountingApplied;

/// Performance condition probability is assessed each reporting period to determine compensation expense.
///
/// Source: ASC 718-10-25-20 — Performance Conditions
#[derive(elicitation::Prop)]
pub struct PerformanceConditionAssessed;

/// Market condition is incorporated into the grant-date fair value (not reassessed).
///
/// Source: ASC 718-10-25-20 — Market Conditions
#[derive(elicitation::Prop)]
pub struct MarketConditionIncludedInFairValue;

/// Modification of a share-based award triggers measurement of incremental fair value.
///
/// Source: ASC 718-20-35-3 — Award Modifications
#[derive(elicitation::Prop)]
pub struct AwardModificationAccountedFor;

/// Share-based award tax effects are recognized through income tax expense in the income statement.
///
/// Source: ASC 718-740-45-7 — Tax Effects of Share-Based Awards
#[derive(elicitation::Prop)]
pub struct StockAwardTaxEffectInEarnings;

// ── ASC 740 — Income Taxes ────────────────────────────────────────────────

/// Deferred tax asset is recognized for deductible temporary differences and carryforwards.
///
/// Source: ASC 740-10-25-2 — Deferred Tax Assets
#[derive(elicitation::Prop)]
pub struct DeferredTaxAssetRecognized;

/// Deferred tax liability is recognized for taxable temporary differences.
///
/// Source: ASC 740-10-25-3 — Deferred Tax Liabilities
#[derive(elicitation::Prop)]
pub struct DeferredTaxLiabilityRecognized;

/// Valuation allowance is assessed against the deferred tax asset when realization is not more likely than not.
///
/// Source: ASC 740-10-30-18 — Valuation Allowance Assessment
#[derive(elicitation::Prop)]
pub struct ValuationAllowanceAssessed;

/// The "more likely than not" standard is applied to determine DTA realizability.
///
/// Source: ASC 740-10-30-5 — More-Likely-Than-Not Standard
#[derive(elicitation::Prop)]
pub struct MoreLikelyThanNotStandardApplied;

/// Uncertain tax positions are evaluated using the recognition and measurement thresholds.
///
/// Source: ASC 740-10-25-6 — Uncertain Tax Positions
#[derive(elicitation::Prop)]
pub struct UncertainTaxPositionEvaluated;

/// Effective tax rate reconciliation is disclosed, including all material reconciling items.
///
/// Source: ASC 740-10-50-12 — Effective Tax Rate Reconciliation
#[derive(elicitation::Prop)]
pub struct EffectiveTaxRateDisclosed;

/// Deferred tax assets and liabilities are classified as noncurrent on the balance sheet.
///
/// Source: ASC 740-10-45-4 — Noncurrent Classification of Deferred Taxes
#[derive(elicitation::Prop)]
pub struct DeferredTaxNoncurrentClassified;

/// Intraperiod tax allocation is applied to allocate income tax among continuing operations, OCI, and equity.
///
/// Source: ASC 740-20-45 — Intraperiod Tax Allocation
#[derive(elicitation::Prop)]
pub struct IntraperiodTaxAllocationApplied;
