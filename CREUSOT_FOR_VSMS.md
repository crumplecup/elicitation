# Creusot Verification for Verified State Machines

> **Operational guide.** This document explains how Creusot deductive
> verification is applied to `VerifiedStateMachine` types in this codebase:
> what problems were encountered, what the current design is, and how to run
> and extend it.
>
> For the architecture of VSMs themselves (layers, traits, type theory), see
> [`VERIFIED_STATE_MACHINES.md`](VERIFIED_STATE_MACHINES.md).
> For the Kani model-checking backend, see [`KANI_FOR_VSMS.md`](KANI_FOR_VSMS.md).
> For the Creusot proof infrastructure (contracts, Pearlite, extern specs), see
> [`CREUSOT_GUIDE.md`](CREUSOT_GUIDE.md).

---

## 1. What Creusot Verifies (vs Kani)

Kani is a bounded model checker. It exhaustively explores all execution paths
within a finite unwind depth and returns a counterexample if any assertion
fails. For VSMs, Kani verifies that every concrete variant and every bounded
input satisfies the invariant — but it only checks up to depth-2 collections.

Creusot is a **deductive verifier**. It translates Rust functions with
`#[requires]` / `#[ensures]` contracts into WhyML, then asks an SMT solver
to prove the postcondition holds given the precondition for **all inputs
simultaneously** — no bounding, no unwind limits. A Creusot proof is a
mathematical theorem.

For VSMs, the contract pattern is:

```rust
#[requires(machine_consistent(&state))]   // invariant holds on entry
#[ensures(machine_consistent(&result.0))] // invariant holds on exit
pub fn my_transition(state: MyState, proof: Established<MyConsistent>)
    -> (MyState, Established<MyConsistent>)
{ ... }
```

When Creusot proves this, it means: *for every possible input state satisfying
the invariant, the output state also satisfies the invariant* — regardless of
which variant `state` holds or how large its collection fields are.

---

## 2. The Companion Pattern

Creusot cannot be run directly on `elicit_server` — the production crate uses
async code, network I/O, and sqlx which cannot be translated to WhyML. Instead,
**companion functions** are generated in `elicit_proofs` that mirror the
production transitions using the same types but without the async runtime.

Each companion:
- Has the same signature as the production function
- Calls through to the same implementation via `elicit_server::archive::vsm::*`
- Carries `#[requires]` / `#[ensures]` contracts expressing the VSM invariant
- Is gated with `#[cfg(creusot)]` so it only exists during Creusot compilation

The companion name convention is `{transition_fn}__creusot`:

```rust
#[cfg(creusot)]
#[requires(archive_panel_consistent(&state))]
#[ensures(archive_panel_consistent(&result.0))]
pub(crate) fn column_detail__creusot(
    state: ArchivePanelState,
    proof: Established<ArchivePanelConsistent>,
) -> (ArchivePanelState, Established<ArchivePanelConsistent>) {
    column_detail(state, proof)   // delegates to production function
}
```

Companions are **auto-generated** by `crates/elicit_proofs/build.rs`. You
never write them by hand. To regenerate after changing a VSM:

```bash
cargo build -p elicit_proofs
```

---

## 3. Problem 1 — Cross-Module `#[logic]` Opacity

### What goes wrong

The VSM invariant for each machine is a `#[logic]` predicate (e.g.
`archive_panel_consistent`). In `vsm_invariants.rs` these are defined as pure
Pearlite functions. The generated companions import them:

```rust
use crate::creusot::vsm_invariants::archive_panel_consistent;
```

**When a `#[logic]` function is imported from another module, its body is
opaque in Why3/COMA.** Why3find sees:

```
predicate archive_panel_consistent (state : ArchivePanelState) (* no body *)
```

With no body, the only fact the solver has about `archive_panel_consistent` is
its postcondition: `result = archive_panel_consistent(state)`. It cannot
unfold the body to verify that the output state satisfies the predicate. Every
companion fails with:

```
Goal Coma.vc_..._creusot: ✘
```

### Gallery evidence

