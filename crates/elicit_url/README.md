# elicit_url

Elicitation-enabled wrapper around [`url::Url`](https://docs.rs/url).

## MCP reflect methods

`scheme`, `host`, `port`, `port_or_default`, `path`, `query`, `fragment`,
`username`, `has_authority`, `join`, `origin`, `as_str`.

## Usage

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
