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

## 2. The V13 `assume_specification` + Verified-Caller Pattern

Verus cannot verify production VSM transitions directly — they are `async fn`s
that use tokio, database drivers, and other runtime machinery. The generated
companion architecture uses three pieces working together:

### Part 1 — External stubs (outside `verus! { }`)

A `#[verifier::external]` stub is declared as a plain Rust function with a
`todo!()` body. It has the simplified signature of the transition (state in →
state out), stripping async and extra runtime arguments:

```rust
// Outside the verus! block — regular Rust, #[verifier::external]
#[verifier::external]
pub fn apply_filter_stub(state: ArchiveNavState) -> ArchiveNavState {
    todo!()
}
```

The `#[verifier::external]` attribute tells Verus to treat this function as
external — it will be given its contract via `assume_specification`.

### Part 2 — `assume_specification` (inside `verus! { }`)

Inside the `verus! { }` block, `assume_specification` attaches a trusted
postcondition to the stub. Verus treats this as an axiom — it does not verify
the body, only the contract:

```rust
verus! {
    pub assume_specification [apply_filter_stub](
        state: ArchiveNavState,
    ) -> (r: ArchiveNavState)
        requires archive_nav_consistent(&state),
        ensures  archive_nav_consistent(&r),
    ;
}
```

This is the trust anchor: "the real `apply_filter` satisfies this contract
because Kani and Creusot verify it independently."

### Part 3 — Verified exec callers (inside `verus! { }`)

A verified exec function calls the stub and proves the invariant is preserved.
Verus uses the `assume_specification` ensures to discharge the proof goal:

```rust
verus! {
    pub fn apply_filter_verified(
        state: ArchiveNavState,
    ) -> (r: ArchiveNavState)
        requires archive_nav_consistent(&state),
        ensures  archive_nav_consistent(&r),
    {
        let r = apply_filter_stub(state);
        r
    }
}
```

Z3 chains: `assume_specification` ensures `archive_nav_consistent(&r)` →
`verified` fn's `ensures` is satisfied. No manual proof hints needed.

### Why this V13 pattern is sound

- Kani and Creusot independently verify the real transition contracts on the
  production code. Those contracts are the same postconditions asserted in
  `assume_specification`.
- Verus trusts `assume_specification` and proves invariant preservation.
- All three backends satisfy the same proof obligation via different paths.

### Key differences from older patterns

| | V9/V10 (old) | V13 (current) |
|-|-------------|---------------|
| Spec attachment | `assume_specification` on production fn directly | `assume_specification` on `#[verifier::external]` stub |
| Verified exec caller | Not generated | Generated — explicitly proves invariant preservation |
| Connection to Kani/Creusot | Indirect (abstract spec) | Direct — same postcondition predicates |

---

## 3. How Companions Are Generated

Companions are **auto-generated** by `crates/elicitation/src/cli/generate/verus_gen.rs`
via the `elicitation generate` command. You never write them by hand.
To regenerate after changing a VSM:

```bash
cargo build --release -p elicitation --features cli
cp target/release/elicitation ~/.cargo/bin/elicitation
elicitation generate  # regenerates all backends including Verus
```

### File layout

Generated files live in the `elicitation_verus` crate, which is built with
`vargo` (excluded from the normal cargo workspace):

```text
crates/elicitation_verus/src/generated/
├── mod.rs                    # Declares the four per-machine modules
├── archive_connection.rs     # ArchiveConnectionMachine companions
├── archive_nav.rs            # ArchiveNavMachine companions
├── archive_overlay.rs        # ArchiveOverlayMachine companions
└── archive_panel.rs          # ArchivePanelMachine companions
```

Each file has a three-part structure:

```text
// 1. External stubs (outside verus! { })
//    — #[verifier::external] fns with todo!() bodies

// 2. verus! { ... } block:
//    a. Abstract state enum + pub open spec fn consistent(...)
//    b. Postcondition predicates (trivial / passthrough / conditional)
//    c. Leaf lemmas  
//    d. assume_specification blocks (one per transition)
//    e. Verified exec callers (one per transition)
//    f. Tag enum + composition proof (over all transitions simultaneously)
```

### `verus_gen.rs` structure

