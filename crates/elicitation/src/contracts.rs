//! Proof-carrying composition primitives.
//!
//! This module provides a minimal type-based contract system for building
//! **formally verified** agent programs. Contracts are zero-cost proof markers that enable
//! composing elicitation steps with machine-checked guarantees.
//!
//! # What Are Contracts?
//!
//! Contracts are **compile-time proof markers** that track guarantees through your program.
//! Unlike runtime validation or testing, contracts provide **mathematical certainty** that
//! invariants hold at every step of a multi-step workflow.
//!
//! ```text
//! Traditional Approach         Contract Approach
//! ==================          =================
//! validate(x)                 validate(x) → (x, Proof)
//! use(x)  // Hope valid       use(x, Proof)  // Type-checked!
//! ```
//!
//! **Key insight**: Validate once, carry proof forward. The type system prevents using
//! unvalidated data, and all proofs compile away to nothing.
//!
//! # Overview
//!
//! Contracts let you build multi-step agent workflows where each step's
//! guarantees are **checked at compile time**. Instead of re-validating data
//! at every step, you establish proof once and carry it forward.
//!
//! ## Comparison to Other Approaches
//!
//! | Approach | When Checked | Cost | Guarantees |
//! |----------|-------------|------|------------|
//! | **Runtime validation** | Every use | High | None (can forget) |
//! | **Testing** | Test time | Medium | Statistical only |
//! | **Static analysis** | Compile time | Low | Heuristic |
//! | **Contracts (this)** | Compile time | **Zero** | **Mathematical** |
//!
//! ## Why Not Dependent Types?
//!
//! Dependent type systems (Idris, Agda, Coq) provide similar guarantees but:
//! - Require theorem proving skills
//! - Have steep learning curves
//! - Don't integrate with existing Rust code
//!
//! Contracts give you **80% of the benefit with 5% of the complexity**.
//!
//! # Quick Start
//!
//! ```rust
//! use elicitation::contracts::{Established, And, both};
//!
//! // Define your workflow's propositions
//! #[derive(elicitation::Prop)]
//! struct EmailValidated;
//! #[derive(elicitation::Prop)]
//! struct ConsentObtained;
//!
//! // Step 1: Validate (returns proof if valid)
//! fn validate_email(email: &str) -> Option<Established<EmailValidated>> {
//!     if email.contains('@') { Some(Established::assert()) } else { None }
//! }
//!
//! // Step 2: Function requiring BOTH proofs
//! fn register_user(
//!     email: String,
//!     _proof: Established<And<EmailValidated, ConsentObtained>>
//! ) {
//!     println!("Registered: {}", email);
//! }
//!
//! // Compose: Can't call register_user without both proofs!
//! # let email = "user@example.com";
//! if let Some(email_proof) = validate_email(email) {
//!     let consent_proof = Established::assert(); // Would come from consent flow
//!     let combined = both(email_proof, consent_proof);
//!     register_user(email.to_string(), combined); // ✅ Compiles
//! }
//! // register_user(email.to_string(), ...); // ❌ Won't compile without proof
//! ```
//!
//! # Core Concepts
//!
//! - **Proposition (`Prop`)**: A type-level statement that can be true or false
//! - **Proof (`Established<P>`)**: Evidence that proposition P holds
//! - **Inhabitation (`Is<T>`)**: The proposition that a value inhabits type T
//!
//! # Example
//!
//! ```rust
//! use elicitation::contracts::{Prop, Established, Is};
//! use std::marker::PhantomData;
//!
//! // A proposition: value inhabits String
//! type StringValid = Is<String>;
//!
//! // Function that requires proof
//! fn use_validated_string(
//!     s: String,
//!     _proof: Established<StringValid>
//! ) {
//!     println!("Processing: {}", s);
//! }
//!
//! // Establish proof after validation
//! let s = String::from("hello");
//! let proof = Established::assert();
//! use_validated_string(s, proof);
//! ```
//!
//! # Design Principles
//!
//! - **Zero runtime cost**: All proofs are `PhantomData` and disappear at compile time
//! - **Minimal logic**: Just what's needed for composition (no quantifiers, no negation)
//! - **Type-safe composition**: Cannot call functions without required proofs
//! - **Monotonic refinement**: Guarantees accumulate, never weaken unexpectedly
//!
//! # When to Use
//!
//! Use contracts when:
//! - Building multi-step agent flows with dependencies between steps
//! - Enforcing preconditions that must be established by prior steps
//! - Verifying that validation happens before use (no re-validation needed)
//!
//! Don't use contracts when:
//! - Single-step elicitation (just use `.elicit()` directly)
//! - No dependencies between steps
//! - Performance is so critical you can't afford any abstraction (though cost is zero!)
//!
//! # Multi-Step Composition Example
//!
//! ```rust
//! use elicitation::contracts::{Established, And, both, Is};
//!
//! // Define propositions for agent workflow
//! #[derive(elicitation::Prop)]
//! struct EmailValidated;
//! #[derive(elicitation::Prop)]
//! struct ConsentObtained;
//!
//! // Step 1: Validate email (establishes EmailValidated)
//! fn validate_email(email: &str) -> Option<Established<EmailValidated>> {
//!     if email.contains('@') {
//!         Some(Established::assert())
//!     } else {
//!         None
//!     }
//! }
//!
//! // Step 2: Get consent (establishes ConsentObtained)
//! fn get_consent(user: &str) -> Established<ConsentObtained> {
//!     println!("Getting consent from {}", user);
//!     Established::assert()
//! }
//!
//! // Step 3: Register user (requires BOTH proofs)
//! fn register_user(
//!     email: String,
//!     _proof: Established<And<EmailValidated, ConsentObtained>>
//! ) {
//!     println!("Registered: {}", email);
//! }
//!
//! // Compose the workflow
//! let email = "user@example.com";
//! if let Some(email_proof) = validate_email(email) {
//!     let consent_proof = get_consent(email);
//!     let combined_proof = both(email_proof, consent_proof);
//!     register_user(email.to_string(), combined_proof);
//! }
//! ```
//!
//! # API Overview
//!
//! ## Core Types
//!
//! - [`Prop`]: Marker trait for propositions (type-level statements)
//! - [`Established<P>`]: Proof that proposition P holds
//! - [`Is<T>`]: Proposition that a value inhabits type T
//!
//! ## Logical Operators
//!
//! - [`And<P, Q>`][]: Conjunction (both P and Q hold)
//! - [`Implies<Q>`][]: Implication (P → Q)
//! - [`Refines<Base>`][]: Type refinement (Refined is a Base with extra constraints)
//! - [`InVariant<E, V>`][]: Enum is in specific variant
//!
//! ## Composition Functions
//!
//! - [`both(p, q)`][]: Combine two proofs into conjunction
//! - [`fst(pq)`][]: Project left proof from conjunction
//! - [`snd(pq)`][]: Project right proof from conjunction
//! - [`downcast(refined)`][]: Safe downcast from refined type to base
//!
//! # Advanced Patterns
//!
//! ## State Machines
//!
//! Use `InVariant` to enforce state transitions:
//!
//! ```rust
//! use elicitation::contracts::{Established, InVariant};
//!
//! enum Workflow { Draft, Review, Approved }
//! struct DraftVariant;
//! struct ReviewVariant;
//!
//! fn submit(
//!     _workflow: Workflow,
//!     _draft: Established<InVariant<Workflow, DraftVariant>>
//! ) -> Established<InVariant<Workflow, ReviewVariant>> {
//!     Established::assert()
//! }
//!
//! // Can only submit from Draft state (type-checked!)
//! ```
//!
//! ## Type Refinement
//!
//! Use `Refines` for type hierarchies. Note: both traits must be implemented
//! in your crate to satisfy orphan rules:
//!
//! ```rust,ignore
//! use elicitation::contracts::{Refines, Is, Established, Implies, downcast};
//!
//! struct NonEmptyString(String);
//! impl Refines<String> for NonEmptyString {}
//! impl Implies<Is<String>> for Is<NonEmptyString> {}
//!
//! let refined: Established<Is<NonEmptyString>> = Established::assert();
//! let base: Established<Is<String>> = downcast(refined);
//! // Safe: NonEmptyString is always a String
//! ```
//!
//! # Integration with Elicitation
//!
//! Use `elicit_proven()` to get values with proofs:
//!
//! ```rust,ignore
//! use elicitation::{Elicitation, contracts::{Established, Is}};
//!
//! // Elicit with proof
//! let (email, proof): (String, Established<Is<String>>) =
//!     String::elicit_proven(&client).await?;
//!
//! // Pass proof to functions requiring validation
//! send_email(email, proof).await?;
//! ```
//!
//! # Migration Guide
//!
//! Contracts are **100% opt-in** and don't affect existing code:
//!
//! ```text
//! Before (still works):
//!   let email = String::elicit(&client).await?;
//!   use_email(email);
//!
//! After (with contracts):
//!   let (email, proof) = String::elicit_proven(&client).await?;
//!   use_email_proven(email, proof);
//! ```
//!
//! You can adopt incrementally:
//! 1. Start with single-step validation
//! 2. Add contracts to critical paths
//! 3. Extend to full workflows over time
//!
//! # Performance
//!
//! **Zero cost at runtime**: All proofs are `PhantomData<fn() -> T>` and compile away completely.
//!
//! ```rust
//! use elicitation::contracts::{Established, Is};
//!
//! let proof: Established<Is<String>> = Established::assert();
//! assert_eq!(std::mem::size_of_val(&proof), 0); // Zero bytes!
//! ```
//!
//! Benchmarks show no measurable overhead compared to unvalidated code.
//!
//! # Formal Verification
//!
//! All core properties are **formally verified with Kani**:
//!
//! - ✅ Proofs are zero-sized (verified with symbolic execution)
//! - ✅ Cannot call functions without required proofs (type system + Kani)
//! - ✅ Composition preserves invariants (Kani proves `then()` and `both_tools()`)
//! - ✅ Refinement is sound (Kani proves `downcast()` safety)
//!
//! See `src/kani_tests.rs` for complete verification harnesses.
//!
//! # When NOT to Use Contracts
//!
//! - **Single-step operations**: Just use regular functions
//! - **External APIs**: You can't force external code to use proofs
//! - **Prototyping**: Add contracts after the design stabilizes
//! - **Performance-critical inner loops**: (Though cost is zero, type complexity adds compile time)
//!
//! # Further Reading
//!
//! - [Tool contracts](crate::tool): MCP tools with preconditions/postconditions
//! - [Examples](../../examples): Complete working examples
//! - [Kani verification](https://model-checking.github.io/kani/): How we verify properties

