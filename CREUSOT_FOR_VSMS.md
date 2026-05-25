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

## 2. The Current Companion Pattern

Creusot for downstream VSMs is now a **same-crate generated companion** flow.
You keep the runtime transition in the consumer crate, then generate a Creusot
companion under that same crate:

```text
src/vsm/*.rs                      # runtime transitions
src/proofs/creusot/generated/*.rs # generated Creusot companions
verif/{pkg}_rlib/proofs/creusot/generated/ # COMA produced by cargo creusot
```

The generated surface is:

1. `extern_spec!` for the production transition
2. a local helper `{transition}_creusot_local(...)` when Creusot needs a
   tracing-free or proof-simplified body
3. a public wrapper `{transition}_creusot(...)` that Why3 proves

Current naming matters:

- public wrapper: `{transition}_creusot`
- local helper: `{transition}_creusot_local`
- spec helper: `{transition}_creusot_spec`

Do **not** reintroduce `__creusot` names in generated code. That convention is
obsolete.

---

## 3. Invariant Visibility Rule

Creusot needs the invariant body to be visible in the same generated module
that uses it. Cross-module `#[logic]` imports remain proof-hostile.

Gallery level C29 still captures the core rule:

| Experiment | Setup | Result |
|---|---|---|
| C29a | `#[logic]` fn defined in same file | ✅ proves |
| C29d | Same `#[logic]` fn imported from another module | ✗ fails |

### Current fix

Do **not** hand-maintain a separate Creusot invariant module for VSM proofs.
Instead, put the Creusot invariant body on the proposition itself:

```rust
#[derive(Prop)]
#[prop(
    kani_invariant_fn = "combat_consistent",
    creusot_invariant_fn = "combat_consistent",
    creusot_inv_body = r#"pearlite! {
        match state {
            CombatState::Uninitialized => true,
            CombatState::Active { combatants, turn_order, current_actor, round } =>
                combatants@.len() > 0
                && turn_order@.len() > 0
                && current_actor@ < turn_order@.len()
                && round@ > 0
                && forall<i: Int> 0 <= i && i < turn_order@.len()
                    ==> turn_order@[i]@ < combatants@.len(),
            CombatState::Concluded { .. } => true,
        }
    }"#
)]
pub struct CombatConsistent;
```

The scanner/generator turns that into an inline `#[logic]` function inside the
generated companion file, which keeps the body visible to Why3.

### Rule

- Put the Creusot invariant on `#[prop(...)]` via `creusot_inv_body`
- Make it **real**, not `true`, unless the state is genuinely trivial
- Do not rely on imported `#[logic]` helpers for the main invariant body

Trivial `true` invariants make proofs pass for the wrong reason and are a
regression, not a fix.

---

## 4. When Runtime Bodies Are Proof-Hostile

Some runtime transitions are correct but awkward for Creusot:

- traced delegation bodies
- proof-hostile indexing / sorting loops
- unmodeled stdlib calls
- runtime-only details that are irrelevant to the invariant proof

### Preferred fix order

1. Strengthen the invariant or preconditions if the proof is genuinely missing facts
2. Add crate-local `extern_spec!` models for unmodeled library calls
3. Use a **Creusot-only body override** for the generated local companion

### `creusot_body` override

`#[formal_method(...)]` supports a Creusot-only override:

```rust
#[formal_method(
    contracts = [CombatConsistent],
    creusot_requires = ["combatants@.len() > 0"],
    creusot_body = r#"{
        let turn_order: Vec<usize> = (0..combatants.len()).collect();
        (
            CombatState::Active {
                combatants,
                turn_order,
                current_actor: 0,
                round: 1,
            },
            proof,
        )
    }"#
)]
pub fn initialize_combat(...) -> (CombatState, Established<CombatConsistent>) {
    // runtime body can stay richer / more operational
}
```

Use `creusot_body` when the production body is valid but not worth forcing
through the solver. This is preferable to gutting the runtime code or weakening
the invariant.

### Stdlib modeling still matters

If the proof needs semantic reasoning about a library call, add a crate-local
`extern_spec!`. Extern specs do **not** propagate across crates.

See also the gallery guidance around string modeling and same-file logic bodies.

---

## 5. How the Current Generator Works

### Source-side annotations

The two important source annotations are:

1. `#[derive(Prop)] #[prop(...)]`
2. `#[formal_method(...)]`

`#[prop(...)]` provides:

- `creusot_invariant_fn = "..."`
- `creusot_inv_body = r#"pearlite! { ... }"#`

`#[formal_method(...)]` provides:

- `contracts = [MyConsistent]`
- optional `creusot_requires = ["..."]`
- optional `creusot_body = r#"{ ... }"#`

### Generation pipeline

```text
consumer crate source
  └─ scan_vsms(src/vsm)
      ├─ reads #[prop(...)] invariant metadata
      └─ reads #[formal_method(...)] transition metadata
           ↓
elicitation generate creusot --crate-path src/vsm --out src/proofs/creusot/generated
           ↓
generated same-crate companion file
```

Important behavior:

- simple delegation bodies like `{ other_fn(args) }` are rewritten to call
  `other_fn_creusot_local(args)` so Creusot does not descend into traced originals
- local helper bodies now carry the same `#[requires]` / `#[ensures]` as the
  wrapper, so delegated proofs compose correctly
- if `creusot_body` is present, it wins over the scanned runtime body

### Regeneration rule

`elicitation prove --creusot` proves what is already generated; it does not
magically refresh `src/proofs/creusot/generated/*.rs` in every consumer crate.
If you changed a VSM source file, regenerate first:

