//! Tests for journal entry types, builder, and state transitions.

use chrono::NaiveDate;

use elicit_server::ledger2::{
    Account, AccountClass, Amount, AssetType, CurrentAsset, EntityId, JournalEntry,
    JournalEntryErrorKind, RevenueType,
};

// ─────────────────────────────────────────────────────────────
//  Test Helpers
// ─────────────────────────────────────────────────────────────

fn test_entity() -> EntityId {
    EntityId::new_v4()
}

fn test_date() -> NaiveDate {
    NaiveDate::from_ymd_opt(2024, 1, 15).expect("Valid date")
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

fn inactive_account(entity_id: EntityId) -> Account {
    Account::builder()
        .number("9999")
        .name("Inactive Account")
        .class(AccountClass::Asset(AssetType::CurrentAsset(
            CurrentAsset::Cash,
        )))
        .entity_id(entity_id)
        .active(false)
        .build()
        .expect("Valid account")
}

// ─────────────────────────────────────────────────────────────
//  Amount Tests
// ─────────────────────────────────────────────────────────────

#[test]
fn test_amount_from_dollars() {
    let amount = Amount::from_dollars(100);
    assert_eq!(amount.cents(), 10000);
    assert_eq!(amount.dollars(), 100);
}

#[test]
fn test_amount_from_cents() {
    let amount = Amount::from_cents(12345);
    assert_eq!(amount.cents(), 12345);
    assert_eq!(amount.dollars(), 123);
    assert_eq!(amount.cents_component(), 45);
}

#[test]
fn test_amount_display() {
    assert_eq!(Amount::from_dollars(100).to_string(), "$100.00");
    assert_eq!(Amount::from_cents(12345).to_string(), "$123.45");
    assert_eq!(Amount::from_cents(-5000).to_string(), "-$50.00");
}

#[test]
fn test_amount_arithmetic() {
    let a = Amount::from_dollars(100);
    let b = Amount::from_dollars(50);

    assert_eq!(a + b, Amount::from_dollars(150));
    assert_eq!(a - b, Amount::from_dollars(50));
    assert_eq!(-a, Amount::from_dollars(-100));
}

#[test]
fn test_amount_predicates() {
    let positive = Amount::from_dollars(100);
    let negative = Amount::from_dollars(-50);
    let zero = Amount::from_cents(0);

    assert!(positive.is_positive());
    assert!(!positive.is_negative());
    assert!(!positive.is_zero());

    assert!(!negative.is_positive());
    assert!(negative.is_negative());
    assert!(!negative.is_zero());

    assert!(!zero.is_positive());
    assert!(!zero.is_negative());
    assert!(zero.is_zero());
}

// ─────────────────────────────────────────────────────────────
//  Journal Entry Builder Tests
// ─────────────────────────────────────────────────────────────

#[test]
fn test_build_balanced_entry() {
    let entity_id = test_entity();
    let cash = cash_account(entity_id);
    let revenue = revenue_account(entity_id);

    let result = JournalEntry::builder(entity_id, test_date())
        .description("Cash sale")
        .debit(cash, Amount::from_dollars(100), "Payment received")
        .credit(revenue, Amount::from_dollars(100), "Sale of goods")
        .build();

    assert!(result.is_ok());
    let entry = result.unwrap();
    assert_eq!(entry.total_debits(), Amount::from_dollars(100));
    assert_eq!(entry.total_credits(), Amount::from_dollars(100));
    assert!(entry.is_balanced());
}

#[test]
fn test_build_multi_line_entry() {
    let entity_id = test_entity();
    let cash = cash_account(entity_id);
    let revenue = revenue_account(entity_id);

    // Entry with multiple credits
    let result = JournalEntry::builder(entity_id, test_date())
        .description("Split sale")
        .debit(cash.clone(), Amount::from_dollars(150), "Payment received")
        .credit(revenue.clone(), Amount::from_dollars(100), "Product sale")
        .credit(revenue, Amount::from_dollars(50), "Service fee")
        .build();

    assert!(result.is_ok());
    let entry = result.unwrap();
    assert_eq!(entry.total_debits(), Amount::from_dollars(150));
    assert_eq!(entry.total_credits(), Amount::from_dollars(150));
    assert_eq!(entry.lines().len(), 3);
}

#[test]
fn test_build_imbalanced_entry_fails() {
    let entity_id = test_entity();
    let cash = cash_account(entity_id);
    let revenue = revenue_account(entity_id);

    let result = JournalEntry::builder(entity_id, test_date())
        .description("Imbalanced entry")
        .debit(cash, Amount::from_dollars(100), "Too much debit")
        .credit(revenue, Amount::from_dollars(50), "Not enough credit")
        .build();

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(err.kind, JournalEntryErrorKind::Imbalance { .. }));
}

#[test]
fn test_build_empty_entry_fails() {
    let entity_id = test_entity();

    let result = JournalEntry::builder(entity_id, test_date())
        .description("Empty entry")
        .build();

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(err.kind, JournalEntryErrorKind::EmptyEntry));
}

#[test]
fn test_build_single_line_entry_fails() {
    let entity_id = test_entity();
    let cash = cash_account(entity_id);

    let result = JournalEntry::builder(entity_id, test_date())
        .description("Single line entry")
        .debit(cash, Amount::from_dollars(100), "Only one line")
        .build();

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(err.kind, JournalEntryErrorKind::SingleLine));
}