use proc_macro2;
use std::marker::PhantomData;

/// Marker trait: types that represent propositions.
///
/// A proposition is a type-level statement that can be true or false.
/// Propositions are combined using logical operators (`And`, `Implies`)
/// and witnessed by `Established<P>` proofs.
///
/// # Examples
///
/// ```rust
/// use elicitation::contracts::{Prop, Is};
///
/// // Built-in proposition: value inhabits type T
/// type StringProp = Is<String>;
/// ```
pub trait Prop: 'static {
    /// Generate a Kani proof harness for this proposition.
    ///
    /// Returns a [`proc_macro2::TokenStream`] containing a `#[kani::proof]` harness
    /// that encodes this proposition as a trusted axiom. There is no default —
    /// every proposition must supply a proof. Use `#[derive(Prop)]` for trivial
    /// zero-cost marker propositions.
    ///
    /// Available with the `proofs` feature.
    fn kani_proof() -> proc_macro2::TokenStream;

    /// Generate a Verus proof for this proposition.
    ///
    /// Returns a [`proc_macro2::TokenStream`] containing a Verus-verified function
    /// encoding this proposition's postcondition invariant. There is no default —
    /// use `#[derive(Prop)]` for trivial marker propositions.
    ///
    /// Available with the `proofs` feature.
    fn verus_proof() -> proc_macro2::TokenStream;

    /// Generate a Creusot contract proof for this proposition.
    ///
    /// Returns a [`proc_macro2::TokenStream`] containing a `#[trusted]` Creusot
    /// contract function encoding this proposition's postcondition. There is no
    /// default — use `#[derive(Prop)]` for trivial marker propositions.
    ///
    /// Available with the `proofs` feature.
    fn creusot_proof() -> proc_macro2::TokenStream;

    /// The name of the Creusot pearlite `#[logic]` function that expresses this
    /// proposition as a predicate over the machine state.
    ///
    /// When non-empty, `#[formal_method]`-generated Creusot companions emit
    /// real `#[requires]` / `#[ensures]` contracts using this function instead
    /// of the weaker `#[trusted]` placeholder.
    ///
    /// Set via `#[prop(creusot_invariant_fn = "my_fn")]` on the prop struct.
    /// The named function must be a `#[cfg(creusot)] #[logic]` function in scope
    /// wherever the generated companions are compiled.
    fn creusot_invariant_fn_name() -> &'static str {
        ""
    }

    /// The name of a `#[cfg(kani)]` boolean predicate function that expresses
    /// this proposition as a Kani-evaluable invariant check.
    ///
    /// When non-empty, [`formal_method`][crate::formal_method]-generated companion
    /// structs emit `#[kani::requires(fn(&state))]` / `#[kani::ensures]` contracted
    /// wrappers, enabling `#[kani::proof_for_contract]` closure proofs that close
    /// the depth-based inductive argument into a full symbolic proof.
    ///
    /// Set via `#[prop(kani_invariant_fn = "my_fn")]` on the prop struct.
    fn kani_invariant_fn_name() -> &'static str {
        ""
    }
}

