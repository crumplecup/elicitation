//! Phase 4: Auto-Selection
//!
//! Demonstrates style auto-selection: styles are only elicited when needed.
//! If a style is pre-set via `with_style()`, it's used automatically without
//! prompting the user.

use elicitation::Elicit;

#[derive(Debug, Elicit)]
#[allow(dead_code)]
struct Config {
    #[prompt("Name", style = "short")]
    #[prompt("Configuration name", style = "detailed")]
    name: String,

    #[prompt("Timeout (s)", style = "short")]
    #[prompt("Timeout in seconds", style = "detailed")]
    timeout: u32,
}

fn main() {
    println!("\n=== Phase 4: Auto-Selection ===\n");
    println!("✅ Auto-selection implemented via style_or_elicit()");
    println!("\nScenario 1: No pre-set style");
    println!("  let config = Config::elicit(&client).await?;");
    println!("  → User is prompted to choose: Default, Short, or Detailed");
    println!("  → Then fields are elicited with chosen style");
    println!("\nScenario 2: Pre-set style");
    println!("  let client = client.with_style::<Config, _>(ConfigElicitStyle::Short);");
    println!("  let config = Config::elicit(&client).await?;");
    println!("  → Style choice is skipped (Short is used)");
    println!("  → Fields are elicited directly with Short prompts");
    println!("\nBenefits:");
    println!("  - Silent defaults: zero ceremony when no style needed");
    println!("  - Explicit control: pre-set styles skip user prompts");
    println!("  - Composable: nested types have independent style contexts");
    println!("\n=== Phase 4 Complete! ===\n");
}
