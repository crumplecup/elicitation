# elicit_regex

Elicitation-enabled wrapper around [`regex::Regex`](https://docs.rs/regex).

## Why this crate

`regex::Regex` has no `Serialize`, `Deserialize`, or `JsonSchema` implementations.
This blocks its use as a field in `#[derive(Elicit)]` structs registered as MCP tools.

`elicit_regex` provides a `Regex` newtype that:

- Serializes to/from the pattern string (e.g. `"^\\d+$"`)
- Implements `JsonSchema` as `{ "type": "string" }`
- Exposes useful methods as MCP tools via `#[reflect_methods]`

## Usage

```rust
use elicit_regex::Regex;

// Compile a pattern
let r = Regex::new(r"\d+").unwrap();

// Test a string
assert!(r.is_match("abc123".to_string()));

// Find first match
assert_eq!(r.find("abc123".to_string()), Some("123".to_string()));

// Serde round-trip (pattern string)
let json = serde_json::to_string(&r).unwrap();
let r2: Regex = serde_json::from_str(&json).unwrap();
assert_eq!(r.as_str(), r2.as_str());
```

## MCP reflect methods

Exposes: `as_str`, `captures_len`, `is_match`, `find`, `find_all`, `replace_all`.
