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
//!
//! // Create entity and chart of accounts
//! let entity_id = EntityId::new_v4();
//! let chart = ChartOfAccounts::standard_small_business(entity_id);
//!
//! // Get accounts
//! let cash = chart.get_account("1110").expect("Cash account");
//! let revenue = chart.get_account("4100").expect("Sales revenue");
//! ```

mod account;
mod account_types;
mod chart_of_accounts;

pub use account::{Account, AccountBuilder, AccountBuilderError, AccountNumber, EntityId};
pub use account_types::{
    AccountClass, AssetType, CurrentAsset, CurrentLiability, DebitCredit, EquityType, ExpenseType,
    FixedAsset, IntangibleAsset, LiabilityType, LongTermLiability, NormalBalance, RevenueType,
};
pub use chart_of_accounts::{ChartError, ChartOfAccounts};
