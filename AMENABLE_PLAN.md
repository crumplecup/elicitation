# AMENABLE_PLAN.md

## Goal

Introduce `amenable` as a small, dependency-light constitutional crate that
defines the trait family governing lawful proof exchange across the elicitation
ecosystem.

`amenable` is not intended to replace `elicitation`, nor to compete with the
existing `Established<P>` and `ProvableFrom<C>` machinery. Its role is to
distill the ad hoc architectural patterns that grew up around verified
workflows, interface crates, and proof sidecars into a coherent trait accord
that other crates can depend on as law.

This is a parallel track, not a migration campaign. The existing architecture
stays in place. `amenable` should run beside it like a second current: a new
constitutional line whose shape can be explored without forcing deprecation,
breakage, or immediate downstream adoption.

The first exploration point is not `elicit_temporal`. It is `elicitation`
itself, in a local `amenable` module, where the trait family can be tried
against the current proof machinery and standard library carriers before any
separate crate boundary is frozen.

## Architectural Thesis

The strongest interpretation of `amenable` is constitutional rather than
behavioral.

Its traits primarily define legal roles and admissibility criteria inside a
proof economy:

- which types are permitted to serve as trusted roots
- which types may count as derived evidence
- which exchanges are lawful
- which workflows are closed under those exchanges
- which seams must carry proof sidecars explicitly

At this stage, the trait bounds themselves matter more than a fully enumerated
method surface. Methods should emerge from real boundary pressure, not be
invented ceremonially.

## Relationship to Existing Elicitation Primitives

`amenable` should map directly onto the concrete proof grammar already present
in `elicitation`.

- `Prop` remains the primitive naming mechanism for propositions.
- `Established<P>` remains the canonical proof sidecar token.
- `ProvableFrom<C>` remains the canonical exchange relation for minting a new
  proof from a credential.
- `FormalMethod` and `VerifiedStateMachine` remain the concrete high-assurance
  execution story.
- `VerifiedStateMachine` should be understood as a concrete existing
  implementation of the broader `Amenable` idea: a closed world of lawful proof
  exchanges where all state transitions preserve declared invariants.

The plan should also acknowledge a deeper existing fact: `elicitation` already
contains an implicit analogue of the proposed `Verifier` and `Witness` roles.

- `Prop` itself requires `kani_proof`, `verus_proof`, and `creusot_proof`.
- `Elicitation`-participating types carry that proof-emission surface through
  the broader framework.
- `ElicitComplete` already treats those proof methods as required infrastructure
  and provides validation helpers over them.

So `Verifier` and `Witness` are not purely speculative abstractions. They are a
possible constitutional extraction and clarification of proof-emission roles
that already exist today, albeit implicitly and unevenly named.

`amenable` contributes the family story that explains why these pieces belong
together and how interface crates should structure themselves around them.

The desired long-term separation of concerns is:

- `amenable`: constitutional law for proof-bearing software structure
- interface crates such as `elicit_temporal`: domain law anchored in external
  standards
- `elicitation`: concrete proof-carrying machinery, macros, and verifier-facing
  infrastructure

The short-term incubation plan is narrower:

- prototype the trait family inside `elicitation`
- test blanket compatibility with existing proof primitives
- explore implementations over standard library types and existing proposition
  types
- only then decide whether the crate boundary should be split out

## Design Constraints

The crate should remain strict on structure and light on dependency burden.

- no unnecessary runtime dependencies
- no backend-specific verifier coupling in the core trait family
- no policy specific to one domain such as time, GIS, or UI
- no erosion of the explicit sidecar token pattern
- no reinvention of concrete time, geometry, or workflow payload types

The crate must earn its abstraction budget. A trait belongs in `amenable` only
if it tightens one of:

- auditability
- lawful proof exchange
- closure under composition
- separation between trusted roots and derived evidence

## Proposed Trait Family

The current candidate family, as sketched in [amenable.md](amenable.md), is:

- `Verifier`
- `Witness`
- `Standard`
- `Objective`
- `Evidence`
- `Sidecar`
- `Establish`
- `Exchange`
- `StateMachine`
- `Amenable`

This list should be treated as a worksheet, not a commitment.

The plan is to refine each trait against three questions:

1. Does it mark a distinct constitutional role?
2. Does it clarify a real proof boundary?
3. Does it map cleanly onto existing `elicitation` primitives?

Any trait that fails those tests should be re-examined carefully against the
constitutional design constraints that motivated the family in the first place.
Collapse, renaming, or removal is only justified when it preserves the original
architectural law more faithfully, not when it merely makes the surface more
convenient or superficially simpler.

## Core Design Principle: Explicit Sidecar Exchange

`amenable` should state plainly that explicit proof sidecars are the canonical
exchange grammar.

In schematic form:

