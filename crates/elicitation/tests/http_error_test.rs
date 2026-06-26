#![cfg(feature = "reqwest")]

use elicitation::{ElicitIntrospect, emit_code::ToCodeLiteral, lookup_type_spec};
use std::str::FromStr;

#[test]
fn http_error_metadata_is_registered() {
    let metadata = <http::Error as ElicitIntrospect>::metadata();

    assert_eq!(metadata.type_name, "http::Error");
    assert!(metadata.description.is_some());
}

#[test]
fn http_error_spec_registered() {
    let spec = lookup_type_spec("http::Error").expect("http::Error registered");

    assert_eq!(spec.type_name(), "http::Error");
    assert!(
        spec.summary().contains("public error family"),
        "summary should describe the closed public variant family"
    );
    assert_eq!(spec.categories().len(), 2);
}

#[test]
fn status_code_http_error_code_literal_uses_status_constructor() {
    let error = http::StatusCode::from_u16(0)
        .expect_err("0 is not a valid HTTP status code")
        .into();

    let tokens = <http::Error as ToCodeLiteral>::to_code_literal(&error).to_string();

    assert!(tokens.contains("StatusCode :: from_u16"));
    assert!(tokens.contains("http :: Error :: from"));
}

#[test]
fn invalid_uri_parts_http_error_code_literal_uses_uri_parts_constructor() {
    let mut parts = http::uri::Parts::default();
    parts.scheme = Some(http::uri::Scheme::HTTP);
    let error = http::Uri::from_parts(parts)
        .expect_err("scheme without authority/path must fail")
        .into();

    let tokens = <http::Error as ToCodeLiteral>::to_code_literal(&error).to_string();

    assert!(tokens.contains("Uri :: from_parts"));
    assert!(tokens.contains("Parts :: default"));
}

#[test]
fn max_size_reached_http_error_code_literal_uses_header_map_capacity() {
    let error = http::HeaderMap::<()>::try_with_capacity(usize::MAX)
        .expect_err("usize::MAX capacity must exceed HeaderMap limits")
        .into();

    let tokens = <http::Error as ToCodeLiteral>::to_code_literal(&error).to_string();

    assert!(tokens.contains("HeaderMap :: <"));
    assert!(tokens.contains("try_with_capacity"));
}

#[test]
fn invalid_uri_http_error_code_literal_uses_uri_parser() {
    let error = http::Uri::from_str("http://[::1")
        .expect_err("malformed IPv6 URI must fail")
        .into();

    let tokens = <http::Error as ToCodeLiteral>::to_code_literal(&error).to_string();

    assert!(tokens.contains("FromStr"));
    assert!(tokens.contains("http :: Uri"));
}
