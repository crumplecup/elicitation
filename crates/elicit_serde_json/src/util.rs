//! Shared utilities for plugin implementations.

use rmcp::{ErrorData, model::CallToolRequestParams};
use serde::de::DeserializeOwned;

pub(crate) fn parse_args<T: DeserializeOwned>(
    params: &CallToolRequestParams,
) -> Result<T, ErrorData> {
    let value = serde_json::Value::Object(params.arguments.clone().unwrap_or_default());
    serde_json::from_value(value).map_err(|e| ErrorData::invalid_params(e.to_string(), None))
}
