//! Default contract implementations for primitive types.
//!
//! These contracts provide basic validation properties for common types
//! with [`Elicitation`](crate::Elicitation) implementations.
//!
//! # Philosophy
//!
//! Contracts are **swappable and refinable**:
//! - We provide sensible defaults
//! - Users can replace with stricter contracts
//! - Different verifiers can be used per type
//!
//! # Verifier-Specific Implementations
//!
//! Each verifier has its own submodule:
//! - **Kani** (default): Model checking with bounded execution
//! - **Creusot**: Deductive verification with Why3
//! - **Prusti**: Separation logic with Viper
//! - **Verus**: SMT-based verification with Z3
//!
//! # Example
//!
//! ```rust,ignore
//! use elicitation::verification::contracts::StringNonEmpty;
//!
//! // Use default contract
//! let s = String::from("hello");
//! assert!(StringNonEmpty::requires(&s));
//! assert!(StringNonEmpty::ensures(&s, &s));
//! ```

use super::Contract;

// Verifier-specific contract implementations (feature-gated)
#[cfg(feature = "verify-creusot")]
pub mod creusot;

#[cfg(feature = "verify-prusti")]
pub mod prusti;

#[cfg(feature = "verify-verus")]
pub mod verus;

// ============================================================================
// String Contracts
// ============================================================================

/// Contract ensuring string is non-empty.
///
/// **Precondition:** Input string has length > 0  
/// **Postcondition:** Output string has length > 0
///
/// # Verification
///
/// This contract can be verified with:
/// - **Kani**: Bounded model checking
/// - **Creusot**: Deductive proofs  
/// - **Prusti**: Separation logic
/// - **Verus**: SMT solver
///
/// # Example
///
/// ```rust,ignore
/// let input = String::from("hello");
/// assert!(StringNonEmpty::requires(&input));
///
/// let output = input.clone();
/// assert!(StringNonEmpty::ensures(&input, &output));
/// ```
pub struct StringNonEmpty;

impl Contract for StringNonEmpty {
    type Input = String;
    type Output = String;

    fn requires(input: &String) -> bool {
        !input.is_empty()
    }

    fn ensures(_input: &String, output: &String) -> bool {
        !output.is_empty()
    }

    fn invariant(&self) -> bool {
        true // No state to check
    }
}

// ============================================================================
// Integer Contracts
// ============================================================================

/// Contract ensuring i32 is positive (> 0).
///
/// **Precondition:** Input number > 0  
/// **Postcondition:** Output number > 0
///
/// # Verification
///
/// Verifiable with all four tools:
/// - **Kani**: Bounded checking
/// - **Creusot**: Mathematical proofs
/// - **Prusti**: Range analysis
/// - **Verus**: Linear arithmetic
///
/// # Example
///
/// ```rust,ignore
/// let input = 42i32;
/// assert!(I32Positive::requires(&input));
///
/// let output = input;
/// assert!(I32Positive::ensures(&input, &output));
/// ```
pub struct I32Positive;

impl Contract for I32Positive {
    type Input = i32;
    type Output = i32;

    fn requires(input: &i32) -> bool {
        *input > 0
    }

    fn ensures(_input: &i32, output: &i32) -> bool {
        *output > 0
    }

    fn invariant(&self) -> bool {
        true
    }
}

// ============================================================================
// Boolean Contracts
// ============================================================================

/// Contract for boolean values (always valid).
///
/// **Precondition:** true (no restriction)  
/// **Postcondition:** true (no restriction)
///
/// # Purpose
///
/// Trivial contract that completes primitive coverage. Useful for:
/// - Testing verification infrastructure
/// - Contract composition
/// - Documentation completeness
///
/// # Example
///
/// ```rust,ignore
/// let input = true;
/// assert!(BoolValid::requires(&input));
///
/// let output = input;
/// assert!(BoolValid::ensures(&input, &output));
/// ```
pub struct BoolValid;

impl Contract for BoolValid {
    type Input = bool;
    type Output = bool;

    fn requires(_input: &bool) -> bool {
        true // All booleans are valid
    }

    fn ensures(_input: &bool, _output: &bool) -> bool {
        true // All booleans are valid
    }

    fn invariant(&self) -> bool {
        true
    }
}

// ============================================================================
// Additional Contracts for Composition Testing
// ============================================================================

/// Contract for i32 values that are non-negative (>= 0).
///
/// Useful for composition with I32Positive to create ranges.
#[derive(Debug, Clone, Copy)]
pub struct I32NonNegative;

impl Contract for I32NonNegative {
    type Input = i32;
    type Output = i32;

    fn requires(input: &i32) -> bool {
        *input >= 0
    }

