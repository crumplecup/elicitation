//! Core domain types for the ledger.

use derive_more::Display;
use serde::{Deserialize, Serialize};

/// Account identifier (string-based for simplicity).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AccountId(pub String);

impl AccountId {
    /// Creates a new account identifier.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

/// Transfer amount (positive integer, in smallest currency unit).
///
/// Contract: Always positive (> 0).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Amount(pub i64);

/// Error when creating an invalid amount.
#[derive(Debug, Clone, Copy, Display)]
#[display("Amount must be positive, got {}", _0)]
pub struct InvalidAmount(pub i64);

impl std::error::Error for InvalidAmount {}

impl Amount {
    /// Creates a new amount.
    ///
    /// Returns error if amount is not positive (≤ 0).
    pub fn new(value: i64) -> Result<Self, InvalidAmount> {
        if value <= 0 {
            Err(InvalidAmount(value))
        } else {
            Ok(Self(value))
        }
    }

    /// Returns the inner value.
    pub fn value(&self) -> i64 {
        self.0
    }
}

/// Transfer identifier (string-based for simplicity).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TransferId(pub String);

impl TransferId {
    /// Creates a new transfer identifier.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}
