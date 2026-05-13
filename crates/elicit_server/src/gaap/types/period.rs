//! Financial period and accrual/deferral descriptor types.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::gaap::types::{AccountId, MonetaryAmount, PeriodDate};

// ── Financial period ──────────────────────────────────────────────────────────

/// The granularity of a financial reporting period.
///
/// Source: ASC 270 — Interim Reporting; ASC 280 — Segment Reporting.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum PeriodType {
    /// Full fiscal year (12 months).
    Annual,
    /// Calendar or fiscal quarter.
    Quarterly,
    /// Single calendar month.
    Monthly,
    /// Any non-annual sub-period (generalizes quarterly/monthly).
    Interim,
}

/// A defined accounting reporting period with inclusive start/end dates.
///
/// Source: ASC 270 — Interim Reporting; ASC 250 — Accounting Changes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct FinancialPeriod {
    /// Period granularity.
    pub period_type: PeriodType,
    /// First day of the period (ISO 8601).
    pub start_date: PeriodDate,
    /// Last day of the period (ISO 8601, inclusive).
    pub end_date: PeriodDate,
    /// The four-digit fiscal year this period belongs to.
    pub fiscal_year: u32,
}

// ── Accrual and deferral ──────────────────────────────────────────────────────

/// Descriptor for a period-end accrual — an amount earned or incurred during
/// the period but not yet received or paid in cash.
///
/// The factory asserts `AccrualRecordedAtPeriodEnd` when the accrual date falls
/// within the covered period.
///
/// Source: ASC 420 / accrual-basis accounting (ASC 105-10-05-2).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AccrualDescriptor {
    /// Account to debit (expense or asset) or credit (revenue or liability).
    pub account: AccountId,
    /// Offsetting account (e.g. accrued liability or accrued receivable).
    pub contra_account: AccountId,
    /// Amount of the accrual.
    pub amount: MonetaryAmount,
    /// Period during which the accrual was earned or incurred.
    pub period: FinancialPeriod,
    /// Free-text description (e.g. `"Q4 accrued wages"`).
    pub description: String,
}

/// Descriptor for a deferral — cash received or paid before the associated
/// revenue is earned or expense is consumed.
///
/// The factory asserts `DeferralReleasedInEarnedPeriod` when the release date
/// falls in the period during which the item is earned.
///
/// Source: accrual-basis accounting; ASC 606 deferred-revenue model.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct DeferralDescriptor {
    /// Balance-sheet account holding the deferred amount.
    pub deferred_account: AccountId,
    /// Income statement account to which the amount is released.
    pub recognition_account: AccountId,
    /// Amount to defer or release.
    pub amount: MonetaryAmount,
    /// The ISO 8601 date on which the item is earned/consumed and should be
    /// released from the deferred account.
    pub earned_date: PeriodDate,
    /// Free-text description (e.g. `"Prepaid insurance Q2 release"`).
    pub description: String,
}
