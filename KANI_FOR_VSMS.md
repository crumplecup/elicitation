# Kani Verification for Verified State Machines

> **Operational guide.** This document explains how Kani model-checking is
> applied to `VerifiedStateMachine` types in this codebase: what approaches were
> tried, which failed and why, what the current design is, and how to run and
> extend it.
>
> For the architecture of VSMs themselves (layers, traits, type theory), see
> [`VERIFIED_STATE_MACHINES.md`](VERIFIED_STATE_MACHINES.md).
> For the CREUSOT backend, see [`CREUSOT_GUIDE.md`](CREUSOT_GUIDE.md).

---

## 1. The Core Problem — Why Naïve Kani Fails on Enums

Kani is a bounded model checker built on CBMC (C Bounded Model Checker). It
translates Rust MIR into a Boolean satisfiability problem and exhaustively
checks all paths within an **unwind bound**. CBMC must reason about destructors
for every heap-allocated value.

The naïve approach to verifying a state-machine transition is:

```rust
#[cfg(kani)]
#[kani::proof]
fn close_overlay__kani() {
    let state: ArchiveOverlayState = kani::any();  // ← PROBLEM
    let proof = ArchiveOverlayConsistent::kani_proof_credential();
    let _result = close_overlay(state, Established::prove(&proof));
}
```

**This does not work.** `kani::any::<ArchiveOverlayState>()` tells CBMC to
symbolically represent *all possible values* of the enum at once. For enums
with variants that contain heap-allocated fields (`String`, `Vec<T>`,
`Box<T>`), CBMC must reason about all destructors simultaneously. The
destructor drop logic — `match discriminant { V1 => drop_fields_of_v1, V2 =>
drop_fields_of_v2, ... }` — creates an unbounded recursion that CBMC cannot
resolve within finite unwinding bounds, and the harness runs forever.

### What does not fix it

| Attempted approach | Why it fails |
|---|---|
| `#[kani::unwind(N)]` | Sets a hard cap on loop iterations. Stops the hang but **invalidates the proof**: any path that requires more than N unwinds is silently ignored. You prove the "easy" cases only. |
| `kani::assume(discriminant == V)` | Must be applied before the symbolic enum is created. CBMC has already materialised all variants' drop code at assumption-check time. |
| `BoundedArbitrary` trait + manual `impl` | The symbolic-length destructor problem is in CBMC's internal bookkeeping, not in your code's loop. Bounding your iterator does not bound CBMC's drop reasoning. |
| `kani::any_vec::<T, 0>()` on the *state* variant's inner fields | Helps for Vec parameters to *transitions* (see §4), but the problem is the enum discriminant itself being symbolic. |
| `impl kani::Arbitrary for StateEnum` manually with bounded fields | Same root cause: CBMC sees all branches of the match-on-discriminant drop code simultaneously. |

### The solution: one harness per variant

If the discriminant is **concrete**, CBMC sees only one branch of the drop
match. All destructors for the other variants are pruned at CBMC compile time.
There is nothing to unwind.

```rust
// This works — discriminant is known at CBMC-compile time
#[cfg(kani)]
#[kani::proof]
fn close_overlay__kani__overlay_none() {
    let _state: ArchiveOverlayState = ArchiveOverlayState::OverlayNone;
    let proof = ArchiveOverlayConsistent::kani_proof_credential();
    let _result = close_overlay(_state, Established::prove(&proof));
}

// One more harness for each other reachable variant...
#[kani::proof]
fn close_overlay__kani__export_picker_open() {
    let _state: ArchiveOverlayState = ArchiveOverlayState::ExportPickerOpen;
    // ...
}
```

**Naming convention:** `{transition_fn}__kani__{variant_in_snake_case}`

---

## 2. The Machinery — How Per-Variant Harnesses Are Generated

The codebase generates these harnesses automatically. You never write them by
hand. The pipeline has four stages:

```
  1. #[formal_method] on a transition fn
         ↓  (at proc-macro expansion time)
  2. FooTransition::kani_harness_for_variant(variant_name, state_expr)
         ↓  (called from VerifiedStateMachine::transition_harnesses())
  3. #[derive(VerifiedStateMachine)] loops KaniVariantState::kani_variant_constructions()
         ↓  (at cargo build -p elicit_proofs)
  4. build.rs writes src/kani/generated/*.rs + manifest.json
```

### Stage 1: `#[formal_method]`

The `#[formal_method]` attribute macro (`elicitation_derive::formal_method`)
processes each transition function at compile time. It:

- Identifies which parameter is the state enum (the first non-`String`,
  non-`Vec`, non-`Established` parameter).
- Captures the state parameter name, type, and all other parameter bindings as
  strings at macro-expansion time.
