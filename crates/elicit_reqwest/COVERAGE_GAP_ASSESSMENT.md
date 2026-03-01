# Reqwest Coverage Gap Assessment

## Current State

### ✅ Already Supported in Elicitation
- **url::Url** - Has full Elicitation support via `feature = "url"`
  - Location: `crates/elicitation/src/primitives/url.rs`
  - Verification: `crates/elicitation/src/verification/types/urls.rs`
  - Proofs: Kani verification via UrlValid wrapper

### ❌ Missing from Elicitation (Blocks elicit_reqwest Demo)

The following reqwest types need Elicitation support before elicit_reqwest can demonstrate the full macro capabilities:

#### 1. **reqwest::Client** (High Priority)
- **Needed for:** Client constructor + generic HTTP method delegation
- **Trait Impls Required:**
  - `Prompt` - "Create HTTP client" or similar
  - `Elicitation` - Default construction via `reqwest::Client::new()`
  - `ElicitIntrospect` - Pattern: Primitive (opaque type)
  - `JsonSchema` - Opaque type represented as string

- **Verification Approach:**
  - Client is an opaque handle, minimal verification needed
  - Could use wrapper type `ClientValid` that verifies construction succeeded
  - Kani proof: construction doesn't panic

- **Example Usage:**
  ```rust
  #[reflect_methods]
  impl Client {
      pub fn get<U>(&self, url: U) -> RequestBuilder
      where U: Elicitation + JsonSchema + reqwest::IntoUrl
  }
  ```

#### 2. **reqwest::RequestBuilder** (High Priority)
- **Needed for:** Builder pattern with consuming methods
- **Trait Impls Required:**
  - `Prompt` - Cannot be elicited directly (only created from Client methods)
  - `Elicitation` - Returns error (not directly constructible)
  - `ElicitIntrospect` - Pattern: Builder
  - `JsonSchema` - Opaque type represented as string

- **Verification Approach:**
  - Builder is an opaque intermediate state
  - Verification: cannot be directly constructed
  - Kani proof: confirms impl panics/errors as expected

- **Example Usage:**
  ```rust
  #[reflect_methods]
  impl RequestBuilder {
      pub fn timeout(self, duration: Duration) -> Self { ... }
      pub fn json<T>(self, json: &T) -> Self { ... }
      pub async fn send(self) -> Result<Response, String> { ... }
  }
  ```

#### 3. **reqwest::Response** (High Priority)
- **Needed for:** Async consuming generic methods
- **Trait Impls Required:**
  - `Prompt` - Cannot be elicited directly (only from send())
  - `Elicitation` - Returns error (not directly constructible)
  - `ElicitIntrospect` - Pattern: Primitive (opaque result)
  - `JsonSchema` - Opaque type represented as string

- **Verification Approach:**
  - Response is opaque HTTP response
  - Verification: cannot be directly constructed
  - Kani proof: confirms impl errors as expected

- **Example Usage:**
  ```rust
  #[reflect_methods]
  impl Response {
      pub async fn json<T>(self) -> Result<T, String>
      where T: Elicitation + JsonSchema + DeserializeOwned
  }
  ```

#### 4. **http::HeaderMap** (Medium Priority)
- **Needed for:** HTTP header manipulation
- **Trait Impls Required:**
  - `Prompt` - "Enter HTTP headers (key: value format)"
  - `Elicitation` - Parse from HashMap<String, String> or similar
  - `ElicitIntrospect` - Pattern: Collection
  - `JsonSchema` - Object with string keys and values

- **Verification Approach:**
  - Could use `HeaderMapValid` wrapper
  - Verify: header names are ASCII, values are valid
  - Kani proof: parsing doesn't panic

#### 5. **reqwest::StatusCode** (Low Priority)
- **Trait Impls Required:**
  - Already implements Clone, Display
  - `Prompt` - "Enter HTTP status code (100-599)"
  - `Elicitation` - Parse from u16 with validation
  - `JsonSchema` - Integer with range constraints

- **Note:** Can use u16 directly for initial demos

## Implementation Priority

### Phase 1: Core Types (Enables Basic Demo)
1. **reqwest::Client** - Opaque constructor
2. **reqwest::RequestBuilder** - Non-constructible marker
3. **reqwest::Response** - Non-constructible marker