/// Witness that proposition P has been established.
///
/// This is a zero-sized proof marker. Its existence proves that
/// proposition P holds in the current context.
///
/// # Zero Cost
///
/// ```rust
/// use elicitation::contracts::{Established, Is};
/// use std::mem::size_of;
///
/// let proof: Established<Is<String>> = Established::assert();
/// assert_eq!(size_of::<Established<Is<String>>>(), 0);
/// ```
///
/// # Safety Model
///
/// `Established<P>` is a semantic contract, not a memory safety guarantee.
/// Calling `Established::assert()` asserts that P is true. The type system
/// ensures you cannot call functions requiring proof without providing it,
/// but it's your responsibility to only assert when P actually holds.
pub struct Established<P: Prop> {
    _marker: PhantomData<fn() -> P>,
}

/// Relation: proposition `P` can be proven from credential `C`.
///
/// Implement this on a `Prop` type to declare which credentials can mint it.
/// The only way to construct `Established<P>` externally is via
/// `Established::<P>::prove(&credential)` where `P: ProvableFrom<C>`.
pub trait ProvableFrom<C>: Prop {}

/// Declare one or more proof-credential ZST types and wire each to its proposition.
///
/// # Syntax
///
/// ```text
/// proof_credential! {
///     /// Optional doc comment.
///     VISIBILITY CredentialName => PropositionType;
///     ...
/// }
/// ```
///
/// Each entry emits:
/// 1. A zero-sized struct `CredentialName` with the given visibility and doc comments.
/// 2. `impl ProvableFrom<CredentialName> for PropositionType {}`
///
/// Credentials are typically `pub(crate)` so only the factory method that
/// performed the runtime check can construct them.  External code cannot call
/// `Established::prove(&CredentialName)` without access to the type.
///
/// # Example
///
/// ```rust
/// use elicitation::contracts::{Established, Prop, ProvableFrom};
/// use elicitation::proof_credential;
///
/// #[derive(elicitation::Prop)]
/// pub struct ContrastChecked;
///
/// proof_credential! {
///     /// Witness that a colour pair was verified for WCAG contrast.
///     pub(crate) NormalContrastVerified => ContrastChecked;
/// }
///
/// // Inside the factory (same crate):
/// let proof: Established<ContrastChecked> =
///     Established::prove(&NormalContrastVerified);
/// ```
#[macro_export]
macro_rules! proof_credential {
    (
        $(
            $(#[$meta:meta])*
            $vis:vis $cred:ident => $prop:ty;
        )*
    ) => {
        $(
            $(#[$meta])*
            $vis struct $cred;

            impl $crate::contracts::ProvableFrom<$cred> for $prop {}
        )*
    };
}

impl<P: Prop> Established<P> {
    /// Mint a proof token by presenting a valid credential.
    ///
    /// The credential type `C` must implement `ProvableFrom<C>` for `P`,
    /// establishing a declared relationship between the two.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use elicitation::contracts::{Established, Is, Prop, ProvableFrom};
    ///
    /// #[derive(elicitation::Prop)]
    /// struct InputValidated;
    ///
    /// struct ValidInput(String);
    /// impl ProvableFrom<ValidInput> for InputValidated {}
    ///
    /// let input = ValidInput("hello".to_string());
    /// let proof: Established<InputValidated> = Established::prove(&input);
    /// ```
    #[inline(always)]
    pub fn prove<C>(_credential: &C) -> Self
    where
        P: ProvableFrom<C>,
    {
        Self {
            _marker: PhantomData,
        }
    }

    /// Assert that proposition `P` holds without a credential.
    ///
    /// This is the unchecked escape hatch: callers take responsibility for
    /// ensuring `P` genuinely holds.  Prefer
    /// [`prove`][Established::prove] when a credential type exists, since
    /// `prove` encodes the check in the type system.
    ///
    /// WCAG proposition types are protected by their credential types being
    /// `pub(crate)` within `elicit_ui` — even with `assert()` available,
    /// external code cannot construct the required credential to call
    /// `prove()`, and calling `assert()` on a WCAG type directly is a clear
    /// audit-trail violation.
    #[inline(always)]
    pub fn assert() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    /// Weaken proof to a more general proposition.
    ///
    /// If P implies Q, then a proof of P can be used as a proof of Q.
    /// This is logical weakening - moving from a stronger to a weaker claim.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use elicitation::contracts::{Established, Is, Implies};
    /// use std::marker::PhantomData;
    ///
    /// // StringNonEmpty implies String (via refinement)
    /// #[derive(elicitation::Prop)]
    /// struct StringNonEmpty;
    /// impl Implies<Is<String>> for StringNonEmpty {}
    ///
    /// let strong_proof: Established<StringNonEmpty> = Established::assert();
    /// let weak_proof: Established<Is<String>> = strong_proof.weaken();
    /// ```
    #[inline(always)]
    pub fn weaken<Q: Prop>(self) -> Established<Q>
    where
        P: Implies<Q>,
    {
        Established {
            _marker: PhantomData,
        }
    }
}

// Make Established copyable (it's zero-sized)
impl<P: Prop> Copy for Established<P> {}
impl<P: Prop> Clone for Established<P> {
    fn clone(&self) -> Self {
        *self
    }
}

// Under Kani, `stub_verified` replaces a call with `kani::any()` of the
// return type.  `Established<P>` is a ZST — its unique value is trivially
// constructible, so any() just returns the unit token.
#[cfg(kani)]
impl<P: Prop> kani::Arbitrary for Established<P> {
    fn any() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<P: Prop> std::fmt::Debug for Established<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Established")
    }
}

impl<P: Prop + crate::Elicitation> crate::Prompt for Established<P> {
    fn prompt() -> Option<&'static str> {
        P::prompt()
    }
}

