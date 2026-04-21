//! `RedbTypesPlugin` — `Key`, `Value`, and `MutInPlaceValue` trait implementation skeletons.

use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

fn ok(text: impl Into<String>) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(text.into())]))
}

// ── params ────────────────────────────────────────────────────────────────────

/// Parameters for `redb_types__impl_key`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ImplKeyParams {
    /// Rust type name to implement `redb::Key` for.
    pub type_name: String,
}

/// Parameters for `redb_types__impl_value`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ImplValueParams {
    /// Rust type name to implement `redb::Value` for.
    pub type_name: String,
}

/// Parameters for `redb_types__impl_mut_in_place`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ImplMutInPlaceParams {
    /// Rust type name to implement `redb::MutInPlaceValue` for.
    pub type_name: String,
}

/// Parameters for `redb_types__derive_key_bincode`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeriveKeyBincodeParams {
    /// Rust type name to generate a bincode-based `redb::Key` impl for.
    pub type_name: String,
}

/// Parameters for `redb_types__derive_value_json`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeriveValueJsonParams {
    /// Rust type name to generate a serde_json-based `redb::Value` impl for.
    pub type_name: String,
}

/// Parameters for `redb_types__type_name`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TypeNameParams {
    /// Rust type name.
    pub type_name: String,
    /// String to use as the redb type name constant.
    pub name_str: String,
}

/// Parameters for `redb_types__fixed_width_key`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FixedWidthKeyParams {
    /// Rust type name for the fixed-width key struct.
    pub type_name: String,
    /// Number of bytes in the fixed-width representation.
    pub byte_width: usize,
}

// ── tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "redb_types",
    name = "redb_types__impl_key",
    description = "Emit a `impl redb::Key for MyType` skeleton requiring Ord, `as_bytes`, and `from_bytes`."
)]
#[instrument]
async fn redb_types_impl_key(p: ImplKeyParams) -> Result<CallToolResult, ErrorData> {
    let t = &p.type_name;
    ok(format!(
        "// {t} must implement Ord (and therefore PartialOrd, Eq, PartialEq).\n\
impl redb::Key for {t} {{\n\
    fn compare(data1: &[u8], data2: &[u8]) -> ::std::cmp::Ordering {{\n\
        let a = Self::from_bytes(data1);\n\
        let b = Self::from_bytes(data2);\n\
        a.cmp(&b)\n\
    }}\n\
}}\n\
\n\
impl redb::Value for {t} {{\n\
    type SelfType<'a> = Self;\n\
    type AsBytes<'a> = Vec<u8>;\n\
\n\
    fn fixed_width() -> Option<usize> {{\n\
        None // set to Some(N) for fixed-width types\n\
    }}\n\
\n\
    fn from_bytes<'a>(data: &'a [u8]) -> Self {{\n\
        todo!(\"deserialise {t} from bytes\")\n\
    }}\n\
\n\
    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {{\n\
        todo!(\"serialise {t} to bytes\")\n\
    }}\n\
\n\
    fn type_name() -> redb::TypeName {{\n\
        redb::TypeName::new(\"{t}\")\n\
    }}\n\
}}"
    ))
}

#[elicit_tool(
    plugin = "redb_types",
    name = "redb_types__impl_value",
    description = "Emit a `impl redb::Value for MyType` skeleton with `from_bytes`, `as_bytes`, and `type_name`."
)]
#[instrument]
async fn redb_types_impl_value(p: ImplValueParams) -> Result<CallToolResult, ErrorData> {
    let t = &p.type_name;
    ok(format!(
        "impl redb::Value for {t} {{\n\
    type SelfType<'a> = Self;\n\
    type AsBytes<'a> = Vec<u8>;\n\
\n\
    fn fixed_width() -> Option<usize> {{\n\
        None // set to Some(N) for fixed-width types\n\
    }}\n\
\n\
    fn from_bytes<'a>(data: &'a [u8]) -> Self {{\n\
        todo!(\"deserialise {t} from bytes\")\n\
    }}\n\
\n\
    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {{\n\
        todo!(\"serialise {t} to bytes\")\n\
    }}\n\
\n\
    fn type_name() -> redb::TypeName {{\n\
        redb::TypeName::new(\"{t}\")\n\
    }}\n\
}}"
    ))
}

#[elicit_tool(
    plugin = "redb_types",
    name = "redb_types__impl_mut_in_place",
    description = "Emit a `impl redb::MutInPlaceValue for MyType` skeleton for mutable in-place updates."
)]
#[instrument]
async fn redb_types_impl_mut_in_place(
    p: ImplMutInPlaceParams,
) -> Result<CallToolResult, ErrorData> {
    let t = &p.type_name;
    ok(format!(
        "impl redb::MutInPlaceValue for {t} {{\n\
    type BaseRefType = Self;\n\
\n\
    unsafe fn from_bytes_mut(data: &mut [u8]) -> &mut Self {{\n\
        todo!(\"return a &mut {t} backed by the provided byte slice\")\n\
    }}\n\
}}"
    ))
}

