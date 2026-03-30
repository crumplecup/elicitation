//! Error types for ledger operations.

use derive_more::Display;

/// Validation error when checking transfer preconditions.
#[derive(Debug, Clone, Display)]
pub enum ValidationError {
    /// Amount is not positive.
    #[display("Amount must be positive, got {}", _0)]
    NegativeAmount(i64),

    /// Insufficient funds in source account.
    #[display("Insufficient funds: balance={}, required={}", balance, required)]
    InsufficientFunds {
        /// Current account balance.
        balance: i64,
        /// Amount required for transfer.
        required: i64,
    },

    /// Source and destination accounts are the same.
    #[display("Cannot transfer to same account")]
    SameAccount,

    /// Database error during validation.
    #[display("Database error: {}", _0)]
    Database(String),
}

/// Commit error when executing transfer.
#[derive(Debug, Clone, Display)]
pub enum CommitError {
    /// Database error during commit.
    #[display("Database error: {}", _0)]
    Database(String),
}

impl std::error::Error for ValidationError {}

impl std::error::Error for CommitError {}

/// Reason for rejecting a transfer.
#[derive(Debug, Clone, Display)]
pub enum RejectionReason {
    /// Validation failed.
    #[display("Validation failed: {}", _0)]
    ValidationFailed(ValidationError),

    /// Manual rollback before commit.
    #[display("Manual rollback")]
    ManualRollback,
}

impl From<sqlx::Error> for ValidationError {
    fn from(err: sqlx::Error) -> Self {
        Self::Database(err.to_string())
    }
}

impl From<sqlx::Error> for CommitError {
    fn from(err: sqlx::Error) -> Self {
        Self::Database(err.to_string())
    }
}
