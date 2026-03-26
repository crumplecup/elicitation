//! MCP tools for runtime type graph inspection.
//!
//! Agents can enumerate registered types, render structural graphs, and
//! explore edge relationships — all without reading source code.
//!
//! # Tool Names
//!
//! | Tool | Input | Output |
//! |------|-------|--------|
//! | `type_graph__list_types` | *(none)* | Sorted list of registered type names |
//! | `type_graph__graph_type` | `root`, `format?`, `include_primitives?` | Mermaid or DOT graph |
//! | `type_graph__describe_edges` | `type_name` | Human-readable edge summary |
//!
//! # Usage Pattern
//!
//! ```text
//! Agent: list_types()
//!   → "ApplicationConfig, NetworkConfig, Role, ..."
//!
//! Agent: graph_type("ApplicationConfig")
//!   → "graph TD\n    ApplicationConfig[\"ApplicationConfig (survey)\"]\n ..."
//!
//! Agent: describe_edges("ApplicationConfig")
//!   → "ApplicationConfig (survey, 3 fields)\n  network → NetworkConfig\n  ..."
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
    type_graph::{
        DotRenderer, GraphRenderer, MermaidDirection, MermaidRenderer, TypeGraph,
        all_graphable_types, lookup_type_graph,
    },
};

// ── Parameter types ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GraphTypeParams {
    /// The exact registered name of the root type (e.g. `ApplicationConfig`).
    root: String,
    /// Output format: `"mermaid"` (default) or `"dot"`.
    #[serde(default = "default_format")]
    format: String,
    /// Include primitive and generic leaf nodes. Default: `false`.
    #[serde(default)]
    include_primitives: bool,
}

