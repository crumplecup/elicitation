//! GAAP-native ledger with typesafe state machines.
//!
//! This module implements a production-grade accounting system where GAAP
//! (Generally Accepted Accounting Principles) types are the foundational IR.
//!
//! # Architecture
//!
//! 1. **GAAP as IR** - Account types (Asset/Liability/Equity/Revenue/Expense)
//!    are primitive, not validation layers
//! 2. **Typesafe state machines** - Journal entries transition through states
//!    with proofs carried at each stage
//! 3. **Double-entry by construction** - Builder pattern enforces balance
//! 4. **Audit-ready** - Every transaction carries GAAP compliance proof
//!
//! # Quick Start
//!
//! ```rust,ignore
//! use elicit_server::ledger2::*;
//! use chrono::NaiveDate;
//!
//! // Create entity and chart of accounts
//! let entity_id = EntityId::new_v4();
//! let chart = ChartOfAccounts::standard_small_business(entity_id);
//!
//! // Get accounts
//! let cash = chart.get_account("1110").expect("Cash account");
//! let revenue = chart.get_account("4100").expect("Sales revenue");
//!
//! // Create a journal entry (cash sale)
//! let entry = JournalEntry::builder(entity_id, NaiveDate::from_ymd(2024, 1, 15))
//!     .description("Cash sale")
//!     .debit(cash.clone(), Amount::from_dollars(100), "Payment received")
//!     .credit(revenue.clone(), Amount::from_dollars(100), "Sale of goods")
//!     .build()
//!     .expect("Valid entry");
//!
//! // Post to ledger
//! let posted = entry.post();
//! ```

mod account;
mod account_types;
mod builder;
mod chart_of_accounts;
mod errors;
mod journal_entry;
mod journal_line;

pub use account::{Account, AccountBuilder, AccountBuilderError, AccountNumber, EntityId};
pub use account_types::{
    AccountClass, AssetType, CurrentAsset, CurrentLiability, DebitCredit, EquityType, ExpenseType,
    FixedAsset, IntangibleAsset, LiabilityType, LongTermLiability, NormalBalance, RevenueType,
};
pub use builder::JournalEntryBuilder;
pub use chart_of_accounts::{ChartError, ChartOfAccounts};
pub use errors::{JournalEntryError, JournalEntryErrorKind, JournalEntryResult};
pub use journal_entry::{
    Balanced, Closed, Draft, EntryId, GaapProof, JournalEntry, Posted, StateData,
};
pub use journal_line::{Amount, JournalLine};
