//! MCP tools for lazy type-spec exploration.
//!
//! Agents call `describe_type` to get a summary and list of available spec
//! categories for a type, then `explore_type` to drill into a specific
//! category. This is a "dictionary lookup" approach — agents request only
//! the spec context they need rather than having full contracts injected into
//! every prompt.
//!
//! # Tool Names
//!
//! | Tool | Input | Output |
//! |------|-------|--------|
//! | `type_spec__describe_type` | `type_name` | Summary + category list |
//! | `type_spec__explore_type`  | `type_name`, `category` | Entries for that category |
//!
//! # Usage Pattern
//!
//! ```text
//! Agent: describe_type("I32Positive")
//!   → "A positive i32 contract type. Categories: [requires (1), related (1)]"
//!
//! Agent: explore_type("I32Positive", "requires")
//!   → label: "positive", expr: "value > 0", desc: "Value must be > 0."
//! ```

use futures::future::BoxFuture;
use rmcp::{
    ErrorData,
    model::{CallToolRequestParams, CallToolResult, Content, Tool},
    service::RequestContext,
};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{
    plugin::{ArcPlugin, ElicitPlugin},
    rmcp::RoleServer,
    type_spec::lookup_type_spec,
};

// ── Parameter types ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DescribeTypeParams {
    /// The exact registered name of the type (e.g., "I32Positive", "String", "f64").
    type_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExploreTypeParams {
    /// The exact registered name of the type.
    type_name: String,
    /// The spec category to explore (e.g., "requires", "ensures", "bounds", "related").
    category: String,
}

// ── Tool implementations ──────────────────────────────────────────────────────

#[instrument(fields(type_name))]
fn describe_type(type_name: &str) -> String {
    match lookup_type_spec(type_name) {
        Some(spec) => {
            let mut out = format!("**{}**\n{}\n\n", spec.type_name(), spec.summary());
            out.push_str("**Available categories** (call `explore_type` for details):\n");
            for cat in spec.categories() {
                out.push_str(&format!(
                    "  - `{}` ({} entries)\n",
                    cat.name(),
                    cat.entries().len()
                ));
            }
            out
        }
        None => format!(
            "Type `{type_name}` not found in the spec registry.\n\
             Try one of the well-known names: `i32`, `String`, `I32Positive`, `VecNonEmpty`, etc.\n\
             Use `explore_type` with any registered type to browse its spec."
        ),
    }
}

#[instrument(fields(type_name, category))]
fn explore_type(type_name: &str, category: &str) -> String {
    let Some(spec) = lookup_type_spec(type_name) else {
        return format!(
            "Type `{type_name}` not found in the spec registry.\n\
             Call `describe_type(\"{type_name}\")` to check the exact registered name."
        );
    };

    let Some(cat) = spec.categories().iter().find(|c| c.name() == category) else {
        let available: Vec<String> = spec
            .categories()
            .iter()
            .map(|c| format!("`{}`", c.name()))
            .collect();
        return format!(
            "Category `{category}` not found for type `{type_name}`.\n\
             Available categories: {}\n\
             Call `explore_type(\"{type_name}\", \"<category>\")` to explore one.",
            available.join(", ")
        );
    };

    let mut out = format!("**{} / {}**\n\n", type_name, category);
    for entry in cat.entries() {
        out.push_str(&format!("**{}**: {}", entry.label(), entry.description()));
        if let Some(expr) = entry.expression() {
            out.push_str(&format!("\n  `{expr}`"));
        }
        out.push('\n');
    }
    out
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin exposing `describe_type` and `explore_type` tools.
///
/// Register this with a [`PluginRegistry`](crate::PluginRegistry) under the
/// `"type_spec"` namespace to make the tools available as
/// `type_spec__describe_type` and `type_spec__explore_type`.
///
/// # Example
///
/// ```rust,no_run
/// use elicitation::{PluginRegistry, TypeSpecPlugin};
///
/// let registry = PluginRegistry::new()
///     .register("type_spec", TypeSpecPlugin::new());
/// ```
#[derive(Debug, Clone, Default)]
pub struct TypeSpecPlugin;

impl TypeSpecPlugin {
    /// Create a new `TypeSpecPlugin`.
    pub fn new() -> Self {
        Self
    }

    /// Wrap this plugin in an `Arc<dyn ElicitPlugin>` for use with `PluginRegistry`.
    pub fn into_arc(self) -> ArcPlugin {
        use std::sync::Arc;
        Arc::new(self)
    }
}

fn make_describe_tool() -> Tool {
    use std::sync::Arc;
    let schema = serde_json::json!({
        "type": "object",
        "properties": {
            "type_name": {
                "type": "string",
                "description": "The exact registered name of the type (e.g. 'I32Positive', 'String', 'f64')."
            }
        },
        "required": ["type_name"]
    });
    let schema_obj = match schema {
        serde_json::Value::Object(m) => Arc::new(m),
        _ => unreachable!(),
    };
    Tool::new(
        "describe_type",
        "Get a summary and list of available spec categories for a type. \
         Call this first to discover what spec information is available.",
        schema_obj,
    )
}

fn make_explore_tool() -> Tool {
    use std::sync::Arc;
    let schema = serde_json::json!({
        "type": "object",
        "properties": {
            "type_name": {
                "type": "string",
                "description": "The exact registered name of the type."
            },
            "category": {
                "type": "string",
                "description": "The spec category to explore (e.g. 'requires', 'ensures', 'bounds', 'related')."
            }
        },
        "required": ["type_name", "category"]
    });
    let schema_obj = match schema {
        serde_json::Value::Object(m) => Arc::new(m),
        _ => unreachable!(),
    };
    Tool::new(
        "explore_type",
        "Get the detailed spec entries for a specific category of a type. \
         Use describe_type first to discover available categories.",
        schema_obj,
    )
}

impl ElicitPlugin for TypeSpecPlugin {
    fn name(&self) -> &'static str {
        "type_spec"
    }

    fn list_tools(&self) -> Vec<Tool> {
        vec![make_describe_tool(), make_explore_tool()]
    }

    fn call_tool<'a>(
        &'a self,
        params: CallToolRequestParams,
        _ctx: RequestContext<RoleServer>,
    ) -> BoxFuture<'a, Result<CallToolResult, ErrorData>> {
        Box::pin(async move {
            let args = params.arguments.unwrap_or_default();
            let text = match params.name.as_ref() {
                "describe_type" => {
                    let p: DescribeTypeParams =
                        serde_json::from_value(serde_json::Value::Object(args)).map_err(|e| {
                            ErrorData::invalid_params(format!("describe_type params: {e}"), None)
                        })?;
                    describe_type(&p.type_name)
                }
                "explore_type" => {
                    let p: ExploreTypeParams =
                        serde_json::from_value(serde_json::Value::Object(args)).map_err(|e| {
                            ErrorData::invalid_params(format!("explore_type params: {e}"), None)
                        })?;
                    explore_type(&p.type_name, &p.category)
                }
                other => {
                    return Err(ErrorData::invalid_params(
                        format!("unknown tool `{other}`"),
                        None,
                    ));
                }
            };

            Ok(CallToolResult::success(vec![Content::text(text)]))
        })
    }
}
