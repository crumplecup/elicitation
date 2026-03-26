//! Typestate state machine for ledger transfers.
//!
//! Each state is a distinct type with state-specific fields. This encodes
//! invariants at compile time - a `Committed` transfer ALWAYS has balances
//! captured, not `Option<i64>`.

use std::marker::PhantomData;

use elicitation::contracts::{both, Established};

use crate::ledger::contracts::{
    AccountsDistinct, AmountPositive, BalancedEntries, SufficientFunds, ValidTransfer,
};
use crate::ledger::errors::{CommitError, RejectionReason, ValidationError};
use crate::ledger::types::{AccountId, Amount, TransferId};

// ─────────────────────────────────────────────────────────────
//  State Marker Types
// ─────────────────────────────────────────────────────────────

/// Marker: Transfer is pending validation.
pub struct Pending;

/// Marker: Transfer has been validated and is ready to commit.
pub struct Validated;

/// Marker: Transfer has been committed to the ledger.
pub struct Committed;

/// Marker: Transfer was rejected (validation failed or manual rollback).
pub struct Rejected;

// ─────────────────────────────────────────────────────────────
//  Transfer<S> - Typestate State Machine
// ─────────────────────────────────────────────────────────────

/// Transfer in a specific state `S`.
///
/// The state determines which operations are available:
/// - `Transfer<Pending>`: Can validate() or reject()
/// - `Transfer<Validated>`: Can commit() or rollback()
/// - `Transfer<Committed>`: Read-only, can verify invariants
/// - `Transfer<Rejected>`: Read-only, terminal state
pub struct Transfer<S> {
    /// Source account.
    pub from_account: AccountId,
    /// Destination account.
    pub to_account: AccountId,
    /// Transfer amount (positive).
    pub amount: Amount,
    /// Transfer identifier (must be unique).
    pub transfer_id: TransferId,
    /// State-specific data (captured during transitions).
    pub state_data: StateData<S>,
    /// Zero-sized state marker.
    _state: PhantomData<S>,
}

/// State-specific data captured during transitions.
pub enum StateData<S> {
    /// Pending: No additional data.
    Pending(PendingData),
    /// Validated: Source balance captured during validation.
    Validated(ValidatedData),
    /// Committed: Final balances captured after commit.
    Committed(CommittedData),
    /// Rejected: Rejection reason.
    Rejected(RejectedData),
    /// Phantom marker for type safety.
    _Phantom(PhantomData<S>),
}

/// Data for Pending state (empty).
pub struct PendingData;

/// Data for Validated state.
pub struct ValidatedData {
    /// Source account balance at validation time.
    pub from_balance: i64,
}

/// Data for Committed state.
pub struct CommittedData {
    /// Source account balance before transfer.
    pub from_balance_before: i64,
    /// Source account balance after transfer.
    pub from_balance_after: i64,
    /// Destination account balance after transfer.
    pub to_balance_after: i64,
}

/// Data for Rejected state.
pub struct RejectedData {
    /// Reason for rejection.
    pub reason: RejectionReason,
}

// ─────────────────────────────────────────────────────────────
//  Transfer<Pending> - Initial State
// ─────────────────────────────────────────────────────────────

impl Transfer<Pending> {
    /// Creates a new pending transfer.
    pub fn new(
        from: AccountId,
        to: AccountId,
        amount: Amount,
        transfer_id: TransferId,
    ) -> Self {
        Self {
            from_account: from,
            to_account: to,
            amount,
            transfer_id,
            state_data: StateData::Pending(PendingData),
            _state: PhantomData,
        }
    }

    /// Validates the transfer (establishes ValidTransfer proof).
    ///
    /// This is a non-async version for testing. The full async version
    /// with database queries is in the `sqlx-types` feature.
    pub fn validate_sync(
        self,
        from_balance: i64,
    ) -> Result<Transfer<Validated>, ValidationError> {
        // Validate amount is positive
        let _amount_proof = validate_amount_positive(&self)?;

        // Validate sufficient funds
        let _funds_proof = validate_sufficient_funds(&self, from_balance)?;

        // Validate accounts distinct
        let _distinct_proof = validate_accounts_distinct(&self)?;

        // All validations passed - transition to Validated
        Ok(Transfer {
            from_account: self.from_account,
            to_account: self.to_account,
            amount: self.amount,
            transfer_id: self.transfer_id,
            state_data: StateData::Validated(ValidatedData { from_balance }),
            _state: PhantomData,
        })
    }

