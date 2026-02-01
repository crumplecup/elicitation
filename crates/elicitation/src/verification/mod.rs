//! Formal verification framework for tool chains.
//!
//! Provides a generic `Contract` trait that can be implemented by multiple
//! verification tools (Kani, Creusot, Prusti, Verus, etc).
//!
//! # Quick Start
//!
//! ```rust,ignore
//! use elicitation::verification::{
//!     contracts::StringNonEmpty,
//!     WithContract,
//! };
//!
//! // Use with_contract() to verify elicited values
//! let value = String::with_contract(StringNonEmpty)
//!     .elicit(peer)
//!     .await?;
//! // value is guaranteed to be non-empty
//! ```
//!
//! # Architecture
//!
//! - **Core**: `Contract` trait - tool-agnostic interface
//! - **Adapters**: Tool-specific implementations (feature-gated)
//! - **Registry**: Unified interface for swapping verifiers at runtime
//! - **Composition**: Combine contracts with AND/OR/NOT logic
//! - **Examples**: Demonstration of verification with different tools
//!
//! # Feature Flags
//!
//! - `verification` - Core trait only (Kani contracts by default)
//! - `verify-kani` - Kani model checker support (default for verification)
//! - `verify-creusot` - Creusot deductive verifier
//! - `verify-prusti` - Prusti separation logic
//! - `verify-verus` - Verus SMT-based verifier
//!
//! # Contract Swapping
//!
//! ## Compile-Time (via features)
//!
//! Choose your verifier backend at compile-time:
//!
//! ```bash
//! # Use Kani (default)
//! cargo build --features verification
//!
//! # Use Creusot
//! cargo build --features verify-creusot
//!
//! # Use Prusti
//! cargo build --features verify-prusti
//!
//! # Use Verus
//! cargo build --features verify-verus
//! ```
//!
//! Then use default contracts in your code:
//!
//! ```rust,ignore
//! use elicitation::verification::DEFAULT_STRING_CONTRACT;
//!
//! let value = String::with_contract(DEFAULT_STRING_CONTRACT)
//!     .elicit(peer)
//!     .await?;
//! ```
//!
//! ## Runtime (via VerifierBackend)
//!
//! Swap verifiers dynamically at runtime:
//!
//! ```rust,ignore
//! use elicitation::verification::{VerifierBackend, contracts::*};
//!
//! // Choose backend at runtime
//! let verifier = VerifierBackend::Kani(Box::new(StringNonEmpty));
//! assert!(verifier.check_precondition(&String::from("hello")));
//!
//! // Verify a transformation
//! let result = verifier.verify(input, |x| x.to_uppercase())?;
//! ```
//!
//! # Contract Composition
//!
//! Combine contracts using boolean logic:
//!
//! ```rust,ignore
//! use elicitation::verification::{compose, contracts::*};
//!
//! // Non-empty string with max 100 characters
//! let contract = compose::and(
//!     StringNonEmpty,
//!     StringMaxLength::<100>
//! );
//!
//! // Positive OR non-negative (basically >= 0)
//! let contract = compose::or(I32Positive, I32NonNegative);
//!
//! // NOT positive (basically <= 0)
//! let contract = compose::not(I32Positive);
//!
//! // Complex nested composition
//! let contract = compose::or(
//!     compose::and(I32Positive, I32NonNegative),
//!     compose::not(I32Positive)
//! );
//! ```
//!
//! # Creating Custom Contracts
//!
//! Implement the `Contract` trait for your own types:
//!
//! ```rust,ignore
//! use elicitation::verification::Contract;
//!
//! struct ValidEmail;
//!
//! impl Contract for ValidEmail {
//!     type Input = String;
//!     type Output = String;
//!     
//!     fn requires(input: &String) -> bool {
//!         input.contains('@') && input.contains('.')
//!     }
//!     
//!     fn ensures(_input: &String, output: &String) -> bool {
//!         output.contains('@') && output.contains('.')
//!     }
//! }
//!
//! // Use with elicitation
//! let email = String::with_contract(ValidEmail)
//!     .elicit(peer)
//!     .await?;
//! ```
//!
//! # Formal Verification
//!
//! ## Kani (Model Checking)
//!
//! Kani contracts work out-of-the-box. Write harnesses in your tests:
//!
//! ```rust,ignore
//! #[cfg(kani)]
//! #[kani::proof]
//! fn verify_string_non_empty() {
//!     let inputs = ["hello", "world", "test"];
//!     for input in inputs {
//!         assert!(StringNonEmpty::requires(&input.to_string()));
//!     }
//! }
//! ```
//!
//! Run with: `cargo kani`
//!
//! ## Creusot (Deductive Verification)
//!
//! Creusot requires annotated source code:
//!
//! ```rust,ignore
//! #[requires(input.len() > 0)]
//! #[ensures(result.len() > 0)]
//! fn process(input: String) -> String {
//!     input
//! }
//! ```
//!
//! Our runtime contracts provide fallback checking.
//!
//! ## Prusti (Separation Logic)
//!
//! Prusti uses `prusti-contracts`:
//!
//! ```rust,ignore
//! #[requires(input > 0)]
//! #[ensures(result > 0)]
//! fn increment(input: i32) -> i32 {
//!     input + 1
//! }
//! ```
//!
//! ## Verus (SMT-Based)
//!
//! Verus uses modes and proof blocks:
//!
//! ```rust,ignore
//! fn process(input: i32) -> (result: i32)
//!     requires input > 0,
//!     ensures result > 0,
//! {
//!     input + 1
//! }
//! ```
//!
//! # Example
//!
//! See `examples/verification_demo.rs` for a complete demonstration:
//!
//! ```bash
//! cargo run --example verification_demo --features verification
//! ```

