# Creusot Verification Tracking System

This document describes the Creusot proof infrastructure for the elicitation project.

## Overview

The Creusot verification system provides:

- **240 SMT goals** discharged by Alt-Ergo/cvc5 across 26 modules (up from all-`#[trusted]` scaffolding)
- **Module-level tracking** — CLI runner tracks compilation status for each of 26 modules
- **CSV output** — Verification results with timestamps and timing data
- **Progressive de-trusting** — `extern_spec!` axioms replace `#[trusted]` on witness functions
- **Cloud of assumptions** — trust stdlib and validation libraries; prove wrapper contracts
- **Compositional verification** — user types inherit proofs automatically
- **Feature-gated coverage** — optional proofs for uuid, url, regex, datetime types

## Philosophy: Progressive De-Trusting

The proof suite started as all-`#[trusted]` scaffolding (compilation checks
only) and has been progressively strengthened through 11 de-trusting batches.

### Strategy

1. **`extern_spec!` as trusted axioms** — add postcondition contracts to
   constructors that the solver cannot translate (stdlib, external crates)
2. **Remove `#[trusted]` from witness functions** — the solver then discharges
   the witness against the axiom as a real SMT obligation
3. **Hard walls stay trusted** — types without any Why3 model (floats, serde,
   runtime-dependent) remain `#[trusted]`

### What the SMT solver proves

For each de-trusted witness function, Alt-Ergo proves that the concrete call
satisfies the extern_spec postcondition. For example:

```
extern_spec: Utf8Bytes::from_slice — bytes@.len() == 0 ==> Ok
witness:     verify_utf8_empty_valid() calls from_slice(&[])
proof:       &[]@.len() == 0 is true → postcondition satisfied ∎
```

### What remains trusted

- **Stdlib constructors** (`String::new`, `Ipv4Addr::new`, etc.) — the
  extern_spec axioms themselves are trusted
- **Hard-wall types** — floats (`f32`/`f64` missing `OrdLogic`), serde
  deserialization, runtime URL/regex/path/datetime checks, opaque string
  literal content
- **Logic functions** in `logic_fns.rs` — trusted wrappers bridging program
  functions into Pearlite logic context

## Coverage Summary

### Core Contract Modules (10)

Always available, no feature gates:

| Module | Status | Types |
|--------|--------|-------|
| **bools** | ✅ SMT proved | BoolTrue, BoolFalse |
| **chars** | ✅ SMT proved | Alphabetic, Numeric, Alphanumeric |
| **integers** | ✅ SMT proved | All signed/unsigned with Positive/NonNegative/NonZero/Range |
| **strings** | partial | StringNonEmpty — empty case proved; literal cases trusted |
| **floats** | 🔒 trusted | F32/F64 — `f32`/`f64` missing `OrdLogic` in Creusot |
| **durations** | ✅ SMT proved | DurationPositive |
| **tuples** | ✅ SMT proved | Tuple2/3/4 (trivially-true postconditions) |
| **collections** | partial | Vec/Option/Box/Arc/Rc/Array proved; HashMap/BTree/LinkedList trusted |
| **networks** | ✅ SMT proved | IpPrivate, IpPublic, IPv4/IPv6, Loopback |
| **paths** | 🔒 trusted | PathBufExists, IsDir, IsFile — filesystem-dependent |

### Trenchcoat Wrapper Modules (7)

Internal wrappers verifying the stdlib → contract bridge:

| Module | Status | Types |
|--------|--------|-------|
| **ipaddr_bytes** | ✅ SMT proved | Ipv4Bytes, Ipv6Bytes, Ipv4/6 Private/Public |
| **macaddr** | ✅ SMT proved | MacAddr, Multicast/Unicast/Local/Universal |
| **socketaddr** | ✅ SMT proved | SocketAddrV4/V6Bytes, port validators |
| **utf8** | ✅ SMT proved | Utf8Bytes, Utf8NonEmpty, Utf8Bounded |
| **pathbytes** | ✅ SMT proved | PathBytes, PathAbsolute, PathRelative, PathNonEmpty |
| **regexbytes** | ✅ SMT proved | RegexBytes, BalancedDelimiters, ValidCharClass, ValidEscapes |
| **urlbytes** | ✅ SMT proved | UrlBytes, SchemeBytes, AuthorityBytes, UrlAbsoluteBytes, UrlHttpBytes |

### Feature-Gated Contract Modules (7)

| Module | Status | Feature | Types |
|--------|--------|---------|-------|
| **uuids** | 🔒 trusted | `uuid` | UuidNonNil, UuidV4 — `Uuid::parse_str` opaque |
| **values** | 🔒 trusted | `serde_json` | ValueArray/Object/NonNull — no Value discriminant model |
| **urls** | 🔒 trusted | `url` | UrlValid, UrlHttp, UrlHttps — runtime URL parsing |
| **regexes** | 🔒 trusted | `regex` | RegexValid, etc. — runtime regex compilation |
| **datetimes_chrono** | 🔒 trusted | `chrono` | DateTimeUtc* — runtime datetime |
| **datetimes_time** | 🔒 trusted | `time` | OffsetDateTime* — runtime datetime |
| **datetimes_jiff** | 🔒 trusted | `jiff` | Timestamp* — runtime datetime |

