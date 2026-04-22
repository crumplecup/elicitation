//! Tools for working with `toml_edit::Key` and `KeyMut`.

use elicitation::elicit_tool;
use rmcp::{ErrorData, model::CallToolResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::plugin::{err_text, ok_json, ok_text};

// ── Key::new snippet ─────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct KeyNewSnippetParams {
    /// The key string.
    pub key: String,
}

/// Return a Rust snippet for constructing a `toml_edit::Key`.
#[elicit_tool(
    plugin = "toml",
    name = "toml__key__new_snippet",
    description = "Return a Rust snippet for constructing a toml_edit::Key using Key::new."
)]
async fn key_new_snippet(p: KeyNewSnippetParams) -> Result<CallToolResult, ErrorData> {
    ok_text(format!("let key = toml_edit::Key::new(\"{}\");", p.key))
}

// ── Key: parse ───────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct KeyParseParams {
    /// Key string to parse (may be dotted, e.g. `"a.b.c"`).
    pub key: String,
}

/// Parse a TOML key string and return its display representation.
///
/// Returns the key as `toml_edit` would display it (handles quoting).
#[elicit_tool(
    plugin = "toml",
    name = "toml__key__parse",
    description = "Parse a TOML key string and return its display representation. Handles dotted and quoted keys."
)]
async fn key_parse(p: KeyParseParams) -> Result<CallToolResult, ErrorData> {
    let wrapper = format!("{} = 0", p.key);
    let doc: toml_edit::DocumentMut = wrapper
        .parse()
        .map_err(|e| ErrorData::invalid_params(format!("key parse error: {}", e), None))?;
    let keys: Vec<String> = doc.iter().map(|(k, _)| k.to_string()).collect();
    ok_json(&keys)
}

// ── Key: display_repr ────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct KeyDisplayReprParams {
    /// Key string to display.
    pub key: String,
    /// If true, emit the quoted form always; if false, use bare form when safe.
    #[serde(default)]
    pub force_quoted: bool,
}

/// Return the TOML display representation for a key.
#[elicit_tool(
    plugin = "toml",
    name = "toml__key__display_repr",
    description = "Return the TOML representation for a key: bare if safe (e.g. 'foo'), quoted otherwise (e.g. '\"foo-bar\"')."
)]
async fn key_display_repr(p: KeyDisplayReprParams) -> Result<CallToolResult, ErrorData> {
    // Determine whether the key needs quoting
    let needs_quote = p.force_quoted
        || p.key.is_empty()
        || !p
            .key
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_');
    if needs_quote {
        ok_text(format!("\"{}\"", p.key))
    } else {
        ok_text(p.key)
    }
}

// ── Key: dotted reference ────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct KeyDottedReferenceParams {}

/// Return a reference explanation for dotted keys in TOML and `toml_edit`.
#[elicit_tool(
    plugin = "toml",
    name = "toml__key__dotted_reference",
    description = "Return an explanation of dotted keys in TOML (a.b.c = 1) and how they are represented in toml_edit."
)]
async fn key_dotted_reference(_p: KeyDottedReferenceParams) -> Result<CallToolResult, ErrorData> {
    ok_text(
        "Dotted keys in TOML (a.b.c = 1) create an implicit nested table structure.\n\
         \n\
         In toml_edit, dotted keys are represented as a chain of Key objects.\n\
         A Table with set_dotted(true) renders as dotted keys rather than a [section] header.\n\
         \n\
         To create a dotted-key entry programmatically:\n\
         ```rust\n\
         let mut doc = DocumentMut::new();\n\
         // Approach 1: parse it\n\
         let parsed: DocumentMut = \"a.b.c = 1\".parse().unwrap();\n\
         // Approach 2: build from table\n\
         let mut inner = Table::new();\n\
         inner.set_dotted(true);\n\
         inner.insert(\"c\", Item::Value(Value::Integer(Formatted::new(1))));\n\
         let mut outer = Table::new();\n\
         outer.set_dotted(true);\n\
         outer.insert(\"b\", Item::Table(inner));\n\
         doc.insert(\"a\", Item::Table(outer));\n\
         ```\n\
         \n\
         Key::default_repr() returns the display representation toml_edit will use.\n\
         Key::leaf_decor() and Key::dotted_decor() give access to the whitespace/comment\n\
         decoration around the leaf and dotted segments respectively.",
    )
}

