//! Tools for inspecting `toml_edit::Item` variants.

use elicitation::elicit_tool;
use rmcp::{ErrorData, model::CallToolResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::plugin::{err_text, ok_text};

// ── Item helpers operate on TOML fragment strings, not live UUIDs ─────────────
// This keeps these tools stateless: the caller parses a document, serializes
// an item to its TOML fragment, and passes that fragment here for inspection.

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ItemFromFragmentParams {
    /// TOML text fragment for the item, e.g. `42` or `"hello"` or `[1,2,3]`.
    pub value_toml: String,
}

/// Parse a TOML value fragment and return its type name.
///
/// Returns one of: `"integer"`, `"float"`, `"boolean"`, `"string"`, `"datetime"`,
/// `"array"`, `"inline_table"`, or an error.
#[elicit_tool(
    plugin = "toml",
    name = "toml__item__type_name",
    description = "Return the type name of a TOML value fragment: integer, float, boolean, string, datetime, array, or inline_table."
)]
async fn item_type_name(p: ItemFromFragmentParams) -> Result<CallToolResult, ErrorData> {
    let wrapper = format!("__v = {}", p.value_toml);
    let doc: toml_edit::DocumentMut = wrapper
        .parse()
        .map_err(|e| ErrorData::invalid_params(format!("parse error: {}", e), None))?;
    match doc.get("__v").and_then(|i| i.as_value()) {
        Some(v) => ok_text(value_type_name(v)),
        None => ok_text("table"),
    }
}

fn value_type_name(v: &toml_edit::Value) -> &'static str {
    match v {
        toml_edit::Value::String(_) => "string",
        toml_edit::Value::Integer(_) => "integer",
        toml_edit::Value::Float(_) => "float",
        toml_edit::Value::Boolean(_) => "boolean",
        toml_edit::Value::Datetime(_) => "datetime",
        toml_edit::Value::Array(_) => "array",
        toml_edit::Value::InlineTable(_) => "inline_table",
    }
}

// ── Type predicates ───────────────────────────────────────────────────────────

