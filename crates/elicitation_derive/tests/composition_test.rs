//! Tests for contract composition (And, Or).

use elicitation::Generator;
use elicitation_derive::Rand;

#[derive(Debug, Clone, PartialEq, Rand)]
#[rand(and(positive, even))]
struct PositiveEven(i32);

#[derive(Debug, Clone, PartialEq, Rand)]
#[rand(and(bounded(1, 100), even))]
struct BoundedEven(u32);

#[derive(Debug, Clone, PartialEq, Rand)]
#[rand(or(bounded(1, 10), bounded(90, 100)))]
struct SmallOrLarge(u32);

#[test]
fn test_and_positive_even() {
    let generator = PositiveEven::random_generator(42);

    // All values should be positive AND even
    for _ in 0..100 {
        let val = generator.generate();
        assert!(val.0 > 0, "Value not positive: {}", val.0);
        assert_eq!(val.0 % 2, 0, "Value not even: {}", val.0);
    }
}

#[test]
fn test_and_bounded_even() {
    let generator = BoundedEven::random_generator(123);

    // All values should be in [1, 100) AND even
    for _ in 0..100 {
        let val = generator.generate();
        assert!(val.0 >= 1 && val.0 < 100, "Value out of bounds: {}", val.0);
        assert_eq!(val.0 % 2, 0, "Value not even: {}", val.0);
    }
}

#[test]
fn test_or_small_or_large() {
    let generator = SmallOrLarge::random_generator(999);

    // Values should be either [1, 10) OR [90, 100)
    for _ in 0..100 {
        let val = generator.generate();
        let in_small = val.0 >= 1 && val.0 < 10;
        let in_large = val.0 >= 90 && val.0 < 100;
        assert!(
            in_small || in_large,
            "Value {} not in either range [1,10) or [90,100)",
            val.0
        );
    }
}

#[test]
fn test_composition_deterministic() {
    let seed = 42;
    let gen1 = PositiveEven::random_generator(seed);
    let gen2 = PositiveEven::random_generator(seed);

    // Same seed should produce same sequence even with composition
    for _ in 0..10 {
        assert_eq!(gen1.generate(), gen2.generate());
    }
}
