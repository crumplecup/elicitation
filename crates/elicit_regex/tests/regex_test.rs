//! Integration tests for `elicit_regex` types.

use elicit_regex::Regex;
use schemars::schema_for;
use serde_json::json;

#[test]
fn regex_serialize_as_pattern() {
    let r = Regex::new(r"^\d{3}-\d{4}$").unwrap();
    let json = serde_json::to_string(&r).unwrap();
    assert_eq!(json, r#""^\\d{3}-\\d{4}$""#);
}

#[test]
fn regex_deserialize_from_pattern() {
    let json = r#""hello\\s+world""#;
    let r: Regex = serde_json::from_str(json).unwrap();
    assert_eq!(r.as_str(), r"hello\s+world");
}

#[test]
fn regex_roundtrip() {
    let pattern = r"^[a-z]+\d*$";
    let r = Regex::new(pattern).unwrap();
    let json = serde_json::to_string(&r).unwrap();
    let r2: Regex = serde_json::from_str(&json).unwrap();
    assert_eq!(r.as_str(), r2.as_str());
}

#[test]
fn regex_invalid_pattern_deserialize_fails() {
    let json = r#""[unclosed bracket""#;
    let result = serde_json::from_str::<Regex>(json);
    assert!(result.is_err());
}

#[test]
fn regex_is_match() {
    let r = Regex::new(r"\d+").unwrap();
    assert!(r.is_match("abc123".to_string()));
    assert!(!r.is_match("abcdef".to_string()));
}

#[test]
fn regex_find() {
    let r = Regex::new(r"\d+").unwrap();
    assert_eq!(r.find("abc123def".to_string()), Some("123".to_string()));
    assert_eq!(r.find("nope".to_string()), None);
}

#[test]
fn regex_find_all() {
    let r = Regex::new(r"\d+").unwrap();
    let matches = r.find_all("1 plus 2 equals 3".to_string());
    assert_eq!(matches, vec!["1", "2", "3"]);
}

#[test]
fn regex_replace_all() {
    let r = Regex::new(r"\d+").unwrap();
    let result = r.replace_all("a1b2c3".to_string(), "X".to_string());
    assert_eq!(result, "aXbXcX");
}

#[test]
fn regex_captures_len() {
    let r = Regex::new(r"(\d+)-(\w+)").unwrap();
    assert_eq!(r.captures_len(), 2);
}

#[test]
fn regex_json_schema_is_string() {
    let schema = schema_for!(Regex);
    let value = serde_json::to_value(&schema).unwrap();
    assert_eq!(value["type"], json!("string"));
}
