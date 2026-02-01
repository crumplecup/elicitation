//! Mechanism-level verification contracts for elicitation methods.
//!
//! These contracts verify that elicitation *mechanisms* work correctly,
//! independent of the types being elicited.
//!
//! # Key Insight
//!
//! We verify two layers:
//! 1. **Mechanism layer**: Survey returns valid enum variant
//! 2. **Type layer**: u32 is non-zero
//!
//! These compose: If mechanism proven AND type proven, the whole chain is proven.

use crate::traits::Elicitation;
use crate::verification::Contract;

// ============================================================================
// Survey Mechanism Contracts
// ============================================================================

/// Contract for Survey mechanism: Returns valid enum variant.
///
/// Verifies that Survey elicitation returns one of the declared enum variants.
///
/// # Type Parameters
///
/// * `E` - Enum type being surveyed
///
/// # Properties
///
/// - **Precondition**: None (always callable)
/// - **Postcondition**: Output is a valid variant of E
/// - **Invariant**: E has at least one variant
///
/// # Example
///
/// ```rust,ignore
/// enum Priority { Low, Medium, High }
///
/// let contract = SurveyReturnsValidVariant::<Priority>;
/// let result = Priority::Low;
/// assert!(SurveyReturnsValidVariant::<Priority>::ensures(&(), &result));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct SurveyReturnsValidVariant<E> {
    _phantom: std::marker::PhantomData<E>,
}

impl<E> SurveyReturnsValidVariant<E> {
    /// Create new Survey mechanism contract.
    pub const fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<E> Default for SurveyReturnsValidVariant<E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E> Contract for SurveyReturnsValidVariant<E>
where
    E: Elicitation + Clone + std::fmt::Debug + Send + PartialEq,
{
    type Input = E;
    type Output = E;

    fn requires(_input: &E) -> bool {
        // No precondition - Survey always callable
        true
    }

    fn ensures(_input: &E, output: &E) -> bool {
        // Postcondition: output is a valid instance of E
        // For enums, this is trivially true (Rust's type system guarantees it)
        // But we make it explicit for formal verification

        // We can't enumerate variants generically in Rust,
        // but the type system guarantees output is a valid E
        let _ = output;
        true
    }

    fn invariant(&self) -> bool {
        // Invariant: Type E exists and is constructible
        true
    }
}

// ============================================================================
// Affirm Mechanism Contracts
// ============================================================================

/// Contract for Affirm mechanism: Returns boolean.
///
/// Verifies that Affirm elicitation returns a valid boolean value.
///
/// # Properties
///
/// - **Precondition**: None (always callable)
/// - **Postcondition**: Output is true or false
/// - **Invariant**: Boolean domain is {true, false}
///
/// # Example
///
/// ```rust,ignore
/// let contract = AffirmReturnsBoolean;
/// assert!(AffirmReturnsBoolean::ensures(&(), &true));
/// assert!(AffirmReturnsBoolean::ensures(&(), &false));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct AffirmReturnsBoolean;

impl Contract for AffirmReturnsBoolean {
    type Input = bool;
    type Output = bool;

    fn requires(_input: &bool) -> bool {
        true
    }

    fn ensures(_input: &bool, _output: &bool) -> bool {
        // Postcondition: always satisfied (bool is always valid)
        true
    }

    fn invariant(&self) -> bool {
        true
    }
}

// ============================================================================
// Text Mechanism Contracts
// ============================================================================

/// Contract for Text mechanism: Returns string.
///
/// Verifies that Text elicitation returns a string value.
///
/// # Variants
///
/// - `TextReturnsString`: Any string (including empty)
/// - `TextReturnsNonEmpty`: Non-empty string only
///
/// # Properties
///
/// - **Precondition**: None (always callable)
/// - **Postcondition**: Output is a valid string
/// - **Invariant**: String is UTF-8 encoded
#[derive(Debug, Clone, Copy)]
pub struct TextReturnsString;

impl Contract for TextReturnsString {
    type Input = String;
    type Output = String;

    fn requires(_input: &String) -> bool {
        true
    }

    fn ensures(_input: &String, output: &String) -> bool {
        // Postcondition: output is a valid string (always satisfied)
        let _ = output;
        true
    }

    fn invariant(&self) -> bool {
        true
    }
}

/// Contract for Text mechanism: Returns non-empty string.
///
/// Stronger variant that guarantees non-empty output.
#[derive(Debug, Clone, Copy)]
pub struct TextReturnsNonEmpty;

impl Contract for TextReturnsNonEmpty {
    type Input = String;
    type Output = String;

    fn requires(_input: &String) -> bool {
        true
    }

    fn ensures(_input: &String, output: &String) -> bool {
        // Postcondition: output is non-empty
        !output.is_empty()
    }