fn default_format() -> String {
    "mermaid".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DescribeEdgesParams {
    /// The exact registered name of the type to describe.
    type_name: String,
}

// ── Tool implementations ──────────────────────────────────────────────────────

#[instrument]
fn list_types() -> String {
    let types = all_graphable_types();
    if types.is_empty() {
        "No graphable types registered. \
         Enable the `graph` feature and use `#[derive(Elicit)]` on your types."
            .to_string()
    } else {
        format!(
            "{} registered graphable type(s):\n\n{}",
            types.len(),
            types.join("\n")
        )
    }
}

#[instrument(fields(root, format, include_primitives))]
fn graph_type(root: &str, format: &str, include_primitives: bool) -> String {
    let graph = match TypeGraph::from_root(root) {
        Ok(g) => g,
        Err(e) => {
            let registered = all_graphable_types();
            return format!(
                "Type `{root}` not found in the graph registry.\n\
                 Error: {e}\n\n\
                 Call `list_types` to see all registered types.\n\
                 Registered types ({}):\n{}",
                registered.len(),
                registered.join(", ")
            );
        }
    };

    match format {
        "dot" => DotRenderer {
            include_primitives,
            ..Default::default()
        }
        .render(&graph),
        _ => MermaidRenderer {
            direction: MermaidDirection::TopDown,
            include_primitives,
        }
        .render(&graph),
    }
}

#[instrument(fields(type_name))]
fn describe_edges(type_name: &str) -> String {
    let meta = match lookup_type_graph(type_name) {
        Some(m) => m,
        None => {
            return format!(
                "Type `{type_name}` not found in the graph registry.\n\
                 Call `list_types` to see all registered types."
            );
        }
    };

    let graph = match TypeGraph::from_root(type_name) {
        Ok(g) => g,
        Err(e) => return format!("Failed to build graph for `{type_name}`: {e}"),
    };

    let pattern = format!("{:?}", meta.pattern()).to_lowercase();
    let edges: Vec<_> = graph.edges.iter().filter(|e| e.from == type_name).collect();

    let mut out = format!(
        "**{type_name}** ({pattern}, {} connection(s))\n\n",
        edges.len()
    );

    if edges.is_empty() {
        out.push_str("  *(no outgoing edges — leaf type)*\n");
    } else {
        for edge in &edges {
            let target_kind = graph
                .nodes
                .get(&edge.to)
                .map(|n| format!(" [{:?}]", n.kind).to_lowercase())
                .unwrap_or_default();
            out.push_str(&format!(
                "  `{}` → `{}`{}\n",
                edge.label, edge.to, target_kind
            ));
        }
    }

    // For Select types, show variant detail
    if let crate::PatternDetails::Select { variants } = meta.details {
        out.push_str("\n**Variants:**\n");
        for v in &variants {
            if v.fields.is_empty() {
                out.push_str(&format!("  `{}` *(unit)*\n", v.label));
            } else {
                let field_names: Vec<&str> = v.fields.iter().map(|f| f.name).collect();
                out.push_str(&format!("  `{}` ({})\n", v.label, field_names.join(", ")));
            }
        }
    }

    out
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin exposing type graph inspection tools.
///
/// Exposes three tools under the `"type_graph"` namespace:
/// `type_graph__list_types`, `type_graph__graph_type`,
/// `type_graph__describe_edges`.
///
/// # Example
///
/// ```rust,no_run
/// use elicitation::{PluginRegistry, TypeGraphPlugin};
///
/// let registry = PluginRegistry::new()
///     .register("type_graph", TypeGraphPlugin::new());
/// ```
#[derive(Debug, Clone, Default)]
pub struct TypeGraphPlugin;

impl TypeGraphPlugin {
    /// Create a new `TypeGraphPlugin`.
    pub fn new() -> Self {
        Self
    }

    /// Wrap in an `Arc<dyn ElicitPlugin>` for use with `PluginRegistry`.
    pub fn into_arc(self) -> ArcPlugin {
        use std::sync::Arc;
        Arc::new(self)
    }
}

fn make_list_tool() -> Tool {
    use std::sync::Arc;
    let schema = serde_json::json!({
        "type": "object",
        "properties": {}
    });
    let schema_obj = match schema {
        serde_json::Value::Object(m) => Arc::new(m),
        _ => unreachable!(),
    };
    Tool::new(
        "list_types",
        "List all types registered in the elicitation type graph. \
         Call this first to discover what types can be visualized.",
        schema_obj,
    )
}

fn make_graph_tool() -> Tool {
    use std::sync::Arc;
    let schema = serde_json::json!({
        "type": "object",
        "properties": {
            "root": {
                "type": "string",
                "description": "The root type name to render (e.g. 'ApplicationConfig')."
            },
            "format": {
                "type": "string",
                "enum": ["mermaid", "dot"],
                "description": "Output format. 'mermaid' (default) or 'dot' (Graphviz)."
            },
            "include_primitives": {
                "type": "boolean",
                "description": "Include primitive/generic leaf nodes. Default: false."
            }
        },
        "required": ["root"]
    });
    let schema_obj = match schema {
        serde_json::Value::Object(m) => Arc::new(m),
        _ => unreachable!(),
    };
    Tool::new(
        "graph_type",
        "Render the structural type graph rooted at the given type. \
         Returns Mermaid or Graphviz DOT syntax showing how types compose. \
         Use list_types first to find registered type names.",
        schema_obj,
    )
}

fn make_describe_edges_tool() -> Tool {
    use std::sync::Arc;
    let schema = serde_json::json!({
        "type": "object",
        "properties": {
            "type_name": {
                "type": "string",
                "description": "The exact registered type name to describe."
            }
        },
        "required": ["type_name"]
    });
    let schema_obj = match schema {
        serde_json::Value::Object(m) => Arc::new(m),
        _ => unreachable!(),
    };
    Tool::new(
        "describe_edges",
        "Describe the outgoing field/variant connections for a type in plain text. \
         Shows field names, target types, and variant details for enum types.",
        schema_obj,
    )
}

impl ElicitPlugin for TypeGraphPlugin {
    fn name(&self) -> &'static str {
        "type_graph"
    }

    fn list_tools(&self) -> Vec<Tool> {
        vec![
            make_list_tool(),
            make_graph_tool(),
            make_describe_edges_tool(),
        ]
    }

    fn call_tool<'a>(
        &'a self,
        params: CallToolRequestParams,
        _ctx: RequestContext<RoleServer>,
    ) -> BoxFuture<'a, Result<CallToolResult, ErrorData>> {
        Box::pin(async move {
            let args = params.arguments.unwrap_or_default();
            let text = match params.name.as_ref() {
                "list_types" => list_types(),
                "graph_type" => {
                    let p: GraphTypeParams =
                        serde_json::from_value(serde_json::Value::Object(args)).map_err(|e| {
                            ErrorData::invalid_params(format!("graph_type params: {e}"), None)
                        })?;
                    graph_type(&p.root, &p.format, p.include_primitives)
                }
                "describe_edges" => {
                    let p: DescribeEdgesParams =
                        serde_json::from_value(serde_json::Value::Object(args)).map_err(|e| {
                            ErrorData::invalid_params(format!("describe_edges params: {e}"), None)
                        })?;
                    describe_edges(&p.type_name)
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
