//! Journal entry and trial balance descriptor types.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::gaap::types::{AccountId, FinancialPeriod, MonetaryAmount};

// ── Journal entry ─────────────────────────────────────────────────────────────

/// A single debit line in a journal entry.
///
/// Source: double-entry bookkeeping — each transaction has at least one debit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct DebitEntry {
    /// Account to debit.
    pub account: AccountId,
    /// Amount to debit (must be positive).
    pub amount: MonetaryAmount,
}

/// A single credit line in a journal entry.
///
/// Source: double-entry bookkeeping — each transaction has at least one credit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CreditEntry {
    /// Account to credit.
    pub account: AccountId,
    /// Amount to credit (must be positive).
    pub amount: MonetaryAmount,
}

/// A complete double-entry journal entry.
///
/// A valid entry satisfies `DebitEqualsCreditPerEntry`: the sum of all
/// `debits[i].amount` must equal the sum of all `credits[j].amount`.
///
/// Source: GAAP double-entry bookkeeping; ASC 210/220.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct JournalEntryDescriptor {
    /// Unique entry identifier.
    pub id: String,
    /// Free-text description of the transaction.
    pub description: String,
    /// ISO 8601 date on which the transaction occurred.
    pub date: String,
    /// Debit lines (one or more).
    pub debits: Vec<DebitEntry>,
    /// Credit lines (one or more).
    pub credits: Vec<CreditEntry>,
    /// Optional reference to a source document (invoice number, PO, etc.).
    pub source_document: Option<String>,
}

// ── Trial balance ─────────────────────────────────────────────────────────────

/// A compiled trial balance for a given accounting period.
///
/// A valid trial balance satisfies `TrialBalanceBalances`: the sum of all
/// debit-normal account balances equals the sum of all credit-normal account
/// balances.
///
/// Source: double-entry bookkeeping pre-closing and post-closing trial balance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct TrialBalanceDescriptor {
    /// The accounting period covered.
    pub period: FinancialPeriod,
    /// Sum of all debit-normal account balances.
    pub total_debits: MonetaryAmount,
    /// Sum of all credit-normal account balances.
    pub total_credits: MonetaryAmount,
    /// Number of accounts included.
    pub account_count: u32,
}

// ── Balance sheet totals ──────────────────────────────────────────────────────

/// Key totals extracted from a balance sheet, used to verify the accounting
/// equation invariant.
///
/// Source: ASC 210 — Balance Sheet.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BalanceSheetTotals {
    /// Total of all asset accounts.
    pub total_assets: MonetaryAmount,
    /// Total of all liability accounts.
    pub total_liabilities: MonetaryAmount,
    /// Total of all equity accounts (including retained earnings).
    pub total_equity: MonetaryAmount,
}
