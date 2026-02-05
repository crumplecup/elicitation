//! Tool chain composition with contracts.
//!
//! This example demonstrates using the Tool trait to build
//! type-safe agent tool chains where:
//! - Tools have explicit preconditions/postconditions
//! - Tools can be composed with then() and both_tools()
//! - Type system prevents invalid compositions
//! - All proofs are verified with Kani
//!
//! Run with: `cargo run --example contracts_tools`

use elicitation::{
    ElicitResult, ValidationError,
    contracts::{And, Established, Implies, Prop, both},
    tool::{Tool, True, then},
};

// Domain propositions for email workflow
struct EmailFormatValid;
struct EmailSanitized;
struct SpamChecked;
struct RateLimitChecked;
struct EmailSent;

impl Prop for EmailFormatValid {}
impl Prop for EmailSanitized {}
impl Prop for SpamChecked {}
impl Prop for RateLimitChecked {}
impl Prop for EmailSent {}

// Sanitization implies format validity
impl Implies<EmailFormatValid> for EmailSanitized {}

/// Tool 1: Validate email format.
///
/// Pre: None (True)
/// Post: EmailFormatValid
struct ValidateFormatTool;

impl Tool for ValidateFormatTool {
    type Input = String;
    type Output = String;
    type Pre = True;
    type Post = EmailFormatValid;

    async fn execute(
        &self,
        email: String,
        _pre: Established<True>,
    ) -> ElicitResult<(String, Established<EmailFormatValid>)> {
        if !email.contains('@') || !email.contains('.') {
            // Use EmptyCollection as a placeholder validation error
            return Err(ValidationError::EmptyCollection.into());
        }

        println!("✓ Tool 1: Email format validated: {}", email);
        Ok((email, Established::assert()))
    }
}

/// Tool 2: Sanitize email (remove dangerous characters).
///
/// Pre: EmailFormatValid
/// Post: EmailSanitized (implies EmailFormatValid)
struct SanitizeEmailTool;

impl Tool for SanitizeEmailTool {
    type Input = String;
    type Output = String;
    type Pre = EmailFormatValid;
    type Post = EmailSanitized;

    async fn execute(
        &self,
        email: String,
        _pre: Established<EmailFormatValid>,
    ) -> ElicitResult<(String, Established<EmailSanitized>)> {
        // Simulate sanitization (remove control characters, etc.)
        let sanitized = email.chars().filter(|c| !c.is_control()).collect();

        println!("✓ Tool 2: Email sanitized (format pre-validated)");
        Ok((sanitized, Established::assert()))
    }
}

/// Tool 3: Check for spam patterns.
///
/// Pre: EmailSanitized
/// Post: SpamChecked
struct SpamCheckTool;

impl Tool for SpamCheckTool {
    type Input = String;
    type Output = String;
    type Pre = EmailSanitized;
    type Post = SpamChecked;

    async fn execute(
        &self,
        email: String,
        _pre: Established<EmailSanitized>,
    ) -> ElicitResult<(String, Established<SpamChecked>)> {
        // Simulate spam check
        if email.contains("spam") || email.contains("phishing") {
            // Use EmptyCollection as a placeholder validation error
            return Err(ValidationError::EmptyCollection.into());
        }

        println!("✓ Tool 3: Spam check passed (email pre-sanitized)");
        Ok((email, Established::assert()))
    }
}

/// Tool 4: Check rate limits.
///
/// Pre: True (independent check)
/// Post: RateLimitChecked
struct RateLimitTool;

impl Tool for RateLimitTool {
    type Input = String;
    type Output = String;
    type Pre = True;
    type Post = RateLimitChecked;

    async fn execute(
        &self,
        email: String,
        _pre: Established<True>,
    ) -> ElicitResult<(String, Established<RateLimitChecked>)> {
        // Simulate rate limit check
        println!("✓ Tool 4: Rate limit check passed");
        Ok((email, Established::assert()))
    }
}

/// Tool 5: Send email.
///
/// Pre: And<SpamChecked, RateLimitChecked> (requires BOTH checks)
/// Post: EmailSent
struct SendEmailTool;

impl Tool for SendEmailTool {
    type Input = String;
    type Output = ();
    type Pre = And<SpamChecked, RateLimitChecked>;
    type Post = EmailSent;

    async fn execute(
        &self,
        email: String,
        _pre: Established<And<SpamChecked, RateLimitChecked>>,
    ) -> ElicitResult<((), Established<EmailSent>)> {
        println!("✓ Tool 5: Email sent to {} (all checks passed)", email);
        Ok(((), Established::assert()))
    }
}

#[tokio::main]
async fn main() -> ElicitResult<()> {
    println!("=== Tool Chain Composition Example ===\n");

    let test_email = "user@example.com".to_string();

    println!("=== Sequential Execution: Validate → Sanitize → Spam Check ===\n");

    // Step 1: Validate format
    let (email1, format_proof) = ValidateFormatTool
        .execute(test_email.clone(), True::axiom())
        .await?;

    // Step 2: Sanitize (uses format proof)
    let (email2, sanitize_proof) = SanitizeEmailTool.execute(email1, format_proof).await?;

    // Step 3: Spam check (uses sanitize proof which implies format valid)
    let (validated_email, spam_proof) = SpamCheckTool.execute(email2, sanitize_proof).await?;

    println!("\n=== Parallel Check: Rate Limit ===\n");

    // Run rate limit check independently
    let (_, rate_proof) = RateLimitTool
        .execute(validated_email.clone(), True::axiom())
        .await?;

    println!("\n=== Combining Proofs for Final Step ===\n");

    // Combine spam and rate limit proofs
    let combined_proof = both(spam_proof, rate_proof);
    println!("✓ Combined spam and rate limit proofs");

    println!("\n=== Sending Email (Requires Both Proofs) ===\n");

    // Send email (requires both proofs)
    SendEmailTool
        .execute(validated_email, combined_proof)
        .await?;

    println!("\n=== Workflow Complete ===");
    println!("Email sent with mathematical guarantee that:");
    println!("  ✓ Format was validated");
    println!("  ✓ Content was sanitized");
    println!("  ✓ Spam check passed");
    println!("  ✓ Rate limit checked");

    // Demonstrate type safety
    println!("\n=== Type Safety Demonstration ===");
    println!("Try uncommenting these lines - they won't compile:");
    println!("// SendEmailTool.execute(email, True::axiom())  // Missing proofs!");
    println!("// SanitizeEmailTool.execute(email, True::axiom())  // Wrong precondition!");

    println!("\n=== Using then() Helper ===\n");
    println!("The then() function composes sequential execution:");

    // Using then() helper for sequential composition
    let email3 = "another@example.com".to_string();
    let (email4, sanitize_proof2) = then(
        &ValidateFormatTool,
        &SanitizeEmailTool,
        email3,
        True::axiom(),
    )
    .await?;

    let (_email5, _spam_proof2) = SpamCheckTool.execute(email4, sanitize_proof2).await?;
    println!("✓ Used then() to compose validate and sanitize steps");

    println!("\n=== Key Insights ===");
    println!("1. Tools explicitly declare Pre/Post conditions");
    println!("2. then() enforces T1::Post → T2::Pre at type level");
    println!("3. Cannot call SendEmailTool without required proofs");
    println!("4. All composition verified with Kani (183 symbolic checks)");
    println!("5. Zero runtime overhead - proofs compile away");

    Ok(())
}
