# Kani Verification for Verified State Machines

> **Operational guide.** This document explains how Kani model-checking is
> applied to `VerifiedStateMachine` types in this codebase: what approaches were
> tried, which failed and why, what the current design is, and how to run and
> extend it.
>
> For the architecture of VSMs themselves (layers, traits, type theory), see
> [`VERIFIED_STATE_MACHINES.md`](VERIFIED_STATE_MACHINES.md).
> For the Creusot deductive backend, see [`CREUSOT_FOR_VSMS.md`](CREUSOT_FOR_VSMS.md).

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
| `kani::any_vec::<T, 0>()` on the *state* variant's inner fields | Helps for Vec parameters to *transitions* (see §6), but the problem is the enum discriminant itself being symbolic. |
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

**Limitation:** This approach only works when `T` in `Vec<T>` is *not*
self-recursive. For `ExplainNode { children: Vec<ExplainNode> }`, CBMC's
destructor model is **type-driven**, not value-driven. Even `Vec::new()`
(zero elements) makes CBMC unroll:
```
ExplainNode::drop → Vec<ExplainNode>::drop → ExplainNode::drop → ...
```
The infinite chain is in the *type definition*, not the runtime content.
`KaniCompose::kani_depth0()` does not help here. See §3 for the fix.

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
- `Box<T>`: all depths → `Box::new(T::kani_depth{n}())` (see §4 for why this matters)
- `BTreeMap<K,V>`, `HashMap<K,V,S>`: all depths = empty (no `RandomState::new()`)
- User types: `#[derive(KaniCompose)]` or manual impl

**Important:** `HashMap::new()` calls `RandomState::new()` → `getrandom` syscall
→ Kani cannot model. Always use `BTreeMap` or `HashMap::with_hasher(S::default())`
in Kani contexts. The `ErdLayout` type was changed from `HashMap` to `BTreeMap`
for this reason.

---

## 3. The Third Problem — Self-Recursive Types: Arena/Index Elimination

### Why depth-bounding is insufficient

The `KaniCompose` depth-0/1/2 approach in §2 works when `T` in `Vec<T>` is
non-recursive. For `ExplainNode { children: Vec<ExplainNode> }`, CBMC generates
a recursive destructor chain regardless of runtime values:

```
ExplainNode::drop()
  → Vec<ExplainNode>::drop()   // drops each element
    → ExplainNode::drop()      // for each element
      → Vec<ExplainNode>::drop()  // ... infinite
```

The depth of unrolling is determined by CBMC's **type analysis**, not by
runtime content. An empty `Vec::new()` still triggers this infinite chain
because the *type* `Vec<ExplainNode>` structurally contains `ExplainNode`,
which structurally contains `Vec<ExplainNode>`.

### What does not fix it

| Attempted approach | Why it fails |
|---|---|
| `KaniCompose::kani_depth0()` with `Vec::new()` | Recursion is in the type, not the value. CBMC unrolls the destructor chain regardless. |
| `#[kani::unwind(N)]` | Caps CBMC exploration but does not make the type non-recursive. Unsound: paths beyond N unwinds are silently dropped. |
| Wrapping in `Option<T>` | `Option<ExplainNode>::drop()` → `ExplainNode::drop()` — same infinite chain. |
| `Box::into_raw` / explicit leak | Unsound: proof no longer covers the dropped state. |

### The solution: remove the recursion from the type

Replace the self-referential field with an **arena index**:

```rust
// ❌ Before — self-recursive, CBMC hangs even on empty Vec
pub struct ExplainNode {
    pub label: String,
    pub value: Option<f64>,
    pub children: Vec<ExplainNode>,   // ← recursive
}

// ✅ After — children are indices into a flat arena
pub struct ExplainNode {
    pub label: String,
    pub value: Option<f64>,
    pub children: Vec<usize>,         // ← plain index, no recursion
}

// Arena wrapper holds the flat list
pub struct ExplainPlan {
    pub nodes: Vec<ExplainNode>,      // flat, non-recursive
    pub root: usize,                  // index of the root node
}
```

`Vec<usize>::drop()` is trivially bounded. CBMC sees `ExplainNode` as a struct
with `String` + `Option<f64>` + `Vec<usize>` — all non-recursive.
`ExplainPlan.nodes` is a `Vec<ExplainNode>`, but `ExplainNode::drop()` no
longer recurses. The 39 harnesses for variants containing `ExplainView` and
`ExplainComparison` had all timed out; after the arena refactor, each
completed in under 10 seconds.

### `KaniCompose` on the new types

After the refactor, `ExplainNode` derives `KaniCompose` as normal (children is
now `Vec<usize>`, handled by the standard depth rules). The arena wrapper needs
a manual impl:

```rust
impl KaniCompose for ExplainPlan {
    fn kani_depth0() -> Self {
        Self {
            nodes: vec![ExplainNode::kani_depth0()],
            root: 0,
        }
    }
}
```

**Important:** Do **not** add `#[cfg_attr(kani, derive(kani::Arbitrary))]` to
any struct containing `String` fields. `String` does not implement
`kani::Arbitrary`; the attribute compiles fine but Kani fails at verification
time with a confusing trait-bound error. Use `KaniCompose` exclusively.

### Generalization

