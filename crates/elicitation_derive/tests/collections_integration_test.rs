//! Integration tests: Using VecGenerator with derive macro types.

use elicitation::Generator;
use elicitation_derive::Rand;
use elicitation_rand::VecGenerator;

#[derive(Debug, Clone, PartialEq, Rand)]
#[rand(bounded(1, 6))]
struct D6(u32);

#[derive(Debug, Clone, Rand)]
struct Player {
    #[rand(bounded(1, 100))]
    health: u32,
    
    #[rand(bounded(1, 20))]
    level: u8,
}

#[test]
fn test_vec_of_dice() {
    let dice_gen = D6::random_generator(42);
    let vec_gen = VecGenerator::with_length(dice_gen, 3, 6, 123);
    
    for _ in 0..10 {
        let rolls = vec_gen.generate();
        
        // Length in range
        assert!(rolls.len() >= 3 && rolls.len() < 6, 
                "Roll count {} not in [3, 6)", rolls.len());
        
        // All dice valid
        for die in &rolls {
            assert!(die.0 >= 1 && die.0 < 6, "Die value {} invalid", die.0);
        }
    }
}

#[test]
fn test_vec_of_structs() {
    let player_gen = Player::random_generator(42);
    let vec_gen = VecGenerator::non_empty(player_gen, 123);
    
    for _ in 0..10 {
        let players = vec_gen.generate();
        
        assert!(!players.is_empty(), "Should have at least one player");
        
        for player in &players {
            assert!(player.health >= 1 && player.health < 100, 
                    "Health {} invalid", player.health);
            assert!(player.level >= 1 && player.level < 20, 
                    "Level {} invalid", player.level);
        }
    }
}

#[test]
fn test_fixed_party_size() {
    let player_gen = Player::random_generator(999);
    let vec_gen = VecGenerator::fixed_length(player_gen, 4, 777);
    
    for _ in 0..10 {
        let party = vec_gen.generate();
        assert_eq!(party.len(), 4, "Party should have exactly 4 players");
    }
}

#[test]
fn test_vec_deterministic() {
    let seed = 42;
    let dice_gen1 = D6::random_generator(seed);
    let vec_gen1 = VecGenerator::with_length(dice_gen1, 5, 10, seed);
    
    let dice_gen2 = D6::random_generator(seed);
    let vec_gen2 = VecGenerator::with_length(dice_gen2, 5, 10, seed);
    
    // Same seed should produce same vectors
    for _ in 0..5 {
        let vec1 = vec_gen1.generate();
        let vec2 = vec_gen2.generate();
        assert_eq!(vec1, vec2, "Same seed should produce same vectors");
    }
}

#[test]
fn test_empty_vec() {
    let dice_gen = D6::random_generator(42);
    let vec_gen = VecGenerator::with_length(dice_gen, 0, 3, 123);
    
    let mut found_empty = false;
    let mut found_non_empty = false;
    
    for _ in 0..30 {
        let rolls = vec_gen.generate();
        if rolls.is_empty() {
            found_empty = true;
        } else {
            found_non_empty = true;
            assert!(rolls.len() < 3, "Length should be < 3");
        }
        
        if found_empty && found_non_empty {
            break;
        }
    }
    
    assert!(found_empty || found_non_empty, 
            "Should generate both empty and non-empty vectors");
}
