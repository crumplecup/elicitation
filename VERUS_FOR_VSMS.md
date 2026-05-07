# Verus Verification for Verified State Machines

> **Operational guide.** This document explains how Verus deductive
> verification is applied to `VerifiedStateMachine` types in this codebase:
> what problems were encountered, what the current design is, and how to run
> and extend it.
>
> For the architecture of VSMs themselves (layers, traits, type theory), see
> [`VERIFIED_STATE_MACHINES.md`](VERIFIED_STATE_MACHINES.md).
> For the Kani model-checking backend, see [`KANI_FOR_VSMS.md`](KANI_FOR_VSMS.md).
> For the Creusot deductive backend, see [`CREUSOT_FOR_VSMS.md`](CREUSOT_FOR_VSMS.md).
> For the Verus proof infrastructure (running, syntax, tiers), see
> [`VERUS_GUIDE.md`](VERUS_GUIDE.md).

---

## 1. What Verus Verifies (vs Kani and Creusot)

| Backend | Method | Scope | VSM role |
|---------|--------|-------|---------|
| Kani | Bounded model checking (CBMC) | All variants, depth ≤ 2 | Memory-safety, no-panic, bit-level |
| Creusot | Deductive via WhyML + Alt-Ergo | All inputs, unbounded | Functional correctness, loop invariants |
| Verus | Unbounded SMT via Z3 | All inputs, unbounded | Rich contracts, ghost state, type invariants |

Verus and Creusot both offer unbounded proofs through different toolchains. A
property verified by all three has been checked by three completely independent
verification paths.

### Advantages over the other backends

| Issue | Kani workaround | Creusot workaround | Verus: no workaround needed |
|-------|-----------------|--------------------|-----------------------------|
| Heap types in enums | One harness per variant × 3 depths | Companion functions | Z3 ADT theory — single `assume_specification` |
| `String` fields | `kani::any_vec` depth harnesses | `extern_spec!` per crate | `vstd` full specs: `s@ == Seq::<char>::empty()` |
| Cross-module predicate opacity | N/A | Inline `const INV_*` in generated file | `pub open spec fn` — opt-in transparency |
| Cross-crate fn specs | N/A | `extern_spec!` must be in calling crate | `assume_specification` works from any crate |

### Verus-specific capabilities

- **`#[verifier::type_invariant]`** — invariant automatically checked at every
  construction and field mutation; no Kani/Creusot equivalent (gallery V8).
- **`Tracked<T>`** — linear ghost tokens that thread proof witnesses through
  transition chains (gallery V6, V10).
- **`spec fn` / `proof fn` / `exec fn`** three-mode hierarchy with explicit
  body-visibility tiers (`pub open` / `pub closed` / `#[verifier::opaque]`).

---

## 2. The `assume_specification` Companion Pattern

Verus cannot verify the production VSM transitions directly — they are
`async fn`s that use `tokio`, database drivers, and other runtime machinery.
Instead, **companion contracts** are generated in `crates/elicit_proofs/src/verus/generated/`
using `assume_specification`, which injects a trusted postcondition for an
external function without requiring Verus to verify its body.

For each archive machine transition `my_transition`, the generated file contains:

```rust
#[cfg(verus)]
verus! {
    pub assume_specification [my_transition] (
        state: MyMachineState,
        proof: Established<MyMachineConsistent>,
        // ... additional parameters
    ) -> (r: (MyMachineState, Established<MyMachineConsistent>))
        requires archive_machine_consistent(&state),
        ensures  archive_machine_consistent(&r.0),
    ;
}
```

The `;` at the end (no body) tells Verus to trust this contract as an axiom.
Callers can then reason from `requires`/`ensures` in their own proofs.

### Key differences from Creusot companions

| | Creusot | Verus |
|-|---------|-------|
| Mechanism | `#[requires]` / `#[ensures]` on a wrapper fn | `assume_specification` block |
| Body required? | Yes — calls the production fn | No — trusted axiom |
| Works cross-crate? | `extern_spec!` needed per crate | Yes — `assume_specification` is cross-crate |
| Gate | `#[cfg(creusot)]` | `#[cfg(verus)]` |