- Generates a `FooTransition` companion struct with two methods:
  - `kani_harness() -> TokenStream` — the full harness string (useful for
    non-VSM functions).
  - `kani_harness_for_variant(variant_name: &str, state_expr: &str) -> TokenStream`
    — substitutes `state_expr` in place of `kani::any()` for the state param.

**Critical detail:** The inline `#[kani::proof]` function that `#[formal_method]`
*would* emit is intentionally suppressed (returns `quote!{}` instead of the
`kani` token stream). If it were emitted, `cargo kani` would attempt to compile
it with `kani::any::<StateEnum>()` — the naïve approach that hangs. The
companion struct is still generated; the inline harness is not.

See: `crates/elicitation_derive/src/formal_method.rs` line ~494:
```rust
let _ = kani; // consumed by harness_src above — suppress unused warning
(quote! {}, creusot, verus, companion)
//  ↑ empty — inline kani harness not emitted
```

### Stage 2: `kani_harness_for_variant`

Given `variant_name = "export_picker_open"` and
`state_expr = "ArchiveOverlayState :: ExportPickerOpen"`, this method builds
the harness source string by string-concatenation (not `format!` — format
escaping of `{` in Established credential blocks is fragile):

```
# [cfg (kani)] # [:: kani :: proof] fn close_overlay__kani__export_picker_open () {
    let _state : ArchiveOverlayState = ArchiveOverlayState :: ExportPickerOpen ;
    let proof : Established < ArchiveOverlayConsistent > = { ... } ;
    let _result = close_overlay (_state , proof) ;
}
```

The string is parsed back into a `TokenStream` by `str::parse()`.

### Stage 3: `#[derive(VerifiedStateMachine)]` + `#[derive(KaniVariantState)]`

`#[derive(VerifiedStateMachine)]` generates
`VerifiedStateMachine::transition_harnesses()`, which loops over
`KaniVariantState::kani_variant_constructions()` and calls
`kani_harness_for_variant` for every (transition × variant) pair.

`#[derive(KaniVariantState)]` generates `kani_variant_constructions()` for
the state enum. It returns `Vec<(&'static str, &'static str)>` —
`(snake_variant_name, construction_expr)` pairs.

**Field construction rules** (enforced by `kani_variants.rs`):

| Field type | Generated expression | Rationale |
|---|---|---|
| `Vec<T>` | `::std::vec::Vec::new()` | `kani::any::<Vec<T>>()` is not implemented in Kani ≤0.67; empty Vec avoids symbolic heap |
| `String` | `::std::string::String::new()` | `from_utf8_lossy` introduces a UTF-8 validation loop → unbounded state space |
| `Option<T>` | `None` | `Some(kani::any::<T>())` may fail if T doesn't impl `kani::Arbitrary`; None is always valid |
| anything else | `::kani::any()` | Must implement `kani::Arbitrary`; covers all integer/bool/custom types |

### Stage 4: `build.rs` + `manifest.json`

`crates/elicit_proofs/build.rs` runs as part of `cargo build -p elicit_proofs`.
For each VSM machine it:

1. Calls `Machine::vsm_kani_proof()` to get all harness `TokenStream`s.
2. Deduplicates (same harness may appear from multiple machines).
3. Writes the formatted Rust source to `src/kani/generated/{module}.rs`.
4. Writes `src/kani/generated/manifest.json` listing every harness name
   and its module path.

The manifest is embedded into the `elicit_proofs` binary at compile time via:
```rust
const MANIFEST: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/kani/generated/manifest.json"
));
```

**Regeneration:** `cargo build -p elicit_proofs` (or any full build).
The generated files are committed — they are source-faithful and auditable.

---

## 3. Running Proofs with `elicit_proofs vsm`

The `elicit_proofs` binary (in `crates/elicit_proofs`) provides the `vsm`
subcommand for running, tracking, and summarising VSM Kani proofs.

### Critical invocation flag: `--lib`

```bash
# ✅ Correct
cargo kani -p elicit_proofs --lib --features kani --harness close_overlay__kani__overlay_none

# ❌ Wrong — will fail with "requires the features: runner"
cargo kani -p elicit_proofs --features kani --harness close_overlay__kani__overlay_none
```

The `elicit_proofs` crate has a `[[bin]]` target that requires the `runner`
feature. `cargo kani` without `--lib` tries to compile all targets in the
package, including the binary. The harnesses live in the *library* target;
`--lib` scopes the build to the library and avoids the `runner` feature
requirement. **Forgetting `--lib` causes every harness to fail immediately.**

### Justfile recipes