#[test]
fn test_build_without_description_fails() {
    let entity_id = test_entity();
    let cash = cash_account(entity_id);
    let revenue = revenue_account(entity_id);

    let result = JournalEntry::builder(entity_id, test_date())
        .debit(cash, Amount::from_dollars(100), "Payment")
        .credit(revenue, Amount::from_dollars(100), "Sale")
        .build();

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(
        err.kind,
        JournalEntryErrorKind::GaapValidation { .. }
    ));
}

#[test]
fn test_build_with_inactive_account_fails() {
    let entity_id = test_entity();
    let inactive = inactive_account(entity_id);
    let revenue = revenue_account(entity_id);

    let result = JournalEntry::builder(entity_id, test_date())
        .description("Using inactive account")
        .debit(inactive, Amount::from_dollars(100), "Payment")
        .credit(revenue, Amount::from_dollars(100), "Sale")
        .build();

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(
        err.kind,
        JournalEntryErrorKind::InactiveAccount { .. }
    ));
}

#[test]
fn test_build_with_entity_mismatch_fails() {
    let entity1 = test_entity();
    let entity2 = test_entity();
    let cash = cash_account(entity1);
    let revenue = revenue_account(entity2);

    let result = JournalEntry::builder(entity1, test_date())
        .description("Entity mismatch")
        .debit(cash, Amount::from_dollars(100), "Payment")
        .credit(revenue, Amount::from_dollars(100), "Sale")
        .build();

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(err.kind, JournalEntryErrorKind::EntityMismatch));
}

// ─────────────────────────────────────────────────────────────
//  State Transition Tests
// ─────────────────────────────────────────────────────────────

#[test]
fn test_balanced_to_posted_transition() {
    let entity_id = test_entity();
    let cash = cash_account(entity_id);
    let revenue = revenue_account(entity_id);

    let balanced = JournalEntry::builder(entity_id, test_date())
        .description("Cash sale")
        .debit(cash, Amount::from_dollars(100), "Payment received")
        .credit(revenue, Amount::from_dollars(100), "Sale of goods")
        .build()
        .expect("Valid entry");

    let posted = balanced.post();

    assert_eq!(posted.total_debits(), Amount::from_dollars(100));
    assert_eq!(posted.total_credits(), Amount::from_dollars(100));
    assert!(posted.is_balanced());
    assert!(posted.posted_at() > posted.created_at());
}

#[test]
fn test_posted_to_closed_transition() {
    let entity_id = test_entity();
    let cash = cash_account(entity_id);
    let revenue = revenue_account(entity_id);

    let balanced = JournalEntry::builder(entity_id, test_date())
        .description("Cash sale")
        .debit(cash, Amount::from_dollars(100), "Payment received")
        .credit(revenue, Amount::from_dollars(100), "Sale of goods")
        .build()
        .expect("Valid entry");

    let posted = balanced.post();
    let closed = posted.close();

    assert_eq!(closed.total_debits(), Amount::from_dollars(100));
    assert_eq!(closed.total_credits(), Amount::from_dollars(100));
    assert!(closed.is_balanced());
    assert!(closed.closed_at() > closed.posted_at());
}

#[test]
fn test_full_state_machine() {
    let entity_id = test_entity();
    let cash = cash_account(entity_id);
    let revenue = revenue_account(entity_id);

    // Balanced (from builder)
    let balanced = JournalEntry::builder(entity_id, test_date())
        .description("Cash sale")
        .debit(cash, Amount::from_dollars(100), "Payment received")
        .credit(revenue, Amount::from_dollars(100), "Sale of goods")
        .build()
        .expect("Valid entry");

    assert!(balanced.is_balanced());

    // Posted
    let posted = balanced.post();
    assert!(posted.is_balanced());

    // Closed
    let closed = posted.close();
    assert!(closed.is_balanced());
    assert!(closed.closed_at() > closed.posted_at());
    assert!(closed.posted_at() > closed.created_at());
}

// ─────────────────────────────────────────────────────────────
//  Entry Properties Tests
// ─────────────────────────────────────────────────────────────

#[test]
fn test_entry_id_is_unique() {
    let entity_id = test_entity();
    let cash = cash_account(entity_id);
    let revenue = revenue_account(entity_id);

    let entry1 = JournalEntry::builder(entity_id, test_date())
        .description("Entry 1")
        .debit(cash.clone(), Amount::from_dollars(100), "Payment")
        .credit(revenue.clone(), Amount::from_dollars(100), "Sale")
        .build()
        .expect("Valid entry");

    let entry2 = JournalEntry::builder(entity_id, test_date())
        .description("Entry 2")
        .debit(cash, Amount::from_dollars(200), "Payment")
        .credit(revenue, Amount::from_dollars(200), "Sale")
        .build()
        .expect("Valid entry");

    assert_ne!(entry1.entry_id(), entry2.entry_id());
}

#[test]
fn test_entry_display() {
    let entity_id = test_entity();
    let cash = cash_account(entity_id);
    let revenue = revenue_account(entity_id);

    let entry = JournalEntry::builder(entity_id, test_date())
        .description("Cash sale")
        .debit(cash, Amount::from_dollars(100), "Payment received")
        .credit(revenue, Amount::from_dollars(100), "Sale of goods")
        .build()
        .expect("Valid entry");

    let display = format!("{}", entry);
    assert!(display.contains("Cash sale"));
    assert!(display.contains("$100.00"));
}
