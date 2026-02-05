//! Floating-point type implementations using generic macros.

use crate::{ElicitCommunicator, ElicitResult, Elicitation, Prompt};

/// Macro to implement Elicitation for floating-point types using Default wrappers.
///
/// This macro generates Elicitation implementations that delegate to
/// the corresponding Default wrapper type (e.g., f32 -> F32Default).
macro_rules! impl_float_elicit_via_wrapper {
    ($primitive:ty, $wrapper:ident, $style:ident) => {
        crate::default_style!($primitive => $style);

        impl Prompt for $primitive {
            fn prompt() -> Option<&'static str> {
                Some(concat!("Please enter a ", stringify!($primitive), " number:"))
            }
        }

        impl Elicitation for $primitive {
            type Style = $style;

            #[tracing::instrument(skip(communicator), fields(type_name = stringify!($primitive)))]
            async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
                use crate::verification::types::$wrapper;

                tracing::debug!(concat!("Eliciting ", stringify!($primitive), " via ", stringify!($wrapper), " wrapper"));

                // Use verification wrapper internally
                let wrapper = $wrapper::elicit(communicator).await?;

                // Unwrap to primitive
                Ok(wrapper.into_inner())
            }
        }
    };
}

// Apply macro to floating-point types
impl_float_elicit_via_wrapper!(f32, F32Default, F32Style);
// f64 implementation below (already uses F64Default)

// f64 implementation using F64Default verification wrapper
crate::default_style!(f64 => F64Style);

impl Prompt for f64 {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a number:")
    }
}

impl Elicitation for f64 {
    type Style = F64Style;

    #[tracing::instrument(skip(communicator), fields(type_name = "f64"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        use crate::verification::types::F64Default;

        tracing::debug!("Eliciting f64 via F64Default wrapper");

        // Use verification wrapper internally
        let wrapper = F64Default::elicit(communicator).await?;

        // Unwrap to primitive
        Ok(wrapper.into_inner())
    }
}
