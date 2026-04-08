//! Tests for ledger operations, balance tracking, and queries.

use chrono::NaiveDate;

use elicit_server::ledger2::{
    Account, AccountClass, Amount, AssetType, CurrentAsset, EntityId, ExpenseType, JournalEntry,
    Ledger, NormalBalance, RevenueType,
};

// ─────────────────────────────────────────────────────────────
//  Test Helpers
// ─────────────────────────────────────────────────────────────

fn test_entity() -> EntityId {
    EntityId::new_v4()
}

fn date(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).expect("Valid date")
}

fn cash_account(entity_id: EntityId) -> Account {
    Account::builder()
        .number("1110")
        .name("Cash")
        .class(AccountClass::Asset(AssetType::CurrentAsset(
            CurrentAsset::Cash,
        )))
        .entity_id(entity_id)
        .build()
        .expect("Valid account")
}

fn revenue_account(entity_id: EntityId) -> Account {
    Account::builder()
        .number("4100")
        .name("Sales Revenue")
        .class(AccountClass::Revenue(RevenueType::Sales))
        .entity_id(entity_id)
        .build()
        .expect("Valid account")
}

fn expense_account(entity_id: EntityId) -> Account {
    Account::builder()
        .number("5100")
        .name("Cost of Goods Sold")
        .class(AccountClass::Expense(ExpenseType::CostOfGoodsSold))
        .entity_id(entity_id)
        .build()
        .expect("Valid account")
}

// ─────────────────────────────────────────────────────────────
//  Account Balance Tests
// ─────────────────────────────────────────────────────────────

#[test]
fn test_account_balance_initial_zero() {
    let entity_id = test_entity();
    let cash = cash_account(entity_id);

    let ledger = Ledger::new(entity_id);
    let balance = ledger.account_balance(cash.number(), date(2024, 1, 31));

    assert_eq!(balance.total_debits(), Amount::from_cents(0));
    assert_eq!(balance.total_credits(), Amount::from_cents(0));
    assert_eq!(balance.net_balance(), Amount::from_cents(0));
}

#[test]
fn test_account_balance_after_debit() {
    let entity_id = test_entity();
    let cash = cash_account(entity_id);
    let revenue = revenue_account(entity_id);

    let mut ledger = Ledger::new(entity_id);

    // Post entry: Dr. Cash $100, Cr. Revenue $100
    let entry = JournalEntry::builder(entity_id, date(2024, 1, 15))
        .description("Cash sale")
        .debit(cash.clone(), Amount::from_dollars(100), "Payment received")
        .credit(revenue, Amount::from_dollars(100), "Sale of goods")
        .build()
        .expect("Valid entry");

    ledger.post(entry).expect("Post successful");

    // Check cash balance
    let balance = ledger.account_balance(cash.number(), date(2024, 1, 31));
    assert_eq!(balance.total_debits(), Amount::from_dollars(100));
    assert_eq!(balance.total_credits(), Amount::from_cents(0));
    assert_eq!(
        balance.balance(NormalBalance::Debit),
        Amount::from_dollars(100)
    );
}

#[test]
fn test_account_balance_multiple_entries() {
    let entity_id = test_entity();
    let cash = cash_account(entity_id);
    let revenue = revenue_account(entity_id);

    let mut ledger = Ledger::new(entity_id);

    // Entry 1: Dr. Cash $100, Cr. Revenue $100
    let entry1 = JournalEntry::builder(entity_id, date(2024, 1, 15))
        .description("Sale 1")
        .debit(cash.clone(), Amount::from_dollars(100), "Payment")
        .credit(revenue.clone(), Amount::from_dollars(100), "Sale")
        .build()
        .expect("Valid entry");

    // Entry 2: Dr. Cash $50, Cr. Revenue $50
    let entry2 = JournalEntry::builder(entity_id, date(2024, 1, 20))
        .description("Sale 2")
        .debit(cash.clone(), Amount::from_dollars(50), "Payment")
        .credit(revenue, Amount::from_dollars(50), "Sale")
        .build()
        .expect("Valid entry");

    ledger.post(entry1).expect("Post 1 successful");
    ledger.post(entry2).expect("Post 2 successful");

    // Check cash balance
    let balance = ledger.account_balance(cash.number(), date(2024, 1, 31));
    assert_eq!(balance.total_debits(), Amount::from_dollars(150));
    assert_eq!(
        balance.balance(NormalBalance::Debit),
        Amount::from_dollars(150)
    );
}

