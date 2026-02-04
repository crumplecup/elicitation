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

/// Sequentially compose two tools where first's postcondition implies second's precondition.
///
/// Chains tools together: runs the first tool, then uses its output and
/// postcondition proof to run the second tool. The postcondition of the
/// first tool must imply the precondition of the second tool.
///
/// # Type Parameters
///
/// - `T1`: First tool type
/// - `T2`: Second tool type (input must match T1's output)
///
/// # Arguments
///
/// - `tool1`: First tool to execute
/// - `tool2`: Second tool to execute
/// - `input1`: Input for first tool
/// - `pre1`: Precondition proof for first tool
///
/// # Returns
///
/// Tuple of (final output, final postcondition proof)
///
/// # Example
///
/// ```rust,ignore
/// use elicitation::tool::{Tool, True, then};
///
/// // Chain: validate email then send
/// let validator = ValidateEmailTool;
/// let sender = SendEmailTool;
///
/// let (result, proof) = then(
///     &validator,
///     &sender,
///     "user@example.com".to_string(),
///     True::axiom(),
/// ).await?;
/// ```
pub async fn then<T1, T2>(
    tool1: &T1,
    tool2: &T2,
    input1: T1::Input,
    pre1: Established<T1::Pre>,
) -> ElicitResult<(T2::Output, Established<T2::Post>)>
where
    T1: Tool,
    T2: Tool<Input = T1::Output>,
    T1::Post: crate::contracts::Implies<T2::Pre>,
{
    let (output1, post1) = tool1.execute(input1, pre1).await?;
    let pre2 = post1.weaken();
    let (output2, post2) = tool2.execute(output1, pre2).await?;
    Ok((output2, post2))
}

/// Run two tools in parallel and combine their proofs.
///
/// Executes both tools concurrently (though this implementation runs
/// sequentially for now). Requires a proof that both preconditions hold,
/// and returns both outputs with a proof that both postconditions hold.
///
/// # Type Parameters
///
/// - `T1`: First tool type
/// - `T2`: Second tool type
///
/// # Arguments
///
/// - `tool1`: First tool to execute
/// - `tool2`: Second tool to execute
/// - `input1`: Input for first tool
/// - `input2`: Input for second tool
/// - `pre`: Proof that both preconditions hold
///
/// # Returns
///
/// Tuple of ((output1, output2), proof of both postconditions)
///
/// # Example
///
/// ```rust,ignore
/// use elicitation::tool::{Tool, both_tools};
/// use elicitation::contracts::{And, both};
///
/// let (outputs, combined_proof) = both_tools(
///     &tool1,
///     &tool2,
///     input1,
///     input2,
///     both(pre1, pre2),
/// ).await?;
/// ```
pub async fn both_tools<T1, T2>(
    tool1: &T1,
    tool2: &T2,
    input1: T1::Input,
    input2: T2::Input,
    pre: Established<crate::contracts::And<T1::Pre, T2::Pre>>,
) -> ElicitResult<(
    (T1::Output, T2::Output),
    Established<crate::contracts::And<T1::Post, T2::Post>>,
)>
where
    T1: Tool,
    T2: Tool,
{
    use crate::contracts::{both, fst, snd};

    let pre1 = fst(pre);
    let pre2 = snd(pre);

    let (out1, post1) = tool1.execute(input1, pre1).await?;
    let (out2, post2) = tool2.execute(input2, pre2).await?;

    Ok(((out1, out2), both(post1, post2)))
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