    fn ensures(_input: &i32, output: &i32) -> bool {
        *output >= 0
    }
}

/// Contract for String values with maximum length.
///
/// Useful for composition with StringNonEmpty to create bounded strings.
#[derive(Debug, Clone, Copy)]
pub struct StringMaxLength<const MAX: usize>;

impl<const MAX: usize> Contract for StringMaxLength<MAX> {
    type Input = String;
    type Output = String;

    fn requires(input: &String) -> bool {
        input.len() <= MAX
    }

    fn ensures(_input: &String, output: &String) -> bool {
        output.len() <= MAX
    }

    fn invariant(&self) -> bool {
        MAX > 0 // Max length must be positive
    }
}

// ============================================================================
// Unsigned Integer Contracts (Phase 4.1)
// ============================================================================

/// Contract for u32 values that are non-zero.
///
/// Verifies unsigned 32-bit integers are positive (> 0).
#[derive(Debug, Clone, Copy)]
pub struct U32NonZero;

impl Contract for U32NonZero {
    type Input = u32;
    type Output = u32;

    fn requires(input: &u32) -> bool {
        *input > 0
    }

    fn ensures(_input: &u32, output: &u32) -> bool {
        *output > 0
    }
}

/// Contract for u64 values that are non-zero.
///
/// Verifies unsigned 64-bit integers are positive (> 0).
#[derive(Debug, Clone, Copy)]
pub struct U64NonZero;

impl Contract for U64NonZero {
    type Input = u64;
    type Output = u64;

    fn requires(input: &u64) -> bool {
        *input > 0
    }

    fn ensures(_input: &u64, output: &u64) -> bool {
        *output > 0
    }
}

/// Contract for u128 values that are non-zero.
///
/// Verifies unsigned 128-bit integers are positive (> 0).
#[derive(Debug, Clone, Copy)]
pub struct U128NonZero;

impl Contract for U128NonZero {
    type Input = u128;
    type Output = u128;

    fn requires(input: &u128) -> bool {
        *input > 0
    }

    fn ensures(_input: &u128, output: &u128) -> bool {
        *output > 0
    }
}

/// Contract for usize values that are non-zero.
///
/// Verifies platform-dependent unsigned integers are positive (> 0).
#[derive(Debug, Clone, Copy)]
pub struct UsizeNonZero;

impl Contract for UsizeNonZero {
    type Input = usize;
    type Output = usize;

    fn requires(input: &usize) -> bool {
        *input > 0
    }

    fn ensures(_input: &usize, output: &usize) -> bool {
        *output > 0
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_non_empty() {
        let input = String::from("hello");
        assert!(StringNonEmpty::requires(&input));
        assert!(StringNonEmpty::ensures(&input, &input));
    }

    #[test]
    fn test_string_empty_fails_precondition() {
        let input = String::new();
        assert!(!StringNonEmpty::requires(&input));
    }

    #[test]
    fn test_i32_positive() {
        let input = 42i32;
        assert!(I32Positive::requires(&input));
        assert!(I32Positive::ensures(&input, &input));
    }

    #[test]
    fn test_i32_negative_fails_precondition() {
        let input = -1i32;
        assert!(!I32Positive::requires(&input));
    }

    #[test]
    fn test_i32_zero_fails_precondition() {
        let input = 0i32;
        assert!(!I32Positive::requires(&input));
    }

    #[test]
    fn test_bool_always_valid() {
        assert!(BoolValid::requires(&true));
        assert!(BoolValid::requires(&false));
        assert!(BoolValid::ensures(&true, &true));
        assert!(BoolValid::ensures(&false, &false));
        assert!(BoolValid::ensures(&true, &false));
    }

    // Unsigned integer tests
    #[test]
    fn test_u32_non_zero() {
        assert!(U32NonZero::requires(&42u32));
        assert!(U32NonZero::ensures(&42u32, &42u32));
        assert!(!U32NonZero::requires(&0u32));
    }

    #[test]
    fn test_u64_non_zero() {
        assert!(U64NonZero::requires(&42u64));
        assert!(U64NonZero::ensures(&42u64, &42u64));
        assert!(!U64NonZero::requires(&0u64));
    }

    #[test]
    fn test_u128_non_zero() {
        assert!(U128NonZero::requires(&42u128));
        assert!(U128NonZero::ensures(&42u128, &42u128));
        assert!(!U128NonZero::requires(&0u128));
    }

    #[test]
    fn test_usize_non_zero() {
        assert!(UsizeNonZero::requires(&42usize));
        assert!(UsizeNonZero::ensures(&42usize, &42usize));
        assert!(!UsizeNonZero::requires(&0usize));
    }
}
