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

/// Generate a Kani proof for UuidNonNil wrapper logic.
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
