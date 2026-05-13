//! GAAP proof composition — `ProvableFrom` dependency chains.
//!
//! This module declares how higher-order propositions are provable from
//! lower-order evidence bundles. None of this is runtime logic; it is
//! purely a type-level dependency graph that formal verification tools
//! (Kani, Creusot, Verus) can traverse.
//!
//! # ASC 606 five-step chain
//!
//! The five-step revenue recognition model has a natural sequential dependency:
//! each step must be established before the next step can be proven. The chain is:
//!
//! 1. [`ContractIdentified`] ← [`ContractCriteriaMet`] + [`CollectibilityProbable`]
//! 2. [`PerformanceObligationsIdentified`] ← `Established<ContractIdentified>`
//! 3. [`TransactionPriceDetermined`] ← `Established<ContractIdentified>`
//! 4. [`TransactionPriceAllocated`] ← [`Asc606Steps1To3Evidence`]
//! 5. [`RevenueRecognizedAtPointInTime`] / [`RevenueRecognizedOverTime`] ← [`Asc606FullEvidence`]
//!
//! # Fundamental invariant chains
//!
//! - [`TrialBalanceBalances`] ← `Established<DebitEqualsCreditPerEntry>`
//! - [`AccountingEquationHolds`] ← [`AccountingEquationEvidence`]
//! - [`RetainedEarningsRollforward`] ← [`RetainedEarningsEvidence`]
//!
//! Source: FASB ASC 606; double-entry bookkeeping foundations

use elicitation::{Established, contracts::ProvableFrom};

use crate::gaap::{
    // mathematical invariants
    AccountingEquationHolds,
    // ASC 606
    CollectibilityProbable,
    ContractCriteriaMet,
    ContractIdentified,
    DebitEqualsCreditPerEntry,
    // core principles
    DoubleEntryBookkeeping,
    NetIncomeAggregation,
    OverTimeCriteriaMet,
    PerformanceObligationsIdentified,
    ProgressMeasurementMethodSelected,
    RetainedEarningsRollforward,
    RevenueRecognizedAtPointInTime,
    RevenueRecognizedOverTime,
    StandaloneSellingPriceDetermined,
    TransactionPriceAllocated,
    TransactionPriceDetermined,
    TrialBalanceBalances,
};

// ── ASC 606 Step 1: Contract Identification ───────────────────────────────────

/// Evidence bundle for ASC 606 Step 1: Identify the Contract.
///
/// The contract criteria (ASC 606-10-25-1(a)–(e)) must be individually
/// established, and collectibility must be demonstrated as probable.
///
/// Source: ASC 606-10-25-1 — Identifying the Contract
pub struct ContractIdentificationEvidence {
    /// All five contract criteria are met.
    pub criteria: Established<ContractCriteriaMet>,
    /// Collectibility of the consideration is probable.
    pub collectibility: Established<CollectibilityProbable>,
}

impl ProvableFrom<ContractIdentificationEvidence> for ContractIdentified {}

// ── ASC 606 Step 2: Performance Obligations ───────────────────────────────────

/// Evidence bundle for ASC 606 Step 2: Identify Performance Obligations.
///
/// A valid contract must exist before performance obligations can be identified.
///
/// Source: ASC 606-10-25-14 — Identifying Performance Obligations
pub struct PerformanceObligationsEvidence {
    /// The contract has been identified and all Step 1 criteria are satisfied.
    pub contract: Established<ContractIdentified>,
}

impl ProvableFrom<PerformanceObligationsEvidence> for PerformanceObligationsIdentified {}

// ── ASC 606 Step 3: Transaction Price ────────────────────────────────────────

/// Evidence bundle for ASC 606 Step 3: Determine the Transaction Price.
///
/// The transaction price can only be determined in the context of a valid contract.
///
/// Source: ASC 606-10-32-2 — Determining the Transaction Price
pub struct TransactionPriceEvidence {
    /// The contract has been identified and all Step 1 criteria are satisfied.
    pub contract: Established<ContractIdentified>,
}

impl ProvableFrom<TransactionPriceEvidence> for TransactionPriceDetermined {}

// ── ASC 606 Step 4: Allocation ────────────────────────────────────────────────

/// Evidence bundle for ASC 606 Step 4: Allocate the Transaction Price.
///
/// Allocation requires that Steps 1–3 are all established and that standalone
/// selling prices are determined for each identified performance obligation.
///
/// Source: ASC 606-10-32-28 — Allocating the Transaction Price
pub struct Asc606Steps1To3Evidence {
    /// Step 1: Contract has been identified.
    pub contract: Established<ContractIdentified>,
    /// Step 2: All performance obligations are identified.
    pub obligations: Established<PerformanceObligationsIdentified>,
    /// Step 3: Transaction price is determined.
    pub price: Established<TransactionPriceDetermined>,
    /// Supporting: SSP determined for each PO.
    pub ssp: Established<StandaloneSellingPriceDetermined>,
}

impl ProvableFrom<Asc606Steps1To3Evidence> for TransactionPriceAllocated {}