use crate::ElicitResult;
use crate::traits::Elicitation;
use std::fmt::Debug;

// ============================================================================
// Default Contract Selection (Feature-gated)
// ============================================================================

/// Default contract instances for primitives, selected at compile-time based on features.
///
/// When verification features are enabled, these constants resolve to the appropriate
/// verifier-specific contract:
/// - `verify-kani` → Kani contracts (default)
/// - `verify-creusot` → Creusot contracts
/// - `verify-prusti` → Prusti contracts
/// - `verify-verus` → Verus contracts
///
/// Usage:
/// ```rust,ignore
/// use elicitation::verification::DEFAULT_STRING_CONTRACT;
///
/// let value = String::with_contract(DEFAULT_STRING_CONTRACT)
///     .elicit(peer)
///     .await?;
/// ```

// String contracts
/// Default String contract (Kani unless overridden by feature).
#[cfg(all(
    feature = "verification",
    not(any(
        feature = "verify-creusot",
        feature = "verify-prusti",
        feature = "verify-verus"
    ))
))]
pub const DEFAULT_STRING_CONTRACT: contracts::StringNonEmpty = contracts::StringNonEmpty;

/// Default String contract (Creusot).
#[cfg(feature = "verify-creusot")]
pub const DEFAULT_STRING_CONTRACT: contracts::creusot::CreusotStringNonEmpty =
    contracts::creusot::CreusotStringNonEmpty;

/// Default String contract (Prusti).
#[cfg(feature = "verify-prusti")]
pub const DEFAULT_STRING_CONTRACT: contracts::prusti::PrustiStringNonEmpty =
    contracts::prusti::PrustiStringNonEmpty;

/// Default String contract (Verus).
#[cfg(feature = "verify-verus")]
pub const DEFAULT_STRING_CONTRACT: contracts::verus::VerusStringNonEmpty =
    contracts::verus::VerusStringNonEmpty;

// i32 contracts
/// Default i32 contract (Kani unless overridden by feature).
#[cfg(all(
    feature = "verification",
    not(any(
        feature = "verify-creusot",
        feature = "verify-prusti",
        feature = "verify-verus"
    ))
))]
pub const DEFAULT_I32_CONTRACT: contracts::I32Positive = contracts::I32Positive;

/// Default i32 contract (Creusot).
#[cfg(feature = "verify-creusot")]
pub const DEFAULT_I32_CONTRACT: contracts::creusot::CreusotI32Positive =
    contracts::creusot::CreusotI32Positive;

/// Default i32 contract (Prusti).
#[cfg(feature = "verify-prusti")]
pub const DEFAULT_I32_CONTRACT: contracts::prusti::PrustiI32Positive =
    contracts::prusti::PrustiI32Positive;

/// Default i32 contract (Verus).
#[cfg(feature = "verify-verus")]
pub const DEFAULT_I32_CONTRACT: contracts::verus::VerusI32Positive =
    contracts::verus::VerusI32Positive;

