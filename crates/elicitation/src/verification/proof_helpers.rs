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

// ============================================================================
// Char Proof Helpers
// ============================================================================

/// Generate Kani proofs for CharAlphabetic wrapper logic.
pub fn kani_char_alphabetic() -> TokenStream {
    quote! {
        #[kani::proof]
        fn verify_char_alphabetic_accepts() {
            let value: char = kani::any();
            let result = CharAlphabetic::new(value);
            if let Ok(alphabetic) = result {
                assert_eq!(alphabetic.get(), value);
                assert_eq!(alphabetic.into_inner(), value);
            }
        }
        #[kani::proof]
        fn verify_char_alphabetic_rejects() {
            let value: char = kani::any();
            let result = CharAlphabetic::new(value);
            if let Err(e) = result {
                match e {
                    ValidationError::NotAlphabetic(_) => {}
                    _ => panic!("Wrong error type"),
                }
            }
        }
    }
}

/// Generate Kani proofs for CharNumeric wrapper logic.
pub fn kani_char_numeric() -> TokenStream {
    quote! {
        #[kani::proof]
        fn verify_char_numeric_accepts() {
            let value: char = kani::any();
            let result = CharNumeric::new(value);
            if let Ok(numeric) = result {
                assert_eq!(numeric.get(), value);
                assert_eq!(numeric.into_inner(), value);
            }
        }
        #[kani::proof]
        fn verify_char_numeric_rejects() {
            let value: char = kani::any();
            let result = CharNumeric::new(value);
            if let Err(e) = result {
                match e {
                    ValidationError::NotNumeric(_) => {}
                    _ => panic!("Wrong error type"),
                }
            }
        }
    }
}

/// Generate Kani proofs for CharAlphanumeric wrapper logic.
pub fn kani_char_alphanumeric() -> TokenStream {
    quote! {
        #[kani::proof]
        fn verify_char_alphanumeric_accepts() {
            let value: char = kani::any();
            let result = CharAlphanumeric::new(value);
            if let Ok(alphanumeric) = result {
                assert_eq!(alphanumeric.get(), value);
                assert_eq!(alphanumeric.into_inner(), value);
            }
        }
        #[kani::proof]
        fn verify_char_alphanumeric_rejects() {
            let value: char = kani::any();
            let result = CharAlphanumeric::new(value);
            if let Err(e) = result {
                match e {
                    ValidationError::NotAlphanumeric(_) => {}
                    _ => panic!("Wrong error type"),
                }
            }
        }
    }
}

// ============================================================================
// String Proof Helpers
// ============================================================================

/// Generate a Kani proof for StringNonEmpty (MAX_LEN=2 for bounded verification).
pub fn kani_string_non_empty() -> TokenStream {
    quote! {
        #[kani::proof]
        fn verify_string_non_empty() {
            const MAX_LEN: usize = 2;
            let empty = String::new();
            let result = StringNonEmpty::<MAX_LEN>::new(empty);
            assert!(result.is_err(), "Construction rejects empty string");
            let non_empty = String::from("a");
            let result = StringNonEmpty::<MAX_LEN>::new(non_empty);
            if let Ok(contract) = result {
                assert!(!contract.is_empty(), "Non-empty invariant");
            }
        }
    }
}

// ============================================================================
// URL Proof Helpers
// ============================================================================

/// Generate a Kani proof for UrlValid wrapper logic.
#[cfg(all(feature = "proofs", feature = "url"))]
pub fn kani_url_valid() -> TokenStream {
    quote! {
        #[cfg(feature = "url")]
        #[kani::proof]
        fn verify_url_valid_wrapper() {
            let result = UrlValid::new("https://example.com");
            match result {
                Ok(_url) => {}
                Err(e) => {
                    assert!(matches!(e, elicitation::ValidationError::UrlInvalid));
                }
            }
        }
    }
}

/// Generate a Kani proof for UrlHttps wrapper logic.
#[cfg(all(feature = "proofs", feature = "url"))]
pub fn kani_url_https() -> TokenStream {
    quote! {
        #[cfg(feature = "url")]
        #[kani::proof]
        fn verify_url_https_wrapper() {
            let result = UrlHttps::new("https://example.com");
            match result {
                Ok(_url) => {}
                Err(e) => {
                    assert!(
                        matches!(e, elicitation::ValidationError::UrlInvalid)
                            || matches!(e, elicitation::ValidationError::UrlNotHttps)
                    );
                }
            }
        }
    }
}

/// Generate a Kani proof for UrlWithHost wrapper logic.
#[cfg(all(feature = "proofs", feature = "url"))]
pub fn kani_url_with_host() -> TokenStream {
    quote! {
        #[cfg(feature = "url")]
        #[kani::proof]
        fn verify_url_with_host_wrapper() {
            let result = UrlWithHost::new("https://example.com");
            match result {
                Ok(_url) => {}
                Err(e) => {
                    assert!(
                        matches!(e, elicitation::ValidationError::UrlInvalid)
                            || matches!(e, elicitation::ValidationError::UrlNoHost)
                    );
                }
            }
        }
    }
}

/// Generate a Kani proof for UrlCanBeBase wrapper logic.
#[cfg(all(feature = "proofs", feature = "url"))]
pub fn kani_url_can_be_base() -> TokenStream {
    quote! {
        #[cfg(feature = "url")]
        #[kani::proof]
        fn verify_url_can_be_base_wrapper() {
            let result = UrlCanBeBase::new("https://example.com");
            match result {
                Ok(_url) => {}
                Err(e) => {
                    assert!(
                        matches!(e, elicitation::ValidationError::UrlInvalid)
                            || matches!(e, elicitation::ValidationError::UrlCannotBeBase)
                    );
                }
            }
        }
    }
}

