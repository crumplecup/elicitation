//! Test style integration with derive macro.

use elicitation::Elicit;

#[derive(Debug, Clone, Elicit)]
#[allow(dead_code)]
struct BasicConfig {
    #[prompt("Enter your name")]
    name: String,

    #[prompt("Enter your age")]
    age: u32,
}

#[derive(Debug, Clone, Elicit)]
#[allow(dead_code)]
struct StyledConfig {
    #[prompt("Enter your name", style = "curt")]
    #[prompt("Please provide your full name", style = "verbose")]
    name: String,

    #[prompt("Age?", style = "curt")]
    #[prompt("Please enter your age in years", style = "verbose")]
    age: u32,
}

#[derive(Debug, Clone, Elicit)]
#[allow(dead_code)]
struct MixedStyleConfig {
    #[prompt("Name", style = "curt")]
    #[prompt("What is your name?", style = "verbose")]
    name: String,

    // No style override for age - uses default
    #[prompt("Enter age")]
    age: u32,

    #[prompt("City", style = "curt")]
    city: String,
}

#[test]
fn test_basic_config_compiles() {
    // Just verify it compiles - actual elicitation requires MCP
}

#[test]
fn test_styled_config_compiles() {
    // Just verify it compiles - actual elicitation requires MCP
}

#[test]
fn test_mixed_style_config_compiles() {
    // Just verify it compiles - actual elicitation requires MCP
}
