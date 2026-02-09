# elicitation_rand

Random data generation through elicitation, following the "castle on cloud" verification pattern.

## Overview

This crate enables agents to generate random data using the elicitation framework. Agents can control randomness through conversational construction of RNGs and generators, enabling reproducible testing, gaming applications, and simulations.

## Use Cases

### Testing & QA
- Reproducible test failures with agent-controlled seeds
- Property-based testing integration
- Synthetic data generation at scale
- Audit trails for test scenarios

### Gaming & Simulations
- Agents as game masters with controlled randomness
- Procedural generation under agent guidance
- Dice rolls, loot tables, encounter generation
- Balanced RNG for gameplay fairness

## Castle on Cloud

We follow the "castle on cloud" verification pattern:

- **Trust:** `rand` crate's RNG implementations are correct
- **Verify:** Our wrapper logic using Kani symbolic gate verification
- **Don't verify:** Randomness quality (that's `rand`'s job)

We trust the `rand` ecosystem and verify only that our wrappers correctly configure and use it.

## Installation

```toml
[dependencies]
elicitation = "0.6"
elicitation_rand = "0.6"
```

## Quick Start

```rust
use elicitation::Elicitation;
use elicitation_rand::RandomGenerator;

// Agent elicits seed
let seed = u64::elicit(communicator).await?;

// Create generator
let generator = RandomGenerator::<u32>::with_seed(seed);

// Generate random values
let value = generator.generate();
```

## Status

🚧 **In Development** - See [RAND_INTEGRATION_PLAN.md](../../RAND_INTEGRATION_PLAN.md) for implementation plan.

**Planned phases:**
1. RNG elicitation (StdRng, SmallRng, ChaCha8Rng)
2. Basic generators (primitives)
3. Distribution generators (Uniform, Weighted, Normal)
4. Specialized generators (Dice, Shuffle, Sampling)
5. Kani verification (symbolic gate)

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](../../LICENSE-APACHE))
- MIT license ([LICENSE-MIT](../../LICENSE-MIT))

at your option.