// bool contracts
/// Default bool contract (Kani unless overridden by feature).
#[cfg(all(
    feature = "verification",
    not(any(
        feature = "verify-creusot",
        feature = "verify-prusti",
        feature = "verify-verus"
    ))
))]
pub const DEFAULT_BOOL_CONTRACT: contracts::BoolValid = contracts::BoolValid;

/// Default bool contract (Creusot).
#[cfg(feature = "verify-creusot")]
pub const DEFAULT_BOOL_CONTRACT: contracts::creusot::CreusotBoolValid =
    contracts::creusot::CreusotBoolValid;

/// Default bool contract (Prusti).
#[cfg(feature = "verify-prusti")]
pub const DEFAULT_BOOL_CONTRACT: contracts::prusti::PrustiBoolValid =
    contracts::prusti::PrustiBoolValid;

/// Default bool contract (Verus).
#[cfg(feature = "verify-verus")]
pub const DEFAULT_BOOL_CONTRACT: contracts::verus::VerusBoolValid =
    contracts::verus::VerusBoolValid;

// ============================================================================
// Contract Trait
// ============================================================================

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
// Contract Composition
// ============================================================================

/// Combines two contracts with AND logic (both must pass).
///
/// # Example
///
/// ```rust,ignore
/// use elicitation::verification::{contracts::*, AndContract};
///
/// // Both contracts must be satisfied
/// let contract = AndContract::new(StringNonEmpty, StringMaxLength(100));
/// assert!(contract.requires(&String::from("hello")));
/// assert!(!contract.requires(&String::new())); // Fails StringNonEmpty
/// ```
#[derive(Debug, Clone)]
pub struct AndContract<C1, C2> {
    /// First contract.
    pub first: C1,
    /// Second contract.
    pub second: C2,
}

impl<C1, C2> AndContract<C1, C2> {
    /// Create new AND contract combining two contracts.
    #[tracing::instrument(level = "trace", skip(first, second))]
    pub fn new(first: C1, second: C2) -> Self {
        Self { first, second }
    }
}

impl<C1, C2, I, O> Contract for AndContract<C1, C2>
where
    C1: Contract<Input = I, Output = O>,
    C2: Contract<Input = I, Output = O>,
    I: Elicitation + Clone + Debug + Send,
    O: Elicitation + Clone + Debug + Send,
{
    type Input = I;
    type Output = O;

    fn requires(input: &Self::Input) -> bool {
        C1::requires(input) && C2::requires(input)
    }

    fn ensures(input: &Self::Input, output: &Self::Output) -> bool {
        C1::ensures(input, output) && C2::ensures(input, output)
    }

    fn invariant(&self) -> bool {
        self.first.invariant() && self.second.invariant()
    }
}

/// Combines two contracts with OR logic (either can pass).
///
/// # Example
///
/// ```rust,ignore
/// use elicitation::verification::{contracts::*, OrContract};
///
/// // Either contract can be satisfied
/// let contract = OrContract::new(I32Positive, I32Zero);
/// assert!(contract.requires(&42));   // Satisfies I32Positive
/// assert!(contract.requires(&0));    // Satisfies I32Zero
/// assert!(!contract.requires(&-1));  // Satisfies neither
/// ```
#[derive(Debug, Clone)]
pub struct OrContract<C1, C2> {
    /// First contract.
    pub first: C1,
    /// Second contract.
    pub second: C2,
}

impl<C1, C2> OrContract<C1, C2> {
    /// Create new OR contract combining two contracts.
    #[tracing::instrument(level = "trace", skip(first, second))]
    pub fn new(first: C1, second: C2) -> Self {
        Self { first, second }
    }
}

impl<C1, C2, I, O> Contract for OrContract<C1, C2>
where
    C1: Contract<Input = I, Output = O>,
    C2: Contract<Input = I, Output = O>,
    I: Elicitation + Clone + Debug + Send,
    O: Elicitation + Clone + Debug + Send,
{
    type Input = I;
    type Output = O;

    fn requires(input: &Self::Input) -> bool {
        C1::requires(input) || C2::requires(input)
    }

    fn ensures(input: &Self::Input, output: &Self::Output) -> bool {
        C1::ensures(input, output) || C2::ensures(input, output)
    }

    fn invariant(&self) -> bool {
        self.first.invariant() || self.second.invariant()
    }
}