```rust
fn perform_step(
    &self,
    input: Payload,
    _pre: Established<Precondition>,
) -> Result<(Output, Established<Postcondition>), Error>;
```

When a step is not a leaf constructor, proof movement should occur through a
lawful derivation of the form:

```rust
let new_proof = Established::<Postcondition>::prove(&existing_credential);
```

That pattern is the architectural center. `amenable` exists partly to explain
why that repetition is not accidental boilerplate but the governing discipline
of the framework.

## Semantic Framing of `Prop`

`Prop` should be understood as the semantic space of the system, not as a
single evidence role.

Different propositions participate in the constitutional story in different
ways:

- some propositions are roots of authority and can serve as their own evidence
  once refined into `Standard` or `Objective`
- some propositions are derived evidence and therefore must name the upstream
  `Standard` and `Objective` roots on which they depend
- some propositions may encode intermediate semantic steps that are neither
  roots nor terminal user-facing guarantees

Because of that distinction, `amenable` must not blanket-implement `Evidence`
for every `P: Prop`.

Instead, the user should have to define the semantics of each proposition
according to its kind:

- if the proposition is a `Standard`, it can serve as its own evidence root
- if the proposition is an `Objective`, it can serve as its own evidence root
- otherwise, the proposition must declare which existing `Standard` and
  `Objective` roots justify it

This is stricter than a blanket participation model, but it better reflects the
actual semantic law the framework is trying to encode.

It also fits the existing proof infrastructure. A proposition is not just a
name; it already participates in backend proof emission. What varies is the
semantic role that proposition plays inside the architecture.

## Proof-Quality Pressure from `Standard` and `Objective`

One practical value of the constitutional trait family is that it can raise the
quality bar for human- and agent-authored proof implementations.

The traits cannot prevent poor implementations outright. A bad implementation
can still satisfy a weak method body. But the trait surface can make the
expected proof hygiene explicit and visible, which is still a major
architectural improvement over today's uneven and often implicit practice.

For `Standard`, the goal is to communicate what a respectable external-standard
proof artifact should expose. Candidate methods or required disclosures should
cover things such as:

- the authorizing body
- the authoritative source or citation link
- the relevant clause, section, or scope of the source text
- a concise summary of the normative language being encoded
- the rationale for why the proposition faithfully represents that language
- the code-level audit surface responsible for upholding the claim

For `Objective`, the same pattern applies, but the source of authority is
architectural or programmatic rather than external. Candidate methods or
required disclosures should cover things such as:

- the author, owner, or originating design authority
- the architectural rationale for the objective
- how the objective fits into the broader proof architecture
- the intended guarantee or design invariant being claimed
- the code-level audit surface responsible for upholding the claim

The constitutional value is not merely metadata. These traits teach implementers
what kind of proof artifact counts as serious work. They turn vague proposition
types into declared claims with traceable authority, rationale, and audit
surfaces.

## Auditing as a First-Class Constitutional Concern

Another major value of the trait family is that it gives auditing a lawful home
inside the architecture instead of treating it as a later bolt-on concern.

The current system already has enough information to support substantial audit
capabilities, but without a constitutional design those capabilities would
likely emerge as scattered helper APIs or an extra trait layered on after the
fact. `amenable` is a better home because it lets auditing live inside the very
traits that define proof roles and exchange roles.

That matters because complete observability into invariant handling is not one
generic concern. It is the composition of several role-specific audit surfaces:

- `Standard` audits external authority, source, clause scope, summary, and
  fidelity rationale
- `Objective` audits design authority, authorship, architectural fit, intended
  invariant, and rationale
- `Evidence` audits dependency lineage back to the `Standard` and `Objective`
  roots that justify it
- `Exchange` audits the lawful transformation from one proof state to another,
  including the preconditions relied on and the postconditions established
- `Amenable` audits the closed set of lawful transitions and the overall
  invariant-preservation story of the system

Taken together, those trait-scoped audit surfaces promise a much stronger form
of observability than ad hoc metadata or a generic debug dump. The system can
explain:

- what it believes
- why it believes it
- which authority justifies the claim
- which code path is responsible for upholding it
- how one lawful proof state became another

The design constraint here is important: audit methods should remain
jurisdiction-specific. `amenable` should not collapse auditing into one vague
metadata trait. Each constitutional role should expose the audit surface proper
to that role.

## First Exploration Surface: `elicitation` Core

The initial experiment belongs in `elicitation` core, not in a downstream
interface crate.

That first pass should answer a narrower question than "can a domain adopt
this?": can the constitutional family be made to sit lawfully on top of the
existing proof algebra with minimal friction?

The first exploration should focus on:

- compatibility with `Prop` as semantic space rather than as a single evidence
  role
- blanket compatibility with `ProvableFrom`
- the relationship between propositions and constitutional root roles such as
  `Standard` and `Objective`
