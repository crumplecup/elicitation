# Verus VSM Plan

> **Implementation plan.** This document specifies how Verus deductive
> verification will be added to the `VerifiedStateMachine` proof infrastructure.
> It covers the gallery learning curriculum, the VSM companion pattern,
> `elicit_proofs` extension, and documentation deliverables.
>
> For the completed Kani backend, see [`KANI_FOR_VSMS.md`](KANI_FOR_VSMS.md).
> For the completed Creusot backend, see [`CREUSOT_FOR_VSMS.md`](CREUSOT_FOR_VSMS.md).
> For the Verus proof infrastructure (running, syntax, tiers), see
> [`VERUS_GUIDE.md`](VERUS_GUIDE.md).

---

## 1. What Verus Brings to the Table

The three backends are complementary, not redundant:

| Backend | Method | Scope | VSM role |
|---------|--------|-------|---------|
| Kani | Bounded model checking (CBMC) | All variants, depth ≤ 2 | Memory-safety, no-panic, bit-level |
| Creusot | Deductive via WhyML + Alt-Ergo | All inputs, unbounded | Functional correctness, loop invariants |
| Verus | Unbounded SMT via Z3 | All inputs, unbounded | Rich contracts, ghost state, type invariants |

Verus and Creusot both offer unbounded proofs, but through different toolchains. A property
verified by all three has been checked by three completely independent verification paths.

### Advantages over the other backends (discovered via research)

| Issue | Kani workaround | Creusot workaround | Verus: no workaround needed |
|-------|-----------------|--------------------|-----------------------------|
| Heap types in enums | One harness per variant × 3 depths | Companion functions | **Z3 ADT theory — single function** |
| `String::new()` unmodeled | `kani::any_vec` depth harnesses | `extern_spec!` per crate | **vstd full specs: `s@ == Seq::<char>::empty()`** |
| Cross-module predicate opacity | N/A | Inline `const INV_*` in generated file | **`pub open spec fn` — opt-in transparency** |
| External fn specs must be in calling crate | N/A | `extern_spec!` crate-local | **`assume_specification` works from any crate** |

### New Verus-specific capabilities

- **`#[verifier::type_invariant]`** — invariant automatically checked at every construction
  and field mutation; no Kani/Creusot equivalent.
- **`Tracked<T>`** — linear ghost tokens enforcing proper proof-witness threading.
- **`spec fn` / `proof fn` / `exec fn`** three-mode hierarchy with explicit body visibility
  tiers (`pub open` / `pub closed` / `#[verifier::opaque]`).

---

## 2. Key Pre-Research Findings

These were confirmed from the Verus documentation before writing gallery code.
Gallery levels validate that the installed version behaves as documented.

### 2.1 `spec fn` body visibility is explicit and opt-in

Three tiers:

```
pub open spec fn    — body transparent everywhere (cross-module, cross-crate with --import)
pub closed spec fn  — body private to declaring module; expose via proof fn lemmas
#[verifier::opaque] — body hidden even locally; expose with reveal(f)
```

**For VSMs**: use `pub open spec fn wf(s: &MyState) -> bool` — body transparent everywhere,
no workarounds needed. This eliminates the Creusot cross-module `#[logic]` opacity problem.

Cross-crate bodies require `--export CRATE=path` (producer) and `--import CRATE=path`
(consumer) flags. Without these, an `open` spec fn from a dependency behaves as uninterpreted.
Fallback: redeclare the spec in the consuming crate using `assume_specification`.

### 2.2 String model: `Seq<char>` via the `View` trait

```rust
// vstd/string.rs confirms:
impl View for String { type V = Seq<char>; }
impl View for str    { type V = Seq<char>; }

// s@ is shorthand for s.view()
fn validate(s: &str) -> bool
    requires s@.len() > 0 && s@.len() <= 100
{ ... }

// String::new() is fully modeled:
// ensures res@ == Seq::<char>::empty()
```

Operations: `unicode_len()`, `get_char(i)`, `substring_char(from, to)`, `append()`,
`concat()`, `as_str()`, `==` — all have vstd specs. **No `extern_spec` needed for stdlib.**

String literals are opaque by default; use `proof { reveal_strlit("text"); }` or `==` equality.

### 2.3 Enum handling: all variants simultaneously via Z3 ADT theory

