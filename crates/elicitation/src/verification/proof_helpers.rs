//! Helper functions for generating proof code as `proc_macro2::TokenStream`.
//!
//! These functions produce real verification harnesses matching the patterns
//! used in the `elicitation_kani`, `elicitation_verus`, and `elicitation_creusot` crates.

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

// ============================================================================
// Kani Proof Helpers
// ============================================================================

/// Generate a Kani proof for a "positive" numeric constraint (value > 0).
pub fn kani_numeric_positive(type_name: &str, seed_type: &str) -> TokenStream {
    let fn_ident = Ident::new(
        &format!("verify_{}", type_name.to_lowercase()),
        Span::call_site(),
    );
    let type_ident: TokenStream = type_name.parse().expect("valid type ident");
    let seed_tok: TokenStream = seed_type.parse().expect("valid seed type");
    quote! {
        #[kani::proof]
        fn #fn_ident() {
            let value: #seed_tok = kani::any();
            match #type_ident::new(value) {
                Ok(v) => {
                    assert!(value > 0, concat!(stringify!(#type_ident), " invariant: value > 0"));
                    assert!(v.get() > 0, concat!(stringify!(#type_ident), "::get() returns positive value"));
                }
                Err(_) => {
                    assert!(value <= 0, concat!(stringify!(#type_ident), " construction fails when value <= 0"));
                }
            }
        }
    }
}

/// Generate a Kani proof for a "non-negative" numeric constraint (value >= 0).
pub fn kani_numeric_nonneg(type_name: &str, seed_type: &str) -> TokenStream {
    let fn_ident = Ident::new(
        &format!("verify_{}", type_name.to_lowercase()),
        Span::call_site(),
    );
    let type_ident: TokenStream = type_name.parse().expect("valid type ident");
    let seed_tok: TokenStream = seed_type.parse().expect("valid seed type");
    quote! {
        #[kani::proof]
        fn #fn_ident() {
            let value: #seed_tok = kani::any();
            match #type_ident::new(value) {
                Ok(v) => {
                    assert!(value >= 0, concat!(stringify!(#type_ident), " invariant: value >= 0"));
                    assert!(v.get() >= 0, concat!(stringify!(#type_ident), "::get() returns non-negative value"));
                }
                Err(_) => {
                    assert!(value < 0, concat!(stringify!(#type_ident), " construction fails when value < 0"));
                }
            }
        }
    }
}

/// Generate a Kani proof for a "non-zero" numeric constraint (value != 0).
pub fn kani_numeric_nonzero(type_name: &str, seed_type: &str) -> TokenStream {
    let fn_ident = Ident::new(
        &format!("verify_{}", type_name.to_lowercase()),
        Span::call_site(),
    );
    let type_ident: TokenStream = type_name.parse().expect("valid type ident");
    let seed_tok: TokenStream = seed_type.parse().expect("valid seed type");
    quote! {
        #[kani::proof]
        fn #fn_ident() {
            let value: #seed_tok = kani::any();
            match #type_ident::new(value) {
                Ok(v) => {
                    assert!(value != 0, concat!(stringify!(#type_ident), " invariant: value != 0"));
                    assert!(v.get() != 0, concat!(stringify!(#type_ident), "::get() returns non-zero value"));
                }
                Err(_) => {
                    assert!(value == 0, concat!(stringify!(#type_ident), " construction fails when value == 0"));
                }
            }
        }
    }
}

/// Generate a Kani proof for BoolTrue (value must be true).
pub fn kani_bool_true(type_name: &str) -> TokenStream {
    let fn_ident = Ident::new(
        &format!("verify_{}", type_name.to_lowercase()),
        Span::call_site(),
    );
    let type_ident: TokenStream = type_name.parse().expect("valid type ident");
    quote! {
        #[kani::proof]
        fn #fn_ident() {
            let value: bool = kani::any();
            match #type_ident::new(value) {
                Ok(v) => {
                    assert!(value, concat!(stringify!(#type_ident), " invariant: value == true"));
                    assert!(v.get(), concat!(stringify!(#type_ident), "::get() returns true"));
                }
                Err(_) => {
                    assert!(!value, concat!(stringify!(#type_ident), " construction fails when value == false"));
                }
            }
        }
    }
}

/// Generate a Kani proof for BoolFalse (value must be false).
pub fn kani_bool_false(type_name: &str) -> TokenStream {
    let fn_ident = Ident::new(
        &format!("verify_{}", type_name.to_lowercase()),
        Span::call_site(),
    );
    let type_ident: TokenStream = type_name.parse().expect("valid type ident");
    quote! {
        #[kani::proof]
        fn #fn_ident() {
            let value: bool = kani::any();
            match #type_ident::new(value) {
                Ok(v) => {
                    assert!(!value, concat!(stringify!(#type_ident), " invariant: value == false"));
                    assert!(!v.get(), concat!(stringify!(#type_ident), "::get() returns false"));
                }
                Err(_) => {
                    assert!(value, concat!(stringify!(#type_ident), " construction fails when value == true"));
                }
            }
        }
    }
}

/// Generate a Kani proof for a positive float constraint (value > 0.0 && is_finite()).
pub fn kani_float_positive(type_name: &str, seed_type: &str) -> TokenStream {
    let fn_ident = Ident::new(
        &format!("verify_{}", type_name.to_lowercase()),
        Span::call_site(),
    );
    let type_ident: TokenStream = type_name.parse().expect("valid type ident");
    let seed_tok: TokenStream = seed_type.parse().expect("valid seed type");
    quote! {
        #[kani::proof]
        fn #fn_ident() {
            let value: #seed_tok = kani::any();
            kani::assume(value.is_finite());
            match #type_ident::new(value) {
                Ok(v) => {
                    assert!(value > 0.0, concat!(stringify!(#type_ident), " invariant: value > 0.0"));
                    assert!(v.get() > 0.0, concat!(stringify!(#type_ident), "::get() returns positive value"));
                    assert!(v.get().is_finite(), concat!(stringify!(#type_ident), "::get() is finite"));
                }
                Err(_) => {
                    assert!(value <= 0.0, concat!(stringify!(#type_ident), " construction fails when value <= 0.0"));
                }
            }
        }
    }
}

/// Generate a Kani proof for a non-negative float constraint (value >= 0.0 && is_finite()).
pub fn kani_float_nonneg(type_name: &str, seed_type: &str) -> TokenStream {
    let fn_ident = Ident::new(
        &format!("verify_{}", type_name.to_lowercase()),
        Span::call_site(),
    );
    let type_ident: TokenStream = type_name.parse().expect("valid type ident");
    let seed_tok: TokenStream = seed_type.parse().expect("valid seed type");
    quote! {
        #[kani::proof]
        fn #fn_ident() {
            let value: #seed_tok = kani::any();
            kani::assume(value.is_finite());
            match #type_ident::new(value) {
                Ok(v) => {
                    assert!(value >= 0.0, concat!(stringify!(#type_ident), " invariant: value >= 0.0"));
                    assert!(v.get() >= 0.0, concat!(stringify!(#type_ident), "::get() returns non-negative value"));
                }
                Err(_) => {
                    assert!(value < 0.0, concat!(stringify!(#type_ident), " construction fails when value < 0.0"));
                }
            }
        }
    }
}

/// Generate a Kani proof for a finite float constraint (value.is_finite()).
pub fn kani_float_finite(type_name: &str, seed_type: &str) -> TokenStream {
    let fn_ident = Ident::new(
        &format!("verify_{}", type_name.to_lowercase()),
        Span::call_site(),
    );
    let type_ident: TokenStream = type_name.parse().expect("valid type ident");
    let seed_tok: TokenStream = seed_type.parse().expect("valid seed type");
    quote! {
        #[kani::proof]
        fn #fn_ident() {
            let value: #seed_tok = kani::any();
            match #type_ident::new(value) {
                Ok(v) => {
                    assert!(value.is_finite(), concat!(stringify!(#type_ident), " invariant: value.is_finite()"));
                    assert!(v.get().is_finite(), concat!(stringify!(#type_ident), "::get() returns finite value"));
                }
                Err(_) => {
                    assert!(!value.is_finite(), concat!(stringify!(#type_ident), " construction fails when not finite"));
                }
            }
        }
    }
}

// ============================================================================
// Verus Proof Helpers
// ============================================================================

/// Generate a Verus proof for a "positive" numeric constraint (value > 0).
pub fn verus_numeric_positive(type_name: &str, seed_type: &str) -> TokenStream {
    let type_ident: TokenStream = type_name.parse().expect("valid type ident");
    let seed_tok: TokenStream = seed_type.parse().expect("valid seed type");
    let valid_fn: TokenStream = format!("verify_{}_valid", type_name.to_lowercase())
        .parse()
        .expect("valid fn ident");
    let invalid_fn: TokenStream = format!("verify_{}_invalid", type_name.to_lowercase())
        .parse()
        .expect("valid fn ident");
    quote! {
        verus! {
            pub fn #valid_fn(value: #seed_tok) -> (result: #type_ident)
                requires value > 0,
                ensures result.get() == value,
            {
                #type_ident::new(value).expect("verus: positive invariant")
            }

            pub fn #invalid_fn(value: #seed_tok)
                requires value <= 0,
            {
                assert!(#type_ident::new(value).is_err());
            }
        }
    }
}

/// Generate a Verus proof for a "non-zero" numeric constraint (value != 0).
pub fn verus_numeric_nonzero(type_name: &str, seed_type: &str) -> TokenStream {
    let type_ident: TokenStream = type_name.parse().expect("valid type ident");
    let seed_tok: TokenStream = seed_type.parse().expect("valid seed type");
    let valid_fn: TokenStream = format!("verify_{}_valid", type_name.to_lowercase())
        .parse()
        .expect("valid fn ident");
    let invalid_fn: TokenStream = format!("verify_{}_invalid", type_name.to_lowercase())
        .parse()
        .expect("valid fn ident");
    quote! {
        verus! {
            pub fn #valid_fn(value: #seed_tok) -> (result: #type_ident)
                requires value != 0,
                ensures result.get() == value,
            {
                #type_ident::new(value).expect("verus: nonzero invariant")
            }

            pub fn #invalid_fn(value: #seed_tok)
                requires value == 0,
            {
                assert!(#type_ident::new(value).is_err());
            }
        }
    }
}

// ============================================================================
// Creusot Proof Helpers
// ============================================================================

/// Generate a Creusot proof for a "positive" numeric constraint (value > 0).
pub fn creusot_numeric_positive(type_name: &str, seed_type: &str) -> TokenStream {
    let type_ident: TokenStream = type_name.parse().expect("valid type ident");
    let seed_tok: TokenStream = seed_type.parse().expect("valid seed type");
    let valid_fn: TokenStream = format!("verify_{}_valid", type_name.to_lowercase())
        .parse()
        .expect("valid fn ident");
    let invalid_fn: TokenStream = format!("verify_{}_invalid", type_name.to_lowercase())
        .parse()
        .expect("valid fn ident");
    quote! {
        #[requires(value@ > 0)]
        #[ensures(match result { Ok(_) => true, Err(_) => false })]
        #[trusted]
        pub fn #valid_fn(value: #seed_tok) -> Result<#type_ident, crate::ValidationError> {
            #type_ident::new(value)
        }

        #[requires(value@ <= 0)]
        #[ensures(match result { Ok(_) => false, Err(_) => true })]
        #[trusted]
        pub fn #invalid_fn(value: #seed_tok) -> Result<#type_ident, crate::ValidationError> {
            #type_ident::new(value)
        }
    }
}

/// Generate a Creusot proof for a "non-zero" numeric constraint (value != 0).
pub fn creusot_numeric_nonzero(type_name: &str, seed_type: &str) -> TokenStream {
    let type_ident: TokenStream = type_name.parse().expect("valid type ident");
    let seed_tok: TokenStream = seed_type.parse().expect("valid seed type");
    let valid_fn: TokenStream = format!("verify_{}_valid", type_name.to_lowercase())
        .parse()
        .expect("valid fn ident");
    let invalid_fn: TokenStream = format!("verify_{}_invalid", type_name.to_lowercase())
        .parse()
        .expect("valid fn ident");
    quote! {
        #[requires(value@ != 0)]
        #[ensures(match result { Ok(_) => true, Err(_) => false })]
        #[trusted]
        pub fn #valid_fn(value: #seed_tok) -> Result<#type_ident, crate::ValidationError> {
            #type_ident::new(value)
        }

        #[requires(value@ == 0)]
        #[ensures(match result { Ok(_) => false, Err(_) => true })]
        #[trusted]
        pub fn #invalid_fn(value: #seed_tok) -> Result<#type_ident, crate::ValidationError> {
            #type_ident::new(value)
        }
    }
}
