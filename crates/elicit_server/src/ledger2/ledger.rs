//! Ledger - manages journal entries and account balances.

use std::collections::HashMap;

use chrono::NaiveDate;

use crate::ledger2::{
    AccountBalance, AccountClass, AccountNumber, Amount, BalanceSheet, Balanced, EntityId, EntryId,
    IncomeStatement, JournalEntry, JournalEntryError, JournalEntryResult, Posted, StatementPeriod,
};

// ─────────────────────────────────────────────────────────────
//  Ledger
// ─────────────────────────────────────────────────────────────

/// The general ledger - central repository for all journal entries.
///
/// The ledger maintains:
/// - All posted journal entries
/// - Running account balances
/// - Chronological entry history
///
/// # GAAP Requirements
///
/// - **Completeness**: All transactions recorded
/// - **Accuracy**: Balances computed correctly from entries
/// - **Double-entry**: Every entry maintains A = L + E equation
/// - **Chronological**: Entries ordered by date
/// - **Immutability**: Posted entries cannot be modified (only reversed)
///
/// # Example
///
/// ```rust,ignore
/// use elicit_server::ledger2::{Ledger, JournalEntry, Amount};
/// use chrono::NaiveDate;
///
/// let mut ledger = Ledger::new(entity_id);
///
/// // Create and post an entry
/// let entry = JournalEntry::builder(entity_id, NaiveDate::from_ymd(2024, 1, 15))
///     .description("Cash sale")
///     .debit(cash, Amount::from_dollars(100), "Payment received")
///     .credit(revenue, Amount::from_dollars(100), "Sale of goods")
///     .build()?;
///
/// ledger.post(entry)?;
///
/// // Query balance
/// let balance = ledger.account_balance(cash.number(), NaiveDate::from_ymd(2024, 1, 31))?;
/// assert_eq!(balance.balance(cash.normal_balance()), Amount::from_dollars(100));
/// ```
#[derive(Debug, Clone)]
pub struct Ledger {
    /// Entity this ledger belongs to.
    entity_id: EntityId,

    /// All posted journal entries, indexed by entry ID.
    entries: HashMap<EntryId, JournalEntry<Posted>>,

    /// Journal entries in chronological order.
    chronological: Vec<EntryId>,

    /// Account balances, indexed by account number.
    balances: HashMap<AccountNumber, AccountBalance>,
}

impl Ledger {
    /// Creates a new empty ledger for the given entity.
    pub fn new(entity_id: EntityId) -> Self {
        Self {
            entity_id,
            entries: HashMap::new(),
            chronological: Vec::new(),
            balances: HashMap::new(),
        }
    }

    /// Returns the entity ID.
    pub fn entity_id(&self) -> EntityId {
        self.entity_id
    }

    /// Returns the number of posted entries in this ledger.
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    /// Posts a journal entry to the ledger.
    ///
    /// This:
    /// 1. Transitions the entry from Balanced → Posted
    /// 2. Stores the entry in the ledger
    /// 3. Updates account balances
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Entry belongs to different entity
    /// - Entry ID already exists (duplicate)
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let posted = ledger.post(balanced_entry)?;
    /// ```
    pub fn post(
        &mut self,
        entry: JournalEntry<Balanced>,
    ) -> JournalEntryResult<JournalEntry<Posted>> {
        // Validate entity matches
        if entry.entity_id() != self.entity_id {
            return Err(JournalEntryError::entity_mismatch());
        }

        // Validate no duplicate entry ID
        if self.entries.contains_key(&entry.entry_id()) {
            return Err(JournalEntryError::already_posted(
                entry.entry_id().to_string(),
            ));
        }

        // Transition to Posted state
        let posted = entry.post();

        // Update account balances
        for line in posted.lines() {
            let account_number = line.account().number().clone();
            let balance = self
                .balances
                .entry(account_number.clone())
                .or_insert_with(|| AccountBalance::new(account_number, posted.date()));

            balance.apply(line.side(), line.amount());
        }

        // Store entry
        let entry_id = posted.entry_id();
        self.entries.insert(entry_id, posted.clone());
        self.chronological.push(entry_id);

        Ok(posted)
    }