impl<P: Prop + crate::Elicitation> crate::Elicitation for Established<P> {
    type Style = <P as crate::Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: crate::ElicitCommunicator>(communicator: &C) -> crate::ElicitResult<Self> {
        tracing::debug!("Eliciting proof proposition for Established");
        let _p = P::elicit(communicator).await?;
        Ok(Established::assert())
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }
}

/// `Established<P>` is a zero-sized proof token — re-asserting it in emitted
/// code is safe because the token carries no runtime state. The emitted call
/// `Established::<P>::assert()` is identical to the original construction.
impl<P: Prop + 'static> crate::emit_code::ToCodeLiteral for Established<P> {
    fn type_tokens() -> proc_macro2::TokenStream {
        let p: proc_macro2::TokenStream = std::any::type_name::<P>()
            .parse()
            .unwrap_or_else(|_| quote::quote! { _ });
        quote::quote! { ::elicitation::Established<#p> }
    }

    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let ty = <Self as crate::emit_code::ToCodeLiteral>::type_tokens();
        quote::quote! { <#ty>::assert() }
    }
}

/// `Established<P>` is a zero-sized proof token — it carries no user-facing
/// prompt. Any struct that holds one as a field still needs the bound satisfied
/// when `prompt-tree` is enabled; we return an empty `Leaf` so the derive
/// can compile without noise in the assembled prompt output.
impl<P: Prop> crate::ElicitPromptTree for Established<P> {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Leaf {
            prompt: String::new(),
            type_name: "Established".to_string(),
        }
    }
}

/// `Established<P>` is a zero-sized proof token. Serialization is trivial —
/// the token carries no runtime state, so it round-trips through any format
/// as a unit value. Deserializing reconstructs the token via `assert()`,
/// which is safe because the surrounding serialized state already encodes the
/// invariant that P holds (e.g. the bet balance is present in the ledger).
#[cfg(feature = "serde")]
impl<P: Prop> serde::Serialize for Established<P> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_unit()
    }
}

#[cfg(feature = "serde")]
impl<'de, P: Prop> serde::Deserialize<'de> for Established<P> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <()>::deserialize(deserializer)?;
        Ok(Self::assert())
    }
}

impl<P: Prop + 'static> schemars::JsonSchema for Established<P> {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "EstablishedProof".into()
    }

    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({ "type": "null" })
    }
}

