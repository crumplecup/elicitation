//! Derive macro for contract-aware random generation.
//!
//! Enables automatic `Generator` implementation for types with contracts,
//! mapping contract constraints to appropriate sampling strategies.
//!
//! # Example
//!
//! ```rust,ignore
//! use elicitation_derive_rand::Rand;
//!
//! #[derive(Rand)]
//! #[rand(bounded(1, 6))]
//! struct D6(u32);
//!
//! let generator = D6::random_generator(42);
//! let roll = generator.generate(); // Always in [1, 6)
//! ```
//!
//! # Supported Contracts
//!
//! - `#[rand(bounded(L, H))]` - Uniform distribution in [L, H)
//! - `#[rand(positive)]` - Positive values only
//! - `#[rand(nonzero)]` - Non-zero values
//! - `#[rand(even)]` - Even values only
//! - `#[rand(odd)]` - Odd values only
//!
//! # Castle on Cloud
//!
//! We trust:
//! - rand's sampling correctness
//! - Contract definitions are accurate
//! - Rust type system enforces constraints
//!
//! We verify:
//! - Generated code constructs samplers correctly
//! - Contract parsing is accurate
//! - Sampling strategies match contracts

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod contract_parser;
mod generator_impl;

/// Derive macro for contract-aware random generation.
///
/// Generates a `Generator` implementation that respects the type's contract.
///
/// # Example
///
/// ```rust,ignore
/// #[derive(Rand)]
/// #[rand(bounded(1, 100))]
/// struct Score(u32);
/// ```
///
/// Generates:
/// ```rust,ignore
/// impl Score {
///     pub fn random_generator(seed: u64) -> impl Generator<Target = Self> {
///         MapGenerator::new(
///             UniformGenerator::with_seed(seed, 1, 100),
///             |v| Score(v)
///         )
///     }
/// }
/// ```
#[proc_macro_derive(Rand, attributes(rand))]
pub fn derive_rand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    generator_impl::expand_derive_rand(&input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
