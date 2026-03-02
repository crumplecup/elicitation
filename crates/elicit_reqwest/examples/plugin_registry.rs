//! Example: Full plugin registry with all elicit_reqwest types
//!
//! Demonstrates registering all six plugin types and previewing their tools.
//!
//! Run with:
//! ```bash
//! cargo run --example plugin_registry -p elicit_reqwest
//! ```

use elicit_reqwest::plugins::{
    HeaderMapPlugin, MethodPlugin, Plugin, RequestBuilderPlugin, StatusCodePlugin, UrlPlugin,
    WorkflowPlugin,
};
use elicitation::{ElicitPlugin, PluginRegistry};

#[tokio::main]
async fn main() {
    let plugins: Vec<(&str, Box<dyn ElicitPlugin>)> = vec![
        ("http", Box::new(Plugin::new())),
        ("status_code", Box::new(StatusCodePlugin)),
        ("url", Box::new(UrlPlugin)),
        ("method", Box::new(MethodPlugin)),
        ("header_map", Box::new(HeaderMapPlugin)),
        ("request_builder", Box::new(RequestBuilderPlugin::new())),
        ("workflow", Box::new(WorkflowPlugin::default_client())),
    ];

    let mut total = 0usize;
    for (ns, plugin) in &plugins {
        let tools = plugin.list_tools();
        total += tools.len();
        println!("[{ns}] {} tools:", tools.len());
        for tool in &tools {
            println!(
                "  {ns}__{}: {}",
                tool.name,
                tool.description.as_deref().unwrap_or("")
            );
        }
    }
    println!("\n{total} tools across {} plugins", plugins.len());

    // Build the full registry
    let registry = PluginRegistry::new()
        .register("http", Plugin::new())
        .register("status_code", StatusCodePlugin)
        .register("url", UrlPlugin)
        .register("method", MethodPlugin)
        .register("header_map", HeaderMapPlugin)
        .register("request_builder", RequestBuilderPlugin::new())
        .register("workflow", WorkflowPlugin::default_client());

    // Curate a safe read-only toolchain
    let _toolchain = registry.filter(|t| {
        matches!(
            t.name.as_ref(),
            "http__get" | "http__head" | "url__parse" | "url__join"
        )
    });

    println!("\n✓ Plugin registry example passed.");
}
