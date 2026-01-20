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
}
