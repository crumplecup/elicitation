//! Test Phase 3: Derived Type Styles with multiple field types
//!
//! This example demonstrates inline elicitation with styled prompts
//! for all supported primitive types:
//! - String (text)
//! - Integers (u8, u16, u32, u64, i8, i16, i32, i64, usize, isize)
//! - Booleans
//! - Floats (f32, f64)

use elicitation::Elicit;

/// User profile with styled prompts for all field types.
///
/// Each field has two style variants: "curt" and "verbose".
/// The style enum is automatically generated with variants: Default, Curt, Verbose.
#[derive(Debug, Elicit)]
#[allow(dead_code)]
struct UserProfile {
    // String field
    #[prompt("Username", style = "curt")]
    #[prompt("What's your username?", style = "verbose")]
    name: String,

    // Integer field (u32)
    #[prompt("Age", style = "curt")]
    #[prompt("How old are you?", style = "verbose")]
    age: u32,

    // Boolean field
    #[prompt("Premium user?", style = "curt")]
    #[prompt("Do you have a premium account?", style = "verbose")]
    is_premium: bool,

    // Float field (f64)
    #[prompt("Height (m)", style = "curt")]
    #[prompt("What is your height in meters?", style = "verbose")]
    height: f64,

    // Another integer (i32)
    #[prompt("Rank", style = "curt")]
    #[prompt("What is your ranking score?", style = "verbose")]
    score: i32,
}

/// Simplified config with just one styled field.
#[derive(Debug, Elicit)]
#[allow(dead_code)]
struct Config {
    #[prompt("Name", style = "short")]
    #[prompt("Please provide the configuration name", style = "detailed")]
    config_name: String,

    // This field has no style - uses default elicitation
    timeout_seconds: u64,
}

fn main() {
    println!("\n=== Phase 3: Derived Type Styles ===\n");
    println!("✅ UserProfile has custom styles: curt, verbose");
    println!("   - String: inline text elicitation with styled prompts");
    println!("   - u32: inline number elicitation with styled prompts");
    println!("   - bool: inline boolean elicitation with styled prompts");
    println!("   - f64: inline number elicitation with styled prompts");
    println!("   - i32: inline number elicitation with styled prompts");
    println!("\n✅ Config has custom styles: short, detailed");
    println!("   - config_name: inline text elicitation with styled prompts");
    println!("   - timeout_seconds: standard u64 elicitation (no styles)");
    println!("\n=== Phase 3 Complete! ===\n");
}
