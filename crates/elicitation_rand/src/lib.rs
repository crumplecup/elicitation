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
//! use elicitation_rand::{RandomGenerator, StdRng};
//!
//! // Agent elicits seed for reproducibility
//! let seed = u64::elicit(communicator).await?;
//! let generator = RandomGenerator::<User>::with_seed(seed);
//!
//! // Generate test data
//! let test_users: Vec<_> = (0..1000)
//!     .map(|_| generator.generate())
//!     .collect();
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
//!
//! // Weighted loot table
//! let loot = WeightedGenerator::new(vec![
//!     (Item::CommonSword, 70),
//!     (Item::RareBow, 25),
//!     (Item::LegendaryArmor, 5),
//! ]);
//! let drop = loot.generate();
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
//! - `verify-kani` - Enable Kani verification (dev only)

#![warn(missing_docs)]
#![cfg_attr(kani, feature(kani))]

// TODO: Phase 1 - Implement RNG elicitation
// pub mod rng;

// TODO: Phase 2 - Implement basic generators
// pub mod generators;

// TODO: Phase 5 - Implement Kani verification
// #[cfg(kani)]
// mod verification;
