# The Contract System

> Standards are type systems written in prose. This document explains how
> elicitation finds their Rust form.

---

## Discovery, Not Construction

The contract system in elicitation was not designed from scratch. It was
discovered by following a sequence of forced moves:

1. Domain standards (OGC, ISO, GAAP) encode invariants that every correct
   implementation must honor. These invariants exist whether you name them or
   not.
2. Rust's type system can represent any invariant as a zero-sized type. The
   representation costs nothing at runtime.
3. Without dependent types, some invariants cannot be *proven* mechanically —
   but they can be *named and bounded*, exactly as `unsafe` names and bounds
   memory-safety assertions.
4. Formal verification tools (Kani, Verus, Creusot) are best at mathematical
   properties over bounded data. Pure structural facts are already proven by
   the compiler. Behavioral invariants need a human audit — but only at one
   specific site.

Each step follows from the previous. The result is the contract system:
proof-carrying types that make domain invariants explicit, compiler-enforced,
and auditable — without requiring theorem-proving skills or external tooling
to use.

---

## The Core Idea: Standards as Type Systems

The OGC, FASB, and ISO didn't know they were writing type systems. But they
were. Every statement of the form "this field is mandatory," "this transition
is forbidden," or "these two values must be consistent" is a proposition about
program state. Prose specifications are type systems without a type checker.

The contract system finds the Rust form of those specifications. Rather than
re-implementing a homegrown, ad hoc subset of a standard — which will
inevitably be incomplete and undocumented — elicitation's interface crates
(`elicit_gis`, `elicit_db`, `elicit_ui`) transcribe the standard directly into
named propositions:

```rust
// ISO 19115-1:2014 §6.2 — MD_Metadata / fileIdentifier
// "fileIdentifier is optional (0..1)"
pub struct MdMetadataFileIdentifierOptional;

// OGC Simple Features §2.1.3
// "All positions in a geometry must have consistent dimension"
pub struct AllPositionsInGeometryHaveConsistentDimension;

// GAAP double-entry invariant
// "Total debits equal total credits for every journal entry"
pub struct DebitEqualsCreditPerEntry;
```

Each struct is a zero-sized type — it occupies no memory — and its name is
the full, authoritative statement of the invariant. The complexity in these
crates is not imposed by elicitation. It is inherent domain complexity that
previously lived in documentation nobody read. The contract system moves it
into the type checker.

The alternative is not simplicity. It is the same complexity expressed
informally, scattered across comments, validation functions, and runtime
checks, with no compiler enforcement.

---

## `Established<P>`: The Proof Token

```rust
pub struct Established<P: Prop> {
    _marker: PhantomData<fn() -> P>,
}
```

`Established<P>` is a zero-sized type that means: *proposition P holds at
this point in the program*. It compiles away to nothing. It costs nothing to
carry through a call chain. But it cannot be constructed without going through
the designated proof site.

Functions that require an invariant to hold take `Established<P>` as a
parameter. Functions that cannot be called without proof *structurally*
cannot be called without proof — not by convention, not by documentation, but
by the type system.

```rust
// Cannot call this without proof that the move is legal.
pub fn execute_move(action: Move, game: &mut GameState, _proof: Established<LegalMove>) {
    // No defensive check needed. The proof carries the guarantee.
}
```

Proof tokens compose naturally:

```rust
// And<P, Q> is the proposition "both P and Q hold"
pub type LegalMove = And<SquareEmpty, PlayerTurn>;

let square_proof: Established<SquareEmpty>  = validate_square_empty(&action, game)?;
let turn_proof:   Established<PlayerTurn>   = validate_player_turn(&action, game)?;
let proof:        Established<LegalMove>    = both(square_proof, turn_proof);

execute_move(action, &mut game, proof); // ✅ Compiles
execute_move(action, &mut game, ???);   // ❌ Without proof, this is a type error
```

---

## The `unsafe` Analogy

This is the most important design insight in the system.

Rust's `unsafe` block is not a way to disable the compiler. It is a way to
*name and bound* an assertion that the compiler cannot verify:

> "I, the programmer, assert that the memory safety invariant holds here.
> The compiler cannot check it. The audit surface is this block."

`Established<P>` is the same pattern applied to domain invariants:

