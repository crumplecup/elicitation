# Verus Verification Guide

This guide covers how to write, run, and understand the Verus formal
verification proof suite for `elicitation`.

**Current status: 293 proofs registered, 623 verification goals discharged.**
The proof suite includes substantive arithmetic proofs (ui_types), shadow
struct field-preservation proofs (egui, ratatui composites), and boolean
stub proofs (select enums, float-field types).

---

## What Is Verus?

[Verus](https://github.com/verus-lang/verus) is an SMT-based verification
tool for Rust. It uses a custom Rust compiler fork to check `requires` /
`ensures` contracts at compile time via Z3. Code is wrapped in `verus! {}`
macro blocks.

Key differences from Creusot:

- Verus uses its own toolchain (not cargo-creusot)
- Contracts use `requires` / `ensures` keywords (not attributes)
- Return values use named binding: `fn f() -> (result: T)`
- No Pearlite — specs are written in a Rust-like language
- Cannot import and verify code from external crates directly

---

## Prerequisites

### Install Verus

```bash
# Clone and build
git clone https://github.com/verus-lang/verus ~/repos/verus
cd ~/repos/verus/source
./tools/get-z3.sh
source ./tools/activate
vargo build --release
```

Set `VERUS_PATH` in `.env`:

```bash
VERUS_PATH=~/.cargo/bin/verus
```

### Verify the Install

```bash
verus --version
```

---

## Running the Proof Suite

### Direct Verification (all modules)

```bash
verus --crate-type=lib crates/elicitation_verus/src/lib.rs
```

### Tracked Run (recommended)

```bash
just verify-verus-tracked                    # Full run, CSV output
just verify-verus-resume results.csv         # Resume (skip passed)
just verify-verus-summary results.csv        # Show statistics
just verify-verus-failed results.csv         # Show failures only
just verify-verus-list                       # List all 293 proofs
```

### Run a Single Module

```bash
verus --crate-type=lib crates/elicitation_verus/src/lib.rs \
  --verify-module egui_types
```

---

## Proof Quality Tiers

Not all proofs are equal. This codebase has three tiers of proof quality.
When writing new proofs, always aim for Tier 1 or Tier 2. **Never submit
Tier 3 boolean stubs for types that can be modeled with shadow structs.**

### Tier 1: Substantive Arithmetic Proofs

Real computation with `requires` preconditions and meaningful `ensures`
postconditions. The solver must reason about arithmetic, overflow, and
control flow.

```rust
/// Element fits within viewport: no overflow.
pub fn verify_no_overflow(
    x: u32, y: u32, w: u32, h: u32, vp_w: u32, vp_h: u32,
) -> (result: bool)
    requires
        x as int + w as int <= vp_w as int,
        y as int + h as int <= vp_h as int,
        x as int + w as int <= u32::MAX as int,
        y as int + h as int <= u32::MAX as int,
    ensures result == true,
{
    (x + w) <= vp_w && (y + h) <= vp_h
}
```

**When to use:** Domain invariants, arithmetic properties, overflow checks,
boundary conditions. Found in `ui_types.rs`, `primitives.rs`.

### Tier 2: Shadow Struct Proofs

Define a Verus-native struct that models the field layout of a foreign type.
Prove field-preservation properties on the shadow struct. The solver
verifies that construct → read → reconstruct preserves all field values.

```rust
pub struct ShadowPadding {
    pub left: u16,
    pub right: u16,
    pub top: u16,
    pub bottom: u16,
}

pub fn make_padding(left: u16, right: u16, top: u16, bottom: u16)
    -> (result: ShadowPadding)
    ensures
        result.left == left,
        result.right == right,
        result.top == top,
        result.bottom == bottom,
{
    ShadowPadding { left, right, top, bottom }
}

/// Roundtrip: construct → read fields → reconstruct preserves values.
pub fn verify_padding_roundtrip(left: u16, right: u16, top: u16, bottom: u16)
    -> (result: ShadowPadding)
    ensures
        result.left == left,
        result.right == right,
        result.top == top,
        result.bottom == bottom,
{
    let original = make_padding(left, right, top, bottom);
    make_padding(original.left, original.right, original.top, original.bottom)
}

/// Concrete values verify specific construction.
pub fn verify_padding_concrete() -> (result: ShadowPadding)
    ensures
        result.left == 1u16,
        result.right == 2u16,
        result.top == 3u16,
        result.bottom == 4u16,
{
    make_padding(1, 2, 3, 4)
}
```

**When to use:** Any foreign composite type with integer fields (u8, u16,
i8, etc.). The shadow struct models our wrapper's field layout. We trust
that the shadow matches the real type; the solver verifies our wrapper
logic.

**Currently verified shadow types:**

| Shadow Struct | Models | Fields |
|--------------|--------|--------|
| `ShadowColor32` | `egui::Color32` | r, g, b, a: u8 |
| `ShadowCornerRadius` | `egui::CornerRadius` | nw, ne, sw, se: u8 |
| `ShadowMargin` | `egui::Margin` | left, right, top, bottom: i8 |
| `ShadowPadding` | `ratatui::Padding` | left, right, top, bottom: u16 |
| `ShadowRatatuiMargin` | `ratatui::Margin` | horizontal, vertical: u16 |

### Tier 3: Boolean Stubs (last resort)

A function that takes boolean parameters and returns them. The `ensures`
clause is trivially true. **The solver verifies nothing meaningful.**

```rust
// ❌ This proves NOTHING — it's a tautology
pub fn verify_alignment_roundtrip(all_known: bool) -> (result: bool)
    ensures result == all_known,
{
    all_known
}
```

**When this is acceptable:**

- Select enum label matching — Verus cannot inspect string content
- Float-field composites — f32 equality is limited in Verus
- Types where Kani or Creusot already provide real verification

**When this is NOT acceptable:**

- Any type with integer fields — use a shadow struct instead
- Any arithmetic property — use a Tier 1 proof instead
- New types being added — always start with Tier 1 or 2

---

## Writing a New Proof Module

### 1. Create the file

```rust
// crates/elicitation_verus/src/mytype.rs
use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

// Shadow struct for foreign composite type
pub struct ShadowMyType {
    pub field_a: u32,
    pub field_b: u16,
}

pub fn make_my_type(a: u32, b: u16) -> (result: ShadowMyType)
    ensures
        result.field_a == a,
        result.field_b == b,
{
    ShadowMyType { field_a: a, field_b: b }
}

pub fn verify_my_type_roundtrip(a: u32, b: u16) -> (result: ShadowMyType)
    ensures
        result.field_a == a,
        result.field_b == b,
{
    let original = make_my_type(a, b);
    make_my_type(original.field_a, original.field_b)
}

pub fn verify_my_type_concrete() -> (result: ShadowMyType)
    ensures
        result.field_a == 42u32,
        result.field_b == 7u16,
{
    make_my_type(42, 7)
}

} // verus!
```

### 2. Register in `lib.rs`

```rust
// crates/elicitation_verus/src/lib.rs
pub mod mytype;
```

### 3. Register in the runner

```rust
// crates/elicitation/src/verification/verus_runner.rs
// In VerusProof::all():
Self::new("mytype", "verify_my_type_roundtrip"),
Self::new("mytype", "verify_my_type_concrete"),
```

### 4. Verify

```bash
verus --crate-type=lib crates/elicitation_verus/src/lib.rs
```

---

## Verus Syntax Reference

### Function Signature with Contracts

```rust
pub fn my_function(x: u32, y: u32) -> (result: u32)
    requires
        x as int + y as int <= u32::MAX as int,  // precondition
    ensures
        result == x + y,                          // postcondition
{
    x + y
}
```

### Integer Overflow

Verus checks overflow by default. Use `as int` to lift to mathematical
integers in specs:

```rust
requires x as int + y as int <= u32::MAX as int,
```

### Multiple Ensures

Separate with commas — each is checked independently:

```rust
ensures
    result.left == left,     // checked independently
    result.right == right,   // checked independently
```

### Struct Construction

Structs defined inside `verus! {}` blocks work normally:

```rust
pub struct MyStruct { pub x: u32, pub y: u32 }

pub fn make(x: u32, y: u32) -> (result: MyStruct)
    ensures result.x == x, result.y == y,
{
    MyStruct { x, y }
}
```

### Boolean Patterns

```rust
// Conditional ensures
pub fn check(value: bool) -> (result: bool)
    ensures result == !value,
{
    !value
}

// Conjunction in body
pub fn both(a: bool, b: bool) -> (result: bool)
    ensures result == (a && b),
{
    a && b
}
```

---

## Common Errors

### `main function not found`

Verus must be run on `lib.rs`, not individual module files:

```bash
# ❌ Wrong
verus crates/elicitation_verus/src/mytype.rs

# ✅ Correct
verus --crate-type=lib crates/elicitation_verus/src/lib.rs
```

### `overflow of a u32`

Add overflow precondition:

```rust
requires x as int + y as int <= u32::MAX as int,
```

### External crate types not available

Verus cannot import types from external crates (ratatui, egui, etc.).
Use shadow structs to model the type layout within `verus! {}` blocks.

### `use of unresolved module`

The Verus crate depends on `elicitation` but Verus's custom compiler may
not resolve all transitive dependencies. Keep proof files self-contained
within `verus! {}` blocks — don't import elicitation types directly.

---

## Architecture

### Crate Layout

```text
crates/
├── elicitation/                    # Main library
│   └── src/verification/
│       └── verus_runner.rs         # Orchestrator: tracks all 293 proofs
└── elicitation_verus/              # Proof crate (Verus only)
    └── src/
        ├── lib.rs                  # Module declarations
        ├── primitives.rs           # i8–u64, f32/f64, bool, char
        ├── stdlib_collections.rs   # Option, Result, Vec, tuples
        ├── ui_types.rs             # Tier 1 arithmetic proofs
        ├── egui_types.rs           # Shadow structs + select stubs
        ├── ratatui_types.rs        # Shadow structs + select stubs
        ├── clap_types.rs           # Select stubs
        ├── sqlx_types.rs           # Select stubs
        ├── tokio_types.rs          # Select stubs
        └── ...                     # Other modules
```

### What Verus Verifies vs What Other Tools Cover

| Property | Verus | Kani | Creusot |
|----------|-------|------|---------|
| Integer field roundtrips | Shadow structs | Actual types | Logic accessors + extern_spec |
| Arithmetic overflow | `requires` + `as int` | Bounded model checking | `Int` type |
| Select enum labels | Boolean stubs only | Full label iteration | String wall (trusted) |
| Float fields | Boolean stubs only | Actual f32 comparison | Float wall (trusted) |

### Relationship to Kani and Creusot

The three verifiers provide complementary coverage:

- **Kani**: Tests actual library types with bounded model checking —
  the ground truth. If Kani passes, the real code works.
- **Creusot**: Deductive proofs with SMT discharge — strongest formal
  guarantees for types it can model (integers, constructors).
- **Verus**: Shadow struct proofs verify our wrapper logic independently
  of the foreign type implementation. Different solver (Z3 vs Alt-Ergo).

---

## Current Status

| Metric | Value |
|--------|-------|
| Registered proofs | 293 |
| Verification goals | 623 |
| Shadow struct types | 5 |
| Modules | 9 |

### Module Breakdown

| Module | Proofs | Tier | Notes |
|--------|--------|------|-------|
| `egui_types` | 79 | 2+3 | 6 shadow struct proofs, 64 select stubs, 6 float stubs |
| `sqlx_types` | 47 | 3 | All boolean stubs |
| `tokio_types` | 37 | 3 | All boolean stubs |
| `ratatui_types` | 31 | 2+3 | 4 shadow struct proofs, 24 select stubs, 3 boolean stubs |
| `ui_types` | 29 | 1 | Real arithmetic proofs — the gold standard |
| `clap_types` | 26 | 3 | All boolean stubs |
| `external_types` | 25 | 3 | All boolean stubs |
| `primitives` | 13 | 1 | Identity proofs (trivial but real) |
| `stdlib_collections` | 11 | 1 | Option/Result/tuple proofs |

### Hard Walls (remain boolean stubs)

| Category | Reason |
|----------|--------|
| Select enum labels | Verus cannot inspect string content at proof time |
| Float-field composites | f32 equality limited in Verus |
| Third-party runtime logic | Verus cannot import foreign crate code |

---

## Anti-Patterns

### ❌ Boolean Pass-Through for Integer Types

```rust
// WRONG: This proves nothing. Use a shadow struct.
pub fn verify_padding_roundtrip(ok: bool) -> (result: bool)
    ensures result == ok,
{
    ok
}
```

### ❌ Missing Runner Registration

Every `pub fn verify_*` must have a corresponding `Self::new(...)` entry
in `verus_runner.rs`. Unregistered proofs are invisible to the tracking
system.

### ❌ Running Verus on Individual Files

Verus needs `--crate-type=lib` and the crate root (`lib.rs`), not
individual module files.

### ✅ Correct Pattern for New Integer-Field Types

1. Define `ShadowFoo` struct inside `verus! {}`
2. Write `make_foo()` constructor with field-preserving ensures
3. Write `verify_foo_roundtrip()` using make → read → make
4. Write `verify_foo_concrete()` with literal values
5. Register all functions in `verus_runner.rs`
