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
// i64 uses verification wrapper - see below
impl_integer_elicit!(i128, I128Style);
impl_integer_elicit!(isize, IsizeStyle);

// i64 implementation using I64Default verification wrapper
crate::default_style!(i64 => I64Style);

impl Prompt for i64 {
    fn prompt() -> Option<&'static str> {
        Some("Please enter an integer:")
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
impl_integer_elicit!(u8, U8Style);
impl_integer_elicit!(u16, U16Style);
impl_integer_elicit!(u32, U32Style);
impl_integer_elicit!(u64, U64Style);
impl_integer_elicit!(u128, U128Style);
impl_integer_elicit!(usize, UsizeStyle);
