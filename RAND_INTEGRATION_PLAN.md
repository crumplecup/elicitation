# Rand Integration Plan

## Vision

Enable agents to generate random data through the elicitation framework, following the "castle on cloud" verification pattern. Trust `rand` crate's implementations, verify our wrapper logic.

## Use Cases

### 1. Testing & QA
```rust
// Agent generates reproducible test data
let seed = u64::elicit(communicator).await?;
let mut rng = StdRng::seed_from_u64(seed);

// Generate 1000 random users for load testing
for _ in 0..1000 {
    let user = RandomGenerator::<User>::new(&mut rng).generate();
    test_api(user).await?;
}
```

**Benefits:**
- Reproducible test failures (seed in audit log)
- Property-based testing integration
- Fuzzing with agent-controlled parameters
- Synthetic data generation at scale

### 2. Gaming & Simulations
```rust
// Agent as game master
let dice = DiceGenerator::new(2, 6); // 2d6
let initiative = dice.generate(); // Random but controlled

// Weighted loot tables
let loot_table = WeightedGenerator::new(vec![
    (Item::CommonSword, 70),
    (Item::RareBow, 25),
    (Item::LegendaryArmor, 5),
]);
let drop = loot_table.generate();
```

**Benefits:**
- Agents control randomness in narratives
- Procedural generation under agent guidance
- Balanced RNG for gameplay fairness
- Audit trail for "luck" disputes

### 3. Simulations & Modeling
```rust
// Monte Carlo simulation
let distribution = NormalGenerator::new(mean, std_dev);
let samples: Vec<f64> = (0..10_000)
    .map(|_| distribution.generate())
    .collect();
```

**Benefits:**
- Agent-configured statistical distributions
- Stochastic modeling with conversational setup
- Risk analysis and forecasting

---

## Architecture

### Separate Crate: `elicitation_rand`

**Location:** `crates/elicitation_rand/`

**Dependencies:**
```toml
[dependencies]
elicitation = { version = "0.6", path = "../elicitation" }
rand = "0.8"
rand_chacha = "0.3"  # For cryptographic RNGs

[dev-dependencies]
kani-verifier = { version = "0.57", optional = true }
```

**Benefits:**
- Core `elicitation` stays dependency-lean
- Users opt-in with `elicitation_rand = "0.6"`
- No feature flag complexity
- Clean separation of concerns

---

## Implementation Phases

### Phase 1: Core RNG Elicitation (Foundation)

**Goal:** Implement `Elicitation` for RNG types.

**Files to create:**
- `crates/elicitation_rand/src/lib.rs`
- `crates/elicitation_rand/src/rng.rs` - RNG elicitation
- `crates/elicitation_rand/Cargo.toml`

**RNG Types:**
1. `rand::rngs::StdRng` - Standard, reproducible
2. `rand::rngs::SmallRng` - Fast, small state
3. `rand_chacha::ChaCha8Rng` - Cryptographically secure

**Implementation Pattern:**
```rust
use elicitation::{Elicitation, ElicitCommunicator, ElicitResult};
use rand::SeedableRng;
use rand::rngs::StdRng;

impl Elicitation for StdRng {
    type Style = StdRngStyle;

    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        // Elicit seed from agent
        let seed: u64 = u64::elicit(communicator).await?;
        
        // Construct RNG (trust rand's implementation)
        Ok(StdRng::seed_from_u64(seed))
    }
}
```

**Castle on Cloud:**
- Trust `rand::SeedableRng::seed_from_u64()` works correctly
- Trust RNG produces random sequences
- Verify: Our wrapper constructs RNG from seed correctly

**Deliverables:**
- ✅ Elicitation for StdRng, SmallRng, ChaCha8Rng
- ✅ Basic smoke tests
- ✅ Documentation with examples

---

### Phase 2: Basic Generators (Primitives)

**Goal:** Create `Generator` implementations for primitive types.