// ── Key: snippet for document path access ────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct KeyPathSnippetParams {
    /// Dot-separated path, e.g. `"server.host"`.
    pub path: String,
}

/// Return a Rust snippet for accessing a nested key path in a `DocumentMut`.
#[elicit_tool(
    plugin = "toml",
    name = "toml__key__path_access_snippet",
    description = "Return a Rust snippet for accessing a nested key path in a DocumentMut using chained index operations."
)]
async fn key_path_access_snippet(p: KeyPathSnippetParams) -> Result<CallToolResult, ErrorData> {
    let segments: Vec<&str> = p.path.split('.').collect();
    if segments.is_empty() {
        return err_text("path must not be empty");
    }
    let chain = segments
        .iter()
        .map(|s| format!("[\"{}\"]", s))
        .collect::<Vec<_>>()
        .join("");
    ok_text(format!(
        "// Access path: {}\n\
         let value = doc{};\n\
         // Or with mut access:\n\
         let value_mut = doc{};",
        p.path, chain, chain,
    ))
}

// ── KeyMut reference ─────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct KeyMutReferenceParams {}

/// Return a reference explanation for `KeyMut` in `toml_edit`.
#[elicit_tool(
    plugin = "toml",
    name = "toml__key__key_mut_reference",
    description = "Explain KeyMut in toml_edit: how to mutate a key's decor and repr in-place while iterating."
)]
async fn key_mut_reference(_p: KeyMutReferenceParams) -> Result<CallToolResult, ErrorData> {
    ok_text(
        "KeyMut is returned by Table::iter_mut() and Table::get_key_value_mut().\n\
         It gives mutable access to the key's Decor and Repr without moving the entry.\n\
         \n\
         ```rust\n\
         for (mut key, value) in table.iter_mut() {\n\
             // Set a comment before the key\n\
             key.leaf_decor_mut().set_prefix(\" # my comment\\n\");\n\
         }\n\
         ```\n\
         \n\
         KeyMut does NOT allow changing the key's string value — use remove+insert for that.\n\
         Use KeyMut when you only want to adjust formatting (whitespace, comments, repr).",
    )
}

// ── List active documents/tables/arrays ──────────────────────────────────────

use crate::plugin::TomlCtx;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListContextParams {}

/// List all live UUIDs in the TOML plugin context (documents, tables, arrays, inline_tables).
#[elicit_tool(
    plugin = "toml",
    name = "toml__context__list",
    description = "List all live document_id, table_id, array_id, and inline_table_id UUIDs in the TOML plugin context."
)]
async fn list_context(
    ctx: Arc<TomlCtx>,
    _p: ListContextParams,
) -> Result<CallToolResult, ErrorData> {
    let docs: Vec<String> = ctx
        .documents
        .lock()
        .unwrap()
        .keys()
        .map(|u| u.to_string())
        .collect();
    let tables: Vec<String> = ctx
        .tables
        .lock()
        .unwrap()
        .keys()
        .map(|u| u.to_string())
        .collect();
    let arrays: Vec<String> = ctx
        .arrays
        .lock()
        .unwrap()
        .keys()
        .map(|u| u.to_string())
        .collect();
    let inline_tables: Vec<String> = ctx
        .inline_tables
        .lock()
        .unwrap()
        .keys()
        .map(|u| u.to_string())
        .collect();
    ok_json(&serde_json::json!({
        "documents": docs,
        "tables": tables,
        "arrays": arrays,
        "inline_tables": inline_tables,
    }))
}