### Feature-Gated Trenchcoat Modules (3)

| Module | Status | Feature |
|--------|--------|---------|
| **uuid_bytes** | ✅ SMT proved | `uuid` |
| **http** | ✅ SMT proved | `reqwest` — StatusCodeValid |
| **serde_boundary** | 🔒 trusted | `serde_json` — no formal serde model |

**Total: 240 SMT goals proved. Remaining trusted functions hit genuine hard walls in Creusot 0.10.x.**

## Verification Pattern

Proof functions follow one of two patterns:

### De-trusted (real SMT proof)

```rust
// extern_specs.rs — trusted axiom about constructor behavior
extern_spec! {
    impl<const MAX_LEN: usize> StringNonEmpty<MAX_LEN> {
        #[ensures(value@.len() == 0 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: String) -> Result<StringNonEmpty<MAX_LEN>, ValidationError>;
    }
}

// strings.rs — real proof obligation discharged by Alt-Ergo
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
// No #[trusted] — the solver proves this from the extern_spec axiom
pub fn verify_string_non_empty_invalid() -> Result<StringNonEmpty, ValidationError> {
    StringNonEmpty::new(String::new())
}
```

### Trusted (hard wall)

```rust
// floats.rs — stays trusted: f32/f64 missing OrdLogic in Creusot
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_f32_positive_valid() -> Result<F32Positive, ValidationError> {
    F32Positive::new(42.5)
}
```

## Quick Start

### List All Modules

```bash
# List all 26 Creusot modules
just verify-creusot-list

# Or directly:
cargo run -p elicitation --features cli --bin elicitation -- creusot list
```

### Run All Module Checks

```bash
# Run verification on all modules with CSV tracking
just verify-creusot-tracked

# Or with custom CSV file:
just verify-creusot-tracked creusot_results.csv
```

### View Summary Statistics

```bash
# Show summary from CSV
just verify-creusot-summary

# Or with custom CSV file:
just verify-creusot-summary creusot_results.csv
```

### Check Core Proofs Compile

```bash
# Core proofs only (no features)
cargo check -p elicitation_creusot

# Should complete instantly with zero warnings
```

### Check All Proofs (With Features)

```bash
# All 456 proofs with all features enabled
cargo check -p elicitation_creusot --all-features

# Should complete instantly with zero warnings
```

## Proof Function Pattern

Every contract type has 2+ proof functions in `elicitation_creusot`:

```rust
/// Verify TypeName construction with valid input.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]  // ← Cloud of assumptions: trust stdlib, verify wrapper
pub fn verify_typename_valid() -> Result<TypeName, ValidationError> {
    TypeName::new(valid_input)
}

/// Verify TypeName rejects invalid input.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]  // ← All proofs marked trusted
pub fn verify_typename_invalid() -> Result<TypeName, ValidationError> {
    TypeName::new(invalid_input)
}
```

### Key Annotations

- `#[requires(true)]` - Precondition (unconstrained for trusted proofs)
- `#[ensures(...)]` - Postcondition (documents expected behavior)
- `#[trusted]` - Creusot accepts this proof without verification (cloud of assumptions)

## Compositional Verification Chain

When users derive `Elicit` on their types, they automatically get compositional Creusot proofs:

```rust
// 1. User defines type with contract fields
#[derive(Elicit)]
struct Config {
    timeout: I8Positive,
    retries: U8NonZero,
}

// 2. Derive macro generates (gated with #[cfg(creusot)]):
impl Elicitation for Config {
    fn creusot_proof() {
        I8Positive::creusot_proof();  // → elicitation_creusot::integers
        U8NonZero::creusot_proof();   // → elicitation_creusot::integers
    }
}

// 3. Type system enforces verification chain
// Config is verified ⟹ all fields are verified ⟹ Config verified ∎
```

## Trait Integration

The `Elicitation` trait includes a `creusot_proof()` method:

```rust
pub trait Elicitation {
    // ... other methods ...

    #[cfg(creusot)]
    fn creusot_proof() {
        // Default: witness that type is verified
    }
}
```

- **Contract types**: Override to link to proofs in `elicitation_creusot` crate
- **User types**: Derive macro generates calls to field proofs
- **Composition**: Type system enforces verification chain transitively

## File Organization