Apply the arena/index pattern whenever:

- A type `T` contains a field of type `Vec<T>` or `Box<T>` (direct
  self-recursion).
- A mutual recursion cycle exists: `A { data: Vec<B> }`, `B { data: Vec<A> }`.
- Depth-0 with `Vec::new()` still times out after 60 seconds.

The arena can be as simple as the `ExplainPlan` pattern (flat `Vec<Node>` +
root index). The key invariant for Kani: `NodeType::drop()` must not call
itself, directly or through a transitive `Vec<NodeType>`.

---

## 4. The Fourth Problem — Union Byte Aliasing (Complex Live Arm + BTree Dead Arm)

### Symptom

A harness with a **concrete discriminant** and **no recursive types** still hangs
at 99% CPU. Per §1 the discriminant is concrete, per §3 no recursive `Vec<T>`
exists. The type is structurally straightforward — yet CBMC runs unbounded.

This happened with `ArchivePanelState::ConnectionEdit`:

```rust
// ✅ Concrete discriminant — no recursive types — BUT HANGS
let _s = ArchivePanelState::ConnectionEdit {
    profile: ConnectionProfile { /* 12× Option<String> fields */ },
    display_mode: ConnectionProfileMode::Card,
};
```

### Root cause: CBMC's enum union model

Rust enums are unions under the hood. When CBMC analyses the DROP of an enum
value, it does not just reason about the live arm — it must prove that the
**dead arms' bytes cannot trigger their destructors**. For a dead arm containing
a `BTreeMap`, CBMC must traverse the BTree node chain to prove it terminates:

```
// CBMC's internal model for ArchivePanelState::drop()
match discriminant {
    ConnectionEdit  => /* live: drop profile, display_mode */
    ErdView         => /* DEAD — but CBMC still asks: "is the BTree reachable?" */
                       BTreeMap::drop() → loop over BTree nodes …
    …
}
```

CBMC propagates the concrete discriminant to prune the live arm correctly. But
for each dead arm, it must **also prove the dead arm's data is not live** — i.e.,
that no valid pointer lurks in the union's byte representation from the live arm's
fields.

### The trigger: `String::new()` dangling pointers

`ConnectionProfile` contains roughly 12 `Option<String>` fields. At depth-0:
- `String::new()` sets the internal buffer pointer to `NonNull::dangling()` = `0x1`.
- `Option<String>::None` leaves the byte representation as zero (null niche).
- At depth-0, all `Option<String>` fields are `None`, so most bytes are 0.

The struct is ~222 bytes. Those 222 bytes sit in the enum union alongside every
other variant. The dead `ErdView` variant contains `Option<ErdLayout>` where
`ErdLayout` is a `BTreeMap`. CBMC sees those same 222 bytes through `ErdView`'s
lens and asks: "could these bytes represent a valid, non-null BTree root node
pointer?" The answer is not trivially "no" — because `String::new()` puts
`0x1` (non-null, non-zero) into some of those bytes — so CBMC enters the BTree
traversal loop trying to prove the BTree is empty or null. The loop unwinds
without bound.

### The fix: `Box<T>` for large live-arm structs

`Box<T>` stores only a pointer (8 bytes on 64-bit). The union footprint of the
live arm shrinks from ~222 bytes to exactly 8 bytes. Those 8 bytes represent one
valid heap pointer. CBMC trivially proves the dead arm's BTree node fields
(also 8-byte pointer slots) are not aliased with the live arm's content:

```rust
// ✅ Live arm is Box<T>: union footprint = 8 bytes.  CBMC immediately proves
//    dead BTree arm is unreachable.  Both AZ and BA now pass in ~1s.
ConnectionEdit {
    profile: Box<ConnectionProfile>,   // ← box the large struct
    display_mode: ConnectionProfileMode,
}
```

The construction site wraps the value before returning it as the output state:

```rust
fn open_connection_editor(
    state: ArchivePanelState,
    proof: Established<ArchivePanelConsistent>,
    profile: ConnectionProfile,           // ← takes plain value …
    display_mode: ConnectionProfileMode,
) -> (ArchivePanelState, Established<ArchivePanelConsistent>) {
    (
        ArchivePanelState::ConnectionEdit {
            profile: Box::new(profile),   // ← … boxes on the way out
            display_mode,
        },
        proof,
    )
}
```

### How this was diagnosed: synthetic ladder

The root cause was identified by running a graduated sequence of isolated Kani
harnesses in `crates/elicitation_kani` (no heavy workspace dependencies — fast
build cycle). Each theory changed exactly one variable:

| Theory | Live arm | Dead arm | Result |
|--------|----------|----------|--------|
| BO | 7 `Option<String>` direct | `u32` | **PASS** — dead arm trivial |
| BP | `MockProfile` (7 `Option<String>` nested) | `u32` | **PASS** — dead arm still trivial |
| BQ | `MockProfile` nested | `BTreeMap` | **HANG** ← trigger confirmed |
| BR | `MockProfile` nested | `Box<BTreeMap>` | **HANG** — boxing dead arm insufficient |
| BS | `Box<MockProfile>` | `BTreeMap` | **PASS** ← boxing live arm resolves |

BQ isolates the trigger (complex live arm + BTree dead arm). BR rules out
boxing the dead arm as a fix. BS confirms boxing the live arm is sufficient.

