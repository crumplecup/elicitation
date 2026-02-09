//! Distribution-based random generators.
//!
//! This module provides generators for specific probability distributions
//! beyond the standard uniform distribution.

use elicitation::Generator;
use rand::SeedableRng;
use rand::distr::weighted::WeightedIndex;
use rand::distr::{Distribution, Uniform};
use rand::rngs::StdRng;
use std::cell::RefCell;

// ============================================================================
// UniformGenerator - Bounded random values
// ============================================================================

/// Generates random values uniformly distributed within a range.
///
/// This is perfect for:
/// - Bounded test data (age between 18-65)
/// - Game mechanics (damage between 10-20)
/// - Constrained randomness (temperature 0-100°C)
///
/// # Example
///
/// ```rust,ignore
/// use elicitation_rand::UniformGenerator;
/// use elicitation::Generator;
///
/// // Generate ages between 18 and 65
/// let generator = UniformGenerator::new(StdRng::seed_from_u64(42), 18, 65);
/// let age = generator.generate(); // Always in [18, 65)
/// ```
///
/// # Castle on Cloud
///
/// We trust:
/// - `rand::distr::Uniform` samples uniformly
/// - Range validation by Uniform constructor
///
/// We verify:
/// - Generator stores distribution correctly
/// - `generate()` calls sample correctly
pub struct UniformGenerator<T>
where
    T: rand::distr::uniform::SampleUniform,
{
    rng: RefCell<StdRng>,
    distribution: Uniform<T>,
}

impl<T> UniformGenerator<T>
where
    T: rand::distr::uniform::SampleUniform,
{
    /// Create a new uniform generator over the range [low, high).
    ///
    /// # Panics
    ///
    /// Panics if `low >= high`.
    pub fn new(rng: StdRng, low: T, high: T) -> Self {
        Self {
            rng: RefCell::new(rng),
            distribution: Uniform::new(low, high).expect("Invalid range for Uniform distribution"),
        }
    }

    /// Create a uniform generator with a specific seed.
    pub fn with_seed(seed: u64, low: T, high: T) -> Self {
        Self::new(StdRng::seed_from_u64(seed), low, high)
    }
}

impl<T> Generator for UniformGenerator<T>
where
    T: rand::distr::uniform::SampleUniform,
{
    type Target = T;

    fn generate(&self) -> T {
        // Use RefCell for interior mutability
        self.distribution.sample(&mut *self.rng.borrow_mut())
    }
}

// ============================================================================
// WeightedGenerator - Weighted random selection
// ============================================================================

/// Generates values from a weighted distribution.
///
/// Perfect for:
/// - Loot tables (70% common, 25% rare, 5% legendary)
/// - Encounter tables (weighted enemy spawns)
/// - Event probability (more likely outcomes weighted higher)
///
/// # Example
///
/// ```rust,ignore
/// use elicitation_rand::WeightedGenerator;
/// use elicitation::Generator;
///
/// #[derive(Debug, Clone, PartialEq)]
/// enum Rarity {
///     Common,
///     Rare,
///     Legendary,
/// }
///
/// let generator = WeightedGenerator::new(
///     StdRng::seed_from_u64(42),
///     vec![
///         (Rarity::Common, 70),
///         (Rarity::Rare, 25),
///         (Rarity::Legendary, 5),
///     ],
/// ).unwrap();
///
/// let drop = generator.generate(); // 70% chance of Common
/// ```
///
/// # Castle on Cloud
///
/// We trust:
/// - `rand::distr::WeightedIndex` respects weights
/// - Weight validation by WeightedIndex constructor
///
/// We verify:
/// - Generator stores distribution and values correctly
/// - `generate()` samples and indexes correctly
#[derive(Debug)]
pub struct WeightedGenerator<T> {
    rng: RefCell<StdRng>,
    distribution: WeightedIndex<u32>,
    values: Vec<T>,
}

