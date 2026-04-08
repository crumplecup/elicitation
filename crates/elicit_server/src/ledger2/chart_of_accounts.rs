//! Chart of accounts structure and standard templates.

use std::collections::BTreeMap;

use tracing::instrument;

use crate::ledger2::account::{Account, AccountNumber, EntityId};
use crate::ledger2::account_types::{
    AccountClass, AssetType, CurrentAsset, CurrentLiability, EquityType, ExpenseType, FixedAsset,
    LiabilityType, RevenueType,
};

// ─────────────────────────────────────────────────────────────
//  Chart of Accounts
// ─────────────────────────────────────────────────────────────

/// Chart of accounts for an entity.
///
/// A chart of accounts is the complete list of accounts used by an entity.
/// Accounts are organized hierarchically (parent/child relationships) and
/// grouped by GAAP account class (Asset, Liability, Equity, Revenue, Expense).
///
/// # Example
///
/// ```rust,ignore
/// let chart = ChartOfAccounts::standard_small_business(entity_id);
///
/// // Query accounts by number
/// let cash = chart.get_account("1100").expect("Cash account");
///
/// // Query accounts by class
/// let assets = chart.accounts_by_class(AccountClass::Asset);
/// ```
#[derive(Debug, Clone)]
pub struct ChartOfAccounts {
    /// Entity this chart belongs to.
    entity_id: EntityId,

    /// All accounts indexed by account number.
    accounts: BTreeMap<AccountNumber, Account>,
}

impl ChartOfAccounts {
    /// Creates a new empty chart of accounts.
    pub fn new(entity_id: EntityId) -> Self {
        Self {
            entity_id,
            accounts: BTreeMap::new(),
        }
    }

    /// Returns the entity ID for this chart.
    pub fn entity_id(&self) -> EntityId {
        self.entity_id
    }

    /// Returns the number of accounts in this chart.
    pub fn len(&self) -> usize {
        self.accounts.len()
    }

    /// Returns true if the chart contains no accounts.
    pub fn is_empty(&self) -> bool {
        self.accounts.is_empty()
    }

    /// Gets an account by number.
    pub fn get_account(&self, number: &str) -> Option<&Account> {
        self.accounts.get(&AccountNumber::new(number))
    }

    /// Adds an account to the chart.
    #[instrument(skip(self, account), fields(account_number = %account.number()))]
    pub fn add_account(&mut self, account: Account) -> Result<(), ChartError> {
        // Verify account belongs to this entity
        if *account.entity_id() != self.entity_id {
            return Err(ChartError::EntityMismatch {
                chart_entity: self.entity_id,
                account_entity: *account.entity_id(),
            });
        }

        let account_number = account.number().clone();
        let account_name = account.name();

        // Check for duplicate account numbers
        if self.accounts.contains_key(&account_number) {
            return Err(ChartError::DuplicateAccountNumber(account_number));
        }

        // Verify parent exists if specified
        if let Some(parent) = account.parent() {
            if !self.accounts.contains_key(parent) {
                return Err(ChartError::ParentNotFound(parent.clone()));
            }
        }

        tracing::debug!(
            account_number = %account_number,
            account_name = %account_name,
            "Added account to chart"
        );

        self.accounts.insert(account_number, account);
        Ok(())
    }

    /// Returns all accounts in the chart.
    pub fn accounts(&self) -> impl Iterator<Item = &Account> {
        self.accounts.values()
    }

    /// Returns accounts filtered by account class.
    pub fn accounts_by_class(&self, class_filter: fn(&AccountClass) -> bool) -> Vec<&Account> {
        self.accounts
            .values()
            .filter(|account| class_filter(account.class()))
            .collect()
    }

    /// Returns all asset accounts.
    pub fn asset_accounts(&self) -> Vec<&Account> {
        self.accounts_by_class(|class| class.is_asset())
    }

    /// Returns all liability accounts.
    pub fn liability_accounts(&self) -> Vec<&Account> {
        self.accounts_by_class(|class| class.is_liability())
    }

    /// Returns all equity accounts.
    pub fn equity_accounts(&self) -> Vec<&Account> {
        self.accounts_by_class(|class| class.is_equity())
    }

    /// Returns all revenue accounts.
    pub fn revenue_accounts(&self) -> Vec<&Account> {
        self.accounts_by_class(|class| class.is_revenue())
    }

    /// Returns all expense accounts.
    pub fn expense_accounts(&self) -> Vec<&Account> {
        self.accounts_by_class(|class| class.is_expense())
    }

    /// Returns all child accounts of a parent account.
    pub fn child_accounts(&self, parent_number: &str) -> Vec<&Account> {
        let parent_num = AccountNumber::new(parent_number);
        self.accounts
            .values()
            .filter(|account| account.parent() == Some(&parent_num))
            .collect()
    }