---

## 3. How Companions Are Generated

Companions are **auto-generated** by `crates/elicit_proofs/build.rs`. You
never write them by hand. To regenerate after changing a VSM:

```bash
cargo build -p elicit_proofs
```

### File layout

```text
crates/elicit_proofs/src/verus/generated/
├── mod.rs                    # Static: declares the four per-machine modules
├── archive_connection.rs     # Generated: ArchiveConnectionMachine companions
├── archive_nav.rs            # Generated: ArchiveNavMachine companions
├── archive_overlay.rs        # Generated: ArchiveOverlayMachine companions
└── archive_panel.rs          # Generated: ArchivePanelMachine companions
```

### build.rs structure

```rust
// Invariant predicate bodies (currently trivially true; Phase 4 adds real bodies)
const VERUS_INV_CONNECTION: &str = "\
#[cfg(verus)]
verus! { pub open spec fn archive_connection_consistent(_state: &ArchiveConnectionState) -> bool { true } }";

// Called for each machine
fn write_verus_vsm_file(
    out_dir: &str,
    machine_name: &str,       // e.g. "archive_connection"
    state_type: &str,         // e.g. "ArchiveConnectionState"
    consistent_type: &str,    // e.g. "ArchiveConnectionConsistent"
    inv_fn: &str,             // e.g. "archive_connection_consistent"
    inv_body: &str,           // the VERUS_INV_* constant
    contracts: Vec<String>,   // one assume_specification per transition
) { ... }
```

The contracts come from the `VerifiedStateMachine::transition_verus_contracts()`
method, which is auto-generated by `#[derive(VerifiedStateMachine)]` using
each transition's `#[formal_method]` companion struct.

### Marking the invariant type

Each machine's invariant type carries a `verus_invariant_fn` attribute
so the derive macro knows which spec function to use:

```rust
#[prop(
    kani_invariant_fn   = "archive_connection_consistent",
    creusot_invariant_fn = "archive_connection_consistent",
    verus_invariant_fn  = "archive_connection_consistent",
)]
pub struct ArchiveConnectionConsistent;
```

---

## 4. Problem 1 — Cross-Crate Spec Fn Transparency

### What could go wrong

In Verus, a `pub open spec fn` defined in crate A is *syntactically* visible
from crate B, but its **body** is only transparent if:

1. The producer exports it with `--export CRATE=path` flag, **and**
2. The consumer imports it with `--import CRATE=path` flag.

Without those flags, an `open` spec fn imported from another crate behaves as
uninterpreted: the prover sees the signature but not the body, so it cannot
unfold it to prove postconditions.

### The fix: redeclare inline

`build.rs` defines each invariant predicate as a raw string constant
(`VERUS_INV_*`) and embeds it verbatim in the generated companion file.
This means the `pub open spec fn archive_connection_consistent` is **declared
in `archive_connection.rs` itself**, not imported from elsewhere.

Verus sees the body and can unfold it during verification without any
`--export`/`--import` flag dance.

This is analogous to the Creusot fix (inlining `#[logic]` predicates into
each companion file), but simpler: no `const &str` roundtrip through
`prettyplease` is needed because Verus files are written as raw strings
(Verus syntax is not valid Rust, so `syn::parse_file` would fail).

---

## 5. Problem 2 — Async Transitions

### What could go wrong

Production VSM transitions are `async fn`. Verus is a synchronous verifier —
it cannot model `async`/`await` semantics.

### Why it does not matter for companions

`assume_specification` does not require Verus to call or verify the function
body. It only annotates the **synchronous signature** as a trusted axiom.
The async-ness is irrelevant; the contract is about the relationship between
the input state and the output state regardless of how the transition is executed.

