//! Creusot proofs for mechanism contracts.

#[cfg(creusot)]
use crate::*;

#[cfg(creusot)]
use elicitation::{I8Positive, ValidationError};

/// Prove mechanism + type composition maintains contracts.
#[cfg(creusot)]
#[requires(value@ > 0)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
pub fn verify_mechanism_type_composition(value: i8) -> Result<I8Positive, ValidationError> {
    I8Positive::new(value)
}

/// Prove mechanisms preserve the trenchcoat pattern (wrap/unwrap identity).
#[cfg(creusot)]
#[requires(value@ > 0)]
#[ensures(match result {
    Ok(ref wrapped) => i8pos_inner(*wrapped) == value,
    Err(_) => false,
})]
pub fn verify_mechanism_trenchcoat_preservation(value: i8) -> Result<I8Positive, ValidationError> {
    let wrapped = I8Positive::new(value)?;
    Ok(wrapped)
}

/// Master theorem: trenchcoat pattern preserves identity.
///
/// Wrapping and unwrapping a value through a contract type preserves the
/// original value when validation succeeds. This enables zero-cost abstraction.
#[cfg(creusot)]
#[requires(value@ > 0)]
#[ensures(match result {
    Ok(ref wrapped) => i8pos_inner(*wrapped) == value,
    Err(_) => false,
})]
pub fn verify_trenchcoat_identity_preservation(value: i8) -> Result<I8Positive, ValidationError> {
    let wrapped = I8Positive::new(value)?;
    Ok(wrapped)
}