```rust
spec fn wf(s: &ArchiveNavState) -> bool {
    match s {
        ArchiveNavState::NavLoading { label } => label@.len() > 0,
        ArchiveNavState::NavLoaded { label, items } =>
            label@.len() > 0 && items@.len() > 0,
        _ => true,
    }
}

fn transition(state: ArchiveNavState) -> (new: ArchiveNavState)
    requires wf(&state)
    ensures wf(&new)
{ ... }
```

Z3 reasons about `state` symbolically across all variants simultaneously. No per-variant
harnesses, no depth parameters. One function proves all cases.

### 2.4 Ghost tokens: `Tracked<T>` (linear) and `Ghost<T>` (non-linear)

```rust
struct WfToken { ghost state: MyState }

proof fn mk_token(s: MyState) -> (tracked tok: WfToken)
    requires wf(&s)
    ensures tok.state == s
{ WfToken { state: s } }

fn transition(
    exec_state: MyState,
    Tracked(tok): Tracked<WfToken>,
) -> (MyState, Tracked<WfToken>)
    requires tok.state == exec_state, wf(&exec_state)
{ ... }
```

`Tracked<T>` is linear — must be consumed or re-stored; cannot be duplicated.
`Ghost<T>` is non-linear — freely copyable, erased at compile time.
Both are zero-size at runtime.

### 2.5 External function specs: `assume_specification` is cross-crate

```rust
// In your verification crate — can spec any function from any crate:
pub assume_specification [production_crate::my_fn](arg: ArgType) -> (res: RetType)
    ensures res.field > 0;
```

Unlike Creusot's `extern_spec!` (which must be redeclared in every calling crate),
`assume_specification` in the verification crate propagates to all users of that crate.

### 2.6 Type invariants: `#[verifier::type_invariant]`

```rust
struct MachineState { phase: Phase, count: u64, name: String }

impl MachineState {
    #[verifier::type_invariant]
    spec fn wf(self) -> bool {
        match self.phase {
            Phase::Active => self.count > 0 && self.name@.len() > 0,
            _ => true,
        }
    }
}
// Verus auto-checks wf(x) at every construction + field assignment
// Must explicitly request in proofs: proof { use_type_invariant(&x); }
```

**Constraint**: requires all fields private (enforces encapsulation). May not be appropriate
for enums whose variant fields need to be public. The simpler `requires`/`ensures` approach
is the primary VSM pattern; `#[verifier::type_invariant]` is an optional enhancement.

### 2.7 `tokenized_state_machine!` is NOT needed for VSMs

The `tokenized_state_machine!` macro is designed for concurrent/ownership-discipline proofs
(generating linear token types for concurrent transitions). It is heavyweight infrastructure.

**For single-threaded VSM invariant preservation**: `requires wf(&state)` / `ensures wf(&new)`
is sufficient. The macro is noted here only to avoid confusion when reading Verus examples.

---

## 3. Gallery Design

The gallery provides an executable learning curriculum in `elicitation_verus`.
Levels confirm the research findings with real Verus runs before touching production code.

**Location**: `crates/elicitation_verus/src/gallery/`
**Prefix**: `V` (Kani uses `K`-prefixed harnesses, Creusot uses `C1`–`C30`)
**Structure**: one file per level, `mod.rs` declares all levels

Unlike the Creusot gallery (which discovered painful failures at C29/C30), many Verus
levels are expected to be positive confirmations. The gallery is still essential for:
- Validating that the installed Verus version behaves as documented
- Building muscle memory for Verus syntax
- Establishing the canonical VSM idiom before applying it to production
- Documenting any version-specific surprises for future contributors

### Level Table

| Level | File | Hypothesis | Tests |
|-------|------|------------|-------|
| V1 | `level1.rs` | Unit type, `pub open spec fn inv = true`, identity + constructor | Basic Verus workflow |
| V2 | `level2.rs` | 2-variant enum (no heap), `spec fn` pattern-matching variants | Enum ADT in spec |
| V3 | `level3.rs` | Enum with `u64` field, invariant = counter > 0, transition preserves | Arithmetic invariant |
| V4 | `level4.rs` | Spec fn visibility tiers: `open` vs `closed` cross-module | V4a open=transparent, V4b closed=opaque, V4c closed+lemma=works |
| V5 | `level5.rs` | String field with vstd specs: `@.len() > 0` in requires/ensures | String model correctness |
| V6 | `level6.rs` | `Tracked<WfToken>` pattern: mk_token, pass through transition | Ghost token for invariant |
| V7 | `level7.rs` | Full VSM: enum with String + u64, `wf` spec fn, multiple transitions | Core VSM pattern |
| V8 | `level8.rs` | `#[verifier::type_invariant]` on a struct — auto-check at construction | Type invariant feature |
| V9 | `level9.rs` | `assume_specification` for an unverified external function | External fn specs |
| V10 | `level10.rs` | Two-step composition: token from V6 used to sequence two transitions | Proof composition |

