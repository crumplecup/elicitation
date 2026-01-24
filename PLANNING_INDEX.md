# Planning Documents

This file tracks all planning documents for the elicitation project.

## Verification Documentation

### UTF8_VERIFICATION_STRATEGY.md
**Status:** Complete  
**Created:** 2026-01-23  
**Purpose:** Explains the multi-layered formal verification strategy for UTF-8 validation using Kani model checker.

**Key Content:**
- Constrained symbolic proofs (2-byte, 3-byte, 4-byte sequences)
- Bounded buffer proofs (2-4096 byte buffers)
- Marginal cost benchmarking methodology
- Verification time measurements and growth analysis (O(N^1.48))
- Completeness argument for production usage

**Key Insights:**
- Constraints reduce 4-byte proof from 4B combinations to 786K (5,461x reduction)
- Growth is O(N^1.48) - sub-exponential, therefore tractable
- Layering prevents combinatorial explosion (prove parts, trust composition)
- 4096-byte proof completes in ~24 hours on commodity hardware
- Pattern generalizes to other complex validation (URL, Regex, etc.)

**Related Files:**
- `crates/elicitation/src/verification/types/utf8.rs`
- `crates/elicitation/src/verification/types/kani_proofs/utf8.rs`
- `crates/elicitation/src/verification/types/kani_proofs/benchmark_marginal.rs`
- `justfile` recipes: `benchmark-kani-marginal`, `benchmark-kani-long`, `kani-long-proofs`

---

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

### DATETIME_IMPLEMENTATION_PLAN.md
**Status**: Active - v0.2.3 feature development
**Created**: 2026-01-19
**Purpose**: Implementation plan for adding `Elicitation` support for the top 3 Rust datetime libraries (chrono, time, jiff) behind feature flags, providing 95%+ ecosystem coverage.

**Motivation**: Support the datetime libraries users already have, not force a specific choice. Feature-gated implementations allow zero-cost opt-in.

**Core Strategy**:
- Support top 3 libraries: chrono (50M downloads/month), time (40M downloads/month), jiff (100K/month growing)
- Shared UX pattern across all: ISO 8601 string OR manual components
- Mutually compatible features (users can enable multiple)
- 95%+ ecosystem coverage with 3 implementations

**Libraries & Types:**

**chrono** (most popular, mature):
- `DateTime<Utc>` - UTC timestamps
- `DateTime<FixedOffset>` - Fixed timezone offset
- `NaiveDateTime` - Timezone-agnostic

**time** (modern, high performance):
- `OffsetDateTime` - With timezone offset
- `PrimitiveDateTime` - No timezone

**jiff** (newest, best ergonomics):
- `Timestamp` - Absolute moment
- `Zoned` - Timestamp + timezone
- `civil::DateTime` - Calendar date + time

**Elicitation UX:**
1. Choose input method: ISO 8601 string OR manual components
2. For ISO: Parse RFC 3339 / ISO 8601 string
3. For components: Elicit year, month, day, hour, minute, second (validated)
4. Construct datetime using library-specific API

**Shared Code Pattern:**
- `datetime_common.rs` - Input method selection, component elicitation, error types
- `datetime_chrono.rs` - chrono implementations
- `datetime_time.rs` - time implementations
- `datetime_jiff.rs` - jiff implementations

**Impact:**
- Unblocks: BotStats and any struct with datetime fields
- Enables: 95%+ of Rust datetime users can derive Elicit
- Maintains: Zero-cost when features not enabled

**Phases:**
1. chrono implementation (2-3 hours) - Priority 1
2. time implementation (1-2 hours) - Priority 2
3. jiff implementation (1-2 hours) - Priority 3
4. Integration & polish (1 hour) - Documentation, CI

**Target**: 5-8 hours, release as elicitation 0.2.3

---

### ELICITATION_STYLE_SYSTEM_PLAN.md
**Status**: Active - v0.2.4+ revolutionary feature development
**Created**: 2026-01-19
**Purpose**: Implementation plan for trait-based style system that separates elicitation behavior (what's asked) from elicitation style (how it's presented).

**Vision**: Enable customizable elicitation UX through pluggable `ElicitationStyle` trait implementations. Users can choose from built-in styles (default, compact, verbose, wizard) or implement custom styles for specialized needs.

**The Innovation**: Instead of hardcoding UX decisions, let styles control prompt formatting, help text, error messages, and selection rendering. Opens door to framework integration (egui, ratatui), domain-specific UX (config files, secrets, web forms), and ecosystem growth.

**Core Architecture**:

**ElicitationStyle Trait:**
```rust
pub trait ElicitationStyle: Send + Sync {
    fn prompt_for_field(&self, field_name: &str, field_type: &str, context: &PromptContext) -> String;
    fn help_text(&self, field_name: &str, field_type: &str) -> Option<String>;
    fn validation_error(&self, field_name: &str, error: &str) -> String;
    fn show_type_hints(&self) -> bool;
    fn select_style(&self) -> SelectStyle;  // Menu vs inline vs search
    fn use_decorations(&self) -> bool;
    fn prompt_prefix(&self) -> &str;
}
```

