//! Core primitive types shared across all GAAP accounting domains.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ── Currency and amount ───────────────────────────────────────────────────────

/// ISO 4217 currency code (e.g. `"USD"`, `"EUR"`, `"GBP"`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct CurrencyCode(pub String);

impl CurrencyCode {
    /// USD — United States Dollar.
    pub fn usd() -> Self {
        Self("USD".to_string())
    }
}

/// A monetary amount in the smallest indivisible unit of the currency
/// (e.g. cents for USD).
///
/// Positive values represent increases (debits to asset accounts, credits
/// to liability/equity accounts).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema,
)]
pub struct MonetaryAmount {
    /// Amount in the smallest currency unit (e.g. cents).  May be negative
    /// for contra-accounts or adjustments.
    pub units: i64,
}

impl MonetaryAmount {
    /// Construct a monetary amount from a cent value.
    pub fn from_cents(cents: i64) -> Self {
        Self { units: cents }
    }

    /// Return the absolute value.
    pub fn abs(self) -> Self {
        Self::from_cents(self.units.abs())
    }

    /// Return true when the amount is zero.
    pub fn is_zero(self) -> bool {
        self.units == 0
    }
}

// ── Identifiers ───────────────────────────────────────────────────────────────

/// Chart-of-accounts identifier for a single ledger account.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct AccountId(pub String);

impl AccountId {
    /// Create an account identifier from any string.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

/// Legal-entity identifier (e.g. company or reporting-unit code).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct EntityId(pub String);

impl EntityId {
    /// Create an entity identifier.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

/// An ISO 8601 calendar date string (`YYYY-MM-DD`) used as a temporal anchor
/// for period cut-off and accrual/deferral calculations.
///
/// Stored as a plain string so the trait interface is not coupled to any
/// particular date library; implementing crates can parse with `elicit_chrono`,
/// `elicit_jiff`, or `elicit_time` as appropriate.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema,
)]
pub struct PeriodDate(pub String);

impl PeriodDate {
    /// Create a date from an ISO 8601 string.
    pub fn new(date: impl Into<String>) -> Self {
        Self(date.into())
    }
}

// ── Account taxonomy ──────────────────────────────────────────────────────────

/// GAAP chart-of-accounts account type per the accounting equation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum AccountType {
    /// Asset account (normal debit balance).
    Asset,
    /// Liability account (normal credit balance).
    Liability,
    /// Equity account (normal credit balance).
    Equity,
    /// Revenue account (normal credit balance).
    Revenue,
    /// Expense account (normal debit balance).
    Expense,
    /// Contra account (opposite of its paired account's normal balance).
    Contra,
}

/// Which side of a T-account increases the balance.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum NormalBalance {
    /// Balance increases with debits (assets, expenses, contra-liability).
    Debit,
    /// Balance increases with credits (liabilities, equity, revenue, contra-asset).
    Credit,
}

/// A single account in the chart of accounts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AccountDescriptor {
    /// Unique account code.
    pub id: AccountId,
    /// Human-readable account name (e.g. `"Accounts Receivable"`).
    pub name: String,
    /// Account category per the accounting equation.
    pub account_type: AccountType,
    /// Normal (increase) side for this account.
    pub normal_balance: NormalBalance,
    /// Current balance in smallest currency units.
    pub balance: MonetaryAmount,
}
