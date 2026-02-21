//! Verus proofs for tuple contract types.
//!
//! Tuples demonstrate compositional verification - if all elements
//! are validated contract types, the tuple is automatically valid.

use verus_builtin::*;
use verus_builtin_macros::*;
use vstd::prelude::*;

verus! {

// ============================================================================
// Tuple2 - 2-element tuple
// ============================================================================

/// A 2-element tuple composition.
///
/// For Verus, we track that both elements have been validated.
/// Compositional property: if C1 and C2 are valid, Tuple2<C1, C2> is valid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tuple2 {
    pub first_validated: bool,
    pub second_validated: bool,
}

impl Tuple2 {
    /// Creates a Tuple2 given validation results for both elements.
    ///
    /// Both elements must be validated for tuple to be valid.
    pub fn new(first_valid: bool, second_valid: bool) -> (result: Self)
        ensures
            result.first_validated == first_valid,
            result.second_validated == second_valid,
    {
        Tuple2 {
            first_validated: first_valid,
            second_validated: second_valid,
        }
    }

    /// Check if both elements are validated.
    pub fn is_validated(&self) -> (result: bool)
        ensures result == (self.first_validated && self.second_validated),
    {
        self.first_validated && self.second_validated
    }
}

// ============================================================================
// Tuple3 - 3-element tuple
// ============================================================================

/// A 3-element tuple composition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tuple3 {
    pub first_validated: bool,
    pub second_validated: bool,
    pub third_validated: bool,
}

impl Tuple3 {
    /// Creates a Tuple3 given validation results for all elements.
    pub fn new(first_valid: bool, second_valid: bool, third_valid: bool) -> (result: Self)
        ensures
            result.first_validated == first_valid,
            result.second_validated == second_valid,
            result.third_validated == third_valid,
    {
        Tuple3 {
            first_validated: first_valid,
            second_validated: second_valid,
            third_validated: third_valid,
        }
    }

    /// Check if all elements are validated.
    pub fn is_validated(&self) -> (result: bool)
        ensures result == (self.first_validated && self.second_validated && self.third_validated),
    {
        self.first_validated && self.second_validated && self.third_validated
    }
}

// ============================================================================
// Tuple4 - 4-element tuple
// ============================================================================

/// A 4-element tuple composition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tuple4 {
    pub first_validated: bool,
    pub second_validated: bool,
    pub third_validated: bool,
    pub fourth_validated: bool,
}

impl Tuple4 {
    /// Creates a Tuple4 given validation results for all elements.
    pub fn new(
        first_valid: bool,
        second_valid: bool,
        third_valid: bool,
        fourth_valid: bool,
    ) -> (result: Self)
        ensures
            result.first_validated == first_valid,
            result.second_validated == second_valid,
            result.third_validated == third_valid,
            result.fourth_validated == fourth_valid,
    {
        Tuple4 {
            first_validated: first_valid,
            second_validated: second_valid,
            third_validated: third_valid,
            fourth_validated: fourth_valid,
        }
    }

    /// Check if all elements are validated.
    pub fn is_validated(&self) -> (result: bool)
        ensures result == (
            self.first_validated &&
            self.second_validated &&
            self.third_validated &&
            self.fourth_validated
        ),
    {
        self.first_validated &&
        self.second_validated &&
        self.third_validated &&
        self.fourth_validated
    }
}

} // verus!
