# Elicitation Library Implementation Plan

## Overview

Implement a Rust library for conversational elicitation of strongly-typed values via LLM interactions using MCP (Model Context Protocol). Create a workspace with `elicitation` (core) + `elicitation_derive` (proc macros).

**License**: Apache-2.0 OR MIT (dual license for crates.io)
**MCP Client**: pmcp v1.4+ (high-performance, Claude Code compatible)

## Workspace Structure

```
elicitation/
├── Cargo.toml                    # Workspace root
├── justfile                      # Build recipes
├── README.md
├── LICENSE-APACHE
├── LICENSE-MIT
├── crates/
│   ├── elicitation/              # Core library
│   │   ├── src/
│   │   │   ├── lib.rs            # Only mod + pub use
│   │   │   ├── error.rs          # ElicitError + ElicitErrorKind
│   │   │   ├── traits.rs         # Prompt, Elicit
│   │   │   ├── paradigm.rs       # Select, Affirm, Survey, Authorize
│   │   │   ├── primitives/       # Integer, float, bool, String impls
│   │   │   ├── containers/       # Option<T>, Vec<T>
│   │   │   └── mcp/              # Tool schemas, parsing
│   │   └── tests/                # All tests here (no #[cfg(test)])
│   └── elicitation_derive/       # Proc macros
│       ├── src/
│       │   ├── lib.rs
│       │   ├── derive_elicit.rs
│       │   ├── enum_impl.rs      # Enum → Select
│       │   └── struct_impl.rs    # Struct → Survey
│       └── tests/
└── examples/
```

## Core Architecture

### Traits (src/traits.rs)

```rust
pub trait Prompt {
    fn prompt() -> Option<&'static str> { None }
}

pub trait Elicit: Sized + Prompt {
    async fn elicit(client: &pmcp::Client) -> ElicitResult<Self>;
}
```

### Interaction Paradigms (src/paradigm.rs)

- **Select** - Choose from finite options (enum pattern)
- **Affirm** - Yes/no confirmation (bool pattern)
- **Survey** - Multi-field elicitation (struct pattern)
- **Authorize** - Permission policies (future: v0.2.0)

### Error Handling (src/error.rs)

Using derive_more pattern from CLAUDE.md:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
pub enum ElicitErrorKind {
    #[display("Invalid format: expected {expected}, received {received}")]
    InvalidFormat { expected: String, received: String },
    #[display("Out of range: {min} to {max}")]
    OutOfRange { min: String, max: String },
    #[display("MCP tool error: {}", _0)]
    ToolError(String),
    // ... more variants
}

#[derive(Debug, Clone, Display, Error)]
#[display("Elicit error: {} at {}:{}", kind, file, line)]
pub struct ElicitError {
    pub kind: ElicitErrorKind,
    pub line: u32,
    pub file: &'static str,  // ✅ &'static str
}
```

## Implementation Phases

### Phase 1: Foundation

**Files**: Cargo.toml (workspace), error.rs, traits.rs, paradigm.rs, mcp/tools.rs

**Tasks**:

1. Create workspace with both crates
2. Add licenses (Apache-2.0, MIT)
3. Implement error types with derive_more
4. Define core traits (Prompt, Elicit)
5. Define paradigm traits (Select, Affirm, Survey, Authorize)
6. Create MCP tool parameter builders
7. Create justfile with check/test/check-all recipes

**Success**: `just check` passes, zero warnings

### Phase 2: Primitives

**Files**: primitives/integers.rs, primitives/floats.rs, primitives/boolean.rs, primitives/string.rs, mcp/parsing.rs

**Tasks**:

1. Implement macro `impl_integer_elicit!` for i8-i64, u8-u64
2. Implement macro `impl_float_elicit!` for f32, f64
3. Implement bool (Affirm pattern)
4. Implement String
5. Add parsing helpers (parse_integer, parse_bool, parse_string)
6. Add `#[instrument]` to all public functions
7. Write unit tests in tests/primitives_test.rs

**Success**: All primitives compile, mock tests pass, zero clippy warnings

### Phase 3: Containers

**Files**: containers/option.rs, containers/vec.rs