    /// Validates the transfer with database queries (async version).
    pub async fn validate(
        self,
        pool: &sqlx::AnyPool,
    ) -> Result<Transfer<Validated>, ValidationError> {
        use sqlx::Row as _;

        // Validate amount is positive
        let amount_proof = validate_amount_positive(&self)?;

        // Query current balance
        let row = sqlx::query(
            "SELECT COALESCE(SUM(amount), 0) as balance
             FROM ledger_entries
             WHERE account_name = ?",
        )
        .bind(&self.from_account.0)
        .fetch_one(pool)
        .await?;

        let balance: i64 = row.get("balance");

        // Validate sufficient funds
        let funds_proof = validate_sufficient_funds(&self, balance)?;

        // Validate accounts distinct
        let distinct_proof = validate_accounts_distinct(&self)?;

        // Compose proofs: And<AmountPositive, And<SufficientFunds, AccountsDistinct>>
        let _funds_and_distinct = both(funds_proof, distinct_proof);
        let _valid_proof: Established<ValidTransfer> = both(amount_proof, _funds_and_distinct);

        // All validations passed - transition to Validated
        Ok(Transfer {
            from_account: self.from_account,
            to_account: self.to_account,
            amount: self.amount,
            transfer_id: self.transfer_id,
            state_data: StateData::Validated(ValidatedData {
                from_balance: balance,
            }),
            _state: PhantomData,
        })
    }

    /// Rejects the transfer without validation.
    pub fn reject(self, reason: RejectionReason) -> Transfer<Rejected> {
        Transfer {
            from_account: self.from_account,
            to_account: self.to_account,
            amount: self.amount,
            transfer_id: self.transfer_id,
            state_data: StateData::Rejected(RejectedData { reason }),
            _state: PhantomData,
        }
    }
}

// ─────────────────────────────────────────────────────────────
//  Transfer<Validated> - Ready to Commit
// ─────────────────────────────────────────────────────────────

impl Transfer<Validated> {
    /// Returns the source account balance at validation time.
    pub fn from_balance(&self) -> i64 {
        match &self.state_data {
            StateData::Validated(data) => data.from_balance,
            _ => unreachable!("Validated transfer must have ValidatedData"),
        }
    }

    /// Commits the transfer to the ledger (async version with database).
    pub async fn commit(self, pool: &sqlx::AnyPool) -> Result<Transfer<Committed>, CommitError> {
        use sqlx::Row as _;

        let from_balance_before = self.from_balance();

        // Begin transaction
        let mut tx = pool.begin().await?;

        // Insert debit entry
        sqlx::query(
            "INSERT INTO ledger_entries (account_name, amount, transfer_id, created_at)
             VALUES (?, ?, ?, ?)",
        )
        .bind(&self.from_account.0)
        .bind(-self.amount.0) // Negative for debit
        .bind(&self.transfer_id.0)
        .bind(chrono::Utc::now().timestamp())
        .execute(&mut *tx)
        .await?;

        // Insert credit entry
        sqlx::query(
            "INSERT INTO ledger_entries (account_name, amount, transfer_id, created_at)
             VALUES (?, ?, ?, ?)",
        )
        .bind(&self.to_account.0)
        .bind(self.amount.0) // Positive for credit
        .bind(&self.transfer_id.0)
        .bind(chrono::Utc::now().timestamp())
        .execute(&mut *tx)
        .await?;

        // Commit transaction
        tx.commit().await?;

        // Query final balances
        let from_row = sqlx::query(
            "SELECT COALESCE(SUM(amount), 0) as balance
             FROM ledger_entries
             WHERE account_name = ?",
        )
        .bind(&self.from_account.0)
        .fetch_one(pool)
        .await?;

        let to_row = sqlx::query(
            "SELECT COALESCE(SUM(amount), 0) as balance
             FROM ledger_entries
             WHERE account_name = ?",
        )
        .bind(&self.to_account.0)
        .fetch_one(pool)
        .await?;

        let from_balance_after: i64 = from_row.get("balance");
        let to_balance_after: i64 = to_row.get("balance");

        Ok(Transfer {
            from_account: self.from_account,
            to_account: self.to_account,
            amount: self.amount,
            transfer_id: self.transfer_id,
            state_data: StateData::Committed(CommittedData {
                from_balance_before,
                from_balance_after,
                to_balance_after,
            }),
            _state: PhantomData,
        })
    }

