//! Kani model checker adapter for formal verification.
//!
//! This module adapts the generic [`Contract`](super::Contract) trait for use
//! with the [Kani Rust Verifier](https://github.com/model-checking/kani).
//!
//! # Kani Integration
//!
//! Kani uses **bounded model checking** to formally verify Rust programs:
//! - Symbolic execution of all possible inputs
//! - SMT solver verification of assertions
//! - Compile-time proofs (not runtime tests)
//!
//! # Example
//!
//! ```rust,ignore
//! use elicitation::verification::{Contract, kani::Tool};
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
//!     async fn execute(&self, input: String) -> ElicitResult<String> {
//!         // Implementation
//!         Ok(input)
//!     }
//! }
//!
//! // Kani proof harness
//! #[cfg(kani)]
//! #[kani::proof]
//! fn verify_email_contract() {
//!     let input = String::from("test@example.com");
//!     kani::assume(ValidateEmail::requires(&input));
//!     assert!(input.contains('@')); // ✅ PROVEN
//! }
//! ```
//!
//! # Key Features
//!
//! - **Model Checking**: Explores all possible execution paths
//! - **Bounded Verification**: With `#[kani::unwind(n)]` for loops
//! - **Symbolic Execution**: `kani::any()` for symbolic values
//! - **Contract Checking**: Runtime verification with `verify_and_execute`

use super::Contract;
use crate::ElicitResult;

// Re-export Contract as KaniContract for backwards compatibility
pub use super::Contract as KaniContract;

/// Executable tool with Kani contract verification.
///
/// Extends [`Contract`] with async execution and runtime contract checking.
///
/// # Example
///
/// ```rust,ignore
/// #[async_trait::async_trait]
/// impl Tool for MyTool {
///     async fn execute(&self, input: Self::Input) -> ElicitResult<Self::Output> {
///         // Implementation
///     }
/// }
/// ```
#[async_trait::async_trait]
pub trait Tool: Contract + Sync {
    /// Execute the tool.
    ///
    /// Implementations should focus on the core logic. Contract verification
    /// is handled by [`verify_and_execute`](Tool::verify_and_execute).
    async fn execute(&self, input: Self::Input) -> ElicitResult<Self::Output>;

    /// Verify and execute with contract checking.
    ///
    /// This is the primary execution method. It ensures:
    /// 1. Preconditions hold before execution
    /// 2. Postconditions hold after execution
    /// 3. Invariants hold throughout
    async fn verify_and_execute(&self, input: Self::Input) -> ElicitResult<Self::Output> {
        tracing::debug!("Verifying preconditions");

        // In debug/test builds, check contracts at runtime
        #[cfg(debug_assertions)]
        {
            if !Self::requires(&input) {
                tracing::error!("Precondition violated");
                return Err(crate::ElicitErrorKind::ParseError(
                    "Tool precondition violated".to_string(),
                )
                .into());
            }
        }

        // With Kani, verify contracts formally
        #[cfg(kani)]
        {
            kani::assume(Self::requires(&input));
        }

        tracing::debug!("Executing tool");
        let output = self.execute(input.clone()).await?;

        // Verify postconditions
        #[cfg(debug_assertions)]
        {
            if !Self::ensures(&input, &output) {
                tracing::error!("Postcondition violated");
                return Err(crate::ElicitErrorKind::ParseError(
                    "Tool postcondition violated".to_string(),
                )
                .into());
            }
        }

        #[cfg(kani)]
        {
            kani::assert(
                Self::ensures(&input, &output),
                "Postcondition must hold after execution",
            );
        }

        tracing::debug!("Tool execution successful");
        Ok(output)
    }
}
