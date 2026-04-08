//! Tests for GAAP account types and chart of accounts.

use elicit_server::ledger2::*;

// ─────────────────────────────────────────────────────────────
//  Normal Balance Tests
// ─────────────────────────────────────────────────────────────

#[test]
fn test_asset_normal_balance_is_debit() {
    let account_class = AccountClass::Asset(AssetType::CurrentAsset(CurrentAsset::Cash));
    assert_eq!(
        account_class.normal_balance(),
        NormalBalance::Debit,
        "Assets should have debit normal balance"
    );
}

#[test]
fn test_liability_normal_balance_is_credit() {
    let account_class = AccountClass::Liability(LiabilityType::CurrentLiability(
        CurrentLiability::AccountsPayable,
    ));
    assert_eq!(
        account_class.normal_balance(),
        NormalBalance::Credit,
        "Liabilities should have credit normal balance"
    );
}

#[test]
fn test_equity_normal_balance_is_credit() {
    let account_class = AccountClass::Equity(EquityType::CommonStock);
    assert_eq!(
        account_class.normal_balance(),
        NormalBalance::Credit,
        "Equity should have credit normal balance"
    );
}

#[test]
fn test_revenue_normal_balance_is_credit() {
    let account_class = AccountClass::Revenue(RevenueType::Sales);
    assert_eq!(
        account_class.normal_balance(),
        NormalBalance::Credit,
        "Revenue should have credit normal balance"
    );
}

#[test]
fn test_expense_normal_balance_is_debit() {
    let account_class = AccountClass::Expense(ExpenseType::Salaries);
    assert_eq!(
        account_class.normal_balance(),
        NormalBalance::Debit,
        "Expenses should have debit normal balance"
    );
}

// ─────────────────────────────────────────────────────────────
//  Temporary vs Permanent Account Tests
// ─────────────────────────────────────────────────────────────

#[test]
fn test_revenue_is_temporary_account() {
    let account_class = AccountClass::Revenue(RevenueType::Sales);
    assert!(account_class.is_temporary(), "Revenue should be temporary");
    assert!(
        !account_class.is_permanent(),
        "Revenue should not be permanent"
    );
}

#[test]
fn test_expense_is_temporary_account() {
    let account_class = AccountClass::Expense(ExpenseType::Salaries);
    assert!(account_class.is_temporary(), "Expense should be temporary");
    assert!(
        !account_class.is_permanent(),
        "Expense should not be permanent"
    );
}

#[test]
fn test_asset_is_permanent_account() {
    let account_class = AccountClass::Asset(AssetType::CurrentAsset(CurrentAsset::Cash));
    assert!(account_class.is_permanent(), "Asset should be permanent");
    assert!(
        !account_class.is_temporary(),
        "Asset should not be temporary"
    );
}

#[test]
fn test_liability_is_permanent_account() {
    let account_class = AccountClass::Liability(LiabilityType::CurrentLiability(
        CurrentLiability::AccountsPayable,
    ));
    assert!(
        account_class.is_permanent(),
        "Liability should be permanent"
    );
    assert!(
        !account_class.is_temporary(),
        "Liability should not be temporary"
    );
}

#[test]
fn test_equity_is_permanent_account() {
    let account_class = AccountClass::Equity(EquityType::RetainedEarnings);
    assert!(account_class.is_permanent(), "Equity should be permanent");
    assert!(
        !account_class.is_temporary(),
        "Equity should not be temporary"
    );
}

// ─────────────────────────────────────────────────────────────
//  Account Class Predicates
// ─────────────────────────────────────────────────────────────

