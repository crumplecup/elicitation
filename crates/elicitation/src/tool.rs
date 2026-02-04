//! Contract-based tool system for MCP.
//!
//! This module provides a type-safe interface for MCP tools with
//! explicit preconditions and postconditions expressed as proofs.
//!
//! # Design
//!
//! Tools are functions with contracts:
//! - **Precondition**: What must be true before the tool can be called
//! - **Postcondition**: What becomes true after the tool succeeds
//!
//! The type system enforces that:
//! - Tools cannot be called without establishing preconditions
//! - Tool chains must prove intermediate conditions
//! - Proofs are zero-cost (compile away completely)
//!
//! # Example
//!
//! ```rust,ignore
//! use elicitation::tool::{Tool, True};
//! use elicitation::contracts::{Established, Prop};
//!
//! struct EmailValidated;
//! impl Prop for EmailValidated {}
//!
//! struct SendEmailTool;
//! impl Tool for SendEmailTool {
//!     type Input = String;
//!     type Output = ();
//!     type Pre = EmailValidated;  // Requires validated email
//!     type Post = True;           // No postcondition
//!
//!     async fn execute(
//!         &self,
//!         email: String,
//!         _pre: Established<EmailValidated>,
//!     ) -> Result<((), Established<True>), ToolError> {
//!         // Send email...
//!         Ok(((), True::axiom()))
//!     }
//! }
//! ```

use crate::{
    contracts::{Established, Prop},
    ElicitResult, Elicitation,
};

/// MCP tool with explicit preconditions and postconditions.
///
/// Tools are contract-preserving functions that:
/// - Require proof of preconditions before execution
/// - Return proof of postconditions after success
/// - Cannot be called without establishing prerequisites
///
/// # Type Parameters
///
/// - `Input`: The input type (must implement `Elicitation`)
/// - `Output`: The output type
/// - `Pre`: Precondition proposition (what must be true before)
/// - `Post`: Postcondition proposition (what's true after success)
///
/// # Example
///
/// ```rust,ignore
/// struct ValidateEmailTool;
/// 
/// impl Tool for ValidateEmailTool {
///     type Input = String;
///     type Output = String;
///     type Pre = True;           // No precondition
///     type Post = EmailValidated; // Establishes validation
///     
///     async fn execute(&self, email: String, _pre: Established<True>)
///         -> Result<(String, Established<EmailValidated>), ToolError>
///     {
///         if email.contains('@') {
///             Ok((email, Established::assert()))
///         } else {
///             Err(ToolError::validation("Invalid email"))
///         }
///     }
/// }
/// ```
pub trait Tool {
    /// Tool input type (must be elicitable).
    type Input: Elicitation;

    /// Tool output type.
    type Output;

    /// Precondition proposition (what must be true before calling).
    type Pre: Prop;

    /// Postcondition proposition (what's true after success).
    type Post: Prop;

    /// Execute tool with precondition proof, returns output and postcondition proof.
    ///
    /// # Arguments
    ///
    /// - `input`: The tool input
    /// - `_pre`: Proof that precondition holds
    ///
    /// # Returns
    ///
    /// On success: `(output, proof_of_postcondition)`
    /// On failure: `ToolError`
    fn execute(
        &self,
        input: Self::Input,
        _pre: Established<Self::Pre>,
    ) -> impl std::future::Future<Output = ElicitResult<(Self::Output, Established<Self::Post>)>>
           + Send;
}

/// Trivially true proposition.
///
/// `True` is always established and can be used as a precondition for
/// unconstrained tools or as a postcondition for tools with no guarantees.
///
/// # Example
///
/// ```rust
/// use elicitation::tool::True;
/// use elicitation::contracts::{Established, Prop};
///
/// // Tools with no preconditions use True
/// let no_constraint: Established<True> = True::axiom();
///
/// // Can call anytime
/// assert_eq!(std::mem::size_of_val(&no_constraint), 0);
/// ```
pub struct True;

impl Prop for True {}

impl True {
    /// Axiom: truth is always established.
    ///
    /// This provides a proof of `True` without any preconditions.
    /// Use this as the precondition proof for unconstrained tools.
    ///
    /// # Example
    ///
    /// ```rust
    /// use elicitation::tool::True;
    ///
    /// let proof = True::axiom();
    /// // Can pass this to any tool with Pre = True
    /// ```
    #[inline(always)]
    pub fn axiom() -> Established<True> {
        Established::assert()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true_is_zero_sized() {
        let proof = True::axiom();
        assert_eq!(std::mem::size_of_val(&proof), 0);
    }

    #[test]
    fn test_true_is_copy() {
        let proof = True::axiom();
        let proof2 = proof;
        let _proof3 = proof; // Can still use original
        let _proof4 = proof2;
    }

    #[test]
    fn test_true_axiom_always_succeeds() {
        // Can call anytime, anywhere
        let _proof1 = True::axiom();
        let _proof2 = True::axiom();
        let _proof3 = True::axiom();
    }
}
