# serde_json::Value Elicitation Implementation Plan
## Feature: Elicit arbitrary JSON values conversationally

## Executive Summary

Implement `Elicitation` for `serde_json::Value` behind a feature flag, enabling conversational elicitation of arbitrary JSON data. This unblocks all Rust types containing `Value` (tool arguments, API responses, config files) and makes elicitation universally useful.

**Target version:** 0.2.2
**Feature flag:** `serde_json`
**Estimated effort:** 3-5 hours

---

## Motivation

### The Problem

Common Rust types frequently contain `serde_json::Value`:

```rust
// Tool arguments (unknown shape at compile time)
pub struct ToolCall {
    id: String,
    name: String,
    arguments: serde_json::Value,  // ❌ Can't derive Elicit
}

// Dynamic config
pub enum Output {
    Json(serde_json::Value),  // ❌ Can't derive Elicit
}

// API responses
pub struct ApiResponse {
    data: serde_json::Value,  // ❌ Can't derive Elicit
}
```

**Without this feature:** These types cannot use `#[derive(Elicit)]`, forcing manual implementations or abandoning elicitation entirely.

**With this feature:** Every type with `Value` gets free elicitation via the derive macro.

### Why serde_json::Value?

1. **Ubiquitous** - Every Rust project with JSON uses it
2. **Standard escape hatch** - When shape is unknown/dynamic
3. **Natural fit** - Value is essentially an enum with elicitable variants
4. **Ecosystem impact** - Unlocks elicitation for thousands of crates

### Why Feature-Gate?

- **Zero cost** - Users without JSON don't pay dependency tax
- **Opt-in semantics** - Explicit: "I want JSON elicitation"
- **Convention** - Matches ecosystem patterns (`serde` has 15+ features)
- **Testing** - Can verify behavior with/without feature

---

## Design

### Value Structure

```rust
pub enum Value {
    Null,
    Bool(bool),
    Number(Number),  // i64, u64, or f64
    String(String),
    Array(Vec<Value>),
    Object(Map<String, Value>),
}
```

### Elicitation Flow

**State machine per variant:**

1. **Type Selection** (State 0)
   - Select: "string", "number", "boolean", "array", "object", "null"
   - Transition to variant-specific states

2. **Variant States:**

   **Null:** Immediate terminal (no fields)
   
   **Bool:** `bool::elicit()` → terminal
   
   **String:** `String::elicit()` → terminal
   
   **Number:** 
   - Prompt: "Enter number (integer or decimal):"
   - Parse as f64 (covers i64/u64 range + decimals)
   - Construct via `serde_json::json!(num)`
   
   **Array:**
   - Loop:
     - Prompt: "Add item to array?"
     - If yes: `Value::elicit()` recursively → push to vec
     - If no: terminal
   
   **Object:**
   - Loop:
     - Prompt: "Add field to object?"
     - If yes:
       - `String::elicit()` for key
       - `Value::elicit()` recursively for value
       - Insert into map
     - If no: terminal

### Recursion Management

**Depth limit:** 10 levels (configurable via attribute in future)

```rust
impl Value {
    async fn elicit_with_depth<C: McpClient>(
        client: &C,
        depth: usize,
    ) -> Result<Self, ElicitError> {
        if depth > 10 {
            return Err(ElicitError::RecursionDepthExceeded(10));
        }
        // ... variant elicitation with depth + 1 for recursive calls
    }
}
```

**Rationale:**
- Prevents infinite recursion
- 10 levels is deeper than typical JSON nesting
- User can Ctrl+C if they go too deep
- Error message guides user to simplify structure

---

## Implementation Plan

### Phase 1: Add Feature Flag and Dependencies

**File: `crates/elicitation/Cargo.toml`**

```toml
[features]
default = []
api = []
serde_json = []  # NEW: Enable serde_json::Value elicitation
```

**Note:** `serde_json` is already in workspace dependencies, so no new deps needed.

### Phase 2: Implement Elicitation for Value

**File: `crates/elicitation/src/value_impl.rs` (NEW)**

