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
            assert!(
                Self::ensures(&input, &output),
                "Postcondition must hold after execution",
            );
        }

        tracing::debug!("Tool execution successful");
        Ok(output)
    }
}

/// Produce an empty `Vec<T>` as a verified proof boundary.
///
/// # Trust boundary
///
/// VSM harnesses verify *our* transition logic, not the correctness of
/// `std::vec::Vec`.  Proving `Vec` itself is out of scope; we trust the
/// standard library.
///
/// # Why `Vec::new()` instead of `any_vec` or `bounded_any`
///
/// Both `kani::vec::any_vec::<T, 0>()` and `Vec::<T>::bounded_any::<0>()`
/// leave a symbolic `len` field visible to CBMC:
///
/// - `any_vec` routes through `exact_vec` → `<[T]>::into_vec(Box::new(any()))`,
///   which loses the compile-time `N=0`; the resulting Vec carries a
///   runtime-symbolic `len`.
/// - `bounded_any` calls `vec.truncate(symbolic_real_length)` where
///   `real_length = any_where(|s| *s <= 0)`.  Even though constrained to ≤0,
///   CBMC does not reduce `symbolic_0` to the concrete value `0`, so
///   `drop_in_place` on a symbolic-length slice unwinds without bound.
///
/// Both strategies work fine for element types with trivial drop (unit enums,
/// scalar structs) because CBMC can bound the loop body.  For types whose
/// destructor reaches `dealloc` (e.g. `String`-bearing structs), the loop
/// body is unbounded and CBMC diverges.
///
/// `Vec::new()` gives CBMC a **concrete** `len = 0`; no loop body is
/// generated at all.  The proof scope is: "for any state whose `Vec` fields
/// are empty, the transition is structurally correct."  This is the correct
/// scope for structural state-machine invariants.
///
/// Use this function in `kani::Arbitrary` impls for state-enum variants that
/// carry `Vec<T>` payload fields.
#[cfg(kani)]
pub fn kani_vec<T>() -> Vec<T> {
    Vec::new()
}