### Level Detail

#### V1 — Unit Type (baseline)

```
Hypothesis: verus! { pub open spec fn v1_inv(_s: &GUnit) -> bool { true } }
            fn v1_identity(s: GUnit) -> (r: GUnit) requires v1_inv(&s) ensures v1_inv(&r)
Expected: ✓ proves — confirms basic toolchain works
```

#### V2 — Simple Enum

```
Hypothesis: enum V2State { Off, On }
            pub open spec fn v2_wf(s: &V2State) -> bool { match s { V2State::Off => true, V2State::On => true } }
            fn v2_turn_on(s: V2State) -> (r: V2State) requires s matches V2State::Off ensures v2_wf(&r)
Expected: ✓ proves — Z3 ADT theory handles discriminant
```

#### V3 — Enum with u64 Field

```
Hypothesis: enum V3State { Idle, Active { counter: u64 } }
            spec fn v3_wf(s: &V3State) -> bool { match s { V3State::Active { counter } => counter > 0, _ => true } }
            fn v3_increment(s: V3State) -> (r: V3State) requires v3_wf(&s) ensures v3_wf(&r)
Expected: ✓ proves — arithmetic invariant in spec match
```

#### V4 — Spec Fn Visibility

```
V4a: pub open spec fn from module A, used in module B — Expected: ✓ transparent
V4b: pub closed spec fn from module A, used in module B — Expected: ✗ fails (body opaque)
V4c: pub closed spec fn from module A + pub proof fn lemma, used in module B — Expected: ✓ proves via lemma
Purpose: Documents the `open`/`closed` distinction; confirms `open` is the right choice for VSMs
```

#### V5 — String Fields

```
Hypothesis: struct V5State { name: String }
            spec fn v5_wf(s: &V5State) -> bool { s.name@.len() > 0 && s.name@.len() <= 100 }
            fn v5_new(name: String) -> (r: V5State) requires name@.len() > 0 && name@.len() <= 100 ensures v5_wf(&r)
            fn v5_rename(s: V5State, new_name: String) requires v5_wf(&s) && new_name@.len() > 0
Expected: ✓ proves — vstd String view provides full Seq<char> model
```

#### V6 — `Tracked<WfToken>` Pattern

```
Hypothesis: proof fn mk_v6_token(s: V3State) -> (tracked tok: V6Token) requires v3_wf(&s) ensures tok.state == s
            fn v6_transition(s: V3State, Tracked(tok): Tracked<V6Token>) -> (V3State, Tracked<V6Token>)
                requires tok.state == s, v3_wf(&s)
Expected: ✓ proves — linear token threads invariant witness through execution
Note: confirms Tracked<T> pattern before applying to production
```

#### V7 — Full VSM Pattern

```
Hypothesis: Multi-variant enum with String + u64 fields, multiple transitions
            all covered by a single spec fn wf, no per-variant boilerplate
Expected: ✓ proves — validates the production companion shape
This is the key level: if it proves, the VSM companion pattern is confirmed
```

#### V8 — Type Invariant

```
Hypothesis: struct V8Machine { phase: Phase, count: u64, name: String } with #[verifier::type_invariant]
            fn v8_activate(m: &mut V8Machine, name: String) mutates m — Verus auto-checks wf
Expected: ✓ proves — confirms type_invariant feature
Note: Documents private-fields constraint; compare to V7 (requires/ensures) approach
```

#### V9 — `assume_specification`

```
Hypothesis: assume_specification [some_unmodeled_fn](arg: T) -> (r: U) ensures r > 0
            fn v9_use() — calls the unmodeled fn, proves postcondition via assume_specification
Expected: ✓ proves — confirms cross-crate spec injection pattern for production companions
```

#### V10 — Composition