```bash
# List all 550+ harnesses from the manifest
just verify-vsm-list

# Run all harnesses, 300s timeout each, write CSV
just verify-vsm csv=vsm_results.csv timeout=300

# Run only harnesses matching a substring (e.g., one VSM module)
just verify-vsm-filter archive_panel vsm_panel.csv 300

# Resume after partial run (skips SUCCESS entries in existing CSV)
just verify-vsm-resume vsm_results.csv

# Show pass/fail/timeout counts
just verify-vsm-summary vsm_results.csv

# Show only failing harnesses
just verify-vsm-failed vsm_results.csv
```

### CSV output format

```
Module,Harness,Status,Time_Seconds,Timestamp,Error_Message
kani::generated::archive_overlay,close_overlay__kani__overlay_none,SUCCESS,7,,
kani::generated::archive_overlay,picker_move_down__kani__export_picker_open,FAILED,12,,"VERIFICATION FAILED"
```

Statuses: `SUCCESS`, `FAILED`, `TIMEOUT`, `ERROR`.

---

## 4. Bugs Kani Has Found

The per-variant approach has proven its value by finding real bugs. Because
symbolic `usize` parameters cover the full `0..=usize::MAX` range, Kani
catches arithmetic overflow that deterministic tests miss.

### Integer overflow in "move down" operations

Pattern: `(idx + 1).min(max - 1)` — overflows when `idx == usize::MAX`.

Fixed in three places (all the same mistake):

```rust
// ❌ Overflows at usize::MAX
(idx + 1).min(upper_bound)

// ✅ Saturates instead
idx.saturating_add(1).min(upper_bound)
```

| File | Function | Harness that found it |
|---|---|---|
| `archive/vsm/overlay.rs` | `picker_move_down` | `picker_move_down__kani__export_picker_open` |
| `archive/vsm/overlay.rs` | `saved_browser_down` | `saved_browser_down__kani__saved_browser_open` |
| `archive/vsm/nav.rs` | `move_cursor_down` | `move_cursor_down__kani__nav_ready` |

These were in production code prior to Kani; unit tests did not catch them
because `usize::MAX` is never a realistic test input. Kani's symbolic inputs
made it inevitable.

**Lesson:** Every "increment-then-clamp" operation in `usize` arithmetic should
use `saturating_add`. Review any `x + 1` in index math.

---

## 5. How to Add a New VSM to the Proof Suite

### Step 1: Derive `KaniVariantState` on the state enum

```rust
// In the state enum definition:
#[derive(Debug, Clone, KaniVariantState, /* other derives */)]
pub enum MyState {
    Idle,
    Processing { count: usize },
    Error(String),
}
```

If any variant contains fields that require `kani::Arbitrary` but don't
implement it (e.g., custom structs), add those impls or map them to `None`/
`Vec::new()` via a newtype wrapper.

### Step 2: Derive `VerifiedStateMachine`

```rust
#[derive(VerifiedStateMachine)]
#[vsm(transitions = [start, stop, error_out])]
pub struct MyMachine;
```

The macro infers `type State = MyState` and `type Invariant = MyConsistent`
from the name. Override with `#[vsm(state = ..., invariant = ...)]` if needed.

### Step 3: Add `#[formal_method]` to each transition

```rust
#[formal_method(contracts = [MyConsistent])]
pub fn start(state: MyState, proof: Established<MyConsistent>)
    -> (MyState, Established<MyConsistent>)
{
    (MyState::Processing { count: 0 }, proof)
}
```

### Step 4: Add the machine to `build.rs`

```rust
// In crates/elicit_proofs/build.rs, generate_kani_proofs():
let body = dedup_kani_proofs(&[MyMachine::vsm_kani_proof()]);
for name in extract_harness_names(&body) {
    manifest.push(("kani::generated::my_module".to_string(), name));
}
write_vsm_file(gen_dir, "my_module.rs", "MyMachine", &body);
```

Add a `rerun-if-changed` directive for the source file.

### Step 5: Add the `mod` declaration to `lib.rs`

```rust
// In crates/elicit_proofs/src/kani/generated/mod.rs (or lib.rs):
#[cfg(kani)]
pub mod my_module;
```

### Step 6: Rebuild and verify a canary

```bash
cargo build -p elicit_proofs          # regenerates manifest
cargo kani -p elicit_proofs --lib --features kani --harness start__kani__idle
```

---

## 6. Proof Suite Counts (as of last generation)

| Module | Harnesses | Notes |
|---|---|---|
| `archive_connection` | 43 | All pass |
| `archive_nav` | 37 | All pass; 1 overflow fixed |
| `archive_overlay` | 56 | All pass; 2 overflows fixed |
| `archive_panel` | ~414 | Largest suite; verification ongoing |
| **Total** | **~551** | Embedded in `manifest.json` |

---

## 7. Architecture Decision Record

### Why not `impl kani::Arbitrary for StateEnum`?

