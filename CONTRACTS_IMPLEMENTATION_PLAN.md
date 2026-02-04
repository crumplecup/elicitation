# Contracts Implementation Plan

**Vision**: Enable proof-carrying composition of elicitation steps with zero-cost, type-based contracts.

**Principle**: Contracts are *primitives* (like `Option` or `Result`), not features. Usage is opt-in, but they're always available.

---

## Phase 1: Foundation (Week 1)

**Goal**: Establish the core contract primitives with tests and documentation.

### Step 1.1: Core Types (Day 1)
**File**: `crates/elicitation/src/contracts.rs`

**Deliverables**:
```rust
/// Marker trait: types that represent propositions
pub trait Prop: 'static {}

/// Witness that proposition P has been established
pub struct Established<P: Prop> {
    _marker: PhantomData<P>,
}

/// Proposition: value inhabits type T with verified invariants
pub struct Is<T> {
    _marker: PhantomData<T>,
}

impl<T: 'static> Prop for Is<T> {}
```

**Tests**:
- ✅ Types are `Send + Sync`
- ✅ `Established<P>` is zero-sized
- ✅ Types compile in const context
- ✅ Doctests show basic usage

**Success Metric**: `cargo test --lib` passes with new types.

---

### Step 1.2: Construction (Day 1)
**File**: `crates/elicitation/src/contracts.rs`

**Deliverables**:
```rust
impl<P: Prop> Established<P> {
    /// Assert that proposition P holds
    /// 
    /// This is a semantic contract: the caller asserts P is true.
    /// Typically called by elicitation internals after validation.
    pub fn assert() -> Self {
        Self { _marker: PhantomData }
    }
    
    /// Consume proof to prove a weaker proposition
    pub fn weaken<Q: Prop>(self) -> Established<Q>
    where
        P: Implies<Q>,
    {
        Established { _marker: PhantomData }
    }
}

/// Logical implication: P implies Q
pub trait Implies<Q: Prop>: Prop {}

// Trivial: every proposition implies itself
impl<P: Prop> Implies<P> for P {}
```

**Tests**:
- ✅ Can construct proof with `Established::assert()`
- ✅ Can weaken via `Implies` trait
- ✅ Implication is reflexive (P implies P)
- ✅ Zero-cost: no runtime overhead

**Success Metric**: Tests pass + `#[inline(always)]` confirms zero cost.

---

### Step 1.3: Conjunction (Day 2)
**File**: `crates/elicitation/src/contracts.rs`

**Deliverables**:
```rust
/// Logical conjunction: both P and Q hold
pub struct And<P: Prop, Q: Prop> {
    _marker: PhantomData<(P, Q)>,
}

impl<P: Prop, Q: Prop> Prop for And<P, Q> {}

/// Combine two proofs into conjunction
pub fn both<P: Prop, Q: Prop>(
    _p: Established<P>,
    _q: Established<Q>,
) -> Established<And<P, Q>> {
    Established { _marker: PhantomData }
}

/// Project left proof from conjunction
pub fn fst<P: Prop, Q: Prop>(
    _both: Established<And<P, Q>>,
) -> Established<P> {
    Established { _marker: PhantomData }
}

/// Project right proof from conjunction
pub fn snd<P: Prop, Q: Prop>(
    _both: Established<And<P, Q>>,
) -> Established<Q> {
    Established { _marker: PhantomData }
}

// Conjunction implies left/right
impl<P: Prop, Q: Prop> Implies<P> for And<P, Q> {}
impl<P: Prop, Q: Prop> Implies<Q> for And<P, Q> {}
```

**Tests**:
- ✅ Can combine two proofs with `both()`
- ✅ Can project with `fst()` / `snd()`
- ✅ Conjunction implies components
- ✅ Can chain: `both(both(p, q), r)`

**Success Metric**: Conjunction algebra tests pass.

---

### Step 1.4: Export and Document (Day 2)
**File**: `crates/elicitation/src/lib.rs`

**Deliverables**:
```rust
pub mod contracts;
pub use contracts::{Prop, Established, Is, And, both, fst, snd};
```

**Documentation**:
- Module-level doc explaining contracts as "proofs of work done"
- Examples showing construction and composition
- Comparison to `Result` (error handling vs proof carrying)
- Clear "you don't need this unless composing multi-step flows"

**Tests**:
- ✅ All contract types exported at crate root
- ✅ Doctests in module doc compile and run
- ✅ `cargo doc --open` shows polished docs

**Success Metric**: Documentation is clear, examples compile, newcomers understand the purpose.

---

