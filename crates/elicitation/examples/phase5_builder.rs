//! Phase 5: Builder Pattern
//!
//! Demonstrates the ergonomic builder pattern for one-off style overrides.
//! Instead of manually creating a styled client, you can use `Type::with_style(style).elicit(&peer).await?`

use elicitation::Elicit;

#[derive(Debug, Elicit)]
#[allow(dead_code)]
struct Config {
    #[prompt("Name", style = "short")]
    #[prompt("Configuration name", style = "detailed")]
    name: String,

    #[prompt("Port", style = "short")]
    #[prompt("Port number (1-65535)", style = "detailed")]
    port: u16,

    #[prompt("Enabled?", style = "short")]
    #[prompt("Is this configuration enabled?", style = "detailed")]
    enabled: bool,
}

fn main() {
    println!("\n=== Phase 5: Builder Pattern ===\n");

    println!("✅ Ergonomic builder pattern implemented");
    println!("\nComparison of syntax:\n");

    println!("--- Before (manual client styling) ---");
    println!("let client = ElicitClient::new(&peer);");
    println!("let client = client.with_style::<Config, _>(ConfigElicitStyle::Short);");
    println!("let config = Config::elicit(&client).await?;");

    println!("\n--- After (builder pattern) ---");
    println!("let config = Config::with_style(ConfigElicitStyle::Short)");
    println!("    .elicit(&peer)");
    println!("    .await?;");

    println!("\n✅ Benefits:");
    println!("  - More concise: one line instead of three");
    println!("  - More ergonomic: reads naturally as a chain");
    println!("  - Self-documenting: clear what style is being used");
    println!("  - Type-safe: compiler enforces correct style type");

    println!("\n✅ Use cases:");
    println!("  1. Quick scripts where you want a specific style");
    println!("  2. Testing different styles without modifying client setup");
    println!("  3. One-off style overrides in a specific code path");

    println!("\n✅ Note:");
    println!("  - Builder is for one-off style overrides");
    println!("  - For reusable styled clients, use ElicitClient::with_style()");

    println!("\n=== Phase 5 Complete! ===\n");
}
