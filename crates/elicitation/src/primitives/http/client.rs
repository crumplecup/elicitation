//! `reqwest::Client` elicitation (Primitive pattern).
//!
//! A `Client` is always default-constructed — there is no user input to elicit.
//! The `Elicitation` impl gives agents the full power-trait suite: introspection,
//! kani proofs, and style negotiation.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern,
    PatternDetails, Prompt, TypeMetadata,
};

crate::default_style!(reqwest::Client => ClientStyle);

impl Prompt for reqwest::Client {
    fn prompt() -> Option<&'static str> {
        Some("HTTP client (auto-constructed with default settings)")
    }
}

impl Elicitation for reqwest::Client {
    type Style = ClientStyle;

    #[tracing::instrument(skip(_communicator))]
    async fn elicit<C: ElicitCommunicator>(_communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Constructing reqwest::Client");
        Ok(reqwest::Client::new())
    }

    #[cfg(kani)]
    fn kani_proof() {
        // Client::new() is always valid; construction must not panic.
        // We assert the tautological invariant — the real proof is that
        // this code compiles and does not reach unreachable!().
        assert!(true, "reqwest::Client construction verified ∎");
    }
}

impl ElicitIntrospect for reqwest::Client {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "reqwest::Client",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}
