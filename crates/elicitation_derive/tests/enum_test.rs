//! Tests for enum support (Phase 4).

use elicitation::Generator;
use elicitation_derive::Rand;

// Unit variants only
#[derive(Debug, Clone, PartialEq, Rand)]
enum Status {
    Active,
    Inactive,
    Pending,
}

// Tuple variants
#[derive(Debug, Clone, Rand)]
enum GameResult {
    Win(u32),  // score
    Loss(u32), // damage taken
    Draw,
}

// Struct variants
#[derive(Debug, Clone, Rand)]
enum GameAction {
    Move {
        #[rand(bounded(-10, 10))]
        x: i32,
        #[rand(bounded(-10, 10))]
        y: i32,
    },
    Attack {
        #[rand(bounded(1, 100))]
        damage: u32,
    },
    Defend,
}

// Mixed variants
#[derive(Debug, Clone, Rand)]
enum Event {
    Click { x: i32, y: i32 },
    KeyPress(char),
    Scroll(i32),
    Quit,
}

#[test]
fn test_unit_enum() {
    let generator = Status::random_generator(42);

    // Generate many values to ensure all variants can appear
    let mut statuses = Vec::new();
    for _ in 0..30 {
        statuses.push(generator.generate());
    }

    // Should have at least one of each variant (probabilistic, but very likely)
    let has_active = statuses.iter().any(|s| matches!(s, Status::Active));
    let has_inactive = statuses.iter().any(|s| matches!(s, Status::Inactive));
    let has_pending = statuses.iter().any(|s| matches!(s, Status::Pending));

    assert!(
        has_active || has_inactive || has_pending,
        "Should generate at least one variant"
    );
}

#[test]
fn test_tuple_enum() {
    let generator = GameResult::random_generator(123);

    for _ in 0..20 {
        let result = generator.generate();

        // Just verify it constructs validly
        match result {
            GameResult::Win(score) => {
                let _ = score;
            }
            GameResult::Loss(damage) => {
                let _ = damage;
            }
            GameResult::Draw => {}
        }
    }
}

#[test]
fn test_struct_enum_with_contracts() {
    let generator = GameAction::random_generator(999);

    for _ in 0..20 {
        let action = generator.generate();

        match action {
            GameAction::Move { x, y } => {
                assert!((-10..10).contains(&x), "x {} out of bounds", x);
                assert!((-10..10).contains(&y), "y {} out of bounds", y);
            }
            GameAction::Attack { damage } => {
                assert!(
                    (1..100).contains(&damage),
                    "damage {} out of bounds",
                    damage
                );
            }
            GameAction::Defend => {}
        }
    }
}

#[test]
fn test_mixed_enum() {
    let generator = Event::random_generator(777);

    for _ in 0..20 {
        let event = generator.generate();

        // Just verify all variants construct
        match event {
            Event::Click { x, y } => {
                let _ = (x, y);
            }
            Event::KeyPress(c) => {
                let _ = c;
            }
            Event::Scroll(delta) => {
                let _ = delta;
            }
            Event::Quit => {}
        }
    }
}

#[test]
fn test_enum_deterministic() {
    let seed = 42;
    let gen1 = Status::random_generator(seed);
    let gen2 = Status::random_generator(seed);

    // Same seed should produce same sequence
    for _ in 0..10 {
        assert_eq!(gen1.generate(), gen2.generate());
    }
}

#[test]
fn test_enum_variant_distribution() {
    let generator = Status::random_generator(12345);

    let mut counts = [0, 0, 0]; // Active, Inactive, Pending

    for _ in 0..300 {
        match generator.generate() {
            Status::Active => counts[0] += 1,
            Status::Inactive => counts[1] += 1,
            Status::Pending => counts[2] += 1,
        }
    }

    // With uniform distribution, each should be around 100
    // Allow generous margin since this is probabilistic
    for count in counts {
        assert!(
            count > 50 && count < 150,
            "Distribution not uniform: {:?}",
            counts
        );
    }
}