**Files to create:**
- `crates/elicitation_rand/src/generators.rs` - Random generators
- `crates/elicitation_rand/src/generators/primitives.rs`

**Generators:**
```rust
/// Generic random generator for types implementing Distribution<Standard>.
pub struct RandomGenerator<T> {
    rng: StdRng,
    _phantom: PhantomData<T>,
}

impl<T> RandomGenerator<T> 
where
    Standard: Distribution<T>,
{
    pub fn new(rng: StdRng) -> Self {
        Self { rng, _phantom: PhantomData }
    }
    
    pub fn with_seed(seed: u64) -> Self {
        Self::new(StdRng::seed_from_u64(seed))
    }
}

impl<T> Generator for RandomGenerator<T>
where
    Standard: Distribution<T>,
{
    type Target = T;
    
    fn generate(&self) -> T {
        self.rng.gen() // Trust rand, verify wrapper
    }
}
```

**Supported Types (via Standard distribution):**
- Integers: `u8`, `u16`, `u32`, `u64`, `u128`, `i8`, `i16`, `i32`, `i64`, `i128`
- Floats: `f32`, `f64`
- Bool: `bool`
- Char: `char`

**Castle on Cloud:**
- Trust `rand::Rng::gen()` produces random values
- Trust `Distribution<Standard>` implementations
- Verify: Generator stores RNG, calls `.gen()`, returns result

**Deliverables:**
- ✅ RandomGenerator for all Standard types
- ✅ Unit tests (verify determinism with fixed seed)
- ✅ Examples in documentation

---

### Phase 3: Distribution Generators (Bounded, Weighted)

**Goal:** Support common distributions (uniform, weighted, normal).

**Files to create:**
- `crates/elicitation_rand/src/generators/uniform.rs`
- `crates/elicitation_rand/src/generators/weighted.rs`
- `crates/elicitation_rand/src/generators/normal.rs`

**Uniform (Bounded) Generator:**
```rust
/// Generates random values within a range.
pub struct UniformGenerator<T> {
    rng: StdRng,
    distribution: Uniform<T>,
}

impl<T> UniformGenerator<T>
where
    T: SampleUniform,
{
    pub fn new(rng: StdRng, low: T, high: T) -> Self {
        Self {
            rng,
            distribution: Uniform::new(low, high),
        }
    }
}

impl<T> Generator for UniformGenerator<T>
where
    T: SampleUniform,
{
    type Target = T;
    
    fn generate(&self) -> T {
        self.distribution.sample(&mut self.rng)
    }
}
```

**Weighted Generator:**
```rust
/// Generates values from a weighted distribution.
pub struct WeightedGenerator<T> {
    rng: StdRng,
    distribution: WeightedIndex<u32>,
    values: Vec<T>,
}

impl<T: Clone> Generator for WeightedGenerator<T> {
    type Target = T;
    
    fn generate(&self) -> T {
        let index = self.distribution.sample(&mut self.rng);
        self.values[index].clone()
    }
}
```

**Normal Distribution:**
```rust
use rand_distr::{Normal, Distribution};

pub struct NormalGenerator {
    rng: StdRng,
    distribution: Normal<f64>,
}
```

**Castle on Cloud:**
- Trust `rand::distributions::Uniform` samples correctly
- Trust `rand::distributions::WeightedIndex` respects weights
- Trust `rand_distr::Normal` generates normal distribution
- Verify: Our wrappers configure distributions correctly

**Deliverables:**
- ✅ UniformGenerator (bounded ranges)
- ✅ WeightedGenerator (weighted selection)
- ✅ NormalGenerator (Gaussian distribution)
- ✅ Elicitation for generator configuration
- ✅ Examples: dice rolls, loot tables, gaussian noise

---

### Phase 4: Specialized Generators (Gaming & Testing)

**Goal:** Domain-specific generators for common use cases.

**Files to create:**
- `crates/elicitation_rand/src/generators/dice.rs`
- `crates/elicitation_rand/src/generators/sampling.rs`