- the semantic distinction between root propositions and derived propositions
- how the already-existing proof emitter surface for Kani, Creusot, and Verus
  maps onto the proposed `Verifier` and `Witness` roles
- what it means to implement the family over standard library carriers or other
  low-level types already present in the core
- whether `Standard` and `Objective` can meaningfully improve proof quality by
  stating the expected audit and authority surface explicitly
- whether audit methods belong natively inside the constitutional traits in a
  role-specific way
- whether the trait family clarifies the existing VSM and proof-token story
  without requiring disruptive refactors

`elicit_temporal` remains a later proving ground, but not the first one.

## Phased Implementation Plan

### Phase 1: Core-side incubation module

Define an `amenable` module inside `elicitation` and write the first-pass trait
family with minimal or no methods where lawful.

- [ ] Add an exploratory `amenable` module inside `crates/elicitation`.
- [ ] Write module-level docs describing the constitutional role.
- [ ] Add the initial trait family as marker or near-marker traits.
- [ ] State the sidecar exchange pattern explicitly in docs.
- [ ] Document the intended mapping back to `elicitation` primitives.
- [ ] Keep the experiment parallel to existing APIs rather than entangling it
  with migrations.

### Phase 2: Blanket-impl bridge experiments

Test whether the constitutional traits can ride on top of the current proof
machinery through blanket impls and lightweight adapters.

- [ ] Explore a blanket bridge from `ProvableFrom<C>` to `Establish<C>` if that
  trait remains in the family.
- [ ] Reject blanket `Evidence` impls for all `P: Prop` and identify the
  narrower bridges that preserve semantic distinctions.
- [ ] Decide whether `Established<P>` itself should participate in traits like
  `Witness` or `Sidecar`, or whether those roles belong elsewhere.
- [ ] Explore helper pathways for standard library types that should participate
  in the constitutional story without inventing new payload hierarchies.
- [ ] Record which blanket impls preserve constitutional distinctions and which
  collapse them too aggressively.
- [ ] Sketch the minimum high-value disclosure surface for `Standard` and
  `Objective` without overcommitting to a large first-pass method inventory.
- [ ] Identify the smallest lawful extraction of the already-existing
  `kani_proof` / `creusot_proof` / `verus_proof` surface into explicit
  `Verifier` and `Witness` roles.
- [ ] Sketch the minimum high-value audit surface for each constitutional role
  without collapsing them into one generic audit API.

### Phase 3: Trait-family reduction pass

Interrogate the candidate family for redundancy and role confusion.

- [ ] Compare `Establish` against `ProvableFrom` and decide whether it is a
  real abstraction or just a renamed exchange relation.
- [ ] Compare `Exchange` against lawful trait methods and existing
  `FormalMethod` structure.
- [ ] Decide whether `Witness` and `Evidence` are meaningfully distinct.
- [ ] Decide whether `StateMachine` and `Amenable` are both needed, or whether
  one is a constitutional role and the other a concrete closed-world property.
- [ ] Separate verifier backend concerns from proof-ontology concerns.
- [ ] Distinguish truly new constitutional concepts from ones that are already
  present implicitly in `Prop`, `Elicitation`, and `ElicitComplete`.
- [ ] Decide whether root-role traits such as `Standard` and `Objective` can be
  meaningfully assigned through blanket structure, or must remain explicit.
- [ ] Decide which audit methods belong intrinsically to each trait role and
  which are merely convenience helpers that should stay out of the
  constitutional surface.

### Phase 4: Root-role assignment model

Clarify how a proposition becomes a constitutional root such as a `Standard` or
an `Objective` without dissolving the distinction between roots and derived
evidence.

- Current preferred direction:

  Root-role assignment should be an explicit refinement step in code. A plain
  `P: Prop` remains just a proposition until the user deliberately promotes it
  into a constitutional root role such as `Standard` or `Objective`.

  The favored mechanism is blanket `From<P>` conversion into explicit wrapper
  types, for example `AsStandard<P>` and `AsObjective<P>`, rather than blanket
  root-role impls on every proposition.

  This preserves convenience while keeping the authority boundary visible:

  ```rust
  let standard_root: AsStandard<MyProp> = my_prop.into();
  let objective_root: AsObjective<MyProp> = my_prop.into();
  ```

  Derived propositions should not gain `Evidence` status automatically merely by
  being `Prop`s. Instead, their semantics should point back to the upstream
  roots they depend on.

- [ ] Decide whether `Standard` and `Objective` are explicit marker roles on
  proposition types, wrapper roles, or the result of some more structured
  adapter.
- [ ] Reject any model that lets arbitrary derived propositions silently
  masquerade as roots of authority.
- [ ] If conversion pathways are desirable, define them as explicit and auditable
  promotions rather than accidental blanket erasure.
