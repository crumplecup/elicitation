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
// Signed Integer Contracts (Phase 4.2)
// ============================================================================

/// Contract for i64 values that are positive (> 0).
///
/// Verifies signed 64-bit integers are strictly positive.
#[derive(Debug, Clone, Copy)]
pub struct I64Positive;

impl Contract for I64Positive {
    type Input = i64;
    type Output = i64;

    fn requires(input: &i64) -> bool {
        *input > 0
    }

    fn ensures(_input: &i64, output: &i64) -> bool {
        *output > 0
    }
}

/// Contract for i128 values that are positive (> 0).
///
/// Verifies signed 128-bit integers are strictly positive.
#[derive(Debug, Clone, Copy)]
pub struct I128Positive;

impl Contract for I128Positive {
    type Input = i128;
    type Output = i128;

    fn requires(input: &i128) -> bool {
        *input > 0
    }

    fn ensures(_input: &i128, output: &i128) -> bool {
        *output > 0
    }
}

/// Contract for isize values that are positive (> 0).
///
/// Verifies platform-dependent signed integers are strictly positive.
#[derive(Debug, Clone, Copy)]
pub struct IsizePositive;

impl Contract for IsizePositive {
    type Input = isize;
    type Output = isize;

    fn requires(input: &isize) -> bool {
        *input > 0
    }

    fn ensures(_input: &isize, output: &isize) -> bool {
        *output > 0
    }
}

// ============================================================================
// Floating Point Contracts (Phase 4.3)
// ============================================================================

/// Contract for f32 values that are finite (not NaN or Infinity).
///
/// **Limitations:**
/// - Does not verify precision or rounding
/// - Does not distinguish +0.0 from -0.0
/// - Formal verification of floating point is limited in most tools
///
/// Verifies 32-bit floats are finite and usable.
#[derive(Debug, Clone, Copy)]
pub struct F32Finite;

impl Contract for F32Finite {
    type Input = f32;
    type Output = f32;

    fn requires(input: &f32) -> bool {
        input.is_finite()
    }

    fn ensures(_input: &f32, output: &f32) -> bool {
        output.is_finite()
    }
}

/// Contract for f64 values that are finite (not NaN or Infinity).
///
/// **Limitations:**
/// - Does not verify precision or rounding
/// - Does not distinguish +0.0 from -0.0
/// - Formal verification of floating point is limited in most tools
///
/// Verifies 64-bit floats are finite and usable.
#[derive(Debug, Clone, Copy)]
pub struct F64Finite;

impl Contract for F64Finite {
    type Input = f64;
    type Output = f64;

    fn requires(input: &f64) -> bool {
        input.is_finite()
    }

    fn ensures(_input: &f64, output: &f64) -> bool {
        output.is_finite()
    }
}

// ============================================================================
// Option<T> Contracts (Phase 5.1)
// ============================================================================

/// Contract for Option<T> that must be Some (not None).
///
/// Verifies that an Option contains a value. Does not verify the inner value.
///
/// # Example
///
/// ```rust,ignore
/// use elicitation::verification::contracts::OptionIsSome;
///
/// let contract = OptionIsSome::<i32>;
/// assert!(contract.requires(&Some(42)));
/// assert!(!contract.requires(&None));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct OptionIsSome<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> OptionIsSome<T> {
    /// Create new OptionIsSome contract.
    pub const fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T> Default for OptionIsSome<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Contract for OptionIsSome<T>
where
    T: crate::traits::Elicitation + Clone + std::fmt::Debug + Send,
{
    type Input = Option<T>;
    type Output = Option<T>;

    fn requires(input: &Option<T>) -> bool {
        input.is_some()
    }

    fn ensures(_input: &Option<T>, output: &Option<T>) -> bool {
        output.is_some()
    }
}

