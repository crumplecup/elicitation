# Planning Documents

This file tracks all planning documents for the elicitation project.

## Active Plans

### elicitation_vision.md
**Status**: Complete - Design document
**Created**: 2025-12-28
**Purpose**: Complete vision and architecture design for the elicitation library, including trait-based metadata system, interaction paradigms (Select, Affirm, Survey, Authorize), and implementation patterns for conversational elicitation of strongly-typed Rust values via MCP.

**Key Sections**:
- Core concept and trait architecture
- Interaction paradigms vocabulary
- Trait-based metadata foundation
- Generic implementations for type families
- MCP tool integration patterns
- Derive macro strategies

### implementation_plan.md
**Status**: Active - Implementation guide
**Created**: 2025-12-28
**Purpose**: Phased implementation plan for v0.1.0 covering workspace setup, core traits, primitive implementations, container types, derive macros, documentation, and release preparation.

**Phases**:
1. Foundation - workspace, errors, traits
2. Primitives - integers, floats, bool, String
3. Containers - Option<T>, Vec<T>
4. Derive Macros - Enums (Select pattern)
5. Derive Macros - Structs (Survey pattern)
6. Documentation & Examples
7. Release Prep

**Key Decisions**:
- Dual Apache-2.0 OR MIT license for crates.io
- pmcp v1.4+ for MCP client integration
- derive_more for error handling
- Trait-based metadata (fully static)
- Async-first API

### ELICITATION_FIELD_VARIANTS_PLAN.md
**Status**: Complete - v0.2.1 implementation
**Created**: 2026-01-18
**Implemented**: 2026-01-18
**Purpose**: Detailed implementation plan for extending `#[derive(Elicit)]` to support enum variants with fields (tuple and struct variants), enabling elicitation of complex enum types.

**Core Philosophy**: Values as state machines - every value provokes an elicitation, complex values are state machines where each state represents satisfied information and transitions represent eliciting the next piece.

**Key Features Implemented**:
- Tuple variants: `Variant(T1, T2, ...)`
- Struct variants: `Variant { field1: T1, field2: T2 }`
- Mixed variants in same enum
- Nested enums with recursive elicitation
- Full tracing instrumentation at each state transition
- Two-phase elicitation: variant selection → field elicitation

**Technical Implementation**:
- New structures: `VariantInfo`, `VariantFields`, `FieldInfo`
- `generate_variant_match_arm()` for per-variant code generation
- Enhanced `generate_elicit_impl()` with match-based field elicitation
- Error context preservation for field failures
- Automatic recursion for nested types

### SERDE_JSON_IMPLEMENTATION_PLAN.md
**Status**: Active - v0.2.2 feature development
**Created**: 2026-01-19
**Purpose**: Implementation plan for adding `Elicitation` support for `serde_json::Value` behind a feature flag, enabling conversational elicitation of arbitrary JSON data structures.

**Motivation**: Unblock all Rust types containing `serde_json::Value` (tool arguments, API responses, dynamic config) from using `#[derive(Elicit)]`. Makes elicitation universally useful for the Rust ecosystem.

**Core Design**:
- Feature flag: `serde_json` (zero-cost when disabled)
- State machine per JSON type: null, boolean, string, number, array, object
- Recursive elicitation for nested structures (max depth: 10)
- Delegates to existing primitive elicitation (String, bool, etc.)

**Elicitation Flow**:
1. Type selection: User picks JSON type
2. Variant-specific elicitation:
   - Scalars: Single prompt → terminal
   - Array: Loop adding items (recursive `Value::elicit()`)
   - Object: Loop adding key-value pairs (recursive for values)
3. Terminal: Construct `serde_json::Value`

**Impact**:
- Enables: `ToolCall { arguments: Value }`, `Output::Json(Value)`
- Ecosystem: Any crate with `Value` fields can derive `Elicit`
- Zero overhead without feature flag

**Phases**:
1. Add feature flag and `#[cfg]` gates
2. Implement `value_impl.rs` with depth tracking
3. Integrate module into library
4. Comprehensive test suite (scalars, collections, nesting, limits)
5. Update documentation (README, CHANGELOG, feature guide)
6. Manual testing with example
7. Release v0.2.2

**Target**: 3-5 hours, same-day implementation and release
