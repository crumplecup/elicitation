# Fusion Semantics Implementation Plan

## Proof-Driven MCP Boundary Elision via Trait-Based Rewrites

---

## 0. Objective

Enable **sound elimination of MCP boundaries** by fusing tool chains:

```text
Tool₁: A → JSON
Tool₂: JSON → B
```

into:

```text
f: A → B
```

**Condition:** A machine-checked proof (Kani / Creusot / Verus) establishes:

```text
∀a: A, Tool₂(Tool₁(a)) ≡ f(a)
```

This plan defines:

- A **trait-based proof surface**
- A **fusion registry**
- A **typed IR rewrite pass**
- **Effect + boundary safety gating**

---

## 1. Core Design Principles

### 1.1 Proofs are Admission Tickets

No proof → no fusion.

### 1.2 Traits as Proof Carriers

Traits expose fusion opportunities in a **harvestable, macro-friendly form**.

### 1.3 IR is the Source of Truth

All fusion occurs as a **verified rewrite pass over workflow IR**.

### 1.4 Boundaries are First-Class

MCP is modeled explicitly and can only be removed if:

- Semantics preserved
- Effects preserved
- Policy allows

---

## 2. Trait System Design

### 2.1 Base Tool Trait

```rust
trait Tool<I, O> {
    fn call(input: I) -> O;

    // Marker metadata (used by optimizer)
    const PURE: bool;
    const DETERMINISTIC: bool;
}
```

---

### 2.2 Fusion Trait (Core Abstraction)

```rust
trait Fusable<Next, A, B> {
    type Fused: Tool<A, B>;

    // Proof obligation (implemented via Verus / Creusot / Kani)
    proof fn fusion_equivalence(a: A);

    // Optional: classify proof strength
    const PROOF_STRENGTH: ProofStrength;
}
```

Where:

```rust
enum ProofStrength {
    Bounded,   // Kani
    Symbolic,  // Creusot
    Full,      // Verus
}
```

---

### 2.3 Example Implementation

```rust
struct Tool1; // A → JSON
struct Tool2; // JSON → B
struct FusedAB; // A → B

impl Tool<A, JSON> for Tool1 { ... }
impl Tool<JSON, B> for Tool2 { ... }

impl Tool<A, B> for FusedAB { ... }

impl Fusable<Tool2, A, B> for Tool1 {
    type Fused = FusedAB;

    proof fn fusion_equivalence(a: A) {
        // Verus / Creusot proof:
        // Tool2(Tool1(a)) == FusedAB(a)
    }

    const PROOF_STRENGTH: ProofStrength = ProofStrength::Full;
}
```

---

## 3. Macro Harvesting Layer

### 3.1 Fusion Registration Macro

```rust
#[fuse(Tool1 => Tool2 as FusedAB)]
proof fn tool1_tool2_fusion(a: A) {
    // proof body
}
```

### 3.2 Macro Responsibilities

- Generate `Fusable` impl
- Register fusion rule in global registry
- Attach:
  - input/output types
  - proof strength
  - tool identities

---

## 4. Fusion Registry

### 4.1 Structure

```rust
struct FusionRule {
    tool1: TypeId,
    tool2: TypeId,
    fused: TypeId,
    proof_strength: ProofStrength,
}
```

```rust
struct FusionRegistry {
    rules: HashMap<(TypeId, TypeId), FusionRule>,
}
```

---

### 4.2 Lookup API

```rust
fn lookup_fusion(t1: TypeId, t2: TypeId) -> Option<&FusionRule>;
```

---

## 5. Workflow IR Design

### 5.1 Node Representation

```rust
struct Node {
    id: NodeId,
    input_type: TypeId,
    output_type: TypeId,
    tool: TypeId,
    boundary: Boundary,
    effects: Effects,
}
```

---

### 5.2 Boundary Model

```rust
enum Boundary {
    MCP,
    InProcess,
    Trusted,
    Audited,
}
```

---

### 5.3 Effect Model

```rust
struct Effects {
    pure: bool,
    deterministic: bool,
    has_io: bool,
}
```

