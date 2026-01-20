//! Formal verification framework for tool chains.
//!
//! Provides a generic `Contract` trait that can be implemented by multiple
//! verification tools (Kani, Creusot, Prusti, Verus, etc).
//!
//! # Architecture
//!
//! - **Core**: `Contract` trait - tool-agnostic interface
//! - **Adapters**: Tool-specific implementations (feature-gated)
//! - **Examples**: Demonstration of verification with different tools
//!
//! # Feature Flags
//!
//! - `verification` - Core trait only
//! - `verify-kani` - Kani model checker support
//! - `verify-creusot` - Creusot deductive verifier (future)
//! - `verify-prusti` - Prusti separation logic (future)
//! - `verify-verus` - Verus SMT-based verifier (future)
//!
//! # Example
//!
//! ```rust,ignore
//! use elicitation::verification::Contract;
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
//! ```

use crate::traits::Elicitation;

/// Generic contract for formal verification.
///
/// This trait defines the interface for specifying contracts that can be
/// verified by various formal verification tools (Kani, Creusot, etc).
///
/// # Contract Semantics
///
/// - **Precondition** (`requires`): Must hold before execution
/// - **Postcondition** (`ensures`): Must hold after execution
/// - **Invariant**: Must hold throughout execution
///
/// # Tool Integration
///
/// Verification tools provide their own extensions:
/// - Kani: `#[kani::proof]` harnesses
/// - Creusot: Prophecy annotations
/// - Prusti: Separation logic specs
/// - Verus: Mode-based specifications
///
/// # Design Philosophy
///
/// 1. Tool-agnostic core trait
/// 2. Tool-specific adapters (feature-gated)
/// 3. Composable contracts
/// 4. Elicitation-compatible types
pub trait Contract {
    /// Input type (must be elicitable).
    type Input: Elicitation + Clone + std::fmt::Debug + Send;

    /// Output type (must be elicitable).
    type Output: Elicitation + Clone + std::fmt::Debug + Send;

    /// Precondition: What the tool requires to execute safely.
    ///
    /// This predicate must be **decidable** (finite execution) and **pure**
    /// (no side effects).
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// fn requires(input: &String) -> bool {
    ///     !input.is_empty() && input.len() < 1000
    /// }
    /// ```
    fn requires(input: &Self::Input) -> bool;

    /// Postcondition: What the tool guarantees after execution.
    ///
    /// This predicate relates input to output, expressing the functional
    /// correctness property.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// fn ensures(input: &String, output: &Email) -> bool {
    ///     output.to_string() == *input && output.is_valid()
    /// }
    /// ```
    fn ensures(input: &Self::Input, output: &Self::Output) -> bool;

    /// Invariant: What remains true throughout execution.
    ///
    /// Default implementation allows all states. Override for stateful tools.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// fn invariant(&self) -> bool {
    ///     self.connection.is_alive()
    /// }
    /// ```
    fn invariant(&self) -> bool {
        true
    }
}

// ============================================================================
// Submodules
// ============================================================================

// Default contract implementations for primitive types
pub mod contracts;

// Tool-specific adapters (feature-gated)
#[cfg(feature = "verify-kani")]
pub mod kani;

#[cfg(feature = "verify-creusot")]
pub mod creusot;

#[cfg(feature = "verify-prusti")]
pub mod prusti;

#[cfg(feature = "verify-verus")]
pub mod verus;
