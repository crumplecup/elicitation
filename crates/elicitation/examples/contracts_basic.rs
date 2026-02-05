//! Basic proof-carrying code example.
//!
//! This example demonstrates the fundamentals of contracts:
//! - Defining propositions
//! - Establishing proofs
//! - Combining proofs with And
//! - Passing proofs to functions
//!
//! Run with: `cargo run --example contracts_basic`

use elicitation::contracts::{And, Established, Prop, both};

// Define propositions for our domain
struct EmailValidated;
struct PasswordStrong;
struct ConsentObtained;

impl Prop for EmailValidated {}
impl Prop for PasswordStrong {}
impl Prop for ConsentObtained {}

/// Validate email format (simplified for example).
///
/// Returns proof only if validation succeeds.
fn validate_email(email: &str) -> Option<Established<EmailValidated>> {
    if email.contains('@') && email.contains('.') {
        println!("✓ Email validated: {}", email);
        Some(Established::assert())
    } else {
        println!("✗ Invalid email format");
        None
    }
}

/// Check password strength (simplified).
///
/// Returns proof only if password is strong enough.
fn validate_password(password: &str) -> Option<Established<PasswordStrong>> {
    if password.len() >= 8 {
        println!("✓ Password is strong enough");
        Some(Established::assert())
    } else {
        println!("✗ Password too weak");
        None
    }
}

/// Get user consent.
///
/// In real system, would check consent checkbox/flow.
fn get_consent() -> Established<ConsentObtained> {
    println!("✓ Consent obtained");
    Established::assert()
}

/// Register user - requires ALL three proofs.
///
/// This function CANNOT be called without proofs that:
/// - Email is validated
/// - Password is strong
/// - Consent was obtained
///
/// The type system enforces this at compile time!
fn register_user(
    email: String,
    password: String,
    _proof: Established<And<And<EmailValidated, PasswordStrong>, ConsentObtained>>,
) {
    println!("\n🎉 User registered successfully!");
    println!("   Email: {}", email);
    println!("   Password: {} chars", password.len());
}

fn main() {
    println!("=== Basic Proof-Carrying Code Example ===\n");

    // Test data
    let email = "user@example.com";
    let password = "secure123";

    println!("Step 1: Validate email");
    let email_proof = match validate_email(email) {
        Some(proof) => proof,
        None => {
            println!("\nRegistration failed: invalid email");
            return;
        }
    };

    println!("\nStep 2: Validate password");
    let password_proof = match validate_password(password) {
        Some(proof) => proof,
        None => {
            println!("\nRegistration failed: weak password");
            return;
        }
    };

    println!("\nStep 3: Get consent");
    let consent_proof = get_consent();

    println!("\nStep 4: Combine proofs");
    // Combine email and password proofs
    let email_and_password = both(email_proof, password_proof);
    println!("✓ Email and password proofs combined");

    // Combine with consent proof
    let all_proofs = both(email_and_password, consent_proof);
    println!("✓ All proofs combined");

    println!("\nStep 5: Register user");
    register_user(email.to_string(), password.to_string(), all_proofs);

    // Try to uncomment this line - it won't compile!
    // register_user(email.to_string(), password.to_string(), ...);
    // Error: missing required proof parameter

    println!("\n=== Key Takeaway ===");
    println!("The type system prevents calling register_user() without");
    println!("establishing all required preconditions. No runtime checks needed!");
}