```
crates/elicitation_creusot/
├── Cargo.toml          # Features: uuid, serde_json, url, regex, chrono, time, jiff
├── src/
│   ├── lib.rs          # Imports and module declarations
│   ├── bools.rs        # BoolTrue, BoolFalse (4 proofs)
│   ├── chars.rs        # Alphabetic, Numeric, Alphanumeric (6 proofs)
│   ├── integers.rs     # All numeric types (47 proofs)
│   ├── strings.rs      # StringNonEmpty variants (4 proofs)
│   ├── floats.rs       # F32/F64 constraints (12 proofs)
│   ├── durations.rs    # DurationPositive (2 proofs)
│   ├── tuples.rs       # Tuple2/3/4 (6 proofs)
│   ├── collections.rs  # Vec, Option, Result, etc. (26 proofs)
│   ├── networks.rs     # IP address types (12 proofs)
│   ├── paths.rs        # PathBuf validation (8 proofs)
│   ├── uuids.rs        # UUID types [feature: uuid] (4 proofs)
│   ├── values.rs       # JSON value types [feature: serde_json] (6 proofs)
│   ├── urls.rs         # URL types [feature: url] (10 proofs)
│   ├── regexes.rs      # Regex types [feature: regex] (10 proofs)
│   ├── datetimes_chrono.rs  # Chrono types [feature: chrono] (6 proofs)
│   ├── datetimes_time.rs    # Time types [feature: time] (4 proofs)
│   └── datetimes_jiff.rs    # Jiff types [feature: jiff] (4 proofs)
```

## Feature Configuration

### Cargo.toml Features

```toml
[features]
uuid = ["dep:uuid", "elicitation/uuid"]
serde_json = ["dep:serde_json", "elicitation/serde_json"]
url = ["dep:url", "elicitation/url"]
regex = ["dep:regex", "elicitation/regex"]
chrono = ["dep:chrono", "elicitation/chrono"]
time = ["dep:time", "elicitation/time"]
jiff = ["dep:jiff", "elicitation/jiff"]
all = ["uuid", "serde_json", "url", "regex", "chrono", "time", "jiff"]
```

### Testing Feature Combinations

```bash
# Core only
cargo check -p elicitation_creusot

# With UUID support
cargo check -p elicitation_creusot --features uuid

# With datetime support (all three implementations)
cargo check -p elicitation_creusot --features chrono,time,jiff

# All features
cargo check -p elicitation_creusot --all-features
```

## Comparison: Kani vs. Verus vs. Creusot

| Aspect | Kani | Verus | Creusot |
|--------|------|-------|---------|
| **Approach** | Symbolic execution | Executable specs | extern_spec axioms + SMT |
| **Verification Time** | Seconds to minutes | Instant to seconds | Seconds (SMT discharge) |
| **Trust Model** | Verify everything | Verify specs | Axiomatize stdlib, prove wrappers |
| **SMT Goals** | 100+ | 85 | **240** |
| **Proof Complexity** | Medium | Medium | Low–Medium |
| **Hard walls** | Heap aliasing | Linear type limits | Floats, serde, runtime types |

## Benefits of Cloud of Assumptions

### For Users

1. **Fast**: Zero verification time means no waiting for proofs to complete
2. **Simple**: Straightforward pattern, easy to understand what's being verified
3. **Scalable**: Proven to work with 171 types, can easily add more
4. **Compositional**: User types automatically inherit verification

### For the Project

1. **Maintainable**: Simple proofs don't break when dependencies update
2. **Comprehensive**: Easy to achieve 100% coverage of derivable types
3. **Pragmatic**: Focuses verification effort on contract layer, not stdlib
4. **Ecosystem Coverage**: Separate user base from Kani/Verus

### For Formal Methods Adoption

1. **Practical**: Shows formal verification can be pragmatic and scalable
2. **Bridges Gap**: Moves beyond "toy proofs" to production systems
3. **Demonstrates Value**: Compositional verification without complexity explosion
4. **Lowers Barrier**: Makes formal verification accessible to more developers

## Future Work

### Short Term

- ✅ Complete trait integration (DONE)
- ✅ Create compositional example (DONE)
- ✅ Document cloud of assumptions (DONE)

### Medium Term

- Add testing infrastructure for proof compilation
- Create CI/CD integration for proof checking
- Benchmark compilation time with all features

### Long Term

- Explore hybrid approach: trusted wrappers + selected deep verification
- Integration with Creusot's newer features
- Performance profiling and optimization

## Resources

- **Example**: `examples/creusot_compositional_verification.rs`
- **Proof Crate**: `crates/elicitation_creusot/`
- **Contract Types**: `crates/elicitation/src/verification/types/`
- **Derive Macros**: `crates/elicitation_derive/`
- **Creusot Documentation**: <https://github.com/creusot-rs/creusot>

## Summary

The Creusot verification system provides **progressive formal verification** for elicitation:

- **240 SMT goals proved** across 11 de-trusting batches
- **`extern_spec!` axioms** — trusted postconditions on stdlib constructors enable real proofs
- **Axiomatize stdlib, prove wrappers** — trust what can't be modelled, prove what can
- **Compositional** — user types inherit verification automatically
- **Feature-gated** — optional proofs for external integrations
- **Hard walls documented** — floats, serde, runtime types stay trusted with clear rationale

The de-trusting strategy demonstrates that formal verification can advance incrementally from all-trusted scaffolding to genuine SMT proofs, with each batch leaving the repository in a stronger proof state than before.