/// Proposition: value inhabits type T with its invariants.
///
/// `Is<T>` represents the statement "a value of type T exists and
/// satisfies T's type invariants". For contract types (like `StringNonEmpty`),
/// this includes the contract's preconditions.
///
/// # Examples
///
/// ```rust
/// use elicitation::contracts::Is;
///
/// // Proposition: a valid String exists
/// type StringValid = Is<String>;
///
/// // Proposition: a non-empty String exists (with contract)
/// // type StringNonEmptyValid = Is<StringNonEmpty>;
/// ```
pub struct Is<T> {
    _marker: PhantomData<fn() -> T>,
}

impl<T: 'static> Prop for Is<T> {
    fn kani_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }
}

/// Logical implication: P implies Q.
///
/// If P implies Q, then whenever P is true, Q must also be true.
/// This enables weakening: converting a proof of P into a proof of Q.
///
/// # Laws
///
/// 1. **Reflexivity**: Every proposition implies itself (P → P)
/// 2. **Transitivity**: If P → Q and Q → R, then P → R
///
/// # Examples
///
/// ```rust
/// use elicitation::contracts::Implies;
///
/// #[derive(elicitation::Prop)]
/// struct Strong;
/// #[derive(elicitation::Prop)]
/// struct Weak;
///
/// // Declare that Strong implies Weak
/// impl Implies<Weak> for Strong {}
/// ```
pub trait Implies<Q: Prop>: Prop {}

// Reflexivity: Every proposition implies itself
impl<P: Prop> Implies<P> for P {}

/// Type alias to reduce PhantomData complexity.
type AndMarker<P, Q> = (fn() -> P, fn() -> Q);

/// Logical conjunction: both P and Q hold.
///
/// `And<P, Q>` represents the proposition that both P and Q are true.
/// This enables combining multiple proofs into a single compound proof.
///
/// # Properties
///
/// - **Commutative** (logically): P ∧ Q ≡ Q ∧ P
/// - **Associative** (logically): (P ∧ Q) ∧ R ≡ P ∧ (Q ∧ R)
/// - **Projectable**: And<P, Q> → P and And<P, Q> → Q
///
/// # Examples
///
/// ```rust
/// use elicitation::contracts::{Established, And, both, fst, snd};
/// use std::marker::PhantomData;
///
/// // Two propositions
/// #[derive(elicitation::Prop)]
/// struct ValidUrl;
/// #[derive(elicitation::Prop)]
/// struct HasPort;
///
/// let url_proof: Established<ValidUrl> = Established::assert();
/// let port_proof: Established<HasPort> = Established::assert();
///
/// // Combine into conjunction
/// let both_proof: Established<And<ValidUrl, HasPort>> = both(url_proof, port_proof);
///
/// // Project back out
/// let url_again: Established<ValidUrl> = fst(both_proof);
/// let port_again: Established<HasPort> = snd(both_proof);
/// ```
pub struct And<P: Prop, Q: Prop> {
    _marker: PhantomData<AndMarker<P, Q>>,
}

impl<P: Prop, Q: Prop> Prop for And<P, Q> {
    fn kani_proof() -> proc_macro2::TokenStream {
        let mut ts = P::kani_proof();
        ts.extend(Q::kani_proof());
        ts
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        let mut ts = P::verus_proof();
        ts.extend(Q::verus_proof());
        ts
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        let mut ts = P::creusot_proof();
        ts.extend(Q::creusot_proof());
        ts
    }
}

/// Combine two proofs into a conjunction.
///
/// Given proofs of P and Q, construct a proof that both hold.
///
/// # Examples
///
/// ```rust
/// use elicitation::contracts::{Established, And, both};
///
/// #[derive(elicitation::Prop)]
/// struct P;
/// #[derive(elicitation::Prop)]
/// struct Q;
///
/// let p: Established<P> = Established::assert();
/// let q: Established<Q> = Established::assert();
/// let pq: Established<And<P, Q>> = both(p, q);
/// ```
#[inline(always)]
pub fn both<P: Prop, Q: Prop>(_p: Established<P>, _q: Established<Q>) -> Established<And<P, Q>> {
    Established {
        _marker: PhantomData,
    }
}

/// Project left component from conjunction.
///
/// Given a proof that both P and Q hold, extract a proof of P.
///
/// # Examples
///
/// ```rust
/// use elicitation::contracts::{Established, And, both, fst};
///
/// #[derive(elicitation::Prop)]
/// struct P;
/// #[derive(elicitation::Prop)]
/// struct Q;
///
/// let pq: Established<And<P, Q>> = both(Established::assert(), Established::assert());
/// let p: Established<P> = fst(pq);
/// ```
#[inline(always)]
pub fn fst<P: Prop, Q: Prop>(_both: Established<And<P, Q>>) -> Established<P> {
    Established {
        _marker: PhantomData,
    }
}

/// Project right component from conjunction.
///
/// Given a proof that both P and Q hold, extract a proof of Q.
///
/// # Examples
///
/// ```rust
/// use elicitation::contracts::{Established, And, both, snd};
///
/// #[derive(elicitation::Prop)]
/// struct P;
/// #[derive(elicitation::Prop)]
/// struct Q;
///
/// let pq: Established<And<P, Q>> = both(Established::assert(), Established::assert());
/// let q: Established<Q> = snd(pq);
/// ```
#[inline(always)]
pub fn snd<P: Prop, Q: Prop>(_both: Established<And<P, Q>>) -> Established<Q> {
    Established {
        _marker: PhantomData,
    }
}