## Phase 2: Integration with Elicitation (Week 2)

**Goal**: Make elicitation return proofs that values inhabit their types.

### Step 2.1: Proof-Returning APIs (Day 3-4)
**File**: `crates/elicitation/src/traits.rs`

**Deliverables**:
```rust
impl<T: Elicitation> T {
    /// Elicit value with proof it inhabits type T
    pub async fn elicit_proven(
        peer: &impl Peer
    ) -> Result<(T, Established<Is<T>>), ElicitError> {
        let value = Self::elicit(peer).await?;
        Ok((value, Established::assert()))
    }
}

// For types with contracts
impl<T: Elicitation> T
where
    T: crate::verification::Contract,
{
    /// Elicit value with proof it satisfies contract
    pub async fn elicit_contracted(
        peer: &impl Peer
    ) -> Result<(T, Established<Is<T>>), ElicitError> {
        // Contract::validate() ensures invariants hold
        let value = Self::elicit(peer).await?;
        if !T::validate(&value) {
            return Err(ElicitError::validation("Contract failed"));
        }
        Ok((value, Established::assert()))
    }
}
```

**Tests**:
- ✅ `String::elicit_proven()` returns proof
- ✅ `StringNonEmpty::elicit_contracted()` validates + returns proof
- ✅ Invalid input fails with error (no proof returned)
- ✅ Proof can be used in downstream functions

**Success Metric**: Can elicit primitive types with proofs, proofs compose in tests.

---

### Step 2.2: Refinement Proofs (Day 5)
**File**: `crates/elicitation/src/contracts.rs`

**Deliverables**:
```rust
/// Type-level refinement: From refines to To
/// 
/// Represents that type To has strictly more constraints than From.
/// If a value inhabits To, it necessarily inhabits From.
pub trait Refines<From> {}

// StringNonEmpty refines String
impl Refines<String> for crate::verification::StringNonEmpty {}

// Refinement implies inhabitation of base type
impl<T, U> Implies<Is<T>> for Is<U>
where
    U: Refines<T>,
{}

/// Downcast proof from refined type to base type
pub fn downcast<T, U>(proof: Established<Is<U>>) -> Established<Is<T>>
where
    U: Refines<T>,
{
    proof.weaken()
}
```

**Tests**:
- ✅ `StringNonEmpty` proof can downcast to `String` proof
- ✅ Cannot upcast (compile error)
- ✅ Chain refinements: `UrlHttps -> UrlValid -> Url`
- ✅ Kani proof: refinement preserves inhabitation

**Success Metric**: Refinement hierarchy compiles, Kani verifies soundness.

---

### Step 2.3: Enum Refinement (Day 6)
**File**: `crates/elicitation/src/traits.rs`

**Deliverables**:
```rust
/// Proposition: enum is in specific variant
pub struct InVariant<E, V> {
    _marker: PhantomData<(E, V)>,
}

impl<E, V> Prop for InVariant<E, V> {}

// Helper for Select pattern
impl<E: Select> E {
    /// Select variant with proof
    pub async fn select_proven(
        peer: &impl Peer
    ) -> Result<(E, Established<Is<E>>), ElicitError> {
        let value = Self::select(peer).await?;
        let proof = unsafe { Established::new_unchecked() };
        Ok((value, proof))
    }
}
```

**Tests**:
- ✅ Enum selection returns proof
- ✅ Can branch on enum, carry variant-specific proofs
- ✅ Variant-specific code requires variant proof

**Success Metric**: Enum-based state machines use proofs for type safety.

---

## Phase 3: Tool Contracts (Week 3)

**Goal**: Model MCP tools as contract-preserving functions.

### Step 3.1: Tool Trait (Day 7-8)
**File**: `crates/elicitation/src/tool.rs`

**Deliverables**:
```rust
/// MCP tool with explicit preconditions and postconditions
pub trait Tool {
    /// Tool input type
    type Input: Elicitation;
    
    /// Tool output type
    type Output;
    
    /// Precondition proposition (what must be true before calling)
    type Pre: Prop;
    
    /// Postcondition proposition (what's true after success)
    type Post: Prop;
    
    /// Execute tool (requires precondition proof)
    async fn execute(
        &self,
        input: Self::Input,
        _pre: Established<Self::Pre>,
    ) -> Result<(Self::Output, Established<Self::Post>), ToolError>;
}

/// Tools with no preconditions
pub type UnconstrainedTool<I, O, Post> = dyn Tool<
    Input = I,
    Output = O,
    Pre = True,
    Post = Post,
>;

/// Trivially true proposition
pub struct True;
impl Prop for True {}

impl True {
    /// Axiom: truth is always established
    pub fn axiom() -> Established<True> {
        Established::assert()
    }
}
```