**Impact:** Enables demonstration of:
- Generic methods (`get<U: IntoUrl>`)
- Consuming builder patterns
- Async methods
- Return type wrapping

### Phase 2: Supporting Types
4. **http::HeaderMap** - Collection elicitation
5. **reqwest::StatusCode** - Validated integer

## Architecture Pattern for Reqwest Support

### Location in elicitation Crate
```
crates/elicitation/src/
├── primitives/
│   └── http/           # New module
│       ├── mod.rs
│       ├── client.rs   # reqwest::Client impl
│       ├── request_builder.rs
│       ├── response.rs
│       └── header_map.rs
└── verification/types/
    └── http/           # Verification wrappers
        ├── client_valid.rs
        └── ...
```

### Feature Flag Strategy
```toml
[features]
reqwest = ["dep:reqwest", "dep:http", "url"]  # Depends on url feature
http = ["dep:http"]  # HTTP types without reqwest
```

### Trait Implementation Template
```rust
// crates/elicitation/src/primitives/http/client.rs
use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, TypeMetadata,
};

crate::default_style!(reqwest::Client => ClientStyle);

impl Prompt for reqwest::Client {
    fn prompt() -> Option<&'static str> {
        Some("HTTP Client (auto-constructed)")
    }
}

impl Elicitation for reqwest::Client {
    type Style = ClientStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(_communicator: &C) -> ElicitResult<Self> {
        // Client is always default constructed
        Ok(reqwest::Client::new())
    }

    #[cfg(kani)]
    fn kani_proof() {
        // Verify construction doesn't panic
        let _client = reqwest::Client::new();
        assert!(true, "reqwest::Client construction verified");
    }
}

impl ElicitIntrospect for reqwest::Client {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "reqwest::Client",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}
```

## What This Unlocks

Once Phase 1 is complete, elicit_reqwest can demonstrate:

```rust
use elicitation::elicit_newtype;
use elicitation_derive::reflect_methods;

elicit_newtype!(reqwest::Client, as Client);

#[reflect_methods]
impl Client {
    // ✅ Generic method with trait bounds
    pub fn get<U>(&self, url: U) -> RequestBuilder
    where
        U: Elicitation + JsonSchema + reqwest::IntoUrl,
    {
        RequestBuilder::from(self.0.get(url))
    }
}

#[reflect_methods]
impl RequestBuilder {
    // ✅ Consuming builder method (returns Self)
    pub fn timeout(self, duration: Duration) -> Self {
        let inner = Arc::try_unwrap(self.0).expect("...");
        Self::from(inner.timeout(duration))
    }

    // ✅ Generic consuming builder method
    pub fn json<T>(self, json: &T) -> Self
    where
        T: Elicitation + JsonSchema + Serialize,
    {
        let inner = Arc::try_unwrap(self.0).expect("...");
        Self::from(inner.json(json))
    }

    // ✅ Async consuming method returning other type
    pub async fn send(self) -> Result<Response, String> {
        let inner = Arc::try_unwrap(self.0).expect("...");
        inner.send().await.map(Response::from).map_err(|e| e.to_string())
    }
}

#[reflect_methods]
impl Response {
    // ✅ The ultimate test: async consuming generic method!
    pub async fn json<T>(self) -> Result<T, String>
    where
        T: Elicitation + JsonSchema + DeserializeOwned,
    {
        let inner = Arc::try_unwrap(self.0).expect("...");
        inner.json::<T>().await.map_err(|e| e.to_string())
    }
}
```

This demonstrates **ALL** macro capabilities in one comprehensive example:
- ✅ Arc-based newtypes (universal Clone)
- ✅ Generic methods with complex trait bounds
- ✅ Consuming methods (builder pattern)
- ✅ Async methods
- ✅ Mixed borrowing and consuming
- ✅ Return type wrapping (Self and other types)
- ✅ Third-party library integration

## Recommendation

Implement Phase 1 (Client, RequestBuilder, Response) in elicitation crate:
- Add `reqwest` feature flag
- Create `primitives/http/` module
- Implement minimal verification wrappers
- Add Kani proofs for construction/non-construction
- Enable elicit_reqwest as the definitive integration test

This provides a complete, verified, architecturally-correct demonstration of the entire macro system.
