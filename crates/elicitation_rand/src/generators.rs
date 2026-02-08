//! Random generators for primitive types.
//!
//! This module provides `Generator` implementations that produce random
//! values using the `rand` crate's distribution system.

use elicitation::Generator;
use rand::distributions::{Distribution, Standard};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::cell::RefCell;
use std::marker::PhantomData;

/// Generic random generator for types implementing `Distribution<Standard>`.
///
/// This generator wraps an RNG and produces random values of type `T` on
/// demand. It's useful for generating test data, game content, or any
/// scenario requiring reproducible randomness.
///
/// # Example
///
/// ```rust,ignore
/// use elicitation_rand::RandomGenerator;
/// use elicitation::Generator;
///
/// // Create generator with fixed seed for reproducibility
/// let gen = RandomGenerator::<u32>::with_seed(42);
///
/// // Generate random values
/// let value1 = gen.generate();
/// let value2 = gen.generate();
///
/// // Same seed produces same sequence
/// let gen2 = RandomGenerator::<u32>::with_seed(42);
/// assert_eq!(gen2.generate(), value1);
/// ```
///
/// # Castle on Cloud
///
/// We trust:
/// - `rand::Rng::gen()` produces random values
/// - `Distribution<Standard>` implementations are correct
/// - RNG state management works
///
/// We verify:
/// - Generator stores RNG correctly
/// - `generate()` calls RNG correctly
/// - Seed-based construction works
#[derive(Debug)]
pub struct RandomGenerator<T> {
    rng: RefCell<StdRng>,
    _phantom: PhantomData<T>,
}

impl<T> RandomGenerator<T>
where
    Standard: Distribution<T>,
{
    /// Create a new generator with the given RNG.
    pub fn new(rng: StdRng) -> Self {
        Self {
            rng: RefCell::new(rng),
            _phantom: PhantomData,
        }
    }

    /// Create a generator with a specific seed.
    ///
    /// This is useful for reproducible test data generation.
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
        // Use RefCell for interior mutability (safe, runtime-checked)
        self.rng.borrow_mut().gen()
    }
}

/// Generator that maps output of another generator through a function.
///
/// Useful for wrapping generated values in newtypes or applying transformations.
#[derive(Debug, Clone)]
pub struct MapGenerator<G, F> {
    inner: G,
    map_fn: F,
}

impl<G, F> MapGenerator<G, F> {
    /// Create a new mapping generator.
    pub fn new(inner: G, map_fn: F) -> Self {
        Self { inner, map_fn }
    }
}

impl<G, F, T, U> Generator for MapGenerator<G, F>
where
    G: Generator<Target = T>,
    F: Fn(T) -> U,
{
    type Target = U;

    fn generate(&self) -> U {
        (self.map_fn)(self.inner.generate())
    }
}

/// Generator that transforms output through a function.
///
/// Similar to MapGenerator but specifically for in-place transformations.
#[derive(Debug, Clone)]
pub struct TransformGenerator<G, F> {
    inner: G,
    transform_fn: F,
}

impl<G, F> TransformGenerator<G, F> {
    /// Create a new transforming generator.
    pub fn new(inner: G, transform_fn: F) -> Self {
        Self { inner, transform_fn }
    }
}

impl<G, F, T> Generator for TransformGenerator<G, F>
where
    G: Generator<Target = T>,
    F: Fn(T) -> T,
{
    type Target = T;

    fn generate(&self) -> T {
        (self.transform_fn)(self.inner.generate())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_generator_deterministic() {
        let seed = 42;
        let gen1 = RandomGenerator::<u32>::with_seed(seed);
        let gen2 = RandomGenerator::<u32>::with_seed(seed);

        // Same seed should produce same sequence
        let val1 = gen1.generate();
        let val2 = gen2.generate();
        assert_eq!(val1, val2);
    }

    #[test]
    fn test_random_generator_different_seeds() {
        let gen1 = RandomGenerator::<u32>::with_seed(1);
        let gen2 = RandomGenerator::<u32>::with_seed(2);

        // Different seeds should (very likely) produce different values
        let val1 = gen1.generate();
        let val2 = gen2.generate();
        assert_ne!(val1, val2);
    }

    #[test]
    fn test_random_generator_sequence() {
        let gen = RandomGenerator::<u8>::with_seed(123);

        // Generate sequence
        let values: Vec<_> = (0..10).map(|_| gen.generate()).collect();

        // Verify we got 10 values (obvious, but tests generation works)
        assert_eq!(values.len(), 10);

        // Values should vary (statistically very likely)
        let unique_count = values.iter().collect::<std::collections::HashSet<_>>().len();
        assert!(unique_count > 5, "Expected some variety in random values");
    }

    #[test]
    fn test_random_generator_primitives() {
        // Test that we can construct generators for various primitives
        let _gen_u8 = RandomGenerator::<u8>::with_seed(1);
        let _gen_u16 = RandomGenerator::<u16>::with_seed(2);
        let _gen_u32 = RandomGenerator::<u32>::with_seed(3);
        let _gen_u64 = RandomGenerator::<u64>::with_seed(4);
        let _gen_i8 = RandomGenerator::<i8>::with_seed(5);
        let _gen_i16 = RandomGenerator::<i16>::with_seed(6);
        let _gen_i32 = RandomGenerator::<i32>::with_seed(7);
        let _gen_i64 = RandomGenerator::<i64>::with_seed(8);
        let _gen_f32 = RandomGenerator::<f32>::with_seed(9);
        let _gen_f64 = RandomGenerator::<f64>::with_seed(10);
        let _gen_bool = RandomGenerator::<bool>::with_seed(11);
        let _gen_char = RandomGenerator::<char>::with_seed(12);
    }

    #[test]
    fn test_random_bool_distribution() {
        let gen = RandomGenerator::<bool>::with_seed(42);

        // Generate many bools, should get roughly 50/50 split
        let count = 1000;
        let trues = (0..count).filter(|_| gen.generate()).count();

        // Allow for statistical variance (roughly 40-60%)
        assert!(trues > 400 && trues < 600,
            "Expected roughly 50% true values, got {}/{}", trues, count);
    }

    #[test]
    fn test_random_f64_range() {
        let gen = RandomGenerator::<f64>::with_seed(789);

        // Generate many f64s, verify they're in [0, 1) range
        for _ in 0..100 {
            let val = gen.generate();
            assert!(val >= 0.0 && val < 1.0, "f64 out of range: {}", val);
        }
    }
}