Gallery level C29 (`crates/elicitation_creusot/src/gallery/level29.rs`)
establishes this precisely:

| Experiment | Setup | Result |
|---|---|---|
| C29a | `#[logic]` fn defined in same file | ✅ proves |
| C29b | `#[trusted] #[logic]` fn defined in same file | ✅ proves |
| C29c | Non-trivial same-file `#[logic]` with match | ✅ proves |
| C29d | Same `#[logic]` fn imported from another module | ✗ fails |

C29d is included as a documented failure to confirm the diagnosis.

### The fix: inline the invariant

`build.rs` defines the invariant predicates as `const &str` and embeds them
directly in each generated companion file:

```rust
const INV_PANEL: &str = "\
#[cfg(creusot)]
#[logic]
pub fn archive_panel_consistent(state: &ArchivePanelState) -> bool {
    pearlite! {
        match state {
            ArchivePanelState::SqlEditor { running, result, .. } =>
                *running ==> match result { None => true, Some(_) => false },
            _ => true,
        }
    }
}";
```

This means `archive_panel_consistent` is defined in `archive_panel.rs`
(the file that uses it), not imported from `vsm_invariants.rs`. Why3find sees
the body and can unfold it.

**Rule:** Whenever you add a new VSM machine, define its `#[logic]` invariant
as a `const &str` in `build.rs` and pass it to `write_creusot_vsm_file`. Do
not `use` the invariant from `vsm_invariants.rs` in the generated companion.

---

## 4. Problem 2 — Unmodeled Stdlib Methods (`String::new()`, etc.)

### What goes wrong

Some VSM transitions call standard library methods that `creusot-std` does not
model. The clearest example is `String::new()` — used in `nav_loaded` to
initialise the `filter` field of `NavReady`.

When Creusot encounters a method call with no model, it emits `{false} any`
in the COMA output:

```
s6 = {false} any
```

`{false}` is an unprovable precondition. Any VC that depends on `s6` will
fail regardless of what the postcondition says.

Affected methods (not in `creusot-std` v0.10):

| Method | Where used |
|---|---|
| `String::new()` | struct literal initialisation (`filter`, `text` fields) |
| `String::is_empty()` | `apply_filter`, `clear_filter` |
| `String::push()` | `prompt_push` |
| `String::pop()` | `prompt_backspace` |
| `str::is_empty()` | via `Deref` from `String::is_empty()` |

### Gallery evidence

Gallery level C30 (`crates/elicitation_creusot/src/gallery/level30.rs`)
establishes the problem and both solutions:

| Experiment | Setup | Result |
|---|---|---|
| C30a | Companion calling `String::new()` in body | ✗ `{false}` |
| C30b | Same companion, marked `#[trusted]` | ✅ axiom accepted |
| C30c | `String::new()` isolated in `#[trusted]` helper | ✅ rest verifiable |

### Solution A: `extern_spec!` contracts (preferred)

When you need the solver to **reason about** the result of the method call
(e.g., `is_empty()` returns `true` iff the string has length 0), add a local
`extern_spec!` block to the crate where the call appears.

**Critical: extern specs are crate-local.** An `extern_spec!` in
`elicitation_creusot` is NOT visible to `elicit_proofs` even if you add
`elicitation_creusot` as a dependency. The spec must be defined in the crate
that makes the call.

`crates/elicit_proofs/src/creusot/extern_specs.rs` provides the needed specs:

```rust
extern_spec! {
    impl String {
        #[ensures(result@ == Seq::empty())]
        fn new() -> String;
        #[ensures(result == ((*self)@.len() == 0))]
        fn is_empty(&self) -> bool;
        #[ensures((^self)@ == (*self)@.push_back(ch))]
        fn push(&mut self, ch: char);
        #[ensures(match result {
            Some(t) =>
                (^self)@ == (*self)@.subsequence(0, (*self)@.len() - 1) &&
                (*self)@ == (^self)@.push_back(t),
            None => *self == ^self && (*self)@.len() == 0
        })]
        fn pop(&mut self) -> Option<char>;
    }
}

extern_spec! {
    impl str {
        #[ensures(result == ((*self)@.len() == 0))]
        fn is_empty(&self) -> bool;
    }
}
```