/// Type-level refinement: `Self` refines `Base`.
///
/// A refinement type has all the properties of its base type, plus
/// additional constraints. This means:
/// - If a value inhabits the refined type, it necessarily inhabits the base type
/// - You can safely downcast from refined proof to base proof
/// - You cannot upcast (compile error without additional validation)
///
/// # Laws
///
/// 1. **Reflexivity**: Every type refines itself (T → T)
/// 2. **Transitivity**: If A refines B and B refines C, then A refines C
/// 3. **Inhabitation**: `Is<Refined>` implies `Is<Base>` (requires explicit `Implies` impl)
///
/// # Example
///
/// ```rust,no_run
/// use elicitation::contracts::{Refines, Is, Established, Implies, Prop, downcast};
///
/// // Define a refined string type (in practice, would have validation)
/// struct NonEmptyString(String);
///
/// // Declare refinement relationship and implication
/// impl Refines<String> for NonEmptyString {}
/// // In actual code: impl Implies<Is<String>> for Is<NonEmptyString> {}
///
/// // Can downcast from refined to base:
/// // let refined_proof: Established<Is<NonEmptyString>> = Established::assert();
/// // let base_proof: Established<Is<String>> = downcast(refined_proof);
/// ```
pub trait Refines<Base>: 'static {}

// Reflexivity: every type refines itself
impl<T: 'static> Refines<T> for T {}

/// Downcast proof from refined type to base type.
///
/// If you have a proof that a value inhabits a refined type,
/// you automatically have proof it inhabits the base type.
///
/// This works via the `Refines` trait relationship. Users must
/// implement `Implies<Is<Base>>` for `Is<Refined>` to enable downcasting.
///
/// # Examples
///
/// ```rust,no_run
/// use elicitation::contracts::{Refines, Is, Established, Implies, Prop, downcast};
///
/// struct NonEmptyString(String);
/// impl Refines<String> for NonEmptyString {}
///
/// // In actual code, you'd implement this in your crate:
/// // impl Implies<Is<String>> for Is<NonEmptyString> {}
///
/// // Then downcast works:
/// // let refined: Established<Is<NonEmptyString>> = Established::assert();
/// // let base: Established<Is<String>> = downcast(refined);
/// ```
#[inline(always)]
pub fn downcast<Base: 'static, Refined: Refines<Base>>(
    proof: Established<Is<Refined>>,
) -> Established<Is<Base>>
where
    Is<Refined>: Implies<Is<Base>>,
{
    proof.weaken()
}

/// Type alias to reduce PhantomData complexity.
type InVariantMarker<E, V> = (fn() -> E, fn() -> V);

/// Proposition: enum value is in specific variant.
///
/// `InVariant<E, V>` represents the statement "enum E is currently
/// in variant V". This enables variant-specific proofs for enum-based
/// state machines.
///
/// # Type Parameters
///
/// - `E`: The enum type
/// - `V`: A marker type representing the variant (typically a unit struct)
///
/// # Example
///
/// ```rust
/// use elicitation::contracts::{InVariant, Established, Prop};
///
/// enum Status {
///     Active,
///     Inactive,
/// }
///
/// // Marker types for variants
/// struct ActiveVariant;
/// struct InactiveVariant;
///
/// // Use InVariant to track which variant
/// fn process_active(_status: Status, _proof: Established<InVariant<Status, ActiveVariant>>) {
///     // This function can only be called with Active variant
///     println!("Processing active status");
/// }
/// ```
pub struct InVariant<E, V> {
    _marker: PhantomData<InVariantMarker<E, V>>,
}

impl<E: 'static, V: 'static> Prop for InVariant<E, V> {
    fn kani_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }
}

// ── Formal Methods ────────────────────────────────────────────────────────────

/// A function that consumes a proof and produces a proof.
///
/// Any `Fn(In, Established<PIn>) -> (Out, Established<POut>)` automatically
/// implements this trait via the blanket impl below. The signature IS the
/// contract declaration — no attribute or registry required.
///
/// # Type-driven call-graph closure
///
/// A function calling an informal helper cannot derive `Established<POut>` from
/// that helper's result without using [`Established::assert`], the explicit,
/// auditable escape hatch. The type system therefore closes the call graph
/// automatically: proof tokens only flow from formal call sites.
///
/// # Evidence bundles
///
/// When multiple propositions must be satisfied, use an evidence bundle struct
/// (a plain struct whose fields are `Established<P>` tokens that implements
/// `ProvableFrom<Bundle>`). Pass the bundle as `In` or as `PIn` evidence.
///
/// # Examples
///
/// ```rust
/// use elicitation::contracts::{Established, FormalMethod, Prop};
///
/// #[derive(elicitation::Prop)]
/// struct Validated;
/// #[derive(elicitation::Prop)]
/// struct Processed;
///
/// // Any function with the matching signature is automatically a FormalMethod.
/// fn process(input: String, _proof: Established<Validated>)
///     -> (String, Established<Processed>)
/// {
///     (input.to_uppercase(), Established::assert())
/// }
///
/// // Use via the trait or call directly — both work.
/// let proof_in = Established::assert();
/// let (_result, _proof_out) = process("hello".to_string(), proof_in);
/// ```
pub trait FormalMethod<In, PIn: Prop, Out, POut: Prop> {
    /// Call this method, consuming the input proof and producing an output proof.
    fn call_formal(&self, input: In, proof: Established<PIn>) -> (Out, Established<POut>);

    /// Generate a Kani proof harness for this method's precondition/postcondition.
    ///
    /// The default returns an empty token stream (no harness). Override to emit
    /// a `#[kani::proof]` harness asserting that the method honours its contract.
    fn kani_harness() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    /// Generate a Verus proof harness for this method's contract.
    fn verus_harness() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    /// Generate a Creusot contract for this method.
    fn creusot_harness() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }
}

