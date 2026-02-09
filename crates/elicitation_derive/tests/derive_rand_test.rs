//! Integration tests for derive Rand macro.

use elicitation::Generator;
use elicitation_derive::Rand;

#[derive(Debug, Clone, PartialEq, Rand)]
#[rand(bounded(1, 6))]
struct D6(u32);

#[derive(Debug, Clone, PartialEq, Rand)]
#[rand(bounded(1, 20))]
struct D20(u32);

#[derive(Debug, Clone, PartialEq, Rand)]
#[rand(positive)]
struct PositiveScore(i32);

#[derive(Debug, Clone, PartialEq, Rand)]
#[rand(even)]
struct EvenNumber(u32);

#[derive(Debug, Clone, PartialEq, Rand)]
#[rand(odd)]
struct OddNumber(u32);

#[test]
fn test_bounded_d6() {
    let generator = D6::random_generator(42);

    // Generate 100 values and verify all in range [1, 6)
    for _ in 0..100 {
        let roll = generator.generate();
        assert!(
            roll.0 >= 1 && roll.0 < 6,
            "D6 roll out of bounds: {}",
            roll.0
        );
    }
}

#[test]
fn test_bounded_d20() {
    let generator = D20::random_generator(123);

    // Generate 100 values and verify all in range [1, 20)
    for _ in 0..100 {
        let roll = generator.generate();
        assert!(
            roll.0 >= 1 && roll.0 < 20,
            "D20 roll out of bounds: {}",
            roll.0
        );
    }
}

#[test]
fn test_deterministic() {
    let seed = 42;
    let gen1 = D6::random_generator(seed);
    let gen2 = D6::random_generator(seed);

    // Same seed should produce same sequence
    for _ in 0..10 {
        assert_eq!(gen1.generate(), gen2.generate());
    }
}

#[test]
fn test_positive() {
    let generator = PositiveScore::random_generator(999);

    // All values should be positive
    for _ in 0..100 {
        let score = generator.generate();
        assert!(score.0 > 0, "Score not positive: {}", score.0);
    }
}

#[test]
fn test_even() {
    let generator = EvenNumber::random_generator(555);

    // All values should be even
    for _ in 0..100 {
        let num = generator.generate();
        assert_eq!(num.0 % 2, 0, "Number not even: {}", num.0);
    }
}

#[test]
fn test_odd() {
    let generator = OddNumber::random_generator(777);

    // All values should be odd
    for _ in 0..100 {
        let num = generator.generate();
        assert_eq!(num.0 % 2, 1, "Number not odd: {}", num.0);
    }
}