#[test]
fn test_account_class_predicates() {
    let asset = AccountClass::Asset(AssetType::CurrentAsset(CurrentAsset::Cash));
    assert!(asset.is_asset());
    assert!(!asset.is_liability());
    assert!(!asset.is_equity());
    assert!(!asset.is_revenue());
    assert!(!asset.is_expense());

    let liability = AccountClass::Liability(LiabilityType::CurrentLiability(
        CurrentLiability::AccountsPayable,
    ));
    assert!(!liability.is_asset());
    assert!(liability.is_liability());
    assert!(!liability.is_equity());
    assert!(!liability.is_revenue());
    assert!(!liability.is_expense());

    let equity = AccountClass::Equity(EquityType::CommonStock);
    assert!(!equity.is_asset());
    assert!(!equity.is_liability());
    assert!(equity.is_equity());
    assert!(!equity.is_revenue());
    assert!(!equity.is_expense());

    let revenue = AccountClass::Revenue(RevenueType::Sales);
    assert!(!revenue.is_asset());
    assert!(!revenue.is_liability());
    assert!(!revenue.is_equity());
    assert!(revenue.is_revenue());
    assert!(!revenue.is_expense());

    let expense = AccountClass::Expense(ExpenseType::Salaries);
    assert!(!expense.is_asset());
    assert!(!expense.is_liability());
    assert!(!expense.is_equity());
    assert!(!expense.is_revenue());
    assert!(expense.is_expense());
}

// ─────────────────────────────────────────────────────────────
//  Account Builder Tests
// ─────────────────────────────────────────────────────────────

#[test]
fn test_account_builder_creates_valid_account() {
    let entity_id = EntityId::new_v4();

    let account = Account::builder()
        .number("1100")
        .name("Cash")
        .class(AccountClass::Asset(AssetType::CurrentAsset(
            CurrentAsset::Cash,
        )))
        .entity_id(entity_id)
        .build()
        .expect("Valid account");

    assert_eq!(account.number().0, "1100");
    assert_eq!(account.name(), "Cash");
    assert!(account.class().is_asset());
    assert_eq!(account.entity_id(), &entity_id);
    assert!(account.active()); // Default is true
    assert_eq!(account.normal_balance(), NormalBalance::Debit);
}

#[test]
fn test_account_builder_with_parent() {
    let entity_id = EntityId::new_v4();

    let account = Account::builder()
        .number("1110")
        .name("Petty Cash")
        .class(AccountClass::Asset(AssetType::CurrentAsset(
            CurrentAsset::Cash,
        )))
        .parent("1100")
        .entity_id(entity_id)
        .build()
        .expect("Valid account");

    assert_eq!(account.parent().map(|p| p.0.as_str()), Some("1100"));
}

#[test]
fn test_account_builder_inactive_account() {
    let entity_id = EntityId::new_v4();

    let account = Account::builder()
        .number("1000")
        .name("Assets Header")
        .class(AccountClass::Asset(AssetType::CurrentAsset(
            CurrentAsset::Cash,
        )))
        .entity_id(entity_id)
        .active(false)
        .build()
        .expect("Valid account");

    assert!(!account.active());
}

#[test]
fn test_account_builder_missing_number() {
    let entity_id = EntityId::new_v4();

    let result = Account::builder()
        .name("Cash")
        .class(AccountClass::Asset(AssetType::CurrentAsset(
            CurrentAsset::Cash,
        )))
        .entity_id(entity_id)
        .build();

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("number"));
}

#[test]
fn test_account_builder_missing_name() {
    let entity_id = EntityId::new_v4();

    let result = Account::builder()
        .number("1100")
        .class(AccountClass::Asset(AssetType::CurrentAsset(
            CurrentAsset::Cash,
        )))
        .entity_id(entity_id)
        .build();

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("name"));
}

#[test]
fn test_account_builder_missing_class() {
    let entity_id = EntityId::new_v4();

    let result = Account::builder()
        .number("1100")
        .name("Cash")
        .entity_id(entity_id)
        .build();

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("class"));
}

#[test]
fn test_account_builder_missing_entity_id() {
    let result = Account::builder()
        .number("1100")
        .name("Cash")
        .class(AccountClass::Asset(AssetType::CurrentAsset(
            CurrentAsset::Cash,
        )))
        .build();

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("entity_id"));
}

