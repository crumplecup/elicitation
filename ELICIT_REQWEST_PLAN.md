# elicit_reqwest Shadow Crate Implementation Plan

## Overview

Create a shadow crate that wraps the reqwest HTTP client library to demonstrate and test all elicitation macro capabilities, especially the new generic support.

## Research Summary

Based on [reqwest documentation](https://docs.rs/reqwest/latest/reqwest/):

- **Version**: 0.13.2 (latest)
- **Key Types**: Client, RequestBuilder, Response, Method, HeaderMap, Url
- **Architecture**: Async-first (requires tokio), connection pooling, builder pattern

## Crate Structure

```
crates/elicit_reqwest/
├── Cargo.toml
├── src/
│   ├── lib.rs                 # Main exports
│   ├── client.rs              # Client wrapper
│   ├── request_builder.rs     # RequestBuilder wrapper
│   ├── response.rs            # Response wrapper
│   ├── types.rs               # Re-exported types (Method, Url, etc.)
│   └── error.rs               # Error wrapper
└── tests/
    ├── integration_test.rs    # End-to-end tests
    ├── client_test.rs         # Client-specific tests
    └── macros_test.rs         # Macro expansion tests
```

## Type Wrapping Strategy

### 1. Client Wrapper (`client.rs`)

**Macro Strategy:**
- Use `elicit_newtype!(reqwest::Client, as Client)` for the wrapper
- Use `#[reflect_methods]` for ALL methods (all are generic over `U: IntoUrl`)

**Methods to Wrap (all generic):**
```rust
#[reflect_methods]
impl Client {
    // Generic over U: IntoUrl
    pub fn get<U: IntoUrl>(&self, url: U) -> RequestBuilder
    pub fn post<U: IntoUrl>(&self, url: U) -> RequestBuilder
    pub fn put<U: IntoUrl>(&self, url: U) -> RequestBuilder
    pub fn patch<U: IntoUrl>(&self, url: U) -> RequestBuilder
    pub fn delete<U: IntoUrl>(&self, url: U) -> RequestBuilder
    pub fn head<U: IntoUrl>(&self, url: U) -> RequestBuilder

    // Generic over method + url
    pub fn request<U: IntoUrl>(&self, method: Method, url: U) -> RequestBuilder

    // Note: execute() is async and takes pre-built Request
    pub async fn execute(&self, request: Request) -> Result<Response, Error>
}
```

**Rationale:**
- All HTTP convenience methods are generic over `IntoUrl`
- This perfectly tests our new generic method support in `#[reflect_methods]`
- Cannot use `elicit_newtype_methods!` because all methods are generic

### 2. RequestBuilder Wrapper (`request_builder.rs`)

**Macro Strategy:**
- Use `elicit_newtype!(reqwest::RequestBuilder, as RequestBuilder)` for wrapper
- **Mixed approach:**
  - Use `elicit_newtype_methods!` for non-generic methods
  - Use `#[reflect_methods]` for generic methods

**Non-Generic Methods (use `elicit_newtype_methods!`):**
```rust
elicit_newtype_methods! {
    RequestBuilder => [
        headers,           // headers(self, headers: HeaderMap) -> Self
        timeout,           // timeout(self, timeout: Duration) -> Self
        version,           // version(self, version: Version) -> Self
        build,             // build(self) -> Result<Request>
        send,              // send(self) -> Future<Response>
        try_clone,         // try_clone(&self) -> Option<Self>
    ]
}
```

**Generic Methods (use `#[reflect_methods]`):**
```rust
#[reflect_methods]
impl RequestBuilder {
    // Generic over K, V
    pub fn header<K, V>(self, key: K, value: V) -> RequestBuilder

    // Generic over T: Serialize
    pub fn query<T: Serialize + ?Sized>(self, query: &T) -> RequestBuilder
    pub fn json<T: Serialize + ?Sized>(self, json: &T) -> RequestBuilder
    pub fn form<T: Serialize + ?Sized>(self, form: &T) -> RequestBuilder

    // Generic over T: Into<Body>
    pub fn body<T: Into<Body>>(self, body: T) -> RequestBuilder

    // Generic over U, P: Display
    pub fn basic_auth<U, P>(self, username: U, password: Option<P>) -> RequestBuilder

    // Generic over T: Display
    pub fn bearer_auth<T: Display>(self, token: T) -> RequestBuilder
}
```

**Rationale:**
- Tests both macro systems on same type
- Demonstrates when to use each approach
- `elicit_newtype_methods!` for simple delegation
- `#[reflect_methods]` required for generics

### 3. Response Wrapper (`response.rs`)

**Macro Strategy:**
- Use `elicit_newtype!(reqwest::Response, as Response)` for wrapper
- **Mixed approach:**

**Non-Generic Methods (use `elicit_newtype_methods!`):**
```rust
elicit_newtype_methods! {
    Response => [
        status,            // status(&self) -> StatusCode
        version,           // version(&self) -> Version
        headers,           // headers(&self) -> &HeaderMap
        content_length,    // content_length(&self) -> Option<u64>
        url,               // url(&self) -> &Url
        remote_addr,       // remote_addr(&self) -> Option<SocketAddr>
        text,              // text(self) -> Future<String>
        bytes,             // bytes(self) -> Future<Bytes>
        chunk,             // chunk(&mut self) -> Future<Option<Bytes>>
        error_for_status,  // error_for_status(self) -> Result<Self>
    ]
}
```

**Generic Method (use `#[reflect_methods]`):**
```rust
#[reflect_methods]
impl Response {
    // Generic over T: DeserializeOwned
    pub async fn json<T: DeserializeOwned>(self) -> Result<T>
}
```

**Rationale:**
- Most Response methods are non-generic
- Only `json<T>()` requires generic support
- Demonstrates predominant use of `elicit_newtype_methods!` with selective generic support

### 4. Supporting Types (`types.rs`)

**Macro Strategy:**
- Use `elicit_newtype!` for simple wrappers of re-exported types
- No method wrappers needed (these are just type aliases)

```rust
// Re-export from reqwest
pub use reqwest::{Method, StatusCode, Version};

// Wrap types we control
elicit_newtype!(reqwest::Error, as Error);
elicit_newtype!(url::Url, as Url);
elicit_newtype!(http::HeaderMap, as HeaderMap);
```

**Rationale:**
- Provides type-safe elicitation for complete HTTP workflows
- Tests newtype wrapper generation for external types
- No methods to wrap, just type identity

## Trait Bounds Strategy

### For Generic Methods

All generic type parameters must satisfy:
```rust
where
    T: Elicitation + JsonSchema + [OriginalBound],
```

**Examples:**

```rust
// Original reqwest signature:
pub fn json<T: Serialize + ?Sized>(self, json: &T) -> RequestBuilder

// Our wrapper signature:
pub fn json_tool<T>(
    &self,
    params: Parameters<JsonParams<T>>,
) -> Result<Json<RequestBuilder>, ErrorData>
where
    T: Elicitation + JsonSchema + Serialize,
```

**Key Points:**
- Preserve original bounds (`Serialize`, `Display`, etc.)
- Add elicitation bounds (`Elicitation + JsonSchema`)
- Use reference types (`&T`) in parameter structs (auto-converted by macro)

## Dependencies

```toml
[dependencies]
elicitation = { path = "../elicitation", version = "0.8.2" }
elicitation_derive = { path = "../elicitation_derive", version = "0.8.2" }

# Core reqwest with commonly-used features
reqwest = { version = "0.13", features = [
    "json",
    "cookies",
    "stream",
] }

# Required for types
url = "2.5"
http = "1.0"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["rt", "macros"] }

[dev-dependencies]
tokio = { version = "1", features = ["rt", "macros", "test-util"] }
```

## Testing Strategy

### 1. Macro Expansion Tests (`tests/macros_test.rs`)

Verify macro output compiles and generates expected code:

```rust
#[test]
fn test_client_generic_methods_compile() {
    // Verify #[reflect_methods] generates correct generic wrappers
    let client = Client::new();

    // Test generic method exists with correct signature
    let _builder = client.get_tool(/* ... */);
}

#[test]
fn test_request_builder_mixed_macros() {
    // Verify both macro types work together
    // Non-generic from elicit_newtype_methods!
    // Generic from #[reflect_methods]
}
```

### 2. Integration Tests (`tests/integration_test.rs`)

End-to-end HTTP workflow tests:

```rust
#[tokio::test]
async fn test_get_request_workflow() {
    // Create client
    let client = Client::new();

    // Build request
    let response = client
        .get("https://httpbin.org/get")
        .send()
        .await
        .unwrap();

    // Read response
    let status = response.status();
    let text = response.text().await.unwrap();

    assert_eq!(status, StatusCode::OK);
}
```

### 3. Generic Method Tests (`tests/client_test.rs`)

Specific tests for generic method support:

```rust
#[test]
fn test_generic_url_types() {
    let client = Client::new();

    // Test with different IntoUrl implementations
    let _b1 = client.get("https://example.com");
    let _b2 = client.get(Url::parse("https://example.com").unwrap());
}

#[test]
fn test_json_generic() {
    // Test generic T: Serialize method
    #[derive(Serialize)]
    struct Payload {
        key: String,
    }

    let client = Client::new();
    let _builder = client.post("https://httpbin.org/post")
        .json(&Payload { key: "value".to_string() });
}
```

## Implementation Phases

### Phase 1: Core Structure
1. Create crate skeleton
2. Add dependencies
3. Create basic newtype wrappers

### Phase 2: Non-Generic Methods
1. Implement `elicit_newtype_methods!` for RequestBuilder
2. Implement `elicit_newtype_methods!` for Response
3. Write basic tests

### Phase 3: Generic Methods
1. Implement `#[reflect_methods]` for Client
2. Implement `#[reflect_methods]` for RequestBuilder generic methods
3. Implement `#[reflect_methods]` for Response::json
4. Write generic-specific tests

### Phase 4: Integration
1. End-to-end workflow tests
2. Documentation with examples
3. Benchmark macro expansion times
4. Update main crate docs with real-world example

## Success Criteria

- ✅ All three macro types used in realistic scenarios
- ✅ Generic methods work correctly with MCP tool generation
- ✅ Mixed macro usage (newtype_methods + reflect_methods) on same type
- ✅ Complete HTTP workflow demonstrable
- ✅ All tests pass
- ✅ Zero clippy warnings
- ✅ Demonstrates real-world usage patterns

## Edge Cases to Test

1. **Generic bounds preservation**: Verify `Serialize`, `Display`, etc. preserved
2. **Reference conversion**: `&str` → `String`, `&T` → `T` in parameter structs
3. **Async method support**: Both sync and async generics
4. **Multiple type parameters**: e.g., `basic_auth<U, P>`
5. **Trait object safety**: Methods with `?Sized` bounds
6. **Builder pattern chains**: Method chaining works correctly

## Documentation Strategy

Each wrapper type should include:
- Module-level docs explaining wrapping strategy
- Examples showing MCP tool usage
- Notes on when to use each macro type
- Links to original reqwest documentation

## Future Enhancements

After initial implementation:
1. Add more reqwest features (multipart, cookies, redirects)
2. Create example MCP server using elicit_reqwest
3. Benchmark against hand-written wrappers
4. Document performance characteristics
5. Consider upstreaming patterns to reqwest if valuable

---

## References

- [reqwest Documentation](https://docs.rs/reqwest/latest/reqwest/)
- [Client Methods](https://docs.rs/reqwest/latest/reqwest/struct.Client.html)
- [RequestBuilder Methods](https://docs.rs/reqwest/latest/reqwest/struct.RequestBuilder.html)
- [Response Methods](https://docs.rs/reqwest/latest/reqwest/struct.Response.html)
- [HeaderMap Documentation](https://docs.rs/reqwest/latest/reqwest/header/struct.HeaderMap.html)
- [How to Build HTTP Clients in Rust with Reqwest](https://oneuptime.com/blog/post/2026-01-26-rust-reqwest-http-client/view)
- [GitHub - reqwest](https://github.com/seanmonstar/reqwest)