```
Hypothesis: sequence two V7-style transitions using a Tracked token from V6
            token from transition_a passed directly into transition_b
Expected: ✓ proves — confirms composition before applying to archive VSM pipeline
```

---

## 4. VSM Companion Pattern for Verus

Based on gallery findings (especially V7, V9), the production companion pattern differs
from both Kani and Creusot in key ways.

### Key differences

| Aspect | Kani | Creusot | Verus |
|--------|------|---------|-------|
| Location | `elicit_proofs/src/kani/` | `elicit_proofs/src/creusot/` | `elicit_proofs/src/verus/` |
| Gate | `#[cfg(kani)]` | `#[cfg(creusot)]` | `#[cfg(verus_verify_core)]` or feature |
| Per-variant? | Yes (one harness per variant × depth) | No (one fn per transition) | No (one fn per transition) |
| String handling | Depth harnesses, `kani::any_vec` | `extern_spec!` in same crate | `assume_specification`, vstd |
| External fn spec | N/A | `extern_spec!` | `assume_specification` |
| Invariant expression | `kani_proof_credential()` | `#[requires(consistent(&state))]` | `requires wf(&state)` |
| Token mechanism | `kani::any::<Credential>()` | `Established<T>` | `Tracked<WfToken>` |

### Proposed companion shape

```rust
// crates/elicit_proofs/src/verus/generated/archive_nav.rs
// AUTO-GENERATED by build.rs

use verus_builtin_macros::verus;

verus! {

pub open spec fn archive_nav_wf(state: &ArchiveNavState) -> bool {
    match state {
        ArchiveNavState::NavInitial => true,
        ArchiveNavState::NavLoading { label } => label@.len() > 0,
        ArchiveNavState::NavLoaded { label, items } =>
            label@.len() > 0 && items@.len() > 0,
        // ... remaining variants
    }
}

pub fn begin_nav__verus(
    state: ArchiveNavState,
    label: String,
) -> (result: ArchiveNavState)
    requires
        archive_nav_wf(&state),
        label@.len() > 0,
    ensures archive_nav_wf(&result)
{
    // delegates to production implementation
    elicit_server::archive::vsm::nav::begin_nav(state, label)
}

// ... one fn per transition

} // verus!
```

### `assume_specification` for production functions (alternative approach)

If the production functions cannot be called directly from the Verus crate
(e.g., async, network I/O), use `assume_specification` to attach specs:

```rust
// Declare the spec without calling the body — trusted axiom
pub assume_specification [elicit_server::archive::vsm::nav::begin_nav](
    state: ArchiveNavState,
    label: String,
) -> (result: ArchiveNavState)
    ensures archive_nav_wf(&result);
```