```rust
// Production transition (async, not verified):
pub async fn begin_connect_sql(
    state: ArchiveConnectionState,
    proof: Established<ArchiveConnectionConsistent>,
    profile_name: String, backend: BackendKind,
) -> (ArchiveConnectionState, Established<ArchiveConnectionConsistent>) { ... }

// Companion contract (sync signature, trusted axiom):
#[cfg(verus)]
verus! {
    pub assume_specification [begin_connect_sql] (
        _state: ArchiveConnectionState,
        proof: Established<ArchiveConnectionConsistent>,
        profile_name: String, backend: BackendKind,
    ) -> (r: (ArchiveConnectionState, Established<ArchiveConnectionConsistent>))
        requires archive_connection_consistent(&_state),
        ensures  archive_connection_consistent(&r.0),
    ;
}
```

---

## 6. The Gallery Curriculum (V1–V10)

The gallery in `crates/elicitation_verus/src/gallery/` is a ten-level
learning curriculum that validates each Verus capability needed by the
production VSM companions before applying it to real archive code.

| Level | File | Theme | Key concept |
|-------|------|-------|-------------|
| V1 | `level1.rs` | Unit type baseline | `pub open spec fn`, basic `requires`/`ensures` |
| V2 | `level2.rs` | Two-variant enum | Z3 ADT theory, pattern-match in spec fn |
| V3 | `level3.rs` | Enum with `u64` field | Arithmetic invariant, overflow guard |
| V4 | `level4.rs` | Spec fn visibility tiers | `pub open` vs `pub closed` vs `#[verifier::opaque]` |
| V5 | `level5.rs` | `String` fields | `vstd` string model (`s@ == Seq::<char>::empty()`) |
| V6 | `level6.rs` | `Tracked<WfToken>` | Linear ghost tokens, `proof fn` mint/advance/consume |
| V7 | `level7.rs` | Full VSM pattern | Multi-variant enum + `String` + `u64`, multiple transitions |
| V8 | `level8.rs` | `#[verifier::type_invariant]` | Auto-checked invariant, private fields, spec accessors |
| V9 | `level9.rs` | `assume_specification` | Trusted axiom for external/async fn |
| V10 | `level10.rs` | Proof composition | `Tracked<T>` through two transitions, exec compose |

V7 is the **key level**: it proves that a state machine with the exact shape of
a production archive machine (multi-variant, `String` + `u64` fields, multiple
transitions all preserving the same `pub open spec fn` invariant) can be fully
verified by Verus without per-variant harnesses.

V9 is the **mechanism level**: it validates `assume_specification` in isolation
before we rely on it for every archive transition companion.

All ten levels pass: `verification results:: 780 verified, 0 errors`.

---

## 7. Running Verification

### Gallery only

```bash
just verify-verus
```

This runs Verus on `crates/elicitation_verus/src/lib.rs` (the gallery crate,
which is excluded from the normal workspace). Expected output:
`verification results:: 780 verified, 0 errors`.

### Tracked run

```bash
just verify-verus-tracked     # Full run, CSV output
just verify-verus-resume results.csv   # Resume (skip passed)
just verify-verus-summary results.csv  # Show statistics
```

### Companion files only (normal cargo check)

```bash
just check elicit_proofs
```

This compiles `elicit_proofs` including the generated companion files. The
companions contain only `#[cfg(verus)]`-gated code, so the normal Rust
compiler silently ignores the `verus! { }` blocks. The check confirms that
all imports, types, and `mod` declarations are correct.

---

## 8. Extending the Companion Suite

### Adding a new VSM machine

1. **Add `verus_invariant_fn` to the `#[prop]` attribute** on the machine's
   invariant type:

   ```rust
   #[prop(verus_invariant_fn = "my_machine_consistent")]
   pub struct MyMachineConsistent;
   ```

2. **Add a `VERUS_INV_*` constant** in `build.rs` with the invariant predicate body:

   ```rust
   const VERUS_INV_MY_MACHINE: &str = "\
   #[cfg(verus)]
   verus! { pub open spec fn my_machine_consistent(_state: &MyMachineState) -> bool { true } }";
   ```

3. **Call `write_verus_vsm_file`** from `generate_verus_companions()` for the
   new machine, passing `MyMachine::default().transition_verus_contracts("my_machine_consistent")`.