**Derive Syntax:**
```rust
// Built-in style (string lookup)
#[derive(Elicit)]
#[elicit(style = "compact")]
struct Config { ... }

// Custom style (type reference)
#[derive(Elicit)]
#[elicit(style = MyStyle)]
struct Config { ... }

// Field-level override
#[derive(Elicit)]
#[elicit(style = "default")]
struct Config {
    host: String,
    #[elicit(style = "compact")]
    port: u16,
}
```

**Built-in Styles (0.2.4):**

1. **DefaultStyle** - Balanced, current behavior
   - "Enter host:"
   - Type hints enabled
   - Standard errors

2. **CompactStyle** - Minimal, terse
   - "host:"
   - No type hints
   - Concise errors

3. **VerboseStyle** - Detailed, helpful
   - "Enter host (text, field 1/2)"
   - Extensive help text
   - Friendly errors

4. **WizardStyle** - Step-by-step
   - "Step 1 of 2: Enter host"
   - Progress indicators
   - Decorative elements

**Datetime Integration (0.2.6):**

Apply style pattern to datetime elicitation:
- **Iso8601Style** - ISO string only (fast)
- **ComponentsStyle** - Manual input (guided)
- **SmartDatetimeStyle** - User chooses method

**Ecosystem Potential:**

Third-party styles:
- `elicitation-egui` - GUI widgets
- `elicitation-ratatui` - TUI interface
- `elicitation-web-styles` - HTML forms
- `elicitation-secrets` - Masked input
- `elicitation-config` - TOML/YAML-like

**Implementation Phases:**

**0.2.4** (10-14 hours):
- Core trait + PromptContext
- 4 built-in styles
- StyleRegistry for dynamic lookup
- Derive macro `#[elicit(style = ...)]` support
- Backward compatible (no annotation = DefaultStyle)

**0.2.5** (3-4 hours):
- Field-level `#[elicit(style = ...)]` overrides
- Per-field style selection
- Mixed styles in one struct

**0.2.6** (4-6 hours):
- Datetime-specific styles
- Refactor datetime impls to use styles
- Documentation & examples

**Key Benefits:**

- **Separation of concerns**: UX decoupled from elicitation logic
- **Extensibility**: New styles without touching core
- **Zero-cost abstraction**: Derive generates monomorphized code (no trait objects)
- **Backward compatible**: No annotation = unchanged behavior
- **Ecosystem growth**: Third-party styles for specialized needs
- **Framework integration**: GUI/TUI frameworks can provide styles
- **Innovation**: Opens new paradigms (visual config, web forms, etc.)

**Versioning**: All non-breaking additions = patch bumps (0.2.4, 0.2.5, 0.2.6)

**Timeline**: 17-24 hours across 2-3 weeks

**Impact**: Foundational feature enabling elicitation beyond terminal prompts. Makes library useful for GUI applications, web interfaces, config file generation, and domain-specific tooling.

**Truly innovative** - no other Rust elicitation library has this pattern.

---

### UUID_VERIFICATION.md
**Status**: Complete - UUID byte-level validation
**Created**: 2026-01-22
**Purpose**: Formal verification foundation for UUID types using byte-level validation following RFC 4122.

**Architecture**: Layered validation pattern (same as UTF-8):
- Layer 1: `[u8; 16]` - Raw bytes
- Layer 2: `UuidBytes` - RFC 4122 variant validation (10xx pattern)
- Layer 3: `UuidV4Bytes`/`UuidV7Bytes` - Version-specific constraints
- Layer 4: `UuidV4`/`UuidV7` - High-level contract types (wrap uuid::Uuid)

**What We Prove** (14 Kani proofs, all complete in ~2s each):
1. Variant validation (4 proofs) - RFC 4122 vs NCS/Microsoft/Reserved
2. Version detection (1 proof) - All 16 versions correctly extracted
3. UUID V4 validation (3 proofs) - Construction, wrong version, invalid variant
4. UUID V7 validation (3 proofs) - Construction, wrong version, timestamp extraction
5. Round-trip properties (3 proofs) - Byte preservation

**Why UUID Proofs Are Fast**:
- Fixed 16 bytes (no variable length like UTF-8)
- Bit-level operations only (no loops/memchr)
- Small state space: 64 combinations vs UTF-8's 786K
- Complete symbolic verification in seconds, not days

**Key Innovation**: Trait-based validation approach can be applied to other fixed-format types (IP addresses, MAC addresses, etc.)

---

### KANI_VERIFICATION_PATTERNS.md
**Status**: Complete - Constraint-based byte validation patterns ✅ **VALIDATED**
**Created**: 2026-01-22
**Purpose**: Comprehensive documentation of the constraint-based byte validation pattern that enables fast, tractable formal verification of complex types using Kani.