/// Return `true` if the TOML fragment is a scalar integer.
#[elicit_tool(
    plugin = "toml",
    name = "toml__item__is_integer",
    description = "Return true if the TOML value fragment is an integer."
)]
async fn item_is_integer(p: ItemIsIntegerParams) -> Result<CallToolResult, ErrorData> {
    ok_text(
        fragment_value_is(&p.value_toml, |v| matches!(v, toml_edit::Value::Integer(_))).to_string(),
    )
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ItemIsIntegerParams {
    /// TOML value fragment to test.
    pub value_toml: String,
}

/// Return `true` if the TOML fragment is a scalar float.
#[elicit_tool(
    plugin = "toml",
    name = "toml__item__is_float",
    description = "Return true if the TOML value fragment is a float."
)]
async fn item_is_float(p: ItemIsFloatParams) -> Result<CallToolResult, ErrorData> {
    ok_text(
        fragment_value_is(&p.value_toml, |v| matches!(v, toml_edit::Value::Float(_))).to_string(),
    )
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ItemIsFloatParams {
    /// TOML value fragment to test.
    pub value_toml: String,
}

/// Return `true` if the TOML fragment is a boolean.
#[elicit_tool(
    plugin = "toml",
    name = "toml__item__is_boolean",
    description = "Return true if the TOML value fragment is a boolean."
)]
async fn item_is_boolean(p: ItemIsBooleanParams) -> Result<CallToolResult, ErrorData> {
    ok_text(
        fragment_value_is(&p.value_toml, |v| matches!(v, toml_edit::Value::Boolean(_))).to_string(),
    )
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ItemIsBooleanParams {
    /// TOML value fragment to test.
    pub value_toml: String,
}

/// Return `true` if the TOML fragment is a string.
#[elicit_tool(
    plugin = "toml",
    name = "toml__item__is_string",
    description = "Return true if the TOML value fragment is a string."
)]
async fn item_is_string(p: ItemIsStringParams) -> Result<CallToolResult, ErrorData> {
    ok_text(
        fragment_value_is(&p.value_toml, |v| matches!(v, toml_edit::Value::String(_))).to_string(),
    )
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ItemIsStringParams {
    /// TOML value fragment to test.
    pub value_toml: String,
}

/// Return `true` if the TOML fragment is a datetime.
#[elicit_tool(
    plugin = "toml",
    name = "toml__item__is_datetime",
    description = "Return true if the TOML value fragment is a datetime."
)]
async fn item_is_datetime(p: ItemIsDatetimeParams) -> Result<CallToolResult, ErrorData> {
    ok_text(
        fragment_value_is(&p.value_toml, |v| {
            matches!(v, toml_edit::Value::Datetime(_))
        })
        .to_string(),
    )
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ItemIsDatetimeParams {
    /// TOML value fragment to test.
    pub value_toml: String,
}

/// Return `true` if the TOML fragment is an inline array.
#[elicit_tool(
    plugin = "toml",
    name = "toml__item__is_array",
    description = "Return true if the TOML value fragment is an array."
)]
async fn item_is_array(p: ItemIsArrayParams) -> Result<CallToolResult, ErrorData> {
    ok_text(
        fragment_value_is(&p.value_toml, |v| matches!(v, toml_edit::Value::Array(_))).to_string(),
    )
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ItemIsArrayParams {
    /// TOML value fragment to test.
    pub value_toml: String,
}

/// Return `true` if the TOML fragment is an inline table.
#[elicit_tool(
    plugin = "toml",
    name = "toml__item__is_inline_table",
    description = "Return true if the TOML value fragment is an inline table."
)]
async fn item_is_inline_table(p: ItemIsInlineTableParams) -> Result<CallToolResult, ErrorData> {
    ok_text(
        fragment_value_is(&p.value_toml, |v| {
            matches!(v, toml_edit::Value::InlineTable(_))
        })
        .to_string(),
    )
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ItemIsInlineTableParams {
    /// TOML value fragment to test.
    pub value_toml: String,
}

fn fragment_value_is(toml_fragment: &str, pred: impl Fn(&toml_edit::Value) -> bool) -> bool {
    let wrapper = format!("__v = {}", toml_fragment);
    wrapper
        .parse::<toml_edit::DocumentMut>()
        .ok()
        .and_then(|d| d.get("__v").and_then(|i| i.as_value()).map(pred))
        .unwrap_or(false)
}

// ── Value extraction ──────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ItemAsIntegerParams {
    /// TOML value fragment.
    pub value_toml: String,
}