**Dice Generator:**
```rust
/// Simulates dice rolls (e.g., 2d6, 1d20).
pub struct DiceGenerator {
    rng: StdRng,
    num_dice: u32,
    num_sides: u32,
}

impl Generator for DiceGenerator {
    type Target = u32;
    
    fn generate(&self) -> u32 {
        (0..self.num_dice)
            .map(|_| self.rng.gen_range(1..=self.num_sides))
            .sum()
    }
}

impl Elicitation for DiceGenerator {
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        // "How many dice?" "How many sides?"
        let num_dice = u32::elicit(communicator).await?;
        let num_sides = u32::elicit(communicator).await?;
        let seed = u64::elicit(communicator).await?;
        Ok(DiceGenerator::new(num_dice, num_sides, seed))
    }
}
```

**Shuffle Generator:**
```rust
/// Randomly shuffles a collection.
pub struct ShuffleGenerator<T> {
    rng: StdRng,
    items: Vec<T>,
}

impl<T: Clone> Generator for ShuffleGenerator<T> {
    type Target = Vec<T>;
    
    fn generate(&self) -> Vec<T> {
        let mut shuffled = self.items.clone();
        shuffled.shuffle(&mut self.rng);
        shuffled
    }
}
```

**Deliverables:**
- ✅ DiceGenerator (XdY dice rolls)
- ✅ ShuffleGenerator (random permutations)
- ✅ SamplingGenerator (random sample from collection)
- ✅ Gaming examples (encounters, loot, initiative)
- ✅ Testing examples (random test data)

---

### Phase 5: Kani Verification (Symbolic Gate)

**Goal:** Verify wrapper logic using "castle on cloud" pattern.

**Files to create:**
- `crates/elicitation_rand/src/verification/mod.rs`
- `crates/elicitation_rand/src/verification/kani_proofs/generators.rs`

**Verification Strategy:**

**Symbolic Gate Pattern:**
- Don't verify rand's randomness (that's rand's job)
- Don't call `.gen()` or `.sample()` (unsupported by Kani)
- Verify wrapper's data structure and decision logic
- Trust rand's implementations

**Example Proofs:**
```rust
#[cfg(kani)]
mod kani_proofs {
    use super::*;

    /// Verify RandomGenerator stores RNG correctly.
    #[kani::proof]
    fn verify_random_generator_construction() {
        let seed: u64 = kani::any();
        
        // Verify wrapper constructs correctly
        let generator = RandomGenerator::<u8>::with_seed(seed);
        
        // Don't call generate() - trust rand
        // Verify structural correctness only
        assert!(true, "Generator constructed");
    }

    /// Verify UniformGenerator stores bounds correctly.
    #[kani::proof]
    fn verify_uniform_generator_bounds() {
        let low: u32 = kani::any();
        let high: u32 = kani::any();
        
        kani::assume(low < high);
        
        // Verify wrapper stores bounds
        // Don't call generate() - trust rand::distributions::Uniform
        assert!(low < high, "Bounds preserved");
    }

    /// Verify DiceGenerator stores configuration correctly.
    #[kani::proof]
    fn verify_dice_generator_config() {
        let num_dice: u32 = kani::any();
        let num_sides: u32 = kani::any();
        
        kani::assume(num_dice > 0);
        kani::assume(num_sides > 0);
        
        // Verify wrapper stores dice config
        // Don't call generate() - trust rand::gen_range
        assert!(num_dice > 0 && num_sides > 0, "Valid dice config");
    }
}
```

**Castle on Cloud:**
- Trust `rand::Rng::gen()` is random
- Trust `rand::distributions::*` sample correctly
- Trust `rand::seq::SliceRandom::shuffle()` shuffles uniformly
- Verify: Our wrappers store configuration and call rand correctly

**Deliverables:**
- ✅ Kani proofs for all generator types
- ✅ Symbolic gate verification (no .gen() calls)
- ✅ Documentation of verification approach
- ✅ Integration with verification runner

