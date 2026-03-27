//! Double-entry ledger with typestate state machines and proof-carrying contracts.
//!
//! This module demonstrates the elicitation framework's typestate pattern applied
//! to bookkeeping. A transfer progresses through distinct states:
//!
//! ```text
//! Pending ──validate()──> Validated ──commit()──> Committed
//!    │                          │
//!    └────reject()──────────────┴──rollback()──> Rejected
//! ```
//!
//! Each state transition requires establishing proofs of preconditions, enforced
//! at compile time via the type system.
//!
//! # Example
//!
//! ```rust,ignore
//! use elicit_server::ledger::{Transfer, AccountId, Amount, TransferId};
//!
//! // Create pending transfer
//! let transfer = Transfer::new(
//!     AccountId::new("Alice"),
//!     AccountId::new("Bob"),
//!     Amount::new(100)?,
//!     TransferId::new("tx1"),
//! );
//!
//! // Validate (establishes ValidTransfer proof)
//! let validated = transfer.validate(&pool).await?;
//!
//! // Commit (requires ValidTransfer proof)
//! let committed = validated.commit(&pool).await?;
//!
//! // Verify double-entry invariant
//! assert!(committed.verify_invariant());
//! ```

mod contracts;
mod errors;
mod types;
mod typestate;

pub use contracts::{
    AccountsDistinct, AmountPositive, BalancedEntries, SufficientFunds, ValidTransfer,
};
pub use errors::{CommitError, RejectionReason, ValidationError};
pub use types::{AccountId, Amount, TransferId};
pub use typestate::{Committed, Pending, Rejected, Transfer, Validated};