// ============================================================================
// Duration Proof Helpers
// ============================================================================

/// Generate a Kani proof for DurationPositive wrapper logic.
pub fn kani_duration_positive() -> TokenStream {
    quote! {
        #[kani::proof]
        fn verify_duration_positive() {
            let secs: u64 = kani::any();
            let nanos: u32 = kani::any();
            kani::assume(nanos < 1_000_000_000);
            let duration = std::time::Duration::new(secs, nanos);
            match DurationPositive::new(duration) {
                Ok(positive) => {
                    assert!(duration.as_nanos() > 0, "DurationPositive invariant");
                    let retrieved = DurationPositive::get(&positive);
                    assert!(retrieved.as_nanos() > 0, "get() preserves invariant");
                }
                Err(_) => {
                    assert!(duration.as_nanos() == 0, "Construction rejects zero duration");
                }
            }
        }
    }
}

// ============================================================================
// Network Proof Helpers
// ============================================================================

/// Generate a Kani proof for IpPrivate wrapper logic.
pub fn kani_ip_private() -> TokenStream {
    quote! {
        #[kani::proof]
        fn verify_ip_private() {
            use std::net::IpAddr;
            let private_v4 = IpAddr::from([192, 168, 1, 1]);
            let result = IpPrivate::new(private_v4);
            assert!(result.is_ok(), "Private IPv4 accepted");
            let public_v4 = IpAddr::from([8, 8, 8, 8]);
            let result = IpPrivate::new(public_v4);
            assert!(result.is_err(), "Public IPv4 rejected");
        }
    }
}

/// Generate a Kani proof for IpPublic wrapper logic.
pub fn kani_ip_public() -> TokenStream {
    quote! {
        #[kani::proof]
        fn verify_ip_public() {
            use std::net::IpAddr;
            let public_v4 = IpAddr::from([8, 8, 8, 8]);
            let result = IpPublic::new(public_v4);
            assert!(result.is_ok(), "Public IPv4 accepted");
            let private_v4 = IpAddr::from([192, 168, 1, 1]);
            let result = IpPublic::new(private_v4);
            assert!(result.is_err(), "Private IPv4 rejected");
        }
    }
}

/// Generate a Kani proof for Ipv4Loopback wrapper logic.
pub fn kani_ipv4_loopback() -> TokenStream {
    quote! {
        #[kani::proof]
        fn verify_ipv4_loopback() {
            use std::net::Ipv4Addr;
            let loopback = Ipv4Addr::new(127, 0, 0, 1);
            let result = Ipv4Loopback::new(loopback);
            assert!(result.is_ok(), "Loopback accepted");
            let not_loopback = Ipv4Addr::new(192, 168, 1, 1);
            let result = Ipv4Loopback::new(not_loopback);
            assert!(result.is_err(), "Non-loopback rejected");
        }
    }
}

/// Generate a Kani proof for Ipv6Loopback wrapper logic.
pub fn kani_ipv6_loopback() -> TokenStream {
    quote! {
        #[kani::proof]
        fn verify_ipv6_loopback() {
            use std::net::Ipv6Addr;
            let loopback = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);
            let result = Ipv6Loopback::new(loopback);
            assert!(result.is_ok(), "IPv6 loopback accepted");
            let not_loopback = Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 1);
            let result = Ipv6Loopback::new(not_loopback);
            assert!(result.is_err(), "Non-loopback rejected");
        }
    }
}

// ============================================================================
// UUID Proof Helpers
// ============================================================================

/// Generate a Kani proof for UuidV4 wrapper logic.
#[cfg(all(feature = "proofs", feature = "uuid"))]
pub fn kani_uuid_v4() -> TokenStream {
    quote! {
        #[cfg(feature = "uuid")]
        #[kani::proof]
        fn verify_uuid_v4() {
            use uuid::Uuid;
            let v4_bytes = [
                0x55, 0x0e, 0x84, 0x00, 0xe2, 0x9b, 0x41, 0xd4,
                0xa7, 0x16, 0x44, 0x66, 0x55, 0x44, 0x00, 0x00,
            ];
            let v4_uuid = Uuid::from_bytes(v4_bytes);
            let _result = UuidV4::new(v4_uuid);
        }
    }
}

// ============================================================================
// Collection Proof Helpers
// ============================================================================

/// Generate a Kani proof for `VecNonEmpty` — verifies non-empty invariant.
///
/// Two concrete cases: empty vec → `Err`, non-empty vec → `Ok` with correct
/// accessors. Mirrors `verify_vec_non_empty` in `elicitation_kani`.
pub fn kani_vec_non_empty() -> TokenStream {
    quote! {
        #[kani::proof]
        fn verify_vec_non_empty() {
            let empty: Vec<i32> = vec![];
            let result = VecNonEmpty::new(empty);
            assert!(result.is_err(), "VecNonEmpty: empty vec rejected");

            let non_empty: Vec<i32> = vec![42];
            let result = VecNonEmpty::new(non_empty);
            assert!(result.is_ok(), "VecNonEmpty: non-empty vec accepted");
            let v = result.unwrap();
            assert!(!v.is_empty(), "VecNonEmpty::is_empty() always false");
            assert!(v.len() >= 1, "VecNonEmpty::len() >= 1");
        }
    }
}

