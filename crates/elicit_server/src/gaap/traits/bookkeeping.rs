//! `GaapBookkeeping` — double-entry recording factory (Role 1a).
//! `GaapLedgerMeta`  — ledger-state reporter (Role 2).

use futures::future::BoxFuture;

use crate::gaap::errors::GaapResult;
use crate::gaap::mathematical::{
    AccountingEquationHolds, DebitEqualsCreditPerEntry, TrialBalanceBalances,
};
use crate::gaap::types::{
    AccountDescriptor, FinancialPeriod, JournalEntryDescriptor, MonetaryAmount,
    TrialBalanceDescriptor,
};
use elicitation::Established;

// ── Role 1a: double-entry recording factory ───────────────────────────────────

/// Core double-entry bookkeeping factory.
///
/// Each method takes raw descriptors and returns proof tokens asserting that
/// the recorded data satisfies the relevant GAAP invariant.
///
/// Source: double-entry bookkeeping; GAAP accrual-basis accounting (ASC 105-10-05-2).
pub trait GaapBookkeeping: Send + Sync {
    /// Record a journal entry.
    ///
    /// Returns `DebitEqualsCreditPerEntry` — the entry is balanced.
    ///
    /// Source: double-entry bookkeeping; ASC 210.
    fn record_journal_entry(
        &self,
        entry: JournalEntryDescriptor,
    ) -> GaapResult<(
        JournalEntryDescriptor,
        Established<DebitEqualsCreditPerEntry>,
    )>;

    /// Compile the trial balance for the given period from already-recorded entries.
    ///
    /// Returns `TrialBalanceBalances` — debit totals equal credit totals.
    ///
    /// Source: double-entry bookkeeping post-close procedure.
    fn compile_trial_balance(
        &self,
        period: FinancialPeriod,
    ) -> GaapResult<(TrialBalanceDescriptor, Established<TrialBalanceBalances>)>;

    /// Verify that the accounting equation holds for the given balance sheet totals.
    ///
    /// Returns `AccountingEquationHolds` — Assets = Liabilities + Equity.
    ///
    /// Source: accounting equation; ASC 210.
    fn verify_accounting_equation(
        &self,
        total_assets: MonetaryAmount,
        total_liabilities: MonetaryAmount,
        total_equity: MonetaryAmount,
    ) -> GaapResult<Established<AccountingEquationHolds>>;
}

// ── Role 2: ledger-state reporter ─────────────────────────────────────────────

/// Orthogonal ledger metadata reporter.
///
/// These methods query backend state independently of proof construction.
/// No proof tokens are consumed or produced.
///
/// Source: general bookkeeping administration.
pub trait GaapLedgerMeta: Send + Sync {
    /// Return the full chart of accounts.
    fn chart_of_accounts(&self) -> BoxFuture<'_, GaapResult<Vec<AccountDescriptor>>>;

    /// Return the currently open accounting period.
    fn current_period(&self) -> BoxFuture<'_, GaapResult<FinancialPeriod>>;

    /// Return all journal entries recorded in the given period.
    fn journal_entries_for_period(
        &self,
        period: FinancialPeriod,
    ) -> BoxFuture<'_, GaapResult<Vec<JournalEntryDescriptor>>>;
}
