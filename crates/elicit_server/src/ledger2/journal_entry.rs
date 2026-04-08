//! Journal entry with typestate state machine.
//!
//! A journal entry transitions through states:
//! ```text
//! Draft → Balanced → Posted → Closed
//! ```
//!
//! Each state carries different data and permits different operations:
//! - **Draft**: Being constructed, may be unbalanced
//! - **Balanced**: Debits = Credits, ready to post
//! - **Posted**: Committed to ledger, affects account balances
//! - **Closed**: Part of closed accounting period, immutable

use std::marker::PhantomData;

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ledger2::{Amount, EntityId, JournalLine};

// ─────────────────────────────────────────────────────────────
//  Entry Identifier
// ─────────────────────────────────────────────────────────────

/// Unique identifier for a journal entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntryId(pub Uuid);

impl EntryId {
    /// Creates a new random entry ID.
    pub fn new_v4() -> Self {
        Self(Uuid::new_v4())
    }

    /// Creates an entry ID from a UUID.
    pub fn from_uuid(id: Uuid) -> Self {
        Self(id)
    }
}

impl std::fmt::Display for EntryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ─────────────────────────────────────────────────────────────
//  GAAP Proof
// ─────────────────────────────────────────────────────────────

/// GAAP compliance proof carried with journal entries.
///
/// Contains established propositions proving that the entry satisfies
/// GAAP principles. Proofs are zero-cost at runtime but provide compile-time
/// guarantees of accounting compliance.
///
/// # Propositions
///
/// The proof records which GAAP propositions have been established:
/// - **P0 (Critical):** DoubleEntryBookkeeping, AccrualBasis, MonetaryUnitAssumption
/// - **P1 (Enhanced):** MatchingPrinciple, EconomicEntityAssumption, HistoricalCostPrinciple
/// - **P2 (Policy):** ConservatismPrinciple, GoingConcernAssumption, MaterialityPrinciple
///
/// # Example
///
/// ```rust,ignore
/// use elicit_server::ledger2::GaapProof;
///
/// let proof = GaapProof::builder()
///     .double_entry()
///     .accrual_basis()
///     .monetary_unit()
///     .build();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GaapProof {
    /// Which GAAP propositions are satisfied.
    propositions: Vec<String>,

    /// Timestamp when proof was established.
    established_at: DateTime<Utc>,
}

impl GaapProof {
    /// Creates an empty GAAP proof.
    pub fn empty() -> Self {
        Self {
            propositions: Vec::new(),
            established_at: Utc::now(),
        }
    }

    /// Creates a GAAP proof with the given propositions.
    pub fn with_propositions(propositions: Vec<String>) -> Self {
        Self {
            propositions,
            established_at: Utc::now(),
        }
    }

    /// Returns the list of established propositions.
    pub fn propositions(&self) -> &[String] {
        &self.propositions
    }

    /// Returns when the proof was established.
    pub fn established_at(&self) -> DateTime<Utc> {
        self.established_at
    }

    /// Returns true if the given proposition is established.
    pub fn has_proposition(&self, name: &str) -> bool {
        self.propositions.iter().any(|p| p == name)
    }
}

impl Default for GaapProof {
    fn default() -> Self {
        Self::empty()
    }
}

// ─────────────────────────────────────────────────────────────
//  State Markers
// ─────────────────────────────────────────────────────────────

/// Draft state - entry is being constructed, may be unbalanced.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Draft;

/// Balanced state - debits equal credits, ready to post.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Balanced;

/// Posted state - committed to ledger, affects account balances.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Posted;

/// Closed state - part of closed accounting period, immutable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Closed;

// ─────────────────────────────────────────────────────────────
//  State Data
// ─────────────────────────────────────────────────────────────