**Tasks**:

1. Implement `Option<T>` generically
2. Implement `Vec<T>` with loop-based elicitation
3. Test nested containers (Option<Vec<i32>>)
4. Add instrumentation

**Success**: Generic containers work for any T: Elicit

### Phase 4: Derive Macros - Enums

**Files**: elicitation_derive/src/enum_impl.rs, elicitation_derive/tests/enum_derive_test.rs

**Tasks**:

1. Set up proc macro crate
2. Implement derive dispatcher (derive_elicit.rs)
3. Generate Select trait for enums
4. Support unit variants only (v0.1.0)
5. Parse `#[prompt]` attribute
6. Generate from_label() method

**Success**: Derived enums compile and implement Select correctly

### Phase 5: Derive Macros - Structs

**Files**: elicitation_derive/src/struct_impl.rs, elicitation_derive/tests/struct_derive_test.rs

**Tasks**:

1. Generate Survey pattern for structs
2. Sequential field elicitation
3. Parse `#[prompt]` and `#[skip]` attributes
4. Test nested structs

**Success**: Derived structs elicit all fields in sequence

### Phase 6: Documentation & Examples

**Files**: README.md, examples/*.rs, lib.rs (crate docs)

**Tasks**:

1. Write comprehensive module documentation
2. Add doctests to all public items
3. Create examples: simple_types, enums, structs, complex_survey
4. Document MCP tool schemas
5. Write README with quickstart

**Success**: `cargo doc` clean, all examples compile and run

### Phase 7: Release Prep

**Tasks**:

1. Run `just check-all` on workspace
2. Run `markdownlint-cli2 "**/*.md"`
3. Verify all CLAUDE.md standards met
4. Optional: Feature-gated API tests (`just test-api`)
5. Tag v0.1.0

**Success**: Ready for crates.io publish

## Critical Files

1. **crates/elicitation/src/error.rs** - Error architecture using derive_more
2. **crates/elicitation/src/traits.rs** - Core API (Prompt, Elicit)
3. **crates/elicitation/src/primitives/integers.rs** - Generic macro pattern
4. **crates/elicitation_derive/src/enum_impl.rs** - Select pattern generation
5. **crates/elicitation_derive/src/struct_impl.rs** - Survey pattern generation

## Key Design Decisions

1. **Trait-based metadata** - No runtime descriptors, fully static
2. **derive_more for errors** - Following CLAUDE.md exactly
3. **Generic macros** - impl_integer_elicit! for type families
4. **pmcp abstraction** - Only interact via pmcp::Client
5. **Async-first** - MCP is inherently async
6. **Feature-gated API tests** - `#[cfg(feature = "api")]` to prevent token waste
7. **lib.rs only mod + pub use** - Following CLAUDE.md module organization

## Testing Strategy

**Tier 1**: Trait bound tests (compile-time verification)
**Tier 2**: Mock-based tests (no API calls, run in standard `cargo test`)
**Tier 3**: API integration tests (feature-gated, run via `just test-api`)

## Out of Scope for v0.1.0

- Authorize paradigm implementation
- `#[alts]` attribute for enum synonyms
- Multi-select and ranked choice paradigms
- Conditional field elicitation
- Custom validation attributes
- Concurrent elicitation
- Session state management

## Dependencies

```toml
[workspace.dependencies]
pmcp = "1.4"
derive_more = { version = "1", features = ["display", "error", "from"] }
tracing = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
serde_json = "1"
syn = { version = "2", features = ["full"] }
quote = "1"
proc-macro2 = "1"
```

## CLAUDE.md Compliance Checklist

- ✅ No `#[cfg(test)]` in source - all tests in tests/
- ✅ Use derive_more::Display + derive_more::Error
- ✅ All public functions have `#[instrument]`
- ✅ Builders only (no struct literals in examples)
- ✅ lib.rs only has mod + pub use
- ✅ Import as `use crate::{Type}`
- ✅ No re-exports between workspace crates
- ✅ Error file fields use `&'static str` not String
- ✅ Never use `#[allow]`
- ✅ Fix all warnings before commit
