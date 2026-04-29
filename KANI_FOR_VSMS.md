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

### The solution: one harness per variant, three harnesses per depth

If the discriminant is **concrete**, CBMC sees only one branch of the drop
match. All destructors for the other variants are pruned at CBMC compile time.
There is nothing to unwind.

```rust
// This works — discriminant is known at CBMC-compile time
#[cfg(kani)]
#[kani::proof]
fn close_overlay__kani__overlay_none__d0() {
    let _state: ArchiveOverlayState = ArchiveOverlayState::OverlayNone;
    let proof = ArchiveOverlayConsistent::kani_proof_credential();
    let _result = close_overlay(_state, Established::prove(&proof));
}
```

**Naming convention:** `{transition_fn}__kani__{variant_in_snake_case}__d{depth}`
where `depth` ∈ `{0, 1, 2}`.

---

## 2. The Second Problem — Recursive / Collection Field Types

Even with concrete discriminants, variants whose *fields* contain `Vec<T>`
where `T` itself contains `Vec<T>` (recursive types) still cause CBMC to hang.

### Why this happens

CBMC's destructor model is **type-driven**, not value-driven. For
`ExplainNode { children: Vec<ExplainNode> }`, CBMC generates:

```
ExplainNode::drop() → Vec<ExplainNode>::drop() → ExplainNode::drop() → ...
```

Even when `children` is `Vec::new()` (empty), CBMC has already unrolled the
recursive destructor call tree. The hang is in CBMC's internal bookkeeping,
not in your runtime values.

### The solution: compositional depth-bounded instances

Instead of `Vec::new()` (which CBMC models as "zero or more ExplainNodes"),
use concrete instances bounded by explicit depth.

| Depth | Collection field rule |
|---|---|
| 0 | `Vec::new()` — zero elements |
| 1 | `vec![T::kani_depth0()]` — one element, itself at depth-0 |
| 2 | `vec![T::kani_depth0(), T::kani_depth0()]` — two elements, both at depth-0 |

With `vec![leaf]` (depth-1), CBMC unrolls exactly once: `ExplainNode::drop()`
→ `Vec<ExplainNode>::drop()` (one element) → `ExplainNode::drop()` (the leaf,
with `children: Vec::new()` → zero unrolls). Termination is guaranteed.

This is the compositional proof argument:
- **Base case (depth-0):** single ExplainNode with empty children is sound.
- **Inductive step (depth-1, 2):** adding one or two children, each proven
  sound at depth-0, is also sound.
- **By induction:** any finite tree is covered.

### The `KaniCompose` trait

`crates/elicitation/src/kani_compose.rs` defines:

```rust
#[cfg(kani)]
pub trait KaniCompose: Sized {
    fn kani_depth0() -> Self;  // base case: empty collections
    fn kani_depth1() -> Self { Self::kani_depth0() }  // one element
    fn kani_depth2() -> Self { Self::kani_depth1() }  // two elements
}
```

Impls:
- Primitives (`bool`, `u8`, ..., `f64`): all depths → `kani::any::<T>()`
- `String`: all depths → `String::new()` (symbolic strings cause path explosion)
- `Vec<T>`: depth-0 = empty; depth-1 = one element; depth-2 = two elements
- `Option<T>`: depth-0 = `None`; depth-1/2 = `Some(T::kani_depth0())`
- `BTreeMap<K,V>`, `HashMap<K,V,S>`: all depths = empty (no `RandomState::new()`)
- User types: `#[derive(KaniCompose)]` or manual impl

**Important:** `HashMap::new()` calls `RandomState::new()` → `getrandom` syscall
→ Kani cannot model. Always use `BTreeMap` or `HashMap::with_hasher(S::default())`
in Kani contexts. The `ErdLayout` type was changed from `HashMap` to `BTreeMap`
for this reason.

---

## 3. The Machinery — How Per-Variant, Per-Depth Harnesses Are Generated

The codebase generates these harnesses automatically. You never write them by
hand. The pipeline has four stages:

```
  1. #[formal_method] on a transition fn
         ↓  (at proc-macro expansion time)
  2. FooTransition::kani_harness_for_variant_at_depth(variant_name, state_expr, depth)
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
- Generates a `FooTransition` companion struct with:
  - `kani_harness() -> TokenStream` — the full harness string (for non-VSM use).
  - `kani_harness_for_variant_at_depth(variant_name, state_expr, depth) -> TokenStream`
    — substitutes `state_expr` in place of `kani::any()` for the state param,
    and appends `__d{depth}` to the harness function name.

**Critical detail:** The inline `#[kani::proof]` function that `#[formal_method]`
*would* emit is intentionally suppressed (returns `quote!{}` instead of the
`kani` token stream). If it were emitted, `cargo kani` would attempt to compile
it with `kani::any::<StateEnum>()` — the naïve approach that hangs.

