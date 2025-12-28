//! Response parsing helpers for MCP tool results.

use crate::{ElicitError, ElicitErrorKind, ElicitResult};
use serde_json::Value;

/// Extract a Value from a pmcp CallToolResult.
///
/// MCP tools return CallToolResult which contains content. This function
/// extracts the first text content and attempts to parse it as JSON.
///
/// # Arguments
///
/// * `result` - The result from a pmcp call_tool invocation
///
/// # Returns
///
/// A `serde_json::Value` representing the tool's response.
///
/// # Errors
///
/// Returns `ElicitError` if the result is empty or cannot be parsed.
pub fn extract_value(result: pmcp::types::protocol::CallToolResult) -> ElicitResult<Value> {
    let text = result
        .content
        .into_iter()
        .find_map(|c| match c {
            pmcp::types::protocol::Content::Text { text } => Some(text),
            _ => None,
        })
        .ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::InvalidFormat {
                expected: "text content".to_string(),
                received: "empty or non-text response".to_string(),
            })
        })?;

    // Try to parse as JSON first, fallback to string
    serde_json::from_str(&text).or_else(|_| Ok(Value::String(text)))
}

/// Parse an integer from MCP tool response.
///
/// Handles both JSON numbers and string representations. Validates that
/// the value fits within the target type's range.
///
/// # Type Parameters
///
/// * `T` - Target integer type (must support TryFrom<i64>)
///
/// # Arguments
///
/// * `raw` - The raw value from the MCP tool
///
/// # Returns
///
/// The parsed integer value, or an error if parsing fails or the value
/// is out of range.
///
/// # Errors
///
/// Returns `ElicitError` with:
/// - `InvalidFormat` if the value is not a number or numeric string
/// - `OutOfRange` if the value doesn't fit in the target type
pub fn parse_integer<T>(raw: Value) -> ElicitResult<T>
where
    T: TryFrom<i64> + std::fmt::Display + Copy,
{
    match raw {
        Value::Number(n) => {
            let v = n.as_i64().ok_or_else(|| {
                ElicitError::new(ElicitErrorKind::InvalidFormat {
                    expected: "integer".to_string(),
                    received: n.to_string(),
                })
            })?;
            T::try_from(v).map_err(|_| {
                ElicitError::new(ElicitErrorKind::OutOfRange {
                    min: "type minimum".to_string(),
                    max: "type maximum".to_string(),
                })
            })
        }
        Value::String(s) => {
            let v: i64 = s.trim().parse().map_err(|_| {
                ElicitError::new(ElicitErrorKind::InvalidFormat {
                    expected: "integer".to_string(),
                    received: s.clone(),
                })
            })?;
            T::try_from(v).map_err(|_| {
                ElicitError::new(ElicitErrorKind::OutOfRange {
                    min: "type minimum".to_string(),
                    max: "type maximum".to_string(),
                })
            })
        }
        _ => Err(ElicitError::new(ElicitErrorKind::InvalidFormat {
            expected: "number or string".to_string(),
            received: format!("{:?}", raw),
        })),
    }
}

/// Parse a boolean from MCP tool response.
///
/// Handles JSON booleans and common yes/no string variants.
///
/// # Arguments
///
/// * `raw` - The raw value from the MCP tool
///
/// # Returns
///
/// The parsed boolean value.
///
/// # Errors
///
/// Returns `ElicitError` with `InvalidFormat` if the value cannot be
/// interpreted as a boolean.
///
/// # Accepted Values
///
/// - JSON `true` or `false`
/// - Strings: "yes", "y", "true", "t", "1" (case-insensitive) → true
/// - Strings: "no", "n", "false", "f", "0" (case-insensitive) → false
pub fn parse_bool(raw: Value) -> ElicitResult<bool> {
    match raw {
        Value::Bool(b) => Ok(b),
        Value::String(s) => {
            let normalized = s.trim().to_lowercase();
            match normalized.as_str() {
                "yes" | "y" | "true" | "t" | "1" => Ok(true),
                "no" | "n" | "false" | "f" | "0" => Ok(false),
                _ => Err(ElicitError::new(ElicitErrorKind::InvalidFormat {
                    expected: "yes/no".to_string(),
                    received: s,
                })),
            }
        }
        _ => Err(ElicitError::new(ElicitErrorKind::InvalidFormat {
            expected: "boolean or yes/no string".to_string(),
            received: format!("{:?}", raw),
        })),
    }
}

/// Parse a string from MCP tool response.
///
/// # Arguments
///
/// * `raw` - The raw value from the MCP tool
///
/// # Returns
///
/// The string value.
///
/// # Errors
///
/// Returns `ElicitError` with `InvalidFormat` if the value is not a string.
pub fn parse_string(raw: Value) -> ElicitResult<String> {
    match raw {
        Value::String(s) => Ok(s),
        _ => Err(ElicitError::new(ElicitErrorKind::InvalidFormat {
            expected: "string".to_string(),
            received: format!("{:?}", raw),
        })),
    }
}
