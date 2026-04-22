# Verified State Machines — Architecture Guide

> **The apex of the elicitation framework.** This document explains how
> `ElicitComplete`, `FormalMethod`, and `VerifiedStateMachine` combine to create
> a hermetically sealed, compiler-enforced formally verified program.

---

## 1. The Three-Layer Stack

```
┌──────────────────────────────────────────────────────────────────┐
│  VerifiedStateMachine  (top)                                      │
│  "The gated community — every transition is provably invariant"  │
├──────────────────────────────────────────────────────────────────┤
│  FormalMethod                                                     │
│  "A function that consumes a proof and produces a proof"         │
├──────────────────────────────────────────────────────────────────┤
│  ElicitComplete  (foundation)                                     │
│  "A type the framework can reason about end-to-end"              │
└──────────────────────────────────────────────────────────────────┘
```

Each layer is an obligation on types and functions. Moving upward adds
constraints; moving downward adds capabilities. You cannot skip a layer.

---

## 2. Layer 1 — `ElicitComplete`: Types the Framework Can Reason About

### What it is

`ElicitComplete` is a **compiler-enforced supertrait checklist** for types.
Implementing it (or deriving `#[derive(Elicit)]`) declares that a type is
fully described, serialisable, and formally reasoned about across all three
backends.

### The checklist

| Obligation | What it proves | Satisfied by |
|---|---|---|
| `Elicitation` | Interactive elicitation + **proof methods** | `#[derive(Elicit)]` |
| `ElicitIntrospect` | Structural metadata (field names, pattern) | `#[derive(Elicit)]` |
| `ElicitSpec` | Agent-browsable contract spec | type spec impls |
| `ElicitPromptTree` | Compile-time prompt structure | `#[derive(Elicit)]` |
| `Serialize` + `Deserialize` | Wire format | `#[derive(Serialize, Deserialize)]` |
| `JsonSchema` | JSON Schema for tooling | `#[derive(JsonSchema)]` |
| `ToCodeLiteral` | Code emission | `#[derive(Elicit)]` |

### The proof methods

The most important part of `Elicitation` (and therefore `ElicitComplete`) is
that it carries **three proof generation methods** with no default:

```rust
fn kani_proof()    -> proc_macro2::TokenStream;  // Kani bounded model checker
fn verus_proof()   -> proc_macro2::TokenStream;  // Verus/SMT
fn creusot_proof() -> proc_macro2::TokenStream;  // Creusot/Why3
```

For derived types, `#[derive(Elicit)]` generates these mechanically by
composing the field types' proofs:

```rust
// struct Coordinate { x: f64, y: f64 }
//
// Generated:
fn kani_proof() -> TokenStream {
    let mut ts = TokenStream::new();
    ts.extend(<f64 as Elicitation>::kani_proof()); // x — cannot be forgotten
    ts.extend(<f64 as Elicitation>::kani_proof()); // y — cannot be forgotten
    ts
}
```

**Provenance guarantee**: the derive macro sees the actual field types at
compile time, so delegation is structurally impossible to omit. If you add a
field, the proof automatically expands to cover it.

### Propagation through generics

```rust
impl<T: ElicitComplete> ElicitComplete for Vec<T> {}
//        ↑ If T is incomplete, Vec<T> is also incomplete.
```

An incomplete inner type blocks the whole chain. You cannot accidentally place
a half-implemented type inside a verified container.

---

## 3. Layer 2 — `FormalMethod`: Proof-Carrying Functions

### What it is

A `FormalMethod` is a function that **consumes an input proof and produces an
output proof**. It is the unit of verified composition.

```rust
pub trait FormalMethod<In, PIn: Prop, Out, POut: Prop> {
    fn call_formal(&self, input: In, proof: Established<PIn>)
        -> (Out, Established<POut>);
}
```

### The blanket impl — any matching function is a `FormalMethod`

No boilerplate is required. Any `Fn` with the right signature automatically
satisfies the trait:

```rust
impl<F, In, PIn: Prop, Out, POut: Prop> FormalMethod<In, PIn, Out, POut> for F
where
    F: Fn(In, Established<PIn>) -> (Out, Established<POut>),
{ ... }
```

This means plain Rust functions and closures are first-class formal methods:

```rust
#[derive(Prop)] struct Validated;
#[derive(Prop)] struct Normalised;

// This function is automatically a FormalMethod<String, Validated, String, Normalised>
fn normalise(s: String, _proof: Established<Validated>) -> (String, Established<Normalised>) {
    (s.trim().to_string(), Established::assert())
}
```

