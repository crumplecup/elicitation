//! Error types for journal entry operations.

use derive_more::{Display, Error};

use crate::ledger2::Amount;

// ─────────────────────────────────────────────────────────────
//  Error Kinds
// ─────────────────────────────────────────────────────────────

/// Specific journal entry error conditions.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
pub enum JournalEntryErrorKind {
    /// Journal entry is not balanced (debits != credits).
    #[display(
        "Imbalance: debits {} != credits {} (difference: {})",
        debits,
        credits,
        difference
    )]
    Imbalance {
        /// Total debit amount.
        debits: Amount,
        /// Total credit amount.
        credits: Amount,
        /// Difference (debits - credits).
        difference: Amount,
    },

    /// Invalid state transition attempted.
    #[display("Invalid transition from {} to {}: {}", from, to, reason)]
    InvalidTransition {
        /// Current state.
        from: String,
        /// Target state.
        to: String,
        /// Reason the transition is invalid.
        reason: String,
    },

    /// Journal entry has no lines.
    #[display("Empty entry: journal entry must have at least two lines")]
    EmptyEntry,

    /// Journal entry has only one line (minimum is two for double-entry).
    #[display("Single line: journal entry must have at least two lines for double-entry")]
    SingleLine,

    /// GAAP validation failed.
    #[display("GAAP validation failed: {}", reason)]
    GaapValidation {
        /// Reason for validation failure.
        reason: String,
    },

    /// Entry is already posted and cannot be modified.
    #[display("Entry already posted: cannot modify posted entry {}", entry_id)]
    AlreadyPosted {
        /// Entry ID that is already posted.
        entry_id: String,
    },

    /// Entry is in closed period and cannot be modified.
    #[display("Closed period: cannot modify entry {} in closed period", entry_id)]
    ClosedPeriod {
        /// Entry ID in closed period.
        entry_id: String,
    },

    /// Account is inactive and cannot be used.
    #[display("Inactive account: account {} is inactive", account_number)]
    InactiveAccount {
        /// Account number that is inactive.
        account_number: String,
    },

    /// Accounts belong to different entities.
    #[display("Entity mismatch: accounts belong to different entities")]
    EntityMismatch,
}

// ─────────────────────────────────────────────────────────────
//  Error Wrapper
// ─────────────────────────────────────────────────────────────

/// Journal entry error with location tracking.
///
/// Wraps a [`JournalEntryErrorKind`] with file and line information for
/// debugging and audit trail purposes.
///
/// # Example
///
/// ```rust,ignore
/// use elicit_server::ledger2::{JournalEntryError, JournalEntryErrorKind, Amount};
///
/// let error = JournalEntryError::imbalance(
///     Amount::from_dollars(100),
///     Amount::from_dollars(50),
/// );
/// ```
#[derive(Debug, Clone, Display, Error)]
#[display("Journal entry error: {} at {}:{}", kind, file, line)]
pub struct JournalEntryError {
    /// The specific error condition.
    pub kind: JournalEntryErrorKind,
    /// Line number where error occurred.
    pub line: u32,
    /// Source file where error occurred.
    pub file: &'static str,
}

impl JournalEntryError {
    /// Creates a new journal entry error.
    #[track_caller]
    pub fn new(kind: JournalEntryErrorKind) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            kind,
            line: loc.line(),
            file: loc.file(),
        }
    }

    /// Creates an imbalance error.
    #[track_caller]
    pub fn imbalance(debits: Amount, credits: Amount) -> Self {
        Self::new(JournalEntryErrorKind::Imbalance {
            debits,
            credits,
            difference: debits - credits,
        })
    }

    /// Creates an invalid transition error.
    #[track_caller]
    pub fn invalid_transition(
        from: impl Into<String>,
        to: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self::new(JournalEntryErrorKind::InvalidTransition {
            from: from.into(),
            to: to.into(),
            reason: reason.into(),
        })
    }

    /// Creates an empty entry error.
    #[track_caller]
    pub fn empty_entry() -> Self {
        Self::new(JournalEntryErrorKind::EmptyEntry)
    }

    /// Creates a single line error.
    #[track_caller]
    pub fn single_line() -> Self {
        Self::new(JournalEntryErrorKind::SingleLine)
    }

    /// Creates a GAAP validation error.
    #[track_caller]
    pub fn gaap_validation(reason: impl Into<String>) -> Self {
        Self::new(JournalEntryErrorKind::GaapValidation {
            reason: reason.into(),
        })
    }

    /// Creates an already posted error.
    #[track_caller]
    pub fn already_posted(entry_id: impl Into<String>) -> Self {
        Self::new(JournalEntryErrorKind::AlreadyPosted {
            entry_id: entry_id.into(),
        })
    }

    /// Creates a closed period error.
    #[track_caller]
    pub fn closed_period(entry_id: impl Into<String>) -> Self {
        Self::new(JournalEntryErrorKind::ClosedPeriod {
            entry_id: entry_id.into(),
        })
    }

    /// Creates an inactive account error.
    #[track_caller]
    pub fn inactive_account(account_number: impl Into<String>) -> Self {
        Self::new(JournalEntryErrorKind::InactiveAccount {
            account_number: account_number.into(),
        })
    }

    /// Creates an entity mismatch error.
    #[track_caller]
    pub fn entity_mismatch() -> Self {
        Self::new(JournalEntryErrorKind::EntityMismatch)
    }
}

/// Result type for journal entry operations.
pub type JournalEntryResult<T> = Result<T, JournalEntryError>;
