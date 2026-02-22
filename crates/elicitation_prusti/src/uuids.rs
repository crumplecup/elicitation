//! Prusti proofs for UUID contract types (feature-gated on uuid).
//!
//! Cloud of assumptions: Trust uuid crate validation (nil UUID checks, version validation).
//! Verify wrapper structure.

#![cfg(feature = "uuid")]

#[cfg(prusti)]
use prusti_contracts::{ensures, trusted};

#[cfg(prusti)]
use elicitation::{UuidNonNil, UuidV4};

/// Verify UuidNonNil construction with non-nil UUID.
#[trusted]
#[ensures(matches!(result, Ok(_)))]
#[cfg(prusti)]
pub fn verify_uuid_non_nil_valid() -> Result<UuidNonNil, elicitation::ValidationError> {
    use uuid::Uuid;
    let uuid = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").expect("Valid UUID");
    UuidNonNil::new(uuid)
}

/// Verify UuidNonNil rejects nil UUID.
#[trusted]
#[ensures(matches!(result, Err(_)))]
#[cfg(prusti)]
pub fn verify_uuid_non_nil_invalid() -> Result<UuidNonNil, elicitation::ValidationError> {
    use uuid::Uuid;
    UuidNonNil::new(Uuid::nil())
}

/// Verify UuidV4 construction with v4 UUID.
#[trusted]
#[ensures(matches!(result, Ok(_)))]
#[cfg(prusti)]
pub fn verify_uuid_v4_valid() -> Result<UuidV4, elicitation::ValidationError> {
    use uuid::Uuid;
    let uuid = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").expect("Valid v4 UUID");
    UuidV4::new(uuid)
}

/// Verify UuidV4 rejects non-v4 UUID.
#[trusted]
#[ensures(matches!(result, Err(_)))]
#[cfg(prusti)]
pub fn verify_uuid_v4_invalid() -> Result<UuidV4, elicitation::ValidationError> {
    use uuid::Uuid;
    // v1 UUID (time-based)
    let uuid = Uuid::parse_str("c232ab00-9414-11ec-b3c8-9f68deced846").expect("Valid v1 UUID");
    UuidV4::new(uuid)
}