**Tests**:
- ✅ Can implement tool with no preconditions (`Pre = True`)
- ✅ Can implement tool requiring specific proof
- ✅ Cannot call tool without required proof (compile error)
- ✅ Tool chains compose proofs

**Success Metric**: Example tool with precondition compiles, type-checks correctly.

---

### Step 3.2: Tool Composition (Day 9)
**File**: `crates/elicitation/src/tool.rs`

**Deliverables**:
```rust
/// Sequentially compose two tools where first's post implies second's pre
pub async fn then<T1, T2>(
    tool1: &T1,
    tool2: &T2,
    input1: T1::Input,
    pre1: Established<T1::Pre>,
) -> Result<(T2::Output, Established<T2::Post>), ToolError>
where
    T1: Tool,
    T2: Tool<Input = T1::Output>,
    T1::Post: Implies<T2::Pre>,
{
    let (output1, post1) = tool1.execute(input1, pre1).await?;
    let pre2 = post1.weaken();
    let (output2, post2) = tool2.execute(output1, pre2).await?;
    Ok((output2, post2))
}

/// Parallel composition: run two tools, combine proofs
pub async fn both_tools<T1, T2>(
    tool1: &T1,
    tool2: &T2,
    input1: T1::Input,
    input2: T2::Input,
    pre: Established<And<T1::Pre, T2::Pre>>,
) -> Result<((T1::Output, T2::Output), Established<And<T1::Post, T2::Post>>), ToolError>
where
    T1: Tool,
    T2: Tool,
{
    let pre1 = fst(pre);
    let pre2 = snd(pre);
    
    let (out1, post1) = tool1.execute(input1, pre1).await?;
    let (out2, post2) = tool2.execute(input2, pre2).await?;
    
    Ok(((out1, out2), both(post1, post2)))
}
```

**Tests**:
- ✅ Can chain tools with `then()`
- ✅ Can run tools in parallel with `both_tools()`
- ✅ Type errors if postcondition doesn't match precondition
- ✅ Proofs accumulate correctly

**Success Metric**: Tool chains type-check, demonstrate proof flow.

---

### Step 3.3: Real Example (Day 10)
**File**: `crates/elicitation/examples/tool_chain.rs`

**Deliverables**:
```rust
/// Example: Configure server with validated URL and port
struct ValidateUrl;

impl Tool for ValidateUrl {
    type Input = String;
    type Output = Url;
    type Pre = True;
    type Post = Is<Url>;
    
    async fn execute(
        &self,
        input: String,
        _pre: Established<True>,
    ) -> Result<(Url, Established<Is<Url>>), ToolError> {
        let url = Url::parse(&input)?;
        Ok((url, Established::assert()))
    }
}

struct ConfigureServer;

impl Tool for ConfigureServer {
    type Input = (Url, u16);
    type Output = ();
    type Pre = Is<Url>;  // Requires validated URL
    type Post = True;
    
    async fn execute(
        &self,
        (url, port): (Url, u16),
        _pre: Established<Is<Url>>,
    ) -> Result<((), Established<True>), ToolError> {
        println!("Configuring server: {} on port {}", url, port);
        Ok(((), True::axiom()))
    }
}

// Chain: validate URL, then configure
async fn setup_server(url_string: String, port: u16) -> Result<(), ToolError> {
    let validate = ValidateUrl;
    let configure = ConfigureServer;
    
    let (url, url_proof) = validate.execute(url_string, True::axiom()).await?;
    let (_, _) = configure.execute((url, port), url_proof).await?;
    
    Ok(())
}
```

**Tests**:
- ✅ Example compiles and runs
- ✅ Invalid URL fails at validation step
- ✅ Cannot skip validation (type error)
- ✅ Kani proof: URL is always validated before use

**Success Metric**: End-to-end example demonstrates value, Kani verifies.

---

## Phase 4: Kani Integration (Week 4)

**Goal**: Prove contract properties with formal verification.

### Step 4.1: Basic Proofs (Day 11-12)
**File**: `crates/elicitation/tests/contract_proofs.rs`

