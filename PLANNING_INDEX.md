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

### Implementation Plan (.claude/plans/scalable-drifting-lobster.md)
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
