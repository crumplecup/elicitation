//! serde_json::Value elicitation implementation.
//!
//! Available with the `serde_json` feature.

use crate::{
    ElicitClient, ElicitError, ElicitErrorKind, ElicitResult, Elicitation, Prompt, Select, mcp,
};
use serde_json::Value;

/// Maximum depth for recursive JSON elicitation.
const MAX_DEPTH: usize = 10;

/// JSON type selection for Value elicitation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum JsonType {
    Null,
    Bool,
    String,
    Number,
    Array,
    Object,
}

impl Prompt for JsonType {
    fn prompt() -> Option<&'static str> {
        Some("Select JSON type:")
    }
}

impl Select for JsonType {
    fn options() -> &'static [Self] {
        &[
            JsonType::Null,
            JsonType::Bool,
            JsonType::String,
            JsonType::Number,
            JsonType::Array,
            JsonType::Object,
        ]
    }

    fn labels() -> &'static [&'static str] {
        &["null", "boolean", "string", "number", "array", "object"]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "null" => Some(JsonType::Null),
            "boolean" => Some(JsonType::Bool),
            "string" => Some(JsonType::String),
            "number" => Some(JsonType::Number),
            "array" => Some(JsonType::Array),
            "object" => Some(JsonType::Object),
            _ => None,
        }
    }
}

// Style enums
crate::default_style!(JsonType => JsonTypeStyle);
crate::default_style!(Value => ValueStyle);

impl Elicitation for JsonType {
    type Style = JsonTypeStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let prompt = Self::prompt().unwrap();
        tracing::debug!("Eliciting JSON type selection");

        let params = mcp::select_params(prompt, Self::labels());
        let result = client
            .peer()
            .call_tool(rmcp::model::CallToolRequestParams {
                meta: None,
                name: mcp::tool_names::elicit_select().into(),
                arguments: Some(params),
                task: None,
            })
            .await?;

        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;

        Self::from_label(&label)
            .ok_or_else(|| ElicitError::new(ElicitErrorKind::InvalidSelection(label)))
    }
}

impl Prompt for Value {
    fn prompt() -> Option<&'static str> {
        Some("Enter JSON value:")
    }
}

impl Elicitation for Value {
    type Style = ValueStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        elicit_with_depth(client, 0).await
    }
}

/// Elicit a JSON Value with depth tracking.
#[tracing::instrument(skip(communicator), fields(depth))]
fn elicit_with_depth<'a>(
    client: &'a ElicitClient,
    depth: usize,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = ElicitResult<Value>> + Send + 'a>> {
    Box::pin(async move {
        if depth > MAX_DEPTH {
            return Err(ElicitError::new(ElicitErrorKind::RecursionDepthExceeded(
                MAX_DEPTH,
            )));
        }

        tracing::debug!(depth, "Eliciting JSON value");

        // Step 1: Select JSON type
        let json_type = JsonType::elicit(communicator).await?;
        tracing::debug!(?json_type, "JSON type selected");

        // Step 2: Elicit based on type
        match json_type {
            JsonType::Null => {
                tracing::debug!("Returning null");
                Ok(Value::Null)
            }
            JsonType::Bool => {
                tracing::debug!("Eliciting boolean");
                let b = bool::elicit(communicator).await?;
                Ok(Value::Bool(b))
            }
            JsonType::String => {
                tracing::debug!("Eliciting string");
                let s = String::elicit(communicator).await?;
                Ok(Value::String(s))
            }
            JsonType::Number => {
                tracing::debug!("Eliciting number");
                elicit_number(client).await
            }
            JsonType::Array => {
                tracing::debug!("Eliciting array");
                elicit_array(client, depth + 1).await
            }
            JsonType::Object => {
                tracing::debug!("Eliciting object");
                elicit_object(client, depth + 1).await
            }
        }
    })
}