    /// Rolls back the validated transfer without committing.
    pub fn rollback(self, reason: RejectionReason) -> Transfer<Rejected> {
        Transfer {
            from_account: self.from_account,
            to_account: self.to_account,
            amount: self.amount,
            transfer_id: self.transfer_id,
            state_data: StateData::Rejected(RejectedData { reason }),
            _state: PhantomData,
        }
    }
}

// ─────────────────────────────────────────────────────────────
//  Transfer<Committed> - Terminal Success State
// ─────────────────────────────────────────────────────────────

impl Transfer<Committed> {
    /// Returns the committed data.
    pub fn committed_data(&self) -> &CommittedData {
        match &self.state_data {
            StateData::Committed(data) => data,
            _ => unreachable!("Committed transfer must have CommittedData"),
        }
    }

    /// Returns the balance change for the source account.
    pub fn from_balance_delta(&self) -> i64 {
        let data = self.committed_data();
        data.from_balance_after - data.from_balance_before
    }

    /// Verifies the double-entry invariant was preserved.
    ///
    /// The debit amount should equal the transfer amount (negative).
    pub fn verify_invariant(&self) -> bool {
        self.from_balance_delta() == -self.amount.0
    }
}

// ─────────────────────────────────────────────────────────────
//  Transfer<Rejected> - Terminal Failure State
// ─────────────────────────────────────────────────────────────

impl Transfer<Rejected> {
    /// Returns the rejection reason.
    pub fn reason(&self) -> &RejectionReason {
        match &self.state_data {
            StateData::Rejected(data) => &data.reason,
            _ => unreachable!("Rejected transfer must have RejectedData"),
        }
    }
}

// ─────────────────────────────────────────────────────────────
//  Validation Functions (Establish Proofs)
// ─────────────────────────────────────────────────────────────

/// Validates that the amount is positive.
fn validate_amount_positive(
    transfer: &Transfer<Pending>,
) -> Result<Established<AmountPositive>, ValidationError> {
    if transfer.amount.0 <= 0 {
        Err(ValidationError::NegativeAmount(transfer.amount.0))
    } else {
        Ok(Established::assert())
    }
}

/// Validates that the account has sufficient funds.
fn validate_sufficient_funds(
    transfer: &Transfer<Pending>,
    balance: i64,
) -> Result<Established<SufficientFunds>, ValidationError> {
    if balance < transfer.amount.0 {
        Err(ValidationError::InsufficientFunds {
            balance,
            required: transfer.amount.0,
        })
    } else {
        Ok(Established::assert())
    }
}

/// Validates that the accounts are distinct.
fn validate_accounts_distinct(
    transfer: &Transfer<Pending>,
) -> Result<Established<AccountsDistinct>, ValidationError> {
    if transfer.from_account == transfer.to_account {
        Err(ValidationError::SameAccount)
    } else {
        Ok(Established::assert())
    }
}

/// Establishes that entries are balanced (for committed transfers).
#[allow(dead_code)]
fn establish_balanced_entries(
    debit: i64,
    credit: i64,
) -> Result<Established<BalancedEntries>, ValidationError> {
    if debit + credit == 0 {
        Ok(Established::assert())
    } else {
        Err(ValidationError::Database(format!(
            "Entries not balanced: debit={}, credit={}",
            debit, credit
        )))
    }
}
