//! Transfer-level proof propositions for the ledger typestate machine.
//!
//! These propositions are structural proof tokens — they carry no runtime data and
//! compose freely with `both()` / `And<A, B>`. Backends establish them by calling
//! `Established::assert()` after satisfying the stated criterion.

use elicitation::contracts::And;
/// Transfer amount is positive (> 0).
///
/// Source: pre-ASC ledger invariant — amounts must be non-zero and non-negative.
#[derive(elicitation::Prop)]
pub struct AmountPositive;

/// Source account holds at least the transfer amount.
///
/// Source: pre-ASC ledger invariant — no overdraft without explicit credit facility.
#[derive(elicitation::Prop)]
pub struct SufficientFunds;

/// Source and destination accounts are distinct.
///
/// Source: ASC 230 — Statement of Cash Flows; gross vs. net presentation.
#[derive(elicitation::Prop)]
pub struct AccountsDistinct;

/// Debit entry and credit entry balance (debit + credit = 0).
///
/// Source: pre-ASC foundational double-entry requirement.
#[derive(elicitation::Prop)]
pub struct BalancedEntries;
/// Composite: transfer is valid when amount, funds, and account identity all hold.
pub type ValidTransfer = And<AmountPositive, And<SufficientFunds, AccountsDistinct>>;