The gallery V9 level validates this pattern. Whether direct call or `assume_specification`
is needed depends on whether the production function is async (can't be called from Verus).

### Companion generation (`build.rs`)

The existing `build.rs` generates Creusot companions via `write_creusot_vsm_file`.
A parallel `write_verus_vsm_file` function will generate Verus companions:

1. Generate `pub open spec fn {machine}_wf(state: &{State}) -> bool` with match arms
   for each variant and its invariant condition.
2. For each transition in `Machine::vsm_verus_proof()`, generate one `pub fn ...__verus`
   with `requires` + `ensures` wrapping.
3. Gate with `verus! { }` macro block.
4. Commit to `src/verus/generated/{machine}.rs`.

The `VerifiedStateMachine` trait gains a `vsm_verus_proof()` method returning a
`TokenStream` (parallel to `vsm_creusot_proof()`), implemented via `#[formal_method]`
on production transition functions.

---

## 5. `elicit_proofs` Extension Plan

The scaffolding already exists:

```
crates/elicit_proofs/src/verus/
├── mod.rs       ← exists (stub)
└── generated/
    ├── mod.rs   ← exists (stub)
    └── archive.rs  ← exists (stub comment only)
```

Work items:

1. **Extend `crates/elicitation_verus/src/gallery/`** (Phase 1 & 2 above)
2. **Implement `VerifiedStateMachine::vsm_verus_proof()`** in `elicitation_derive`
   - New method on the `VerifiedStateMachine` trait
   - `#[formal_method]` expands to include a `vsm_verus_proof` branch
3. **Extend `build.rs`** with `write_verus_vsm_file` and call site
4. **Implement `extern_specs_verus.rs`** in `elicit_proofs/src/verus/`
   - `assume_specification` for any production functions that need it
5. **Run `cargo build -p elicit_proofs`** to regenerate
6. **Run `just verify-verus`** and confirm companions prove

---

## 6. Documentation Deliverables

### `VERUS_FOR_VSMS.md` (new, after Phase 2)

Parallel to `KANI_FOR_VSMS.md` and `CREUSOT_FOR_VSMS.md`. Sections:

1. What Verus verifies (vs Kani and Creusot)
2. The companion pattern
3. `spec fn` visibility and why `pub open` is used
4. String handling (`Seq<char>` model, no `extern_spec` needed)
5. `Tracked<WfToken>` — the invariant witness
6. `assume_specification` for production functions
7. `#[verifier::type_invariant]` — optional enhancement
8. Running the proof suite
9. Adding a new machine
10. Current status (tracking table)
11. Comparison with Kani and Creusot

### `VERUS_GUIDE.md` updates

- Add "Gallery" section covering the V1–V10 levels
- Add "VSM Companions" section linking to `VERUS_FOR_VSMS.md`
- Add `--verify-module` flag usage to "Running" section
- Cross-reference to `KANI_FOR_VSMS.md` and `CREUSOT_FOR_VSMS.md`

---

## 7. Work Phases

### Phase 1 — Gallery V1–V6 (fundamentals)

Prerequisites: none  
Output: `crates/elicitation_verus/src/gallery/` with levels 1–6, all verifying

Sequence:
1. Create `gallery/mod.rs` with level declarations
2. Add `pub mod gallery` to `lib.rs`
3. Write and run level 1 (unit type baseline)
4. Write and run levels 2–3 (enum + arithmetic)
5. Write and run level 4 (visibility tiers — split V4a/b/c)
6. Write and run level 5 (String)
7. Write and run level 6 (Tracked token)
8. Commit as `feat(elicitation_verus): add gallery V1–V6`

### Phase 2 — Gallery V7–V10 (VSM patterns)

Prerequisites: Phase 1 complete  
Output: Levels 7–10, establishing canonical VSM pattern

Sequence:
1. Write and run level 7 (full VSM — multi-variant enum + String)
2. Write and run level 8 (type_invariant — confirm constraint)
3. Write and run level 9 (assume_specification)
4. Write and run level 10 (composition)
5. Decide: direct call or assume_specification for production companions
6. Commit as `feat(elicitation_verus): add gallery V7–V10`

### Phase 3 — `elicit_proofs` Verus extension

Prerequisites: Phase 2 complete  
Output: Verus companions in `elicit_proofs/src/verus/generated/`

Sequence:
1. Implement `vsm_verus_proof()` on `VerifiedStateMachine` trait in `elicitation_derive`
2. Implement `write_verus_vsm_file` in `build.rs`
3. Run `cargo build -p elicit_proofs` to generate companions
4. Implement `assume_specifications.rs` if needed
5. Run `just verify-verus` — confirm all companions prove
6. Commit as `feat(elicit_proofs): add Verus VSM companions`

### Phase 4 — Documentation

Prerequisites: Phase 3 complete  
Output: `VERUS_FOR_VSMS.md`, `VERUS_GUIDE.md` updates, cross-references

Sequence:
1. Write `VERUS_FOR_VSMS.md`
2. Update `VERUS_GUIDE.md` (gallery + VSM sections)
3. Add cross-references in `KANI_FOR_VSMS.md` and `CREUSOT_FOR_VSMS.md`
4. Commit as `docs: add VERUS_FOR_VSMS; update VERUS_GUIDE`

---

## 8. Open Questions

These are resolved by the gallery, not by prior research:

| Question | Gallery level | Expected |
|----------|---------------|---------|
| Does `pub open spec fn` actually work cross-module in installed version? | V4a | ✓ |
| Does vstd `s@.len()` work with the installed toolchain version? | V5 | ✓ |
| Does `Tracked<T>` work through an `exec fn` parameter? | V6 | ✓ |
| Can `assume_specification` reach functions from an unverified crate? | V9 | ✓ |
| Does `#[verifier::type_invariant]` require private fields for enums too? | V8 | TBD |
| Can production async functions be called from Verus companions? | V9 (variant) | Likely ✗ → use assume_specification |
| Does `--export`/`--import` work for `elicitation_verus` → `elicit_proofs`? | Phase 3 | TBD |