#[elicit_tool(
    plugin = "redb_types",
    name = "redb_types__derive_key_bincode",
    description = "Emit a complete bincode-based `redb::Key + redb::Value` impl using `bincode::encode_to_vec` / `decode_from_slice`."
)]
#[instrument]
async fn redb_types_derive_key_bincode(
    p: DeriveKeyBincodeParams,
) -> Result<CallToolResult, ErrorData> {
    let t = &p.type_name;
    ok(format!(
        "// Requires: bincode = {{ version = \"2\", features = [\"serde\"] }}, serde on {t}\n\
impl redb::Value for {t} {{\n\
    type SelfType<'a> = Self;\n\
    type AsBytes<'a> = Vec<u8>;\n\
\n\
    fn fixed_width() -> Option<usize> {{ None }}\n\
\n\
    fn from_bytes<'a>(data: &'a [u8]) -> Self {{\n\
        let (v, _) = bincode::decode_from_slice(data, bincode::config::standard())\n\
            .expect(\"bincode decode\");\n\
        v\n\
    }}\n\
\n\
    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {{\n\
        bincode::encode_to_vec(value, bincode::config::standard())\n\
            .expect(\"bincode encode\")\n\
    }}\n\
\n\
    fn type_name() -> redb::TypeName {{\n\
        redb::TypeName::new(\"{t}\")\n\
    }}\n\
}}\n\
\n\
impl redb::Key for {t} {{\n\
    fn compare(data1: &[u8], data2: &[u8]) -> ::std::cmp::Ordering {{\n\
        let a = Self::from_bytes(data1);\n\
        let b = Self::from_bytes(data2);\n\
        a.cmp(&b)\n\
    }}\n\
}}"
    ))
}

#[elicit_tool(
    plugin = "redb_types",
    name = "redb_types__derive_value_json",
    description = "Emit a serde_json-based `redb::Value` impl using `serde_json::to_vec` / `from_slice`."
)]
#[instrument]
async fn redb_types_derive_value_json(
    p: DeriveValueJsonParams,
) -> Result<CallToolResult, ErrorData> {
    let t = &p.type_name;
    ok(format!(
        "// Requires: serde_json, serde Deserialize + Serialize on {t}\n\
impl redb::Value for {t} {{\n\
    type SelfType<'a> = Self;\n\
    type AsBytes<'a> = Vec<u8>;\n\
\n\
    fn fixed_width() -> Option<usize> {{ None }}\n\
\n\
    fn from_bytes<'a>(data: &'a [u8]) -> Self {{\n\
        serde_json::from_slice(data).expect(\"json decode\")\n\
    }}\n\
\n\
    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {{\n\
        serde_json::to_vec(value).expect(\"json encode\")\n\
    }}\n\
\n\
    fn type_name() -> redb::TypeName {{\n\
        redb::TypeName::new(\"{t}\")\n\
    }}\n\
}}"
    ))
}

#[elicit_tool(
    plugin = "redb_types",
    name = "redb_types__type_name",
    description = "Emit a `redb::TypeName::new(...)` expression or a type-name string constant."
)]
#[instrument]
async fn redb_types_type_name(p: TypeNameParams) -> Result<CallToolResult, ErrorData> {
    ok(format!(
        "// Used inside a redb::Value impl:\nredb::TypeName::new({name:?})\n\n// Or as a module-level constant:\nconst {upper}_TYPE_NAME: &str = {name:?};",
        name = p.name_str,
        upper = p.type_name.to_uppercase(),
    ))
}

#[elicit_tool(
    plugin = "redb_types",
    name = "redb_types__fixed_width_key",
    description = "Emit a fixed-width key type backed by a byte array, with `redb::Key + redb::Value` impl."
)]
#[instrument]
async fn redb_types_fixed_width_key(p: FixedWidthKeyParams) -> Result<CallToolResult, ErrorData> {
    let t = &p.type_name;
    let w = p.byte_width;
    ok(format!(
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]\n\
pub struct {t}([u8; {w}]);\n\
\n\
impl {t} {{\n\
    pub fn new(bytes: [u8; {w}]) -> Self {{ Self(bytes) }}\n\
    pub fn as_bytes(&self) -> &[u8; {w}] {{ &self.0 }}\n\
}}\n\
\n\
impl redb::Value for {t} {{\n\
    type SelfType<'a> = Self;\n\
    type AsBytes<'a> = [u8; {w}];\n\
\n\
    fn fixed_width() -> Option<usize> {{ Some({w}) }}\n\
\n\
    fn from_bytes<'a>(data: &'a [u8]) -> Self {{\n\
        let mut arr = [0u8; {w}];\n\
        arr.copy_from_slice(&data[..{w}]);\n\
        Self(arr)\n\
    }}\n\
\n\
    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {{\n\
        value.0\n\
    }}\n\
\n\
    fn type_name() -> redb::TypeName {{\n\
        redb::TypeName::new(\"{t}\")\n\
    }}\n\
}}\n\
\n\
impl redb::Key for {t} {{\n\
    fn compare(data1: &[u8], data2: &[u8]) -> ::std::cmp::Ordering {{\n\
        data1.cmp(data2)\n\
    }}\n\
}}"
    ))
}

// ── plugin ────────────────────────────────────────────────────────────────────

/// Plugin providing `redb::Key`, `redb::Value`, and `redb::MutInPlaceValue` trait implementation skeletons.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "redb_types")]
pub struct RedbTypesPlugin;
