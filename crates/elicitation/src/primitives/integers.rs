//! Integer type implementations using generic macros.

use crate::{ElicitClient, ElicitResult, Elicitation, Prompt, mcp};

/// Macro to implement Elicitation for all integer types.
///
/// This macro generates default style enum, Prompt, and Elicitation trait
/// implementations for a given integer type. It uses the type's MIN and MAX
/// constants for range validation.
macro_rules! impl_integer_elicit {
    ($t:ty, $style:ident) => {
        // Generate default-only style enum
        crate::default_style!($t => $style);

        impl Prompt for $t {
            fn prompt() -> Option<&'static str> {
                Some(concat!(
                    "Please enter a ",
                    stringify!($t),
                    " (between ",
                    stringify!(<$t>::MIN),
                    " and ",
                    stringify!(<$t>::MAX),
                    "):"
                ))
            }
        }

        impl Elicitation for $t {
            type Style = $style;

            #[tracing::instrument(skip(client), fields(type_name = stringify!($t)))]
            async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
                let prompt = Self::prompt().unwrap();
                tracing::debug!("Eliciting integer type");

                let params = mcp::number_params(prompt, <$t>::MIN as i64, <$t>::MAX as i64);

                let result = client
                    .peer()
                    .call_tool(rmcp::model::CallToolRequestParam {
                        name: mcp::tool_names::elicit_number().into(),
                        arguments: Some(params),
                        task: None,
                    })
                    .await?;

                let value = mcp::extract_value(result)?;
                mcp::parse_integer::<$t>(value)
            }
        }
    };
}

// Apply macro to all signed integer types
impl_integer_elicit!(i8, I8Style);
impl_integer_elicit!(i16, I16Style);
impl_integer_elicit!(i32, I32Style);
impl_integer_elicit!(i64, I64Style);
impl_integer_elicit!(i128, I128Style);
impl_integer_elicit!(isize, IsizeStyle);

// Apply macro to all unsigned integer types
impl_integer_elicit!(u8, U8Style);
impl_integer_elicit!(u16, U16Style);
impl_integer_elicit!(u32, U32Style);
impl_integer_elicit!(u64, U64Style);
impl_integer_elicit!(u128, U128Style);
impl_integer_elicit!(usize, UsizeStyle);
