//! Floating-point type implementations using generic macros.

use crate::{ElicitClient, ElicitError, ElicitErrorKind, ElicitResult, Elicitation, Prompt, mcp};
use serde_json::Value;

/// Parse a floating-point number from MCP tool response.
///
/// Handles both JSON numbers and string representations.
///
/// # Type Parameters
///
/// * `T` - Target float type (f32 or f64)
///
/// # Arguments
///
/// * `raw` - The raw value from the MCP tool
///
/// # Returns
///
/// The parsed float value, or an error if parsing fails.
///
/// # Errors
///
/// Returns `ElicitError` with `InvalidFormat` if the value is not a number.
fn parse_float<T>(raw: Value) -> ElicitResult<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    match raw {
        Value::Number(n) => {
            let f64_val = n.as_f64().ok_or_else(|| {
                ElicitError::new(ElicitErrorKind::InvalidFormat {
                    expected: "float".to_string(),
                    received: n.to_string(),
                })
            })?;
            // Convert f64 to target type via string to handle precision
            f64_val.to_string().parse::<T>().map_err(|e| {
                ElicitError::new(ElicitErrorKind::InvalidFormat {
                    expected: "float".to_string(),
                    received: e.to_string(),
                })
            })
        }
        Value::String(s) => s.trim().parse::<T>().map_err(|e| {
            ElicitError::new(ElicitErrorKind::InvalidFormat {
                expected: "float".to_string(),
                received: format!("{}: {}", s, e),
            })
        }),
        _ => Err(ElicitError::new(ElicitErrorKind::InvalidFormat {
            expected: "number or string".to_string(),
            received: format!("{:?}", raw),
        })),
    }
}

/// Macro to implement Elicitation for floating-point types.
///
/// This macro generates default style enum, Prompt, and Elicitation trait
/// implementations for f32 and f64.
macro_rules! impl_float_elicit {
    ($t:ty, $style:ident) => {
        // Generate default-only style enum
        crate::default_style!($t => $style);

        impl Prompt for $t {
            fn prompt() -> Option<&'static str> {
                Some(concat!("Please enter a ", stringify!($t), " number:"))
            }
        }

        impl Elicitation for $t {
            type Style = $style;

            #[tracing::instrument(skip(client), fields(type_name = stringify!($t)))]
            async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
                let prompt = Self::prompt().unwrap();
                tracing::debug!("Eliciting float type");

                let params = mcp::text_params(prompt);

                let result = client
                    .peer()
                    .call_tool(rmcp::model::CallToolRequestParam {
                        name: mcp::tool_names::elicit_text().into(),
                        arguments: Some(params),
                        task: None,
                    })
                    .await?;

                let value = mcp::extract_value(result)?;
                parse_float::<$t>(value)
            }
        }
    };
}

// Apply macro to floating-point types
impl_float_elicit!(f32, F32Style);
impl_float_elicit!(f64, F64Style);