/// Negates a contract (logical NOT).
///
/// # Example
///
/// ```rust,ignore
/// use elicitation::verification::{contracts::*, NotContract};
///
/// // Inverts the contract logic
/// let contract = NotContract::new(StringNonEmpty);
/// assert!(!contract.requires(&String::from("hello"))); // Was true, now false
/// assert!(contract.requires(&String::new()));          // Was false, now true
/// ```
#[derive(Debug, Clone)]
pub struct NotContract<C> {
    /// Inner contract to negate.
    pub inner: C,
}

impl<C> NotContract<C> {
    /// Create new NOT contract negating inner contract.
    #[tracing::instrument(level = "trace", skip(inner))]
    pub fn new(inner: C) -> Self {
        Self { inner }
    }
}

impl<C, I, O> Contract for NotContract<C>
where
    C: Contract<Input = I, Output = O>,
    I: Elicitation + Clone + Debug + Send,
    O: Elicitation + Clone + Debug + Send,
{
    type Input = I;
    type Output = O;

    fn requires(input: &Self::Input) -> bool {
        !C::requires(input)
    }

    fn ensures(input: &Self::Input, output: &Self::Output) -> bool {
        !C::ensures(input, output)
    }

    fn invariant(&self) -> bool {
        !self.inner.invariant()
    }
}

/// Helper functions for contract composition.
pub mod compose {
    use super::*;

    /// Create AND contract from two contracts.
    #[tracing::instrument(level = "trace", skip(first, second))]
    pub fn and<C1, C2>(first: C1, second: C2) -> AndContract<C1, C2> {
        AndContract::new(first, second)
    }

    /// Create OR contract from two contracts.
    #[tracing::instrument(level = "trace", skip(first, second))]
    pub fn or<C1, C2>(first: C1, second: C2) -> OrContract<C1, C2> {
        OrContract::new(first, second)
    }

