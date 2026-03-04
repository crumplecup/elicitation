use rmcp::model::{CallToolRequestParams, Tool};
use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use std::sync::Arc;

pub(crate) fn parse_args<T: DeserializeOwned>(
    params: &CallToolRequestParams,
) -> Result<T, rmcp::ErrorData> {
    let value = serde_json::Value::Object(params.arguments.clone().unwrap_or_default());
    serde_json::from_value(value).map_err(|e| rmcp::ErrorData::invalid_params(e.to_string(), None))
}

pub(crate) fn typed_tool<T: JsonSchema + 'static>(
    name: &'static str,
    description: &'static str,
) -> Tool {
    Tool::new(name, description, Arc::new(Default::default())).with_input_schema::<T>()
}