    fn invariant(&self) -> bool {
        true
    }
}

// ============================================================================
// Numeric Mechanism Contracts
// ============================================================================

/// Contract for Numeric mechanism: Returns number in valid range.
///
/// Verifies that numeric elicitation returns value within type bounds.
///
/// # Type Parameters
///
/// * `T` - Numeric type (i32, u64, etc.)
///
/// # Properties
///
/// - **Precondition**: None (always callable)
/// - **Postcondition**: Output is within T::MIN..=T::MAX
/// - **Invariant**: Type T has defined bounds
#[derive(Debug, Clone, Copy)]
pub struct NumericReturnsValid<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> NumericReturnsValid<T> {
    /// Create new Numeric mechanism contract.
    pub const fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T> Default for NumericReturnsValid<T> {
    fn default() -> Self {
        Self::new()
    }
}

// Implement for signed integers
macro_rules! impl_numeric_contract {
    ($t:ty) => {
        impl Contract for NumericReturnsValid<$t> {
            type Input = $t;
            type Output = $t;

            fn requires(_input: &$t) -> bool {
                true
            }

            fn ensures(_input: &$t, output: &$t) -> bool {
                // Postcondition: output is valid $t (always satisfied by type system)
                *output >= <$t>::MIN && *output <= <$t>::MAX
            }

            fn invariant(&self) -> bool {
                true
            }
        }
    };
}

impl_numeric_contract!(i8);
impl_numeric_contract!(i16);
impl_numeric_contract!(i32);
impl_numeric_contract!(i64);
impl_numeric_contract!(i128);
impl_numeric_contract!(isize);
impl_numeric_contract!(u8);
impl_numeric_contract!(u16);
impl_numeric_contract!(u32);
impl_numeric_contract!(u64);
impl_numeric_contract!(u128);
impl_numeric_contract!(usize);

// ============================================================================
// Composition: Mechanism + Type Contracts
// ============================================================================

/// Composed contract: Mechanism guarantees + Type constraints.
///
/// Verifies both that:
/// 1. The elicitation mechanism works correctly
/// 2. The resulting value satisfies type-specific constraints
///
/// # Example
///
/// ```rust,ignore
/// // Mechanism: Numeric returns valid i32
/// // Type: i32 must be positive
/// let contract = MechanismWithType::new(
///     NumericReturnsValid::<i32>::new(),
///     I32Positive
/// );
/// ```
#[derive(Debug, Clone, Copy)]
pub struct MechanismWithType<M, T> {
    /// Mechanism-level contract
    pub mechanism: M,
    /// Type-level contract
    pub type_contract: T,
}

impl<M, T> MechanismWithType<M, T> {
    /// Create new composed mechanism + type contract.
    pub const fn new(mechanism: M, type_contract: T) -> Self {
        Self {
            mechanism,
            type_contract,
        }
    }
}

impl<M, T, I, O> Contract for MechanismWithType<M, T>
where
    M: Contract<Input = I, Output = O>,
    T: Contract<Input = I, Output = O>,
    I: Elicitation + Clone + std::fmt::Debug + Send,
    O: Elicitation + Clone + std::fmt::Debug + Send,
{
    type Input = I;
    type Output = O;

    fn requires(input: &I) -> bool {
        // Precondition: Both contracts must accept the input
        M::requires(input) && T::requires(input)
    }

    fn ensures(input: &I, output: &O) -> bool {
        // Postcondition: BOTH contracts must hold
        // 1. Mechanism contract (output is valid for mechanism)
        M::ensures(input, output) &&
        // 2. Type contract (output satisfies type constraints)
        T::ensures(input, output)
    }

    fn invariant(&self) -> bool {
        // Invariant: Both contracts maintain their invariants
        self.mechanism.invariant() && self.type_contract.invariant()
    }
}

// ============================================================================
// Input Mechanism Contracts
// ============================================================================

/// Contract for Input mechanism: Returns non-empty input.
#[derive(Debug, Clone, Copy)]
pub struct InputNonEmpty;

impl Contract for InputNonEmpty {
    type Input = String;
    type Output = String;

    fn requires(_input: &String) -> bool {
        true
    }

    fn ensures(_input: &String, output: &String) -> bool {
        !output.is_empty()
    }

    fn invariant(&self) -> bool {
        true
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::verification::contracts::I32Positive;

    #[test]
    fn test_affirm_mechanism() {
        let input = true;
        assert!(AffirmReturnsBoolean::requires(&input));
        assert!(AffirmReturnsBoolean::ensures(&input, &true));
        assert!(AffirmReturnsBoolean::ensures(&input, &false));
    }

    #[test]
    fn test_text_mechanism() {
        let input = String::new();
        assert!(TextReturnsString::requires(&input));
        assert!(TextReturnsString::ensures(&input, &String::from("hello")));
        assert!(TextReturnsString::ensures(&input, &String::new()));
    }

    #[test]
    fn test_text_non_empty_mechanism() {
        let input = String::new();
        assert!(TextReturnsNonEmpty::requires(&input));
        assert!(TextReturnsNonEmpty::ensures(&input, &String::from("hello")));
        assert!(!TextReturnsNonEmpty::ensures(&input, &String::new()));
    }

    #[test]
    fn test_numeric_mechanism() {
        let input = 0i32;
        assert!(NumericReturnsValid::<i32>::requires(&input));
        assert!(NumericReturnsValid::<i32>::ensures(&input, &42));
        assert!(NumericReturnsValid::<i32>::ensures(&input, &-1));
        assert!(NumericReturnsValid::<i32>::ensures(&input, &i32::MIN));
        assert!(NumericReturnsValid::<i32>::ensures(&input, &i32::MAX));
    }

    #[test]
    fn test_mechanism_with_type_composition() {
        let mechanism = NumericReturnsValid::<i32>::new();
        let type_contract = I32Positive;
        let contract = MechanismWithType::new(mechanism, type_contract);

        // Positive values pass both mechanism and type contracts
        let positive = 42i32;
        assert!(
            MechanismWithType::<NumericReturnsValid<i32>, I32Positive>::ensures(
                &positive, &positive
            )
        );

        // Negative values fail type contract (but pass mechanism)
        let negative = -1i32;
        assert!(
            !MechanismWithType::<NumericReturnsValid<i32>, I32Positive>::ensures(
                &negative, &negative
            )
        );

        assert!(contract.invariant());
    }
}
