# Shadow Crate Motivation

> **For agents reading this codebase:** this document explains *why* the
> `elicit_*` crates exist, what problem they solve, and how they fit into the
> broader goal of compositional formal verification.

---

## The problem: methods are hard to verify

Formal verification tools (Creusot, Kani, Verus) are powerful, but they face a
practical barrier: the unit of proof is the *method*, and methods are complex.
A non-trivial method may span dozens of lines, call other methods, mutate state
across branches, and carry implicit invariants that are hard to express as
pre/postconditions.  Writing and maintaining those proofs by hand is expensive.

The standard response is to verify small utility functions and trust the rest.
This leaves the most important code — complex business logic — unverified.

---

## The inversion: generate methods from verified building blocks

This project inverts the relationship between methods and verification.

Instead of writing a method and then verifying it, we:

1. **Define a vocabulary of atomic, verified operations** — the building blocks
   of a crate (type constructors, methods, trait impls), each with a simple,
   machine-checkable contract
2. **Let an agent compose those operations** into a tool chain
3. **The tool chain *is* the method** — its correctness follows structurally
   from the correctness of each link plus the type-safety of the connections

The agent is not generating arbitrary Rust code.  It is doing *proof search*:
finding a sequence of verified steps that transforms the input types into the
desired output types while satisfying all intermediate contracts.

```
Atomic operations (verified individually)
    ↓ composed by agent under type-theoretic constraints
Tool chain (correct by construction)
    ↓ emitted as Rust source
Formally verified method ∎
```

---

## What shadow crates are

A **shadow crate** (`elicit_*`) is a crate-shaped vocabulary for a third-party
library.  It exposes three things:

| Layer | What it provides | Example |
|---|---|---|
| **Types** | `serde` + `JsonSchema` wrappers so values can cross the MCP boundary | `elicit_clap::Command` wraps `clap::Command` |
| **Methods** | `#[reflect_methods]` exposes instance methods as MCP tools | `Arg::get_long()`, `Command::get_name()` |
| **Traits** | `#[reflect_trait]` factories expose derive-trait methods per registered type | `clap::ValueEnum::value_variants()`, `serde::Serialize` |

Together these form a **complete vocabulary** for the library.  An agent that
has access to all three layers can reason about and compose the library's
behaviour without writing a single line of Rust.

### Why newtypes instead of upstream impls

Upstream crates intentionally omit `serde`/`schemars` impls (they live at the
CLI or data boundary, not the serialization boundary).  The orphan rule prevents
adding those impls in a downstream crate.  Newtypes sidestep both constraints:
the wrapper is ours, so we can impl any trait on it, and `From`/`Into` provide
the lossless bridge back to the original type.

---

## How agents use the vocabulary

An agent given a goal expressed as types reasons as follows:

1. **What type do I need to produce?** (the postcondition)
2. **What types do I have?** (the current state)
3. **Which tool call advances me from current state toward the goal?** (proof step)
4. **Does the resulting type satisfy the next tool's precondition?** (type check)
5. Repeat until the goal type is in hand.

This mirrors how a human programmer writes a method body — tracing cause and
effect through a sequence of function calls — but every step is an explicit,
auditable tool invocation with a known contract.

The agent is not writing a method; it is *discovering* one through typed
composition.  The method body emerges from the tool chain.

---

## Formal verification enters at two levels

**Per-tool contracts** — each atomic operation carries a Creusot/Kani/Verus
proof of its own behaviour (see `FORMAL_VERIFICATION_LEGOS.md` and
`THIRD_PARTY_SUPPORT_GUIDE.md`).

**Chain-level verification** — the type system enforces that the output type of
each step matches the input type of the next.  An ill-typed composition is
rejected at registration time, not at runtime.  For stronger guarantees,
Creusot logic annotations on the wrapper types carry invariants across the
chain.

The result is a proof structure that scales: verifying 100 atomic operations
once is far cheaper than verifying every combination of those operations in
every method that could be written from them.

---

## The goal: methods as verified tool chains

The long-term goal is that complex behaviour in this ecosystem is *not* hand-
written Rust.  Instead:

- A human defines the types (the vocabulary of the domain)
- An agent composes tool chains that operate on those types
- Formal verification guarantees the chains are correct
- The chains are emitted as Rust source and compiled

This is how we get to **verified methods** without paying the cost of
per-method proof maintenance.  The proof work is front-loaded into the atomic
building blocks; the composition is safe by construction.

Shadow crates are the dictionary.  Tool chains are the sentences.  The type
system is the grammar that makes every grammatical sentence meaningful.

---

## Reading order for a new agent

| Document | What it covers |
|---|---|
| `README.md` | Crate overview and quick start |
| `THIRD_PARTY_SUPPORT_GUIDE.md` | How to add a new shadow crate |
| `FORMAL_VERIFICATION_LEGOS.md` | Compositional proof strategy |
| `ELICITATION_WORKFLOW_ARCHITECTURE.md` | Workflow infrastructure deep dive |
| `CREUSOT_GUIDE.md` | Creusot-specific annotation patterns |
| `crates/elicit_clap/` | Canonical reference implementation |