```bash
elicitation generate creusot --crate-path src/vsm --out src/proofs/creusot/generated
```

Or use the consumer crate's wrapper recipe, e.g.:

```bash
just generate-proofs-creusot
```

---

## 6. Running Proofs Correctly

### Recommended path

For downstream crates, use a dedicated workspace proof crate and run:

```bash
elicitation prove --creusot --csv
```

Current behavior is the normal workspace Creusot path:

1. generate companions into the proof crate
2. run `cargo creusot prove -- -p <proof-crate>`

There is no supported same-crate shadow-workspace path.

### Logs

Current log is:

- `prove_creusot.log`

If you see:

```text
🔬 Running cargo creusot prove…
📝 Logging to ./prove_creusot.log
```

you are on the supported workspace-proof-crate path.

### Guardrail: stale installed CLI

If `command -v elicitation` resolves to `~/.cargo/bin/elicitation`, make sure
it has been reinstalled after CLI changes:

```bash
cargo install --path crates/elicitation --features cli --bin elicitation --force
```

During development, `./target/debug/elicitation prove --creusot` is the most
reliable way to avoid accidentally using a stale installed binary.

### Consumer-crate workflow

Typical downstream loop:

```bash
just generate-proofs-creusot
just verify-creusot
```

For `valinoreth`, `verify-creusot` runs:

```bash
elicitation prove --creusot
```

so regeneration must happen first.

### Manual invocation

If you need to debug manually, use the normal workspace Creusot command:

```bash
cargo creusot prove -- -p <pkg>
```

---

## 7. Inspecting Failures

### Generated Rust companion

First inspect the generated Rust source:

```bash
sed -n '1,220p' src/proofs/creusot/generated/<machine>.rs
```

That shows:

- the inline `#[logic]` invariant
- the `extern_spec!`
- `{transition}_creusot_local`
- `{transition}_creusot`

### COMA / Why3 side

Then inspect the generated COMA root:

```bash
find verif -path '*proofs/creusot/generated*' -o -path '*creusot/generated*'
```

And prove a narrowed target if needed:

```bash
why3find prove verif/<pkg>_rlib/proofs/creusot/generated
```

Look for:

- `{false} any` → unmodeled call
- wrapper passes but `_creusot_local` fails → body issue
- wrapper fails and local passes → contract / delegation issue

### Common current failure shapes

1. **Index/swap obligations**
   - usually from proof-hostile loops
   - prefer `creusot_body` over contorting runtime code
2. **Invariant too weak**
   - add real structural facts to `creusot_inv_body`
3. **Stale generated file**
   - regenerate `src/proofs/creusot/generated` before proving
4. **Old installed CLI**
   - reinstall or run `./target/debug/elicitation`

---

## 8. Adding or Updating a VSM

### Step 1: write a real invariant

Use `creusot_inv_body` on the proposition:

```rust
#[derive(Prop)]
#[prop(
    creusot_invariant_fn = "my_machine_consistent",
    creusot_inv_body = r#"pearlite! { /* real structural invariant */ }"#
)]
pub struct MyMachineConsistent;
```

Do not default to `true` unless you can justify that all states are vacuously
well-formed.

### Step 2: annotate transitions

```rust
#[formal_method(
    contracts = [MyMachineConsistent],
    creusot_requires = ["payload@.len() > 0"]
)]
pub fn my_transition(...) -> (MyState, Established<MyMachineConsistent>) { ... }
```

If needed, add `creusot_body = r#"{ ... }"#`.

### Step 3: regenerate

```bash
elicitation generate creusot --crate-path src/vsm --out src/proofs/creusot/generated
```

### Step 4: prove

```bash
elicitation prove --creusot
```

Or the crate wrapper:

```bash
just verify-creusot
```

### Step 5: debug the right layer

- invariant wrong or too weak → source `#[prop(...)]`
- body too operational → `creusot_body`
- missing model → crate-local `extern_spec!`
- stale behavior → regenerate or reinstall CLI

---

## 9. Comparison with Kani and Verus

| Property | Kani | Creusot | Verus |
|---|---|---|---|
| Proof style | Bounded model checking | Deductive (SMT via WhyML) | Deductive (Z3 directly) |
| Coverage | All variants × bounded depth | All inputs simultaneously | All inputs simultaneously |
| String fields | `String::new()` (bounded) | crate-local `extern_spec!` model may be required | `vstd` full specs built-in |
| Async code | Not supported | Not supported | Not supported |
| Proof artefact | `proof_for_contract` harness | COMA + Why3 session | `assume_specification` + verified callers |
| Failure output | CBMC counterexample | Unproved goal + COMA position | Z3 counterexample |
| Composition | `stub_verified` | Contracts inline | `assume_specification` chains |
| Turnaround | ~30s per harness | ~5min for full suite | seconds for all files |
| Companion source | `src/kani/generated/*.rs` | `src/proofs/creusot/generated/*.rs` in the consumer crate | `elicitation_verus/src/generated/*.rs` |
| Generated by | derive + CLI pipeline | scanner + `elicitation generate creusot` | `verus_gen.rs` via `elicitation generate` |
| Per-variant harnesses? | Yes (diagnostic) / No (production) | No | No |
| Cross-crate fn specs | N/A | `extern_spec!` crate-local | `assume_specification` any crate |

The three backends are independent — not complementary slices of one proof.
Each satisfies the full proof obligation through a different verification path:
Kani via bounded model checking with DFCC contracts, Creusot via WhyML
deduction, Verus via Z3 directly with `assume_specification` trust anchors.