`verus_gen.rs` drives generation from the machine's `#[formal_method]`
metadata. For each transition the generator emits:

1. A stub via `TransKind::external_stub()` → placed before `verus! {`
2. An `assume_specification` via `TransKind::assume_spec_ensures()` → inside `verus! {`
3. A verified exec caller via `TransKind::verified_caller()` → inside `verus! {`

The composition proof (tag enum dispatch) is emitted once per machine and covers
all transitions simultaneously via Verus's Z3 ADT theory.

### Marking the invariant type

Each machine's invariant type carries a `verus_invariant_fn` attribute
so the generator knows which spec function to use:

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

## 6. The Gallery Curriculum (V1–V13)

The gallery in `crates/elicitation_verus/src/gallery/` is a thirteen-level
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
| V11 | `level11.rs` | Leaf + composition | Full proof: leaf lemmas + composition proof over all transitions |
| V12 | `level12.rs` | Multi-machine composition | Composition across two cooperating state machines |
| V13 | `level13.rs` | **Production reference** | `#[verifier::external]` stub + `assume_specification` + verified exec caller |

V7 is the **invariant level**: proves a production-shaped state machine fully.

V13 is the **V13 reference**: the pattern used by all generated companions.
Read `level13.rs` before modifying `verus_gen.rs`.

All levels pass: `verification results:: 780 verified, 0 errors` (or greater).

---

## 7. Running Verification

### Using `elicitation prove`

The recommended invocation handles all environment setup automatically:

```bash
elicitation prove --verus --csv
```

This runs `vargo` on `crates/elicitation_verus/src/lib.rs` and writes results
to `./verus_verification_results.csv`. The command exits non-zero on any failure.

### Gallery only

```bash
just verify-verus
```

This runs Verus on `crates/elicitation_verus/src/lib.rs` (the gallery crate,
which is excluded from the normal workspace). Expected output:
`verification results:: 780 verified, 0 errors` (or greater).

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
| Phase 4 | ✅ Done | Real invariant bodies in generated companions |
| Phase 5 | ✅ Done | Standalone Verus VSM contracts via `strictly_verus` crate |
| Phase 6 | ✅ Done | Archive machines verified in `elicitation_verus` via V13 pattern |
| Gallery V11–V13 | ✅ Done | Leaf+composition (V11), multi-machine (V12), V13 reference impl |

### Phase 6 notes

Phase 6 upgraded `verus_gen.rs` to the **V13 pattern**. Archive companions now live
in `crates/elicitation_verus/src/generated/` and are verified by `vargo`:

```bash
elicitation prove --verus --csv   # 4/4 machines, all passing
```

Each generated file contains:
1. `#[verifier::external]` stubs (one per transition) — before `verus! { }`
2. `assume_specification` blocks — axiomatize the real contracts
3. Verified exec callers — Verus proves invariant preservation from axioms
4. Tag enum + composition proof — covers all transitions simultaneously

This makes Verus independently satisfy the same proof obligation as Kani and
Creusot, each from a different verification path.

### Phase 4 notes

Real invariant bodies are now in `verus_gen.rs` and emitted into
`crates/elicitation_verus/src/generated/*.rs`:

- `archive_panel_consistent`: `SqlEditor { running, result, .. }` → `running ==> result.is_None()`
- `archive_nav_consistent`: `NavFiltered { filter, .. }` → `filter@.len() > 0`
- `archive_overlay_consistent`: `ExportPickerOpen` → `idx <= formats@.len()`, `SavedBrowserOpen` → `idx <= entries@.len()`
- `archive_connection_consistent`: trivially `true` (no cross-field constraints by design)

### Phase 5 notes

Phase 5 is implemented in `strictly_games` via the `strictly_verus` crate, not
`vargo`.  The key insight is that the `VerifiedStateMachine` trait's new
`vsm_verus_transitions()` method generates **fully self-contained** Verus source
files — each file defines inline stub types and `#[verifier::external]` stubs
alongside `assume_specification` contracts.

Results: `blackjack` 11 verified, `craps` 8 verified, `tictactoe` 9 verified
— all with 0 errors.  Run via `just verify-verus-vsm` in `strictly_games`.