// ── ASC 606 Step 5: Revenue Recognition ──────────────────────────────────────

/// Evidence bundle for ASC 606 Step 5 — Point-in-Time Revenue Recognition.
///
/// All four preceding steps must be established before revenue can be recognized
/// at a point in time.
///
/// Source: ASC 606-10-25-30 — Point-in-Time Recognition
pub struct Asc606PointInTimeEvidence {
    /// Step 1: Contract identified.
    pub contract: Established<ContractIdentified>,
    /// Step 2: Performance obligations identified.
    pub obligations: Established<PerformanceObligationsIdentified>,
    /// Step 3: Transaction price determined.
    pub price: Established<TransactionPriceDetermined>,
    /// Step 4: Transaction price allocated.
    pub allocation: Established<TransactionPriceAllocated>,
}

impl ProvableFrom<Asc606PointInTimeEvidence> for RevenueRecognizedAtPointInTime {}

/// Evidence bundle for ASC 606 Step 5 — Over-Time Revenue Recognition.
///
/// Over-time recognition additionally requires that at least one over-time
/// criterion is satisfied and a valid progress measurement method is selected.
///
/// Source: ASC 606-10-25-27 — Over-Time Recognition
pub struct Asc606OverTimeEvidence {
    /// Step 1: Contract identified.
    pub contract: Established<ContractIdentified>,
    /// Step 2: Performance obligations identified.
    pub obligations: Established<PerformanceObligationsIdentified>,
    /// Step 3: Transaction price determined.
    pub price: Established<TransactionPriceDetermined>,
    /// Step 4: Transaction price allocated.
    pub allocation: Established<TransactionPriceAllocated>,
    /// Over-time: at least one of the three criteria in ASC 606-10-25-27 is satisfied.
    pub over_time_criteria: Established<OverTimeCriteriaMet>,
    /// Over-time: an input or output method is selected and applied.
    pub progress_method: Established<ProgressMeasurementMethodSelected>,
}

impl ProvableFrom<Asc606OverTimeEvidence> for RevenueRecognizedOverTime {}

// ── Fundamental bookkeeping invariant chains ──────────────────────────────────

/// Evidence for the trial balance invariant.
///
/// The trial balance can only balance if every individual journal entry has
/// equal debits and credits.
///
/// Source: Double-entry bookkeeping; pre-ASC foundational arithmetic
pub struct TrialBalanceEvidence {
    /// Every journal entry in the ledger has balanced debits and credits.
    pub all_entries_balance: Established<DebitEqualsCreditPerEntry>,
}

impl ProvableFrom<TrialBalanceEvidence> for TrialBalanceBalances {}

/// Evidence for the accounting equation invariant.
///
/// The accounting equation (Assets = Liabilities + Equity) holds at period-end
/// when double-entry bookkeeping is enforced and net income is correctly aggregated.
///
/// Source: ASC 210 — Balance Sheet; double-entry bookkeeping foundations
pub struct AccountingEquationEvidence {
    /// Double-entry bookkeeping is observed throughout.
    pub double_entry: Established<DoubleEntryBookkeeping>,
    /// The trial balance is balanced (all entries in equilibrium).
    pub trial_balance: Established<TrialBalanceBalances>,
    /// Net income is correctly aggregated from revenue less expenses.
    pub net_income: Established<NetIncomeAggregation>,
}

impl ProvableFrom<AccountingEquationEvidence> for AccountingEquationHolds {}

/// Evidence for the retained earnings rollforward invariant.
///
/// The retained earnings rollforward (RE_end = RE_begin + NI − Dividends) holds
/// when net income aggregation is established and the accounting equation holds.
///
/// Source: ASC 505-10 — Retained Earnings; double-entry bookkeeping
pub struct RetainedEarningsEvidence {
    /// The accounting equation holds at both period start and period end.
    pub accounting_equation: Established<AccountingEquationHolds>,
    /// Net income is correctly aggregated.
    pub net_income: Established<NetIncomeAggregation>,
}

impl ProvableFrom<RetainedEarningsEvidence> for RetainedEarningsRollforward {}

// ── Journal-entry → GAAP bridges ─────────────────────────────────────────────
//
// These bridges connect domain-level journal-entry propositions
// (`elicit_server::ledger::journal`) to canonical GAAP invariants, encoding
// the accounting rules at the type level.

use crate::ledger::journal::{EntryBalanced, NetIncomeComputed};

/// Given that an entry is balanced (`EntryBalanced`), you can issue a
/// `DebitEqualsCreditPerEntry` GAAP proof token.
///
/// Source: double-entry bookkeeping — debit/credit symmetry; ASC 210.
impl ProvableFrom<EntryBalanced> for DebitEqualsCreditPerEntry {}

/// Given that net income has been correctly computed (`NetIncomeComputed`),
/// you can issue a `NetIncomeAggregation` GAAP proof token.
///
/// Source: ASC 225 — Income Statement.
impl ProvableFrom<NetIncomeComputed> for NetIncomeAggregation {}
