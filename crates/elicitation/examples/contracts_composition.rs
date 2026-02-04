//! Multi-step proof composition example.
//!
//! This example demonstrates building complex workflows where:
//! - Each step establishes new guarantees
//! - Later steps depend on earlier guarantees
//! - The type system tracks dependencies automatically
//! - No redundant validation needed
//!
//! Run with: `cargo run --example contracts_composition`

use elicitation::contracts::{both, fst, snd, And, Established, Implies, Is, Prop, Refines};

// Domain propositions
struct EmailFormatValid;
struct EmailExists;
struct EmailConfirmed;
struct UserProfileCreated;

impl Prop for EmailFormatValid {}
impl Prop for EmailExists {}
impl Prop for EmailConfirmed {}
impl Prop for UserProfileCreated {}

// Implication: Email existence implies format validity
impl Implies<EmailFormatValid> for EmailExists {}

// Implication: Confirmation implies existence
impl Implies<EmailExists> for EmailConfirmed {}

/// Refined email types
struct ValidatedEmail(String);
struct ConfirmedEmail(String);

impl Refines<String> for ValidatedEmail {}
impl Refines<String> for ConfirmedEmail {}
impl Refines<ValidatedEmail> for ConfirmedEmail {}

/// Step 1: Validate email format.
fn validate_email_format(email: &str) -> Result<Established<EmailFormatValid>, String> {
    if !email.contains('@') {
        return Err("Missing @ symbol".to_string());
    }
    if !email.contains('.') {
        return Err("Missing domain".to_string());
    }

    println!("✓ Step 1: Email format valid");
    Ok(Established::assert())
}

/// Step 2: Check if email exists (requires format to be valid).
///
/// In real system, would query mail server.
fn check_email_exists(
    email: &str,
    _format_proof: Established<EmailFormatValid>,
) -> Result<Established<EmailExists>, String> {
    // Simulate checking if email exists
    if email.starts_with("nonexistent") {
        return Err("Email does not exist".to_string());
    }

    println!("✓ Step 2: Email exists (format was pre-validated)");
    Ok(Established::assert())
}

/// Step 3: Send confirmation code.
///
/// Requires email exists (format validation is implied).
fn send_confirmation_code(
    email: &str,
    _exists_proof: Established<EmailExists>,
) -> Result<String, String> {
    let code = "123456";
    println!(
        "✓ Step 3: Confirmation code sent to {} (code: {})",
        email, code
    );
    Ok(code.to_string())
}

/// Step 4: Verify confirmation code.
///
/// Requires email exists and establishes email is confirmed.
fn verify_confirmation_code(
    email: &str,
    code: &str,
    expected_code: &str,
    _exists_proof: Established<EmailExists>,
) -> Result<Established<EmailConfirmed>, String> {
    if code != expected_code {
        return Err("Invalid confirmation code".to_string());
    }

    println!("✓ Step 4: Email confirmed");
    Ok(Established::assert())
}

/// Step 5: Create user profile.
///
/// Requires email is confirmed.
fn create_user_profile(
    email: String,
    _confirmed_proof: Established<EmailConfirmed>,
) -> Result<Established<UserProfileCreated>, String> {
    println!("✓ Step 5: User profile created for {}", email);
    Ok(Established::assert())
}

/// Final step: Welcome email.
///
/// Requires both email confirmation AND profile creation.
fn send_welcome_email(
    email: &str,
    _proof: Established<And<EmailConfirmed, UserProfileCreated>>,
) {
    println!("✓ Step 6: Welcome email sent to {}", email);
}

/// Run the complete registration workflow.
fn run_registration_workflow(email: &str) -> Result<(), String> {
    println!("=== Starting Registration for {} ===\n", email);

    // Step 1: Validate format
    let format_proof = validate_email_format(email)?;

    // Step 2: Check existence (uses format proof)
    let exists_proof = check_email_exists(email, format_proof)?;

    // Step 3: Send code (uses existence proof)
    let code = send_confirmation_code(email, exists_proof.clone())?;

    // Step 4: Verify code (uses existence proof, establishes confirmation)
    let user_code = "123456"; // In real app, would get from user
    let confirmed_proof = verify_confirmation_code(email, user_code, &code, exists_proof)?;

    // Step 5: Create profile (uses confirmation proof)
    let profile_proof = create_user_profile(email.to_string(), confirmed_proof.clone())?;

    // Step 6: Send welcome (needs BOTH proofs)
    let combined_proof = both(confirmed_proof, profile_proof);
    send_welcome_email(email, combined_proof);

    Ok(())
}

fn main() {
    println!("=== Multi-Step Proof Composition Example ===\n");

    // Success case
    match run_registration_workflow("user@example.com") {
        Ok(()) => println!("\n✅ Registration completed successfully!\n"),
        Err(e) => println!("\n❌ Registration failed: {}\n", e),
    }

    // Failure case: bad format
    println!("=== Testing Failure Case: Bad Format ===\n");
    match run_registration_workflow("invalid-email") {
        Ok(()) => println!("\n✅ Registration completed\n"),
        Err(e) => println!("\n❌ Registration failed: {}\n", e),
    }

    println!("=== Key Insights ===");
    println!("1. Each step builds on previous guarantees");
    println!("2. Cannot skip validation steps (type-checked)");
    println!("3. No redundant validation needed");
    println!("4. Implications (EmailExists → EmailFormatValid) tracked by types");
    println!("5. Zero runtime cost - all proofs compile away");

    println!("\n=== Proof Dependencies ===");
    println!("Step 1 (validate)  → EmailFormatValid");
    println!("Step 2 (exists)    → EmailExists (requires EmailFormatValid)");
    println!("Step 3 (send code) → uses EmailExists");
    println!("Step 4 (verify)    → EmailConfirmed (requires EmailExists)");
    println!("Step 5 (profile)   → UserProfileCreated (requires EmailConfirmed)");
    println!("Step 6 (welcome)   → requires EmailConfirmed AND UserProfileCreated");

    println!("\n=== What the Type System Prevents ===");
    println!("✗ Calling check_email_exists() without format validation");
    println!("✗ Creating profile without email confirmation");
    println!("✗ Sending welcome email without both proofs");
    println!("✗ Skipping any step in the workflow");
}