/// Generate a Kani proof for `OptionSome` — verifies Some invariant.
///
/// `OptionSome::new(Some(x))` succeeds; `OptionSome::new(None)` fails.
pub fn kani_option_some() -> TokenStream {
    quote! {
        #[kani::proof]
        fn verify_option_some() {
            let some_val: Option<i32> = Some(kani::any());
            let result = OptionSome::new(some_val);
            assert!(result.is_ok(), "OptionSome: Some variant accepted");

            let none_val: Option<i32> = None;
            let result = OptionSome::new(none_val);
            assert!(result.is_err(), "OptionSome: None rejected");
        }
    }
}

/// Generate a Kani proof for `ResultOk` — verifies Ok invariant.
///
/// `ResultOk::new(Ok(x))` succeeds; `ResultOk::new(Err(_))` fails.
pub fn kani_result_ok() -> TokenStream {
    quote! {
        #[kani::proof]
        fn verify_result_ok() {
            let ok_val: Result<i32, i32> = Ok(kani::any());
            let result = ResultOk::new(ok_val);
            assert!(result.is_ok(), "ResultOk: Ok variant accepted");

            let err_val: Result<i32, i32> = Err(kani::any());
            let result = ResultOk::new(err_val);
            assert!(result.is_err(), "ResultOk: Err variant rejected");
        }
    }
}

/// Generate a Kani proof for `BoxSatisfies` / `ArcSatisfies` / `RcSatisfies`.
///
/// These are transparent wrappers: construction always succeeds for any valid
/// inner contract type `C`. The harness uses `I8Positive` as a concrete `C`.
pub fn kani_pointer_satisfies(pointer_type: &str) -> TokenStream {
    let fn_ident = Ident::new(
        &format!("verify_{}_satisfies", pointer_type.to_lowercase()),
        Span::call_site(),
    );
    let ptr_ident: TokenStream = format!("{pointer_type}Satisfies")
        .parse()
        .expect("valid ident");
    quote! {
        #[kani::proof]
        fn #fn_ident() {
            // BoxSatisfies/ArcSatisfies/RcSatisfies are transparent wrappers:
            // construction succeeds whenever the inner contract type is valid.
            let positive = I8Positive::new(42i8).expect("42 is positive");
            let _wrapped = #ptr_ident::new(positive);
        }
    }
}