/// Elicit a JSON number.
#[tracing::instrument(skip(communicator))]
async fn elicit_number(client: &ElicitClient) -> ElicitResult<Value> {
    let prompt = "Enter number (integer or decimal):";
    tracing::debug!("Eliciting number");

    let params = mcp::text_params(prompt);
    let result = client
        .peer()
        .call_tool(rmcp::model::CallToolRequestParams {
            meta: None,
            name: mcp::tool_names::elicit_text().into(),
            arguments: Some(params),
            task: None,
        })
        .await?;

    let value = mcp::extract_value(result)?;
    let text = mcp::parse_string(value)?;

    // Parse as f64 (covers i64/u64 range + decimals)
    let num: f64 = text.parse().map_err(|_| {
        ElicitError::new(ElicitErrorKind::ParseError(format!(
            "Invalid number: {}",
            text
        )))
    })?;

    Ok(serde_json::json!(num))
}

/// Elicit a JSON array.
#[tracing::instrument(skip(communicator), fields(depth))]
fn elicit_array<'a>(
    client: &'a ElicitClient,
    depth: usize,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = ElicitResult<Value>> + Send + 'a>> {
    Box::pin(async move {
        let mut items = Vec::new();

        loop {
            let prompt = if items.is_empty() {
                "Add item to array?"
            } else {
                "Add another item to array?"
            };

            tracing::debug!(count = items.len(), "Prompting to add array item");

            // Ask if user wants to add an item
            let params = mcp::bool_params(prompt);
            let result = client
                .peer()
                .call_tool(rmcp::model::CallToolRequestParams {
                    meta: None,
                    name: mcp::tool_names::elicit_bool().into(),
                    arguments: Some(params),
                    task: None,
                })
                .await?;

            let value = mcp::extract_value(result)?;
            let add_item = mcp::parse_bool(value)?;

            if !add_item {
                tracing::debug!(count = items.len(), "Array elicitation complete");
                break;
            }

            // Recursively elicit the item
            let item = elicit_with_depth(client, depth).await?;
            items.push(item);
            tracing::debug!(count = items.len(), "Item added to array");
        }

        Ok(Value::Array(items))
    })
}

/// Elicit a JSON object.
#[tracing::instrument(skip(communicator), fields(depth))]
fn elicit_object<'a>(
    client: &'a ElicitClient,
    depth: usize,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = ElicitResult<Value>> + Send + 'a>> {
    Box::pin(async move {
        let mut map = serde_json::Map::new();

        loop {
            let prompt = if map.is_empty() {
                "Add field to object?"
            } else {
                "Add another field to object?"
            };

            tracing::debug!(count = map.len(), "Prompting to add object field");

            // Ask if user wants to add a field
            let params = mcp::bool_params(prompt);
            let result = client
                .peer()
                .call_tool(rmcp::model::CallToolRequestParams {
                    meta: None,
                    name: mcp::tool_names::elicit_bool().into(),
                    arguments: Some(params),
                    task: None,
                })
                .await?;

            let value = mcp::extract_value(result)?;
            let add_field = mcp::parse_bool(value)?;

            if !add_field {
                tracing::debug!(count = map.len(), "Object elicitation complete");
                break;
            }

            // Elicit key
            let key_prompt = "Enter field name:";
            let key_params = mcp::text_params(key_prompt);
            let key_result = client
                .peer()
                .call_tool(rmcp::model::CallToolRequestParams {
                    meta: None,
                    name: mcp::tool_names::elicit_text().into(),
                    arguments: Some(key_params),
                    task: None,
                })
                .await?;

            let key_value = mcp::extract_value(key_result)?;
            let key = mcp::parse_string(key_value)?;

            // Recursively elicit the value
            let field_value = elicit_with_depth(client, depth).await?;
            map.insert(key.clone(), field_value);
            tracing::debug!(key = %key, count = map.len(), "Field added to object");
        }

        Ok(Value::Object(map))
    })
}
