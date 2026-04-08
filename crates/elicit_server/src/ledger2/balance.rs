//! Account balance tracking and queries.

use std::collections::HashMap;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::ledger2::{AccountNumber, Amount, DebitCredit, NormalBalance};

// ─────────────────────────────────────────────────────────────
//  Account Balance
// ─────────────────────────────────────────────────────────────

/// Balance for a specific account at a point in time.
///
/// Tracks the running balance for an account, computed from all posted
/// journal entries affecting that account. The balance is always computed
/// in the account's normal balance direction.
///
/// # Normal Balance Rules
///
/// - **Debit normal** (Assets, Expenses): Debits increase, credits decrease
/// - **Credit normal** (Liabilities, Equity, Revenue): Credits increase, debits decrease
///
/// # Example
///
/// ```rust,ignore
/// use elicit_server::ledger2::{AccountBalance, Amount};
/// use chrono::NaiveDate;
///
/// let balance = AccountBalance::new(
///     cash_account.number().clone(),
///     NaiveDate::from_ymd(2024, 1, 31),
/// );
///
/// // Post debit (increases cash, an asset)
/// balance.apply_debit(Amount::from_dollars(100));
/// assert_eq!(balance.balance(), Amount::from_dollars(100));
///
/// // Post credit (decreases cash)
/// balance.apply_credit(Amount::from_dollars(50));
/// assert_eq!(balance.balance(), Amount::from_dollars(50));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountBalance {
    /// Account number this balance belongs to.
    account_number: AccountNumber,

    /// Date this balance is computed as of.
    as_of_date: NaiveDate,

    /// Total debits posted to this account.
    total_debits: Amount,

    /// Total credits posted to this account.
    total_credits: Amount,

    /// Number of journal entries affecting this account.
    entry_count: u64,
}

impl AccountBalance {
    /// Creates a new account balance with zero amounts.
    pub fn new(account_number: AccountNumber, as_of_date: NaiveDate) -> Self {
        Self {
            account_number,
            as_of_date,
            total_debits: Amount::from_cents(0),
            total_credits: Amount::from_cents(0),
            entry_count: 0,
        }
    }

    /// Returns the account number.
    pub fn account_number(&self) -> &AccountNumber {
        &self.account_number
    }

    /// Returns the as-of date.
    pub fn as_of_date(&self) -> NaiveDate {
        self.as_of_date
    }

    /// Returns total debits.
    pub fn total_debits(&self) -> Amount {
        self.total_debits
    }

    /// Returns total credits.
    pub fn total_credits(&self) -> Amount {
        self.total_credits
    }

    /// Returns the number of entries affecting this account.
    pub fn entry_count(&self) -> u64 {
        self.entry_count
    }

    /// Returns the net balance (debits - credits).
    ///
    /// For debit-normal accounts (Assets, Expenses), a positive balance means
    /// debits exceed credits (normal state).
    ///
    /// For credit-normal accounts (Liabilities, Equity, Revenue), a positive
    /// balance means credits exceed debits (would show as negative in normal state).
    pub fn net_balance(&self) -> Amount {
        self.total_debits - self.total_credits
    }

    /// Returns the balance in the account's normal balance direction.
    ///
    /// # Arguments
    ///
    /// * `normal_balance` - The account's normal balance (from account type)
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // Asset account (debit normal)
    /// assert_eq!(balance.balance(NormalBalance::Debit), Amount::from_dollars(100));
    ///
    /// // Liability account (credit normal)
    /// assert_eq!(balance.balance(NormalBalance::Credit), Amount::from_dollars(50));
    /// ```
    pub fn balance(&self, normal_balance: NormalBalance) -> Amount {
        match normal_balance {
            NormalBalance::Debit => self.net_balance(),
            NormalBalance::Credit => -self.net_balance(),
        }
    }

    /// Applies a debit to this account.
    pub fn apply_debit(&mut self, amount: Amount) {
        self.total_debits = self.total_debits + amount;
        self.entry_count += 1;
    }

    /// Applies a credit to this account.
    pub fn apply_credit(&mut self, amount: Amount) {
        self.total_credits = self.total_credits + amount;
        self.entry_count += 1;
    }

    /// Applies a transaction to this account (debit or credit).
    pub fn apply(&mut self, side: DebitCredit, amount: Amount) {
        match side {
            DebitCredit::Debit => self.apply_debit(amount),
            DebitCredit::Credit => self.apply_credit(amount),
        }
    }
}