/// Blanket impl: any `Fn(In, Established<PIn>) -> (Out, Established<POut>)`
/// is automatically a `FormalMethod`.
impl<F, In, PIn: Prop, Out, POut: Prop> FormalMethod<In, PIn, Out, POut> for F
where
    F: Fn(In, Established<PIn>) -> (Out, Established<POut>),
{
    #[inline(always)]
    fn call_formal(&self, input: In, proof: Established<PIn>) -> (Out, Established<POut>) {
        self(input, proof)
    }
}

// ── Verified State Machines ───────────────────────────────────────────────────

/// A state machine whose states are fully described and whose transitions
/// preserve a declared invariant.
///
/// # Contract
///
/// A `VerifiedStateMachine` declares two associated types:
///
/// - `State` — must be [`ElicitComplete`][crate::ElicitComplete]: fully
///   introspectable, serialisable, and reasoned about by elicitation tooling.
/// - `Invariant` — a [`Prop`] that every valid transition must preserve.
///
/// Transitions are [`FormalMethod`]s with signature
/// `(State, Established<Invariant>) -> (State, Established<Invariant>)`.
/// The type system guarantees that a transition cannot produce a new state
/// without presenting proof that the invariant still holds.
///
/// # "Gated community" for formal verification
///
/// Declaring a `VerifiedStateMachine` is the top-level compiler-enforced
/// claim that a system preserves its invariants. Outside a VSM any piece of
/// the contracts stack can be used freely; inside, every transition must be
/// a `FormalMethod`.
///
/// # Examples
///
/// ```rust
/// use elicitation::contracts::{
///     Established, FormalMethod, Prop, VerifiedStateMachine, VerifiedTransition,
/// };
/// use elicitation::ElicitComplete;
///
/// // --- State ---
/// #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize,
///          schemars::JsonSchema, elicitation::Elicit)]
/// enum OrderState { Draft, Submitted, Shipped }
///
/// // --- Invariant proposition ---
/// #[derive(elicitation::Prop)]
/// struct OrderIntact;
///
/// // --- The VSM declaration ---
/// struct OrderMachine;
/// impl VerifiedStateMachine for OrderMachine {
///     type State     = OrderState;
///     type Invariant = OrderIntact;
/// }
///
/// // --- A verified transition ---
/// fn submit(state: OrderState, proof: Established<OrderIntact>)
///     -> (OrderState, Established<OrderIntact>)
/// {
///     (OrderState::Submitted, proof) // invariant preserved
/// }
///
/// // `submit` satisfies VerifiedTransition<OrderMachine> automatically.
/// fn run<T: VerifiedTransition<OrderMachine>>(t: &T) {
///     let proof = Established::assert();
///     let (_new_state, _new_proof) = t.call_formal(OrderState::Draft, proof);
/// }
/// run(&submit);
/// ```
pub trait VerifiedStateMachine {
    /// The state type.  Must be [`ElicitComplete`][crate::ElicitComplete].
    type State: crate::ElicitComplete;

    /// The invariant that every transition must preserve.
    type Invariant: Prop;

    /// Return the Kani harness [`proc_macro2::TokenStream`] for every
    /// transition in this machine.
    ///
    /// Override this in each `VerifiedStateMachine` implementation, listing the
    /// companion structs generated by [`formal_method`][crate::formal_method]:
    ///
    /// ```rust,ignore
    /// fn transition_harnesses() -> Vec<proc_macro2::TokenStream> {
    ///     vec![
    ///         MyTransitionATransition::kani_harness(),
    ///         MyTransitionBTransition::kani_harness(),
    ///     ]
    /// }
    /// ```
    ///
    /// The default implementation returns an empty list (no harnesses).
    #[cfg(not(kani))]
    fn transition_harnesses() -> Vec<proc_macro2::TokenStream> {
        vec![]
    }

    /// Compose the full VSM Kani proof: the invariant proposition proof
    /// followed by all `proof_for_contract` closure harnesses for each
    /// registered transition.
    ///
    /// This is the token stream that a `build.rs` should write to a generated
    /// `.rs` file so that Kani can verify the entire state machine end-to-end.
    ///
    /// The composition says: "the invariant is a valid proposition AND every
    /// transition preserves it without panicking for any reachable input."
    #[cfg(not(kani))]
    fn vsm_kani_proof() -> proc_macro2::TokenStream {
        let mut ts = Self::Invariant::kani_proof();
        let inv_fn = Self::Invariant::kani_invariant_fn_name();
        for closure in Self::transition_kani_closure_proofs(inv_fn) {
            ts.extend(closure);
        }
        ts
    }

    /// Return one `proof_for_contract` closure harness per transition in this machine.
    ///
    /// Each entry is a `#[kani::proof_for_contract(fn_name)]` harness using the
    /// forgive-and-forget pattern.  Contracts on the original function (emitted by
    /// `#[formal_method]` via `cfg_attr(kani, kani::requires/ensures)`) are verified
    /// by DFCC.  Once verified, each transition can be replaced with
    /// `stub_verified(fn_name)` in multi-step composition harnesses.
    ///
    /// When `inv_fn` is empty, all entries are empty `TokenStream`s.
    ///
    /// The default implementation returns an empty list.
    #[cfg(not(kani))]
    fn transition_kani_closure_proofs(_inv_fn: &str) -> Vec<proc_macro2::TokenStream> {
        vec![]
    }