```rust
//! Elicitation implementation for serde_json::Value.
//!
//! Available with the `serde_json` feature flag.

use crate::{Affirm, Elicitation, McpClient, Prompt};
use derive_more::{Display, Error};
use rmcp::ErrorData;
use tracing::instrument;

/// Maximum recursion depth for nested JSON structures.
const MAX_DEPTH: usize = 10;

/// Error during Value elicitation.
#[derive(Debug, Clone, Display, Error)]
pub enum ValueElicitError {
    /// Exceeded maximum recursion depth.
    #[display("Recursion depth exceeded: max {}", _0)]
    RecursionDepthExceeded(usize),

    /// Invalid number format.
    #[display("Invalid number: {}", _0)]
    InvalidNumber(String),

    /// Client error.
    #[display("Client error: {}", _0)]
    Client(String),
}

impl From<ValueElicitError> for ErrorData {
    fn from(err: ValueElicitError) -> Self {
        ErrorData::new(
            rmcp::model::ErrorCode::INTERNAL_ERROR,
            err.to_string(),
            None,
        )
    }
}

#[cfg(feature = "serde_json")]
impl Elicitation for serde_json::Value {
    #[instrument(skip(client))]
    async fn elicit<C: McpClient>(client: &C) -> Result<Self, ErrorData> {
        elicit_with_depth(client, 0).await
    }
}

/// Internal implementation with depth tracking.
#[instrument(skip(client))]
async fn elicit_with_depth<C: McpClient>(
    client: &C,
    depth: usize,
) -> Result<serde_json::Value, ErrorData> {
    use serde_json::{Map, Value};

    // Check recursion depth
    if depth > MAX_DEPTH {
        tracing::error!(depth, max = MAX_DEPTH, "Recursion depth exceeded");
        return Err(ValueElicitError::RecursionDepthExceeded(MAX_DEPTH).into());
    }

    tracing::debug!(depth, "Eliciting JSON value");

    // Phase 1: Select variant
    let type_choice = String::prompt("Select JSON value type:")
        .with_select(vec![
            "string",
            "number",
            "boolean",
            "array",
            "object",
            "null",
        ])
        .elicit(client)
        .await?;

    tracing::debug!(type_choice = %type_choice, "User selected type");

    // Phase 2: Elicit variant-specific data
    match type_choice.as_str() {
        "null" => {
            tracing::debug!("Eliciting null");
            Ok(Value::Null)
        }

        "boolean" => {
            tracing::debug!("Eliciting boolean");
            let value = bool::prompt("Enter boolean value:").elicit(client).await?;
            Ok(Value::Bool(value))
        }

        "string" => {
            tracing::debug!("Eliciting string");
            let value = String::prompt("Enter string value:").elicit(client).await?;
            Ok(Value::String(value))
        }

        "number" => {
            tracing::debug!("Eliciting number");
            let input = String::prompt("Enter number (integer or decimal):")
                .elicit(client)
                .await?;

            // Try parsing as f64 (covers all JSON number cases)
            let num: f64 = input.parse().map_err(|e| {
                tracing::error!(input = %input, error = ?e, "Invalid number format");
                ValueElicitError::InvalidNumber(input.clone())
            })?;

            tracing::debug!(number = num, "Parsed number");
            Ok(serde_json::json!(num))
        }

        "array" => {
            tracing::debug!("Eliciting array");
            let mut items = Vec::new();

            loop {
                let add_more = bool::prompt("Add item to array?")
                    .elicit(client)
                    .await?;

                if !add_more {
                    tracing::debug!(count = items.len(), "Array elicitation complete");
                    break;
                }

                tracing::debug!(index = items.len(), "Eliciting array item");
                let item = elicit_with_depth(client, depth + 1).await?;
                items.push(item);
            }

            Ok(Value::Array(items))
        }

        "object" => {
            tracing::debug!("Eliciting object");
            let mut map = Map::new();

            loop {
                let add_more = bool::prompt("Add field to object?")
                    .elicit(client)
                    .await?;

                if !add_more {
                    tracing::debug!(field_count = map.len(), "Object elicitation complete");
                    break;
                }

                // Elicit key
                let key = String::prompt("Field name:").elicit(client).await?;
                tracing::debug!(key = %key, "Eliciting object field");

                // Elicit value recursively
                let value = elicit_with_depth(client, depth + 1).await?;
                map.insert(key, value);
            }

            Ok(Value::Object(map))
        }

        _ => {
            tracing::error!(type_choice = %type_choice, "Invalid type choice");
            Err(ErrorData::new(
                rmcp::model::ErrorCode::INVALID_PARAMS,
                format!("Invalid type choice: {}", type_choice),
                None,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests will use mock MCP client (to be added in Phase 4)
}
```

### Phase 3: Add Module to Library

**File: `crates/elicitation/src/lib.rs`**

Add at appropriate location:

```rust
#[cfg(feature = "serde_json")]
mod value_impl;

#[cfg(feature = "serde_json")]
pub use value_impl::ValueElicitError;
```

### Phase 4: Add Tests

**File: `crates/elicitation/tests/value_elicit_test.rs` (NEW)**

```rust
//! Tests for serde_json::Value elicitation.

#![cfg(feature = "serde_json")]

use elicitation::Elicitation;
use serde_json::{json, Value};

mod helpers;

#[tokio::test]
async fn test_elicit_null() -> anyhow::Result<()> {
    helpers::init_test_tracing("info");
    tracing::info!("Testing null value elicitation");

    // Mock client that responds: "null"
    let client = helpers::MockClient::new(vec![
        helpers::Response::Select("null".to_string()),
    ]);

    let value = Value::elicit(&client).await?;
    assert_eq!(value, Value::Null);

    tracing::info!("Null elicitation test passed");
    Ok(())
}

#[tokio::test]
async fn test_elicit_boolean() -> anyhow::Result<()> {
    helpers::init_test_tracing("info");
    tracing::info!("Testing boolean value elicitation");

    let client = helpers::MockClient::new(vec![
        helpers::Response::Select("boolean".to_string()),
        helpers::Response::Bool(true),
    ]);

    let value = Value::elicit(&client).await?;
    assert_eq!(value, Value::Bool(true));

    tracing::info!("Boolean elicitation test passed");
    Ok(())
}

#[tokio::test]
async fn test_elicit_string() -> anyhow::Result<()> {
    helpers::init_test_tracing("info");
    tracing::info!("Testing string value elicitation");

    let client = helpers::MockClient::new(vec![
        helpers::Response::Select("string".to_string()),
        helpers::Response::Text("hello".to_string()),
    ]);

    let value = Value::elicit(&client).await?;
    assert_eq!(value, Value::String("hello".to_string()));

    tracing::info!("String elicitation test passed");
    Ok(())
}

#[tokio::test]
async fn test_elicit_number_integer() -> anyhow::Result<()> {
    helpers::init_test_tracing("info");
    tracing::info!("Testing integer number elicitation");

    let client = helpers::MockClient::new(vec![
        helpers::Response::Select("number".to_string()),
        helpers::Response::Text("42".to_string()),
    ]);

    let value = Value::elicit(&client).await?;
    assert_eq!(value, json!(42.0));

    tracing::info!("Integer number elicitation test passed");
    Ok(())
}

#[tokio::test]
async fn test_elicit_number_decimal() -> anyhow::Result<()> {
    helpers::init_test_tracing("info");
    tracing::info!("Testing decimal number elicitation");

    let client = helpers::MockClient::new(vec![
        helpers::Response::Select("number".to_string()),
        helpers::Response::Text("3.14".to_string()),
    ]);

    let value = Value::elicit(&client).await?;
    assert_eq!(value, json!(3.14));

    tracing::info!("Decimal number elicitation test passed");
    Ok(())
}

#[tokio::test]
async fn test_elicit_array_empty() -> anyhow::Result<()> {
    helpers::init_test_tracing("info");
    tracing::info!("Testing empty array elicitation");

    let client = helpers::MockClient::new(vec![
        helpers::Response::Select("array".to_string()),
        helpers::Response::Bool(false), // Don't add items
    ]);

    let value = Value::elicit(&client).await?;
    assert_eq!(value, json!([]));

    tracing::info!("Empty array elicitation test passed");
    Ok(())
}

#[tokio::test]
async fn test_elicit_array_with_items() -> anyhow::Result<()> {
    helpers::init_test_tracing("info");
    tracing::info!("Testing array with items elicitation");

    let client = helpers::MockClient::new(vec![
        helpers::Response::Select("array".to_string()),
        helpers::Response::Bool(true),              // Add first item
        helpers::Response::Select("string".to_string()),
        helpers::Response::Text("hello".to_string()),
        helpers::Response::Bool(true),              // Add second item
        helpers::Response::Select("number".to_string()),
        helpers::Response::Text("42".to_string()),
        helpers::Response::Bool(false),             // Done
    ]);

    let value = Value::elicit(&client).await?;
    assert_eq!(value, json!(["hello", 42.0]));

    tracing::info!("Array with items elicitation test passed");
    Ok(())
}

#[tokio::test]
async fn test_elicit_object_empty() -> anyhow::Result<()> {
    helpers::init_test_tracing("info");
    tracing::info!("Testing empty object elicitation");

    let client = helpers::MockClient::new(vec![
        helpers::Response::Select("object".to_string()),
        helpers::Response::Bool(false), // Don't add fields
    ]);

    let value = Value::elicit(&client).await?;
    assert_eq!(value, json!({}));

    tracing::info!("Empty object elicitation test passed");
    Ok(())
}

#[tokio::test]
async fn test_elicit_object_with_fields() -> anyhow::Result<()> {
    helpers::init_test_tracing("info");
    tracing::info!("Testing object with fields elicitation");

    let client = helpers::MockClient::new(vec![
        helpers::Response::Select("object".to_string()),
        helpers::Response::Bool(true),              // Add first field
        helpers::Response::Text("name".to_string()),
        helpers::Response::Select("string".to_string()),
        helpers::Response::Text("Alice".to_string()),
        helpers::Response::Bool(true),              // Add second field
        helpers::Response::Text("age".to_string()),
        helpers::Response::Select("number".to_string()),
        helpers::Response::Text("30".to_string()),
        helpers::Response::Bool(false),             // Done
    ]);

    let value = Value::elicit(&client).await?;
    assert_eq!(value, json!({"name": "Alice", "age": 30.0}));

    tracing::info!("Object with fields elicitation test passed");
    Ok(())
}

#[tokio::test]
async fn test_elicit_nested_structure() -> anyhow::Result<()> {
    helpers::init_test_tracing("info");
    tracing::info!("Testing nested structure elicitation");

    // Build: {"user": {"name": "Bob", "scores": [10, 20]}}
    let client = helpers::MockClient::new(vec![
        helpers::Response::Select("object".to_string()),
        helpers::Response::Bool(true),              // Add "user" field
        helpers::Response::Text("user".to_string()),
        helpers::Response::Select("object".to_string()), // user is object
        helpers::Response::Bool(true),              // Add "name" field
        helpers::Response::Text("name".to_string()),
        helpers::Response::Select("string".to_string()),
        helpers::Response::Text("Bob".to_string()),
        helpers::Response::Bool(true),              // Add "scores" field
        helpers::Response::Text("scores".to_string()),
        helpers::Response::Select("array".to_string()), // scores is array
        helpers::Response::Bool(true),              // Add first score
        helpers::Response::Select("number".to_string()),
        helpers::Response::Text("10".to_string()),
        helpers::Response::Bool(true),              // Add second score
        helpers::Response::Select("number".to_string()),
        helpers::Response::Text("20".to_string()),
        helpers::Response::Bool(false),             // Done with scores array
        helpers::Response::Bool(false),             // Done with user object
        helpers::Response::Bool(false),             // Done with root object
    ]);

    let value = Value::elicit(&client).await?;
    assert_eq!(
        value,
        json!({
            "user": {
                "name": "Bob",
                "scores": [10.0, 20.0]
            }
        })
    );

    tracing::info!("Nested structure elicitation test passed");
    Ok(())
}

#[tokio::test]
async fn test_recursion_depth_limit() -> anyhow::Result<()> {
    helpers::init_test_tracing("info");
    tracing::info!("Testing recursion depth limit");

    // Try to build 12 nested arrays (exceeds limit of 10)
    let mut responses = vec![helpers::Response::Select("array".to_string())];
    for _ in 0..12 {
        responses.push(helpers::Response::Bool(true)); // Add item
        responses.push(helpers::Response::Select("array".to_string())); // Nested array
    }

    let client = helpers::MockClient::new(responses);

    let result = Value::elicit(&client).await;
    assert!(result.is_err(), "Should fail with recursion depth error");

    if let Err(err) = result {
        assert!(
            err.message.contains("Recursion depth exceeded"),
            "Error should mention recursion depth"
        );
    }

    tracing::info!("Recursion depth limit test passed");
    Ok(())
}
```