#[test]
fn test_account_balance_as_of_date() {
    let entity_id = test_entity();
    let cash = cash_account(entity_id);
    let revenue = revenue_account(entity_id);

    let mut ledger = Ledger::new(entity_id);

    // Entry 1: Jan 15
    let entry1 = JournalEntry::builder(entity_id, date(2024, 1, 15))
        .description("Sale 1")
        .debit(cash.clone(), Amount::from_dollars(100), "Payment")
        .credit(revenue.clone(), Amount::from_dollars(100), "Sale")
        .build()
        .expect("Valid entry");

    // Entry 2: Jan 25
    let entry2 = JournalEntry::builder(entity_id, date(2024, 1, 25))
        .description("Sale 2")
        .debit(cash.clone(), Amount::from_dollars(50), "Payment")
        .credit(revenue, Amount::from_dollars(50), "Sale")
        .build()
        .expect("Valid entry");

    ledger.post(entry1).expect("Post 1 successful");
    ledger.post(entry2).expect("Post 2 successful");

    // Balance as of Jan 20 (only first entry)
    let balance_jan_20 = ledger.account_balance(cash.number(), date(2024, 1, 20));
    assert_eq!(
        balance_jan_20.balance(NormalBalance::Debit),
        Amount::from_dollars(100)
    );

    // Balance as of Jan 31 (both entries)
    let balance_jan_31 = ledger.account_balance(cash.number(), date(2024, 1, 31));
    assert_eq!(
        balance_jan_31.balance(NormalBalance::Debit),
        Amount::from_dollars(150)
    );
}

// ─────────────────────────────────────────────────────────────
//  Ledger Posting Tests
// ─────────────────────────────────────────────────────────────

#[test]
fn test_post_entry() {
    let entity_id = test_entity();
    let cash = cash_account(entity_id);
    let revenue = revenue_account(entity_id);

    let mut ledger = Ledger::new(entity_id);

    let entry = JournalEntry::builder(entity_id, date(2024, 1, 15))
        .description("Cash sale")
        .debit(cash, Amount::from_dollars(100), "Payment received")
        .credit(revenue, Amount::from_dollars(100), "Sale of goods")
        .build()
        .expect("Valid entry");

    let posted = ledger.post(entry).expect("Post successful");

    assert_eq!(ledger.entry_count(), 1);
    assert!(posted.posted_at() > posted.created_at());
}

#[test]
fn test_post_entry_wrong_entity_fails() {
    let entity1 = test_entity();
    let entity2 = test_entity();
    let cash = cash_account(entity2);
    let revenue = revenue_account(entity2);

    let mut ledger = Ledger::new(entity1);

    let entry = JournalEntry::builder(entity2, date(2024, 1, 15))
        .description("Cash sale")
        .debit(cash, Amount::from_dollars(100), "Payment received")
        .credit(revenue, Amount::from_dollars(100), "Sale of goods")
        .build()
        .expect("Valid entry");

    let result = ledger.post(entry);
    assert!(result.is_err());
}

#[test]
fn test_get_entry() {
    let entity_id = test_entity();
    let cash = cash_account(entity_id);
    let revenue = revenue_account(entity_id);

    let mut ledger = Ledger::new(entity_id);

    let entry = JournalEntry::builder(entity_id, date(2024, 1, 15))
        .description("Cash sale")
        .debit(cash, Amount::from_dollars(100), "Payment received")
        .credit(revenue, Amount::from_dollars(100), "Sale of goods")
        .build()
        .expect("Valid entry");

    let entry_id = entry.entry_id();
    ledger.post(entry).expect("Post successful");

    let retrieved = ledger.get_entry(&entry_id);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().entry_id(), entry_id);
}

#[test]
fn test_entries_chronological_order() {
    let entity_id = test_entity();
    let cash = cash_account(entity_id);
    let revenue = revenue_account(entity_id);

    let mut ledger = Ledger::new(entity_id);

    // Post entries in reverse chronological order
    let entry2 = JournalEntry::builder(entity_id, date(2024, 1, 25))
        .description("Sale 2")
        .debit(cash.clone(), Amount::from_dollars(50), "Payment")
        .credit(revenue.clone(), Amount::from_dollars(50), "Sale")
        .build()
        .expect("Valid entry");

    let entry1 = JournalEntry::builder(entity_id, date(2024, 1, 15))
        .description("Sale 1")
        .debit(cash, Amount::from_dollars(100), "Payment")
        .credit(revenue, Amount::from_dollars(100), "Sale")
        .build()
        .expect("Valid entry");

    ledger.post(entry2).expect("Post 2 successful");
    ledger.post(entry1).expect("Post 1 successful");

    // Entries should be in posting order (not date order)
    let entries = ledger.entries();
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].date(), date(2024, 1, 25)); // Posted first
    assert_eq!(entries[1].date(), date(2024, 1, 15)); // Posted second
}

// ─────────────────────────────────────────────────────────────
//  Query Tests
// ─────────────────────────────────────────────────────────────

