//! Test String with multiple style variants (Agent vs Human).

use elicitation::StringStyle;

/// Test type with String field that has multiple styles.
#[derive(Debug, Clone, elicitation::Elicit)]
#[allow(dead_code)] // Test struct
struct Config {
    /// Server name (supports Agent/Human styles).
    name: String,
}

#[test]
fn test_string_style_compiles() {
    // This test verifies that:
    // 1. String has a real style enum (not just Default)
    // 2. StringStyle has Agent and Human variants
    // 3. The elicitation impl works with styles
    
    let _agent_style = StringStyle::Agent;
    let _human_style = StringStyle::Human;
    let _default_style = StringStyle::default();
    
    assert_eq!(_default_style, StringStyle::Human);
}

#[test]
fn test_config_derives_correctly() {
    // Verifies that structs containing styled Strings
    // compile correctly with the derive macro
}
