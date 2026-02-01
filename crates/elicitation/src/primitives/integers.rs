//! Integer type implementations using generic macros.

use crate::{ElicitClient, ElicitResult, Elicitation, Prompt};

/// Macro to implement Elicitation for integer types using Default wrappers.
///
/// This macro generates Elicitation implementations that delegate to
/// the corresponding Default wrapper type (e.g., i8 -> I8Default).
macro_rules! impl_integer_elicit_via_wrapper {
    ($primitive:ty, $wrapper:ident, $style:ident) => {
        crate::default_style!($primitive => $style);

        impl Prompt for $primitive {
            fn prompt() -> Option<&'static str> {
                Some(concat!(
                    "Please enter a ",
                    stringify!($primitive),
                    " (between ",
                    stringify!(<$primitive>::MIN),
                    " and ",
                    stringify!(<$primitive>::MAX),
                    "):"
                ))
            }
        }

        impl Elicitation for $primitive {
            type Style = $style;

            #[tracing::instrument(skip(client), fields(type_name = stringify!($primitive)))]
            async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
                use crate::verification::types::$wrapper;

                tracing::debug!(concat!("Eliciting ", stringify!($primitive), " via ", stringify!($wrapper), " wrapper"));

                // Use verification wrapper internally
                let wrapper = $wrapper::elicit(client).await?;

                // Unwrap to primitive
                Ok(wrapper.into_inner())
            }
        }
    };
}

// Apply macro to all signed integer types
impl_integer_elicit_via_wrapper!(i8, I8Default, I8Style);
impl_integer_elicit_via_wrapper!(i16, I16Default, I16Style);
impl_integer_elicit_via_wrapper!(i32, I32Default, I32Style);
// i64 implementation below (already uses I64Default)
impl_integer_elicit_via_wrapper!(i128, I128Default, I128Style);
impl_integer_elicit_via_wrapper!(isize, IsizeDefault, IsizeStyle);

// i64 implementation using I64Default verification wrapper
crate::default_style!(i64 => I64Style);

impl Prompt for i64 {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a i64 (between -9223372036854775808 and 9223372036854775807):")
    }
}

impl Elicitation for i64 {
    type Style = I64Style;

    #[tracing::instrument(skip(client), fields(type_name = "i64"))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        use crate::verification::types::I64Default;

        tracing::debug!("Eliciting i64 via I64Default wrapper");

        // Use verification wrapper internally
        let wrapper = I64Default::elicit(client).await?;

        // Unwrap to primitive
        Ok(wrapper.into_inner())
    }
}

// Apply macro to all unsigned integer types
impl_integer_elicit_via_wrapper!(u8, U8Default, U8Style);
impl_integer_elicit_via_wrapper!(u16, U16Default, U16Style);
impl_integer_elicit_via_wrapper!(u32, U32Default, U32Style);
impl_integer_elicit_via_wrapper!(u64, U64Default, U64Style);
impl_integer_elicit_via_wrapper!(u128, U128Default, U128Style);
impl_integer_elicit_via_wrapper!(usize, UsizeDefault, UsizeStyle);