#[test]
fn test_entries_in_range() {
    let entity_id = test_entity();
    let cash = cash_account(entity_id);
    let revenue = revenue_account(entity_id);

    let mut ledger = Ledger::new(entity_id);

    // Entry 1: Jan 15
    let entry1 = JournalEntry::builder(entity_id, date(2024, 1, 15))
        .description("Sale 1")
        .debit(cash.clone(), Amount::from_dollars(100), "Payment")
        .credit(revenue.clone(), Amount::from_dollars(100), "Sale")
        .build()
        .expect("Valid entry");

    // Entry 2: Feb 10
    let entry2 = JournalEntry::builder(entity_id, date(2024, 2, 10))
        .description("Sale 2")
        .debit(cash.clone(), Amount::from_dollars(50), "Payment")
        .credit(revenue.clone(), Amount::from_dollars(50), "Sale")
        .build()
        .expect("Valid entry");

    // Entry 3: Mar 5
    let entry3 = JournalEntry::builder(entity_id, date(2024, 3, 5))
        .description("Sale 3")
        .debit(cash, Amount::from_dollars(75), "Payment")
        .credit(revenue, Amount::from_dollars(75), "Sale")
        .build()
        .expect("Valid entry");

    ledger.post(entry1).expect("Post 1 successful");
    ledger.post(entry2).expect("Post 2 successful");
    ledger.post(entry3).expect("Post 3 successful");

    // Query Jan 1 - Feb 28 (should get first two entries)
    let entries = ledger.entries_in_range(date(2024, 1, 1), date(2024, 2, 28));
    assert_eq!(entries.len(), 2);
}

#[test]
fn test_total_revenue() {
    let entity_id = test_entity();
    let cash = cash_account(entity_id);
    let revenue = revenue_account(entity_id);

    let mut ledger = Ledger::new(entity_id);

    // Entry 1: $100 revenue
    let entry1 = JournalEntry::builder(entity_id, date(2024, 1, 15))
        .description("Sale 1")
        .debit(cash.clone(), Amount::from_dollars(100), "Payment")
        .credit(revenue.clone(), Amount::from_dollars(100), "Sale")
        .build()
        .expect("Valid entry");

    // Entry 2: $50 revenue
    let entry2 = JournalEntry::builder(entity_id, date(2024, 1, 25))
        .description("Sale 2")
        .debit(cash, Amount::from_dollars(50), "Payment")
        .credit(revenue, Amount::from_dollars(50), "Sale")
        .build()
        .expect("Valid entry");

    ledger.post(entry1).expect("Post 1 successful");
    ledger.post(entry2).expect("Post 2 successful");

    let total = ledger.total_revenue(date(2024, 1, 1), date(2024, 1, 31));
    assert_eq!(total, Amount::from_dollars(150));
}

#[test]
fn test_total_expenses() {
    let entity_id = test_entity();
    let cash = cash_account(entity_id);
    let expense = expense_account(entity_id);

    let mut ledger = Ledger::new(entity_id);

    // Entry: $75 expense
    let entry = JournalEntry::builder(entity_id, date(2024, 1, 15))
        .description("Cost of goods sold")
        .debit(expense, Amount::from_dollars(75), "COGS")
        .credit(cash, Amount::from_dollars(75), "Payment")
        .build()
        .expect("Valid entry");

    ledger.post(entry).expect("Post successful");

    let total = ledger.total_expenses(date(2024, 1, 1), date(2024, 1, 31));
    assert_eq!(total, Amount::from_dollars(75));
}

#[test]
fn test_net_income() {
    let entity_id = test_entity();
    let cash = cash_account(entity_id);
    let revenue = revenue_account(entity_id);
    let expense = expense_account(entity_id);

    let mut ledger = Ledger::new(entity_id);

    // Revenue: $150
    let entry1 = JournalEntry::builder(entity_id, date(2024, 1, 15))
        .description("Sale")
        .debit(cash.clone(), Amount::from_dollars(150), "Payment")
        .credit(revenue, Amount::from_dollars(150), "Sale")
        .build()
        .expect("Valid entry");

    // Expense: $75
    let entry2 = JournalEntry::builder(entity_id, date(2024, 1, 20))
        .description("COGS")
        .debit(expense, Amount::from_dollars(75), "Cost")
        .credit(cash, Amount::from_dollars(75), "Payment")
        .build()
        .expect("Valid entry");

    ledger.post(entry1).expect("Post 1 successful");
    ledger.post(entry2).expect("Post 2 successful");

    // Net income = $150 - $75 = $75
    let net_income = ledger.net_income(date(2024, 1, 1), date(2024, 1, 31));
    assert_eq!(net_income, Amount::from_dollars(75));
}

// ─────────────────────────────────────────────────────────────
//  Balance Sheet Tests
// ─────────────────────────────────────────────────────────────

#[test]
fn test_balance_sheet_generation() {
    let entity_id = test_entity();
    let cash = cash_account(entity_id);
    let revenue = revenue_account(entity_id);

    let mut ledger = Ledger::new(entity_id);

    // Entry: Dr. Cash $100, Cr. Revenue $100
    let entry = JournalEntry::builder(entity_id, date(2024, 1, 15))
        .description("Cash sale")
        .debit(cash, Amount::from_dollars(100), "Payment received")
        .credit(revenue, Amount::from_dollars(100), "Sale of goods")
        .build()
        .expect("Valid entry");

    ledger.post(entry).expect("Post successful");

    let balance_sheet = ledger.balance_sheet(date(2024, 1, 31));

    // Cash (asset) should be $100
    assert_eq!(balance_sheet.total_assets(), Amount::from_dollars(100));

    // Note: Revenue is a temporary account and doesn't appear on balance sheet
    // In a real system, revenue would be closed to retained earnings
}