    /// Return one Creusot companion contract per transition in this machine.
    ///
    /// Each entry is a `#[cfg(creusot)]` function with `#[requires]`/`#[ensures]`
    /// annotations. When the invariant type's [`Prop::creusot_invariant_fn_name`]
    /// returns a non-empty string, the contracts are real (no `#[trusted]`).
    /// Otherwise they fall back to `#[trusted]` placeholders.
    ///
    /// The default implementation returns an empty list.
    #[cfg(not(kani))]
    fn transition_creusot_contracts(_inv_fn: &str) -> Vec<proc_macro2::TokenStream> {
        vec![]
    }

    /// Compose the full VSM Creusot proof: the invariant proposition proof
    /// followed by one contract per registered transition.
    ///
    /// When `Self::Invariant::creusot_invariant_fn_name()` is non-empty, the
    /// generated companions use real `#[requires]`/`#[ensures]` annotations and
    /// Creusot will verify the function bodies. Otherwise trusted placeholders
    /// are emitted (same as the stub path).
    #[cfg(not(kani))]
    fn vsm_creusot_proof() -> proc_macro2::TokenStream {
        let mut ts = Self::Invariant::creusot_proof();
        let inv_fn = Self::Invariant::creusot_invariant_fn_name();
        for contract in Self::transition_creusot_contracts(inv_fn) {
            ts.extend(contract);
        }
        ts
    }
}

/// Convenience alias: a verified transition for state machine `VSM`.
///
/// Any function (or closure) whose signature is
/// `(VSM::State, Established<VSM::Invariant>) -> (VSM::State, Established<VSM::Invariant>)`
/// satisfies this bound automatically via the [`FormalMethod`] blanket impl.
pub trait VerifiedTransition<VSM: VerifiedStateMachine>:
    FormalMethod<VSM::State, VSM::Invariant, VSM::State, VSM::Invariant>
{
}

/// Blanket impl: any `FormalMethod` over the right state/invariant types is a
/// `VerifiedTransition` for the corresponding `VerifiedStateMachine`.
impl<VSM, T> VerifiedTransition<VSM> for T
where
    VSM: VerifiedStateMachine,
    T: FormalMethod<VSM::State, VSM::Invariant, VSM::State, VSM::Invariant>,
{
}

/// Per-variant concrete construction expressions at each compositional depth.
///
/// Used by `#[derive(VerifiedStateMachine)]` to emit three harnesses per
/// `(transition × variant)`: one per depth.  Each depth uses `KaniCompose`
/// field expressions from `#[derive(KaniVariantState)]`.
pub struct KaniVariantConstruction {
    /// Snake_case variant name suffix for harness function names
    /// (e.g. `"explain_view"` for `ExplainView`).
    pub variant_name: &'static str,
    /// Depth-0 construction expression: all collections empty / `None`.
    pub depth0: String,
    /// Depth-1 construction expression: one element in each collection.
    pub depth1: String,
    /// Depth-2 construction expression: two elements in each collection.
    pub depth2: String,
}

/// Per-variant concrete construction expressions for VSM state enums.
///
/// Implemented via `#[derive(KaniVariantState)]`. Used by `derive_vsm` to
/// generate per-variant Kani harnesses — three harnesses per
/// `(transition × variant)`, one per compositional depth — so that CBMC
/// receives a concrete discriminant and bounded fields instead of a fully
/// symbolic enum.
///
/// # Motivation
///
/// `kani::any::<StateEnum>()` creates a tagged union where ALL variant fields
/// are globally symbolic in CBMC. Dropping such a value requires reasoning
/// about every variant destructor simultaneously, causing unbounded unwinding
/// for variants that contain `Vec<T>` or `String` (non-trivial destructors).
///
/// Per-variant harnesses give CBMC a concrete discriminant for each proof,
/// eliminating the symbolic-enum-drop problem while preserving exhaustive
/// coverage through case analysis.  Per-depth harnesses extend this to
/// cover recursive / collection fields at sizes 0, 1, and 2.
pub trait KaniVariantState {
    /// Returns per-variant construction expressions at all three depths.
    ///
    /// Each [`KaniVariantConstruction`] provides:
    ///
    /// - `variant_name` — snake_case suffix for the harness function name.
    /// - `depth0` — all collections empty / `None` (base case).
    /// - `depth1` — one element in each collection (inductive step).
    /// - `depth2` — two elements in each collection (inductive step ×2).
    ///
    /// Field expressions use `<T as KaniCompose>::kani_depth{n}()` for
    /// non-primitive types, `Vec::new()` / `None` / `String::new()` for
    /// recognized collection types, and `kani::any()` for primitives.
    fn kani_variant_constructions() -> Vec<KaniVariantConstruction>;
}

// ── Kani-safe label helpers ───────────────────────────────────────────────────

/// Produce a label string for display / diagnostics in formal-method transitions.
///
/// In normal builds, expands to `format!($args*)` as written.  In Kani builds,
/// returns `String::new()` instead so CBMC does not model the entire Rust
/// formatter machinery — `format!()` involves trait-object dispatch through
/// `std::fmt` that multiplies CBMC paths even for fully concrete inputs.
///
/// # Usage
///
/// Replace every `format!("...")` that produces a *display label* (content
/// irrelevant to invariant correctness) in a `#[formal_method]` transition
/// body with `kani_label!("...")`:
///
/// ```rust,ignore
/// let label_left = kani_label!("{old_schema}.{old_table}");
/// ```
#[macro_export]
macro_rules! kani_label {
    ($($arg:tt)*) => {{
        #[cfg(any(kani, creusot))]
        let _s = ::std::string::String::new();
        #[cfg(not(any(kani, creusot)))]
        let _s = format!($($arg)*);
        _s
    }};
}
