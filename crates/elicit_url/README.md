# elicit_url

[Elicitation] shadow crate for [`url::Url`](https://docs.rs/url) — makes `Url` usable
in MCP tool registrations via `#[derive(Elicit)]` and exposes URL inspection and
manipulation as MCP tools.

## Why this crate

`url::Url` does not implement `JsonSchema`, which prevents it from being used directly
in `#[derive(Elicit)]` structs. This crate provides a transparent `Url` newtype that
satisfies the schema requirement while staying fully interoperable with the upstream crate.

## What it provides

| Feature | Details |
|---|---|
| `JsonSchema` | Emits `{ "type": "string", "format": "uri" }` |
| `Serialize`/`Deserialize` | Transparent — encodes/decodes identically to `url::Url` |
| `Deref`/`DerefMut` | All `url::Url` methods accessible without unwrapping |
| `From`/`Into` | Zero-cost conversion to/from `url::Url` |
| `#[reflect_methods]` | Inspect and manipulate URLs as MCP tool calls |

## MCP tools

`scheme`, `host`, `port`, `port_or_default`, `path`, `query`, `fragment`,
`username`, `has_authority`, `join`, `origin`, `as_str`.

## Usage

```toml
[dependencies]
elicit_url = "0.11"
```

```rust
use elicit_url::Url;

let u = Url::parse("https://api.example.com:8080/v2/users?limit=10#results").unwrap();
assert_eq!(u.scheme(), "https");
assert_eq!(u.host(), Some("api.example.com".to_string()));
assert_eq!(u.port(), Some(8080));
assert_eq!(u.path(), "/v2/users");

// Relative resolution
let endpoint = u.join("../v3/items".to_string()).unwrap();
```

## License

Licensed under either of [Apache License 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.

[Elicitation]: https://crates.io/crates/elicitation