These extern specs let the solver reason through the method calls. All 7
production VSM companions that call these methods now prove.

### Solution B: `#[trusted]` companion (fallback)

When the transition body cannot be fully modelled (e.g. it builds a complex
struct from data the solver can't reason about), mark the companion
`#[trusted]`. The contract becomes an axiom — Why3find accepts it without
body analysis:

```rust
#[cfg(creusot)]
#[trusted]
#[requires(archive_nav_consistent(&_state))]
#[ensures(archive_nav_consistent(&result.0))]
pub(crate) fn complex_transition__creusot(...) { ... }
```

**When to use `#[trusted]`:** The `ProvableFrom` type system already enforces
that the invariant holds by construction — the canonical transition is the only
way to mint the output `Established<P>` token. A trusted Creusot companion
documents the contract as a formal axiom without requiring body analysis.

Reserve `#[trusted]` for transitions where the body truly cannot be verified
(complex iterator chains, opaque type constructors). Prefer `extern_spec!` for
individual stdlib methods.

---

## 5. The Machinery — How Companions Are Generated

### The pipeline

```
  1. #[formal_method] on each transition fn in elicit_server
         ↓  (at proc-macro expansion time)
  2. FooMachine::vsm_creusot_proof() → proc_macro2::TokenStream
         ↓  (at cargo build -p elicit_proofs)
  3. build.rs calls vsm_creusot_proof() + prepends inline inv_logic
  4. write_creusot_vsm_file() formats and writes src/creusot/generated/*.rs
```

### Stage 1: `#[formal_method]`

The `#[formal_method(contracts = [MyConsistent])]` attribute on a production
transition emits Creusot contracts on the original function via `cfg_attr`:

```rust
#[cfg_attr(creusot, requires(my_consistent(&state)))]
#[cfg_attr(creusot, ensures(my_consistent(&result.0)))]
pub fn my_transition(state: MyState, proof: Established<MyConsistent>)
    -> (MyState, Established<MyConsistent>)
{ ... }
```

It also generates the companion via `vsm_creusot_proof()`.

### Stage 2: `vsm_creusot_proof()`

`#[derive(VerifiedStateMachine)]` on the machine struct provides
`vsm_creusot_proof()`, which collects all transition companions into a single
`TokenStream`. Each companion function:

- Is named `{transition}__creusot`
- Has `#[cfg(creusot)]` gates
- Has `#[requires]` / `#[ensures]` with the invariant predicate
- Delegates to the production function

### Stage 3: `build.rs`

```rust
write_creusot_vsm_file(
    gen_dir,
    "archive_nav.rs",
    "ArchiveNavMachine",
    INV_NAV,           // inline #[logic] predicate — see §3
    &ArchiveNavMachine::vsm_creusot_proof(),
);
```

`write_creusot_vsm_file` prepends:

1. The `#[cfg(creusot)] use` imports
2. The inline `#[logic]` invariant definition (`INV_*` constant)
3. The companion function bodies from `vsm_creusot_proof()`

The output is formatted with `prettyplease` and written to
`src/creusot/generated/{machine}.rs`. These files are committed to source
control and are auditable.

### Updating the invariant

If you change the invariant logic in `vsm_invariants.rs`, you **must also
update the corresponding `const INV_*` string in `build.rs`** — the inline
copy in the generated file takes precedence and will become stale otherwise.

---

## 6. Running the Proofs

### Using `elicitation prove`

The recommended invocation wraps all environment setup automatically:

```bash
elicitation prove --creusot --csv
```

This reads `PATH`, `WHY3CONFIG`, and `DUNE_DIR_LOCATIONS` from `.env`, runs
`cargo creusot prove`, and writes results to `./creusot_verification_results.csv`
with a log at `./prove_creusot.log`. The command exits non-zero on any failure.

### Consumer projects

If you run `elicitation prove --creusot` in a crate other than `elicit_proofs`,
your proofs crate must declare the `creusot` feature in `Cargo.toml`. The prove
command always passes `--features creusot` to cargo:

```toml
[features]
kani    = []   # Marker feature — cargo kani probe
creusot = []   # Marker feature — cargo creusot prove
verus   = []   # Marker feature — vargo
```

`creusot-std` should remain an unconditional dependency (not gated by this
feature) since it is always needed to compile `#[cfg(creusot)]` code.

### Manual invocation

If you need to run directly without `elicitation prove`:

### Prerequisites

Creusot and Why3 must be installed. The toolchain lives at:

```
~/.local/share/creusot/bin/   — why3find, why3, creusot-rustc
~/.config/creusot/why3.conf   — solver configuration
~/repos/creusot/              — source checkout (for creusot-std)
```

Run the full proof suite:

```bash
PATH="${HOME}/.local/share/creusot/bin:${PATH}" \
DUNE_DIR_LOCATIONS="why3find:lib:${HOME}/.local/share/creusot/share/why3find" \
WHY3CONFIG="${HOME}/.config/creusot/why3.conf" \
cargo creusot prove -- -p elicit_proofs
```

The `just` recipe wraps this:

```bash
just prove-creusot     # full suite (both elicitation_creusot + elicit_proofs)
```

### What `cargo creusot prove` does

1. **Generates COMA files** — runs `cargo creusot` (Creusot's translation step)
   which invokes `creusot-rustc` to translate Rust → Why3/COMA. Output goes to
   `verif/{crate_name}_rlib/`.
2. **Runs why3find prove** — invokes Alt-Ergo/CVC5/Z3 on the generated COMA
   goals. Proof caches are stored in `verif/**/{fn_name}/proof.json`.

The `-p elicit_proofs` flag regenerates `verif/elicit_proofs_rlib/` but
**why3find proves everything in `verif/`** — including
`elicitation_creusot_rlib/`. Pass `-p elicitation_creusot` to regenerate
the gallery proofs.

### Checking compilation without proving

```bash
just check elicit_proofs
```

This compiles the crate (including generated companions) without running
Why3. Use this to verify that contracts parse and imports resolve before
spending time on a full prove run.

### Inspecting COMA for failures

When a proof fails, inspect the generated COMA file to find the bad goal:

```bash
# Find the COMA file for a failing function
ls verif/elicit_proofs_rlib/creusot/generated/archive_nav/

# The file is named {fn_name}__creusot.coma (without the module prefix)
cat verif/elicit_proofs_rlib/creusot/generated/archive_nav/nav_loaded_creusot.coma
```

Look for `{false}` at a value binding — this is the signature of an unmodeled
call:

```
s6 = {false} any    (* String::new() has no model *)
```

The surrounding context tells you which source line produced it. Cross-reference
with the transition source to find the offending method call.

### Proving a single COMA file directly

```bash
PATH="${HOME}/.local/share/creusot/bin:${PATH}" \
DUNE_DIR_LOCATIONS="why3find:lib:${HOME}/.local/share/creusot/share/why3find" \
WHY3CONFIG="${HOME}/.config/creusot/why3.conf" \
why3find prove -p creusot \
  verif/elicit_proofs_rlib/creusot/generated/archive_nav/nav_loaded_creusot.coma
```

This is faster for iteration — no full cargo build, just the Why3 side.

### Stale COMA cache

Creusot uses `target/creusot/` separate from the regular cargo target.
Stale fingerprints can cause COMA not to regenerate after source changes:

```bash
rm -rf target/creusot/debug/.fingerprint/elicit_proofs-*
rm -rf target/creusot/debug/deps/libelicit_proofs*
```

---

## 7. Expected Proof Status

### Production VSM companions (`elicit_proofs`)

| Machine | Companions | Status |
|---|---|---|
| `ArchiveConnection` | 7 | ✅ all prove |
| `ArchiveNav` | 8 | ✅ all prove |
| `ArchiveOverlay` | 10 | ✅ all prove |
| `ArchivePanel` | 23 | ✅ all prove |

All 48 production companions prove. The pre-existing `vc_c24_two_steps (4/6)`
gap is in the gallery (not production) and pre-dates this work.

### Gallery proofs (`elicitation_creusot`)

Gallery levels C1–C30 are proven as described in the gallery module docs.
`vc_c24_two_steps` (4/6) is a pre-existing gap.

---

## 8. Adding a New VSM Machine

### Step 1: Implement `VerifiedStateMachine` and `#[formal_method]`

In `elicit_server`, annotate each transition with
`#[formal_method(contracts = [MyConsistent])]`.

### Step 2: Add the invariant to `build.rs`

```rust
const INV_MY_MACHINE: &str = "\
#[cfg(creusot)]
#[logic]
pub fn my_machine_consistent(state: &MyMachineState) -> bool {
    pearlite! { /* invariant body */ }
}";
```

For trivially-true invariants (all states well-formed by construction):

```rust
const INV_MY_MACHINE: &str = concat!(
    "#[cfg(creusot)] #[logic] pub fn my_machine_consistent",
    "(_state: &MyMachineState) -> bool { true }",
);
```

**Do not** add a `use crate::creusot::vsm_invariants::my_machine_consistent`
import to the generated file — see §3.

### Step 3: Add the `write_creusot_vsm_file` call

```rust
write_creusot_vsm_file(
    gen_dir,
    "my_machine.rs",
    "MyMachine",
    INV_MY_MACHINE,
    &MyMachine::vsm_creusot_proof(),
);
```

### Step 4: Wire into `mod.rs`

```rust
// src/creusot/generated/mod.rs
pub mod my_machine;
```

### Step 5: Run the proofs

```bash
cargo build -p elicit_proofs   # regenerate companions
just prove-creusot              # prove everything
```

### Step 6: Handle unmodeled methods

If the prove run reports failures, inspect the COMA files (see §6). The most
common causes:

1. **`{false}` in a binding** — an unmodeled stdlib call. Add an `extern_spec!`
   to `crates/elicit_proofs/src/creusot/extern_specs.rs`.
2. **Cross-module opacity** — an imported `#[logic]` predicate (see §3). Move
   the predicate inline to `INV_MY_MACHINE` in `build.rs`.
3. **Complex body** — the transition builds types the solver can't reason about.
   Mark the companion `#[trusted]` in the `TokenStream` emitted by
   `vsm_creusot_proof()`.

---

## 9. Comparison with Kani and Verus

| Property | Kani | Creusot | Verus |
|---|---|---|---|
| Proof style | Bounded model checking | Deductive (SMT via WhyML) | Deductive (Z3 directly) |
| Coverage | All variants × bounded depth | All inputs simultaneously | All inputs simultaneously |
| String fields | `String::new()` (bounded) | `extern_spec!` model required | `vstd` full specs built-in |
| Async code | Not supported | Not supported | Not supported |
| Proof artefact | `proof_for_contract` harness | COMA + Why3 session | `assume_specification` + verified callers |
| Failure output | CBMC counterexample | Unproved goal + COMA position | Z3 counterexample |
| Composition | `stub_verified` | Contracts inline | `assume_specification` chains |
| Turnaround | ~30s per harness | ~5min for full suite | seconds for all files |
| Companion source | `src/kani/generated/*.rs` | `src/creusot/generated/*.rs` | `elicitation_verus/src/generated/*.rs` |
| Both generated by | `build.rs` | `build.rs` | `verus_gen.rs` via `elicitation generate` |
| Per-variant harnesses? | Yes (diagnostic) / No (production) | No | No |
| Cross-crate fn specs | N/A | `extern_spec!` crate-local | `assume_specification` any crate |

The three backends are independent — not complementary slices of one proof.
Each satisfies the full proof obligation through a different verification path:
Kani via bounded model checking with DFCC contracts, Creusot via WhyML
deduction, Verus via Z3 directly with `assume_specification` trust anchors.
