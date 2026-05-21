//! Factory meta-tool generation for [`DynamicToolRegistry`](super::DynamicToolRegistry).
//!
//! Each [`AnyToolFactory`] gets one meta-tool automatically
//! registered in `list_tools`.  Agents call these to trigger instantiation of
//! dynamic tools for a specific registered prefix.
//!
//! # Naming convention
//!
//! `"my_crate::MyTrait"` → `"instantiate_my_crate__my_trait"`
//!
//! Double-underscores replace `::` separators so names stay valid in all MCP clients.

use rmcp::model::Tool;
use std::sync::Arc;

use super::AnyToolFactory;

/// Convert a fully-qualified trait name to the meta-tool name.
///
/// `"my_crate::MyTrait"` → `"instantiate_my_crate__my_trait"`
pub fn meta_tool_name(trait_name: &str) -> String {
    let snake = trait_name
        .replace("::", "__")
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if c.is_uppercase() {
                let lower = c.to_lowercase().next().unwrap_or(c);
                if i == 0 {
                    vec![lower]
                } else {
                    vec!['_', lower]
                }
            } else {
                vec![c]
            }
        })
        .collect::<String>();
    format!("instantiate_{snake}")
}

/// Build the rmcp [`Tool`] representing the factory meta-tool.
///
/// The schema is a simple `{ "prefix": "string" }` object.
pub fn make_meta_tool(factory: &dyn AnyToolFactory) -> Tool {
    let trait_name = factory.trait_name();
    let name = meta_tool_name(trait_name);
    let methods = factory.method_names().join(", ");
    let description = format!(
        "{desc} Call with {{\"prefix\": \"<registered_prefix>\"}} to instantiate tools: {methods}. \
         Register a type first with register_type::<T>(prefix) at server startup.",
        desc = factory.factory_description(),
    );

    // Inline schema: { type: "object", properties: { prefix: { type: "string" } }, required: ["prefix"] }
    let schema = serde_json::json!({
        "type": "object",
        "properties": {
            "prefix": {
                "type": "string",
                "description": "The prefix used in register_type::<T>(prefix) at startup"
            }
        },
        "required": ["prefix"]
    });

    let schema_obj = match schema {
        serde_json::Value::Object(m) => Arc::new(m),
        _ => Arc::new(Default::default()),
    };

    Tool::new(name, description, schema_obj)
}
