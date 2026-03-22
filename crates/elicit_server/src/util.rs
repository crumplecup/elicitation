#[cfg(feature = "emit")]
use rmcp::model::{CallToolRequestParams, Tool};
#[cfg(feature = "emit")]
use schemars::JsonSchema;
#[cfg(feature = "emit")]
use serde::de::DeserializeOwned;
#[cfg(feature = "emit")]
use std::sync::Arc;

#[cfg(feature = "emit")]
pub(crate) fn parse_args<T: DeserializeOwned>(
    params: &CallToolRequestParams,
) -> Result<T, rmcp::ErrorData> {
    let value = serde_json::Value::Object(params.arguments.clone().unwrap_or_default());
    serde_json::from_value(value).map_err(|e| rmcp::ErrorData::invalid_params(e.to_string(), None))
}

#[cfg(feature = "emit")]
pub(crate) fn typed_tool<T: JsonSchema + 'static>(
    name: &'static str,
    description: &'static str,
) -> Tool {
    Tool::new(name, description, Arc::new(Default::default())).with_input_schema::<T>()
}