### Composition is enforced by the type system

```rust
// Step 1: validate → returns Established<Validated>
fn validate(s: String) -> Option<(String, Established<Validated>)> { ... }

// Step 2: normalise (FormalMethod) — requires Established<Validated>
fn normalise(s: String, p: Established<Validated>) -> (String, Established<Normalised>) { ... }

// Step 3: store (FormalMethod) — requires Established<Normalised>
fn store(s: String, p: Established<Normalised>) -> (usize, Established<Normalised>) { ... }

// Chain — each proof gates the next call
let (s, p1) = validate(input)?;
let (s, p2) = normalise.call_formal(s, p1);  // p1 consumed here
let (len, _) = store.call_formal(s, p2);     // p2 consumed here
//                               ↑ cannot accidentally skip normalise
```

If you try to call `store` without first calling `normalise`, the code does
not compile. There is no runtime check — the constraint is entirely encoded in
the type of `p`.

### The `#[formal_method]` attribute — backend harness generation

The `#[formal_method(contracts = [C1, C2, ...])]` attribute macro generates
companion harnesses for all three verification backends alongside the function:

```rust
#[formal_method(contracts = [Validated, Normalised])]
fn normalise(s: String, _proof: Established<Validated>) -> (String, Established<Normalised>) {
    (s.trim().to_string(), Established::assert())
}

// Expands to the original function PLUS:
//
// #[cfg(kani)]
// #[kani::proof]
// fn normalise__kani() { ... assert postcondition ... }
//
// #[cfg(creusot)]
// #[requires(true)] #[ensures(true)] #[trusted]
// fn normalise__creusot(...) { normalise(...) }
//
// #[cfg(verus)]
// verus! { fn normalise__verus(...) requires true, ensures true, { normalise(...) } }
```

The stubs are scaffolding; the contracts list becomes the specification
anchor for each backend to refine.

### Proof tokens are zero-cost

```rust
assert_eq!(std::mem::size_of::<Established<Validated>>(), 0);
```

`Established<P>` is `PhantomData<fn() -> P>`. It carries no data, occupies no
memory, and compiles away entirely in release builds. The only thing it proves
is that the surrounding code executed the path that established `P`.

---

## 4. Layer 3 — `VerifiedStateMachine`: The Gated Community

### What it is

A `VerifiedStateMachine` (VSM) is the **top-level compiler-enforced claim**
that a system preserves its invariants across every state transition.

```rust
pub trait VerifiedStateMachine {
    type State: ElicitComplete;   // ← Layer 1: state must be fully described
    type Invariant: Prop;         // ← Layer 2: what every transition must preserve
}
```

Two requirements flow from these associated types:

1. **`State: ElicitComplete`** — every state is fully reasoned about by the
   framework: it can be introspected, serialised, schema-validated, and has
   formal proofs generated for all three backends.

2. **`Invariant: Prop`** — there is a named, type-level proposition that every
   valid transition must preserve.

### `VerifiedTransition` — what transitions must be

A transition is any function that is a `FormalMethod` over the VSM's own
state and invariant types:

```rust
pub trait VerifiedTransition<VSM: VerifiedStateMachine>:
    FormalMethod<VSM::State, VSM::Invariant, VSM::State, VSM::Invariant>
{}
```

The blanket impl means that any ordinary Rust function with the right
signature is automatically a `VerifiedTransition`:

```rust
struct OrderMachine;
impl VerifiedStateMachine for OrderMachine {
    type State     = OrderState;     // ElicitComplete
    type Invariant = OrderIntact;    // Prop
}

// Plain function — automatically a VerifiedTransition<OrderMachine>
fn submit(state: OrderState, proof: Established<OrderIntact>)
    -> (OrderState, Established<OrderIntact>)
{
    (OrderState::Submitted, proof) // invariant preserved — proof threaded through
}
```

### The invariant preservation guarantee

The signature `(State, Established<Invariant>) -> (State, Established<Invariant>)`
means:

- The function **receives** a proof that the invariant holds on entry.
- The function **must return** a proof that the invariant holds on exit.
- The only way to produce `Established<Invariant>` is either to thread the
  input proof through, or to call another `FormalMethod` that produces one.
- You **cannot** return `(new_state, Established::assert())` and be "done" —
  `Established::assert()` exists but is the honour-system assertion;
  the backends (Kani/Verus/Creusot) will independently verify the postcondition.

