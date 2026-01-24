# MCP Verification Integration Plan

## Goal
Use verification types as MCP elicitation wrappers while keeping user API primitive-focused.

## Architecture

```rust
User API (primitives)
        ↓
Elicit trait impl
        ↓
Verification type (wrapper) ← [MCP Schema + Kani Contract]
        ↓
rmcp::elicit<T>()
        ↓
Client validates schema
        ↓
Serde deserializes
        ↓
Unwrap to primitive
        ↓
Return to user
```

## Implementation Steps

### Phase 1: Add Derives to Verification Types ✅ (In Progress)
- [x] Add `Serialize`, `Deserialize`, `JsonSchema` to all verification types
- [x] Add `#[schemars]` attributes for validation constraints
- [x] Add `rmcp::elicit_safe!()` macro calls

Status: **Proof of concept complete** - Added derives to:
- `I8Positive` 
- `I8NonNegative`
- `I8NonZero`
- `I8Range`

These types now work with `rmcp::elicit<T>()`.

**Remaining work**: Add derives to all other verification types systematically
(floats, strings, bools, chars, datetimes, etc.). Can be done incrementally.

### Phase 2: Update Primitive Elicit Implementations (Next)
- [ ] Add `I64Default` wrapper type to verification/types/integers.rs
- [ ] Update `i64` primitive to use `I64Default` + `rmcp::elicit<T>()`
- [ ] Test proof of concept end-to-end
- [ ] Add wrapper types for other primitives (String, bool, etc.)
- [ ] Update their Elicitation impls to use wrappers

**Pattern**:
```rust
// In verification/types/integers.rs
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Integer value")]
pub struct I64Default(#[schemars(range(min = i64::MIN, max = i64::MAX))] i64);
rmcp::elicit_safe!(I64Default);

// In primitives/integers.rs
impl Elicitation for i64 {
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        let wrapper: I64Default = client.peer().elicit(prompt).await?
            .ok_or(ElicitError::NoInput)?;
        Ok(wrapper.into_inner())
    }
}
```

### Phase 3: Style System Integration
- [ ] Update `ElicitationStyle` trait to work with verification types
- [ ] Implement `with_style()` method on primitives
- [ ] Map style selection to different verification type variants

### Phase 4: Cleanup
- [ ] Remove `mcp/parsing.rs` (redundant with serde)
- [ ] Remove `mcp/tools.rs` (redundant with rmcp::elicit)
- [ ] Update examples to use new API

## Example Usage

### Before (Current)
```rust
let params = number_params("Enter age", 0, 150);
let result = call_tool("elicit_number", params).await?;
let age = parse_integer(&result)?; // Manual parsing
```

### After (Proposed)
```rust
let age: i64 = i64::elicit(&client, "Enter age (0-150)").await?;
// Internally uses I64Default with schemars constraints
```

### With Style
```rust
let age: i64 = i64::with_style(CompactStyle)
    .elicit(&client, "Age?")
    .await?;
```

## Benefits

1. **Dual Purpose Types**
   - Kani contracts for verification
   - JsonSchema for MCP validation
   - Single source of truth for constraints

2. **Clean User API**
   - Work with primitives directly
   - No wrapper leakage
   - Familiar Rust types

3. **Type Safety**
   - Client validates against schema
   - Server deserializes with serde
   - Kani verifies contracts
   - Eliminates handrolled parsers

4. **Maintainability**
   - Less code to maintain
   - Standard library usage (serde, schemars)
   - Clear separation of concerns

## Technical Details

### Verification Type Pattern

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Positive integer value (> 0)")]
pub struct I64Positive(
    #[schemars(range(min = 1))]
    i64
);

// MCP elicitation support
rmcp::elicit_safe!(I64Positive);

// Kani verification support
#[cfg(kani)]
impl Contract for I64Positive {
    fn pre_condition(&self) -> bool {
        self.0 > 0
    }
}
```

### Primitive Implementation Pattern

```rust
#[async_trait]
impl Elicit for i64 {
    async fn elicit(client: &impl ElicitClient, prompt: &str) -> Result<Self, ElicitationError> {
        // Use verification type wrapper internally
        let wrapper: I64Default = client.peer()
            .elicit(prompt)
            .await?
            .ok_or(ElicitationError::NoInput)?;
        
        // Unwrap and return primitive
        Ok(wrapper.into_inner())
    }
}
```

## Migration Strategy

### Step 1: Add Phase 1 (Derives) to All Types
Systematically add `Serialize`, `Deserialize`, `JsonSchema` to:
- `integers.rs` - All integer types
- `floats.rs` - All float types  
- `strings.rs` - All string types
- `bools.rs` - Bool type
- `chars.rs` - All char types
- And other verification type modules

### Step 2: Update ElicitClient Trait
Add `peer()` method to provide access to `rmcp::Peer<RoleServer>`:

```rust
pub trait ElicitClient {
    fn peer(&self) -> &rmcp::Peer<rmcp::service::RoleServer>;
}
```

### Step 3: Implement for Primitives
One by one, update primitive `Elicit` implementations to use verification types.

### Step 4: Test and Validate
- Ensure schemas generate correctly
- Verify client-side validation works
- Check contracts still verify with Kani
- Update all examples

### Step 5: Remove Old Code
Once all primitives migrated:
- Delete `mcp/parsing.rs`
- Delete `mcp/tools.rs`
- Update documentation

## Related Documents

- [ELICITATION_STYLE_SYSTEM_PLAN.md](./ELICITATION_STYLE_SYSTEM_PLAN.md)
- [VERIFICATION_FRAMEWORK_DESIGN.md](./VERIFICATION_FRAMEWORK_DESIGN.md)
- [KANI_VERIFICATION_PATTERNS.md](./KANI_VERIFICATION_PATTERNS.md)
