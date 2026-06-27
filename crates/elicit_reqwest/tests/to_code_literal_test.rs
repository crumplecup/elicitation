//! Goal 2: `ToCodeLiteral` round-trip tests.
//!
//! Verifies that `to_code_literal()` emits code that is semantically identical
//! to what you would write by hand. Both sides are parsed into token streams
//! and normalized with `prettyplease` before comparison, so whitespace and
//! trivial formatting differences do not cause false failures.

use elicit_reqwest::{Body, Method, NoProxy, Policy, Proxy, Request, StatusCode, Version};
use elicitation::emit_code::ToCodeLiteral;
use elicitation::proc_macro2::TokenStream;

/// Normalize a token stream to a canonical string via `prettyplease`.
///
/// Wraps the stream in `fn _() { let _ = <ts>; }` so `syn` can parse any
/// expression, then unparses with `prettyplease` for deterministic output.
fn normalize(ts: TokenStream) -> String {
    let file: syn::File = syn::parse2(quote::quote! {
        fn __check__() { let __v__ = #ts; }
    })
    .expect("token stream should be a valid expression");
    prettyplease::unparse(&file)
}

/// Assert that the emitted token stream normalizes to the same code as the
/// hand-written `expected` token stream.
macro_rules! assert_code_eq {
    ($value:expr, $expected:expr) => {{
        let emitted = normalize($value.to_code_literal());
        let expected = normalize($expected);
        assert_eq!(
            emitted, expected,
            "\nEmitted:\n{emitted}\nExpected:\n{expected}"
        );
    }};
}

// ── Method ────────────────────────────────────────────────────────────────────

#[test]
fn method_get_round_trips() {
    let m = Method::from(reqwest::Method::GET);
    assert_code_eq!(
        m,
        quote::quote! {
            ::elicit_reqwest::Method::from(
                match ::reqwest::Method::from_bytes("GET".as_bytes()) {
                    ::std::result::Result::Ok(m) => m,
                    ::std::result::Result::Err(error) => {
                        return ::std::result::Result::Err(error.into());
                    }
                }
            )
        }
    );
}

#[test]
fn method_post_round_trips() {
    let m = Method::from(reqwest::Method::POST);
    assert_code_eq!(
        m,
        quote::quote! {
            ::elicit_reqwest::Method::from(
                match ::reqwest::Method::from_bytes("POST".as_bytes()) {
                    ::std::result::Result::Ok(m) => m,
                    ::std::result::Result::Err(error) => {
                        return ::std::result::Result::Err(error.into());
                    }
                }
            )
        }
    );
}

// ── StatusCode ────────────────────────────────────────────────────────────────

#[test]
fn status_code_200_round_trips() {
    let s = StatusCode::from(reqwest::StatusCode::OK);
    assert_code_eq!(
        s,
        quote::quote! {
            match ::elicit_reqwest::StatusCode::from_u16(200u16) {
                ::std::result::Result::Ok(s) => s,
                ::std::result::Result::Err(error) => {
                    return ::std::result::Result::Err(error.into());
                }
            }
        }
    );
}

#[test]
fn status_code_404_round_trips() {
    let s = StatusCode::from(reqwest::StatusCode::NOT_FOUND);
    assert_code_eq!(
        s,
        quote::quote! {
            match ::elicit_reqwest::StatusCode::from_u16(404u16) {
                ::std::result::Result::Ok(s) => s,
                ::std::result::Result::Err(error) => {
                    return ::std::result::Result::Err(error.into());
                }
            }
        }
    );
}

// ── Version ───────────────────────────────────────────────────────────────────

#[test]
fn version_http11_round_trips() {
    let v = Version::from(reqwest::Version::HTTP_11);
    assert_code_eq!(
        v,
        quote::quote! {
            ::elicit_reqwest::Version::from(::reqwest::Version::HTTP_11)
        }
    );
}

#[test]
fn version_http2_round_trips() {
    let v = Version::from(reqwest::Version::HTTP_2);
    assert_code_eq!(
        v,
        quote::quote! {
            ::elicit_reqwest::Version::from(::reqwest::Version::HTTP_2)
        }
    );
}

// ── Body ──────────────────────────────────────────────────────────────────────

#[test]
fn body_empty_round_trips() {
    let b = Body::from_bytes(vec![]);
    assert_code_eq!(
        b,
        quote::quote! {
            ::elicit_reqwest::Body::from_bytes(::std::vec![])
        }
    );
}

#[test]
fn body_with_bytes_round_trips() {
    let b = Body::from_bytes(vec![1u8, 2, 3]);
    assert_code_eq!(
        b,
        quote::quote! {
            ::elicit_reqwest::Body::from_bytes(::std::vec![1u8, 2u8, 3u8])
        }
    );
}

// ── NoProxy ───────────────────────────────────────────────────────────────────

#[test]
fn no_proxy_round_trips() {
    let np = NoProxy::from_string("localhost,127.0.0.1");
    assert_code_eq!(
        np,
        quote::quote! {
            ::elicit_reqwest::NoProxy::from_string("localhost,127.0.0.1".to_string())
        }
    );
}

// ── Proxy ─────────────────────────────────────────────────────────────────────

#[test]
fn proxy_http_round_trips() {
    let url: elicit_reqwest::Url = "http://proxy.example.com:8080".parse().unwrap();
    let proxy = Proxy::http(url).expect("valid proxy");
    assert_code_eq!(
        proxy,
        quote::quote! {
            (match ::elicit_reqwest::Proxy::http(
                ::elicit_url::Url::from(::url::Url::parse("http://proxy.example.com:8080/").expect("valid url"))
            ) {
                ::std::result::Result::Ok(proxy) => proxy,
                ::std::result::Result::Err(error) => {
                    return ::std::result::Result::Err(error.into());
                }
            })
        }
    );
}

// ── Policy ────────────────────────────────────────────────────────────────────

#[test]
fn policy_limited_round_trips() {
    // Policy derives Elicit so ToCodeLiteral is auto-generated via #[to_code_literal]
    // We verify the type name and max value appear in the output.
    let p = Policy::limited(10);
    let emitted = p.to_code_literal().to_string();
    assert!(
        emitted.contains("Policy") || emitted.contains("Limited"),
        "expected Policy/Limited in emitted code, got: {emitted}"
    );
    assert!(
        emitted.contains("10"),
        "expected max=10 in emitted code, got: {emitted}"
    );
}

// ── Request ───────────────────────────────────────────────────────────────────

#[test]
fn request_round_trips() {
    let method = Method::from(reqwest::Method::GET);
    let url: elicit_reqwest::Url = "https://example.com/".parse().unwrap();
    let request = Request::new(method, url);
    // Verify the emitted code is parseable and contains key identifiers.
    let ts = request.to_code_literal();
    let emitted = normalize(ts);
    assert!(
        emitted.contains("Request"),
        "expected Request in emitted code"
    );
    assert!(
        emitted.contains("example.com"),
        "expected URL in emitted code"
    );
}