/// Contract for Option<T> with inner value contract.
///
/// Verifies both that Option is Some and that the inner value satisfies a contract.
///
/// # Example
///
/// ```rust,ignore
/// use elicitation::verification::contracts::{OptionWithInner, I32Positive};
///
/// let contract = OptionWithInner::new(I32Positive);
/// assert!(contract.requires(&Some(42)));
/// assert!(!contract.requires(&Some(-1))); // Inner fails
/// assert!(!contract.requires(&None));     // Is None
/// ```
#[derive(Debug, Clone, Copy)]
pub struct OptionWithInner<T, C> {
    inner_contract: C,
    _phantom: std::marker::PhantomData<T>,
}

impl<T, C> OptionWithInner<T, C> {
    /// Create new OptionWithInner contract with specified inner contract.
    pub const fn new(inner_contract: C) -> Self {
        Self {
            inner_contract,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T, C> Contract for OptionWithInner<T, C>
where
    T: crate::traits::Elicitation + Clone + std::fmt::Debug + Send,
    C: Contract<Input = T, Output = T>,
{
    type Input = Option<T>;
    type Output = Option<T>;

    fn requires(input: &Option<T>) -> bool {
        match input {
            Some(value) => C::requires(value),
            None => false,
        }
    }

    fn ensures(_input: &Option<T>, output: &Option<T>) -> bool {
        match output {
            Some(value) => C::ensures(value, value),
            None => false,
        }
    }

    fn invariant(&self) -> bool {
        self.inner_contract.invariant()
    }
}

// ============================================================================
// Result<T, E> Contracts (Phase 5.2)
// ============================================================================

/// Contract for Result<T, E> that must be Ok (not Err).
///
/// Verifies that a Result is Ok. Does not verify the inner value.
#[derive(Debug, Clone, Copy)]
pub struct ResultIsOk<T, E> {
    _phantom: std::marker::PhantomData<(T, E)>,
}

impl<T, E> ResultIsOk<T, E> {
    /// Create new ResultIsOk contract.
    pub const fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T, E> Default for ResultIsOk<T, E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, E> Contract for ResultIsOk<T, E>
where
    T: crate::traits::Elicitation + Clone + std::fmt::Debug + Send,
    E: crate::traits::Elicitation + Clone + std::fmt::Debug + Send,
{
    type Input = Result<T, E>;
    type Output = Result<T, E>;

    fn requires(input: &Result<T, E>) -> bool {
        input.is_ok()
    }

    fn ensures(_input: &Result<T, E>, output: &Result<T, E>) -> bool {
        output.is_ok()
    }
}

/// Contract for Result<T, E> with Ok value contract.
///
/// Verifies both that Result is Ok and that the inner value satisfies a contract.
#[derive(Debug, Clone, Copy)]
pub struct ResultWithOk<T, E, C> {
    ok_contract: C,
    _phantom: std::marker::PhantomData<(T, E)>,
}

impl<T, E, C> ResultWithOk<T, E, C> {
    /// Create new ResultWithOk contract with specified Ok contract.
    pub const fn new(ok_contract: C) -> Self {
        Self {
            ok_contract,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T, E, C> Contract for ResultWithOk<T, E, C>
where
    T: crate::traits::Elicitation + Clone + std::fmt::Debug + Send,
    E: crate::traits::Elicitation + Clone + std::fmt::Debug + Send,
    C: Contract<Input = T, Output = T>,
{
    type Input = Result<T, E>;
    type Output = Result<T, E>;

    fn requires(input: &Result<T, E>) -> bool {
        match input {
            Ok(value) => C::requires(value),
            Err(_) => false,
        }
    }

    fn ensures(_input: &Result<T, E>, output: &Result<T, E>) -> bool {
        match output {
            Ok(value) => C::ensures(value, value),
            Err(_) => false,
        }
    }

    fn invariant(&self) -> bool {
        self.ok_contract.invariant()
    }
}

// ============================================================================
// Vec<T> Contracts (Phase 5.3)
// ============================================================================

/// Contract for Vec<T> that must be non-empty.
///
/// Verifies that a vector contains at least one element.
#[derive(Debug, Clone, Copy)]
pub struct VecNonEmpty<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> VecNonEmpty<T> {
    /// Create new VecNonEmpty contract.
    pub const fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T> Default for VecNonEmpty<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Contract for VecNonEmpty<T>
where
    T: crate::traits::Elicitation + Clone + std::fmt::Debug + Send,
{
    type Input = Vec<T>;
    type Output = Vec<T>;

    fn requires(input: &Vec<T>) -> bool {
        !input.is_empty()
    }

    fn ensures(_input: &Vec<T>, output: &Vec<T>) -> bool {
        !output.is_empty()
    }
}

/// Contract for Vec<T> with maximum length.
///
/// Verifies that a vector does not exceed a maximum length.
#[derive(Debug, Clone, Copy)]
pub struct VecMaxLength<T, const MAX: usize> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T, const MAX: usize> VecMaxLength<T, MAX> {
    /// Create new VecMaxLength contract.
    pub const fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T, const MAX: usize> Default for VecMaxLength<T, MAX> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const MAX: usize> Contract for VecMaxLength<T, MAX>
where
    T: crate::traits::Elicitation + Clone + std::fmt::Debug + Send,
{
    type Input = Vec<T>;
    type Output = Vec<T>;

    fn requires(input: &Vec<T>) -> bool {
        input.len() <= MAX
    }

    fn ensures(_input: &Vec<T>, output: &Vec<T>) -> bool {
        output.len() <= MAX
    }
}

/// Contract for Vec<T> with element contract.
///
/// Verifies that all elements in the vector satisfy a contract.
#[derive(Debug, Clone, Copy)]
pub struct VecAllElements<T, C> {
    element_contract: C,
    _phantom: std::marker::PhantomData<T>,
}

impl<T, C> VecAllElements<T, C> {
    /// Create new VecAllElements contract with specified element contract.
    pub const fn new(element_contract: C) -> Self {
        Self {
            element_contract,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T, C> Contract for VecAllElements<T, C>
where
    T: crate::traits::Elicitation + Clone + std::fmt::Debug + Send,
    C: Contract<Input = T, Output = T>,
{
    type Input = Vec<T>;
    type Output = Vec<T>;

    fn requires(input: &Vec<T>) -> bool {
        input.iter().all(|elem| C::requires(elem))
    }

    fn ensures(_input: &Vec<T>, output: &Vec<T>) -> bool {
        output.iter().all(|elem| C::ensures(elem, elem))
    }

    fn invariant(&self) -> bool {
        self.element_contract.invariant()
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

    // Signed integer tests
    #[test]
    fn test_i64_positive() {
        assert!(I64Positive::requires(&42i64));
        assert!(I64Positive::ensures(&42i64, &42i64));
        assert!(!I64Positive::requires(&0i64));
        assert!(!I64Positive::requires(&-1i64));
    }

    #[test]
    fn test_i128_positive() {
        assert!(I128Positive::requires(&42i128));
        assert!(I128Positive::ensures(&42i128, &42i128));
        assert!(!I128Positive::requires(&0i128));
        assert!(!I128Positive::requires(&-1i128));
    }

    #[test]
    fn test_isize_positive() {
        assert!(IsizePositive::requires(&42isize));
        assert!(IsizePositive::ensures(&42isize, &42isize));
        assert!(!IsizePositive::requires(&0isize));
        assert!(!IsizePositive::requires(&-1isize));
    }

    // Floating point tests
    #[test]
    fn test_f32_finite() {
        assert!(F32Finite::requires(&42.0f32));
        assert!(F32Finite::requires(&0.0f32));
        assert!(F32Finite::requires(&-1.5f32));
        assert!(!F32Finite::requires(&f32::NAN));
        assert!(!F32Finite::requires(&f32::INFINITY));
        assert!(!F32Finite::requires(&f32::NEG_INFINITY));
    }

    #[test]
    fn test_f64_finite() {
        assert!(F64Finite::requires(&42.0f64));
        assert!(F64Finite::requires(&0.0f64));
        assert!(F64Finite::requires(&-1.5f64));
        assert!(!F64Finite::requires(&f64::NAN));
        assert!(!F64Finite::requires(&f64::INFINITY));
        assert!(!F64Finite::requires(&f64::NEG_INFINITY));
    }

    // Option<T> tests
    #[test]
    fn test_option_is_some() {
        let some_value: Option<i32> = Some(42);
        let none_value: Option<i32> = None;

        assert!(OptionIsSome::<i32>::requires(&some_value));
        assert!(OptionIsSome::<i32>::ensures(&some_value, &some_value));
        assert!(!OptionIsSome::<i32>::requires(&none_value));
    }

    #[test]
    fn test_option_with_inner_contract() {
        let positive: Option<i32> = Some(42);
        let negative: Option<i32> = Some(-1);
        let zero: Option<i32> = Some(0);
        let none: Option<i32> = None;

        let contract = OptionWithInner::new(I32Positive);

        assert!(OptionWithInner::<i32, I32Positive>::requires(&positive));
        assert!(!OptionWithInner::<i32, I32Positive>::requires(&negative));
        assert!(!OptionWithInner::<i32, I32Positive>::requires(&zero));
        assert!(!OptionWithInner::<i32, I32Positive>::requires(&none));

        assert!(OptionWithInner::<i32, I32Positive>::ensures(
            &positive, &positive
        ));
        assert!(contract.invariant());
    }

    // Result<T, E> tests
    #[test]
    fn test_result_is_ok() {
        let ok_value: Result<i32, String> = Ok(42);
        let err_value: Result<i32, String> = Err("error".to_string());

        assert!(ResultIsOk::<i32, String>::requires(&ok_value));
        assert!(ResultIsOk::<i32, String>::ensures(&ok_value, &ok_value));
        assert!(!ResultIsOk::<i32, String>::requires(&err_value));
    }

    #[test]
    fn test_result_with_ok_contract() {
        let positive: Result<i32, String> = Ok(42);
        let negative: Result<i32, String> = Ok(-1);
        let error: Result<i32, String> = Err("error".to_string());

        let contract: ResultWithOk<i32, String, I32Positive> = ResultWithOk::new(I32Positive);

        assert!(ResultWithOk::<i32, String, I32Positive>::requires(
            &positive
        ));
        assert!(!ResultWithOk::<i32, String, I32Positive>::requires(
            &negative
        ));
        assert!(!ResultWithOk::<i32, String, I32Positive>::requires(&error));

        assert!(ResultWithOk::<i32, String, I32Positive>::ensures(
            &positive, &positive
        ));
        assert!(contract.invariant());
    }

    // Vec<T> tests
    #[test]
    fn test_vec_non_empty() {
        let non_empty: Vec<i32> = vec![1, 2, 3];
        let empty: Vec<i32> = vec![];

        assert!(VecNonEmpty::<i32>::requires(&non_empty));
        assert!(VecNonEmpty::<i32>::ensures(&non_empty, &non_empty));
        assert!(!VecNonEmpty::<i32>::requires(&empty));
    }

    #[test]
    fn test_vec_max_length() {
        let short: Vec<i32> = vec![1, 2, 3];
        let exact: Vec<i32> = vec![1, 2, 3, 4, 5];
        let long: Vec<i32> = vec![1, 2, 3, 4, 5, 6];

        assert!(VecMaxLength::<i32, 5>::requires(&short));
        assert!(VecMaxLength::<i32, 5>::requires(&exact));
        assert!(!VecMaxLength::<i32, 5>::requires(&long));

        assert!(VecMaxLength::<i32, 5>::ensures(&short, &short));
    }

    #[test]
    fn test_vec_all_elements() {
        let all_positive: Vec<i32> = vec![1, 2, 3];
        let has_negative: Vec<i32> = vec![1, -2, 3];
        let empty: Vec<i32> = vec![];

        let contract = VecAllElements::new(I32Positive);

        assert!(VecAllElements::<i32, I32Positive>::requires(&all_positive));
        assert!(!VecAllElements::<i32, I32Positive>::requires(&has_negative));
        assert!(VecAllElements::<i32, I32Positive>::requires(&empty)); // Vacuously true

        assert!(VecAllElements::<i32, I32Positive>::ensures(
            &all_positive,
            &all_positive
        ));
        assert!(contract.invariant());
    }
}
