# Server-Side Elicitation Implementation Plan

## Executive Summary

With the architectural foundation now in place (Peer<RoleServer>, elicit_router! macro, proper tool registration), this document outlines the path to implementing actual server-side elicitation logic.

**Status**: Architecture ✅ Complete | Implementation ⏸️ Stubbed | Planning 📋 This Document

## Architecture Recap

### What We Have Now ✅

```rust
// Each #[derive(Elicit)] generates:
impl MyType {
    #[rmcp::tool]
    async fn elicit_checked(peer: Peer<RoleServer>) -> Result<Self, ElicitError> {
        // Currently: returns "not implemented" stub
    }
}

// Users aggregate with macro:
elicit_router! {
    pub MyElicitRouter: Config, User, Settings
}

// Combine routers:
let router = AppRouter::tool_router() + MyElicitRouter::tool_router();
```

**Key Insight**: This is SERVER-SIDE. The server receives a tool call, uses `peer` to communicate back to the requesting client (human or agent).

### The Implementation Gap

Current stub returns error. Need to:
1. Generate prompts for the type being elicited
2. Send prompts to client via `peer.create_message()`
3. Parse client responses
4. Validate and construct the type
5. Handle errors, retries, cancellation

## Design Decisions Needed

### Decision 1: Prompt Strategy

**Option A: JSON Schema (Structured)**
```rust
// System prompt: "Respond with JSON matching this schema"
// Schema: { "host": "string", "port": "number" }
// Parse: serde_json::from_str::<Config>(&response)
```

Pros:
- Works for any Serialize/Deserialize type
- Clear format expectations
- Composable (nested types work naturally)

Cons:
- Requires Serialize + Deserialize bounds
- Less conversational
- Schema generation complexity

**Option B: Natural Language (Conversational)**
```rust
// System: "Help user provide Config values"
// User prompt: "What's the host?" -> parse String
// User prompt: "What's the port?" -> parse u16
```

Pros:
- More natural interaction
- Works with any parseable type
- Better error messages

Cons:
- Multi-turn complexity
- State management needed
- Harder to compose

**Option C: Hybrid**
- Primitives: conversational
- Structs: JSON schema
- Collections: JSON arrays

**Recommendation**: Start with **Option A (JSON Schema)** for simplicity, add conversational later.

### Decision 2: Trait Bounds

**Question**: What bounds should `elicit_checked()` require?

**Option A: Serialize + Deserialize**
```rust
impl MyType where Self: Serialize + Deserialize {
    async fn elicit_checked(peer: Peer<RoleServer>) -> Result<Self, ElicitError>
}
```

Pros: Simple, leverages existing ecosystem
Cons: Requires serde, limits to serializable types

**Option B: Custom ElicitParse trait**
```rust
trait ElicitParse {
    fn from_client_text(text: &str) -> Result<Self, ElicitError>;
}
```

Pros: Flexible, can handle non-serde types
Cons: More traits to implement

**Recommendation**: **Option A** - most types already have serde. Can add ElicitParse later for special cases.

### Decision 3: Style System Integration

The existing `Elicitation` trait has an associated `Style` type. How does this fit server-side?

**Option A: Ignore styles for v1**
- Simplest: always use default style
- Can add later without breaking changes

**Option B: Style in request metadata**
- Client passes preferred style in tool call args
- Server extracts and uses it

**Option C: Server-side style storage**
- Server maintains style preferences per session/user
- Requires session state management

**Recommendation**: **Option A** for v1. Styles are mostly about prompt wording, which we're simplifying anyway with JSON schema.

## Implementation Phases

### Phase 1: Basic JSON Schema Elicitation 🎯 START HERE

**Goal**: Get simple structs working with JSON parsing.

**Scope**:
- Primitive types (String, i32, bool, etc.)
- Simple structs with primitive fields
- JSON schema generation from type info
- Single-turn interaction (one prompt → one response)

**Steps**:
1. Add `serde` and `schemars` dependencies to elicitation_derive
2. Generate JSON schema in derive macro (use `schemars` crate)
3. Create prompt: "Provide value as JSON: {schema}"
4. Call `peer.create_message()` with prompt
5. Parse response with `serde_json::from_str::<Self>()`
6. Return result or error

**Code Skeleton**:
```rust
#[elicitation::rmcp::tool]
pub async fn elicit_checked(
    peer: elicitation::rmcp::service::Peer<elicitation::rmcp::service::RoleServer>,
) -> Result<Self, elicitation::ElicitError>
where
    Self: serde::de::DeserializeOwned,
{
    // 1. Generate JSON schema (compile-time via derive macro)
    let schema = #schema_json;
    
    // 2. Create prompt
    let prompt = format!(
        "Please provide a value for {} as JSON matching this schema:\n{}",
        #type_name_str, schema
    );
    
    // 3. Send to client
    let params = elicitation::rmcp::model::CreateMessageRequestParams {
        messages: vec![elicitation::rmcp::model::SamplingMessage {
            role: elicitation::rmcp::model::Role::User,
            content: elicitation::rmcp::model::Content::text(&prompt),
        }],
        system_prompt: Some("You are helping elicit structured data. Respond with valid JSON only.".to_string()),
        max_tokens: 1000,
        // ... other fields
    };
    
    let result = peer.create_message(params).await
        .map_err(|e| elicitation::ElicitError::new(
            elicitation::ElicitErrorKind::Service(e.into())
        ))?;
    
    // 4. Extract text
    let text = match result.message.content {
        elicitation::rmcp::model::Content::Text { text } => text,
        _ => return Err(/* InvalidFormat */),
    };
    
    // 5. Parse JSON
    let value: Self = elicitation::serde_json::from_str(&text)
        .map_err(|e| elicitation::ElicitError::new(
            elicitation::ElicitErrorKind::Json(e.into())
        ))?;
    
    Ok(value)
}
```

