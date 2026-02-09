//! Tests for named struct support (Phase 3).

use elicitation::Generator;
use elicitation_derive::Rand;

#[derive(Debug, Clone, Rand)]
struct ServerConfig {
    #[rand(bounded(8000, 9000))]
    port: u16,

    #[rand(and(positive, even))]
    timeout_ms: u32,

    retries: u8, // No contract - fully random
}

#[derive(Debug, Clone, Rand)]
struct Point {
    #[rand(bounded(-100, 100))]
    x: i32,

    #[rand(bounded(-100, 100))]
    y: i32,
}

#[derive(Debug, Clone, Rand)]
struct GameSettings {
    #[rand(bounded(1, 6))]
    dice_sides: u8,

    #[rand(odd)]
    player_count: u8,

    #[rand(even)]
    npc_count: u8,
}

#[test]
fn test_server_config() {
    let generator = ServerConfig::random_generator(42);

    for _ in 0..20 {
        let config = generator.generate();

        // Port in range
        assert!(
            config.port >= 8000 && config.port < 9000,
            "Port {} out of range [8000, 9000)",
            config.port
        );

        // Timeout positive and even
        assert!(
            config.timeout_ms > 0,
            "Timeout not positive: {}",
            config.timeout_ms
        );
        assert_eq!(
            config.timeout_ms % 2,
            0,
            "Timeout not even: {}",
            config.timeout_ms
        );

        // Retries can be anything (no contract)
        // Just verify it exists
        let _ = config.retries;
    }
}

#[test]
fn test_point() {
    let generator = Point::random_generator(123);

    for _ in 0..20 {
        let point = generator.generate();

        assert!(
            point.x >= -100 && point.x < 100,
            "x {} out of bounds",
            point.x
        );
        assert!(
            point.y >= -100 && point.y < 100,
            "y {} out of bounds",
            point.y
        );
    }
}

#[test]
fn test_game_settings() {
    let generator = GameSettings::random_generator(999);

    for _ in 0..20 {
        let settings = generator.generate();

        assert!(
            settings.dice_sides >= 1 && settings.dice_sides < 6,
            "Dice sides {} out of range",
            settings.dice_sides
        );
        assert_eq!(
            settings.player_count % 2,
            1,
            "Player count {} not odd",
            settings.player_count
        );
        assert_eq!(
            settings.npc_count % 2,
            0,
            "NPC count {} not even",
            settings.npc_count
        );
    }
}

#[test]
fn test_struct_deterministic() {
    let seed = 42;
    let gen1 = ServerConfig::random_generator(seed);
    let gen2 = ServerConfig::random_generator(seed);

    // Same seed should produce same sequence
    for _ in 0..5 {
        let config1 = gen1.generate();
        let config2 = gen2.generate();

        assert_eq!(config1.port, config2.port);
        assert_eq!(config1.timeout_ms, config2.timeout_ms);
        assert_eq!(config1.retries, config2.retries);
    }
}