### Phase 5: Update Documentation

**File: `crates/elicitation/README.md`**

Add section:

```markdown
### JSON Value Elicitation

With the `serde_json` feature, you can elicit arbitrary `serde_json::Value` types:

```toml
[dependencies]
elicitation = { version = "0.2.2", features = ["serde_json"] }
```

This is useful for:
- Tool arguments with unknown structure
- Dynamic API responses
- Configuration files

Example:

```rust
use elicitation::Elicitation;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = /* your MCP client */;
    
    // Elicit arbitrary JSON
    let config: Value = Value::elicit(&client).await?;
    println!("Configured: {}", config);
    
    Ok(())
}
```

The elicitation prompts the user to:
1. Select JSON type (string, number, boolean, array, object, null)
2. Provide value(s) for that type
3. For arrays/objects, recursively elicit nested values

Recursion is limited to 10 levels deep to prevent infinite nesting.
```

**File: `CHANGELOG.md`**

Add entry:

```markdown
## [0.2.2] - YYYY-MM-DD

### Added
- **Feature: `serde_json`** - Implement `Elicitation` for `serde_json::Value`
  - Enables conversational elicitation of arbitrary JSON data
  - Supports all JSON types: null, boolean, number, string, array, object
  - Recursive elicitation for nested structures (max depth: 10)
  - Zero-cost when feature not enabled
  - Unlocks `#[derive(Elicit)]` for types containing `serde_json::Value`

