# elicit_toml

MCP shadow crate for [`toml`], [`toml_edit`], and [`toml_datetime`] — exposes the
full API surface of all three crates as Model Context Protocol tools.

## Crates covered

| Crate | Version | Coverage |
|---|---|---|
| `toml` | 1.1 | `from_str`, `to_string`, `to_string_pretty`, `Value` methods |
| `toml_edit` | 0.25 | `DocumentMut`, `Table`, `InlineTable`, `Array`, `ArrayOfTables`, `Item`, `Value`, `Key` |
| `toml_datetime` | 1.1 | `Datetime`, `Date`, `Time`, `Offset` construction and accessors |

## Plugin

`TomlPlugin` is a **stateful plugin** holding live objects by UUID:

```rust
pub struct TomlCtx {
    pub documents:     Mutex<HashMap<Uuid, toml_edit::DocumentMut>>,
    pub tables:        Mutex<HashMap<Uuid, toml_edit::Table>>,
    pub arrays:        Mutex<HashMap<Uuid, toml_edit::Array>>,
    pub inline_tables: Mutex<HashMap<Uuid, toml_edit::InlineTable>>,
}
```

## Typical workflow

```
1. parse_document_from_str → document_id
2. document_get(document_id, key) → item JSON
3. document_insert(document_id, key, value)
4. document_to_string(document_id) → TOML text
```