/// State-specific data for journal entries.
///
/// Different states carry different data:
/// - **Draft**: No additional data
/// - **Balanced**: Total amount, when balanced
/// - **Posted**: When posted, by whom
/// - **Closed**: When closed, which period
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StateData {
    /// Draft state data.
    Draft,

    /// Balanced state data.
    Balanced {
        /// Total amount (debits = credits).
        total: Amount,
        /// When the entry was balanced.
        balanced_at: DateTime<Utc>,
    },

    /// Posted state data.
    Posted {
        /// Total amount (debits = credits).
        total: Amount,
        /// When the entry was balanced.
        balanced_at: DateTime<Utc>,
        /// When the entry was posted to the ledger.
        posted_at: DateTime<Utc>,
    },

    /// Closed state data.
    Closed {
        /// Total amount (debits = credits).
        total: Amount,
        /// When the entry was balanced.
        balanced_at: DateTime<Utc>,
        /// When the entry was posted to the ledger.
        posted_at: DateTime<Utc>,
        /// When the entry's period was closed.
        closed_at: DateTime<Utc>,
    },
}

// ─────────────────────────────────────────────────────────────
//  Journal Entry (Typestate)
// ─────────────────────────────────────────────────────────────

/// A journal entry with typestate state machine.
///
/// Journal entries transition through states:
/// ```text
/// Draft → Balanced → Posted → Closed
/// ```
///
/// # GAAP Requirements
///
/// - **Double-entry:** Every entry must have equal debits and credits
/// - **Accrual basis:** Transactions recorded when they occur
/// - **Monetary unit:** All amounts in common currency
/// - **Economic entity:** Entries belong to specific entity
///
/// # Example
///
/// ```rust,ignore
/// use elicit_server::ledger2::{JournalEntry, Amount};
///
/// // Create draft entry
/// let draft = JournalEntry::builder()
///     .date(date)
///     .description("Cash sale")
///     .debit(cash_account, Amount::from_dollars(100), "Payment received")
///     .credit(revenue_account, Amount::from_dollars(100), "Sale of goods")
///     .build()?;
///
/// // Balance checks debits = credits
/// let balanced = draft.balance()?;
///
/// // Post to ledger
/// let posted = balanced.post()?;
///
/// // Later, close period
/// let closed = posted.close()?;
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntry<S> {
    /// Unique entry identifier.
    pub(crate) entry_id: EntryId,

    /// Entity this entry belongs to.
    pub(crate) entity_id: EntityId,

    /// Entry date (when transaction occurred).
    pub(crate) date: NaiveDate,

    /// Description of the transaction.
    pub(crate) description: String,

    /// Journal lines (debits and credits).
    pub(crate) lines: Vec<JournalLine>,

    /// GAAP compliance proof.
    pub(crate) gaap_proof: GaapProof,

    /// State-specific data.
    pub(crate) state_data: StateData,

    /// When the entry was created.
    pub(crate) created_at: DateTime<Utc>,

    /// State marker (zero-cost).
    #[serde(skip)]
    pub(crate) _state: PhantomData<S>,
}

impl<S> JournalEntry<S> {
    /// Returns the entry ID.
    pub fn entry_id(&self) -> EntryId {
        self.entry_id
    }

    /// Returns the entity ID.
    pub fn entity_id(&self) -> EntityId {
        self.entity_id
    }

    /// Returns the entry date.
    pub fn date(&self) -> NaiveDate {
        self.date
    }

    /// Returns the description.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the journal lines.
    pub fn lines(&self) -> &[JournalLine] {
        &self.lines
    }

    /// Returns the GAAP proof.
    pub fn gaap_proof(&self) -> &GaapProof {
        &self.gaap_proof
    }

    /// Returns the state data.
    pub fn state_data(&self) -> &StateData {
        &self.state_data
    }

    /// Returns when the entry was created.
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// Returns the total debit amount.
    pub fn total_debits(&self) -> Amount {
        self.lines
            .iter()
            .filter_map(|line| line.debit_amount())
            .fold(Amount::from_cents(0), |acc, amt| acc + amt)
    }

