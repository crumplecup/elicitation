//! MCP tool schemas and parameter builders.

/// MCP tool names used by elicitation.
pub mod tool_names {
    /// Tool for eliciting numeric values with range constraints.
    #[tracing::instrument(level = "trace")]
    pub fn elicit_number() -> String {
        "elicit_number".to_string()
    }

    /// Tool for eliciting boolean values (yes/no).
    #[tracing::instrument(level = "trace")]
    pub fn elicit_bool() -> String {
        "elicit_bool".to_string()
    }

    /// Tool for eliciting free-form text.
    #[tracing::instrument(level = "trace")]
    pub fn elicit_text() -> String {
        "elicit_text".to_string()
    }

    /// Tool for selecting from finite options.
    #[tracing::instrument(level = "trace")]
    pub fn elicit_select() -> String {
        "elicit_select".to_string()
    }

    /// Tool for multi-field surveys.
    #[tracing::instrument(level = "trace")]
    pub fn elicit_survey() -> String {
        "elicit_survey".to_string()
    }
}

/// Build parameters for elicit_number tool.
///
/// # Arguments
///
/// * `prompt` - The prompt to display to the user
/// * `min` - Minimum valid value (inclusive)
/// * `max` - Maximum valid value (inclusive)
///
/// # Returns
///
/// JSON object with prompt, min, and max fields.
pub fn number_params(
    prompt: &str,
    min: i64,
    max: i64,
) -> serde_json::Map<String, serde_json::Value> {
    serde_json::json!({
        "prompt": prompt,
        "min": min,
        "max": max,
    })
    .as_object()
    .unwrap()
    .clone()
}

/// Build parameters for elicit_bool tool.
///
/// # Arguments
///
/// * `prompt` - The prompt to display to the user
///
/// # Returns
///
/// JSON object with prompt field.
pub fn bool_params(prompt: &str) -> serde_json::Map<String, serde_json::Value> {
    serde_json::json!({ "prompt": prompt })
        .as_object()
        .unwrap()
        .clone()
}

/// Build parameters for elicit_text tool.
///
/// # Arguments
///
/// * `prompt` - The prompt to display to the user
///
/// # Returns
///
/// JSON object with prompt field.
pub fn text_params(prompt: &str) -> serde_json::Map<String, serde_json::Value> {
    serde_json::json!({ "prompt": prompt })
        .as_object()
        .unwrap()
        .clone()
}

/// Build parameters for elicit_select tool.
///
/// # Arguments
///
/// * `prompt` - The prompt to display to the user
/// * `options` - Array of valid option labels
///
/// # Returns
///
/// JSON object with prompt and options fields.
pub fn select_params(prompt: &str, options: &[&str]) -> serde_json::Map<String, serde_json::Value> {
    serde_json::json!({
        "prompt": prompt,
        "options": options,
    })
    .as_object()
    .unwrap()
    .clone()
}