// ─────────────────────────────────────────────────────────────
//  Chart of Accounts Tests
// ─────────────────────────────────────────────────────────────

#[test]
fn test_empty_chart_of_accounts() {
    let entity_id = EntityId::new_v4();
    let chart = ChartOfAccounts::new(entity_id);

    assert_eq!(chart.entity_id(), entity_id);
    assert_eq!(chart.len(), 0);
    assert!(chart.is_empty());
}

#[test]
fn test_add_account_to_chart() {
    let entity_id = EntityId::new_v4();
    let mut chart = ChartOfAccounts::new(entity_id);

    let account = Account::builder()
        .number("1100")
        .name("Cash")
        .class(AccountClass::Asset(AssetType::CurrentAsset(
            CurrentAsset::Cash,
        )))
        .entity_id(entity_id)
        .build()
        .expect("Valid account");

    chart.add_account(account).expect("Add account");

    assert_eq!(chart.len(), 1);
    assert!(!chart.is_empty());

    let retrieved = chart.get_account("1100").expect("Get account");
    assert_eq!(retrieved.name(), "Cash");
}

#[test]
fn test_duplicate_account_number_error() {
    let entity_id = EntityId::new_v4();
    let mut chart = ChartOfAccounts::new(entity_id);

    let account1 = Account::builder()
        .number("1100")
        .name("Cash")
        .class(AccountClass::Asset(AssetType::CurrentAsset(
            CurrentAsset::Cash,
        )))
        .entity_id(entity_id)
        .build()
        .expect("Valid account");

    chart.add_account(account1).expect("Add account");

    // Try to add another account with same number
    let account2 = Account::builder()
        .number("1100")
        .name("Petty Cash")
        .class(AccountClass::Asset(AssetType::CurrentAsset(
            CurrentAsset::Cash,
        )))
        .entity_id(entity_id)
        .build()
        .expect("Valid account");

    let result = chart.add_account(account2);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Duplicate account number")
    );
}

#[test]
fn test_entity_mismatch_error() {
    let entity_id1 = EntityId::new_v4();
    let entity_id2 = EntityId::new_v4();
    let mut chart = ChartOfAccounts::new(entity_id1);

    let account = Account::builder()
        .number("1100")
        .name("Cash")
        .class(AccountClass::Asset(AssetType::CurrentAsset(
            CurrentAsset::Cash,
        )))
        .entity_id(entity_id2) // Different entity!
        .build()
        .expect("Valid account");

    let result = chart.add_account(account);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Entity mismatch"));
}

#[test]
fn test_parent_not_found_error() {
    let entity_id = EntityId::new_v4();
    let mut chart = ChartOfAccounts::new(entity_id);

    let account = Account::builder()
        .number("1110")
        .name("Cash")
        .class(AccountClass::Asset(AssetType::CurrentAsset(
            CurrentAsset::Cash,
        )))
        .parent("1100") // Parent doesn't exist yet
        .entity_id(entity_id)
        .build()
        .expect("Valid account");

    let result = chart.add_account(account);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Parent account not found")
    );
}

#[test]
fn test_hierarchical_accounts() {
    let entity_id = EntityId::new_v4();
    let mut chart = ChartOfAccounts::new(entity_id);

    // Add parent first
    let parent = Account::builder()
        .number("1100")
        .name("Current Assets")
        .class(AccountClass::Asset(AssetType::CurrentAsset(
            CurrentAsset::Cash,
        )))
        .entity_id(entity_id)
        .active(false)
        .build()
        .expect("Valid account");

    chart.add_account(parent).expect("Add parent");

    // Add child
    let child = Account::builder()
        .number("1110")
        .name("Cash")
        .class(AccountClass::Asset(AssetType::CurrentAsset(
            CurrentAsset::Cash,
        )))
        .parent("1100")
        .entity_id(entity_id)
        .build()
        .expect("Valid account");

    chart.add_account(child).expect("Add child");

    assert_eq!(chart.len(), 2);

    // Verify child accounts query
    let children = chart.child_accounts("1100");
    assert_eq!(children.len(), 1);
    assert_eq!(children[0].name(), "Cash");
}

