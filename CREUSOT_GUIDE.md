# Creusot 0.10.0 Verification Guide

This guide covers how to run, extend, and understand the Creusot formal
verification proof suite for `elicitation`.

**Current status: 281 SMT goals proved** across 13 de-trusting batches. The
proof suite began as all-`#[trusted]` scaffolding and has been progressively
strengthened so that Alt-Ergo/cvc5 discharge real proof obligations for the
majority of contract types. See [Current Status](#current-status) for the
complete picture.

> For Creusot applied specifically to `VerifiedStateMachine` types, see
> [`CREUSOT_FOR_VSMS.md`](CREUSOT_FOR_VSMS.md).

---

## What Is Creusot?

[Creusot](https://github.com/creusot-rs/creusot) is a deductive verification
tool for Rust. It translates Rust code with contracts (`#[requires]`,
`#[ensures]`, `#[invariant]`) into WhyML for discharge via the Why3 platform
and SMT solvers (Alt-Ergo, Z3, CVC5).

The contract language is **Pearlite** — a pure, mathematical subset of Rust
used in spec annotations. Pearlite is evaluated at the type-logic level, not
at runtime.

---

## Prerequisites

### 1. Build Creusot from Source

Creusot 0.10.x is not on crates.io. Clone and build it locally:

```bash
git clone https://github.com/creusot-rs/creusot.git ~/repos/creusot
cd ~/repos/creusot
./INSTALL
```

This installs `cargo-creusot` and `creusot-rustc` into your PATH via rustup.

The `justfile` automates this:

```bash
just setup  # installs creusot among other tools
```

### 2. Verify the Install

```bash
cargo creusot version
# Creusot version: 0.10.x
# Rust toolchain: nightly-YYYY-MM-DD
```

### 3. `creusot-std` as a Path Dependency

`creusot-std` is not published to crates.io. The workspace `Cargo.toml` uses
a path dependency:

```toml
# Cargo.toml (workspace root)
creusot-std = { path = "/home/erik/repos/creusot/creusot-std" }
```

If you clone creusot to a different location, update this path.

---

## Running the Proof Suite

### Quick orientation: two steps

`cargo creusot prove` bundles two distinct operations:

1. **Translation** — `cargo creusot` invokes `creusot-rustc` to translate
   Rust → Why3/COMA. Output goes to `verif/{crate}_rlib/`.
2. **Proving** — `why3find prove` discharges the COMA goals via SMT solvers
   (Alt-Ergo, CVC5, Z3). Proof caches are stored as `verif/**/{fn}/proof.json`.

You can run them together (`cargo creusot prove`) or separately.

### Required environment variables

Why3 and why3find are installed under the Creusot prefix, not in your system
PATH by default. Three variables are required:

```bash
export PATH="${HOME}/.local/share/creusot/bin:${PATH}"
export DUNE_DIR_LOCATIONS="why3find:lib:${HOME}/.local/share/creusot/share/why3find"
export WHY3CONFIG="${HOME}/.config/creusot/why3.conf"
```

Add these to your shell profile or prefix every Creusot command with them.
The `just` recipes already set them.

### Toolchain and binary locations

| Binary | Location |
|---|---|
| `cargo creusot` | via rustup cargo extension |
| `creusot-rustc` | `~/.local/share/creusot/bin/creusot-rustc` |
| `why3find` | `~/.local/share/creusot/bin/why3find` |
| `why3` | `~/.local/share/creusot/bin/why3` |
| Solver config | `~/.config/creusot/why3.conf` |
| Why3find libs | `~/.local/share/creusot/share/why3find/` |
| `creusot-std` source | `~/repos/creusot/creusot-std/` |

### Generating COMA files only (translation without proving)

```bash
cargo creusot -- -p elicitation_creusot
```

This runs the Rust → COMA translation for `elicitation_creusot` and writes
output to `verif/elicitation_creusot_rlib/`. No SMT solving happens. Use this
to check that contracts parse and the crate translates cleanly before running
a slow prove step.

For VSM companions:

```bash
cargo creusot -- -p elicit_proofs
```

### Proving COMA files (SMT solving only)

```bash
PATH="${HOME}/.local/share/creusot/bin:${PATH}" \
DUNE_DIR_LOCATIONS="why3find:lib:${HOME}/.local/share/creusot/share/why3find" \
WHY3CONFIG="${HOME}/.config/creusot/why3.conf" \
why3find prove -p creusot verif/elicitation_creusot_rlib/gallery/level29/*.coma
```

This is the fastest iteration loop when debugging a specific function —
no full cargo build.

### Combined translation + prove (full run)

```bash
PATH="${HOME}/.local/share/creusot/bin:${PATH}" \
DUNE_DIR_LOCATIONS="why3find:lib:${HOME}/.local/share/creusot/share/why3find" \
WHY3CONFIG="${HOME}/.config/creusot/why3.conf" \
cargo creusot prove -- -p elicit_proofs
```

`cargo creusot prove -- -p PKG` regenerates `verif/PKG_rlib/` then proves
**everything in `verif/`** (both `elicitation_creusot_rlib/` and
`elicit_proofs_rlib/`). The `-p` flag controls which crate is re-translated,
not which COMA files are proved.

### Checking compilation without proving

```bash
just check elicit_proofs
just check elicitation_creusot
```

This compiles with the regular Rust toolchain (no Creusot, no Why3). Use
this to verify contracts parse, imports resolve, and Rust types are correct
before spending time on a full prove run.

### Inspecting a failure

When `why3find` reports a goal unproved, find the COMA file:

```bash
ls verif/elicit_proofs_rlib/creusot/generated/archive_nav/
# nav_loaded_creusot.coma   nav_loaded_creusot/proof.json  ...

cat verif/elicit_proofs_rlib/creusot/generated/archive_nav/nav_loaded_creusot.coma
```

Look for `{false}` in a value binding — the signature of an unmodeled call:

```
s6 = {false} any    (* String::new() has no model *)
```

Cross-reference the binding number (`s6`) with the surrounding let-chain to
find which source expression produced it.

### Stale COMA cache

Creusot uses `target/creusot/` separately from `target/`. Stale fingerprints
can cause COMA files not to regenerate after source changes:

```bash
rm -rf target/creusot/debug/.fingerprint/elicit_proofs-*
rm -rf target/creusot/debug/deps/libelicit_proofs*
```

`cargo clean -p elicit_proofs` only cleans the regular target, not the
Creusot target. Always delete fingerprints manually when debugging missing
or stale `.coma` files.

### Full Tracked Run (elicitation_creusot modules)

```bash
just verify-creusot-tracked
```

This runs all 28 modules via `cargo creusot` and writes results to
`creusot_verification_results.csv`. Output:

```text
[1/28]  🔬 Checking bools...          ✅ PASS (1s)
[2/28]  🔬 Checking chars...          ✅ PASS (0s)
...
[28/28] 🔬 Checking datetimes_jiff... ✅ PASS (19s)

📊 Compilation Summary:
   Total:   28
   Passed:  28 ✅
   Failed:  0  ❌
```

### View Summary of Last Run

```bash
just verify-creusot-summary
```

### List All Modules

```bash
just verify-creusot-list
```

### Run a Single Feature Manually

```bash
cargo creusot -- -p elicitation_creusot
cargo creusot -- -p elicitation_creusot --features uuid
cargo creusot -- -p elicitation_creusot --features regex
```

The `--` separates `cargo creusot` arguments from the cargo build arguments
passed through to the underlying build.

---

## Proof Suite Architecture

### Crate Layout

```text
crates/
├── elicitation/                   # Main library (contract types)
│   └── src/verification/
│       ├── creusot_runner.rs      # Orchestrator: runs & tracks all modules
│       └── types/                 # Shared verification types
│           └── uuid_bytes.rs      # Raw byte types (cfg(any(kani, creusot)))
└── elicitation_creusot/           # Proof crate (Creusot only)
    └── src/
        ├── lib.rs                 # Module declarations
        ├── logic_fns.rs           # Trusted logic wrappers (critical)
        ├── bools.rs               # Core proofs (10 modules)
        ├── chars.rs
        ├── ...
        ├── ipaddr_bytes.rs        # Byte-level proofs (5 + 1 unix)
        ├── macaddr.rs
        ├── ...
        └── uuids.rs               # Feature-gated proofs (10 modules)
        └── uuid_bytes.rs
        └── ...
```

### The `elicitation_creusot` Crate

This crate exists solely for Creusot verification. It has no runtime code.
Key points:

- `creusot-std` as a dependency triggers Creusot's `ToWhy` translation mode
- The main `elicitation` crate must NOT depend on `creusot-std` (it contains
  async code which cannot be translated to WhyML)
- All proof functions are gated with `#[cfg(creusot)]`
- Feature-gated modules mirror the features in `elicitation`

### Module Categories

| Category | Count | Description |
|----------|-------|-------------|
| Core contract modules | 10 | `bools`, `chars`, `collections`, `durations`, `floats`, `integers`, `networks`, `paths`, `strings`, `tuples` |
| Byte-level wrappers | 6 | `ipaddr_bytes`, `macaddr`, `mechanisms`, `socketaddr`, `utf8`, `pathbytes` (unix-only) |
| Feature-gated | 10 | `uuids`, `uuid_bytes`, `values`, `urls`, `urlbytes`, `regexes`, `regexbytes`, `datetimes_chrono`, `datetimes_time`, `datetimes_jiff` |

---

## How Creusot Dispatches

When `cargo creusot` is run:

1. If the crate being compiled depends on `creusot-std` → **`ToWhy` mode**:
   full WhyML translation, contracts are checked, `#[logic]` functions are
   translated. This is what happens to `elicitation_creusot`.

2. Otherwise → **`WithoutContracts` mode** + `--cfg=creusot`: compiled as
   ordinary Rust with `cfg(creusot)` set. This is what happens to `elicitation`
   when it is compiled as a dependency of `elicitation_creusot`.

This means `elicitation` (which has async code) is safe — it is compiled in
`WithoutContracts` mode and async code is never touched by the WhyML
translator.

---

## The Logic Functions Pattern

### The Core Problem

Pearlite logic context (`#[requires]`, `#[ensures]`) can ONLY call `#[logic]`
functions. Ordinary Rust methods are **program functions** and cannot be
called in logic context, even if they look pure.

```rust
// ❌ COMPILE ERROR: called program function in logic context
#[ensures(result.is_loopback())]
pub fn verify_loopback(addr: Ipv4Bytes) -> bool { ... }
```

### The Solution: `logic_fns.rs`

`logic_fns.rs` contains `#[trusted] #[logic(opaque)]` wrapper functions for
every elicitation method that needs to appear in contract specifications:

```rust
#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn ipv4_is_loopback(_addr: Ipv4Bytes) -> bool {
    dead  // body is unreachable; semantics are trusted
}
```

Then in proof files:

```rust
use crate::*;  // brings logic_fns into scope

#[ensures(ipv4_is_loopback(result))]
pub fn verify_loopback() -> Ipv4Bytes { ... }
```

**Key rules for `logic_fns.rs`:**

- Must use `#[logic(opaque)]` (NOT `#[logic]`) — `dead` requires opaque
- Must use `#[trusted]` — there is no body to verify
- Body must be `dead` (Pearlite's unreachable term)
- All parameters prefixed with `_` (unused)
- Everything gated with `#[cfg(creusot)]`

### Available Logic Wrappers

| Wrapper | Wraps | Type |
|---------|-------|------|
| `ipv4_octets(addr)` | `Ipv4Bytes::octets()` | `→ [u8; 4]` |
| `ipv4_is_loopback(addr)` | `Ipv4Bytes::is_loopback()` | `→ bool` |
| `ipv4_is_unspecified(addr)` | `Ipv4Bytes::is_unspecified()` | `→ bool` |
| `ipv4_is_broadcast(addr)` | `Ipv4Bytes::is_broadcast()` | `→ bool` |
| `ipv4_is_multicast(addr)` | `Ipv4Bytes::is_multicast()` | `→ bool` |
| `ipv6_*` | `Ipv6Bytes` methods | various |
| `mac_*` | `MacAddr` methods | various |
| `v4_port(addr)` | `SocketAddrV4Bytes::port()` | `→ u16` |
| `v6_port(addr)` | `SocketAddrV6Bytes::port()` | `→ u16` |
| `utf8_len(s)` | `Utf8Bytes::len()` | `→ usize` |
| `utf8_is_empty(s)` | `Utf8Bytes::is_empty()` | `→ bool` |
| `path_len(p)` | `PathBytes::len()` | `→ usize` |
| `path_is_empty(p)` | `PathBytes::is_empty()` | `→ bool` |
| `i8pos_inner(v)` | `I8Positive::into_inner()` | `→ i8` |
| `i8pos_get(v)` | `I8Positive::get()` | `→ i8` |

---

## Pearlite Syntax Reference

These patterns come up repeatedly in the proof files.

### The `@` (View/ShallowModel) Operator

Use `@` to convert program-world types to Pearlite math types:

```rust
// Slice/array → Seq<T>
bytes@.len()       // ✓  bytes.len() would be a program function call

// Vec → Seq<T>
options@.len()     // ✓
options@[i@]       // index with usize i also needing @

// usize → Int (mathematical integer)
N@                 // where N is a usize const generic

// Comparison: when mixing usize result and Int
path_len(path)@ == bytes@.len()
```

### `is_ok()` / `is_err()` in Contracts

These are program functions. Use match expressions instead:

```rust
// ❌ Program function
#[ensures(result.is_ok())]

// ✓ Pearlite match
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[ensures(match result { Err(_) => true, Ok(_) => false })]
```

### Quantifiers

```rust
// exists
#[ensures(exists<i: usize> options@[i@] == Some(value))]

// forall
#[ensures(forall<i: usize> i < options@.len() ==> options@[i@].is_some())]
```

### Integer Suffixes

Literals in contracts over usize-typed params work without annotation, but
when mixing with `Int`-valued expressions, be explicit:

```rust
#[requires(bytes@.len() > 0)]       // ✓  0 is Int here
#[requires(bytes@.len() <= MAX_LEN@)]  // ✓  coerce const with @
```

---

## Writing a New Proof Module

### 1. Create the file

```rust
// crates/elicitation_creusot/src/mytype.rs

use crate::*;

#[cfg(creusot)]
use elicitation::MyType;

/// Verify: valid input is accepted
#[cfg(creusot)]
#[requires(/* precondition */)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
pub fn verify_mytype_valid(input: SomeInput) -> Result<MyType, MyError> {
    MyType::new(input)
}
```

### 2. Register in `lib.rs`

```rust
// crates/elicitation_creusot/src/lib.rs
#[cfg(feature = "myfeature")]
mod mytype;
```

For feature-gated modules, add the feature to `Cargo.toml`:

```toml
[features]
myfeature = ["dep:mylib", "elicitation/myfeature"]
```

### 3. Register in the runner

```rust
// crates/elicitation/src/verification/creusot_runner.rs
// In CreusotModule::all():
Self::with_feature("mytype", "myfeature"),
```

### 4. Run it

```bash
cargo creusot -- -p elicitation_creusot --features myfeature
```

---

## Common Errors and Fixes

### `called program function in logic context`

A method in `#[ensures]`/`#[requires]` is not a `#[logic]` function.

**Fix:** Add a `#[trusted] #[logic(opaque)]` wrapper to `logic_fns.rs`.

### `cannot find attribute #[pure]`

`#[pure]` is not a Creusot attribute. Remove it. Use `#[logic]` for pure
math functions or `#[trusted]` for trusted axioms.

### `existential quantification not in logic context`

`exists<...>` is only valid inside `#[ensures]`/`#[requires]`/`#[logic]`
bodies. Move the quantifier into a contract annotation.

### `type annotations needed` / inference failure

Often caused by blanket `From` impls on `Error` types. Add explicit type
annotations or `.map_err(|e| MyError::from(e))`.

### Unresolved import for `elicitation::SomeType`

Types gated with `#[cfg(kani)]` in elicitation are invisible to Creusot.
Fix by extending the gate: `#[cfg(any(kani, creusot))]` in both the type
definition and its pub-use export.

### `no method named X found` on a newtype wrapper

Newtype wrappers (e.g., `UuidV4Bytes(UuidBytes)`) only expose methods
explicitly defined on `impl UuidV4Bytes`. Delegation to the inner type
requires `.get().method()` or `.into_inner().method()`.

### `LD_LIBRARY_PATH` / shared library errors

`cargo creusot` sets `RUSTC=creusot-rustc` but doesn't set `LD_LIBRARY_PATH`.
The `creusot_runner.rs` resolves this automatically by querying
`cargo creusot version` for the toolchain channel and finding the rustup lib
directory. If running manually and hitting this error:

```bash
export LD_LIBRARY_PATH=$(rustup run nightly-YYYY-MM-DD rustc --print sysroot)/lib:$LD_LIBRARY_PATH
```

---

## CI Integration

The Creusot suite requires the locally-built `creusot-std` path dependency,
so it is intentionally excluded from the standard workspace checks:

```bash
just check-all       # excludes elicitation_creusot
just verify-creusot-tracked   # run separately, requires creusot installed
```

On CI, Creusot verification is an optional step gated on `creusot` being in
PATH. The `just status` recipe reports whether Creusot is available.

---

## The De-Trusting Strategy

The proof suite evolved through 13 batches from all-`#[trusted]` scaffolding to
real SMT proofs. The core technique: use `extern_spec!` blocks as **trusted
axioms** about constructor behavior, then remove `#[trusted]` from the
`verify_*` witness functions so the solver discharges real obligations.

### Pattern: extern_spec as trusted axiom

```rust
// extern_specs.rs — trusted axiom about constructor behavior
extern_spec! {
    impl<const MAX_LEN: usize> Utf8Bytes<MAX_LEN> {
        #[ensures(bytes@.len() == 0 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(bytes@.len() > MAX_LEN@ ==> match result { Err(_) => true, Ok(_) => false })]
        fn from_slice(bytes: &[u8]) -> Result<Utf8Bytes<MAX_LEN>, ValidationError>;
    }
}

// utf8.rs — now a REAL proof obligation, not trusted
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
// No #[trusted] — Alt-Ergo discharges this
pub fn verify_utf8_empty_valid() -> Result<Utf8Bytes, ValidationError> {
    Utf8Bytes::from_slice(&[])
}
```

The extern_spec says "I trust the constructor does X". The verify function
says "and here is a witness that satisfies that contract". The solver checks
the witness against the spec — no `#[trusted]` needed on the witness.

### The opaque contract rule

Functions with **no** extern_spec generate opaque Why3 contracts. Calling them
from a verified function produces unprovable obligations even for trivial
postconditions. The fix is always `#[ensures(true)]` at minimum:

```rust
// Without this, any call to infallible_accessor() creates an unprovable goal
extern_spec! {
    impl SocketAddrV4Bytes {
        #[ensures(true)]
        fn ip(&self) -> &Ipv4Bytes;
    }
}
```

### Byte string literal limitation

`b"http"` in a witness function body compiles to a `&'static [u8; 4]` in MIR
— a pointer to a static allocation. Creusot cannot model static allocation
references:

```text
Error: Unsupported constant value: Scalar(alloc) of type &[u8; 4]
```

**Fix:** Replace byte string literals with local char arrays:

```rust
// ❌ Fails: generates Scalar(alloc) in MIR
let bytes = b"http";

// ✅ Works: individual b'x' literals are scalar constants
let bytes = [b'h', b't', b't', b'p'];
let slice: &[u8] = &bytes;
```

Individual `b'x'` character literals (not slices) are scalar constants and
are fully supported by Creusot.

### Float comparison wall

`f32` and `f64` do not implement `OrdLogic` in Creusot's type system. Float
comparisons (`>`, `<`, `>=`, `<=`) cannot appear in `#[ensures]`/`#[requires]`
annotations or extern_spec postconditions. The `float_ops.rs` test in the
Creusot repo marks these as `// FIXME` / `// NO_REPLAY`. All `floats.rs`
functions remain `#[trusted]`.

### String literal content wall

`str::view()` (the `@` operator on `&str`) is `#[logic(opaque)]` in
creusot-std — meaning `"hello"@` has unknown length in Pearlite. Only
`String::new()` (empty via `result@ == Seq::empty()`) is provable. String
witnesses based on non-empty literals (`"hello"`, `"short"`, etc.) remain
`#[trusted]`.

---

## Foreign Type Composite Proofs

Third-party crates (egui, ratatui) define types whose internals are opaque
to Creusot — the solver cannot see through dependency function bodies. To
verify our `From` wrapper logic, we use a **three-layer trusted surface**
pattern that minimises what is axiomatically trusted while enabling real
SMT-verified proofs.

### The Three Layers

```text
Layer 1: Logic accessors   — model field access in the logic domain
Layer 2: Extern specs /    — link constructors to logic accessors
         trusted ctors
Layer 3: Bridge functions  — connect runtime reads to logic accessors
```

#### Layer 1: Logic Accessors

`#[logic(opaque)]` functions that represent field access purely in Pearlite.
The solver treats them as uninterpreted functions — it knows they exist but
cannot inspect their bodies.

```rust
#[trusted]
#[logic(opaque)]
pub fn padding_left(_p: ratatui::layout::Padding) -> u16 {
    dead
}
```

One accessor per field. Return type matches the field type (`u16`, `u8`,
`i8`, etc.).

#### Layer 2: Constructor Contracts

For types with named constructor functions (e.g., `Padding::new()`), use
`extern_spec!` to axiomatise the constructor:

```rust
extern_spec! {
    impl ratatui::layout::Padding {
        #[ensures(padding_left(result) == left
               && padding_right(result) == right
               && padding_top(result) == top
               && padding_bottom(result) == bottom)]
        fn new(left: u16, right: u16, top: u16, bottom: u16) -> Self;
    }
}
```

For types constructed via **struct literals** (no constructor function),
Creusot cannot reason about foreign struct field assignment. Use a trusted
constructor wrapper instead:

```rust
#[trusted]
#[ensures(corner_radius_nw(result) == nw && corner_radius_ne(result) == ne
       && corner_radius_sw(result) == sw && corner_radius_se(result) == se)]
pub fn make_corner_radius(nw: u8, ne: u8, sw: u8, se: u8) -> egui::CornerRadius {
    egui::CornerRadius { nw, ne, sw, se }
}
```

#### Layer 3: Bridge Functions

Trusted program functions that connect runtime field reads to logic
accessors. These allow verified proof functions to read fields and have the
solver reason about the result.

```rust
#[trusted]
#[ensures(padding_left(*p) == result)]
pub fn read_padding_left(p: &ratatui::layout::Padding) -> u16 {
    p.left
}
```

Key: bridge functions take `&T` references, but logic accessors take values.
Use `*p` (dereference) in the `#[ensures]` clause to match types.

### Non-Trusted Proof Functions

With all three layers in place, the actual proof functions need **no**
`#[trusted]` annotation — the SMT solver verifies them:

```rust
/// Prove Padding roundtrip: construct → read fields → reconstruct.
#[requires(true)]
#[ensures(padding_left(result) == left && padding_right(result) == right
       && padding_top(result) == top && padding_bottom(result) == bottom)]
pub fn verify_ratatui_padding_roundtrip(
    left: u16, right: u16, top: u16, bottom: u16,
) -> ratatui::layout::Padding {
    let original = ratatui::layout::Padding::new(left, right, top, bottom);
    let l = read_padding_left(&original);
    let r = read_padding_right(&original);
    let t = read_padding_top(&original);
    let b = read_padding_bottom(&original);
    ratatui::layout::Padding::new(l, r, t, b)
}
```

The proof **inlines** the wrapper logic (field reads + constructor calls)
instead of calling `From::from()`, because `From::from()` is an external
function call that Creusot cannot see through.

### Type Comparison in Ensures

- Logic accessors return the program type (e.g., `u16`), not `Int`
- Comparing two values of the same type: `padding_left(result) == left` ✓
- Comparing logic result to an integer literal: `padding_left(result)@ == 1`
  ✓ (use `@` to convert both sides to `Int`)
- Bridge functions: `#[ensures(padding_left(*p) == result)]` — both `u16`,
  no `@` needed

### What Can Be Non-Trusted

| Field type | Can de-trust? | Reason |
|-----------|---------------|--------|
| `u8`, `u16`, `u32`, `u64` | ✅ Yes | Integer fields fully supported |
| `i8`, `i16`, `i32`, `i64` | ✅ Yes | Signed integers supported |
| `f32`, `f64` | ❌ No | No `OrdLogic`, float comparison opaque |
| `String`, `&str` | ❌ No | `str::view()` opaque |
| Bitflags | ❌ No | `contains()` is opaque |

### Currently Verified Foreign Composites

| Type | Fields | Goals | Notes |
|------|--------|-------|-------|
| ratatui `Padding` | `u16` ×4 | 8 | `Padding::new()` via extern_spec |
| ratatui `Margin` | `u16` ×2 | 6 | `Margin::new()` via extern_spec |
| egui `Color32` | `u8` ×4 | 8 | `from_rgba_unmultiplied()` via extern_spec |
| egui `CornerRadius` | `u8` ×4 | 8 | Struct literal via trusted ctor |
| egui `Margin` | `i8` ×4 | 8 | Struct literal via trusted ctor |
| **Total** | | **38** | |

### Creusot Caching Gotcha

Creusot uses its own target directory (`target/creusot/`), separate from the
regular cargo target. When modifying proof source files, `.coma` files may
not regenerate due to stale fingerprints. Force recompilation:

```bash
rm -rf target/creusot/debug/.fingerprint/elicitation_creusot-*
rm -rf target/creusot/debug/deps/libelicitation_creusot*
cargo creusot -- -p elicitation_creusot --features <feature>
```

`cargo clean -p elicitation_creusot` only cleans the **regular** target,
not the Creusot target. Always delete fingerprints manually when debugging
missing `.coma` files.

---

## Current Status

| Metric | Value |
|--------|-------|
| Total modules | 32 |
| Passing (compilation) | 32 ✅ |
| SMT goals proved | **281** |
| De-trusting batches | 13 |
| Creusot version | 0.10.x |

All 32 modules compile. 281 proof obligations are now discharged by Alt-Ergo
rather than accepted on trust.

### Proved modules (real SMT proofs)

| Module | Goals | Notes |
|--------|-------|-------|
| `bools` | ✅ all | First de-trusted module |
| `chars` | ✅ all | |
| `integers` | ✅ all | 47 functions |
| `durations` | ✅ all | |
| `ipaddr_bytes` | ✅ all | Uses logic_fns for octet/loopback wrappers |
| `macaddr` | ✅ all | |
| `mechanisms` | ✅ all | |
| `http` | ✅ all | StatusCodeValid behind `reqwest` feature |
| `utf8` | ✅ all | |
| `networks` | ✅ all | IPv4/IPv6 private/public/loopback |
| `socketaddr` | ✅ all | Accessors via `#[ensures(true)]` extern_specs |
| `urlbytes` | ✅ all | Byte literals rewritten to char arrays |
| `pathbytes` | ✅ all | |
| `regexbytes` | ✅ all | |
| `collections` | partial | Box/Arc/Rc/Array proved; HashMap/BTree/LinkedList trusted |
| `strings` | partial | `String::new()` invalid case proved; literals opaque |
| `tuples` | ✅ all | All `#[ensures(true)]` — trivially proved |
| `ratatui_types` | partial | Padding/Margin roundtrip+concrete proved (14 goals); select enums, Style, Borders trusted (string/bitflag opacity) |
| `egui_types` | partial | Color32/CornerRadius/Margin roundtrip+concrete proved (24 goals); float composites, select enums trusted |

### Hard walls (remain `#[trusted]`)

| File | Count | Reason |
|------|-------|--------|
| `serde_boundary.rs` | 35 | No formal model for `serde_json` deserialization |
| `floats.rs` | 12 | `f32`/`f64` missing `OrdLogic` in Creusot |
| `collections.rs` | 11 | `HashMap`/`BTreeMap`/`LinkedList` — no `ShallowModel` in creusot-std |
| `urls.rs` | 10 | Runtime URL parsing (url crate) |
| `regexes.rs` | 10 | Runtime regex compilation |
| `paths.rs` | 8 | Filesystem existence checks |
| `datetimes_*` | 14 | Runtime datetime comparison |
| `values.rs` | 6 | `serde_json::Value` discriminant — no Why3 model |
| `uuids.rs` | 4 | `Uuid::parse_str` is opaque |
| `strings.rs` | 3 | Non-empty string literals — `str::view()` opaque |
| `ratatui_types.rs` | 28 | Select enum string roundtrips, Style/Modifier bitflags, Borders |
| `egui_types.rs` | ~60 | Select enum string roundtrips, float-field composites (Pos2, Vec2, Rect, Stroke, Shadow, FontId) |
| `clap_types.rs` | 16 | String literal opacity + third-party builder types (see below) |
| `sqlx_types.rs` | 9 | String literal opacity (roundtrips/rejection) + `#[non_exhaustive]` `From` totality; label_count proofs de-trusted |

---

## clap Type Proofs — De-Trusting Opportunities

The `clap_types.rs` module contains 22 proof functions, all currently `#[trusted]`.
There is a partial de-trusting path available for the label count invariants.

### What CAN be de-trusted: label count

Each Select enum proves `labels().len() == options().len()`. This is a pure
length comparison on two `Vec<T>` values. `Vec` has a `ShallowModel` as `Seq<T>`
in creusot-std, so length proofs are tractable.

**De-trusting path:**

Write `extern_spec!` blocks in `extern_specs.rs` specifying the lengths of
`labels()` and `options()` for each type:

```rust
extern_spec! {
    impl elicitation::Select for clap::ColorChoice {
        #[ensures(result@.len() == 3)]
        fn labels() -> Vec<String>;

        #[ensures(result@.len() == 3)]
        fn options() -> Vec<clap::ColorChoice>;
    }
}
```

Then remove `#[trusted]` from `verify_color_choice_label_count()` — Alt-Ergo
will discharge the goal `3 == 3` trivially.

This approach works for all 5 Select enums (ColorChoice=3, ArgAction=8,
ValueSource=3, ErrorKind=17, ValueHint=9 variants) — up to 5 de-trusted goals.

**Caution:** Requires `clap-types` feature in `elicitation_creusot` and the
`extern_spec!` macro to accept trait impls on foreign types. Verify the syntax
compiles before removing `#[trusted]`.

### What CANNOT be de-trusted: string roundtrips

The `from_label` roundtrip and unknown rejection proofs all depend on string
literal matching. This hits the **string literal content wall**:

> `str::view()` is `#[logic(opaque)]` in creusot-std — the solver cannot
> know what `"Auto (detect terminal)"` contains.

Even with an `extern_spec` for `from_label`, you cannot write:

```rust
#[ensures(result.is_some())]  // ❌ Cannot prove — string content opaque
fn from_label(label: &str) -> Option<Self>;
```

These 16 proof functions remain `#[trusted]` until creusot-std provides a
`ShallowModel` for `&str` that exposes character content to the solver.

### Trusted builder type axioms

The 6 `verify_clap_*_trusted()` functions are explicit architectural decisions
that clap builder types (`Arg`, `ArgGroup`, `Command`, `Id`, `PossibleValue`,
`ValueRange`) are trusted third-party types. These will never be de-trusted —
they are axioms by design, not implementation gaps.