A handwritten `impl kani::Arbitrary` typically looks like:
```rust
impl kani::Arbitrary for ArchiveOverlayState {
    fn any() -> Self {
        match kani::any::<u8>() % 7 {
            0 => Self::OverlayNone,
            1 => Self::ExportPickerOpen,
            ...
        }
    }
}
```

This creates a `match` on a symbolic `u8`. CBMC must then propagate the
symbolic discriminant into every caller's drop code for the enum. The
destructor problem recurs. The per-variant approach eliminates it by making the
choice before CBMC enters the harness.

### Why not `#[kani::unwind(N)]` with a large N?

`#[kani::unwind(5)]` is what the agent tried first (and the user immediately
rejected as counterproductive). An unwind bound tells CBMC: "if a loop exceeds
N iterations, assume this execution doesn't happen." This is unsound for proofs
— you're not proving the property, you're proving it *given that no loop runs
more than N times*. The overflow bugs found by per-variant Kani would not have
been found with bounded unwinding, because the symbolic `usize::MAX` value
would require more than 5 iterations to propagate through arithmetic.

### Why string concatenation in `kani_harness_for_variant`?

The method builds harness source by string `+` rather than `format!`. The
`Established::prove(&__cred)` block contains literal `{` and `}` characters
(as Rust source braces). In `format!` strings, `{` must be escaped as `{{`.
Since the content is generated by `quote!().to_string()`, which uses Rust
token-stream formatting, the escaping rules are non-obvious. Plain string
concatenation is immune to this.

### Why commit the generated files?

The generated `src/kani/generated/*.rs` and `manifest.json` are committed:

- **Auditability**: reviewers can see exactly what is being verified without
  running `cargo build -p elicit_proofs`.
- **Diff visibility**: a change to a VSM transition appears in the diff of the
  corresponding generated file, not just in the source.
- **Runner stability**: the `manifest.json` is embedded by `include_str!` at
  compile time; committing it means the runner binary is always consistent with
  the most recently built harness set.

---

## 8. Troubleshooting

### All harnesses fail immediately with "requires features: runner"

You are missing `--lib` from the `cargo kani` invocation. The `[[bin]]` target
requires the `runner` feature; the harnesses are in the library. Always use:
```bash
cargo kani -p elicit_proofs --lib --features kani --harness HARNESS_NAME
```

### A new harness hangs or times out

1. **Check for `Vec<T>` or `String` fields in the state variant.** If a variant
   contains a `String` field that `KaniVariantState` maps to `String::new()`,
   but the transition itself takes a `String` parameter, the parameter is
   constructed with `kani::any()` via `from_utf8_lossy` (bounded). If the state
   field is being populated from a symbolic source elsewhere, trace it back.

2. **Check for `kani::any()` on a non-primitive inner type.** If a variant
   field's type does not match Vec/String/Option but also doesn't implement
   `kani::Arbitrary` cleanly, the harness will hang. Add an `impl
   kani::Arbitrary` for the type that uses only bounded primitives.

3. **Check `usize` arithmetic.** Any `x + 1` or `x - 1` on a symbolic `usize`
   will fail; use `saturating_add(1)` / `saturating_sub(1)`.

### Harness names in the manifest don't match actual harnesses

Rebuild: `cargo build -p elicit_proofs`. The manifest is generated at build
time from the live token streams. If the source changed but the generated
files weren't rebuilt, names will be stale.

### `cargo build -p elicit_proofs` fails with a `KaniVariantState` error

A new field type was added to a state variant that `kani_variants.rs` doesn't
know how to handle. The `field_construction_expr` function in
`elicitation_derive/src/kani_variants.rs` only recognises Vec, String, and
Option. For any other type:

- If it implements `kani::Arbitrary`: it falls through to `:: kani :: any ()`
  and should work.
- If it does not: add a `kani::Arbitrary` impl, or wrap it in `Option<T>`
  (which gets `None`) if it is optional state.

---

## 9. Further Reading

| Document | Location |
|---|---|
| VSM architecture (layers, traits, proofs) | [`VERIFIED_STATE_MACHINES.md`](VERIFIED_STATE_MACHINES.md) |
| `KaniVariantState` derive impl | `crates/elicitation_derive/src/kani_variants.rs` |
| `VerifiedStateMachine` derive impl | `crates/elicitation_derive/src/derive_vsm.rs` |
| `#[formal_method]` harness generation | `crates/elicitation_derive/src/formal_method.rs` |
| VSM runner source | `crates/elicit_proofs/src/vsm.rs` |
| Harness generation (`build.rs`) | `crates/elicit_proofs/build.rs` |
| Archive VSM sources | `crates/elicit_server/src/archive/vsm/` |
| Kani vec boundary research | `crates/elicitation_kani/src/vec_boundary.rs` |
| Justfile recipes | `justfile` (`verify-vsm-*`) |
