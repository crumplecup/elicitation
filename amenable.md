# The Amenable Trait Family

A declarative interface amenable to formal verification.

## Trait Verifier

The elicitation framework supports three different verifiers: Kani, Creusot and Verus. Verifier contains abstractions over the backend interface, making it easier for the calling side to backend agnostic. 

## Trait Witness

Witness takes a generic parameter V: Verifier These methods provide proofs for the verifier to consume, along the lines of here is the Kani proof in code, here is the same proof as a tokenstream. This type also includes methods for auditing proofs.

## Trait Standard

Standard is a type of Witness that references a normative standard, such as an ISO. It provides methods for querying the reference source, and auditing fidelity to the standard. The proofs for these types include authoritative references and informative descriptions, rather than mathematical operations. Like unsafe code blocks, implementations of the methods in Standard create an audit friendly surface for upholding this class of program invariants.

## Trait Objective

The Objective trait defines methods for types that express program goals that do not anchor directly to a third-party standard. 
Eg. FlowersArePretty

## Trait Evidence

Methods for types that depend on Standards or Objectives, including ways to audit the proof chain leading back to the originating Objective or Standard.
Eg. PolygonValid
* audit - For a given method, produce the source code associated with computing the result.

Additional convenience methods for analytics and heuristics.

Objective and Standard trivially implement Evidence by referring to their identity.

## Trait Sidecar

The Sidecar defines a primary type and an accompanying "proof" type. Imagine methods like:
* primary - the payload, arguments for a method call or the return data from a call
* sidecar - evidence token associated with the primary data

The primary data must implement Witness. The sidecar type must implement Witness and Evidence.

## Trait Establish

The Established trait represents the relationship between evidence tokens and proofs deduced from the accumulation of evidence. Tokens in this context are types that implement Evidence, and methods on this trait permit exchanging one type of evidence for another. For example:
* establish(T: Evidence) -> U: Evidence

## Trait Exchange

The Exchange trait defines the semantics of producing one Sidecar type, and receiving another Sidecar type in return. Exchanges are the process by which the trait interface composes evidence from leaf constructors into higher order abstractions that aggregate bundles of evidence. For example:
* exchange(T: Sidecar) -> U: Sidecar

Since each Sidecar contains Evidence, users must implement Establish to define legal exchanges of proof tokens, or they will not be able to produce the required proof to form a valid Sidecar type.

## Trait StateMachine

Methods expose a bounded set of states and transitions comprising a finite state machine.

## Amenable

A closed set of Exchanges, where the program is a State Machine and all state transitions implement Exchange.

Methods enumerate the set of Exchanges, all the transitions that are allowing inside the program, and provide accessors to the formal proofs associated with a given state or transition.

## Narrative

Standards and Objectives are ways to plug trusted assumptions into formal verifiers, in a way that makes them auditable. Carefully crafted human decision, based on policy or legal standards, can produce a contained audit trail of source code. Largely speaking, this comes down to embedding good metadata describing the condition, the reference standard or program invariant that it represents, links to the third party standards, or internal steps taken to uphold the objective. The metadata embedded into the formal proofs is the true value of trusted assumptions in formal models, because they create a paper trail detailing how the programmers intended to uphold their promises. This is a good thing because it sweeps up a broader class of program invariants than what we can prove with a few math operations.

Proofs generally refer to evidence that desired program invariants are preserved throughout all states of the program. Acquiring proofs can be computationally intensive (polygon with metadata), or ineffable and requiring human audit (flowers are pretty). The trait interface is only valid way to manufacture a proof token. This restricts the audit surface to the narrow bottleneck of the trait method implementations.
