//! Tests for financial statement generation.

use chrono::NaiveDate;

use elicit_server::ledger2::{
    Account, AccountClass, AccountNumber, Amount, AssetType, ComparativeIncomeStatement,
    CurrentAsset, EntityId, ExpenseType, FinancialRatios, IncomeStatement, JournalEntry, Ledger,
    RevenueType, StatementPeriod,
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
//  Statement Period Tests
// ─────────────────────────────────────────────────────────────

#[test]
fn test_monthly_period() {
    let period = StatementPeriod::monthly(2024, 1);
    assert_eq!(period.start_date(), date(2024, 1, 1));
    assert_eq!(period.end_date(), date(2024, 1, 31));
    assert_eq!(period.description(), "January 2024");
}

#[test]
fn test_quarterly_period_q1() {
    let period = StatementPeriod::quarterly(2024, 1);
    assert_eq!(period.start_date(), date(2024, 1, 1));
    assert_eq!(period.end_date(), date(2024, 3, 31));
    assert_eq!(period.description(), "Q1 2024");
}

#[test]
fn test_quarterly_period_q4() {
    let period = StatementPeriod::quarterly(2024, 4);
    assert_eq!(period.start_date(), date(2024, 10, 1));
    assert_eq!(period.end_date(), date(2024, 12, 31));
    assert_eq!(period.description(), "Q4 2024");
}

#[test]
fn test_annual_period() {
    let period = StatementPeriod::annual(2024);
    assert_eq!(period.start_date(), date(2024, 1, 1));
    assert_eq!(period.end_date(), date(2024, 12, 31));
    assert_eq!(period.description(), "FY 2024");
}

// ─────────────────────────────────────────────────────────────
//  Income Statement Tests
// ─────────────────────────────────────────────────────────────

#[test]
fn test_income_statement_empty() {
    let period = StatementPeriod::monthly(2024, 1);
    let statement = IncomeStatement::new(period);

    assert_eq!(statement.total_revenue(), Amount::from_cents(0));
    assert_eq!(statement.total_expenses(), Amount::from_cents(0));
    assert_eq!(statement.net_income(), Amount::from_cents(0));
}

#[test]
fn test_income_statement_add_revenue() {
    let period = StatementPeriod::monthly(2024, 1);
    let mut statement = IncomeStatement::new(period);

    statement.add_revenue(AccountNumber::new("4100"), Amount::from_dollars(100));

    assert_eq!(statement.total_revenue(), Amount::from_dollars(100));
    assert_eq!(statement.total_expenses(), Amount::from_cents(0));
    assert_eq!(statement.net_income(), Amount::from_dollars(100));
}

#[test]
fn test_income_statement_add_expense() {
    let period = StatementPeriod::monthly(2024, 1);
    let mut statement = IncomeStatement::new(period);

    statement.add_expense(AccountNumber::new("5100"), Amount::from_dollars(50));

    assert_eq!(statement.total_revenue(), Amount::from_cents(0));
    assert_eq!(statement.total_expenses(), Amount::from_dollars(50));
    assert_eq!(statement.net_income(), Amount::from_dollars(-50));
}

#[test]
fn test_income_statement_net_income() {
    let period = StatementPeriod::monthly(2024, 1);
    let mut statement = IncomeStatement::new(period);

    statement.add_revenue(AccountNumber::new("4100"), Amount::from_dollars(150));
    statement.add_expense(AccountNumber::new("5100"), Amount::from_dollars(75));

    assert_eq!(statement.total_revenue(), Amount::from_dollars(150));
    assert_eq!(statement.total_expenses(), Amount::from_dollars(75));
    assert_eq!(statement.net_income(), Amount::from_dollars(75));
}

#[test]
fn test_income_statement_profit_margin() {
    let period = StatementPeriod::monthly(2024, 1);
    let mut statement = IncomeStatement::new(period);

    statement.add_revenue(AccountNumber::new("4100"), Amount::from_dollars(100));
    statement.add_expense(AccountNumber::new("5100"), Amount::from_dollars(25));

    // Net income = $75, Revenue = $100, Margin = 75%
    let margin = statement.profit_margin().expect("Has margin");
    assert!((margin - 0.75).abs() < 0.01);
}

#[test]
fn test_income_statement_profit_margin_zero_revenue() {
    let period = StatementPeriod::monthly(2024, 1);
    let statement = IncomeStatement::new(period);

    assert!(statement.profit_margin().is_none());
}

// ─────────────────────────────────────────────────────────────
//  Ledger Integration Tests
// ─────────────────────────────────────────────────────────────

#[test]
fn test_ledger_generates_income_statement() {
    let entity_id = test_entity();
    let cash = cash_account(entity_id);
    let revenue = revenue_account(entity_id);
    let expense = expense_account(entity_id);

    let mut ledger = Ledger::new(entity_id);

    // Revenue entry: $150
    let entry1 = JournalEntry::builder(entity_id, date(2024, 1, 15))
        .description("Sale")
        .debit(cash.clone(), Amount::from_dollars(150), "Payment")
        .credit(revenue, Amount::from_dollars(150), "Sale")
        .build()
        .expect("Valid entry");

    // Expense entry: $75
    let entry2 = JournalEntry::builder(entity_id, date(2024, 1, 20))
        .description("COGS")
        .debit(expense, Amount::from_dollars(75), "Cost")
        .credit(cash, Amount::from_dollars(75), "Payment")
        .build()
        .expect("Valid entry");

    ledger.post(entry1).expect("Post 1 successful");
    ledger.post(entry2).expect("Post 2 successful");

    // Generate income statement
    let period = StatementPeriod::monthly(2024, 1);
    let statement = ledger.income_statement(&period);

    assert_eq!(statement.total_revenue(), Amount::from_dollars(150));
    assert_eq!(statement.total_expenses(), Amount::from_dollars(75));
    assert_eq!(statement.net_income(), Amount::from_dollars(75));
}

#[test]
fn test_income_statement_only_includes_period_entries() {
    let entity_id = test_entity();
    let cash = cash_account(entity_id);
    let revenue = revenue_account(entity_id);

    let mut ledger = Ledger::new(entity_id);

    // January entry
    let entry1 = JournalEntry::builder(entity_id, date(2024, 1, 15))
        .description("January sale")
        .debit(cash.clone(), Amount::from_dollars(100), "Payment")
        .credit(revenue.clone(), Amount::from_dollars(100), "Sale")
        .build()
        .expect("Valid entry");

    // February entry
    let entry2 = JournalEntry::builder(entity_id, date(2024, 2, 10))
        .description("February sale")
        .debit(cash, Amount::from_dollars(50), "Payment")
        .credit(revenue, Amount::from_dollars(50), "Sale")
        .build()
        .expect("Valid entry");

    ledger.post(entry1).expect("Post 1 successful");
    ledger.post(entry2).expect("Post 2 successful");

    // Generate statement for January only
    let jan_period = StatementPeriod::monthly(2024, 1);
    let jan_statement = ledger.income_statement(&jan_period);

    assert_eq!(jan_statement.total_revenue(), Amount::from_dollars(100));
    assert_eq!(jan_statement.net_income(), Amount::from_dollars(100));

    // Generate statement for February only
    let feb_period = StatementPeriod::monthly(2024, 2);
    let feb_statement = ledger.income_statement(&feb_period);

    assert_eq!(feb_statement.total_revenue(), Amount::from_dollars(50));
    assert_eq!(feb_statement.net_income(), Amount::from_dollars(50));
}

// ─────────────────────────────────────────────────────────────
//  Comparative Statement Tests
// ─────────────────────────────────────────────────────────────

#[test]
fn test_comparative_income_statement() {
    let mut current = IncomeStatement::new(StatementPeriod::monthly(2024, 2));
    current.add_revenue(AccountNumber::new("4100"), Amount::from_dollars(200));
    current.add_expense(AccountNumber::new("5100"), Amount::from_dollars(80));

    let mut prior = IncomeStatement::new(StatementPeriod::monthly(2024, 1));
    prior.add_revenue(AccountNumber::new("4100"), Amount::from_dollars(150));
    prior.add_expense(AccountNumber::new("5100"), Amount::from_dollars(75));

    let comparative = ComparativeIncomeStatement::new(current, prior);

    assert_eq!(comparative.revenue_variance(), Amount::from_dollars(50));
    assert_eq!(comparative.expense_variance(), Amount::from_dollars(5));
    assert_eq!(comparative.net_income_variance(), Amount::from_dollars(45));
}

#[test]
fn test_comparative_revenue_growth() {
    let mut current = IncomeStatement::new(StatementPeriod::monthly(2024, 2));
    current.add_revenue(AccountNumber::new("4100"), Amount::from_dollars(150));

    let mut prior = IncomeStatement::new(StatementPeriod::monthly(2024, 1));
    prior.add_revenue(AccountNumber::new("4100"), Amount::from_dollars(100));

    let comparative = ComparativeIncomeStatement::new(current, prior);

    // Growth = (150 - 100) / 100 = 50%
    let growth = comparative.revenue_growth().expect("Has growth");
    assert!((growth - 0.5).abs() < 0.01);
}

#[test]
fn test_comparative_revenue_growth_zero_prior() {
    let mut current = IncomeStatement::new(StatementPeriod::monthly(2024, 2));
    current.add_revenue(AccountNumber::new("4100"), Amount::from_dollars(150));

    let prior = IncomeStatement::new(StatementPeriod::monthly(2024, 1));

    let comparative = ComparativeIncomeStatement::new(current, prior);

    assert!(comparative.revenue_growth().is_none());
}

// ─────────────────────────────────────────────────────────────
//  Financial Ratios Tests
// ─────────────────────────────────────────────────────────────

#[test]
fn test_financial_ratios_profit_margin() {
    let ratios = FinancialRatios::new(
        Amount::from_dollars(75),  // net income
        Amount::from_dollars(100), // revenue
        Amount::from_dollars(200), // current assets
        Amount::from_dollars(100), // current liabilities
        Amount::from_dollars(150), // total liabilities
        Amount::from_dollars(250), // total equity
    );

    // Profit margin = 75 / 100 = 75%
    let margin = ratios.profit_margin().expect("Has margin");
    assert!((margin - 0.75).abs() < 0.01);
}

#[test]
fn test_financial_ratios_current_ratio() {
    let ratios = FinancialRatios::new(
        Amount::from_dollars(75),  // net income
        Amount::from_dollars(100), // revenue
        Amount::from_dollars(200), // current assets
        Amount::from_dollars(100), // current liabilities
        Amount::from_dollars(150), // total liabilities
        Amount::from_dollars(250), // total equity
    );

    // Current ratio = 200 / 100 = 2.0
    let ratio = ratios.current_ratio().expect("Has ratio");
    assert!((ratio - 2.0).abs() < 0.01);
}

#[test]
fn test_financial_ratios_debt_to_equity() {
    let ratios = FinancialRatios::new(
        Amount::from_dollars(75),  // net income
        Amount::from_dollars(100), // revenue
        Amount::from_dollars(200), // current assets
        Amount::from_dollars(100), // current liabilities
        Amount::from_dollars(150), // total liabilities
        Amount::from_dollars(250), // total equity
    );

    // Debt-to-equity = 150 / 250 = 0.6
    let ratio = ratios.debt_to_equity().expect("Has ratio");
    assert!((ratio - 0.6).abs() < 0.01);
}

#[test]
fn test_financial_ratios_zero_divisors() {
    let ratios = FinancialRatios::new(
        Amount::from_dollars(0),   // net income
        Amount::from_dollars(0),   // revenue (zero)
        Amount::from_dollars(100), // current assets
        Amount::from_dollars(0),   // current liabilities (zero)
        Amount::from_dollars(100), // total liabilities
        Amount::from_dollars(0),   // total equity (zero)
    );

    assert!(ratios.profit_margin().is_none());
    assert!(ratios.current_ratio().is_none());
    assert!(ratios.debt_to_equity().is_none());
}