See: `crates/elicitation_derive/src/formal_method.rs` line ~494:
```rust
let _ = kani; // consumed by harness_src above — suppress unused warning
(quote! {}, creusot, verus, companion)
//  ↑ empty — inline kani harness not emitted
```

### Stage 2: `kani_harness_for_variant_at_depth`

Given `variant_name = "export_picker_open"`,
`state_expr = "ArchiveOverlayState :: ExportPickerOpen"`, and `depth = 0`,
this method builds the harness source string:

```
# [cfg (kani)] # [:: kani :: proof] fn close_overlay__kani__export_picker_open__d0 () {
    let _state : ArchiveOverlayState = ArchiveOverlayState :: ExportPickerOpen ;
    let proof : Established < ArchiveOverlayConsistent > = { ... } ;
    let _result = close_overlay (_state , proof) ;
}
```

For a variant with `Vec<ExplainNode>` children, depth=1 uses
`<ExplainNode as ::elicitation::KaniCompose>::kani_depth0()` as the element expression.

The string is parsed back into a `TokenStream` by `str::parse()`.

### Stage 3: `#[derive(VerifiedStateMachine)]` + `#[derive(KaniVariantState)]`

`#[derive(VerifiedStateMachine)]` generates
`VerifiedStateMachine::transition_harnesses()`, which loops over
`KaniVariantState::kani_variant_constructions()` and calls
`kani_harness_for_variant_at_depth` at depths 0, 1, 2 for every
(transition × variant) triple.

`#[derive(KaniVariantState)]` generates `kani_variant_constructions()` for
the state enum. It returns `Vec<KaniVariantConstruction>` —
`{ variant_name, depth0, depth1, depth2 }` structs.

**Field construction rules** (enforced by `kani_variants.rs`):

| Field type | depth-0 | depth-1 | depth-2 |
|---|---|---|---|
| `Vec<T>` | `Vec::new()` | `vec![<T as KaniCompose>::kani_depth0()]` | two elements |
| `String` | `String::new()` | same | same |
| `Option<T>` | `None` | `Some(<T as KaniCompose>::kani_depth0())` | same as depth-1 |
| primitive | `kani::any::<T>()` | same | same |
| any other `T` | `<T as KaniCompose>::kani_depth0()` | `kani_depth1()` | `kani_depth2()` |

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

## 4. Running Proofs with `elicit_proofs vsm`

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

## 5. Bugs Kani Has Found

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

## 6. How to Add a New VSM to the Proof Suite

### Step 1: Derive `KaniVariantState` and `KaniCompose` on state types

```rust
// On the state enum — generates kani_variant_constructions()
#[derive(Debug, Clone, KaniVariantState, /* other derives */)]
pub enum MyState {
    Idle,
    Processing { count: usize },
    Error(String),
}

// On any user-defined type that appears as a field in state variants:
// #[derive(KaniCompose)]
// pub struct MyData { ... }
```

For recursive types or types with `Vec<Self>` fields, `KaniCompose` generates
depth-bounded instances automatically via `#[derive(KaniCompose)]` (or manual impl).

If any variant contains fields that require `kani::Arbitrary` but don't
implement it (e.g., custom structs), either:
- Add `#[derive(KaniCompose)]` to the type, or
- Implement `KaniCompose` manually with bounded constructions.

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
cargo kani -p elicit_proofs --lib --features kani --harness start__kani__idle__d0
```

---

## 7. Proof Suite Counts (as of last generation)

| Module | Harnesses | Notes |
|---|---|---|
| `archive_connection` | 129 | All pass (3× expansion) |
| `archive_nav` | 111 | All pass; 1 overflow fixed |
| `archive_overlay` | 168 | All pass; 2 overflows fixed |
| `archive_panel` | ~1245 | Largest suite; verification ongoing |
| **Total** | **~1653** | 3× expansion from per-depth harnesses |

---

## 8. Architecture Decision Record

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

### Why `KaniCompose` instead of `kani::Arbitrary` for collection fields?

`kani::Arbitrary for Vec<T>` is not implemented in Kani ≤0.67. Even if it were,
symbolic `Vec` creates unbounded heap allocation that CBMC models as a loop with
unknown iteration count. `KaniCompose` avoids this by providing **concrete**
instances at specific sizes (0, 1, 2 elements). The distinction:

- `kani::any::<Vec<ExplainNode>>()` → CBMC: "this Vec could have any number of
  elements" → unbounded drop loop
- `KaniCompose::kani_depth1()` → CBMC: "this Vec has exactly 1 element"
  → one drop call, terminates immediately

### Why three depths and not more?

Depths 0, 1, 2 provide the **compositional proof argument** without exponential
blowup:
- Depth-0: base case (empty collections are safe)
- Depth-1: adding one element is safe (given base case)
- Depth-2: adding two elements is safe (transitivity)

By induction, any finite collection is safe. Adding depth-3 or higher would
be redundant — the inductive step is already covered at depth-1/2. Depth-2
exists as a second inductive step to build confidence.

### Why not `#[kani::unwind(N)]` with a large N?