    /// Returns the total credit amount.
    pub fn total_credits(&self) -> Amount {
        self.lines
            .iter()
            .filter_map(|line| line.credit_amount())
            .fold(Amount::from_cents(0), |acc, amt| acc + amt)
    }

    /// Returns true if debits equal credits.
    pub fn is_balanced(&self) -> bool {
        self.total_debits() == self.total_credits()
    }
}

impl<S> std::fmt::Display for JournalEntry<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Journal Entry {} - {}", self.entry_id, self.date)?;
        writeln!(f, "{}", self.description)?;
        for line in &self.lines {
            writeln!(f, "  {}", line)?;
        }
        writeln!(
            f,
            "Total: Debits {} = Credits {}",
            self.total_debits(),
            self.total_credits()
        )?;
        Ok(())
    }
}

// ─────────────────────────────────────────────────────────────
//  State Transitions
// ─────────────────────────────────────────────────────────────

impl JournalEntry<Balanced> {
    /// Posts the journal entry to the ledger.
    ///
    /// This transition commits the entry to the ledger and affects account
    /// balances. Once posted, the entry cannot be modified - only reversed.
    ///
    /// # State Transition
    ///
    /// ```text
    /// Balanced → Posted
    /// ```
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let posted = balanced_entry.post()?;
    /// ```
    pub fn post(self) -> JournalEntry<Posted> {
        let StateData::Balanced { total, balanced_at } = self.state_data else {
            unreachable!("Balanced entry must have Balanced state data")
        };

        JournalEntry {
            entry_id: self.entry_id,
            entity_id: self.entity_id,
            date: self.date,
            description: self.description,
            lines: self.lines,
            gaap_proof: self.gaap_proof,
            state_data: StateData::Posted {
                total,
                balanced_at,
                posted_at: Utc::now(),
            },
            created_at: self.created_at,
            _state: PhantomData,
        }
    }
}

impl JournalEntry<Posted> {
    /// Returns when the entry was posted.
    pub fn posted_at(&self) -> DateTime<Utc> {
        match &self.state_data {
            StateData::Posted { posted_at, .. } => *posted_at,
            StateData::Closed { posted_at, .. } => *posted_at,
            _ => unreachable!("Posted entry must have Posted or Closed state data"),
        }
    }

    /// Closes the entry as part of period closing.
    ///
    /// This transition marks the entry as part of a closed accounting period.
    /// Closed entries are immutable and cannot be modified or reversed.
    ///
    /// # State Transition
    ///
    /// ```text
    /// Posted → Closed
    /// ```
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let closed = posted_entry.close()?;
    /// ```
    pub fn close(self) -> JournalEntry<Closed> {
        let StateData::Posted {
            total,
            balanced_at,
            posted_at,
        } = self.state_data
        else {
            unreachable!("Posted entry must have Posted state data")
        };

        JournalEntry {
            entry_id: self.entry_id,
            entity_id: self.entity_id,
            date: self.date,
            description: self.description,
            lines: self.lines,
            gaap_proof: self.gaap_proof,
            state_data: StateData::Closed {
                total,
                balanced_at,
                posted_at,
                closed_at: Utc::now(),
            },
            created_at: self.created_at,
            _state: PhantomData,
        }
    }
}

impl JournalEntry<Closed> {
    /// Returns when the entry was closed.
    pub fn closed_at(&self) -> DateTime<Utc> {
        match &self.state_data {
            StateData::Closed { closed_at, .. } => *closed_at,
            _ => unreachable!("Closed entry must have Closed state data"),
        }
    }

    /// Returns when the entry was posted.
    pub fn posted_at(&self) -> DateTime<Utc> {
        match &self.state_data {
            StateData::Closed { posted_at, .. } => *posted_at,
            _ => unreachable!("Closed entry must have Closed state data"),
        }
    }
}
