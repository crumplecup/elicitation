//! Formal verification framework for tool chains.
//!
//! Provides a generic `Contract` trait that can be implemented by multiple
//! verification tools (Kani, Creusot, Prusti, Verus, etc).
//!
//! # Architecture
//!
//! - **Core**: `Contract` trait - tool-agnostic interface
//! - **Adapters**: Tool-specific implementations (feature-gated)
//! - **Registry**: Unified interface for swapping verifiers at runtime
//! - **Examples**: Demonstration of verification with different tools
//!
//! # Feature Flags
//!
//! - `verification` - Core trait only
//! - `verify-kani` - Kani model checker support
//! - `verify-creusot` - Creusot deductive verifier
//! - `verify-prusti` - Prusti separation logic
//! - `verify-verus` - Verus SMT-based verifier
//!
//! # Contract Swapping
//!
//! Users can swap verification backends at compile-time (via features) or
//! runtime (via `VerifierBackend` enum):
//!
//! ```rust,ignore
//! use elicitation::verification::{VerifierBackend, contracts::*};
//!
//! // Compile-time: Choose via feature flag
//! // cargo build --features verify-kani
//!
//! // Runtime: Choose via enum
//! let verifier = VerifierBackend::Kani(Box::new(StringNonEmpty));
//! assert!(verifier.check_precondition(&String::from("hello")));
//! ```
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
use crate::ElicitResult;
use std::fmt::Debug;

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
// Verifier Backend Registry
// ============================================================================

/// Runtime-friendly contract wrapper for dynamic dispatch.
///
/// Since `Contract` has static methods, we need a trait object compatible version.
pub trait DynContract<I, O>: Send + Sync
where
    I: Elicitation + Clone + Debug + Send,
    O: Elicitation + Clone + Debug + Send,
{
    /// Check precondition.
    fn check_requires(&self, input: &I) -> bool;

    /// Check postcondition.
    fn check_ensures(&self, input: &I, output: &O) -> bool;

    /// Check invariant.
    fn check_invariant(&self) -> bool;
}

/// Blanket implementation to convert any Contract into DynContract.
impl<T, I, O> DynContract<I, O> for T
where
    T: Contract<Input = I, Output = O> + Send + Sync,
    I: Elicitation + Clone + Debug + Send,
    O: Elicitation + Clone + Debug + Send,
{
    fn check_requires(&self, input: &I) -> bool {
        Self::requires(input)
    }

    fn check_ensures(&self, input: &I, output: &O) -> bool {
        Self::ensures(input, output)
    }

    fn check_invariant(&self) -> bool {
        self.invariant()
    }
}

/// Unified interface for swapping verification backends at runtime.
///
/// This enum allows users to choose different verifiers for different types
/// or situations, supporting the refinement workflow.
///
/// # Example
///
/// ```rust,ignore
/// use elicitation::verification::{VerifierBackend, contracts::*};
///
/// // Start with Kani for quick checks
/// let verifier = VerifierBackend::Kani(Box::new(StringNonEmpty));
///
/// // Refine to Creusot for stronger guarantees
/// let verifier = VerifierBackend::Creusot(Box::new(
///     contracts::creusot::CreusotStringNonEmpty
/// ));
/// ```
pub enum VerifierBackend<I, O>
where
    I: Elicitation + Clone + Debug + Send,
    O: Elicitation + Clone + Debug + Send,
{
    /// Kani model checker (bounded verification)
    Kani(Box<dyn DynContract<I, O>>),

    /// Creusot deductive verifier (unbounded proofs)
    #[cfg(feature = "verify-creusot")]
    Creusot(Box<dyn DynContract<I, O>>),

    /// Prusti separation logic verifier
    #[cfg(feature = "verify-prusti")]
    Prusti(Box<dyn DynContract<I, O>>),

    /// Verus SMT-based verifier
    #[cfg(feature = "verify-verus")]
    Verus(Box<dyn DynContract<I, O>>),
}