---

## 6. Fusion Pass (IR Optimization)

### 6.1 Pass Overview

```text
Input: Workflow Graph
Output: Optimized Workflow Graph
```

---

### 6.2 Algorithm

```rust
for each (node1, node2) in graph.adjacent_pairs():
    if let Some(rule) = registry.lookup(node1.tool, node2.tool) {

        // Step 1: Type compatibility
        assert(node1.output_type == node2.input_type);

        // Step 2: Effect safety
        if !is_effect_safe(node1, node2) {
            continue;
        }

        // Step 3: Boundary safety
        if !is_boundary_elidable(node1.boundary, node2.boundary) {
            continue;
        }

        // Step 4: Proof strength policy
        if !allowed(rule.proof_strength) {
            continue;
        }

        // Step 5: Apply rewrite
        let fused_node = Node {
            input_type: node1.input_type,
            output_type: node2.output_type,
            tool: rule.fused,
            boundary: Boundary::InProcess,
            effects: merge_effects(node1, node2),
        };

        graph.replace_pair(node1, node2, fused_node);
    }
}
```

---

## 7. Safety Gates

### 7.1 Effect Safety

```rust
fn is_effect_safe(n1: &Node, n2: &Node) -> bool {
    n1.effects.pure &&
    n2.effects.pure &&
    n1.effects.deterministic &&
    n2.effects.deterministic
}
```

---

### 7.2 Boundary Elision Policy

```rust
fn is_boundary_elidable(b1: Boundary, b2: Boundary) -> bool {
    match (b1, b2) {
        (Boundary::MCP, Boundary::MCP) => true,
        (Boundary::Audited, _) => false,
        (_, Boundary::Audited) => false,
        _ => true,
    }
}
```

---

### 7.3 Proof Policy

```rust
fn allowed(p: ProofStrength) -> bool {
    match p {
        ProofStrength::Full => true,
        ProofStrength::Symbolic => true,
        ProofStrength::Bounded => false, // configurable
    }
}
```

---

## 8. Provenance & Auditability

Each fused node should carry metadata:

```rust
struct FusionProvenance {
    original_tools: (TypeId, TypeId),
    proof_id: &'static str,
    proof_strength: ProofStrength,
}
```

Attach to IR for:

- debugging
- audit logs
- optional “de-optimization” (expansion back to MCP)

---

## 9. Advanced Extensions

### 9.1 Multi-Step Fusion

Allow chaining:

```text
(A → JSON → B → JSON → C) ⇒ (A → C)
```

Implementation:

- iterate fusion pass to fixed point
- or perform graph pattern matching of length N

---

### 9.2 Alternative Tool Selection

If:

```text
ToolX ≡ ToolY
```

Use proof to:

- swap MCP tool for in-process equivalent
- choose lower-latency implementation

---

### 9.3 Partial Evaluation

If inputs are partially known:

```text
f(a, const) ⇒ specialized_f(a)
```

Can be proven and fused similarly.

---

## 10. Testing Strategy

### 10.1 Proof Validation

- Verus: full spec verification
- Creusot: logical equivalence
- Kani: bounded sanity checks

### 10.2 Differential Testing

- Execute:
  - original chain
  - fused function
- Assert equality over sampled domain

### 10.3 IR Roundtrip

- Fuse → defuse → compare graphs

---

## 11. Incremental Adoption Plan

### Phase 1

- Manual `Fusable` impls
- Simple pairwise fusion

### Phase 2

- Macro harvesting
- Registry automation

### Phase 3

- Multi-step fusion
- Proof strength policies

### Phase 4

- Full compiler pipeline
- Cost-based optimization (latency-aware)

---

## 12. End State

You achieve:

- **Zero-cost abstraction over MCP**
- **Proof-carrying workflow optimization**
- **Compiler-grade rewrite system**
- **Deterministic, auditable boundary elimination**

At this point, “tool composition” is indistinguishable from “function composition,”
and MCP becomes a deploy-time concern, not a semantic one.

---
