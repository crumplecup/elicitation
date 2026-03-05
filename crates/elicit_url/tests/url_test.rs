//! Integration tests for `elicit_url` types.

use elicit_url::Url;
use schemars::schema_for;
use serde_json::json;

#[test]
fn url_parse_and_reflect() {
    let u = Url::parse("https://example.com:8080/api/v1?key=val#section").unwrap();
    assert_eq!(u.scheme(), "https");
    assert_eq!(u.host(), Some("example.com".to_string()));
    assert_eq!(u.port(), Some(8080));
    assert_eq!(u.path(), "/api/v1");
    assert_eq!(u.query(), Some("key=val".to_string()));
    assert_eq!(u.fragment(), Some("section".to_string()));
}

#[test]
fn url_serde_roundtrip() {
    let u = Url::parse("https://api.example.com/v2/users").unwrap();
    let json = serde_json::to_string(&u).unwrap();
    let u2: Url = serde_json::from_str(&json).unwrap();
    assert_eq!(u.as_str(), u2.as_str());
}

#[test]
fn url_join() {
    let base = Url::parse("https://example.com/api/v1/").unwrap();
    let result = base.join("users".to_string()).unwrap();
    assert!(result.contains("users"));
}

#[test]
fn url_has_authority() {
    let u = Url::parse("https://example.com/path").unwrap();
    assert!(u.has_authority());
}

#[test]
fn url_origin() {
    let u = Url::parse("https://example.com:443/path").unwrap();
    assert!(u.origin().starts_with("https://example.com"));
}

#[test]
fn url_invalid_parse_returns_none() {
    assert!(Url::parse("not a url !!").is_none());
}

#[test]
fn url_json_schema_is_string() {
    let schema = schema_for!(Url);
    let value = serde_json::to_value(&schema).unwrap();
    assert_eq!(value["type"], json!("string"));
}

#[test]
fn url_port_or_default() {
    let u = Url::parse("https://example.com/path").unwrap();
    assert_eq!(u.port_or_default(), Some(443));
}