### Changed
- N/A

### Fixed
- N/A
```

### Phase 6: Feature Documentation

**File: `SERDE_JSON_FEATURE.md` (NEW)**

```markdown
# serde_json Feature

## Overview

The `serde_json` feature adds `Elicitation` support for `serde_json::Value`, enabling conversational elicitation of arbitrary JSON data.

## Enabling

```toml
[dependencies]
elicitation = { version = "0.2.2", features = ["serde_json"] }
```

## Use Cases

### 1. Tool Arguments (Unknown Shape)

```rust
#[derive(Debug, Elicit)]
pub struct ToolCall {
    id: String,
    name: String,
    arguments: serde_json::Value,  // ✅ Works with serde_json feature
}
```

### 2. Dynamic API Responses

```rust
#[derive(Debug, Elicit)]
pub struct ApiResponse {
    status: u16,
    data: serde_json::Value,  // ✅ Shape varies by endpoint
}
```

### 3. Configuration Files

```rust
#[derive(Debug, Elicit)]
pub struct DynamicConfig {
    plugins: Vec<serde_json::Value>,  // ✅ Plugin-specific config
}
```

## Elicitation Flow

### Type Selection

First, the user selects the JSON type:

```
? Select JSON value type:
  > string
    number
    boolean
    array
    object
    null
```

### Scalar Types

For `null`, `boolean`, `string`, `number` - single prompt, then done.

### Collections

For `array` and `object` - loop until user says "done":

**Array:**
```
? Add item to array? (y/n): y
? Select JSON value type: string
? Enter string value: hello
? Add item to array? (y/n): y
? Select JSON value type: number
? Enter number: 42
? Add item to array? (y/n): n
```

Result: `["hello", 42]`

**Object:**
```
? Add field to object? (y/n): y
? Field name: name
? Select JSON value type: string
? Enter string value: Alice
? Add field to object? (y/n): y
? Field name: age
? Select JSON value type: number
? Enter number: 30
? Add field to object? (y/n): n
```

Result: `{"name": "Alice", "age": 30}`

## Nested Structures

Recursive elicitation supports nesting up to 10 levels deep:

```json
{
  "user": {
    "profile": {
      "contact": {
        "emails": ["alice@example.com"],
        "phones": ["+1-555-0100"]
      }
    }
  }
}
```

If depth exceeds 10, elicitation fails with clear error message.

## Number Handling

All numbers are parsed as `f64` internally, which covers:
- Integers: -2^53 to 2^53 (JavaScript safe integer range)
- Decimals: Standard IEEE 754 double precision

This matches JSON's number semantics.

## Performance

- **Zero-cost when disabled:** Feature flag ensures no dependency/compilation overhead
- **Minimal allocations:** Reuses existing `String`, `bool` elicitation
- **Lazy evaluation:** Only prompts for fields user adds

## Limitations

1. **No schema validation** - Any JSON shape is accepted
   - *Workaround:* Use strongly-typed structs when shape is known
2. **Depth limit (10 levels)** - Prevents infinite recursion
   - *Workaround:* Simplify structure or nest in multiple top-level calls
3. **Number precision** - All numbers stored as f64
   - *Impact:* Integers > 2^53 lose precision
   - *Workaround:* Use strings for large integers (e.g., BigInt)

## Testing

Run tests with feature enabled:

```bash
cargo test --features serde_json
```

Mock client helpers provided in `tests/helpers/mod.rs`.

## Future Enhancements

