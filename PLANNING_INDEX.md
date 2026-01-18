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