4. **Add the new module** to `src/verus/generated/mod.rs`:

   ```rust
   pub mod my_machine;
   ```

5. **Add a placeholder file** `src/verus/generated/my_machine.rs`:

   ```rust
   // AUTO-GENERATED — see build.rs
   ```

6. Rebuild: `cargo build -p elicit_proofs`

### Writing a real invariant body (Phase 4)

Currently all `VERUS_INV_*` constants return `true`. When you add a real
invariant, model it as a `pub open spec fn` with a `match` on the state enum:

```rust
const VERUS_INV_MY_MACHINE: &str = "\
#[cfg(verus)]
verus! {
    pub open spec fn my_machine_consistent(state: &MyMachineState) -> bool {
        match state {
            MyMachineState::Idle => true,
            MyMachineState::Active { name } => name@.len() > 0,
        }
    }
}";
```

Key rules:
- Use `s@` for `String` fields (`s@ == Seq::<char>::empty()` for empty, `s@.len() > 0` for non-empty).
- Use `*count > 0` for `u64` fields (the `*` dereferences the pattern binding).
- Import `vstd::prelude::SpecOrd` if you use `>` or `<` in the body.

---

## 9. Phase Status

| Phase | Status | Description |
|-------|--------|-------------|
| Phase 1 | ✅ Done | Gallery V1–V6 (toolchain, patterns, ghost tokens) |
| Phase 2 | ✅ Done | Gallery V7–V10 (full VSM, type_invariant, compose) |
| Phase 3 | ✅ Done | `vsm_verus_proof()` infra + auto-generated companion files |
| Phase 4 | ✅ Done | Real invariant bodies in `VERUS_INV_*` constants |
| Phase 5 | ✅ Done | Standalone Verus VSM contracts via `strictly_verus` crate |
| Phase 6 | 🔲 Planned | Standalone `elicit_verus` crate — archive machines verified like `strictly_verus` |

### Phase 4 notes

Real invariant bodies are in place in `elicit_proofs/build.rs`:

- `archive_panel_consistent`: `SqlEditor { running, result, .. }` → `running ==> result.is_None()`
- `archive_nav_consistent`: `NavFiltered { filter, .. }` → `filter@.len() > 0`
- `archive_overlay_consistent`: `ExportPickerOpen` → `idx <= formats@.len()`, `SavedBrowserOpen` → `idx <= entries@.len()`
- `archive_connection_consistent`: trivially `true` (no cross-field constraints by design)

These are generated into `crates/elicit_proofs/src/verus/generated/*.rs` at `cargo build -p elicit_proofs`.
The generated files use `#[cfg(verus)]` and are invisible to normal Rust builds.

**Verification status**: The companion files compile as Rust (under `#[cfg(verus)]` gates) but have not
yet been run through the Verus toolchain. This is blocked by the same rlib incompatibility that Phase 5
solved for game crates — `elicit_server` types are compiled by rustc, not vargo. Phase 6 addresses this
by creating a standalone `elicit_verus` crate analogous to `strictly_verus`.

### Phase 5 notes

Phase 5 is implemented in `strictly_games` via the `strictly_verus` crate, not
`vargo`.  The key insight is that the `VerifiedStateMachine` trait's new
`vsm_verus_transitions()` method generates **fully self-contained** Verus source
files — each file defines inline stub types and `#[verifier::external]` stubs
alongside `assume_specification` contracts.

The `vsm_verus_transitions()` implementation is in three parts:

1. **`formal_method.rs`** — `verus_external_stub()` generates the
   `#[verifier::external] pub fn fn_name(inputs) output { todo!() }` stub
2. **`derive_vsm.rs`** — `transition_verus_stubs()` collects all stubs from
   the machine's `#[formal_method]` transitions
3. **`contracts.rs`** — `vsm_verus_transitions()` composes stubs + contracts
   into a single `TokenStream` that `strictly_verus/build.rs` materialises

Results: `blackjack` 11 verified, `craps` 8 verified, `tictactoe` 9 verified
— all with 0 errors.  Run via `just verify-verus-vsm` in `strictly_games`.