impl std::fmt::Display for AccountBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} as of {}: Dr {} Cr {} (Net: {})",
            self.account_number,
            self.as_of_date,
            self.total_debits,
            self.total_credits,
            self.net_balance()
        )
    }
}

// ─────────────────────────────────────────────────────────────
//  Balance Sheet
// ─────────────────────────────────────────────────────────────

/// Balance sheet - snapshot of assets, liabilities, and equity at a point in time.
///
/// The balance sheet equation:
/// ```text
/// Assets = Liabilities + Equity
/// ```
///
/// # GAAP Compliance
///
/// The balance sheet is generated from posted journal entries and must always
/// satisfy the accounting equation. Any imbalance indicates a data integrity issue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceSheet {
    /// Date this balance sheet is computed as of.
    as_of_date: NaiveDate,

    /// Asset account balances.
    assets: HashMap<AccountNumber, Amount>,

    /// Liability account balances.
    liabilities: HashMap<AccountNumber, Amount>,

    /// Equity account balances.
    equity: HashMap<AccountNumber, Amount>,

    /// Total assets.
    total_assets: Amount,

    /// Total liabilities.
    total_liabilities: Amount,

    /// Total equity.
    total_equity: Amount,
}

impl BalanceSheet {
    /// Creates a new balance sheet.
    pub fn new(as_of_date: NaiveDate) -> Self {
        Self {
            as_of_date,
            assets: HashMap::new(),
            liabilities: HashMap::new(),
            equity: HashMap::new(),
            total_assets: Amount::from_cents(0),
            total_liabilities: Amount::from_cents(0),
            total_equity: Amount::from_cents(0),
        }
    }

    /// Returns the as-of date.
    pub fn as_of_date(&self) -> NaiveDate {
        self.as_of_date
    }

    /// Returns asset balances.
    pub fn assets(&self) -> &HashMap<AccountNumber, Amount> {
        &self.assets
    }

    /// Returns liability balances.
    pub fn liabilities(&self) -> &HashMap<AccountNumber, Amount> {
        &self.liabilities
    }

    /// Returns equity balances.
    pub fn equity(&self) -> &HashMap<AccountNumber, Amount> {
        &self.equity
    }

    /// Returns total assets.
    pub fn total_assets(&self) -> Amount {
        self.total_assets
    }

    /// Returns total liabilities.
    pub fn total_liabilities(&self) -> Amount {
        self.total_liabilities
    }

    /// Returns total equity.
    pub fn total_equity(&self) -> Amount {
        self.total_equity
    }

    /// Adds an asset account balance.
    pub fn add_asset(&mut self, account: AccountNumber, balance: Amount) {
        self.total_assets = self.total_assets + balance;
        self.assets.insert(account, balance);
    }

    /// Adds a liability account balance.
    pub fn add_liability(&mut self, account: AccountNumber, balance: Amount) {
        self.total_liabilities = self.total_liabilities + balance;
        self.liabilities.insert(account, balance);
    }

    /// Adds an equity account balance.
    pub fn add_equity(&mut self, account: AccountNumber, balance: Amount) {
        self.total_equity = self.total_equity + balance;
        self.equity.insert(account, balance);
    }

    /// Verifies the accounting equation (Assets = Liabilities + Equity).
    pub fn verify_equation(&self) -> bool {
        self.total_assets == self.total_liabilities + self.total_equity
    }

    /// Returns the difference if equation doesn't balance.
    pub fn equation_difference(&self) -> Amount {
        self.total_assets - (self.total_liabilities + self.total_equity)
    }
}

impl std::fmt::Display for BalanceSheet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Balance Sheet as of {}", self.as_of_date)?;
        writeln!(f, "─────────────────────────────")?;
        writeln!(f, "Assets:       {}", self.total_assets)?;
        writeln!(f, "Liabilities:  {}", self.total_liabilities)?;
        writeln!(f, "Equity:       {}", self.total_equity)?;
        writeln!(f, "─────────────────────────────")?;

        if self.verify_equation() {
            writeln!(f, "✓ Equation verified: A = L + E")
        } else {
            writeln!(f, "✗ Equation imbalance: {}", self.equation_difference())
        }
    }
}