> "I, the trait implementor, assert that invariant P holds here.
> The compiler cannot check it. The audit surface is this trait method."

This analogy is structural, not cosmetic. In both cases:

- The assertion is explicit and named, not implicit
- The scope of the assertion is bounded to a specific site
- The rest of the codebase can rely on the assertion without re-checking it
- The total audit burden is proportional to the number of assertion sites, not
  the size of the codebase

For `unsafe`, the assertion site is a block. For contracts, it is a trait
method. The compiler enforces that callers cannot bypass the assertion — they
must hold `Established<P>`, which can only be constructed inside the
designated site.

---

## ZST Decoupling and Ineffable Invariants

The proof token (`Established<P>`) is deliberately decoupled from the trait
interface that upholds the invariant.

Some invariants are *effable*: they can be expressed as a mathematical
predicate and checked automatically. For these, the Kani/Verus/Creusot proofs
in the generated harness provide independent verification.

Some invariants are *ineffable*: they cannot be reduced to a mechanical check.
Consider:

```rust
pub struct FlowersArePretty;
pub struct MdMetadataContactResponsible; // ISO 19115 §6.5.3 - contact role is Responsible
```

There is no algorithm that decides whether flowers are pretty, or whether the
party named in a metadata record genuinely bears responsibility for it. The
proposition can be named. It cannot be proven by a machine.

For ineffable invariants, the trait method *is* the proof. The implementation
that mints `Established<P>` is the authoritative and canonical place where the
invariant is honored. Calling that method is the act of assertion, equivalent
to entering an `unsafe` block. The credential system enforces that only that
method can mint the token:

```rust
proof_credential! {
    /// Witness that responsibility was verified per ISO 19115 §6.5.3.
    pub(crate) ResponsibilityVerified => MdMetadataContactResponsible;
}

impl GisContactFactory for MyBackend {
    fn verify_responsible_party(
        &self,
        contact: &ContactDescriptor,
    ) -> Option<Established<MdMetadataContactResponsible>> {
        // This method is the audit surface.
        // The credential is pub(crate) — nothing outside this module can mint it.
        if self.check_responsibility(contact) {
            Some(Established::prove(&ResponsibilityVerified))
        } else {
            None
        }
    }
}
```

The credential is `pub(crate)`. No code outside the implementation crate can
construct it. The entire audit surface for `MdMetadataContactResponsible` is
the single method body above. To audit the invariant across an arbitrarily
large codebase, you read one function.

---

## Three Tiers of Proof

Not all propositions are proven the same way. The contract system spans three
tiers:

### Tier 1: Structural (proven by rustc)

The proposition's truth is guaranteed by type construction. No verifier is
needed.

```rust
pub struct MdMetadataFileIdentifierOptional;
```

The field is `Option<String>` in the descriptor type. The compiler proves that
`None` is always a valid value. The Kani harness for this proposition is empty
— not because it is unfinished, but because it is already complete. The
`/* structural */` comment in generated proof stubs is the correct answer, not
a placeholder.

### Tier 2: Mathematical (proven by Kani/Verus/Creusot)

The proposition makes a quantitative claim over data that a model checker can
verify exhaustively.

```rust
pub struct ReplicasInRange; // replicas >= 1 && replicas <= 16
pub struct TrialBalanceBalances; // sum(debits) == sum(credits) for all entries
```

Kani generates a harness with symbolic inputs and `kani::assert` to verify
the predicate holds for all reachable values. This is where formal verification
tools are most powerful: bounded integer arithmetic over a finite state space.

### Tier 3: Behavioral (proven by human audit at trait boundary)

The proposition makes a claim that no algorithm can verify. The trait method
is the assertion site; the credential restricts minting to that site; the
audit surface is the method body alone.

```rust
pub struct DeploymentApproved;  // a human approved this deployment
pub struct AccountingEquationHolds; // Assets = Liabilities + Equity, per GAAP intent
```

The `proof_credential!` macro ensures only the designated method can mint
the token. Auditing the invariant means reading one function.

---

## The Verified State Machine: Frame and Content

The `VerifiedStateMachine` derive generates complete Kani proofs for typed
state machines. A complete proof has two parts:

**The frame** — structural boilerplate, generated automatically, closes the
proof over the state space:

