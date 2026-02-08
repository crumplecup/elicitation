//! Random data generation through elicitation.
//!
//! This crate enables agents to generate random data using the elicitation
//! framework, following the "castle on cloud" verification pattern. We trust
//! the `rand` crate's implementations and verify only our wrapper logic.
//!
//! # Use Cases
//!
//! ## Testing & QA
//!
//! Generate reproducible test data with agent-controlled seeds:
//!
//! ```rust,ignore
//! use elicitation::Elicitation;
//! use elicitation_rand::StdRng;
//! use rand::Rng;
//!
//! // Agent elicits seed for reproducibility
//! let seed = u64::elicit(communicator).await?;
//! let mut rng = StdRng::seed_from_u64(seed);
//!
//! // Generate test data
//! let random_value: u32 = rng.gen();
//! ```
//!
//! ## Gaming & Simulations
//!
//! Agents as game masters with controlled randomness:
//!
//! ```rust,ignore
//! use elicitation_rand::DiceGenerator;
//!
//! // Roll for initiative
//! let dice = DiceGenerator::new(2, 6, seed); // 2d6
//! let initiative = dice.generate();
//! ```
//!
//! # Castle on Cloud
//!
//! We follow the "castle on cloud" verification pattern:
//!
//! - **Trust:** `rand` crate's RNG implementations and distributions
//! - **Verify:** Our wrapper logic (construction, configuration, integration)
//! - **Method:** Kani symbolic gate verification (no `.gen()` calls)
//!
//! We don't verify randomness quality (that's `rand`'s job). We verify that
//! our wrappers correctly configure and use rand's battle-tested implementations.
//!
//! # Features
//!
//! - Default features enable all RNG types

#![warn(missing_docs)]
#![forbid(unsafe_code)]

// Re-export RNG elicitation from main crate
pub use elicitation::rand_rng;

// Re-export common types
pub use rand::rngs::StdRng;
pub use rand_chacha::ChaCha8Rng;
pub use rand::SeedableRng;

// Phase 2 - Basic generators
pub mod generators;

// Phase 3 - Distribution generators
pub mod distributions;

// Phase 5 - Kani verification
#[cfg(all(kani, feature = "verification"))]
pub mod verification;

pub use generators::RandomGenerator;
pub use distributions::{UniformGenerator, WeightedGenerator};

// TODO: Phase 5 - Implement Kani verification
// #[cfg(kani)]
// mod verification;