    /// Returns a posted entry by ID.
    pub fn get_entry(&self, entry_id: &EntryId) -> Option<&JournalEntry<Posted>> {
        self.entries.get(entry_id)
    }

    /// Returns all posted entries in chronological order.
    pub fn entries(&self) -> Vec<&JournalEntry<Posted>> {
        self.chronological
            .iter()
            .filter_map(|id| self.entries.get(id))
            .collect()
    }

    /// Returns entries posted on or before a given date.
    pub fn entries_as_of(&self, as_of_date: NaiveDate) -> Vec<&JournalEntry<Posted>> {
        self.chronological
            .iter()
            .filter_map(|id| self.entries.get(id))
            .filter(|entry| entry.date() <= as_of_date)
            .collect()
    }

    /// Returns entries posted within a date range (inclusive).
    pub fn entries_in_range(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Vec<&JournalEntry<Posted>> {
        self.chronological
            .iter()
            .filter_map(|id| self.entries.get(id))
            .filter(|entry| entry.date() >= start_date && entry.date() <= end_date)
            .collect()
    }

    /// Returns the balance for an account as of a given date.
    ///
    /// Computes the balance by summing all journal lines affecting the account
    /// up to and including the specified date.
    pub fn account_balance(
        &self,
        account_number: &AccountNumber,
        as_of_date: NaiveDate,
    ) -> AccountBalance {
        let mut balance = AccountBalance::new(account_number.clone(), as_of_date);

        // Sum all journal lines for this account up to as_of_date
        for entry in self.entries_as_of(as_of_date) {
            for line in entry.lines() {
                if line.account().number() == account_number {
                    balance.apply(line.side(), line.amount());
                }
            }
        }

        balance
    }

    /// Returns balances for all accounts as of a given date.
    pub fn all_balances(&self, as_of_date: NaiveDate) -> HashMap<AccountNumber, AccountBalance> {
        let mut balances: HashMap<AccountNumber, AccountBalance> = HashMap::new();

        // Iterate through all entries up to as_of_date
        for entry in self.entries_as_of(as_of_date) {
            for line in entry.lines() {
                let account_number = line.account().number().clone();
                let balance = balances
                    .entry(account_number.clone())
                    .or_insert_with(|| AccountBalance::new(account_number, as_of_date));

                balance.apply(line.side(), line.amount());
            }
        }

        balances
    }

    /// Generates a balance sheet as of a given date.
    ///
    /// The balance sheet includes:
    /// - Asset accounts (debit balances)
    /// - Liability accounts (credit balances)
    /// - Equity accounts (credit balances)
    ///
    /// The balance sheet equation (Assets = Liabilities + Equity) is verified.
    pub fn balance_sheet(&self, as_of_date: NaiveDate) -> BalanceSheet {
        let mut balance_sheet = BalanceSheet::new(as_of_date);

        // Get all account balances
        let balances = self.all_balances(as_of_date);

        // Categorize by account class
        for (account_number, balance) in balances {
            // We need to look up the account to get its class
            // For now, we'll compute the balance from the entries
            for entry in self.entries_as_of(as_of_date) {
                for line in entry.lines() {
                    if line.account().number() == &account_number {
                        let account = line.account();
                        let amount = balance.balance(account.normal_balance());

                        match account.class() {
                            AccountClass::Asset(_) => {
                                balance_sheet.add_asset(account_number.clone(), amount);
                                break;
                            }
                            AccountClass::Liability(_) => {
                                balance_sheet.add_liability(account_number.clone(), amount);
                                break;
                            }
                            AccountClass::Equity(_) => {
                                balance_sheet.add_equity(account_number.clone(), amount);
                                break;
                            }
                            // Revenue and Expense accounts are temporary and not on balance sheet
                            _ => break,
                        }
                    }
                }
            }
        }

        balance_sheet
    }

    /// Returns total revenue for a date range.
    pub fn total_revenue(&self, start_date: NaiveDate, end_date: NaiveDate) -> Amount {
        let mut total = Amount::from_cents(0);

        for entry in self.entries_in_range(start_date, end_date) {
            for line in entry.lines() {
                if let AccountClass::Revenue(_) = line.account().class() {
                    // Revenue has credit normal balance, so credits increase revenue
                    if let Some(credit) = line.credit_amount() {
                        total = total + credit;
                    }
                    if let Some(debit) = line.debit_amount() {
                        total = total - debit;
                    }
                }
            }
        }

        total
    }

    /// Returns total expenses for a date range.
    pub fn total_expenses(&self, start_date: NaiveDate, end_date: NaiveDate) -> Amount {
        let mut total = Amount::from_cents(0);

        for entry in self.entries_in_range(start_date, end_date) {
            for line in entry.lines() {
                if let AccountClass::Expense(_) = line.account().class() {
                    // Expenses have debit normal balance, so debits increase expenses
                    if let Some(debit) = line.debit_amount() {
                        total = total + debit;
                    }
                    if let Some(credit) = line.credit_amount() {
                        total = total - credit;
                    }
                }
            }
        }

        total
    }

    /// Returns net income for a date range (revenue - expenses).
    pub fn net_income(&self, start_date: NaiveDate, end_date: NaiveDate) -> Amount {
        self.total_revenue(start_date, end_date) - self.total_expenses(start_date, end_date)
    }

    /// Generates an income statement for a period.
    ///
    /// The income statement summarizes revenues and expenses for the period,
    /// showing net income (revenue - expenses).
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let period = StatementPeriod::monthly(2024, 1);
    /// let statement = ledger.income_statement(&period);
    ///
    /// println!("Revenue: {}", statement.total_revenue());
    /// println!("Net Income: {}", statement.net_income());
    /// ```
    pub fn income_statement(&self, period: &StatementPeriod) -> IncomeStatement {
        let mut statement = IncomeStatement::new(period.clone());

        // Collect revenue and expense amounts by account
        for entry in self.entries_in_range(period.start_date(), period.end_date()) {
            for line in entry.lines() {
                let account = line.account();
                let account_number = account.number().clone();

                match account.class() {
                    AccountClass::Revenue(_) => {
                        // Revenue has credit normal balance
                        let amount = if let Some(credit) = line.credit_amount() {
                            statement
                                .revenue()
                                .get(&account_number)
                                .cloned()
                                .unwrap_or(Amount::from_cents(0))
                                + credit
                        } else if let Some(debit) = line.debit_amount() {
                            statement
                                .revenue()
                                .get(&account_number)
                                .cloned()
                                .unwrap_or(Amount::from_cents(0))
                                - debit
                        } else {
                            statement
                                .revenue()
                                .get(&account_number)
                                .cloned()
                                .unwrap_or(Amount::from_cents(0))
                        };
                        statement.add_revenue(account_number, amount);
                    }
                    AccountClass::Expense(_) => {
                        // Expenses have debit normal balance
                        let amount = if let Some(debit) = line.debit_amount() {
                            statement
                                .expenses()
                                .get(&account_number)
                                .cloned()
                                .unwrap_or(Amount::from_cents(0))
                                + debit
                        } else if let Some(credit) = line.credit_amount() {
                            statement
                                .expenses()
                                .get(&account_number)
                                .cloned()
                                .unwrap_or(Amount::from_cents(0))
                                - credit
                        } else {
                            statement
                                .expenses()
                                .get(&account_number)
                                .cloned()
                                .unwrap_or(Amount::from_cents(0))
                        };
                        statement.add_expense(account_number, amount);
                    }
                    _ => {
                        // Ignore balance sheet accounts (Asset, Liability, Equity)
                    }
                }
            }
        }

        statement
    }
}
