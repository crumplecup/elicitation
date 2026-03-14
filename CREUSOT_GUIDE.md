# Creusot 0.10.0 Verification Guide

This guide covers how to run, extend, and understand the Creusot formal
verification proof suite for `elicitation`.

**Current status: 240 SMT goals proved** across 11 de-trusting batches. The
proof suite began as all-`#[trusted]` scaffolding and has been progressively
strengthened so that Alt-Ergo/cvc5 discharge real proof obligations for the
majority of contract types. See [Current Status](#current-status) for the
complete picture.

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

### Full Tracked Run

```bash
just verify-creusot-tracked
```

This runs all 26 modules via `cargo creusot` and writes results to
`creusot_verification_results.csv`. Output:

```
[1/26]  🔬 Checking bools...          ✅ PASS (1s)
[2/26]  🔬 Checking chars...          ✅ PASS (0s)
...
[26/26] 🔬 Checking datetimes_jiff... ✅ PASS (19s)

📊 Compilation Summary:
   Total:   26
   Passed:  26 ✅
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

```
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

The proof suite evolved through 11 batches from all-`#[trusted]` scaffolding to
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

```
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

## Current Status

| Metric | Value |
|--------|-------|
| Total modules | 26 |
| Passing (compilation) | 26 ✅ |
| SMT goals proved | **240** |
| De-trusting batches | 11 |
| Creusot version | 0.10.x |

All 26 modules compile. 240 proof obligations are now discharged by Alt-Ergo
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
