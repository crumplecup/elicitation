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
#[tracing::instrument(level = "debug")]
pub fn number_params(
    prompt: &str,
    min: i64,
    max: i64,
) -> serde_json::Map<String, serde_json::Value> {
    let mut map = serde_json::Map::new();
    map.insert(
        "prompt".to_string(),
        serde_json::Value::String(prompt.to_string()),
    );
    map.insert("min".to_string(), serde_json::Value::Number(min.into()));
    map.insert("max".to_string(), serde_json::Value::Number(max.into()));
    map
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
#[tracing::instrument(level = "debug")]
pub fn bool_params(prompt: &str) -> serde_json::Map<String, serde_json::Value> {
    let mut map = serde_json::Map::new();
    map.insert(
        "prompt".to_string(),
        serde_json::Value::String(prompt.to_string()),
    );
    map
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
#[tracing::instrument(level = "debug")]
pub fn text_params(prompt: &str) -> serde_json::Map<String, serde_json::Value> {
    let mut map = serde_json::Map::new();
    map.insert(
        "prompt".to_string(),
        serde_json::Value::String(prompt.to_string()),
    );
    map
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
#[tracing::instrument(level = "debug")]
pub fn select_params(prompt: &str, options: &[&str]) -> serde_json::Map<String, serde_json::Value> {
    let mut map = serde_json::Map::new();
    map.insert(
        "prompt".to_string(),
        serde_json::Value::String(prompt.to_string()),
    );
    map.insert(
        "options".to_string(),
        serde_json::Value::Array(
            options
                .iter()
                .map(|s| serde_json::Value::String((*s).to_string()))
                .collect(),
        ),
    );
    map
}