`#[kani::unwind(5)]` is what the agent tried first (and the user immediately
rejected as counterproductive). An unwind bound tells CBMC: "if a loop exceeds
N iterations, assume this execution doesn't happen." This is unsound for proofs
— you're not proving the property, you're proving it *given that no loop runs
more than N times*. The overflow bugs found by per-variant Kani would not have
been found with bounded unwinding, because the symbolic `usize::MAX` value
would require more than 5 iterations to propagate through arithmetic.

### Why string concatenation in `kani_harness_for_variant_at_depth`?

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

## 9. Troubleshooting

### All harnesses fail immediately with "requires features: runner"

You are missing `--lib` from the `cargo kani` invocation. The `[[bin]]` target
requires the `runner` feature; the harnesses are in the library. Always use:
```bash
cargo kani -p elicit_proofs --lib --features kani --harness HARNESS_NAME
```

### A new harness hangs or times out

1. **Check for recursive types in state variant fields.** If a variant field
   is `Vec<T>` where `T` itself contains `Vec<T>` (recursive), the type must
   implement `KaniCompose` with depth-bounded construction. Add
   `#[derive(KaniCompose)]` to `T`.

2. **Check for `HashMap` in state variants.** `HashMap::new()` calls
   `RandomState::new()` → `getrandom` syscall → Kani can't model. Replace
   with `BTreeMap` in VSM state types, or implement `KaniCompose` manually
   using `HashMap::with_hasher(S::default())`.

3. **Check for `String` fields in non-state parameters.** If a transition
   takes a `String` parameter, `formal_method` uses `String::new()` for it
   (not symbolic). If the state field is being populated from a symbolic
   source elsewhere, trace it back.

4. **Check for `kani::any()` on a non-primitive inner type.** If a variant
   field's type does not match Vec/String/Option/primitive but also doesn't
   implement `KaniCompose`, the harness will hang. Add `#[derive(KaniCompose)]`
   or a manual `impl KaniCompose`.

5. **Check `usize` arithmetic.** Any `x + 1` or `x - 1` on a symbolic `usize`
   will fail; use `saturating_add(1)` / `saturating_sub(1)`.

### Harness names in the manifest don't match actual harnesses

Rebuild: `cargo build -p elicit_proofs`. The manifest is generated at build
time from the live token streams. If the source changed but the generated
files weren't rebuilt, names will be stale.

### `cargo build -p elicit_proofs` fails with a `KaniVariantState` error

A new field type was added to a state variant. `field_construction_exprs` in
`elicitation_derive/src/kani_variants.rs` handles: Vec, String, Option,
primitives, and `KaniCompose` types. For any other type:

- Add `#[derive(KaniCompose)]` to the type (generates depth-bounded impls).
- Or implement `KaniCompose` manually with concrete constructions.
- Or wrap it in `Option<T>` (gets `None` at depth-0) if optional.

---

## 10. Further Reading

| Document | Location |
|---|---|
| VSM architecture (layers, traits, proofs) | [`VERIFIED_STATE_MACHINES.md`](VERIFIED_STATE_MACHINES.md) |
| `KaniCompose` trait and impls | `crates/elicitation/src/kani_compose.rs` |
| `KaniVariantState` derive impl | `crates/elicitation_derive/src/kani_variants.rs` |
| `VerifiedStateMachine` derive impl | `crates/elicitation_derive/src/derive_vsm.rs` |
| `#[formal_method]` harness generation | `crates/elicitation_derive/src/formal_method.rs` |
| VSM runner source | `crates/elicit_proofs/src/vsm.rs` |
| Harness generation (`build.rs`) | `crates/elicit_proofs/build.rs` |
| Archive VSM sources | `crates/elicit_server/src/archive/vsm/` |
| Kani vec boundary research | `crates/elicitation_kani/src/vec_boundary.rs` |
| Justfile recipes | `justfile` (`verify-vsm-*`) |