- [ ] Define how a derived proposition names or depends on its justifying
  `Standard` and `Objective` roots.
- [ ] Identify which existing proof roots in `elicitation` would be the first
  lawful examples.

### Phase 5: Verified workflow concordance

Check whether the constitutional model cleanly explains the existing VSM story
without forcing premature refactors in the core crate.

- [ ] State plainly that `VerifiedStateMachine` is a concrete implementation of
  `Amenable`, not a competing architectural concept.
- [ ] Confirm that proof-carrying state transitions remain explainable as lawful
  exchanges under the new vocabulary.
- [ ] Identify any mismatch between the constitutional story and the actual code
  generation pipeline for Kani, Creusot, and Verus.
- [ ] Record which mismatches are acceptable incubation debt versus true design
  flaws.

### Phase 6: Downstream proving grounds

Only after the core-side constitutional story is coherent should `amenable`
move outward into interface crates.

- [ ] Re-evaluate whether `elicit_temporal` is still the best first external
  proving ground.
- [ ] Adopt only the smallest useful subset of the family in a downstream crate.
- [ ] Keep the original and `amenable` tracks in parallel until the value of the
  new current is demonstrated in practice.

## Design Checklist

The plan should be reviewed against this checklist as the crate takes shape.

- [ ] Every trait has a distinct constitutional role.
- [ ] No trait exists solely to rename an existing concept without adding
  clarity.
- [ ] The crate remains dependency-light.
- [ ] The sidecar token pattern is explicit and central.
- [ ] Domain-neutral law stays in `amenable`; domain-specific law stays in
  interface crates.
- [ ] Backend-verifier coupling stays out of the constitutional core unless it
  is proven necessary.
- [ ] `Prop` is treated as semantic space, not collapsed into one undifferentiated
  evidence role.
- [ ] `Standard` and `Objective` communicate a visible quality bar for proof
  artifacts instead of leaving that bar implicit.
- [ ] If `Verifier` and `Witness` remain in the family, they must clarify the
  already-existing proof-emission infrastructure rather than duplicating it
  under new names without added value.
- [ ] `Amenable` explains `VerifiedStateMachine` as one concrete realization of
  the constitutional pattern rather than as a separate or rival design.
- [ ] Auditing is built into the constitutional roles themselves rather than
  bolted on later as an afterthought.
- [ ] Audit methods remain role-specific and jurisdiction-specific instead of
  collapsing into a shapeless metadata surface.
- [ ] The trait family makes the complexity of repeated proof exchange easier to
  understand rather than harder.
- [ ] Blanket impl experiments clarify the current proof algebra instead of
  obscuring it.
- [ ] Root-role traits such as `Standard` and `Objective` remain meaningfully
  distinct from generic derived evidence.
- [ ] Downstream adoption happens only after the core-side model earns it.

## Open Questions

These questions should remain visible until resolved by implementation pressure.

- [ ] Should `Standard` and `Objective` be subtraits of `Evidence`, or should
  they remain distinct constitutional roots that merely participate in the
  broader proof economy?
- [ ] If `Prop` can be adapted into `Standard` or `Objective`, what explicit
  audit boundary keeps that promotion lawful?
- [ ] What is the right trait-level mechanism for expressing that a derived
  proposition depends on particular `Standard` and `Objective` roots?
- [ ] What is the smallest first-pass method surface that meaningfully raises
  proof quality for `Standard` and `Objective` without prematurely freezing the
  design?
- [ ] What is the smallest first-pass audit surface that gives meaningful
  invariant observability without prematurely freezing every trait's method
  inventory?
- [ ] Should `Verifier` and `Witness` be explicit traits at all, or is the
  existing proof-emitter surface on `Prop` and `Elicitation` already the right
  concrete home for that responsibility?
- [ ] Does `Sidecar` need to be a first-class trait, or is payload plus
  `Established<P>` sufficient as the canonical shape?
- [ ] Is `Verifier` part of the constitutional crate, or should verifier-facing
  abstractions stay in `elicitation` proper?
- [ ] Should `Amenable` name a general closed proof system, or specifically the
  closed-world state-machine story?
- [ ] Which traits need methods immediately, and which should remain marker
  bounds until a real seam forces behavior?

## Success Condition

This plan succeeds when `amenable` becomes a lightweight, load-bearing crate
that makes the framework's proof discipline more explicit, more teachable, and
easier to adopt incrementally.

Concrete success looks like:

- a small trait family with clear roles
- direct mapping from constitutional traits to existing proof machinery
- successful blanket compatibility with core proof primitives
- a coherent root-role model for `Standard` and `Objective`
- successful downstream proving once the core experiment is stable
- no accidental reinvention of domain payload types
- a clearer explanation of why explicit sidecar proof exchange is the one true
  pattern