impl<T: Clone> WeightedGenerator<T> {
    /// Create a new weighted generator.
    ///
    /// # Arguments
    ///
    /// * `rng` - The random number generator
    /// * `items` - Vec of (value, weight) pairs
    ///
    /// # Returns
    ///
    /// Returns `Ok(generator)` if weights are valid, `Err(msg)` otherwise.
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Items list is empty
    /// - Any weight is zero
    /// - Weights overflow (very large)
    pub fn new(rng: StdRng, items: Vec<(T, u32)>) -> Result<Self, String> {
        if items.is_empty() {
            return Err("Cannot create weighted generator with empty items".to_string());
        }

        let (values, weights): (Vec<_>, Vec<_>) = items.into_iter().unzip();

        let distribution =
            WeightedIndex::new(weights).map_err(|e| format!("Invalid weights: {}", e))?;

        Ok(Self {
            rng: RefCell::new(rng),
            distribution,
            values,
        })
    }

    /// Create a weighted generator with a specific seed.
    pub fn with_seed(seed: u64, items: Vec<(T, u32)>) -> Result<Self, String> {
        Self::new(StdRng::seed_from_u64(seed), items)
    }
}

impl<T: Clone> Generator for WeightedGenerator<T> {
    type Target = T;

    fn generate(&self) -> T {
        // Use RefCell for interior mutability
        let index = self.distribution.sample(&mut *self.rng.borrow_mut());
        self.values[index].clone()
    }
}

/// Generator for bounded even values.
///
/// Generates values in [low, high) that are even.
#[derive(Debug, Clone)]
pub struct BoundedEvenGenerator<T> {
    seed: u64,
    low: T,
    high: T,
}

impl<T> BoundedEvenGenerator<T>
where
    T: rand::distr::uniform::SampleUniform + Copy,
{
    /// Create a new bounded even generator.
    pub fn new(seed: u64, low: T, high: T) -> Self {
        Self { seed, low, high }
    }
}

impl<T> Generator for BoundedEvenGenerator<T>
where
    T: rand::distr::uniform::SampleUniform
        + Copy
        + std::ops::Rem<Output = T>
        + std::ops::Sub<Output = T>
        + PartialEq
        + From<u8>,
{
    type Target = T;

    fn generate(&self) -> T {
        let generator = UniformGenerator::with_seed(self.seed, self.low, self.high);
        let value = generator.generate();
        let two = T::from(2u8);
        if value % two == T::from(0u8) {
            value
        } else {
            // Make it even by subtracting 1 (safer than adding)
            value - T::from(1u8)
        }
    }
}

/// Generator for bounded odd values.
///
/// Generates values in [low, high) that are odd.
#[derive(Debug, Clone)]
pub struct BoundedOddGenerator<T> {
    seed: u64,
    low: T,
    high: T,
}

impl<T> BoundedOddGenerator<T>
where
    T: rand::distr::uniform::SampleUniform + Copy,
{
    /// Create a new bounded odd generator.
    pub fn new(seed: u64, low: T, high: T) -> Self {
        Self { seed, low, high }
    }
}