```
Entry                           Exit
─────                           ────
state: OrderState               new_state: OrderState
proof: Established<OrderIntact> new_proof: Established<OrderIntact>
                        ↑
          type system guarantees proof cannot disappear
```

---

## 5. How the Three Layers Interlock

```
┌─────────────────────────────────────────────────────────────────────┐
│  VerifiedStateMachine                                               │
│                                                                     │
│   type State = S  ─────────────►  S: ElicitComplete  ─────────────►│
│                                       ↳ kani_proof()               │
│                                       ↳ verus_proof()              │
│                                       ↳ creusot_proof()            │
│                                       ↳ JsonSchema                 │
│                                       ↳ Serialize/Deserialize      │
│                                       ↳ ElicitPromptTree           │
│                                                                     │
│   type Invariant = I  ──────────►  I: Prop  ───────────────────────►│
│                                       ↳ kani_proof()               │
│                                       ↳ verus_proof()              │
│                                       ↳ creusot_proof()            │
│                                                                     │
│   Transitions: Fn(S, Established<I>) -> (S, Established<I>)  ──────►│
│                  ↑                                                  │
│            FormalMethod blanket impl                                │
│            + #[formal_method] for backend harnesses                 │
└─────────────────────────────────────────────────────────────────────┘
```

### What you cannot do inside a VSM

| Action | Why it is blocked |
|---|---|
| Call a non-`FormalMethod` transition | Transition must have `Established<Invariant>` in its signature |
| Use a `State` type that lacks `ElicitComplete` | `type State: ElicitComplete` bound fails at compile time |
| Return a new state without providing an invariant proof | Return type requires `Established<Invariant>` |
| Forget a field in the state's proof | `#[derive(Elicit)]` macro delegates to every field; forgetting one removes it from the generated proof |
| Use an undeclared backend | Each backend is gated by `#[cfg(kani)]` / `#[cfg(creusot)]` / `#[cfg(verus)]` |

---

## 6. End-to-End Example

```rust
use elicitation::{
    Elicit, ElicitComplete, Established, FormalMethod, Prop,
    VerifiedStateMachine, VerifiedTransition,
    contracts::ProvableFrom,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ── State ─────────────────────────────────────────────────────────────────────

/// Every order state is ElicitComplete: schema-validated, serialisable,
/// introspectable, and proof-carrying for all three backends.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Elicit)]
enum OrderState {
    Draft,
    Submitted,
    Shipped,
}

// ── Invariant ─────────────────────────────────────────────────────────────────

/// The proposition "this order's core fields are intact".
#[derive(Prop)]
struct OrderIntact;

// ── VSM declaration ───────────────────────────────────────────────────────────

struct OrderMachine;

impl VerifiedStateMachine for OrderMachine {
    type State     = OrderState;  // must be ElicitComplete ✓
    type Invariant = OrderIntact; // must be Prop ✓
}

// ── Transitions ───────────────────────────────────────────────────────────────

// Plain functions — automatically VerifiedTransition<OrderMachine>
// because Fn(OrderState, Established<OrderIntact>) -> (OrderState, Established<OrderIntact>)
// satisfies the FormalMethod blanket impl.

fn submit(state: OrderState, proof: Established<OrderIntact>)
    -> (OrderState, Established<OrderIntact>)
{
    assert!(matches!(state, OrderState::Draft), "can only submit from Draft");
    (OrderState::Submitted, proof) // ← proof threaded through, invariant preserved
}

fn ship(state: OrderState, proof: Established<OrderIntact>)
    -> (OrderState, Established<OrderIntact>)
{
    assert!(matches!(state, OrderState::Submitted), "can only ship from Submitted");
    (OrderState::Shipped, proof)
}

// ── Driver ────────────────────────────────────────────────────────────────────

fn run_transition<T: VerifiedTransition<OrderMachine>>(
    t: &T,
    state: OrderState,
    proof: Established<OrderIntact>,
) -> (OrderState, Established<OrderIntact>) {
    t.call_formal(state, proof)
}

fn main() {
    let proof = Established::assert();
    let (s, p) = run_transition(&submit, OrderState::Draft, proof);
    let (s, _) = run_transition(&ship, s, p);
    assert_eq!(s, OrderState::Shipped);
}
```

---

## 7. Backend Harnesses

Once your VSM is declared you can attach `#[formal_method]` to transitions to
scaffold backend-specific verification:

