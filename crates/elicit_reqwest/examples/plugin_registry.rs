//! Example: HTTP plugin registry via `elicit_reqwest`
//!
//! Demonstrates how to:
//! 1. Wrap an HTTP client in an `ElicitPlugin`
//! 2. Register it in a `PluginRegistry` under a namespace prefix
//! 3. Curate a `Toolchain` subset for an agent to use
//!
//! In production you would call `registry.serve(rmcp::transport::stdio()).await?`
//! to start the MCP server.
//!
//! Run with:
//! ```bash
//! cargo run --example plugin_registry -p elicit_reqwest
//! ```

use elicit_reqwest::Plugin;
use elicitation::{ElicitPlugin, PluginRegistry};

#[tokio::main]
async fn main() {
    // ── 1. Inspect the plugin ────────────────────────────────────────────────
    let http_plugin = Plugin::new();
    println!("Plugin name : {}", http_plugin.name());
    println!("Tools provided by plugin:");
    for tool in http_plugin.list_tools() {
        println!(
            "  {}: {}",
            tool.name,
            tool.description.as_deref().unwrap_or("")
        );
    }

    // ── 2. Register in a PluginRegistry under the "http" namespace ───────────
    //
    // Tool names become `http__get`, `http__post`, etc.
    // Multiple plugins from different shadow crates can be registered side-by-side;
    // their tools are namespaced to avoid collisions.
    let registry = PluginRegistry::new().register("http", Plugin::new());

    // Preview the namespaced tool names.
    let all_tools: Vec<String> = Plugin::new()
        .list_tools()
        .into_iter()
        .map(|t| format!("http__{}", t.name))
        .collect();

    println!("\nAll tools in registry ({}):", all_tools.len());
    for name in &all_tools {
        println!("  {name}");
    }

    // ── 3. Curate a Toolchain (read-only subset) ────────────────────────────
    //
    // Developers pick exactly which tools agents may call.  This keeps the
    // agent's view small and formally manageable regardless of how many tools
    // the underlying registry exposes.
    let read_only: &[&str] = &["http__get", "http__head"];
    let _toolchain = registry.filter(|tool| read_only.contains(&tool.name.as_ref()));

    let visible: Vec<_> = all_tools
        .iter()
        .filter(|n| read_only.contains(&n.as_str()))
        .collect();

    println!("\nToolchain visible to agent ({} tools):", visible.len());
    for name in &visible {
        println!("  {name}");
    }

    assert_eq!(visible.len(), 2, "toolchain should expose exactly 2 tools");
    println!("\n✓ Plugin registry example passed.");
}
