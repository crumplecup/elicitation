//! ASC 500 series — Equity.
//!
//! Covers ASC 505: stockholders' equity presentation, treasury stock, stock splits,
//! retained earnings, OCI, noncontrolling interests, dividends, and preferred stock.
//!
//! Source: FASB ASC 505 — <https://asc.fasb.org/>
// ── ASC 505 — Equity ──────────────────────────────────────────────────────

/// Stockholders' equity section is complete and clearly presented.
///
/// Source: ASC 505-10-45 — Stockholders' Equity Presentation
#[derive(elicitation::Prop)]
pub struct StockholdersEquityPresented;

/// Par value, shares authorized, issued, and outstanding are disclosed for each class.
///
/// Source: ASC 505-10-50-2 — Capital Structure Disclosures
#[derive(elicitation::Prop)]
pub struct CommonStockParValueDisclosed;

/// Treasury stock is accounted for using the cost method or par value method, disclosed consistently.
///
/// Source: ASC 505-30-30 — Treasury Stock
#[derive(elicitation::Prop)]
pub struct TreasuryStockAccountedFor;

/// Stock split or stock dividend is accounted for and all per-share amounts are retroactively adjusted.
///
/// Source: ASC 505-20-25 — Stock Dividends and Stock Splits
#[derive(elicitation::Prop)]
pub struct StockSplitAccountedFor;

/// Retained earnings are reconciled from the beginning to the end of the period.
///
/// Source: ASC 505-10-50-3 — Retained Earnings Rollforward
#[derive(elicitation::Prop)]
pub struct RetainedEarningsReconciled;

/// Accumulated other comprehensive income (AOCI) is presented as a separate component of equity.
///
/// Source: ASC 220-10-45-14 — AOCI in Stockholders' Equity
#[derive(elicitation::Prop)]
pub struct OciAccumulatedSeparately;

/// Noncontrolling interest is presented within equity (not as a liability or mezzanine).
///
/// Source: ASC 810-10-45-16 — Noncontrolling Interest in Consolidated Balance Sheet
#[derive(elicitation::Prop)]
pub struct NoncontrollingInterestPresented;

/// Dividends declared are recorded in the period of declaration.
///
/// Source: ASC 505-10-25-1 — Dividends
#[derive(elicitation::Prop)]
pub struct DividendsDeclaredRecorded;

/// Preferred stock liquidation preference, dividend rate, and conversion rights are disclosed.
///
/// Source: ASC 505-10-50-4 — Preferred Stock Terms
#[derive(elicitation::Prop)]
pub struct PreferredStockTermsDisclosed;

/// Subscription receivable from stockholders is presented as a contra-equity item, not an asset.
///
/// Source: ASC 505-10-45-2 — Stock Subscriptions Receivable
#[derive(elicitation::Prop)]
pub struct StockSubscriptionReceivableAsContraEquity;

/// Appropriations of retained earnings are separately identified and disclosed.
///
/// Source: ASC 505-10-45-4 — Appropriated Retained Earnings
#[derive(elicitation::Prop)]
pub struct RetainedEarningsAppropriationDisclosed;