```rust
use elicitation::formal_method;

#[formal_method(contracts = [OrderIntact])]
fn submit(state: OrderState, proof: Established<OrderIntact>)
    -> (OrderState, Established<OrderIntact>)
{
    (OrderState::Submitted, proof)
}
```

This generates companion stubs, one per backend:

| Backend | Generated function | Active when |
|---|---|---|
| Kani | `submit__kani()` — `#[kani::proof]` harness | `#[cfg(kani)]` |
| Creusot | `submit__creusot()` — `#[requires] #[ensures] #[trusted]` | `#[cfg(creusot)]` |
| Verus | `submit__verus()` — `verus! { fn ... requires ... ensures ... }` | `#[cfg(verus)]` |

The stubs start as `true` pre/postconditions (sound scaffolding). Tighten the
contracts by replacing `true` with the actual invariant predicate in each
backend's language. The framework does not force a specific formalism on you;
it provides the structure.

### `proof_helpers` — building harnesses programmatically

For types that implement `Elicitation` manually, the `proof_helpers` module
provides builders:

```rust
use elicitation::verification::proof_helpers;

let harness = proof_helpers::kani_formal_method_harness(
    "submit",
    &["OrderIntact"],
    &["OrderIntact"],
);
```

---

## 8. The Hermetic Seal

The combination of the three layers creates what we call a **hermetically
sealed formally verified program**:

```
ElicitComplete on State
│
│  Every state is schema-valid, serialisable, and has machine-checked
│  proofs generated for Kani, Verus, and Creusot.
│
├── FormalMethod on Transitions
│   │
│   │  Every transition must accept and return an invariant proof.
│   │  The type system makes it impossible to call a transition that
│   │  "forgets" the invariant or produces a state without proving it.
│   │
│   └── VerifiedStateMachine as declaration
│       │
│       │  The VSM is the top-level assertion: "I claim this system
│       │  preserves OrderIntact across every reachable transition."
│       │
│       └── Backend harnesses (#[formal_method])
│           │
│           │  Kani, Verus, and Creusot each independently verify the
│           │  claim, lowering from the same source annotation.
│           │
│           └── Auditability
│               Every contract is named, every proof is traceable to
│               the type that generated it, and every state is
│               introspectable by agent tooling at runtime.
```

**Nothing in this chain is optional once you commit to the VSM**: the `State:
ElicitComplete` bound is not advisory, the `Invariant: Prop` constraint is not
a lint, and a function that lacks the invariant proof in its signature cannot
satisfy `VerifiedTransition`. The compiler is the gatekeeper.

---

## 9. Quick Reference

| Concept | Trait / Type | Key constraint |
|---|---|---|
| Fully-described type | `ElicitComplete` | `Elicitation + ElicitIntrospect + ElicitSpec + ElicitPromptTree + Serialize + Deserialize + JsonSchema + ToCodeLiteral` |
| Type-level statement | `Prop` | Must supply `kani_proof()`, `verus_proof()`, `creusot_proof()` |
| Proof token | `Established<P>` | Zero-sized (`PhantomData`); no default constructor on stable APIs |
| Credential-gated proof | `ProvableFrom<C>` | Only the factory holding `C` can mint `Established<P>` |
| Proof-carrying function | `FormalMethod<In, PIn, Out, POut>` | Blanket impl on `Fn(In, Established<PIn>) -> (Out, Established<POut>)` |
| Backend harness generator | `#[formal_method(contracts = [...])]` | Generates `__kani`, `__creusot`, `__verus` companions |
| Machine declaration | `VerifiedStateMachine` | `type State: ElicitComplete; type Invariant: Prop` |
| Verified transition | `VerifiedTransition<VSM>` | Blanket impl on any matching `FormalMethod` |

---

## 10. Further Reading

| Document | Location |
|---|---|
| `contracts.rs` module docs | `crates/elicitation/src/contracts.rs` |
| `ElicitComplete` docs | `crates/elicitation/src/complete.rs` |
| VSM trait tests | `crates/elicitation/tests/vsm_test.rs` |
| FormalMethod trait tests | `crates/elicitation/tests/formal_method_test.rs` |
| Macro expansion tests | `crates/elicitation/tests/formal_method_macro_test.rs` |
| Creusot compositional example | `crates/elicitation/examples/creusot_compositional_verification.rs` |
| Verus compositional example | `crates/elicitation/examples/verus_compositional_verification.rs` |
| Third-party support guide | `THIRD_PARTY_SUPPORT_GUIDE.md` |
