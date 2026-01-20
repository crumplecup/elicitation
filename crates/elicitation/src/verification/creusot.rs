//! Creusot deductive verifier adapter for formal verification.
//!
//! This module adapts the generic [`Contract`](super::Contract) trait for use
//! with the [Creusot Rust Verifier](https://github.com/creusot-rs/creusot).
//!
//! # Creusot Integration
//!
//! Creusot uses **deductive verification** through Why3:
//! - Translates Rust + contracts → Why3 intermediate language
//! - SMT solvers prove verification conditions
//! - Prophecy variables for reasoning about mutable borrows
//! - Supports both safe and unsafe code verification
//!
//! # Contract Syntax
//!
//! Creusot uses the `creusot-contracts` crate with attribute macros:
//!
//! ```rust,ignore
//! use creusot_contracts::*;
//!
//! #[requires(x > 0)]
//! #[ensures(result > x)]
//! fn increment(x: i32) -> i32 {
//!     x + 1
//! }
//! ```
//!
//! # Prophecy Variables
//!
//! For mutable borrows, use `old()` to refer to pre-state:
//!
//! ```rust,ignore
//! #[requires(x.len() > 0)]
//! #[ensures(x[0] == old(x[0]) + 1)]
//! fn increment_first(x: &mut [i32]) {
//!     x[0] += 1;
//! }
//! ```
//!
//! # Integration with Generic Contract
//!
//! This module provides helpers to bridge our generic [`Contract`] trait
//! to Creusot's native annotation system.
//!
//! ```rust,ignore
//! use elicitation::verification::{Contract, creusot::Tool};
//!
//! struct ValidateEmail;
//!
//! impl Contract for ValidateEmail {
//!     type Input = String;
//!     type Output = String;
//!     
//!     fn requires(input: &String) -> bool {
//!         input.contains('@')
//!     }
//!     
//!     fn ensures(_input: &String, output: &String) -> bool {
//!         output.contains('@')
//!     }
//! }
//!
//! #[async_trait::async_trait]
//! impl Tool for ValidateEmail {
//!     #[requires(Self::requires(&input))]
//!     #[ensures(Self::ensures(&input, &result))]
//!     async fn execute(&self, input: String) -> ElicitResult<String> {
//!         // Implementation
//!         Ok(input)
//!     }
//! }
//! ```
//!
//! # Key Features
//!
//! - **Deductive Verification**: Proves correctness via Why3/SMT solvers
//! - **Prophecy Variables**: Handles mutable references elegantly
//! - **Ghost Code**: Specification-only constructs erased at compile time
//! - **Pearlite Language**: Rust-inspired specification language
//! - **Trait Verification**: Can verify trait method contracts
//!
//! # Installation
//!
//! Creusot requires external tools:
//!
//! ```bash
//! # Install Creusot
//! cargo install creusot
//!
//! # Install Why3 (required backend)
//! # See: https://why3.lri.fr/download.html
//! ```
//!
//! # Running Verification
//!
//! ```bash
//! # Verify a file
//! creusot verify my_code.rs
//!
//! # With Why3 GUI for interactive proof
//! creusot verify --why3-gui my_code.rs
//! ```

use super::Contract;
use crate::ElicitResult;

// Re-export Contract as CreusotContract for consistency
pub use super::Contract as CreusotContract;

/// Executable tool with Creusot contract verification.
///
/// Extends [`Contract`] with async execution and Creusot-compatible annotations.
///
/// # Usage
///
/// Use Creusot's native `#[requires]` and `#[ensures]` attributes on the
/// `execute` method, referencing the `Contract` trait methods:
///
/// ```rust,ignore
/// #[async_trait::async_trait]
/// impl Tool for MyTool {
///     #[requires(Self::requires(&input))]
///     #[ensures(Self::ensures(&old(input), &result))]
///     async fn execute(&self, input: Self::Input) -> ElicitResult<Self::Output> {
///         // Implementation
///     }
/// }
/// ```
///
/// # Prophecy Example
///
/// For mutable state, use `old()` prophecy:
///
/// ```rust,ignore
/// #[requires(self.counter < 100)]
/// #[ensures(self.counter == old(self.counter) + 1)]
/// fn increment(&mut self) {
///     self.counter += 1;
/// }
/// ```
#[async_trait::async_trait]
pub trait Tool: Contract + Sync {
    /// Execute the tool with Creusot verification.
    ///
    /// Annotate this method with `#[requires]` and `#[ensures]` attributes
    /// from `creusot-contracts` crate.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use creusot_contracts::*;
    ///
    /// #[requires(Self::requires(&input))]
    /// #[ensures(Self::ensures(&old(input), &result))]
    /// async fn execute(&self, input: Self::Input) -> ElicitResult<Self::Output> {
    ///     // Creusot verifies contracts via Why3
    /// }
    /// ```
    async fn execute(&self, input: Self::Input) -> ElicitResult<Self::Output>;
}

/// Helper macros for bridging to Creusot annotations.
///
/// These provide ergonomic ways to connect generic `Contract` trait
/// to Creusot's native annotation system.

/// Generate Creusot `#[requires]` from Contract trait.
///
/// # Example
///
/// ```rust,ignore
/// #[requires_from_contract(ValidateEmail)]
/// async fn execute(&self, input: String) -> ElicitResult<String> {
///     // Expands to: #[requires(ValidateEmail::requires(&input))]
/// }
/// ```
#[macro_export]
macro_rules! requires_from_contract {
    ($contract:ty) => {
        #[requires(<$contract>::requires(&input))]
    };
}

/// Generate Creusot `#[ensures]` from Contract trait.
///
/// # Example
///
/// ```rust,ignore
/// #[ensures_from_contract(ValidateEmail)]
/// async fn execute(&self, input: String) -> ElicitResult<String> {
///     // Expands to: #[ensures(ValidateEmail::ensures(&old(input), &result))]
/// }
/// ```
#[macro_export]
macro_rules! ensures_from_contract {
    ($contract:ty) => {
        #[ensures(<$contract>::ensures(&old(input), &result))]
    };
}

/// Documentation-only marker for Creusot ghost code.
///
/// In actual Creusot verification, you would use:
/// ```rust,ignore
/// use creusot_contracts::*;
/// ghost! {
///     let previous_state = input.clone();
///     // Ghost code here
/// }
/// ```
pub struct Ghost;

/// Documentation-only marker for Creusot prophecy variables.
///
/// In actual Creusot verification, prophecies are expressed via `old()`:
/// ```rust,ignore
/// #[ensures(output.value == old(input.value) + 1)]
/// ```
pub struct Prophecy;

/// Helper trait for types that can be used in Creusot specifications.
///
/// Creusot requires types in specifications to be clonable and have
/// decidable equality for verification.
pub trait CreusotSpecifiable: Clone + PartialEq {}

// Blanket implementation for types that meet requirements
impl<T: Clone + PartialEq> CreusotSpecifiable for T {}