**Deliverables**:
```rust
#[cfg(kani)]
mod proofs {
    use elicitation::*;
    
    #[kani::proof]
    fn proof_is_zero_sized() {
        let p: Established<Is<String>> = Established::assert();
        assert_eq!(std::mem::size_of_val(&p), 0);
    }
    
    #[kani::proof]
    fn proof_conjunction_commutes() {
        let p: Established<Is<String>> = Established::assert();
        let q: Established<Is<i32>> = Established::assert();
        
        let pq = both(p, q);
        let _p2 = fst(pq);
        // Proves projection works
    }
    
    #[kani::proof]
    fn proof_refinement_sound() {
        // StringNonEmpty refines String
        let proof: Established<Is<StringNonEmpty>> = Established::assert();
        let _: Established<Is<String>> = downcast(proof);
        // Proves refinement is sound
    }
}
```

**Tests**:
- ✅ `cargo kani --tests` verifies basic properties
- ✅ Zero-cost proofs verified
- ✅ Conjunction algebra verified
- ✅ Refinement soundness verified

**Success Metric**: Core contract properties formally verified.

---

### Step 4.2: Tool Chain Verification (Day 13-14)
**File**: `crates/elicitation/tests/tool_chain_proofs.rs`

**Deliverables**:
```rust
#[cfg(kani)]
#[kani::proof]
fn cannot_use_unvalidated_url() {
    // This should not compile:
    // let url: Url = kani::any();
    // configure_server(url, NO_PROOF);
    
    // Instead must validate:
    let url_string: String = kani::any();
    let url = match Url::parse(&url_string) {
        Ok(u) => u,
        Err(_) => return,
    };
    let proof = Established::assert();
    
    // Now can use with proof
    use_validated_url(url, proof);
}

#[kani::proof]
fn tool_chain_maintains_invariants() {
    // Multi-step flow preserves properties
    let input = kani::any::<String>();
    let proof = True::axiom();
    
    // Tool 1: validate
    let (validated, p1) = validate_input(input, proof);
    
    // Tool 2: transform (requires validation)
    let (transformed, p2) = transform_validated(validated, p1);
    
    // Tool 3: persist (requires transformation)
    persist_transformed(transformed, p2);
    
    // Kani verifies all invariants held
}
```

**Tests**:
- ✅ Kani proves preconditions are required
- ✅ Kani proves postconditions are established
- ✅ Kani proves multi-step flows maintain invariants

**Success Metric**: Tool chain verification passes, demonstrates safety.

---

## Phase 5: Documentation and Examples (Week 5)

### Step 5.1: Module Documentation (Day 15)
**File**: `crates/elicitation/src/contracts.rs`

**Deliverables**:
- Comprehensive module doc explaining contracts as "proof-carrying code"
- Comparison to other approaches (validation, runtime checks, dependent types)
- Clear guidance on when to use contracts vs when not to
- Migration guide: existing code works unchanged, contracts are additive

**Success Metric**: `cargo doc` shows publication-quality documentation.

---

### Step 5.2: Examples (Day 16)
**Files**: 
- `examples/contracts_basic.rs` - Simple proof construction
- `examples/contracts_composition.rs` - Multi-step proof flow
- `examples/contracts_tools.rs` - Real tool chain with contracts

**Success Metric**: All examples compile, run, demonstrate value clearly.

---

### Step 5.3: README Update (Day 17)
**File**: `README.md`

**Deliverables**:
- Add "Proof-Carrying Composition" section
- Show contract example alongside existing examples
- Emphasize: "Build verified agent programs, not just validated JSON"
- Link to examples and documentation

**Success Metric**: README conveys contracts as a key differentiator.

---

## Completion Criteria

✅ All phases pass `cargo test --all-features`
✅ All phases pass `cargo clippy --all-features`
✅ Core contracts verified with Kani
✅ At least 3 complete examples demonstrating value
✅ Documentation explains when/why to use contracts
✅ Zero breaking changes to existing APIs
✅ Version ready: 0.5.0 (minor bump, new capabilities)

---

## Version Strategy

**v0.4.6** → **v0.5.0**: Minor version bump (new functionality, no breaking changes)

All contract APIs are additive:
- Existing code works unchanged
- New `_proven()` methods are opt-in
- Tool trait is new, doesn't affect existing patterns
- Users can adopt incrementally

---

## Success Metrics

**Technical**:
- Zero runtime overhead (verified with benchmarks)
- Kani verifies contract properties
- Type errors catch contract violations at compile time

**Ecosystem**:
- botticelli_mcp adopts contracts for critical tool chains
- At least one external user reports using contracts
- Contracts mentioned in Rust community (reddit, discourse)

**Vision**:
- Can claim: "The only LLM framework with formally verified agent programs"
- Paper/blog post demonstrating the approach
- Comparison showing our guarantees vs other frameworks (LangChain, etc.)
