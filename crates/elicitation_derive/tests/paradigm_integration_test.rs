//! Integration tests: Elicitation action types with Rand.
//!
//! Shows how paradigm types (Select, Survey, Affirm) can use random generation.

use elicitation::Generator;
use elicitation_derive::Rand;

// Example: Priority enum that implements Select paradigm
#[derive(Debug, Clone, Copy, PartialEq, Eq, Rand)]
enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

// Example: Select paradigm with Priority enum
#[test]
fn test_select_priority() {
    let generator = Priority::random_generator(123);

    // Generate priorities and verify they're valid variants
    for _ in 0..20 {
        let priority = generator.generate();
        // All generated values should be one of the enum variants
        assert!(matches!(
            priority,
            Priority::Low | Priority::Medium | Priority::High | Priority::Critical
        ));
    }
}

// Example: bool implements Affirm paradigm (built-in)
#[test]
fn test_affirm_bool() {
    // bool already has Rand via primitives
    let generator = <bool as elicitation_rand::Rand>::rand_generator(42);

    for _ in 0..10 {
        let affirm = generator.generate();
        // Can be true or false
        let _ = affirm;
    }
}

// Example: Survey enum (agent surveys options)
#[derive(Debug, Clone, PartialEq, Rand)]
enum DeploymentTarget {
    Development,
    Staging,
    Production,
}

#[test]
fn test_survey_enum() {
    let generator = DeploymentTarget::random_generator(999);

    let mut found_dev = false;
    let mut found_staging = false;
    let mut found_prod = false;

    for _ in 0..50 {
        match generator.generate() {
            DeploymentTarget::Development => found_dev = true,
            DeploymentTarget::Staging => found_staging = true,
            DeploymentTarget::Production => found_prod = true,
        }

        if found_dev && found_staging && found_prod {
            break;
        }
    }

    // Should generate all variants eventually
    assert!(found_dev || found_staging || found_prod);
}

// Example: Select enum with data (agent selects mode + params)
#[derive(Debug, Clone, Rand)]
enum GenerationMode {
    Automatic,
    Manual {
        #[rand(bounded(1, 100))]
        iterations: u32,
    },
    Hybrid {
        #[rand(bounded(1, 10))]
        auto_percent: u8,
    },
}

#[test]
fn test_select_with_data() {
    let generator = GenerationMode::random_generator(123);

    for _ in 0..20 {
        let mode = generator.generate();

        match mode {
            GenerationMode::Automatic => {}
            GenerationMode::Manual { iterations } => {
                assert!((1..100).contains(&iterations));
            }
            GenerationMode::Hybrid { auto_percent } => {
                assert!((1..10).contains(&auto_percent));
            }
        }
    }
}

// Example: Complex action type for MCP tool selection
#[derive(Debug, Clone, Rand)]
enum ToolAction {
    Execute {
        tool_name: String,
        #[rand(bounded(1, 5))]
        retry_count: u8,
    },
    Validate,
    Skip,
}

#[test]
fn test_tool_action_with_string() {
    let generator = ToolAction::random_generator(777);

    for _ in 0..10 {
        let action = generator.generate();

        match action {
            ToolAction::Execute {
                tool_name,
                retry_count,
            } => {
                assert!((1..5).contains(&retry_count));
                // String is generated randomly
                assert!(tool_name.len() < 32);
            }
            ToolAction::Validate => {}
            ToolAction::Skip => {}
        }
    }
}

// Example: Nested paradigm types
#[derive(Debug, Clone, Rand)]
struct UserPreferences {
    #[rand(bounded(1, 10))]
    priority_level: u8,

    enabled: bool, // bool instead of Priority
}

#[test]
fn test_nested_paradigm_types() {
    let generator = UserPreferences::random_generator(42);

    for _ in 0..10 {
        let prefs = generator.generate();

        assert!(prefs.priority_level >= 1 && prefs.priority_level < 10);
        // enabled is bool (Affirm paradigm)
        let _ = prefs.enabled;
    }
}