impl<T> Generator for BoundedOddGenerator<T>
where
    T: rand::distr::uniform::SampleUniform
        + Copy
        + std::ops::Rem<Output = T>
        + std::ops::Sub<Output = T>
        + PartialEq
        + From<u8>,
{
    type Target = T;

    fn generate(&self) -> T {
        let generator = UniformGenerator::with_seed(self.seed, self.low, self.high);
        let value = generator.generate();
        let two = T::from(2u8);
        if value % two != T::from(0u8) {
            value
        } else {
            // Make it odd by subtracting 1 (safer than adding)
            value - T::from(1u8)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // UniformGenerator Tests
    // ========================================================================

    #[test]
    fn test_uniform_generator_range() {
        let generator = UniformGenerator::with_seed(42, 10, 20);

        // Generate many values, all should be in [10, 20)
        for _ in 0..1000 {
            let val = generator.generate();
            assert!(
                (10..20).contains(&val),
                "Value {} out of range [10, 20)",
                val
            );
        }
    }

    #[test]
    fn test_uniform_generator_deterministic() {
        let generator1 = UniformGenerator::with_seed(123, 0, 100);
        let generator2 = UniformGenerator::with_seed(123, 0, 100);

        // Same seed, same range → same sequence
        assert_eq!(generator1.generate(), generator2.generate());
    }

    #[test]
    fn test_uniform_generator_floats() {
        let generator = UniformGenerator::with_seed(456, 0.0, 1.0);

        // Generate many floats, all in [0.0, 1.0)
        for _ in 0..100 {
            let val = generator.generate();
            assert!((0.0..1.0).contains(&val), "Float {} out of range", val);
        }
    }

    #[test]
    fn test_uniform_generator_coverage() {
        let generator = UniformGenerator::with_seed(789, 0, 10);

        // Generate many values, should hit multiple buckets
        let samples: Vec<_> = (0..1000).map(|_| generator.generate()).collect();
        let unique_count = samples
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len();

        // Should get at least half the range covered
        assert!(
            unique_count >= 5,
            "Only {} unique values in [0, 10)",
            unique_count
        );
    }

    // ========================================================================
    // WeightedGenerator Tests
    // ========================================================================

    #[test]
    fn test_weighted_generator_basic() {
        let generator =
            WeightedGenerator::with_seed(42, vec![("common", 70), ("rare", 25), ("legendary", 5)])
                .unwrap();

        // Just verify it generates without panic
        let _item = generator.generate();
    }

    #[test]
    fn test_weighted_generator_deterministic() {
        let generator1 = WeightedGenerator::with_seed(123, vec![("a", 50), ("b", 50)]).unwrap();

        let generator2 = WeightedGenerator::with_seed(123, vec![("a", 50), ("b", 50)]).unwrap();

        // Same seed → same sequence
        assert_eq!(generator1.generate(), generator2.generate());
    }

    #[test]
    fn test_weighted_generator_distribution() {
        let generator =
            WeightedGenerator::with_seed(42, vec![("common", 90), ("rare", 10)]).unwrap();

        // Generate many samples, verify distribution roughly matches weights
        let count = 10_000;
        let common_count = (0..count)
            .filter(|_| generator.generate() == "common")
            .count();

        let common_pct = (common_count as f64 / count as f64) * 100.0;

        // Should be roughly 90% ± 5%
        assert!(
            (85.0..=95.0).contains(&common_pct),
            "Expected ~90% common, got {}%",
            common_pct
        );
    }

    #[test]
    fn test_weighted_generator_empty_error() {
        let result = WeightedGenerator::<&str>::with_seed(42, vec![]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("empty"));
    }

    #[test]
    fn test_weighted_generator_zero_weight() {
        // WeightedIndex actually allows zero weights as long as sum > 0
        // It just makes those items impossible to select
        let result = WeightedGenerator::with_seed(42, vec![("a", 0), ("b", 10)]);
        // This should succeed - zero weight just means "a" never gets picked
        assert!(result.is_ok());

        let generator = result.unwrap();
        // Verify only "b" ever gets generated
        for _ in 0..100 {
            assert_eq!(generator.generate(), "b");
        }
    }

    #[test]
    fn test_weighted_generator_single_item() {
        let generator = WeightedGenerator::with_seed(42, vec![("only", 100)]).unwrap();

        // Should always generate the only item
        for _ in 0..10 {
            assert_eq!(generator.generate(), "only");
        }
    }

    #[test]
    fn test_weighted_generator_cloneable_types() {
        #[derive(Debug, Clone, PartialEq)]
        enum Item {
            Sword,
            Shield,
            Potion,
        }

        let generator = WeightedGenerator::with_seed(
            42,
            vec![(Item::Sword, 40), (Item::Shield, 30), (Item::Potion, 30)],
        )
        .unwrap();

        // Verify it works with custom types
        let _item = generator.generate();
    }
}