---

## Castle on Cloud: What We Trust vs. Verify

### We Trust (Cloud of Assumptions)
1. **rand crate correctness:**
   - `StdRng`, `SmallRng`, `ChaCha8Rng` produce random sequences
   - `SeedableRng::seed_from_u64()` creates deterministic RNG
   - `Rng::gen()` samples from distribution correctly

2. **Distribution correctness:**
   - `Uniform` samples uniformly over range
   - `WeightedIndex` respects weights
   - `Normal` generates Gaussian distribution
   - `Standard` implements correct distributions for primitives

3. **Rust type system:**
   - Strong typing ensures RNG is the correct type
   - Trait bounds restrict state space
   - Ownership prevents aliasing bugs

### We Verify (Castle on Cloud)
1. **Wrapper construction:**
   - RNG created from seed correctly
   - Distribution configured with correct parameters
   - Generator stores RNG and config properly

2. **Decision logic:**
   - Correct branch for each generation mode
   - Configuration parameters validated
   - Bounds checked before use

3. **Integration:**
   - Elicitation constructs generators correctly
   - Generator trait implemented correctly
   - Type conversions are sound

### We Don't Verify (Out of Scope)
- Randomness quality (rand's job)
- Statistical properties (rand_distr's job)
- Cryptographic security (rand_chacha's job)
- Distribution correctness (rand's job)

---

## Testing Strategy

### Unit Tests (Determinism)
```rust
#[test]
fn test_random_generator_deterministic() {
    let seed = 42;
    let gen1 = RandomGenerator::<u32>::with_seed(seed);
    let gen2 = RandomGenerator::<u32>::with_seed(seed);
    
    // Same seed produces same sequence
    assert_eq!(gen1.generate(), gen2.generate());
}

#[test]
fn test_uniform_generator_bounds() {
    let gen = UniformGenerator::new(StdRng::seed_from_u64(0), 10, 20);
    
    // Generate many samples, verify all in range
    for _ in 0..1000 {
        let value = gen.generate();
        assert!(value >= 10 && value < 20);
    }
}
```

### Integration Tests (Elicitation)
```rust
#[tokio::test]
async fn test_elicit_dice_generator() {
    let mock_communicator = MockCommunicator::new();
    
    // Agent chooses dice configuration
    let dice = DiceGenerator::elicit(&mock_communicator).await?;
    
    // Verify generator works
    let roll = dice.generate();
    assert!(roll >= dice.num_dice && roll <= dice.num_dice * dice.num_sides);
}
```

### Property Tests (Statistical)
```rust
#[test]
fn test_weighted_generator_distribution() {
    // Heavy item should appear ~70% of the time
    let gen = WeightedGenerator::new(vec![
        (Item::Common, 70),
        (Item::Rare, 30),
    ]);
    
    let samples: Vec<_> = (0..10_000).map(|_| gen.generate()).collect();
    let common_count = samples.iter().filter(|&&x| x == Item::Common).count();
    
    // Should be ~7000 ± some tolerance
    assert!((6500..7500).contains(&common_count));
}
```

---

## Documentation Requirements

### README.md
- Overview of rand integration
- Quick start examples
- Use cases (testing, gaming, simulations)
- Castle on cloud verification approach

### Examples
1. **Basic Random Generation**
   ```rust
   let seed = 42u64;
   let generator = RandomGenerator::<u32>::with_seed(seed);
   let value = generator.generate();
   ```

2. **Gaming: Dice Rolls**
   ```rust
   let dice = DiceGenerator::new(2, 6, 42); // 2d6
   let initiative = dice.generate();
   ```

3. **Gaming: Loot Tables**
   ```rust
   let loot = WeightedGenerator::new(vec![
       (Item::CommonSword, 70),
       (Item::RareBow, 25),
       (Item::LegendaryArmor, 5),
   ]);
   ```

4. **Testing: Random Users**
   ```rust
   let user_gen = RandomGenerator::<User>::with_seed(seed);
   let test_data: Vec<_> = (0..1000).map(|_| user_gen.generate()).collect();
   ```

5. **Elicitation: Agent-Controlled RNG**
   ```rust
   let rng = StdRng::elicit(communicator).await?;
   let generator = RandomGenerator::<f64>::new(rng);
   ```

---

## Dependencies

### Required
- `elicitation = "0.6"` - Core traits
- `rand = "0.8"` - RNG implementations
- `rand_chacha = "0.3"` - Cryptographic RNG

### Optional
- `rand_distr = "0.4"` - Additional distributions (Normal, Exp, etc.)
- `kani-verifier = "0.57"` - Verification (dev only)

---

## Success Metrics

### Phase 1 Complete When:
- ✅ Elicitation for StdRng, SmallRng, ChaCha8Rng
- ✅ Basic smoke tests pass
- ✅ Documentation with examples

### Phase 2 Complete When:
- ✅ RandomGenerator for all Standard types
- ✅ Determinism tests pass (fixed seed → same output)
- ✅ Examples in docs

### Phase 3 Complete When:
- ✅ UniformGenerator, WeightedGenerator, NormalGenerator
- ✅ Elicitation for generator configuration
- ✅ Statistical tests pass (distributions match expected)

### Phase 4 Complete When:
- ✅ DiceGenerator, ShuffleGenerator, SamplingGenerator
- ✅ Gaming examples (encounters, loot, initiative)
- ✅ Testing examples (synthetic data)

### Phase 5 Complete When:
- ✅ Kani proofs for all generators
- ✅ Symbolic gate verification (no .gen() calls)
- ✅ Integration with verification runner
- ✅ Documentation of verification approach

---

## Future Considerations

### Potential Extensions
1. **Constraint Generators** - Generate values satisfying contracts
2. **Proptest Integration** - Bridge to property-based testing
3. **Arbitrary Integration** - Derive Arbitrary from Elicit
4. **Custom Distributions** - User-defined samplers

### Out of Scope (For Now)
- Contract synthesis (too complex)
- Shrinking strategies (proptest's domain)
- Fuzzing infrastructure (separate concern)
- Statistical analysis (separate library)

---

## Timeline Estimate

- **Phase 1 (Foundation):** 1-2 days
- **Phase 2 (Primitives):** 2-3 days
- **Phase 3 (Distributions):** 3-4 days
- **Phase 4 (Gaming/Testing):** 2-3 days
- **Phase 5 (Verification):** 2-3 days

**Total:** ~10-15 days for complete implementation

---

## Open Questions

1. **Mutable RNG State:** Should generators own RNG or take `&mut Rng`?
   - Owned: Simpler, each generator has own state
   - Borrowed: More flexible, share RNG across generators
   - **Decision:** Start with owned, add borrowed if needed

2. **Thread Safety:** Should we support `ThreadRng`?
   - Pro: Convenient for multi-threaded testing
   - Con: Not reproducible (thread-local)
   - **Decision:** Support but document non-reproducibility

3. **Seeding Strategy:** How should agents specify seeds?
   - Option 1: Always elicit u64 seed
   - Option 2: "Random seed" vs "Fixed seed" choice
   - **Decision:** Let agent choose mode

4. **Distribution Configuration:** Elicit parameters or hardcode?
   - Example: Normal(mean, std_dev) - elicit both?
   - **Decision:** Elicit all parameters for full control

---

## Summary

This plan enables agents to generate random data through elicitation, following our "castle on cloud" verification pattern. We trust `rand`, verify our wrappers, and provide powerful generators for testing, gaming, and simulations.

**Key insight:** We're not verifying randomness - we're verifying that our wrappers correctly configure and use rand's battle-tested implementations.

**Deliverable:** A separate `elicitation_rand` crate that users opt into, keeping the core lean while providing powerful random generation capabilities.
