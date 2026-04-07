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

    /// GAAP double-entry bookkeeping violation.
    #[display(
        "GAAP violation (Double-Entry): debit ({}) != credit ({})",
        debit,
        credit
    )]
    GaapDoubleEntry {
        /// Debit amount.
        debit: i64,
        /// Credit amount.
        credit: i64,
    },

    /// GAAP accrual basis violation.
    #[display("GAAP violation (Accrual Basis): {}", reason)]
    GaapAccrualBasis {
        /// Reason for violation.
        reason: String,
    },

    /// GAAP monetary unit assumption violation.
    #[display("GAAP violation (Monetary Unit): {}", reason)]
    GaapMonetaryUnit {
        /// Reason for violation.
        reason: String,
    },

    /// GAAP matching principle violation.
    #[display("GAAP violation (Matching Principle): {}", reason)]
    GaapMatchingPrinciple {
        /// Reason for violation.
        reason: String,
    },

    /// GAAP economic entity assumption violation.
    #[display("GAAP violation (Economic Entity): {}", reason)]
    GaapEconomicEntity {
        /// Reason for violation.
        reason: String,
    },

    /// GAAP historical cost principle violation.
    #[display("GAAP violation (Historical Cost): {}", reason)]
    GaapHistoricalCost {
        /// Reason for violation.
        reason: String,
    },

    /// GAAP conservatism principle violation.
    #[display("GAAP violation (Conservatism): {}", reason)]
    GaapConservatism {
        /// Reason for violation.
        reason: String,
    },

    /// GAAP going concern assumption violation.
    #[display("GAAP violation (Going Concern): system not operational")]
    GaapGoingConcern,

    /// GAAP materiality principle violation.
    #[display(
        "GAAP violation (Materiality): amount {} exceeds threshold {}",
        amount,
        threshold
    )]
    GaapMateriality {
        /// Transaction amount.
        amount: i64,
        /// Materiality threshold.
        threshold: i64,
    },
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
