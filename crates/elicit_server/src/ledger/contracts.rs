//! Transfer-level proof propositions for the ledger typestate machine.
//!
//! These propositions are structural proof tokens — they carry no runtime data and
//! compose freely with `both()` / `And<A, B>`. Backends establish them by calling
//! `Established::assert()` after satisfying the stated criterion.

use elicitation::contracts::And;

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    macro_rules! structural_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural */ }
                }
            }
        };
    }

    /// Transfer amount is positive (> 0).
    ///
    /// Source: pre-ASC ledger invariant — amounts must be non-zero and non-negative.
    pub struct AmountPositive;

    /// Source account holds at least the transfer amount.
    ///
    /// Source: pre-ASC ledger invariant — no overdraft without explicit credit facility.
    pub struct SufficientFunds;

    /// Source and destination accounts are distinct.
    ///
    /// Source: ASC 230 — Statement of Cash Flows; gross vs. net presentation.
    pub struct AccountsDistinct;

    /// Debit entry and credit entry balance (debit + credit = 0).
    ///
    /// Source: pre-ASC foundational double-entry requirement.
    pub struct BalancedEntries;

    structural_prop!(AmountPositive, "AmountPositive");
    structural_prop!(SufficientFunds, "SufficientFunds");
    structural_prop!(AccountsDistinct, "AccountsDistinct");
    structural_prop!(BalancedEntries, "BalancedEntries");
}

pub use emit_impls::{AccountsDistinct, AmountPositive, BalancedEntries, SufficientFunds};

/// Composite: transfer is valid when amount, funds, and account identity all hold.
pub type ValidTransfer = And<AmountPositive, And<SufficientFunds, AccountsDistinct>>;