    /// Creates a standard chart of accounts for a small business.
    pub fn standard_small_business(entity_id: EntityId) -> Self {
        let mut chart = Self::new(entity_id);

        // Assets (1000-1999)
        chart
            .add_account(
                Account::builder()
                    .number("1000")
                    .name("Assets")
                    .class(AccountClass::Asset(AssetType::CurrentAsset(
                        CurrentAsset::Cash,
                    )))
                    .entity_id(entity_id)
                    .active(false) // Header account
                    .build()
                    .expect("Valid account"),
            )
            .expect("Add account");

        // Current Assets (1100-1199)
        chart
            .add_account(
                Account::builder()
                    .number("1100")
                    .name("Current Assets")
                    .class(AccountClass::Asset(AssetType::CurrentAsset(
                        CurrentAsset::Cash,
                    )))
                    .parent("1000")
                    .entity_id(entity_id)
                    .active(false)
                    .build()
                    .expect("Valid account"),
            )
            .expect("Add account");

        chart
            .add_account(
                Account::builder()
                    .number("1110")
                    .name("Cash")
                    .class(AccountClass::Asset(AssetType::CurrentAsset(
                        CurrentAsset::Cash,
                    )))
                    .parent("1100")
                    .entity_id(entity_id)
                    .build()
                    .expect("Valid account"),
            )
            .expect("Add account");

        chart
            .add_account(
                Account::builder()
                    .number("1120")
                    .name("Accounts Receivable")
                    .class(AccountClass::Asset(AssetType::CurrentAsset(
                        CurrentAsset::AccountsReceivable,
                    )))
                    .parent("1100")
                    .entity_id(entity_id)
                    .build()
                    .expect("Valid account"),
            )
            .expect("Add account");

        chart
            .add_account(
                Account::builder()
                    .number("1130")
                    .name("Inventory")
                    .class(AccountClass::Asset(AssetType::CurrentAsset(
                        CurrentAsset::Inventory,
                    )))
                    .parent("1100")
                    .entity_id(entity_id)
                    .build()
                    .expect("Valid account"),
            )
            .expect("Add account");

        // Fixed Assets (1200-1299)
        chart
            .add_account(
                Account::builder()
                    .number("1200")
                    .name("Fixed Assets")
                    .class(AccountClass::Asset(AssetType::FixedAsset(FixedAsset::Land)))
                    .parent("1000")
                    .entity_id(entity_id)
                    .active(false)
                    .build()
                    .expect("Valid account"),
            )
            .expect("Add account");

        chart
            .add_account(
                Account::builder()
                    .number("1210")
                    .name("Equipment")
                    .class(AccountClass::Asset(AssetType::FixedAsset(
                        FixedAsset::Equipment,
                    )))
                    .parent("1200")
                    .entity_id(entity_id)
                    .build()
                    .expect("Valid account"),
            )
            .expect("Add account");

        chart
            .add_account(
                Account::builder()
                    .number("1220")
                    .name("Accumulated Depreciation - Equipment")
                    .class(AccountClass::Asset(AssetType::FixedAsset(
                        FixedAsset::AccumulatedDepreciation,
                    )))
                    .parent("1200")
                    .entity_id(entity_id)
                    .build()
                    .expect("Valid account"),
            )
            .expect("Add account");

        // Liabilities (2000-2999)
        chart
            .add_account(
                Account::builder()
                    .number("2000")
                    .name("Liabilities")
                    .class(AccountClass::Liability(LiabilityType::CurrentLiability(
                        CurrentLiability::AccountsPayable,
                    )))
                    .entity_id(entity_id)
                    .active(false)
                    .build()
                    .expect("Valid account"),
            )
            .expect("Add account");

        // Current Liabilities (2100-2199)
        chart
            .add_account(
                Account::builder()
                    .number("2100")
                    .name("Current Liabilities")
                    .class(AccountClass::Liability(LiabilityType::CurrentLiability(
                        CurrentLiability::AccountsPayable,
                    )))
                    .parent("2000")
                    .entity_id(entity_id)
                    .active(false)
                    .build()
                    .expect("Valid account"),
            )
            .expect("Add account");

        chart
            .add_account(
                Account::builder()
                    .number("2110")
                    .name("Accounts Payable")
                    .class(AccountClass::Liability(LiabilityType::CurrentLiability(
                        CurrentLiability::AccountsPayable,
                    )))
                    .parent("2100")
                    .entity_id(entity_id)
                    .build()
                    .expect("Valid account"),
            )
            .expect("Add account");

        chart
            .add_account(
                Account::builder()
                    .number("2120")
                    .name("Accrued Expenses")
                    .class(AccountClass::Liability(LiabilityType::CurrentLiability(
                        CurrentLiability::AccruedExpense,
                    )))
                    .parent("2100")
                    .entity_id(entity_id)
                    .build()
                    .expect("Valid account"),
            )
            .expect("Add account");

        // Equity (3000-3999)
        chart
            .add_account(
                Account::builder()
                    .number("3000")
                    .name("Equity")
                    .class(AccountClass::Equity(EquityType::CommonStock))
                    .entity_id(entity_id)
                    .active(false)
                    .build()
                    .expect("Valid account"),
            )
            .expect("Add account");

        chart
            .add_account(
                Account::builder()
                    .number("3100")
                    .name("Common Stock")
                    .class(AccountClass::Equity(EquityType::CommonStock))
                    .parent("3000")
                    .entity_id(entity_id)
                    .build()
                    .expect("Valid account"),
            )
            .expect("Add account");

        chart
            .add_account(
                Account::builder()
                    .number("3200")
                    .name("Retained Earnings")
                    .class(AccountClass::Equity(EquityType::RetainedEarnings))
                    .parent("3000")
                    .entity_id(entity_id)
                    .build()
                    .expect("Valid account"),
            )
            .expect("Add account");

        // Revenue (4000-4999)
        chart
            .add_account(
                Account::builder()
                    .number("4000")
                    .name("Revenue")
                    .class(AccountClass::Revenue(RevenueType::Sales))
                    .entity_id(entity_id)
                    .active(false)
                    .build()
                    .expect("Valid account"),
            )
            .expect("Add account");

        chart
            .add_account(
                Account::builder()
                    .number("4100")
                    .name("Sales Revenue")
                    .class(AccountClass::Revenue(RevenueType::Sales))
                    .parent("4000")
                    .entity_id(entity_id)
                    .build()
                    .expect("Valid account"),
            )
            .expect("Add account");

        chart
            .add_account(
                Account::builder()
                    .number("4200")
                    .name("Service Revenue")
                    .class(AccountClass::Revenue(RevenueType::ServiceRevenue))
                    .parent("4000")
                    .entity_id(entity_id)
                    .build()
                    .expect("Valid account"),
            )
            .expect("Add account");

        // Expenses (5000-5999)
        chart
            .add_account(
                Account::builder()
                    .number("5000")
                    .name("Expenses")
                    .class(AccountClass::Expense(ExpenseType::CostOfGoodsSold))
                    .entity_id(entity_id)
                    .active(false)
                    .build()
                    .expect("Valid account"),
            )
            .expect("Add account");

        chart
            .add_account(
                Account::builder()
                    .number("5100")
                    .name("Cost of Goods Sold")
                    .class(AccountClass::Expense(ExpenseType::CostOfGoodsSold))
                    .parent("5000")
                    .entity_id(entity_id)
                    .build()
                    .expect("Valid account"),
            )
            .expect("Add account");

        chart
            .add_account(
                Account::builder()
                    .number("5200")
                    .name("Salaries and Wages")
                    .class(AccountClass::Expense(ExpenseType::Salaries))
                    .parent("5000")
                    .entity_id(entity_id)
                    .build()
                    .expect("Valid account"),
            )
            .expect("Add account");

        chart
            .add_account(
                Account::builder()
                    .number("5300")
                    .name("Rent Expense")
                    .class(AccountClass::Expense(ExpenseType::Rent))
                    .parent("5000")
                    .entity_id(entity_id)
                    .build()
                    .expect("Valid account"),
            )
            .expect("Add account");

        chart
            .add_account(
                Account::builder()
                    .number("5400")
                    .name("Utilities Expense")
                    .class(AccountClass::Expense(ExpenseType::Utilities))
                    .parent("5000")
                    .entity_id(entity_id)
                    .build()
                    .expect("Valid account"),
            )
            .expect("Add account");

        chart
            .add_account(
                Account::builder()
                    .number("5500")
                    .name("Depreciation Expense")
                    .class(AccountClass::Expense(ExpenseType::Depreciation))
                    .parent("5000")
                    .entity_id(entity_id)
                    .build()
                    .expect("Valid account"),
            )
            .expect("Add account");

        chart
    }
}

// ─────────────────────────────────────────────────────────────
//  Errors
// ─────────────────────────────────────────────────────────────

/// Error type for chart of accounts operations.
#[derive(Debug, Clone)]
pub enum ChartError {
    /// Account belongs to different entity than chart.
    EntityMismatch {
        /// Entity ID of the chart.
        chart_entity: EntityId,
        /// Entity ID of the account.
        account_entity: EntityId,
    },

    /// Duplicate account number in chart.
    DuplicateAccountNumber(AccountNumber),

    /// Parent account not found.
    ParentNotFound(AccountNumber),
}

impl std::fmt::Display for ChartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChartError::EntityMismatch {
                chart_entity,
                account_entity,
            } => write!(
                f,
                "Entity mismatch: chart belongs to {}, account belongs to {}",
                chart_entity, account_entity
            ),
            ChartError::DuplicateAccountNumber(number) => {
                write!(f, "Duplicate account number: {}", number)
            }
            ChartError::ParentNotFound(number) => {
                write!(f, "Parent account not found: {}", number)
            }
        }
    }
}

impl std::error::Error for ChartError {}
