//! Shared utilities used by all plugin implementations.

use rmcp::{
    ErrorData,
    model::{CallToolRequestParams, Tool},
};
use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use std::sync::Arc;

/// Deserialize tool arguments from the call params.
pub(crate) fn parse_args<T: DeserializeOwned>(
    params: &CallToolRequestParams,
) -> Result<T, ErrorData> {
    let value = serde_json::Value::Object(params.arguments.clone().unwrap_or_default());
    serde_json::from_value(value).map_err(|e| ErrorData::invalid_params(e.to_string(), None))
}

/// Build a [`Tool`] with a typed input schema.
pub(crate) fn typed_tool<T: JsonSchema + 'static>(
    name: &'static str,
    description: &'static str,
) -> Tool {
    Tool::new(name, description, Arc::new(Default::default())).with_input_schema::<T>()
}