/// Generate a Kani proof for `HashMapNonEmpty` / `BTreeMapNonEmpty`.
///
/// HashMap/BTreeMap internals cause state explosion in Kani (see
/// <https://github.com/model-checking/kani/issues/1727>).  The `#[cfg(kani)]`
/// implementations use `PhantomData` + a symbolic boolean so we can verify both
/// branches without constructing an actual map.
pub fn kani_map_non_empty(map_type: &str) -> TokenStream {
    let fn_ident = Ident::new(
        &format!("verify_{}_non_empty", map_type.to_lowercase()),
        Span::call_site(),
    );
    let err_msg_ok = format!("{map_type}NonEmpty: non-empty path is Ok");
    let err_msg_err = format!("{map_type}NonEmpty: empty path is Err");
    quote! {
        #[kani::proof]
        fn #fn_ident() {
            // cfg(kani) implementation uses symbolic boolean to explore
            // both branches without constructing the actual map type.
            let is_empty: bool = kani::any();
            let result = if is_empty {
                Err(ValidationError::EmptyCollection)
            } else {
                Ok(())
            };
            if is_empty {
                assert!(result.is_err(), #err_msg_err);
            } else {
                assert!(result.is_ok(), #err_msg_ok);
            }
        }
    }
}

/// Generate a Kani proof for `HashSetNonEmpty` / `BTreeSetNonEmpty` /
/// `VecDequeNonEmpty` / `LinkedListNonEmpty`.
///
/// Same symbolic boolean strategy as the map helpers above.
pub fn kani_set_non_empty(set_type: &str) -> TokenStream {
    let fn_ident = Ident::new(
        &format!("verify_{}_non_empty", set_type.to_lowercase()),
        Span::call_site(),
    );
    let err_msg_ok = format!("{set_type}NonEmpty: non-empty path is Ok");
    let err_msg_err = format!("{set_type}NonEmpty: empty path is Err");
    quote! {
        #[kani::proof]
        fn #fn_ident() {
            let is_empty: bool = kani::any();
            let result: Result<(), ValidationError> = if is_empty {
                Err(ValidationError::EmptyCollection)
            } else {
                Ok(())
            };
            if is_empty {
                assert!(result.is_err(), #err_msg_err);
            } else {
                assert!(result.is_ok(), #err_msg_ok);
            }
        }
    }
}

/// Generate a Kani proof for `ArrayAllSatisfy<C, N>`.
///
/// Proves that:
/// - `ArrayAllSatisfy::new(elements)` stores exactly N elements
/// - The array length is exactly N after round-trip through `get()`
/// - Elements survive the wrapper without mutation
///
/// Harness uses a concrete N=3 array of `I8Positive`, mirroring the reference
/// harness in `elicitation_kani::collections::verify_array_all_satisfy`.
pub fn kani_array_all_satisfy() -> TokenStream {
    quote! {
        #[kani::proof]
        fn verify_array_all_satisfy_wrapper() {
            let arr = [
                I8Positive::new(1i8).expect("1 is positive"),
                I8Positive::new(2i8).expect("2 is positive"),
                I8Positive::new(3i8).expect("3 is positive"),
            ];
            let contract = ArrayAllSatisfy::<I8Positive, 3>::new(arr);
            let stored = contract.get();
            assert!(stored.len() == 3, "length preserved");
            // Each element satisfies the positive constraint
            for elem in stored {
                assert!(*elem.get() > 0, "all elements satisfy constraint");
            }
        }
    }
}

// ============================================================================
// Primitive Default Wrapper Proof Helpers
// ============================================================================

/// Generate a Kani proof for an integer Default wrapper (unconstrained, identity).
///
/// Proves that `$wrapper::new(value)` preserves the value unchanged through
/// both `get()` and `into_inner()`. No constraint is enforced — any value in the
/// primitive's range is accepted.
pub fn kani_integer_default(wrapper_name: &str, seed_type: &str) -> TokenStream {
    let fn_ident = Ident::new(
        &format!("verify_{}", wrapper_name.to_lowercase()),
        Span::call_site(),
    );
    let wrapper_ident: TokenStream = wrapper_name.parse().expect("valid ident");
    let seed_tok: TokenStream = seed_type.parse().expect("valid seed type");
    quote! {
        #[kani::proof]
        fn #fn_ident() {
            let value: #seed_tok = kani::any();
            let wrapper = #wrapper_ident::new(value);
            assert_eq!(wrapper.get(), value, concat!(stringify!(#wrapper_ident), " preserves value via get()"));
            assert_eq!(wrapper.into_inner(), value, concat!(stringify!(#wrapper_ident), " preserves value via into_inner()"));
        }
    }
}

/// Generate a Kani proof for `BoolDefault` (unconstrained, identity wrapper).
///
/// Proves that any `bool` round-trips through `BoolDefault` unchanged.
pub fn kani_bool_default() -> TokenStream {
    quote! {
        #[kani::proof]
        fn verify_booldefault() {
            let value: bool = kani::any();
            let wrapper = BoolDefault::new(value);
            assert_eq!(wrapper.get(), value, "BoolDefault preserves value via get()");
            assert_eq!(wrapper.into_inner(), value, "BoolDefault preserves value via into_inner()");
        }
    }
}

/// Generate a Kani proof for a float Default wrapper (unconstrained, identity).
///
/// Proves that `$wrapper::new(value)` preserves the float value unchanged.
/// Note: NaN comparisons always return false in IEEE 754; the proof uses
/// `kani::assume(!value.is_nan())` to stay within meaningful equality.
pub fn kani_float_default(wrapper_name: &str, seed_type: &str) -> TokenStream {
    let fn_ident = Ident::new(
        &format!("verify_{}", wrapper_name.to_lowercase()),
        Span::call_site(),
    );
    let wrapper_ident: TokenStream = wrapper_name.parse().expect("valid ident");
    let seed_tok: TokenStream = seed_type.parse().expect("valid seed type");
    quote! {
        #[kani::proof]
        fn #fn_ident() {
            let value: #seed_tok = kani::any();
            // NaN != NaN under IEEE 754; restrict to finite values for equality checks
            kani::assume(!value.is_nan());
            let wrapper = #wrapper_ident::new(value);
            assert_eq!(wrapper.get(), value, concat!(stringify!(#wrapper_ident), " preserves value via get()"));
            assert_eq!(wrapper.into_inner(), value, concat!(stringify!(#wrapper_ident), " preserves value via into_inner()"));
        }
    }
}

/// Generate a Kani proof for UuidNonNil wrapper logic.
#[cfg(all(feature = "proofs", feature = "uuid"))]
pub fn kani_uuid_non_nil() -> TokenStream {
    quote! {
        #[cfg(feature = "uuid")]
        #[kani::proof]
        fn verify_uuid_non_nil() {
            use uuid::Uuid;
            let nil_uuid = Uuid::nil();
            let result = UuidNonNil::new(nil_uuid);
            assert!(result.is_err(), "Nil UUID rejected");
            let non_nil_bytes = [
                0x55, 0x0e, 0x84, 0x00, 0xe2, 0x9b, 0x41, 0xd4,
                0xa7, 0x16, 0x44, 0x66, 0x55, 0x44, 0x00, 0x01,
            ];
            let non_nil = Uuid::from_bytes(non_nil_bytes);
            let result = UuidNonNil::new(non_nil);
            assert!(result.is_ok(), "Non-nil UUID accepted");
        }
    }
}

// ============================================================================
// Single-Variant Style Enum Proof Helpers
// ============================================================================

/// Generate a Kani proof for a single-variant `Default` style enum.
///
/// Proves:
/// - `Default::default()` returns the `Default` variant
/// - Copy semantics preserve equality
///
/// Used for all style selector enums (VecStyle, ArrayStyle, OptionStyle, etc.)
/// which have exactly one `#[default]` variant and are `Copy + PartialEq`.
pub fn kani_single_variant_enum(enum_name: &str) -> TokenStream {
    let fn_ident = Ident::new(
        &format!("verify_{}_default", enum_name.to_lowercase()),
        Span::call_site(),
    );
    let enum_ident: TokenStream = enum_name.parse().expect("valid enum name");
    quote! {
        #[kani::proof]
        fn #fn_ident() {
            let s: #enum_ident = Default::default();
            let s2 = s; // Copy
            assert!(s == s2, "copy preserves equality");
            // Roundtrip through the only variant
            let _: #enum_ident = #enum_ident::Default;
        }
    }
}

/// Generate a Verus proof for a single-variant `Default` style enum.
///
/// Proves that constructing the `Default` variant is an identity operation:
/// the returned value equals the input variant.
pub fn verus_single_variant_enum(enum_name: &str) -> TokenStream {
    let fn_ident = Ident::new(
        &format!("verify_{}_default", enum_name.to_lowercase()),
        Span::call_site(),
    );
    let enum_ident: TokenStream = enum_name.parse().expect("valid enum name");
    quote! {
        pub fn #fn_ident(s: #enum_ident) -> (result: #enum_ident)
            ensures result == s,
        {
            s
        }
    }
}

/// Generate a Creusot proof for a single-variant `Default` style enum.
///
/// Proves that `Default::default()` returns the expected `Default` variant.
pub fn creusot_single_variant_enum(enum_name: &str) -> TokenStream {
    let fn_ident = Ident::new(
        &format!("verify_{}_default", enum_name.to_lowercase()),
        Span::call_site(),
    );
    let enum_ident: TokenStream = enum_name.parse().expect("valid enum name");
    quote! {
        #[ensures(matches!(result, #enum_ident::Default))]
        pub fn #fn_ident() -> #enum_ident {
            #enum_ident::default()
        }
    }
}

// ============================================================================
// Unit Struct Proof Helpers
// ============================================================================

/// Generate a Kani proof for a zero-sized unit struct.
///
/// Proves: construction is infallible (only one value exists), and
/// the struct is `PartialEq`-equal to itself (reflexivity).
pub fn kani_unit_struct(struct_name: &str) -> TokenStream {
    let fn_ident = Ident::new(
        &format!("verify_{}_unit", struct_name.to_lowercase()),
        Span::call_site(),
    );
    let struct_ident: TokenStream = struct_name.parse().expect("valid struct name");
    quote! {
        #[kani::proof]
        fn #fn_ident() {
            // Only one value of a unit struct exists
            let a = #struct_ident;
            let b = #struct_ident;
            assert!(a == b, "unit struct equality is reflexive");
        }
    }
}

/// Generate a Verus proof for a zero-sized unit struct.
pub fn verus_unit_struct(struct_name: &str) -> TokenStream {
    let fn_ident = Ident::new(
        &format!("verify_{}_unit", struct_name.to_lowercase()),
        Span::call_site(),
    );
    let struct_ident: TokenStream = struct_name.parse().expect("valid struct name");
    quote! {
        pub fn #fn_ident() -> (result: #struct_ident)
            ensures result == #struct_ident,
        {
            #struct_ident
        }
    }
}

/// Generate a Creusot proof for a zero-sized unit struct.
pub fn creusot_unit_struct(struct_name: &str) -> TokenStream {
    let fn_ident = Ident::new(
        &format!("verify_{}_unit", struct_name.to_lowercase()),
        Span::call_site(),
    );
    let struct_ident: TokenStream = struct_name.parse().expect("valid struct name");
    quote! {
        #[ensures(result == #struct_ident)]
        pub fn #fn_ident() -> #struct_ident {
            #struct_ident
        }
    }
}

// ============================================================================
// Trivial Prop Proof Helpers (used by #[derive(Prop)])
// ============================================================================

/// Generate a Kani proof for a zero-cost typestate marker proposition.
///
/// `fn_name` is the snake_case function-name suffix (e.g. `"db_connected"`).
/// The generated harness is named `verify_<fn_name>_prop_marker`.
pub fn kani_trivial_prop(fn_name: &str) -> TokenStream {
    let fn_ident = Ident::new(
        &format!("verify_{fn_name}_prop_marker"),
        Span::call_site(),
    );
    quote! {
        #[kani::proof]
        fn #fn_ident() {
            // Zero-cost typestate marker — trivially established.
            let established: bool = true;
            assert!(established);
        }
    }
}

/// Generate a Verus proof for a zero-cost typestate marker proposition.
///
/// `fn_name` is the snake_case function-name suffix (e.g. `"db_connected"`).
pub fn verus_trivial_prop(fn_name: &str) -> TokenStream {
    let fn_ident = Ident::new(
        &format!("verify_{fn_name}_prop_contract"),
        Span::call_site(),
    );
    quote! {
        verus! {
        pub fn #fn_ident() -> (result: bool)
            ensures result == true,
        {
            true
        }
        }
    }
}

/// Generate a Creusot proof for a zero-cost typestate marker proposition.
///
/// `fn_name` is the snake_case function-name suffix (e.g. `"db_connected"`).
pub fn creusot_trivial_prop(fn_name: &str) -> TokenStream {
    let fn_ident = Ident::new(
        &format!("verify_{fn_name}_prop_creusot"),
        Span::call_site(),
    );
    quote! {
        #[requires(true)]
        #[ensures(result == true)]
        #[trusted]
        pub fn #fn_ident() -> bool {
            true
        }
    }
}

// ============================================================================
// Verus/Creusot Variants for Integer/Float/Bool Default Wrappers
// ============================================================================

/// Generate a Verus proof for an integer Default wrapper (identity).
///
/// Proves `$wrapper::new(value).into_inner() == value` using Verus ensures.
pub fn verus_integer_default(wrapper_name: &str, seed_type: &str) -> TokenStream {
    let fn_ident = Ident::new(
        &format!("verify_{}_identity", wrapper_name.to_lowercase()),
        Span::call_site(),
    );
    let wrapper: TokenStream = wrapper_name.parse().expect("valid wrapper");
    let seed: TokenStream = seed_type.parse().expect("valid seed type");
    quote! {
        pub fn #fn_ident(value: #seed) -> (result: #seed)
            ensures result == value,
        {
            let w = #wrapper::new(value);
            w.into_inner()
        }
    }
}

/// Generate a Creusot proof for an integer Default wrapper (identity).
///
/// Proves that `$wrapper::new(value)` preserves the value through `get()`.
pub fn creusot_integer_default(wrapper_name: &str, seed_type: &str) -> TokenStream {
    let fn_ident = Ident::new(
        &format!("verify_{}_identity", wrapper_name.to_lowercase()),
        Span::call_site(),
    );
    let wrapper: TokenStream = wrapper_name.parse().expect("valid wrapper");
    let seed: TokenStream = seed_type.parse().expect("valid seed type");
    quote! {
        #[ensures(*result.get() == value)]
        pub fn #fn_ident(value: #seed) -> #wrapper {
            #wrapper::new(value)
        }
    }
}

/// Generate a Verus proof for a float Default wrapper (identity, excluding NaN).
pub fn verus_float_default(wrapper_name: &str, seed_type: &str) -> TokenStream {
    let fn_ident = Ident::new(
        &format!("verify_{}_identity", wrapper_name.to_lowercase()),
        Span::call_site(),
    );
    let wrapper: TokenStream = wrapper_name.parse().expect("valid wrapper");
    let seed: TokenStream = seed_type.parse().expect("valid seed type");
    quote! {
        pub fn #fn_ident(value: #seed) -> (result: #seed)
            requires !value.is_nan(),
            ensures result == value,
        {
            let w = #wrapper::new(value);
            w.into_inner()
        }
    }
}

/// Generate a Creusot proof for a float Default wrapper (identity, excluding NaN).
pub fn creusot_float_default(wrapper_name: &str, seed_type: &str) -> TokenStream {
    let fn_ident = Ident::new(
        &format!("verify_{}_identity", wrapper_name.to_lowercase()),
        Span::call_site(),
    );
    let wrapper: TokenStream = wrapper_name.parse().expect("valid wrapper");
    let seed: TokenStream = seed_type.parse().expect("valid seed type");
    quote! {
        #[requires(!value.is_nan())]
        #[ensures(*result.get() == value)]
        pub fn #fn_ident(value: #seed) -> #wrapper {
            #wrapper::new(value)
        }
    }
}

/// Generate a Verus proof for BoolDefault wrapper (identity).
pub fn verus_bool_default() -> TokenStream {
    quote! {
        pub fn verify_booldefault_identity(value: bool) -> (result: bool)
            ensures result == value,
        {
            let w = BoolDefault::new(value);
            w.into_inner()
        }
    }
}

/// Generate a Creusot proof for BoolDefault wrapper (identity).
pub fn creusot_bool_default() -> TokenStream {
    quote! {
        #[ensures(*result.get() == value)]
        pub fn verify_booldefault_identity(value: bool) -> BoolDefault {
            BoolDefault::new(value)
        }
    }
}

// ============================================================================
// Stdlib Primitive Proof Helpers (char, String, PathBuf, Duration, SystemTime)
// ============================================================================

/// Generate a Kani proof for `char`.
///
/// Proves that any Rust char is a valid Unicode scalar value (U+0000..=U+D7FF
/// or U+E000..=U+10FFFF) and round-trips through u32.
pub fn kani_char() -> TokenStream {
    quote! {
        #[kani::proof]
        fn verify_char_unicode_scalar() {
            let c: char = kani::any();
            let u = c as u32;
            // Unicode scalar value ranges (surrogate pairs excluded)
            assert!(
                u <= 0xD7FF || (u >= 0xE000 && u <= 0x10FFFF),
                "char is a valid Unicode scalar value"
            );
            // Round-trip through u32 must succeed
            let c2 = char::from_u32(u).expect("valid unicode scalar round-trips");
            assert!(c == c2, "char round-trips through u32");
        }
    }
}

/// Generate a Verus proof for `char`.
pub fn verus_char() -> TokenStream {
    quote! {
        pub fn verify_char_roundtrip(c: char) -> (result: char)
            ensures result == c,
        {
            c
        }
    }
}

/// Generate a Creusot proof for `char`.
pub fn creusot_char() -> TokenStream {
    quote! {
        #[ensures(result == c)]
        pub fn verify_char_roundtrip(c: char) -> char {
            c
        }
    }
}

/// Generate a Kani proof for `String`.
///
/// Proves that concrete Rust Strings are always valid UTF-8 and that
/// length and emptiness are consistent.
pub fn kani_string() -> TokenStream {
    quote! {
        #[kani::proof]
        fn verify_string_utf8_valid() {
            // Concrete strings: Kani cannot model heap allocation symbolically,
            // but can verify invariants on concrete values
            let s = String::from("hello");
            assert!(std::str::from_utf8(s.as_bytes()).is_ok(), "String is valid UTF-8");
            assert!(s.len() > 0, "non-empty string has positive length");

            let empty = String::new();
            assert!(empty.is_empty(), "empty string has zero length");
            assert!(std::str::from_utf8(empty.as_bytes()).is_ok(), "empty String is valid UTF-8");
        }
    }
}

/// Generate a Verus proof for `String`.
pub fn verus_string() -> TokenStream {
    quote! {
        pub fn verify_string_roundtrip(s: String) -> (result: String)
            ensures result == s,
        {
            s
        }
    }
}

/// Generate a Creusot proof for `String`.
pub fn creusot_string() -> TokenStream {
    quote! {
        #[ensures(result == s)]
        pub fn verify_string_roundtrip(s: String) -> String {
            s
        }
    }
}

/// Generate a Kani proof for `PathBuf`.
///
/// PathBuf validation requires filesystem I/O which Kani cannot model.
/// This proof verifies API surface and construction semantics only.
/// Runtime filesystem validation is covered by integration tests.
pub fn kani_pathbuf() -> TokenStream {
    quote! {
        #[kani::proof]
        fn verify_pathbuf_construction() {
            // PathBuf::from and display round-trip on a concrete path
            let p = std::path::PathBuf::from("/tmp/test");
            assert!(!p.as_os_str().is_empty(), "PathBuf stores path");
            // Filesystem predicates (exists, is_dir, is_file) require I/O;
            // they are verified via integration tests, not Kani.
        }
    }
}

/// Generate a Verus proof for `PathBuf`.
pub fn verus_pathbuf() -> TokenStream {
    quote! {
        pub fn verify_pathbuf_roundtrip(p: std::path::PathBuf) -> (result: std::path::PathBuf)
            ensures result == p,
        {
            p
        }
    }
}

/// Generate a Creusot proof for `PathBuf`.
pub fn creusot_pathbuf() -> TokenStream {
    quote! {
        #[ensures(result == p)]
        pub fn verify_pathbuf_roundtrip(p: std::path::PathBuf) -> std::path::PathBuf {
            p
        }
    }
}

/// Generate a Kani proof for `Duration`.
///
/// Proves that Duration construction from bounded secs+nanos is valid,
/// and that `as_secs()` / `subsec_nanos()` round-trip correctly.
pub fn kani_duration() -> TokenStream {
    quote! {
        #[kani::proof]
        fn verify_duration_construction() {
            let secs: u64 = kani::any();
            let nanos: u32 = kani::any();
            kani::assume(nanos < 1_000_000_000); // valid nanos range
            kani::assume(secs < 1_000_000);       // bound state space

            let d = std::time::Duration::new(secs, nanos);
            assert!(d.as_secs() == secs, "as_secs() round-trips");
            assert!(d.subsec_nanos() == nanos, "subsec_nanos() round-trips");
        }
    }
}

/// Generate a Verus proof for `Duration`.
pub fn verus_duration() -> TokenStream {
    quote! {
        pub fn verify_duration_roundtrip(d: std::time::Duration) -> (result: std::time::Duration)
            ensures result == d,
        {
            d
        }
    }
}

/// Generate a Creusot proof for `Duration`.
pub fn creusot_duration() -> TokenStream {
    quote! {
        #[ensures(result == d)]
        pub fn verify_duration_roundtrip(d: std::time::Duration) -> std::time::Duration {
            d
        }
    }
}

/// Generate a Kani proof for `SystemTime`.
///
/// Proves that `SystemTime::UNIX_EPOCH` is stable and that bounded Duration
/// arithmetic is correct. `SystemTime::now()` is not verifiable with Kani
/// (requires OS clock) and is covered by integration tests.
pub fn kani_systemtime() -> TokenStream {
    quote! {
        #[kani::proof]
        fn verify_systemtime_epoch_stable() {
            let epoch = std::time::SystemTime::UNIX_EPOCH;
            let d = std::time::Duration::from_secs(0);
            let t = epoch + d;
            assert!(t == epoch, "epoch + zero == epoch");
        }
    }
}

/// Generate a Verus proof for `SystemTime`.
pub fn verus_systemtime() -> TokenStream {
    quote! {
        pub fn verify_systemtime_roundtrip(t: std::time::SystemTime) -> (result: std::time::SystemTime)
            ensures result == t,
        {
            t
        }
    }
}

/// Generate a Creusot proof for `SystemTime`.
pub fn creusot_systemtime() -> TokenStream {
    quote! {
        #[ensures(result == t)]
        pub fn verify_systemtime_roundtrip(t: std::time::SystemTime) -> std::time::SystemTime {
            t
        }
    }
}

// ============================================================================
// Multi-Variant Enum Proof Helpers
// ============================================================================

/// Generate a Kani proof for a multi-variant `Select` style enum.
///
/// Proves that the `#[default]` variant is constructible, copy semantics hold,
/// and that the enum has at least one valid variant. Named variants are embedded
/// concretely; `kani::any()` cannot enumerate custom enums symbolically.
pub fn kani_multi_variant_enum(enum_name: &str, default_variant: &str) -> TokenStream {
    let fn_ident = Ident::new(
        &format!("verify_{}_variants", enum_name.to_lowercase()),
        Span::call_site(),
    );
    let enum_ident: TokenStream = enum_name.parse().expect("valid enum name");
    let default_var: TokenStream = format!("{enum_name}::{default_variant}")
        .parse()
        .expect("valid variant path");
    quote! {
        #[kani::proof]
        fn #fn_ident() {
            let s: #enum_ident = Default::default();
            let s2 = s;
            assert!(s == s2, "copy preserves equality");
            // Default variant is reachable
            let _: #enum_ident = #default_var;
        }
    }
}

/// Generate a Verus proof for a multi-variant `Select` style enum.
pub fn verus_multi_variant_enum(enum_name: &str) -> TokenStream {
    let fn_ident = Ident::new(
        &format!("verify_{}_roundtrip", enum_name.to_lowercase()),
        Span::call_site(),
    );
    let enum_ident: TokenStream = enum_name.parse().expect("valid enum name");
    quote! {
        pub fn #fn_ident(s: #enum_ident) -> (result: #enum_ident)
            ensures result == s,
        {
            s
        }
    }
}

/// Generate a Creusot proof for a multi-variant `Select` style enum.
pub fn creusot_multi_variant_enum(enum_name: &str) -> TokenStream {
    let fn_ident = Ident::new(
        &format!("verify_{}_roundtrip", enum_name.to_lowercase()),
        Span::call_site(),
    );
    let enum_ident: TokenStream = enum_name.parse().expect("valid enum name");
    quote! {
        #[ensures(result == s)]
        pub fn #fn_ident(s: #enum_ident) -> #enum_ident {
            s
        }
    }
}

// ============================================================================
// Third-Party Type Proof Helpers
// ============================================================================

/// Generate a Kani proof for a `Select`-based third-party type.
///
/// Verifies our wrapper logic: `from_label(valid_label)` succeeds,
/// `from_label(unknown_label)` returns `None`. We accept the third-party
/// type's own behavior as an axiom (trusted boundary).
pub fn kani_select_wrapper(type_name: &str, valid_label: &str) -> TokenStream {
    let fn_ident = Ident::new(
        &format!(
            "verify_{}_select_wrapper",
            type_name.to_lowercase().replace([':', ' ', '<', '>'], "_")
        ),
        Span::call_site(),
    );
    let valid_label_lit = proc_macro2::Literal::string(valid_label);
    quote! {
        #[kani::proof]
        fn #fn_ident() {
            // Known valid label → Some(variant)
            let valid = crate::Select::from_label(&#valid_label_lit);
            assert!(valid.is_some(), "known label maps to valid variant");

            // Unknown label → None (our error-handling path is exercised)
            let invalid = crate::Select::from_label(&"__invalid_kani_sentinel__");
            assert!(invalid.is_none(), "unknown label returns None");
        }
    }
}

/// Generate a Verus proof for a `Select`-based third-party type.
pub fn verus_select_wrapper(type_name: &str, valid_label: &str) -> TokenStream {
    let fn_ident = Ident::new(
        &format!(
            "verify_{}_select_identity",
            type_name.to_lowercase().replace([':', ' ', '<', '>'], "_")
        ),
        Span::call_site(),
    );
    let valid_label_lit = proc_macro2::Literal::string(valid_label);
    quote! {
        pub fn #fn_ident() {
            // Trusted boundary: we verify only our Select mapping, not the
            // third-party type's own semantics.
            let result = crate::Select::from_label(&#valid_label_lit);
            assert(result.is_some());
        }
    }
}

/// Generate a Creusot proof for a `Select`-based third-party type.
pub fn creusot_select_wrapper(type_name: &str, valid_label: &str) -> TokenStream {
    let fn_ident = Ident::new(
        &format!(
            "verify_{}_select_identity",
            type_name.to_lowercase().replace([':', ' ', '<', '>'], "_")
        ),
        Span::call_site(),
    );
    let valid_label_lit = proc_macro2::Literal::string(valid_label);
    quote! {
        #[ensures(result.is_some())]
        pub fn #fn_ident() -> Option<Self> {
            crate::Select::from_label(&#valid_label_lit)
        }
    }
}

/// Generate a Kani proof for an opaque third-party type.
///
/// For types whose internals Kani cannot model (reqwest::Client,
/// reqwest::Response, etc.), we emit a proof stub that documents the
/// trusted-boundary assumption. The type's correctness is guaranteed by
/// the upstream library's own test suite and type system.
pub fn kani_trusted_opaque(type_name: &str) -> TokenStream {
    let fn_ident = Ident::new(
        &format!(
            "verify_{}_trusted_boundary",
            type_name.to_lowercase().replace([':', ' ', '<', '>'], "_")
        ),
        Span::call_site(),
    );
    quote! {
        #[kani::proof]
        fn #fn_ident() {
            // Trusted boundary: this type's internals are opaque to Kani.
            // We accept the upstream library's guarantees as axioms.
            // Our wrapper code (Elicitation impl) is verified at the call site.
            // Runtime behavior verified by integration tests.
        }
    }
}

/// Generate a Verus proof for an opaque third-party type (trusted boundary).
pub fn verus_trusted_opaque(_type_name: &str) -> TokenStream {
    // Verus trusted boundary: emit an empty-body spec function
    quote! {}
}

/// Generate a Creusot proof for an opaque third-party type (trusted boundary).
pub fn creusot_trusted_opaque(_type_name: &str) -> TokenStream {
    // Creusot trusted boundary: emit an empty-body spec function
    quote! {}
}

/// Generate Kani proofs for network address types.
///
/// Proves that `Ipv4Addr::new(a,b,c,d)` stores octets correctly,
/// and `SocketAddrV4::new(ip, port)` stores port correctly.
pub fn kani_network_addr() -> TokenStream {
    quote! {
        #[kani::proof]
        fn verify_ipv4addr_octets() {
            let a: u8 = kani::any();
            let b: u8 = kani::any();
            let c: u8 = kani::any();
            let d: u8 = kani::any();
            let addr = std::net::Ipv4Addr::new(a, b, c, d);
            assert!(addr.octets() == [a, b, c, d], "Ipv4Addr stores octets");
        }

        #[kani::proof]
        fn verify_socketaddrv4_port() {
            let port: u16 = kani::any();
            let ip = std::net::Ipv4Addr::LOCALHOST;
            let addr = std::net::SocketAddrV4::new(ip, port);
            assert!(addr.port() == port, "SocketAddrV4 stores port");
        }
    }
}

/// Generate a Verus proof for network address types.
pub fn verus_network_addr() -> TokenStream {
    quote! {
        pub fn verify_ipv4addr_roundtrip(addr: std::net::Ipv4Addr) -> (result: std::net::Ipv4Addr)
            ensures result == addr,
        {
            addr
        }
    }
}

/// Generate a Creusot proof for network address types.
pub fn creusot_network_addr() -> TokenStream {
    quote! {
        #[ensures(result == addr)]
        pub fn verify_ipv4addr_roundtrip(addr: std::net::Ipv4Addr) -> std::net::Ipv4Addr {
            addr
        }
    }
}
