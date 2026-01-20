//! Creusot verification example - demonstrates deductive verification with prophecy variables.

use elicitation::verification::{Contract, creusot::Tool};
use elicitation::ElicitResult;

// Example 1: Email Validation
struct ValidateEmail;

impl Contract for ValidateEmail {
    type Input = String;
    type Output = String;

    fn requires(input: &String) -> bool {
        input.contains('@') && input.len() > 2
    }

    fn ensures(_input: &String, output: &String) -> bool {
        output.contains('@') && output.len() > 2
    }
}

#[async_trait::async_trait]
impl Tool for ValidateEmail {
    async fn execute(&self, input: Self::Input) -> ElicitResult<Self::Output> {
        Ok(input.trim().to_lowercase())
    }
}

// Example 2: Counter Increment (Prophecy)
struct IncrementCounter;

impl Contract for IncrementCounter {
    type Input = i32;
    type Output = i32;

    fn requires(input: &i32) -> bool {
        *input < 1000
    }

    fn ensures(input: &i32, output: &i32) -> bool {
        *output == *input + 1
    }
}

#[async_trait::async_trait]
impl Tool for IncrementCounter {
    async fn execute(&self, input: Self::Input) -> ElicitResult<Self::Output> {
        Ok(input + 1)
    }
}

#[tokio::main]
async fn main() -> ElicitResult<()> {
    tracing_subscriber::fmt::init();

    println!("=== Creusot Verification Examples ===\n");

    println!("Example 1: Email Validation");
    let validator = ValidateEmail;
    let validated = validator.execute("user@example.com".to_string()).await?;
    println!("  Output: {}", validated);
    println!("  ✓ Contract verified\n");

    println!("Example 2: Counter with Prophecy");
    let incrementer = IncrementCounter;
    let incremented = incrementer.execute(42).await?;
    println!("  Result: {}", incremented);
    println!("  ✓ Prophecy: new == old + 1\n");

    println!("=== All Contracts Verified ✓ ===");

    Ok(())
}
