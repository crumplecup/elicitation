Elicitation: Vision and Implementation Plan
1. Vision

Elicitation is a framework for constructing conversational programs with machine-checked guarantees.

It enables developers to:

Obtain strongly typed values from LLM- or human-mediated interaction with semantic soundness: on success, the value inhabits the requested type and its invariants.

Compose these interactions with tool calls into stateful programs whose correctness properties extend over time.

Express and verify contracts—preconditions, postconditions, and invariants—between steps in an agent’s execution.

Constrain LLM behavior so that it operates inside the semantic space of a Rust program, rather than emitting Rust source code.

The result is a system where:

LLMs act as nondeterministic oracles navigating a Rust-defined, contract-checked state space.

This is not prompt engineering, workflow orchestration, or schema validation.
It is programming with conversational effects, backed by Rust’s type system and formal verification.

2. Core Conceptual Model
2.1 Soundness over syntax

Most LLM tooling validates representations (JSON, schemas, ASTs).
Elicitation validates inhabitation:

If elicitation succeeds, the constructed value is a member of the target type, not merely a well-formed representation.

This gives a fundamental guarantee:

You never receive “an orange when you asked for a banana.”

Failure is explicit and recoverable.

Success is semantically meaningful.

2.2 Actions as typed transitions

Elicitation introduces action traits (e.g. Select, Affirm, Survey) that represent classes of conversational steps.

These are not helpers; they are typed transitions:

Each action has admissible preconditions.

Each action establishes postconditions.

Successful execution refines the program’s state.

This induces a typestate-like model:

State is refined monotonically.

Illegal transitions are unrepresentable.

Guarantees accumulate over time.

2.3 Contracts as first-class structure

To compose guarantees across steps, the system introduces a minimal contract calculus.

A contract is:

A compile-time proposition (a type).

Witnessed by a value returned from a successful step.

Carried forward to constrain future steps.

At minimum, the system supports:

Establishment: a proposition is now true.

Implication: one proposition entails another.

Conjunction: multiple propositions hold simultaneously.

Refinement: the reachable state space has narrowed.

These are proof-carrying markers, not runtime logic.

2.4 Tools as contract-preserving functions

Methods and functions exposed as MCP tools are treated as:

Effectful functions with explicit preconditions and postconditions.

Transitions between contract states.

Units of composition in larger agent programs.

A “tool chain” is therefore:

An explicit normal form of a Rust program.

A sequence of contract-checked transitions.

Observationally equivalent (modulo surfaced effects) to the original program.

2.5 Agents as verified programs

An “agent” is not an autonomous entity.
It is a program whose control flow is partially delegated to an LLM, but whose legal behaviors are statically constrained.

In this model:

The LLM chooses which legal action to attempt.

The type system and contracts determine whether that action is admissible.

Formal verification ensures that no execution path violates stated invariants.

3. The Full Vision

When fully realized, the system enables:

A library of elicitable types with machine-checked invariants.

A library of tools with explicit contracts.

A small set of logical primitives for expressing guarantees between steps.

Agent programs built by composing these pieces, not by free-form prompting.

Formal verification (via Kani) that:

illegal tool sequences are unreachable

required guarantees are established before use

invariants are never violated on any execution path

In effect:

Rust becomes the agent’s operational semantics, not its output language.

4. Implementation Plan

This is intentionally incremental and scoped.

Phase 1: Solidify the Contract Core (Foundational)

Goal: Introduce a minimal, stable contract abstraction without disturbing existing APIs.

Deliverables:

Prop marker trait (propositions as types).

Established<P> witness type.

Implies trait for logical weakening.

And<P, Q> for conjunction.

Refines<From, To> for monotonic narrowing.

Constraints:

Zero runtime cost.

No attempt at general logic.

No quantifiers, negation, or disjunction in v1.

Success criteria:

Contracts can be returned, combined, and required by functions.

Kani can reason about simple contract flows.

Phase 2: Integrate Contracts with Elicitation

Goal: Make elicitation explicitly contract-producing.

Deliverables:

A canonical proposition (e.g. Is<T>) representing successful inhabitation.

Elicitation APIs that establish Is<T> on success.

Clear mapping from action traits to contract establishment/refinement.

Success criteria:

Downstream code can assume elicited invariants without revalidation.

Enum/variant elicitation naturally refines state.

Phase 3: Contract-Aware Tools

Goal: Treat MCP tools as contract-preserving transitions.

Deliverables:

A standard Tool trait with associated Pre and Post propositions.

Optional derive support for common tool patterns.

Clear guidance for surfacing effects as contracts.

Success criteria:

Tools cannot be called unless preconditions are satisfied.

Tool chains encode valid execution paths by construction.

Phase 4: Agent Programs as Proof-Carrying Code

Goal: Make multi-step agent logic expressible and verifiable.

Deliverables:

Patterns or helpers for threading Established<P> through async flows.

Example agent programs demonstrating:

monotonic refinement

multi-step guarantees

failure handling

Initial Kani proofs showing unreachable illegal states.

Success criteria:

A nontrivial agent plan can be model-checked end to end.

Violating a contract becomes a compile-time or verification-time failure.

Phase 5: Ergonomics and Expression (Carefully Scoped)

Goal: Improve usability without expanding the logical core.

Possible additions:

Contract aliases and re-exports for readability.

Debug/trace tooling that explains which guarantee failed to establish.

Documentation framing contracts as “guarantees over time”.

Explicit non-goals (for now):

Full dependent typing.

Arbitrary logical predicates.

Automatic proof generation beyond model checking.

5. Guiding Principles

Minimal logic, maximal leverage

Monotonic knowledge only

Make illegal states unrepresentable

If it matters, surface it as a contract

Soundness beats convenience

6. One-Sentence Summary (for the README)

Elicitation lets you build LLM-driven programs that are constrained by Rust’s type system and formally verified contracts, ensuring that conversational agents can only take semantically valid actions.

If you want, next we can:

turn this into a concrete VISION.md

map your existing action traits onto the contract core explicitly

or sketch the very first Kani-verified example that proves the thesis end to end

At this point, the idea is not just coherent — it’s architecturally tight.