**Discovery**: By expressing type constraints as byte-level predicates and building layered validation types, we achieved symbolic verification that completes in seconds (not hours) for complex types.

**Pattern**: Constraint-Based Byte Validation
- Layer 1: Fixed-size byte arrays (Kani's native domain)
- Layer 2+: Incremental constraint validation
- Result: Fast proofs (0.04s - 8s) for complex types

**Types Successfully Verified** (74 proofs total, all tractable):
- **UUID** (16 bytes): Bit patterns, 14 proofs, 1-2s each
- **IPv4** (4 bytes): Range checks (RFC 1918), 12 proofs, 2-3s each
- **IPv6** (16 bytes): Bit masks (RFC 4193), 9 proofs, 2-3s each
- **MAC** (6 bytes): Bit flags (unicast/multicast), 18 proofs, 0.07-8s each
- **SocketAddr** (18/22 bytes): Composition (IP + port), 19 proofs, ~2s each
- **PathBuf (Unix)**: UTF-8 + null checks, 2 proofs, ~0.04s each

**Key Discoveries**:
1. Fixed-size types enable bounded exploration
2. Simple predicates (bit masks, ranges, byte comparisons) map to SMT primitives
3. Composition doesn't break tractability
4. Proof time tracks constraint complexity, not type size
5. Manual loops avoid memchr infinite unwinding

**What Works**: Fixed arrays, bit operations, range checks, bounded loops, composition, contract types
**What Struggles**: Vec/String APIs, complex parsing, unbounded iteration

**Pattern Generality**: Works for ANY type expressible as byte-level constraints

**Implementation**: ~2,120 lines of verified validation code across 5 files

### URL_BOUNDED_COMPONENTS.md
**Status**: Complete - URL validation with bounded components ✅ **PARTIALLY VERIFIED**
**Created**: 2026-01-22
**Purpose**: Documents URL validation using bounded component architecture (SchemeBytes, AuthorityBytes, UrlBytes).

**Key Insight**: Unwind bounds must match **actual data size**, not buffer size.
- ❌ Wrong: `MAX_LEN = 32`, `unwind(32)` explores 32 iterations for 4-byte "http"
- ✅ Right: `MAX_LEN = 8`, `unwind(5)` just enough for "http" (4 bytes + validation)

**Architecture**:
```
UrlBytes<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>
├── SchemeBytes<SCHEME_MAX>       // http, https, ftp (proven: 6s)
├── AuthorityBytes<AUTHORITY_MAX>  // example.com:8080 (proven: 6s)
└── Full URL parsing              // Composition (long: 3+ min)
```

**Proof Results**:
- Component proofs: ~6 seconds ✅
- Composition proofs: 3+ minutes (nested validation complexity)

**Lesson**: Component-level proofs are tractable. Full composition hits nested loop complexity but will complete with patience.

### REGEX_VERIFICATION.md
**Status**: Complete - Recursive trait bounds for regex ✅ **FULLY VERIFIED**
**Created**: 2026-01-22
**Purpose**: Documents **recursive trait bound pattern** that makes regex validation tractable through layer-by-layer constraint proving (1.6s - 8.2s per proof).

**Architecture** (Compositional Constraint Validation):
```
Layer 1: Utf8Bytes           → Valid UTF-8 (proven)
Layer 2: BalancedDelimiters  → ( == ), [ == ], { == } (1.6s)
Layer 3: ValidEscapes        → \n, \t, \d, \w valid (2.2s)
Layer 4: ValidQuantifiers    → *, +, ?, {n,m} follow atoms (4.4s)
Layer 5: ValidCharClass      → [...] ranges valid (3.3s)
Layer 6: RegexBytes          → Complete regex (8.2s)
```

**Key Insight**: Narrow the bit space layer-by-layer
- Each layer proves **one constraint** independently
- Composition is linear, not combinatorial
- Type system enforces: Layer N contains Layer N-1

**Proof Results** (23 proofs, all verified):
- Balanced delimiters: 1.6s ✅
- Escape validation: 2.2s ✅
- Quantifier validation: 4.4s ✅
- Character class validation: 3.3s ✅
- Complete regex: 8.2s ✅

**Before vs After**:
- Before: Monolithic validation (3+ minutes, possibly hours)
- After: Layered validation (1.6s - 8.2s per layer)
- Impact: Intractable → tractable through systematic decomposition

**Pattern Generality**: Apply to ANY complex validation with independent constraints
- Identify constraints that don't depend on each other
- Create validation layer per constraint
- Prove each layer independently (seconds)
- Compose via type wrapping (free)

**Files**: 557 lines implementation, 23 Kani proofs, 14 unit tests

**Achievement**: This is the **proof factory** pattern - the recursive application of trait bounds that makes arbitrarily complex validation tractable.

**Impact**: Eliminates unwind hacks for fixed-format types, enables contract-driven validation, foundation for formally verified LLM tool chains

---

TOTAL_VERIFICATION_PLAN.md