**Testing Strategy**:
- Unit tests with mock responses
- Integration tests with botticelli
- Start with simple Config struct

**Success Criteria**:
- Can elicit simple structs from JSON
- Errors provide actionable feedback
- Works with both human and agent clients

### Phase 2: Validation & Retry

**Goal**: Add contract validation and error recovery.

**Scope**:
- Validate constructed values (if validation traits exist)
- Retry on parse/validation errors
- Provide helpful error messages back to client
- Cancellation support

**Approach**:
```rust
loop {
    match try_elicit(&peer, &schema).await {
        Ok(value) => {
            // Validate if type has validation
            if let Err(e) = validate(&value) {
                // Send error back, ask to retry
                continue;
            }
            return Ok(value);
        }
        Err(e) => {
            // Ask user: retry or cancel?
            // Handle accordingly
        }
    }
}
```

**Challenges**:
- How to detect if type has validation?
- How to extract validation errors for display?
- When to give up (max retries?)

### Phase 3: Nested Types & Collections

**Goal**: Support complex nested structures.

**Scope**:
- Nested structs
- Vectors, HashMaps
- Optional fields
- Recursive structures

**Strategy**:
- Leverage serde's existing nested deserialization
- JSON handles nesting naturally
- Schema generation recurses automatically

**Challenges**:
- Large schemas might be overwhelming
- Consider breaking into multi-turn for deep nesting

### Phase 4: Advanced Features

**Goal**: Polish and optimization.

**Scope**:
- Style system integration (if needed)
- Streaming responses (for long prompts)
- Context management (multi-turn conversations)
- Caching (remember previous values in session)
- Provider-specific optimizations

## Testing Strategy

### Unit Tests (Mock)
```rust
#[test]
async fn test_elicit_simple_struct() {
    let mock_peer = MockPeer::new()
        .expect_create_message()
        .return_json(r#"{"host": "localhost", "port": 8080}"#);
    
    let config = Config::elicit_checked(mock_peer).await?;
    assert_eq!(config.host, "localhost");
    assert_eq!(config.port, 8080);
}
```

### Integration Tests (Real MCP)
- Test with botticelli server
- Human client tests (manual)
- Agent client tests (Claude via rmcp)

### Property Tests
- Generate random valid structs
- Serialize to JSON
- Ensure round-trip works

## Dependencies

### New Cargo Dependencies
```toml
[dependencies]
schemars = "0.8"  # JSON Schema generation
```

Already have:
- serde (for serialization)
- serde_json (for JSON parsing)
- rmcp (for MCP protocol)

## Migration Strategy

### For Existing Code

**No breaking changes** - this is purely additive:
- Old code doesn't use `elicit_checked()` → no impact
- Old client-side `Elicitation::elicit()` → unchanged
- New server-side tools are opt-in via `elicit_router!` macro

### For Botticelli

1. Add `elicit_router!` macro with desired types
2. Combine with existing tool routers
3. Test with simple types first
4. Gradually add more complex types

```rust
// In botticelli:
elicit_router! {
    pub BotticelliElicitRouter: 
        ServerConfig,
        UserProfile,
        CommandArgs
}

let router = 
    BotRouter::tool_router() +
    CacheRouter::tool_router() +
    BotticelliElicitRouter::tool_router();
```

## Open Questions & Future Work

### Questions
1. **Session management**: How to maintain state across multiple elicit calls?
2. **Cancellation**: How does user cancel elicitation mid-flow?
3. **Defaults**: Should we support default values for optional fields?
4. **Schema customization**: How to override schema generation per-field?
5. **Localization**: Support non-English prompts?

### Future Enhancements
- Visual editors (for complex types in GUI clients)
- Auto-suggest (based on context/history)
- Batch elicitation (multiple types in one conversation)
- Import from file (TOML, JSON, YAML)
- Voice input support (transcription → parsing)

## Success Metrics

### Phase 1 Success
- [ ] Can elicit Config struct with 3+ fields
- [ ] JSON parsing works reliably
- [ ] Error messages are helpful
- [ ] Works in botticelli integration tests

### Overall Success
- [ ] 80% of botticelli types work with elicit_checked()
- [ ] Users prefer it over manual JSON editing
- [ ] Less than 5% retry rate on parse errors
- [ ] Positive feedback from beta testers

## Timeline (Rough)

- **Phase 1** (JSON Schema): 1-2 days coding + testing
- **Phase 2** (Validation): 1 day
- **Phase 3** (Nested): 1 day
- **Phase 4** (Polish): Ongoing

**Total**: ~1 week to production-ready basic implementation

## Conclusion

The architectural foundation is solid. Implementation is straightforward JSON schema + parsing for v1. Start with Phase 1, iterate based on real usage in botticelli.

Key principle: **Incremental delivery**. Each phase adds value independently. Ship Phase 1, gather feedback, adjust before Phase 2.
