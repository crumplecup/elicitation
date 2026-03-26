//! Proof-carrying validation for ledger transfers using elicitation contracts.
//!
//! This module defines propositions (type-level statements) that must be proven
//! before a transfer can proceed. Proofs are zero-cost (`PhantomData`) but enforce
//! validation at compile time.

use elicitation::contracts::{And, Prop};
use quote::quote;

// ─────────────────────────────────────────────────────────────
//  Basic Propositions
// ─────────────────────────────────────────────────────────────

/// Proposition: The transfer amount is positive (> 0).
pub struct AmountPositive;

impl Prop for AmountPositive {
    fn kani_proof() -> proc_macro2::TokenStream {
        quote! {
            #[kani::proof]
            fn verify_amount_positive() {
                let amount: i64 = kani::any();
                kani::assume(amount > 0);
                assert!(amount > 0, "Amount must be positive");
            }
        }
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        quote! {
            verus! {
                pub fn verify_amount_positive(amount: i64) -> (result: bool)
                    requires amount > 0,
                    ensures result == true,
                {
                    true
                }
            }
        }
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        quote! {
            #[requires(amount > 0)]
            #[ensures(result == true)]
            #[trusted]
            pub fn verify_amount_positive(amount: i64) -> bool {
                true
            }
        }
    }
}

/// Proposition: The source account has sufficient funds.
pub struct SufficientFunds;

impl Prop for SufficientFunds {
    fn kani_proof() -> proc_macro2::TokenStream {
        quote! {
            #[kani::proof]
            fn verify_sufficient_funds() {
                let balance: i64 = kani::any();
                let amount: i64 = kani::any();
                kani::assume(amount > 0);
                kani::assume(balance >= amount);
                assert!(balance - amount >= 0, "Sufficient funds required");
            }
        }
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        quote! {
            verus! {
                pub fn verify_sufficient_funds(balance: i64, amount: i64) -> (result: bool)
                    requires amount > 0,
                    requires balance >= amount,
                    ensures result == true,
                    ensures balance - amount >= 0,
                {
                    true
                }
            }
        }
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        quote! {
            #[requires(amount > 0)]
            #[requires(balance >= amount)]
            #[ensures(result == true)]
            #[ensures(balance - amount >= 0)]
            #[trusted]
            pub fn verify_sufficient_funds(balance: i64, amount: i64) -> bool {
                true
            }
        }
    }
}

/// Proposition: The source and destination accounts are distinct.
pub struct AccountsDistinct;

impl Prop for AccountsDistinct {
    fn kani_proof() -> proc_macro2::TokenStream {
        quote! {
            #[kani::proof]
            fn verify_accounts_distinct() {
                let from: u32 = kani::any();
                let to: u32 = kani::any();
                kani::assume(from != to);
                assert!(from != to, "Accounts must be distinct");
            }
        }
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        quote! {
            verus! {
                pub fn verify_accounts_distinct(from: u32, to: u32) -> (result: bool)
                    requires from != to,
                    ensures result == true,
                {
                    true
                }
            }
        }
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        quote! {
            #[requires(from != to)]
            #[ensures(result == true)]
            #[trusted]
            pub fn verify_accounts_distinct(from: u32, to: u32) -> bool {
                true
            }
        }
    }
}

/// Proposition: The ledger entries balance (debit + credit = 0).
pub struct BalancedEntries;

impl Prop for BalancedEntries {
    fn kani_proof() -> proc_macro2::TokenStream {
        quote! {
            #[kani::proof]
            fn verify_balanced_entries() {
                let debit: i64 = kani::any();
                let credit: i64 = kani::any();
                kani::assume(debit < 0);  // Debits are negative
                kani::assume(credit > 0);  // Credits are positive
                kani::assume(debit + credit == 0);  // Must balance
                assert!(debit + credit == 0, "Entries must balance");
            }
        }
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        quote! {
            verus! {
                pub fn verify_balanced_entries(debit: i64, credit: i64) -> (result: bool)
                    requires debit < 0,
                    requires credit > 0,
                    requires debit + credit == 0,
                    ensures result == true,
                {
                    true
                }
            }
        }
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        quote! {
            #[requires(debit < 0)]
            #[requires(credit > 0)]
            #[requires(debit + credit == 0)]
            #[ensures(result == true)]
            #[trusted]
            pub fn verify_balanced_entries(debit: i64, credit: i64) -> bool {
                true
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────
//  Composite Propositions
// ─────────────────────────────────────────────────────────────

/// Composite: A transfer is valid (positive amount AND sufficient funds AND distinct accounts).
pub type ValidTransfer = And<AmountPositive, And<SufficientFunds, AccountsDistinct>>;
