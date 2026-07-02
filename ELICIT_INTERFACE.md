# Elicit Interface Architecture

> **For agents and contributors reading this codebase:** this document explains
> why `elicit_db`, `elicit_ui`, and `elicit_gis` exist, why they belong below
> their consumers, and why proof-carrying `ProvableFrom` exchanges are the
> canonical interface pattern in elicitation.

---

## The Thesis

The elicitation core crates provide the platform. The `elicit_*` shadow crates
are the leaves: the most remote consumers and producers, mirroring third-party
libraries as MCP-safe, proof-aware vocabularies.

Between those two levels sit the branch crates:

- `elicit_db`
- `elicit_ui`
- `elicit_gis`

These crates are not generic utility layers. They are interface accords.
They define the lawful exchange surface between independent bodies by naming:

- the contract types that may cross the boundary
- the propositions that must hold before an operation is lawful
- the proof tokens that establish those propositions
- the trait methods that are authorized to mint or consume those proofs

The point is not abstraction for its own sake. The point is governance.

---

## Standards Before Consumers

An interface crate should be defined lower in the stack than its consumers,
even when there is only one producer and one consumer today.

That lower placement matters because it keeps law distinct from policy.
Consumers should not be forced to invent their own standards, translate ad hoc
producer details, or rediscover invariants informally. They should receive a
shared contract vocabulary and operate on it mechanically.

This is analogous to civil society:

- laws and standards exist prior to any one transaction
- independent parties interact through those standards
- compliance is judged against the shared accord, not private intention

In elicitation, the interface crate is that accord. A producer either satisfies
it or it does not. A consumer either requires the proper proofs or it does not.
Judgment becomes an exercise in reading truth tables.

---

## The Trait Accordion

The right mental model for these interface crates is a trait accordion.

The folds of the accordion are the branch-level interfaces. Producers and
consumers are trapped like air between those folds. Data can move, but only by
passing through the accordion of contracts, trait methods, and proof exchanges
that the interface layer defines.

This constraint is a feature, not a limitation.

It prevents:

- backend details from leaking upward into consumers
- consumer-specific policy from leaking downward into producers
- ad hoc side channels that bypass the proof boundary
- repeated reinvention of the same validation logic in leaf crates

It enables:

- object-safe trait interfaces over shared contract types
- local reasoning about lawful and unlawful interactions
- reusable proof infrastructure across multiple leaves
- auditable authority boundaries that can be reviewed in one place

The consumer remains free to act, but only through the folds.

---

## Object Safety and Shared Contract Types

By operating on shared contract types rather than backend-specific concrete
types, the trait interface remains object safe for consumers.

That matters for two reasons.

First, it preserves ignorance in the right place. A consumer should not need to
know which backend produced a contract value, only that the value conforms to
the accord and is accompanied by the proofs the accord requires.

Second, it makes independent implementation possible. A new producer can be
judged by the same interface, with the same truth table, without requiring
changes in every consumer.

This is the architectural difference between:

- a consumer depending on a backend
- a consumer depending on a law

Elicitation prefers the latter.

---

## The One True Pattern: `ProvableFrom` Sidecars

The proof-carrying sidecar pattern backed by `ProvableFrom` exchanges is the
canonical interface pattern in this framework.

An interface method should not merely return useful data. It should return the
data together with the proof witness that authorizes the next lawful step.
Likewise, when a method requires an invariant, it should demand the explicit
token that establishes that invariant.

In schematic form:

```rust
fn perform_step(
    &self,
    input: ContractInput,
    _pre: Established<Precondition>,
) -> Result<(ContractOutput, Established<Postcondition>), InterfaceError>;
```

This pattern is stark, disciplined, and powerful because it makes authority
explicit. No consumer may act on a proposition it has not been given. No
producer may claim a proposition without minting the corresponding proof at the
designated trait method.

`ProvableFrom` is the sealed exchange grammar that turns this into an auditable
standard rather than a coding convention.

---

## Why This Pattern Is Architecturally Mandatory

In many frameworks, proof sidecars would be a style preference. In elicitation,
they are much closer to a buildability requirement.

The project automates proof generation across Kani, Creusot, and Verus. That
automation only scales if interface methods recurse through a stable proof
shape:

- typed inputs
- typed outputs
- named propositions
- explicit proof sidecars
- `ProvableFrom` exchanges that connect one lawful step to the next

This recursive shape is part of the secret sauce. It is what makes generated
proofs tractable.

The code generation macros do not need bespoke reasoning for every workflow if
every workflow speaks the same proof grammar. Each interface method becomes a
small, regular proof step. Verified state machines can then be assembled from
those steps and checked by all three backends without special casing every leaf
interaction.

If we drift away from this standard, proof generation becomes harder to derive,
harder to audit, and more likely to fail to build.

---

## The Interface Layer as an Authority Boundary

An interface crate does more than serialize values between leaves. It decides:

- what facts may cross the boundary
- what proofs must accompany those facts
- what propositions may be established at which methods
- which third-party standards define the legal vocabulary
- which responsibilities belong to producers versus consumers

That is why these crates belong below the consumer layer. The authority to name
and govern lawful exchange should not be owned by the most remote participant.

If a consumer must invent the accord for itself, the accord is already too
high in the stack.

---

## Incremental Adoption Is Still the Right Strategy

A proper interface layer takes time and thought. It should not be created
ceremonially or inflated beyond the real invariants of the domain.

Incremental adoption is the correct strategy. Introduce a branch-level
interface when all of the following are true:

- there is a real external standard or stable domain vocabulary to anchor it
- the consumer should remain ignorant of backend internals
- the interaction carries meaningful invariants, not mere data transport
- the proof boundary is worth auditing once in a shared location
- multiple leaves would otherwise start reinventing the same laws

When those conditions are not yet met, a direct coupling may be acceptable. But
once the proof burden, domain vocabulary, and reuse pressure appear, the branch
crate is no longer optional architecture. It is the correct home for the accord.

---

## Review Standard

Interface work in elicitation should be reviewed against a strict standard:

- contract types come from recognized, respectable domain standards where
  possible
- traits are object safe and speak only in shared contract vocabulary
- invariants are named as propositions, not hidden in comments or booleans
- authority crosses the boundary through `Established<_>` sidecars
- proofs are minted and consumed through `ProvableFrom`-shaped exchanges
- consumers do not re-derive backend facts informally
- no ad hoc escape hatches bypass the accord

The question is not merely "does this API work?" The question is:

> Does this API preserve the one proof grammar that lets the framework generate,
> compose, and verify lawful behaviour across every layer?

If the answer is no, the interface is not yet finished.

---

## Relationship to the Rest of the Framework

This document sits alongside the other major architectural papers:

- [README.md](README.md) explains the overall framework and workspace map.
- [CONTRACTS.md](CONTRACTS.md) explains why standards become propositions and
  why `Established<P>` is analogous to `unsafe` for domain invariants.
- [SHADOW_CRATE_MOTIVATION.md](SHADOW_CRATE_MOTIVATION.md) explains why leaf
  crates mirror third-party libraries as proof-aware vocabularies.
- `ELICIT_INTERFACE.md` explains why branch-level interface crates are the
  lawful folds between those leaves and the core.

Taken together, these documents describe one architecture:

1. name the domain laws as types
2. expose third-party vocabularies as safe leaves
3. govern exchange through branch-level interface accords
4. recurse through `ProvableFrom` proof sidecars
5. generate and verify larger behaviours, including verified state machines

That is the elicitation architectural stack.
