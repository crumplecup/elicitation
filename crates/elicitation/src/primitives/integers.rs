//! Integer type implementations using generic macros.

use crate::{mcp, Elicit, ElicitResult, Prompt};

/// Macro to implement Elicit for all integer types.
///
/// This macro generates both Prompt and Elicit trait implementations for
/// a given integer type. It uses the type's MIN and MAX constants for
/// range validation.
macro_rules! impl_integer_elicit {
    ($t:ty) => {
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

        impl Elicit for $t {
            #[tracing::instrument(skip(client), fields(type_name = stringify!($t)))]
            async fn elicit<T: pmcp::shared::transport::Transport>(
                client: &pmcp::Client<T>,
            ) -> ElicitResult<Self> {
                let prompt = Self::prompt().unwrap();
                tracing::debug!("Eliciting integer type");

                let params = mcp::number_params(prompt, <$t>::MIN as i64, <$t>::MAX as i64);

                let result = client
                    .call_tool(mcp::tool_names::elicit_number(), params)
                    .await?;

                let value = mcp::extract_value(result)?;
                mcp::parse_integer::<$t>(value)
            }
        }
    };
}

// Apply macro to all signed integer types
impl_integer_elicit!(i8);
impl_integer_elicit!(i16);
impl_integer_elicit!(i32);
impl_integer_elicit!(i64);

// Apply macro to all unsigned integer types
impl_integer_elicit!(u8);
impl_integer_elicit!(u16);
impl_integer_elicit!(u32);
impl_integer_elicit!(u64);
