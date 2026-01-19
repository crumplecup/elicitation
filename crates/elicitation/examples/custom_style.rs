//! Example demonstrating custom styles for built-in types.
//!
//! This example shows how to define and use custom style types
//! for built-in types like i32, enabling surgical prompt customization.

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Custom Style Example ===\n");

    // For this example, we demonstrate the type system without
    // actually calling APIs (would require botticelli dependency)
    println!("This example demonstrates the ElicitationStyle trait API.");
    println!("It shows how custom styles can be defined and used.\n");

    // Example: Define custom styles for built-in types
    #[derive(Clone, Default, Debug)]
    #[allow(dead_code)]
    struct CompactI32Style;
    
    #[derive(Clone, Default, Debug)]
    #[allow(dead_code)]
    struct CurtStringStyle;
    
    #[derive(Clone, Default, Debug)]
    #[allow(dead_code)]
    struct VerboseI32Style;

    println!("Defined custom style types:");
    println!("  - CompactI32Style for i32");
    println!("  - CurtStringStyle for String");
    println!("  - VerboseI32Style for i32");
    
    println!("\nUsage patterns:");
    println!("  1. Default: let age = i32::elicit(&client).await?;");
    println!("  2. Custom:  let client = client.with_style::<i32, CompactI32Style>(CompactI32Style);");
    println!("              let age = i32::elicit(&client).await?;");
    println!("  3. Chain:   let client = client");
    println!("                  .with_style::<i32, VerboseI32Style>(VerboseI32Style)");
    println!("                  .with_style::<String, CurtStringStyle>(CurtStringStyle);");

    println!("\nBenefits:");
    println!("  - Surgical prompt customization per type");
    println!("  - Fallback to default when no custom style set");
    println!("  - Type-safe: compiler ensures correct style for each type");
    println!("  - Extensible: users can define styles without modifying library");

    println!("\n=== Done ===\n");
    Ok(())
}