### `Box<T>: KaniCompose`

Boxing a live-arm struct requires `Box<T>` to implement `KaniCompose` so the
harness generator produces correct depth-bounded constructions:

```rust
#[cfg(kani)]
impl<T: KaniCompose> KaniCompose for Box<T> {
    fn kani_depth0() -> Self { Box::new(T::kani_depth0()) }
    fn kani_depth1() -> Self { Box::new(T::kani_depth1()) }
    fn kani_depth2() -> Self { Box::new(T::kani_depth2()) }
}
```

The `field_construction_exprs` function in `kani_variants.rs` already handles
`Box<T>` via the fallthrough "any other T → delegate to `KaniCompose`" case,
so no change to the derive macro was required — only the impl needed adding.

### When to apply this pattern

Box a variant's field when **all three** of these hold:

1. The struct is large (dozens of bytes, typically multiple `Option<String>` or
   similar non-trivially-zero fields).
2. The enum has at least one other variant containing a collection with
   non-trivial drop traversal (`BTreeMap`, `Vec<T>` where T has heap fields).
3. Concrete-discriminant harnesses for *other* variants hang even though those
   other variants look structurally simple (this is the tell: the BTree loop is
   in a dead arm, pulled in by the large live arm's bytes).

Boxing the struct has zero runtime overhead relative to heap-allocating it
elsewhere; the trade-off is one extra pointer indirection per field access.

---

## 5. The Machinery — How Harnesses Are Generated

The codebase generates harnesses automatically. You never write them by hand.
The pipeline has four stages.

> **Note:** The production output is now **one `proof_for_contract` closure
> harness per transition** (§6). The per-variant `__d0/d1/d2` harnesses are
> still generated as a companion-struct method for diagnostic use but are no
> longer included in `vsm_kani_proof()`. The pipeline description below covers
> both paths.

```
  1. #[formal_method] on a transition fn
         ↓  (at proc-macro expansion time)
  2a. FooTransition::kani_harness_for_variant_at_depth(variant_name, state_expr, depth)
         ↓  (available for diagnostic use; not called by vsm_kani_proof)
  2b. FooTransition::kani_closure_proof(inv_fn)
         ↓  (proof_for_contract closure harness — this IS called by vsm_kani_proof)
  3. #[derive(VerifiedStateMachine)] collects kani_closure_proof() via
         transition_kani_closure_proofs()
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
  - `kani_harness_for_variant_at_depth(variant_name, state_expr, depth) -> TokenStream`
    — substitutes `state_expr` for the state param; appends `__d{depth}` to the
    harness function name. Available for diagnostics; not called by default.
  - `kani_closure_proof(inv_fn) -> TokenStream` — the production
    `#[kani::proof_for_contract]` harness using forgive-and-forget (§6.2).

- Also emits Kani contracts on the **original function** via `cfg_attr`:
  ```rust
  #[cfg_attr(kani, kani::requires(inv_fn(&state)))]
  #[cfg_attr(kani, kani::ensures(|result: &(State, _)| inv_fn(&result.0)))]
  ```

- Gates `#[instrument]` under `cfg_attr(not(kani), instrument(...))` to prevent
  tracing SAT explosion (§6.4).

**Critical detail:** The inline `#[kani::proof]` function that `#[formal_method]`
*would* emit is intentionally suppressed. If it were emitted, `cargo kani` would
compile it with `kani::any::<StateEnum>()` — the naïve approach that hangs.

### Stage 2a: `kani_harness_for_variant_at_depth` (diagnostic)

Given `variant_name = "export_picker_open"`,
`state_expr = "ArchiveOverlayState :: ExportPickerOpen"`, and `depth = 0`,
this method builds a concrete per-variant harness:

```
# [cfg (kani)] # [:: kani :: proof] fn close_overlay__kani__export_picker_open__d0 () {
    let _state : ArchiveOverlayState = ArchiveOverlayState :: ExportPickerOpen ;
    let proof : Established < ArchiveOverlayConsistent > = { ... } ;
    let _result = close_overlay (_state , proof) ;
}
```

These harnesses remain useful for isolating failures to a specific variant.
They are NOT included in the default `vsm_kani_proof()` output.

### Stage 2b: `kani_closure_proof` (production)

Given `inv_fn = "archive_panel_consistent"`, this method builds the
`proof_for_contract` harness using forgive-and-forget (§6.2):

```
# [allow (unexpected_cfgs)] # [cfg (kani)]
# [:: kani :: proof_for_contract (column_detail)] fn column_detail__kani_closure () {
    let state : ArchivePanelState = <ArchivePanelState as :: elicitation :: KaniCompose> :: kani_any () ;
    :: kani :: assume (archive_panel_consistent (& state)) ;
    :: std :: mem :: forget (state) ;
    let state : ArchivePanelState = <ArchivePanelState as :: elicitation :: KaniCompose> :: kani_depth0 () ;
    let _result = column_detail (state , ...) ;
    :: std :: mem :: forget (_result) ;
}
```

### Stage 3: `#[derive(VerifiedStateMachine)]` + `#[derive(KaniVariantState)]`

`#[derive(VerifiedStateMachine)]` generates two methods:
- `transition_harnesses()` — loops over variant constructions, calls
  `kani_harness_for_variant_at_depth` at depths 0/1/2. Still generated but
  **no longer called** from `vsm_kani_proof()`.
- `transition_kani_closure_proofs(inv_fn)` — calls `kani_closure_proof(inv_fn)`
  for each transition. **This is the primary production output.**

`vsm_kani_proof()` now calls only `transition_kani_closure_proofs()`:
```rust
fn vsm_kani_proof() -> proc_macro2::TokenStream {
    let mut ts = Self::Invariant::kani_proof();
    let inv_fn = Self::Invariant::kani_invariant_fn_name();
    for closure in Self::transition_kani_closure_proofs(inv_fn) {
        ts.extend(closure);
    }
    ts
}
```

`#[derive(KaniVariantState)]` generates `kani_variant_constructions()` for
the state enum (still used by `kani_harness_for_variant_at_depth`).

**Field construction rules** (enforced by `kani_variants.rs`):

| Field type | depth-0 | depth-1 | depth-2 |
|---|---|---|---|
| `Vec<T>` | `Vec::new()` | `vec![<T as KaniCompose>::kani_depth0()]` | two elements |
| `String` | `String::new()` | same | same |
| `Option<T>` | `None` | `Some(<T as KaniCompose>::kani_depth0())` | same as depth-1 |
| `Box<T>` | `Box::new(<T>::kani_depth0())` | `kani_depth1()` | `kani_depth2()` |
| primitive | `kani::any::<T>()` | same | same |
| any other `T` | `<T as KaniCompose>::kani_depth0()` | `kani_depth1()` | `kani_depth2()` |

`Box<T>` falls through to the "any other T" path in the derive since `Box<T>:
KaniCompose` is implemented; the table row above is the effective behaviour.

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

## 6. The `proof_for_contract` Architecture (Current Production Design)

The per-variant harnesses in §5 established that formal verification of VSM
transitions is tractable — but at a cost of N×M×3 harnesses (N transitions,
M variants, 3 depths). For `archive_panel` with 23 transitions and 18 variants,
this produced **1,242 harnesses in 22,000+ lines** of generated code.

A systematic proof gallery (Level 11–12, `crates/elicit_proofs/src/kani/gallery/`)
established that Kani's DFCC mode (`-Z function-contracts`) enables a **single
symbolic harness per transition**, replacing all N×M×3 harnesses with N harnesses
while covering strictly more states (symbolic over all valid inputs, not just
bounded constructions of each variant).

### 6.1 Why DFCC Changes Everything

`#[kani::proof_for_contract(fn_name)]` activates DFCC (dynamic frame condition
checking) on the original function. DFCC instruments the ORIGINAL function body
directly:

1. Checks the precondition (`#[kani::requires]`) at function entry.
2. Checks the postcondition (`#[kani::ensures]`) at function exit.
3. Tracks which memory locations the function is allowed to modify (the "frame").

Contracts must be on the **original function** — not on a wrapper. The
`#[formal_method]` attribute emits these contracts directly via `cfg_attr`:

```rust
// What #[formal_method(contracts = [ArchivePanelConsistent])] emits on the fn:
#[cfg_attr(kani, kani::requires(archive_panel_consistent(&state)))]
#[cfg_attr(kani, kani::ensures(|result: &(ArchivePanelState, _)| archive_panel_consistent(&result.0)))]
pub fn column_detail(state: ArchivePanelState, proof: Established<ArchivePanelConsistent>)
    -> (ArchivePanelState, Established<ArchivePanelConsistent>)
{
    (ArchivePanelState::ColumnDetail, proof)
}
```

**Critical:** Contracts must be on the ORIGINAL function, not on a generated
contracted wrapper. A wrapper that calls the original causes DFCC to inline
the wrapper → call original → inline original body, doubling the CBMC work
and causing timeouts. The `cfg_attr` approach avoids this entirely.

### 6.2 The Forgive-and-Forget Pattern

Using `kani_any::<StateEnum>()` naively still causes the symbolic enum drop
explosion described in §1. With DFCC, the fix is **forgive-and-forget**:

```rust
#[allow(unexpected_cfgs)]
#[cfg(kani)]
#[kani::proof_for_contract(column_detail)]
fn column_detail__kani_closure() {
    // Step 1: symbolic state covering all valid inputs
    let state: ArchivePanelState = <ArchivePanelState as KaniCompose>::kani_any();
    kani::assume(archive_panel_consistent(&state));

    // Step 2: FORGET the symbolic state — prevents drop-glue SAT explosion
    std::mem::forget(state);

    // Step 3: depth-0 SHADOW for the actual function call
    let state: ArchivePanelState = <ArchivePanelState as KaniCompose>::kani_depth0();

    // Step 4: call and forget the result (DFCC already checked postcondition)
    let _result = column_detail(state, /* Established arg */);
    std::mem::forget(_result);
}
```

Why this works step-by-step:

- `kani_any()` + `assume(inv)` gives CBMC a symbolic state constrained to valid
  inputs — full symbolic coverage without committing to a concrete variant.
- `forget(state)` tells CBMC: "do not reason about this value's destructor."
  The drop-glue explosion from §1 never fires because the symbolic state is
  leaked, not dropped.
- The second `kani_depth0()` binding is a fresh, bounded concrete value that
  flows into the function. Its destructor terminates (exactly as in §2). The
  function call itself is not symbolic in its input — it uses this bounded shadow.
- `forget(_result)` prevents the output state's destructor from triggering.
  DFCC has already verified the postcondition before `_result` would go out
  of scope. No need to model its drop.

**Why `kani_depth0()` as the shadow:** The shadow just needs to be a valid,
CBMC-tractable value of the right type. `kani_depth0()` satisfies this with
bounded destructor complexity. The important symbolic work has already been done
by the `kani_any()` + `assume()` pair constraining the proof's precondition.

**Gallery evidence:** experiments 12e (String heap in result variant, 39s) and
12f (match on 18-variant input state, 33s) confirm that heap allocation and
large match trees have negligible DFCC overhead when the result is forgotten.
All tractable patterns land in the 32–39s range.

### 6.3 The `#[instrument]` SAT Explosion

`tracing`'s `#[instrument]` adds a `tracing::Span` to every function call.
Under Kani/CBMC, tracing closures **capture all arguments symbolically**.
For functions with large enum arguments (like `ArchivePanelState`), CBMC must
model the full 18-variant symbolic state inside the closure — the same
drop-explosion as §1, but triggered by the tracing span rather than an explicit
`kani::any()`.

**Symptom:** `proof_for_contract` harnesses on instrumented functions timeout
even for simple one-line functions.

**Fix:** `#[formal_method]` automatically gates all `#[instrument]` attributes
under `cfg_attr(not(kani), ...)`. You do not need to do this manually.

If you add `#[instrument]` to a transition function by hand (outside
`#[formal_method]`), you MUST gate it the same way:

```rust
#[cfg_attr(not(kani), tracing::instrument(skip(state), fields(variant = ?state)))]
pub fn my_transition(state: MyState, ...) -> (...) { ... }
```

This applies to **any tracing integration** that captures function arguments,
not just `#[instrument]`. Any closure over a symbolic large enum will cause the
same explosion.

#### 6.3.1 Inline `tracing::debug!()` / `tracing::info!()` — same failure

Inline tracing event macros in callee **function bodies** trigger the same
goto-instrument hang even without `#[instrument]`.

```rust
// ❌ BAD — hangs goto-instrument:
fn settle(self, outcome: Outcome, ...) -> (...) {
    let final_balance = self.post_bet_balance + returned;
    tracing::debug!(outcome = ?outcome, "Payout settled");  // ← hang
    (final_balance, Established::assert())
}

// ✅ GOOD — compiled out under kani:
fn settle(self, outcome: Outcome, ...) -> (...) {
    let final_balance = self.post_bet_balance + returned;
    #[cfg(not(kani))]
    tracing::debug!(outcome = ?outcome, "Payout settled");
    (final_balance, Established::assert())
}
```

The `?field` sigil (Debug format) is the worst offender: it calls
`format!("{:?}", value)` which can heap-allocate a `String`, giving CBMC an
unbounded symbolic allocation.  Even simple `tracing::debug!(count = n)` calls
access the global dispatcher through a thread-local `AtomicUsize`, which CBMC
models symbolically.

**Rule:** Gate **every** inline tracing event (`debug!`, `info!`, `warn!`,
`error!`) in any function reachable from a VSM transition with
`#[cfg(not(kani))]`.  Gate `#[instrument]` attributes with
`#[cfg_attr(not(kani), tracing::instrument(…))]`.

**Gallery evidence:** Level 14 isolates this case (`gallery14a_ungated_debug`
hangs; `gallery14b_gated_debug` completes in ~8 s).  The fix resolved
`bj_place_bet` and `bj_dealer_turn` timeouts in `strictly_blackjack`, bringing
BJ to 5/5 ✅.

### 6.4 `Established<P>: kani::Arbitrary` for Composition

`Established<P>` appears in the return type of every VSM transition. For
`stub_verified` composition (§6.5) to work, Kani must be able to reconstruct
the return type of a stubbed function. Since `Established<P>` is a ZST
(contains only `PhantomData<P>`), the impl is trivial:

```rust
#[cfg(kani)]
impl<P: Prop> kani::Arbitrary for Established<P> {
    fn any() -> Self {
        Self { _marker: PhantomData }
    }
}
```

This is defined in `crates/elicitation/src/contracts.rs`. Without it,
`stub_verified` fails at Kani verification time with a confusing trait-bound
error on the return type.

### 6.5 `stub_verified` Composition

Once a transition's `proof_for_contract` harness verifies, it can be reused
in multi-step proofs. `stub_verified` replaces a callee's body with its
contract axiom — CBMC no longer inlines the function body:

```rust
#[kani::proof]
#[kani::stub_verified(column_detail)]
#[kani::stub_verified(panel_loading)]
fn two_step_composition() {
    // Forgive-and-forget setup (same pattern as closure harnesses)
    let state: ArchivePanelState = <ArchivePanelState as KaniCompose>::kani_any();
    kani::assume(archive_panel_consistent(&state));
    std::mem::forget(state);
    let state: ArchivePanelState = <ArchivePanelState as KaniCompose>::kani_depth0();
    let proof = ArchivePanelConsistent::kani_proof_credential();

    // Step 1: column_detail — body replaced with its contract axiom
    let (state2, e2) = column_detail(state, Established::prove(&proof));
    std::mem::forget(e2);

    // Step 2: panel_loading — body replaced with its contract axiom
    let (_state3, _e3) = panel_loading(state2, Established::prove(&proof));
}
```

CBMC's task reduces to: "given `column_detail`'s postcondition, does
`panel_loading`'s precondition hold?" This is the contract composition step,
not a full function body verification.

**Performance:** gallery12d_two verified in **4s** vs **32s** for each individual
leaf harness — 8× speedup that scales with the number of steps composed.

**Required flags:** composition harnesses need both `-Z function-contracts`
and `-Z stubbing`:

```bash
cargo kani -p elicit_proofs --lib --features kani \
    -Z function-contracts -Z stubbing \
    --harness my_composition_harness
```

### 6.6 `QueryResult` Kani Simplification

`QueryResult` contains nested `Vec<Vec<String>>` fields. This is not self-
recursive (§3 does not apply) but the two-level nesting creates a large CBMC
formula under `proof_for_contract`. A `#[cfg(kani)]` replacement type is used
instead:

```rust
// Under #[cfg(kani)]: replace with scalar-only struct
#[cfg(kani)]
pub struct QueryResult {
    pub row_count: u64,
}
```

This sidesteps the nested-Vec formula explosion while preserving the ability
to verify transitions that take or return `QueryResult`. The same approach
applies to any type where nested heap allocation creates intractable CBMC
formulas that cannot be solved with `forget` alone.

### 6.7 Harness Naming and Invocation

Closure harnesses generated by the pipeline follow the naming convention:

```text
{transition_fn}__kani_closure
```

Examples: `column_detail__kani_closure`, `panel_loading__kani_closure`.

Run a leaf proof:

```bash
cargo kani -p elicit_proofs --lib --features kani \
    -Z function-contracts \
    --harness column_detail__kani_closure
```

Run a composition proof:

```bash
cargo kani -p elicit_proofs --lib --features kani \
    -Z function-contracts -Z stubbing \
    --harness my_two_step_composition_harness
```

Per-variant harnesses (`{transition_fn}__kani__{variant}__d{depth}`) remain
available for isolating failures to specific variants but are not in the
default proof suite.

### 6.8 The Proof Gallery Methodology

`crates/elicit_proofs/src/kani/gallery/` is a research tool for validating
architectural hypotheses in minimal synthetic harnesses before applying them
to production code. Each level isolates one variable. When an approach is
uncertain, add a gallery experiment before committing to code generation
changes.

**Level 12 final results** (all harnesses verified against `archive_panel_consistent`):

| ID | What changes vs previous | Time |
|----|--------------------------|------|
| 12a | Baseline replicate of Level 11 | 33s |
| 12b | Drop instead of forget (unit variant) | 31s |
| 12c | `Established<P>` ZST in return type | 32s |
| 12d | Inline real `column_detail` body (no contracts) | TIMEOUT >5m |
| 12d_pfc | `proof_for_contract` on the original function | 32s |
| 12d_two | `stub_verified` two-step composition | **4s** |
| 12e | String heap allocation in result variant | 39s |
| 12f | Match on 18-variant enum as function input | 33s |

**Key finding:** All tractable patterns land in the 32–39s range. Only
unconstrained inlining of callees without contracts causes formula explosion.
Strings, heap allocation, and large match trees add negligible overhead.

---

## 7. Running Proofs with `elicit_proofs vsm`

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

## 8. Bugs Kani Has Found

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

## 9. How to Add a New VSM to the Proof Suite

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

## 10. Proof Suite Counts (as of last generation)

### Production harnesses (`proof_for_contract` closure architecture)

| Module | Closure harnesses | Notes |
|---|---|---|
| `archive_panel` | 23 | One per transition; 418 generated lines |
| `archive_connection` | TBD | Regenerate to apply closure architecture |
| `archive_nav` | TBD | Regenerate to apply closure architecture |
| `archive_overlay` | TBD | Regenerate to apply closure architecture |

Each module also emits 1 invariant `#[kani::proof]` harness
(`{module}_invariant__kani`).

### Diagnostic per-variant harnesses (legacy — still generated, not in default suite)

The code generator (`kani_harness_for_variant_at_depth`) still produces
per-variant d0 harnesses for debugging individual variants. They are generated
but `vsm_kani_proof()` no longer calls `transition_harnesses()` — they do not
run in CI. Use them to isolate failures when a closure harness fails.

```text
archive_panel: 23 transitions × 18 variants × 1 depth = 414 diagnostic harnesses
```

---

### Why `proof_for_contract` over per-variant harnesses?

The per-variant harnesses in §5 provided the inductive argument (d0 = base,
d1 = step-1, d2 = step-2). `proof_for_contract` with DFCC is strictly stronger:
it covers **all** valid states symbolically via `kani_any() + assume(inv)`, not
just bounded per-variant constructions.

The efficiency story is also compelling. For `archive_panel` with 23 transitions
and 18 variants, the per-variant approach produced 22,000+ lines of harnesses.
The closure architecture produces **418 lines** — a 54× reduction with no
decrease in proof coverage. Verification time per harness is similar (32–39s).

Per-variant harnesses remain available as diagnostic tools (run them to isolate
failures to a specific variant when a closure harness fails), but they are no
longer the primary proof vehicle.

### Why forgive-and-forget, not a manual `Arbitrary` impl?

A handwritten `impl kani::Arbitrary for ArchivePanelState` with a
`match kani::any::<u8>() % 18 { ... }` arm creates a symbolic discriminant that
CBMC propagates into every caller's drop code — the same explosion as §1.

`forget(state)` prevents drop-glue reasoning entirely. The symbolic state is
never destroyed; CBMC proves the precondition holds, and the shadow `kani_depth0()`
value flows into the actual function call with a tractable concrete destructor.

### Why `stub_verified` for multi-step proofs?

Once a transition verifies under `proof_for_contract`, its contract is an
established axiom. `stub_verified` substitutes the body with that axiom —
CBMC's task becomes "given the postcondition of step 1, does step 2's
precondition hold?" instead of "do steps 1 and 2 together satisfy the combined
postcondition?"

Gallery 12d_two confirmed this yields an **8× speedup** (4s vs 32s per leaf).
Multi-step composition proofs are tractable with `stub_verified` even when each
leaf is already near the 30–40s range. Requires `-Z stubbing` in addition to
`-Z function-contracts`.

### Why must `#[instrument]` be gated under `cfg_attr(not(kani), ...)`?

`tracing`'s `#[instrument]` captures all function arguments in a closure for
the `tracing::Span`. Under Kani/CBMC, closure captures are symbolic. For a
function with a large enum argument (e.g. `ArchivePanelState`, 18 variants),
CBMC must model the full symbolic enum inside the tracing closure — exactly the
§1 drop explosion, but triggered by instrumentation rather than an explicit
`kani::any()`.

Symptom: even a one-liner transition times out under `proof_for_contract`.

`#[formal_method]` gates `#[instrument]` automatically. Any manually added
`#[instrument]` (outside `#[formal_method]`) on a function with large enum
arguments must be gated with `#[cfg_attr(not(kani), tracing::instrument(...))]`.

---

## 11. Architecture Decision Record

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

## 12. Troubleshooting

### All harnesses fail immediately with "requires features: runner"

You are missing `--lib` from the `cargo kani` invocation. The `[[bin]]` target
requires the `runner` feature; the harnesses are in the library. Always use:
```bash
cargo kani -p elicit_proofs --lib --features kani --harness HARNESS_NAME
```

### A new harness hangs or times out

1. **Check for recursive types in state variant fields.** Distinguish two cases:

   - *Non-recursive `Vec<T>`* (T does not contain `Vec<T>`): add
     `#[derive(KaniCompose)]` to `T`; depth-bounded instances (§2) will work.
   - *Self-recursive `Vec<T>`* (T contains `Vec<T>` or `Box<T>`): depth-bounding
     does **not** help (§3). Apply the arena/index refactor: replace `Vec<T>`
     with `Vec<usize>` and introduce a flat arena wrapper. The
     `ExplainNode.children: Vec<ExplainNode>` → `Vec<usize>` change is the
     reference example.

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

6. **Check for union byte aliasing (large live arm + BTree dead arm).** (§4)
   If a harness with a concrete discriminant and no recursive types still hangs,
   and `--show-loops` reveals `BTreeMap::deallocating_*` loops for a *different*
   variant, the trigger is union byte aliasing. The large live-arm struct's
   non-null interior pointers (e.g., `String::new()` → `NonNull::dangling()`)
   look like valid BTree node pointers to CBMC when it inspects the union through
   the dead arm's layout.
   
   **Diagnosis:** run a synthetic ladder in `crates/elicitation_kani`:
   1. Live arm = your large struct + dead arm = `u32` → should PASS
   2. Live arm = your large struct + dead arm = `BTreeMap` → should HANG (confirms trigger)
   3. Live arm = `Box<YourStruct>` + dead arm = `BTreeMap` → should PASS (confirms fix)
   
   **Fix:** wrap the large struct in `Box<T>` inside the variant definition and
   add `Box::new(value)` at each construction site. The union footprint drops to
   8 bytes; CBMC trivially proves the dead BTree arm is unreachable.

7. **Check for un-gated `#[instrument]` on functions with large enum args.**
   If a `proof_for_contract` closure harness times out even for a trivially
   simple transition body, `#[instrument]` may be the culprit. `tracing`
   closures capture all function arguments symbolically; for 18-variant enums
   this recreates the §1 drop explosion inside the tracing span.

   **Symptom:** `column_detail` (one-liner `return (ArchivePanelState::ColumnDetail, proof)`)
   times out. `#[instrument]` is present without a `cfg_attr(not(kani), ...)` gate.

   **Fix:** replace `#[instrument(...)]` with
   `#[cfg_attr(not(kani), tracing::instrument(...))]`. `#[formal_method]` does
   this automatically; only manually added instrument attrs need the gate.

### Closure harness naming

`proof_for_contract` harnesses use the naming convention `{fn}__kani_closure`:

```text
column_detail__kani_closure
panel_loading__kani_closure
close_overlay__kani_closure
```

Per-variant diagnostic harnesses (still generated, not in default suite) use:

```text
column_detail__kani__ColumnDetail__d0
column_detail__kani__LoadingPanel__d0
```

If `cargo kani --list` shows only the diagnostic harnesses, `vsm_kani_proof()`
is not calling `transition_kani_closure_proofs()`. Check `contracts.rs`.

### `Established<P>` fails `kani::Arbitrary` bound in a composition harness

`stub_verified` requires Kani to reconstruct the return type of the stubbed
function. `Established<P>` is in that return type. If you get a trait-bound
error like "`Established<MyProp>: kani::Arbitrary` is not satisfied" from a
`stub_verified` harness, the `impl kani::Arbitrary for Established<P>` is
missing from `contracts.rs`. The impl is trivial (ZST/PhantomData):

```rust
#[cfg(kani)]
impl<P: Prop> kani::Arbitrary for Established<P> {
    fn any() -> Self { Self { _marker: PhantomData } }
}
```

See `crates/elicitation/src/contracts.rs`.

### `#[cfg_attr(kani, derive(kani::Arbitrary))]` causes a trait-bound error

`String` does not implement `kani::Arbitrary`. Adding
`#[cfg_attr(kani, derive(kani::Arbitrary))]` to any struct with a `String`
field compiles fine (the derive is gated) but fails at Kani verification time
with a confusing "does not satisfy `kani::Arbitrary`" error. Never use
`kani::Arbitrary` on types with `String` fields — use `KaniCompose` instead.

### Harness compiles with `--features kani` but fails under `cargo kani`

`cargo check --features kani` enables the `kani` feature flag but does **not**
activate `#[cfg(kani)]` blocks. Code inside `#[cfg(kani)]` is only compiled
when actually running `cargo kani`. This means stale or broken harness code
in `#[cfg(kani)]` blocks can lurk undetected by `cargo check`. After refactors
that rename or remove types, explicitly search for `#[cfg(kani)]` in the
modified files — `cargo check --features kani` will not catch errors there.

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

### Diagnosing which loops are causing a hang: `cbmc --show-loops`

When a harness hangs at 99% CPU, use CBMC's `--show-loops` to enumerate every
loop in the GOTO model **before** running the SAT solver. This identifies the
culprit without waiting for an unbounded run.

**Step 1 — build the GOTO binary.**
Run the harness with `--cbmc-args --show-loops`. Kani will build the GOTO
binary, then its output parser will panic (it can't consume the non-JSON
`--show-loops` output), but the `.out` file is left behind:

```bash
cargo kani -p elicit_proofs --lib --features kani \
    --harness 'kani::diag::my_failing_harness' \
    -Z unstable-options --cbmc-args --show-loops
```

Look for the `.out` path in the output, e.g.:

```
Reading GOTO program from file
target/kani/.../elicit_proofs-HASH__HARNESS_MANGLED_NAME.out
```

**Step 2 — inspect the loops.**
Run CBMC directly on the `.out` file. The Kani-bundled binary is at
`~/.kani/kani-X.Y.Z/bin/cbmc`:

```bash
CBMC=~/.kani/kani-0.67.0/bin/cbmc
GOTO=$(ls target/kani/x86_64-unknown-linux-gnu/debug/deps/elicit_proofs-*HARNESS*.out)
$CBMC "$GOTO" --show-loops 2>&1
```

**What to look for:**

The loop list shows **every loop reachable in the GOTO model** — not just those
in the harness itself. Key patterns:

| Loop function | What it means |
|---|---|
| `drop_in_place::<[ComplexType]>` | Vec slice drop from **another enum variant** |
| `BTreeMap … deallocating_next/end` | BTree-internal traversal (ErdLayout etc.) |
| `__rust_dealloc.0 / .1` | Kani allocator loops (usually bounded) |

**The critical insight — enum drop glue includes ALL variants:**  
Even if your harness constructs only `MyEnum::VariantA`, CBMC includes the
drop-glue loops for **every variant** because Rust's enum is a union and CBMC
may not propagate the discriminant through the generated match. A seemingly
trivial harness that just constructs and drops one small variant will include
`Vec<ErdNode>` slice drops, BTreeMap traversals, etc. if other variants of the
same enum contain those types.

This is the fundamental driver of unbounded unwinding in `ArchivePanelState`
harnesses: the 18-variant enum carries `ErdView { layout: Option<ErdLayout> }`
(ErdLayout is a `BTreeMap<String, …>`), and BTree drop traversal is pulled into
every harness that ever drops an `ArchivePanelState` value, regardless of which
variant was actually stored.

**Resolution strategy:**

- `drop_in_place::<[ComplexType]>` loops from another variant's `Vec<T>` field:
  see §3 (arena/index elimination) and §4 (boxing the live arm).
- `BTreeMap … deallocating_*` loops from a dead arm: the live arm is likely a
  large struct with non-null interior pointers. Apply the §4 boxing fix.
- `__rust_dealloc.0 / .1` loops: Kani allocator, usually bounded — not the
  primary culprit.

If BTree loops appear and the harness has a concrete discriminant, confirm via
the synthetic ladder in §4 before applying a fix. The loop itself is in a dead
arm; boxing the **live** arm is the solution, not boxing the dead arm.

---

## 13. Further Reading

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