    /// Create NOT contract from a contract.
    #[tracing::instrument(level = "trace", skip(inner))]
    pub fn not<C>(inner: C) -> NotContract<C> {
        NotContract::new(inner)
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

// Mechanism-level contracts (Survey, Affirm, Text, etc)
pub mod mechanisms;

// Contract types for trenchcoat pattern (boundary validation)
pub mod types;

// CLI runner for proof orchestration
#[cfg(feature = "cli")]
pub mod runner;

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
    use crate::verification::contracts::{BoolValid, I32Positive, StringNonEmpty};

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
        assert!(result.unwrap());
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

    #[test]
    #[cfg(feature = "verification")]
    fn test_default_contracts_available() {
        // Test that default contract constants are available when verification feature enabled
        use super::{DEFAULT_BOOL_CONTRACT, DEFAULT_I32_CONTRACT, DEFAULT_STRING_CONTRACT};

        // Compile-time check that these constants exist and are the expected types
        fn check_string_contract<T: Contract<Input = String, Output = String>>(_: T) {}
        fn check_i32_contract<T: Contract<Input = i32, Output = i32>>(_: T) {}
        fn check_bool_contract<T: Contract<Input = bool, Output = bool>>(_: T) {}

        check_string_contract(DEFAULT_STRING_CONTRACT);
        check_i32_contract(DEFAULT_I32_CONTRACT);
        check_bool_contract(DEFAULT_BOOL_CONTRACT);
    }

    #[test]
    #[cfg(feature = "verification")]
    fn test_default_contracts_usable_with_with_contract() {
        // Test that default contracts work with with_contract()
        use super::{DEFAULT_BOOL_CONTRACT, DEFAULT_I32_CONTRACT, DEFAULT_STRING_CONTRACT};

        // These contracts are zero-sized unit structs
        let _string_contracted = String::with_contract(DEFAULT_STRING_CONTRACT);
        let _i32_contracted = i32::with_contract(DEFAULT_I32_CONTRACT);
        let _bool_contracted = bool::with_contract(DEFAULT_BOOL_CONTRACT);
    }

    // ========================================================================
    // Contract Composition Tests
    // ========================================================================

    #[test]
    fn test_and_contract_both_pass() {
        use super::AndContract;
        use super::contracts::{I32NonNegative, I32Positive};

        // 42 is both positive and non-negative
        AndContract::new(I32Positive, I32NonNegative);
        assert!(AndContract::<I32Positive, I32NonNegative>::requires(&42));
        assert!(AndContract::<I32Positive, I32NonNegative>::ensures(
            &42, &42
        ));
    }

    #[test]
    fn test_and_contract_one_fails() {
        use super::AndContract;
        use super::contracts::{I32NonNegative, I32Positive};

        // 0 is non-negative but not positive
        assert!(!AndContract::<I32Positive, I32NonNegative>::requires(&0));

        // -1 fails both
        assert!(!AndContract::<I32Positive, I32NonNegative>::requires(&-1));
    }

    #[test]
    fn test_or_contract_either_passes() {
        use super::OrContract;
        use super::contracts::{I32NonNegative, I32Positive};

        // 42 satisfies both (either is enough)
        assert!(OrContract::<I32Positive, I32NonNegative>::requires(&42));

        // 0 satisfies NonNegative only (still passes)
        assert!(OrContract::<I32Positive, I32NonNegative>::requires(&0));

        // -1 satisfies neither (fails)
        assert!(!OrContract::<I32Positive, I32NonNegative>::requires(&-1));
    }

    #[test]
    fn test_not_contract_inverts_logic() {
        use super::NotContract;
        use super::contracts::I32Positive;

        // Original: 42 is positive
        assert!(I32Positive::requires(&42));

        // Negated: 42 is NOT positive (false)
        assert!(!NotContract::<I32Positive>::requires(&42));

        // Original: -1 is not positive (false)
        assert!(!I32Positive::requires(&-1));

        // Negated: -1 is NOT positive (true)
        assert!(NotContract::<I32Positive>::requires(&-1));
    }

    #[test]
    fn test_string_composition_bounded_non_empty() {
        use super::AndContract;
        use super::contracts::{StringMaxLength, StringNonEmpty};

        // Create contract: non-empty AND max 10 chars
        AndContract::new(StringNonEmpty, StringMaxLength::<10>);

        // "hello" passes (non-empty, 5 chars)
        assert!(AndContract::<StringNonEmpty, StringMaxLength<10>>::requires(&"hello".to_string()));

        // "" fails (empty)
        assert!(!AndContract::<StringNonEmpty, StringMaxLength<10>>::requires(&"".to_string()));

        // "12345678901" fails (too long)
        assert!(
            !AndContract::<StringNonEmpty, StringMaxLength<10>>::requires(
                &"12345678901".to_string()
            )
        );
    }

    #[test]
    fn test_compose_helpers() {
        use super::compose;
        use super::contracts::{I32NonNegative, I32Positive};

        // Test helper functions
        let and_contract = compose::and(I32Positive, I32NonNegative);
        assert!(AndContract::<I32Positive, I32NonNegative>::requires(&42));

        let or_contract = compose::or(I32Positive, I32NonNegative);
        assert!(OrContract::<I32Positive, I32NonNegative>::requires(&0));

        let not_contract = compose::not(I32Positive);
        assert!(NotContract::<I32Positive>::requires(&-1));

        // Verify contracts are constructed correctly
        let _: AndContract<I32Positive, I32NonNegative> = and_contract;
        let _: OrContract<I32Positive, I32NonNegative> = or_contract;
        let _: NotContract<I32Positive> = not_contract;
    }

    #[test]
    fn test_complex_composition() {
        use super::contracts::{I32NonNegative, I32Positive};
        use super::{AndContract, NotContract, OrContract};

        // (Positive AND NonNegative) OR (NOT Positive)
        // This is basically: value >= 0 OR value < 0 (always true)
        let _pos_and_nonneg = AndContract::new(I32Positive, I32NonNegative);
        let _not_pos = NotContract::new(I32Positive);

        // Should accept any i32
        assert!(OrContract::<
            AndContract<I32Positive, I32NonNegative>,
            NotContract<I32Positive>,
        >::requires(&42));
        assert!(OrContract::<
            AndContract<I32Positive, I32NonNegative>,
            NotContract<I32Positive>,
        >::requires(&0));
        assert!(OrContract::<
            AndContract<I32Positive, I32NonNegative>,
            NotContract<I32Positive>,
        >::requires(&-1));
    }

    #[test]
    fn test_string_max_length_invariant() {
        use super::contracts::StringMaxLength;

        let contract = StringMaxLength::<100>;
        assert!(contract.invariant()); // Max length is positive

        let invalid = StringMaxLength::<0>;
        assert!(!invalid.invariant()); // Max length is 0 (invalid)
    }
}
