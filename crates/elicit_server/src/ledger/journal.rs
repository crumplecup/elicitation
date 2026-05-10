//! Journal-entry lifecycle proof propositions.
//!
//! These are generalised accounting contracts for the double-entry journal
//! entry state machine:
//!
//! ```text
//! EntryDrafted ──balance()──> EntryBalanced ──post()──> EntryPosted ──close()──> EntryClosed
//! ```
//!
//! Each proposition is a zero-sized proof token carried as `Established<P>`.
//! They compose with `both()` / `And<A, B>` for richer invariants.
//!
//! GAAP bridges (`ProvableFrom` impls connecting these tokens to canonical
//! GAAP propositions) live in [`crate::gaap::proof_composition`].

use elicitation::{VerifiedWorkflow, contracts::And};

// ── Journal entry lifecycle ───────────────────────────────────────────────────

/// Proposition: A journal entry has been created in Draft state.
///
/// Establishes: entry ID is unique, entity ID is valid, date and description
/// are set, lines collection exists (may be empty).
#[derive(elicitation::Prop)]
pub struct EntryDrafted;

impl VerifiedWorkflow for EntryDrafted {}

/// Proposition: A journal entry is balanced (debits == credits).
///
/// Establishes: total debits equal total credits; GAAP double-entry invariant
/// holds; entry can be posted.
#[derive(elicitation::Prop)]
#[cfg_attr(kani, derive(kani::Arbitrary))]
pub struct EntryBalanced;

impl VerifiedWorkflow for EntryBalanced {}

/// Proposition: A journal entry has been posted to the ledger.
///
/// Establishes: entry is immutable; posted timestamp is set; account balances
/// have been updated; entry appears in chronological ledger.
#[derive(elicitation::Prop)]
pub struct EntryPosted;

impl VerifiedWorkflow for EntryPosted {}

/// Proposition: A journal entry has been closed (period-end close).
///
/// Establishes: entry is part of a completed accounting period; temporary
/// accounts (revenue/expense) zeroed; net income transferred to retained
/// earnings.
#[derive(elicitation::Prop)]
pub struct EntryClosed;

impl VerifiedWorkflow for EntryClosed {}

// ── Persistence propositions ──────────────────────────────────────────────────

/// Proposition: An account has been saved to persistent storage.
#[derive(elicitation::Prop)]
pub struct AccountSaved;

impl VerifiedWorkflow for AccountSaved {}

/// Proposition: A journal entry has been saved to persistent storage.
#[derive(elicitation::Prop)]
pub struct EntrySaved;

impl VerifiedWorkflow for EntrySaved {}

// ── Financial statement propositions ─────────────────────────────────────────

/// Proposition: Net income has been correctly computed.
///
/// Establishes: all revenue accounts summed correctly (credits − debits);
/// all expense accounts summed correctly (debits − credits);
/// net income = total revenue − total expenses;
/// computation used correct period boundaries.
#[derive(elicitation::Prop)]
pub struct NetIncomeComputed;

impl VerifiedWorkflow for NetIncomeComputed {}

// ── Composite type aliases ────────────────────────────────────────────────────

/// Entry is balanced and has been posted — the minimal proof that `post()` can
/// assemble from tokens in scope.
pub type ReadyToPost = And<EntryBalanced, EntryPosted>;

/// Alias for [`ReadyToPost`]; the "balanced-and-posted" framing reads more
/// naturally at some call sites.
pub type BalancedAndPosted = And<EntryBalanced, EntryPosted>;

/// Entry posted and persisted.
pub type PostedAndSaved = And<EntryPosted, EntrySaved>;