- Every variant is inhabited (a witness exists for each state)
- Every state is reachable (there is a valid path to each variant)
- The state enum is exhaustive (no unhandled variant)
- Symbolic constructors for all states at all depths (`KaniCompose`)

The frame proofs are structurally discharged. Their Kani harnesses are minimal
or empty. They are not incomplete — they are Tier 1 proofs.

**The content** — transition proofs, where Kani has substantive work to do:

- From `Setup`, `start_game` always produces `InProgress`, never `Finished`
- From `InProgress`, `make_move` with a valid `LegalMove` proof never panics
- From `Finished`, no transition function is reachable (terminal state)
- No sequence of inputs produces an illegal state

```rust
#[derive(VerifiedStateMachine)]
#[vsm(transitions = [ttt_start_game, ttt_make_move, ttt_restart])]
pub struct TicTacToeMachine;
```

This single derive generates the complete proof — frame and content — for the
tic-tac-toe state machine. The user writes neither.

The transition proofs are tractable because the state space is bounded: a VSM
with *n* states and *m* transitions has a finite, model-checkable reachability
graph. Kani exhausts it.

---

## Interface Crates: Decoupling Standards from Implementations

The interface crates (`elicit_gis`, `elicit_db`, `elicit_ui`) are not
implementations. They are contract vocabularies. Implementations depend on
them; consumers depend on them; nothing in the interface crate does any
computation.

```text
┌─────────────────────────────────────────────────┐
│ elicit_gis (interface)                          │
│  - Propositions from OGC, ISO 19111, RFC 7946   │
│  - Trait interfaces: GisCrsLookup, OgcGeometry  │
│  - Evidence bundles: CrsValid, MdMetadataValid  │
└───────────────┬─────────────────────────────────┘
                │ implements
     ┌──────────┼──────────┐
     ▼          ▼          ▼
 elicit_geo  elicit_proj  elicit_geojson
 (implementation crates)
```

This separation has two consequences:

1. **Object safety**: Trait methods return `Established<P>` rather than
   associated types. `dyn GisCrsLookup` is legal. Heterogeneous backends can
   be mixed at runtime without erasing the proof chain.

2. **Substitution**: Any backend that correctly implements the trait interface
   is provably contract-compliant. Testing with an in-memory stub is as
   contract-sound as production. The proofs transfer to any implementation.

The compiler enforces the chain. There is no way to produce
`Established<CrsValid>` without assembling the CRS authority, axis order, and
unit compatibility proofs from which it derives. A user cannot accidentally
skip a step — the type error is immediate.

---

## Why This Scales to Large Standards

ISO 19115-1 has hundreds of rules. GAAP has thousands. The proposition types
that represent these rules look like a lot of code. They are. But the
complexity is not elicitation's — it belongs to the standard. The only
question is whether that complexity is:

- **Implicit**: scattered across documentation, comments, convention, and
  runtime checks; invisible to the compiler; unauditable; guaranteed to drift
  from the standard over time, or

- **Explicit**: named as types; enforced by the compiler; auditable by
  reading trait implementations; guaranteed to be as complete as the type
  definitions say it is.

Elicitation chooses explicit. The resulting codebase is large, but it is
*all surface* — every proposition is a named, documented, inspectable claim
about a specific part of a specific standard. There is no hidden logic.

---

## Summary

| Concept | What it is |
|---------|-----------|
| `Prop` trait | A marker for types that name propositions |
| `Established<P>` | Zero-cost proof token that P holds here |
| `ProvableFrom<C>` | Declares which credential can mint `Established<P>` |
| `proof_credential!` | Binds a `pub(crate)` credential to a proposition |
| Structural proof | Truth guaranteed by type construction; rustc proves it |
| Mathematical proof | Truth over bounded data; Kani/Verus/Creusot prove it |
| Behavioral proof | Ineffable truth; trait method is the proof; audit surface is one function |
| VSM frame | Structural proofs that close the state space |
| VSM content | Transition proofs over bounded reachability graph |
| Interface crate | Contract vocabulary only — no implementation, no computation |

The rule of thumb: **if you can name the invariant, you can enforce it**.
Some invariants can also be verified independently. Most cannot. The contract
system does not require independent verification to be useful — it requires
only that the assertion be explicit, bounded, and auditable.