#[test]
fn test_standard_small_business_chart() {
    let entity_id = EntityId::new_v4();
    let chart = ChartOfAccounts::standard_small_business(entity_id);

    // Should have multiple accounts
    assert!(chart.len() > 20, "Standard chart should have 20+ accounts");

    // Verify key accounts exist
    assert!(chart.get_account("1110").is_some(), "Cash account");
    assert!(chart.get_account("1120").is_some(), "Accounts Receivable");
    assert!(chart.get_account("1130").is_some(), "Inventory");
    assert!(chart.get_account("2110").is_some(), "Accounts Payable");
    assert!(chart.get_account("3100").is_some(), "Common Stock");
    assert!(chart.get_account("3200").is_some(), "Retained Earnings");
    assert!(chart.get_account("4100").is_some(), "Sales Revenue");
    assert!(chart.get_account("5100").is_some(), "Cost of Goods Sold");
}

#[test]
fn test_accounts_by_class_filter() {
    let entity_id = EntityId::new_v4();
    let chart = ChartOfAccounts::standard_small_business(entity_id);

    let assets = chart.asset_accounts();
    let liabilities = chart.liability_accounts();
    let equity = chart.equity_accounts();
    let revenue = chart.revenue_accounts();
    let expenses = chart.expense_accounts();

    assert!(!assets.is_empty(), "Should have asset accounts");
    assert!(!liabilities.is_empty(), "Should have liability accounts");
    assert!(!equity.is_empty(), "Should have equity accounts");
    assert!(!revenue.is_empty(), "Should have revenue accounts");
    assert!(!expenses.is_empty(), "Should have expense accounts");

    // Verify all assets have debit normal balance
    for account in assets {
        assert_eq!(account.normal_balance(), NormalBalance::Debit);
    }

    // Verify all liabilities have credit normal balance
    for account in liabilities {
        assert_eq!(account.normal_balance(), NormalBalance::Credit);
    }
}

// ─────────────────────────────────────────────────────────────
//  Fixed Asset Tests
// ─────────────────────────────────────────────────────────────

#[test]
fn test_fixed_asset_depreciable() {
    assert!(FixedAsset::Equipment.is_depreciable());
    assert!(FixedAsset::Building.is_depreciable());
    assert!(FixedAsset::Vehicle.is_depreciable());
    assert!(!FixedAsset::Land.is_depreciable());
    assert!(!FixedAsset::AccumulatedDepreciation.is_depreciable());
}

// ─────────────────────────────────────────────────────────────
//  Equity Type Tests
// ─────────────────────────────────────────────────────────────

#[test]
fn test_equity_type_contra() {
    assert!(EquityType::Dividends.is_contra());
    assert!(EquityType::TreasuryStock.is_contra());
    assert!(!EquityType::CommonStock.is_contra());
    assert!(!EquityType::RetainedEarnings.is_contra());
}

// ─────────────────────────────────────────────────────────────
//  Expense Type Tests
// ─────────────────────────────────────────────────────────────

#[test]
fn test_expense_type_cogs() {
    assert!(ExpenseType::CostOfGoodsSold.is_cogs());
    assert!(!ExpenseType::Salaries.is_cogs());
    assert!(!ExpenseType::Rent.is_cogs());
}

#[test]
fn test_expense_type_operating() {
    assert!(ExpenseType::Salaries.is_operating());
    assert!(ExpenseType::Rent.is_operating());
    assert!(ExpenseType::Utilities.is_operating());
    assert!(!ExpenseType::InterestExpense.is_operating());
    assert!(!ExpenseType::TaxExpense.is_operating());
}