impl<I, O> VerifierBackend<I, O>
where
    I: Elicitation + Clone + Debug + Send,
    O: Elicitation + Clone + Debug + Send,
{
    /// Check if precondition holds for input.
    pub fn check_precondition(&self, input: &I) -> bool {
        match self {
            Self::Kani(contract) => contract.check_requires(input),

            #[cfg(feature = "verify-creusot")]
            Self::Creusot(contract) => contract.check_requires(input),

            #[cfg(feature = "verify-prusti")]
            Self::Prusti(contract) => contract.check_requires(input),

            #[cfg(feature = "verify-verus")]
            Self::Verus(contract) => contract.check_requires(input),
        }
    }

    /// Check if postcondition holds for input/output pair.
    pub fn check_postcondition(&self, input: &I, output: &O) -> bool {
        match self {
            Self::Kani(contract) => contract.check_ensures(input, output),

            #[cfg(feature = "verify-creusot")]
            Self::Creusot(contract) => contract.check_ensures(input, output),

            #[cfg(feature = "verify-prusti")]
            Self::Prusti(contract) => contract.check_ensures(input, output),

            #[cfg(feature = "verify-verus")]
            Self::Verus(contract) => contract.check_ensures(input, output),
        }
    }

    /// Check if invariant holds.
    pub fn check_invariant(&self) -> bool {
        match self {
            Self::Kani(contract) => contract.check_invariant(),

            #[cfg(feature = "verify-creusot")]
            Self::Creusot(contract) => contract.check_invariant(),

            #[cfg(feature = "verify-prusti")]
            Self::Prusti(contract) => contract.check_invariant(),

            #[cfg(feature = "verify-verus")]
            Self::Verus(contract) => contract.check_invariant(),
        }
    }

    /// Verify a transformation: check precondition, apply function, check postcondition.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let verifier = VerifierBackend::Kani(Box::new(I32Positive));
    /// let result = verifier.verify(42, |x| x)?;
    /// assert_eq!(result, 42);
    /// ```
    pub fn verify<F>(&self, input: I, f: F) -> Result<O, String>
    where
        F: FnOnce(I) -> O,
    {
        // Check precondition
        if !self.check_precondition(&input) {
            return Err("Precondition failed".to_string());
        }

        // Check invariant before
        if !self.check_invariant() {
            return Err("Invariant failed before execution".to_string());
        }

        // Execute transformation
        let output = f(input.clone());

        // Check postcondition
        if !self.check_postcondition(&input, &output) {
            return Err("Postcondition failed".to_string());
        }

        // Check invariant after
        if !self.check_invariant() {
            return Err("Invariant failed after execution".to_string());
        }

        Ok(output)
    }
}

// ============================================================================
// Contract Integration with Elicitation
// ============================================================================

/// Builder for elicitation with contract verification.
///
/// Enables ergonomic syntax: `String::with_contract(StringNonEmpty).elicit(&peer).await?`
pub struct ContractedElicitation<T, C>
where
    T: Elicitation,
    C: Contract<Input = T, Output = T>,
{
    contract: C,
    _phantom: std::marker::PhantomData<T>,
}

