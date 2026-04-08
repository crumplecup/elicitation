//! Account structure and builder for GAAP-native ledger.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ledger2::account_types::{AccountClass, NormalBalance};

// ─────────────────────────────────────────────────────────────
//  Account Identifiers
// ─────────────────────────────────────────────────────────────

/// Account number (e.g., "1000", "1100", "4100").
///
/// Account numbers typically follow a numbering scheme:
/// - 1000-1999: Assets
/// - 2000-2999: Liabilities
/// - 3000-3999: Equity
/// - 4000-4999: Revenue
/// - 5000-5999: Expenses
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AccountNumber(pub String);

impl AccountNumber {
    /// Creates a new account number.
    pub fn new(number: impl Into<String>) -> Self {
        Self(number.into())
    }
}

impl std::fmt::Display for AccountNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Entity identifier for multi-entity support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntityId(pub Uuid);

impl EntityId {
    /// Creates a new random entity ID.
    pub fn new_v4() -> Self {
        Self(Uuid::new_v4())
    }

    /// Creates an entity ID from a UUID.
    pub fn from_uuid(id: Uuid) -> Self {
        Self(id)
    }
}

impl std::fmt::Display for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ─────────────────────────────────────────────────────────────
//  Account Structure
// ─────────────────────────────────────────────────────────────

/// An account in the chart of accounts.
///
/// Accounts are the fundamental building blocks of double-entry bookkeeping.
/// Each account has a type (Asset, Liability, Equity, Revenue, Expense) that
/// determines its normal balance and where it appears in financial statements.
///
/// # Example
///
/// ```rust,ignore
/// use elicit_server::ledger2::{Account, AccountClass, CurrentAsset, AssetType};
///
/// let cash_account = Account::builder()
///     .number("1100")
///     .name("Cash")
///     .class(AccountClass::Asset(AssetType::CurrentAsset(CurrentAsset::Cash)))
///     .entity_id(entity_id)
///     .build()
///     .expect("Valid account");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Account {
    /// Account number (e.g., "1100" for Cash).
    number: AccountNumber,

    /// Account name (e.g., "Cash", "Accounts Receivable").
    name: String,

    /// GAAP account class (Asset/Liability/Equity/Revenue/Expense).
    class: AccountClass,

    /// Parent account number (for hierarchical chart of accounts).
    parent: Option<AccountNumber>,

    /// Entity this account belongs to (for multi-entity support).
    entity_id: EntityId,

    /// Whether this account is active (inactive accounts cannot be used).
    active: bool,
}

impl Account {
    /// Returns the account number.
    pub fn number(&self) -> &AccountNumber {
        &self.number
    }

    /// Returns the account name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the account class.
    pub fn class(&self) -> &AccountClass {
        &self.class
    }

    /// Returns the parent account number, if any.
    pub fn parent(&self) -> Option<&AccountNumber> {
        self.parent.as_ref()
    }

    /// Returns the entity ID.
    pub fn entity_id(&self) -> &EntityId {
        &self.entity_id
    }

    /// Returns whether the account is active.
    pub fn active(&self) -> bool {
        self.active
    }

    /// Returns the normal balance for this account (debit or credit).
    pub fn normal_balance(&self) -> NormalBalance {
        self.class.normal_balance()
    }

    /// Returns true if this is a temporary account (closed at period end).
    pub fn is_temporary(&self) -> bool {
        self.class.is_temporary()
    }

    /// Returns true if this is a permanent account (carries balance across periods).
    pub fn is_permanent(&self) -> bool {
        self.class.is_permanent()
    }

    /// Creates a builder for constructing an account.
    pub fn builder() -> AccountBuilder {
        AccountBuilder::default()
    }
}

impl std::fmt::Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {} ({})", self.number, self.name, self.class)
    }
}

// ─────────────────────────────────────────────────────────────
//  Account Builder
// ─────────────────────────────────────────────────────────────

/// Builder for constructing accounts.
///
/// # Example
///
/// ```rust,ignore
/// let account = Account::builder()
///     .number("1100")
///     .name("Cash")
///     .class(AccountClass::Asset(AssetType::CurrentAsset(CurrentAsset::Cash)))
///     .entity_id(entity_id)
///     .active(true)
///     .build()
///     .expect("Valid account");
/// ```
#[derive(Debug)]
pub struct AccountBuilder {
    number: Option<AccountNumber>,
    name: Option<String>,
    class: Option<AccountClass>,
    parent: Option<AccountNumber>,
    entity_id: Option<EntityId>,
    active: bool,
}

impl Default for AccountBuilder {
    fn default() -> Self {
        Self {
            number: None,
            name: None,
            class: None,
            parent: None,
            entity_id: None,
            active: true, // Default to active
        }
    }
}

impl AccountBuilder {
    /// Sets the account number.
    pub fn number(mut self, number: impl Into<String>) -> Self {
        self.number = Some(AccountNumber::new(number));
        self
    }

    /// Sets the account name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Sets the account class (Asset/Liability/Equity/Revenue/Expense).
    pub fn class(mut self, class: AccountClass) -> Self {
        self.class = Some(class);
        self
    }

    /// Sets the parent account number (for hierarchical structure).
    pub fn parent(mut self, parent: impl Into<String>) -> Self {
        self.parent = Some(AccountNumber::new(parent));
        self
    }

    /// Sets the entity ID.
    pub fn entity_id(mut self, entity_id: EntityId) -> Self {
        self.entity_id = Some(entity_id);
        self
    }

    /// Sets whether the account is active (default: true).
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// Builds the account.
    pub fn build(self) -> Result<Account, AccountBuilderError> {
        let number = self
            .number
            .ok_or(AccountBuilderError::MissingField("number"))?;
        let name = self.name.ok_or(AccountBuilderError::MissingField("name"))?;
        let class = self
            .class
            .ok_or(AccountBuilderError::MissingField("class"))?;
        let entity_id = self
            .entity_id
            .ok_or(AccountBuilderError::MissingField("entity_id"))?;

        Ok(Account {
            number,
            name,
            class,
            parent: self.parent,
            entity_id,
            active: self.active,
        })
    }
}

/// Error type for account builder failures.
#[derive(Debug, Clone)]
pub enum AccountBuilderError {
    /// Required field is missing.
    MissingField(&'static str),
}

impl std::fmt::Display for AccountBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountBuilderError::MissingField(field) => {
                write!(f, "Missing required field: {}", field)
            }
        }
    }
}

impl std::error::Error for AccountBuilderError {}