- **Custom prompts** - Per-variant prompt customization
- **Schema hints** - Optional JSON Schema for validation
- **Large integer support** - Preserve i64/u64 precision
- **Configurable depth** - `#[elicit(max_depth = N)]` attribute
```

---

## Testing Strategy

### Unit Tests (Phase 4)

1. **Scalar types** - Null, bool, string, number
2. **Empty collections** - `[]`, `{}`
3. **Collections with items** - Arrays with mixed types, objects with fields
4. **Nested structures** - 3-4 levels deep
5. **Recursion limit** - Verify 10-level cap
6. **Error cases** - Invalid number format, malformed responses

### Integration Tests

Using mock MCP client:

```rust
pub struct MockClient {
    responses: VecDeque<Response>,
}

pub enum Response {
    Text(String),
    Bool(bool),
    Select(String),
}
```

### Manual Testing

Create example in `examples/json_elicit.rs`:

```rust
//! Example: Elicit arbitrary JSON configuration

use elicitation::Elicitation;
use rmcp::transport::io::StdioTransport;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    println!("JSON Configuration Elicitor");
    println!("==========================\n");

    let transport = StdioTransport::default();
    let client = rmcp::Client::new(transport);

    println!("Let's build a JSON configuration...\n");
    let config: Value = Value::elicit(&client).await?;

    println!("\n✅ Configuration created:");
    println!("{}", serde_json::to_string_pretty(&config)?);

    Ok(())
}
```

---

## Release Checklist

- [ ] Phase 1: Feature flag added
- [ ] Phase 2: Implementation complete
- [ ] Phase 3: Module integrated
- [ ] Phase 4: Tests written and passing
- [ ] Phase 5: Documentation updated
- [ ] Phase 6: Feature guide written
- [ ] Manual testing with example
- [ ] CI passes with and without feature
- [ ] CHANGELOG updated
- [ ] Version bumped to 0.2.2
- [ ] Git tag created
- [ ] Published to crates.io

---

## Success Metrics

1. **Compilation:** All tests pass with `--features serde_json`
2. **Zero cost:** No compilation/dependency overhead without feature
3. **Unblocking:** Botticelli types (`Input`, `Output`, etc.) can derive `Elicit`
4. **Ecosystem adoption:** Other crates can use `Value` in their types

---

## Open Questions

1. **Number precision:** Should we add separate i64/u64/f64 prompts?
   - **Recommendation:** Start with f64, add precision options in 0.2.3 if needed

2. **Depth limit configuration:** Make it adjustable?
   - **Recommendation:** Fixed at 10 for 0.2.2, add `#[elicit(max_depth)]` in 0.3.0

3. **Schema validation:** Validate against JSON Schema?
   - **Recommendation:** Out of scope for 0.2.2, consider for 0.3.0

4. **Prompt customization:** Allow custom prompts per variant?
   - **Recommendation:** Future work, use defaults for 0.2.2

---

## Implementation Notes

### Why `elicit_with_depth` is separate

```rust
// Public API (depth = 0)
impl Elicitation for Value {
    async fn elicit<C>(client: &C) -> Result<Self, ErrorData> {
        elicit_with_depth(client, 0).await
    }
}

// Internal recursion (tracks depth)
async fn elicit_with_depth<C>(client: &C, depth: usize) -> Result<Value, ErrorData>
```

- **Public API** is clean: `Value::elicit(&client)`
- **Internal tracking** handles recursion depth
- **Error propagation** preserves context through call stack

### Why f64 for numbers

JSON spec defines numbers as IEEE 754 doubles. While Rust's `serde_json::Number` preserves original representation (i64/u64/f64), eliciting from text requires choosing a parse target.

**Trade-offs:**

| Choice | Pros | Cons |
|--------|------|------|
| f64 | Simple, covers decimals | Integer precision loss > 2^53 |
| i64 then f64 fallback | Preserves integer precision | Complex UX (two parse attempts) |
| Ask user | Maximum precision | Tedious for users |

**Decision:** Use f64 for 0.2.2 simplicity. Add precision options in 0.2.3 if demand exists.

### Why feature flag

**Without flag:**
- All users pay `serde_json` dependency cost
- Library appears JSON-specific

**With flag:**
- Opt-in: "I need JSON elicitation"
- Zero cost for non-JSON users
- Follows ecosystem conventions

---

## Timeline

**Estimated: 3-5 hours**

- Phase 1: 15 minutes
- Phase 2: 90 minutes (implementation)
- Phase 3: 5 minutes
- Phase 4: 60 minutes (tests)
- Phase 5: 15 minutes (README update)
- Phase 6: 30 minutes (feature guide)
- Testing/fixes: 30 minutes
- Release: 15 minutes

**Target completion:** Same day as start