impl<T, C> ContractedElicitation<T, C>
where
    T: Elicitation + Clone + Debug + Send,
    C: Contract<Input = T, Output = T> + Send + Sync + 'static,
{
    /// Create a new contracted elicitation builder.
    pub fn new(contract: C) -> Self {
        Self {
            contract,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Elicit the value and verify contracts.
    ///
    /// This method:
    /// 1. Elicits the value using standard elicitation
    /// 2. Treats elicited value as both input and output (identity transformation)
    /// 3. Verifies precondition on elicited value
    /// 4. Verifies postcondition (since input == output for elicitation)
    /// 5. Returns value if contracts hold
    ///
    /// # Arguments
    ///
    /// * `peer` - The RMCP peer to use for interaction
    ///
    /// # Returns
    ///
    /// Returns the elicited value if contracts are satisfied.
    ///
    /// # Errors
    ///
    /// - Contract violation: Precondition or postcondition failed
    /// - Elicitation error: Standard elicitation failures
    pub async fn elicit(
        self,
        peer: &crate::rmcp::Peer<crate::rmcp::RoleClient>,
    ) -> ElicitResult<T> {
        use tracing::{debug, error};

        // Elicit the value
        let client = crate::ElicitClient::new(peer);
        let value = T::elicit(&client).await?;

        debug!(
            value = ?value,
            "Value elicited, checking contracts"
        );

        // Check precondition (value must satisfy entry condition)
        if !C::requires(&value) {
            error!("Contract precondition failed");
            return Err(crate::ElicitError::new(
                crate::ElicitErrorKind::InvalidFormat {
                    expected: "Value satisfying contract precondition".to_string(),
                    received: format!("{:?}", value),
                },
            ));
        }

        // For elicitation, input == output (identity)
        // Check postcondition
        if !C::ensures(&value, &value) {
            error!("Contract postcondition failed");
            return Err(crate::ElicitError::new(
                crate::ElicitErrorKind::InvalidFormat {
                    expected: "Value satisfying contract postcondition".to_string(),
                    received: format!("{:?}", value),
                },
            ));
        }

        // Check invariant
        if !self.contract.invariant() {
            error!("Contract invariant failed");
            return Err(crate::ElicitError::new(
                crate::ElicitErrorKind::InvalidFormat {
                    expected: "Contract invariant to hold".to_string(),
                    received: "Invariant violated".to_string(),
                },
            ));
        }

        debug!("All contracts satisfied");
        Ok(value)
    }
}

/// Extension trait for adding contract verification to elicitation.
///
/// This trait is automatically implemented for all types that implement
/// `Elicitation`, allowing contract-verified elicitation.
///
/// # Example
///
/// ```rust,ignore
/// use elicitation::verification::{WithContract, contracts::StringNonEmpty};
///
/// // Elicit with contract verification
/// let value = String::with_contract(StringNonEmpty)
///     .elicit(&peer)
///     .await?;
///
/// // Value is guaranteed to satisfy StringNonEmpty contract
/// assert!(!value.is_empty());
/// ```
pub trait WithContract: Elicitation + Clone + Debug + Send {
    /// Attach a contract to this type's elicitation.
    ///
    /// Returns a builder that will verify the contract after elicitation.
    ///
    /// # Arguments
    ///
    /// * `contract` - The contract to verify
    ///
    /// # Returns
    ///
    /// A builder that can elicit and verify the value.
    fn with_contract<C>(contract: C) -> ContractedElicitation<Self, C>
    where
        C: Contract<Input = Self, Output = Self> + Send + Sync + 'static,
    {
        ContractedElicitation::new(contract)
    }
}

// Blanket implementation for all Elicitation types
impl<T> WithContract for T where T: Elicitation + Clone + Debug + Send {}

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

// TODO: Phase 2 - These will be top-level modules when we add Tool trait impls
// For now, verifier-specific contracts are in contracts/ submodules
// #[cfg(feature = "verify-prusti")]
// pub mod prusti;

// #[cfg(feature = "verify-verus")]
// pub mod verus;

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::verification::contracts::{StringNonEmpty, I32Positive, BoolValid};

    #[test]
    fn test_verifier_backend_string_kani() {
        let verifier = VerifierBackend::Kani(Box::new(StringNonEmpty));
        let input = String::from("hello");

        assert!(verifier.check_precondition(&input));
        assert!(verifier.check_postcondition(&input, &input));
        assert!(verifier.check_invariant());

        let result = verifier.verify(input.clone(), |x| x);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "hello");
    }

    #[test]
    fn test_verifier_backend_i32_kani() {
        let verifier = VerifierBackend::Kani(Box::new(I32Positive));
        let input = 42i32;

        assert!(verifier.check_precondition(&input));
        assert!(verifier.check_postcondition(&input, &input));
        assert!(verifier.check_invariant());

        let result = verifier.verify(input, |x| x);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_verifier_backend_bool_kani() {
        let verifier = VerifierBackend::Kani(Box::new(BoolValid));
        let input = true;

        assert!(verifier.check_precondition(&input));
        assert!(verifier.check_postcondition(&input, &input));
        assert!(verifier.check_invariant());

        let result = verifier.verify(input, |x| x);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_verifier_backend_precondition_failure() {
        let verifier = VerifierBackend::Kani(Box::new(StringNonEmpty));
        let input = String::new(); // Empty string violates precondition

        assert!(!verifier.check_precondition(&input));

        let result = verifier.verify(input, |x| x);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Precondition failed");
    }

    #[test]
    fn test_verifier_backend_postcondition_failure() {
        let verifier = VerifierBackend::Kani(Box::new(I32Positive));
        let input = 42i32;

        // Transform that violates postcondition
        let result = verifier.verify(input, |_x| -1); // Returns negative
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Postcondition failed");
    }

    #[test]
    fn test_with_contract_creates_contracted_elicitation() {
        // Test that with_contract() creates a ContractedElicitation
        let contracted = String::with_contract(StringNonEmpty);
        
        // Verify the type is correct (compile-time check)
        let _: ContractedElicitation<String, StringNonEmpty> = contracted;
    }

    #[test]
    fn test_contracted_elicitation_builder_pattern() {
        // Test builder pattern construction
        let _contracted = ContractedElicitation {
            _phantom: std::marker::PhantomData::<i32>,
            contract: I32Positive,
        };

        // Verify contract is stored correctly
        assert!(I32Positive::requires(&42));
        assert!(!I32Positive::requires(&-1));
    }

    #[test]
    fn test_with_contract_works_for_all_elicitation_types() {
        // Test that with_contract() is available for all types that impl Elicitation
        let _string_contracted = String::with_contract(StringNonEmpty);
        let _i32_contracted = i32::with_contract(I32Positive);
        let _bool_contracted = bool::with_contract(BoolValid);
    }
}