/// Extract the integer from a TOML fragment. Returns the integer as a string or an error.
#[elicit_tool(
    plugin = "toml",
    name = "toml__item__as_integer",
    description = "Extract the integer value from a TOML fragment. Returns the integer as a string."
)]
async fn item_as_integer(p: ItemAsIntegerParams) -> Result<CallToolResult, ErrorData> {
    let wrapper = format!("__v = {}", p.value_toml);
    let doc: toml_edit::DocumentMut = wrapper
        .parse()
        .map_err(|e| ErrorData::invalid_params(format!("parse error: {}", e), None))?;
    match doc.get("__v").and_then(|i| i.as_integer()) {
        Some(n) => ok_text(n.to_string()),
        None => err_text("value is not an integer"),
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ItemAsFloatParams {
    /// TOML value fragment.
    pub value_toml: String,
}

/// Extract the float from a TOML fragment.
#[elicit_tool(
    plugin = "toml",
    name = "toml__item__as_float",
    description = "Extract the float value from a TOML fragment."
)]
async fn item_as_float(p: ItemAsFloatParams) -> Result<CallToolResult, ErrorData> {
    let wrapper = format!("__v = {}", p.value_toml);
    let doc: toml_edit::DocumentMut = wrapper
        .parse()
        .map_err(|e| ErrorData::invalid_params(format!("parse error: {}", e), None))?;
    match doc.get("__v").and_then(|i| i.as_float()) {
        Some(f) => ok_text(f.to_string()),
        None => err_text("value is not a float"),
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ItemAsBooleanParams {
    /// TOML value fragment.
    pub value_toml: String,
}

/// Extract the boolean from a TOML fragment.
#[elicit_tool(
    plugin = "toml",
    name = "toml__item__as_boolean",
    description = "Extract the boolean value from a TOML fragment."
)]
async fn item_as_boolean(p: ItemAsBooleanParams) -> Result<CallToolResult, ErrorData> {
    let wrapper = format!("__v = {}", p.value_toml);
    let doc: toml_edit::DocumentMut = wrapper
        .parse()
        .map_err(|e| ErrorData::invalid_params(format!("parse error: {}", e), None))?;
    match doc.get("__v").and_then(|i| i.as_bool()) {
        Some(b) => ok_text(b.to_string()),
        None => err_text("value is not a boolean"),
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ItemAsStrParams {
    /// TOML value fragment.
    pub value_toml: String,
}

/// Extract the string from a TOML fragment.
#[elicit_tool(
    plugin = "toml",
    name = "toml__item__as_str",
    description = "Extract the string value from a TOML fragment."
)]
async fn item_as_str(p: ItemAsStrParams) -> Result<CallToolResult, ErrorData> {
    let wrapper = format!("__v = {}", p.value_toml);
    let doc: toml_edit::DocumentMut = wrapper
        .parse()
        .map_err(|e| ErrorData::invalid_params(format!("parse error: {}", e), None))?;
    match doc.get("__v").and_then(|i| i.as_str()) {
        Some(s) => ok_text(s.to_string()),
        None => err_text("value is not a string"),
    }
}

// ── Document item type introspection ─────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DocumentItemTypeParams {
    /// Full TOML document text.
    pub toml_text: String,
    /// Key whose item type you want to inspect.
    pub key: String,
}

/// Return the item type for a key in a full TOML document.
///
/// Returns one of: `"value"`, `"table"`, `"array_of_tables"`, `"none"`.
#[elicit_tool(
    plugin = "toml",
    name = "toml__item__document_key_type",
    description = "Return the Item type for a key in a parsed TOML document: value, table, array_of_tables, or none."
)]
async fn document_key_type(p: DocumentItemTypeParams) -> Result<CallToolResult, ErrorData> {
    let doc: toml_edit::DocumentMut = p
        .toml_text
        .parse()
        .map_err(|e| ErrorData::invalid_params(format!("TOML parse error: {}", e), None))?;
    let label = match doc.get(&p.key) {
        None => "none",
        Some(item) if item.is_value() => "value",
        Some(item) if item.is_table() => "table",
        Some(item) if item.is_array_of_tables() => "array_of_tables",
        Some(_) => "none",
    };
    ok_text(label)
}

// ── Item as JSON ──────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DocumentItemAsJsonParams {
    /// Full TOML document text.
    pub toml_text: String,
    /// Key to extract.
    pub key: String,
}

/// Extract a key's item from a full TOML document and return it as JSON.
#[elicit_tool(
    plugin = "toml",
    name = "toml__item__as_json",
    description = "Extract a key from a parsed TOML document and return it as JSON-serialized Item."
)]
async fn document_item_as_json(p: DocumentItemAsJsonParams) -> Result<CallToolResult, ErrorData> {
    let doc: toml_edit::DocumentMut = p
        .toml_text
        .parse()
        .map_err(|e| ErrorData::invalid_params(format!("TOML parse error: {}", e), None))?;
    match doc.get(&p.key) {
        Some(item) => ok_text(item.to_string()),
        None => ok_text("null"),
    }
}
