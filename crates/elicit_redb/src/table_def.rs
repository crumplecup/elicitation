//! Table operation tools for fixed `(K, V)` combos.
//!
//! `redb` tables are generic: `redb::TableDefinition<K, V>`.  Because MCP tools
//! must have concrete types at compile time, this module provides explicit tools
//! for a small, commonly-needed set of `(K, V)` combinations:
//!
//! | Key   | Value  | Tool prefix                  |
//! |-------|--------|------------------------------|
//! | `u64` | `u64`  | `table_u64_u64__`            |
//! | `u64` | `&str` | `table_u64_str__`            |
//! | `&str`| `u64`  | `table_str_u64__`            |
//! | `&str`| `&str` | `table_str_str__`            |
//!
//! Open a table via `write_txn__open_table_u64_u64` etc., get back a typed
//! table handle, then use `table_u64_u64__insert`, `table_u64_u64__get`, etc.

use std::sync::Arc;

use elicitation::elicit_tool;
use rmcp::{ErrorData, model::CallToolResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    ReadTransaction, WriteTransaction,
    plugin::{RedbCtx, ok_json},
};
use redb::ReadableTable as _;

// ── Shadow table handles ──────────────────────────────────────────────────────

macro_rules! uuid_handle {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, JsonSchema)]
        pub struct $name(pub Uuid);

        impl Serialize for $name {
            fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                s.serialize_str(&self.0.to_string())
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                let s = String::deserialize(d)?;
                s.parse::<Uuid>()
                    .map($name)
                    .map_err(serde::de::Error::custom)
            }
        }
    };
}

uuid_handle!(
    TableU64U64,
    "Handle for an open `Table<u64, u64>` in a write transaction."
);
uuid_handle!(
    TableStrStr,
    "Handle for an open `Table<&str, &str>` in a write transaction."
);

uuid_handle!(
    ReadTableU64U64,
    "Handle for an open `ReadOnlyTable<u64, u64>`."
);
uuid_handle!(
    ReadTableStrStr,
    "Handle for an open `ReadOnlyTable<&str, &str>`."
);

// ── Params ────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct OpenWriteTableParams {
    /// Write transaction handle.
    pub txn: WriteTransaction,
    /// Table name.
    pub name: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct OpenReadTableParams {
    /// Read transaction handle.
    pub txn: ReadTransaction,
    /// Table name.
    pub name: String,
}

// ── Context extension: typed table maps ───────────────────────────────────────
//
// The plugin context (RedbCtx) already handles `redb::Database`, `WriteTransaction`,
// etc.  For the open tables we store the raw bytes in the transaction lifetime.
// Because tables borrow the transaction, we cannot store them independently.
//
// Instead we use the same pattern as `elicit_redb` databases: we keep the table
// *name* and look it up on every operation, which is cheap for the MCP latency
// profile.
//
// So there are NO extra UUID maps for tables in RedbCtx — the UUID returned by
// the open_table tools just carries the (txn_id, table_name) association in a
// short-lived session store we keep in `table_def.rs` via `once_cell::Lazy`.

use std::{collections::HashMap, sync::Mutex};

type TableKey = (Uuid, String); // (txn_uuid, table_name)

static WRITE_TABLES_U64_U64: std::sync::LazyLock<Mutex<HashMap<Uuid, TableKey>>> =
    std::sync::LazyLock::new(|| Mutex::new(HashMap::new()));
static WRITE_TABLES_STR_STR: std::sync::LazyLock<Mutex<HashMap<Uuid, TableKey>>> =
    std::sync::LazyLock::new(|| Mutex::new(HashMap::new()));
static READ_TABLES_U64_U64: std::sync::LazyLock<Mutex<HashMap<Uuid, TableKey>>> =
    std::sync::LazyLock::new(|| Mutex::new(HashMap::new()));
static READ_TABLES_STR_STR: std::sync::LazyLock<Mutex<HashMap<Uuid, TableKey>>> =
    std::sync::LazyLock::new(|| Mutex::new(HashMap::new()));

fn lock_table_map<T>(m: &Mutex<T>) -> Result<std::sync::MutexGuard<'_, T>, ErrorData> {
    m.lock()
        .map_err(|_| ErrorData::internal_error("table map lock poisoned", None))
}

// ── Optional value wrapper ────────────────────────────────────────────────────

#[derive(Serialize)]
struct MaybeValue<T: Serialize> {
    value: Option<T>,
}

// ═══════════════════════════════════════════════════════════════════════════════
// u64 / u64 tables
// ═══════════════════════════════════════════════════════════════════════════════

/// Open (or create) a `Table<u64, u64>` within a write transaction.
#[elicit_tool(plugin = "redb", name = "write_txn__open_table_u64_u64",
    description = "Open or create a Table<u64,u64> within a write transaction. Returns a TableU64U64 handle.",
    emit = None)]
#[instrument(skip(ctx), fields(name = %p.name))]
pub async fn write_txn_open_table_u64_u64(
    ctx: Arc<RedbCtx>,
    p: OpenWriteTableParams,
) -> Result<CallToolResult, ErrorData> {
    // Validate txn exists
    {
        ctx.lock_write_txns()?
            .get(&p.txn.0)
            .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn: {}", p.txn.0), None))?;
    }
    // Verify the table can be opened (open and immediately drop the borrow)
    {
        let txns = ctx.lock_write_txns()?;
        let txn = txns.get(&p.txn.0).unwrap();
        let def = redb::TableDefinition::<u64, u64>::new(&p.name);
        txn.open_table(def)
            .map_err(|e| ErrorData::internal_error(format!("open_table: {e}"), None))?;
    }
    let id = Uuid::new_v4();
    lock_table_map(&WRITE_TABLES_U64_U64)?.insert(id, (p.txn.0, p.name));
    ok_json(&TableU64U64(id))
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TableU64U64InsertParams {
    pub table: TableU64U64,
    pub key: u64,
    pub value: u64,
}
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TableU64U64GetParams {
    pub table: TableU64U64,
    pub key: u64,
}
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TableU64U64RemoveParams {
    pub table: TableU64U64,
    pub key: u64,
}

/// Insert a key-value pair into a `Table<u64, u64>`.
#[elicit_tool(plugin = "redb", name = "table_u64_u64__insert",
    description = "Insert or overwrite a u64→u64 entry. Returns the previous value if any.",
    emit = None)]
#[instrument(skip(ctx))]
pub async fn table_u64_u64_insert(
    ctx: Arc<RedbCtx>,
    p: TableU64U64InsertParams,
) -> Result<CallToolResult, ErrorData> {
    let (txn_id, name) = lock_table_map(&WRITE_TABLES_U64_U64)?
        .get(&p.table.0)
        .cloned()
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown table: {}", p.table.0), None))?;
    let txns = ctx.lock_write_txns()?;
    let txn = txns
        .get(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("txn no longer open: {txn_id}"), None))?;
    let def = redb::TableDefinition::<u64, u64>::new(&name);
    let mut tbl = txn
        .open_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_table: {e}"), None))?;
    let prev = tbl
        .insert(p.key, p.value)
        .map_err(|e| ErrorData::internal_error(format!("insert: {e}"), None))?
        .map(|ag| ag.value());
    ok_json(&MaybeValue { value: prev })
}

/// Get a value from a `Table<u64, u64>`.
#[elicit_tool(plugin = "redb", name = "table_u64_u64__get",
    description = "Get a value from a u64→u64 table.",
    emit = None)]
#[instrument(skip(ctx))]
pub async fn table_u64_u64_get(
    ctx: Arc<RedbCtx>,
    p: TableU64U64GetParams,
) -> Result<CallToolResult, ErrorData> {
    let (txn_id, name) = lock_table_map(&WRITE_TABLES_U64_U64)?
        .get(&p.table.0)
        .cloned()
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown table: {}", p.table.0), None))?;
    let txns = ctx.lock_write_txns()?;
    let txn = txns
        .get(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("txn no longer open: {txn_id}"), None))?;
    let def = redb::TableDefinition::<u64, u64>::new(&name);
    let tbl = txn
        .open_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_table: {e}"), None))?;
    let val = tbl
        .get(p.key)
        .map_err(|e| ErrorData::internal_error(format!("get: {e}"), None))?
        .map(|ag| ag.value());
    ok_json(&MaybeValue { value: val })
}

/// Remove a key from a `Table<u64, u64>`.
#[elicit_tool(plugin = "redb", name = "table_u64_u64__remove",
    description = "Remove a key from a u64→u64 table. Returns the removed value if any.",
    emit = None)]
#[instrument(skip(ctx))]
pub async fn table_u64_u64_remove(
    ctx: Arc<RedbCtx>,
    p: TableU64U64RemoveParams,
) -> Result<CallToolResult, ErrorData> {
    let (txn_id, name) = lock_table_map(&WRITE_TABLES_U64_U64)?
        .get(&p.table.0)
        .cloned()
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown table: {}", p.table.0), None))?;
    let txns = ctx.lock_write_txns()?;
    let txn = txns
        .get(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("txn no longer open: {txn_id}"), None))?;
    let def = redb::TableDefinition::<u64, u64>::new(&name);
    let mut tbl = txn
        .open_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_table: {e}"), None))?;
    let prev = tbl
        .remove(p.key)
        .map_err(|e| ErrorData::internal_error(format!("remove: {e}"), None))?
        .map(|ag| ag.value());
    ok_json(&MaybeValue { value: prev })
}

// ═══════════════════════════════════════════════════════════════════════════════
// &str / &str tables  (read in write-txn; read-txn access)
// ═══════════════════════════════════════════════════════════════════════════════

/// Open (or create) a `Table<&str, &str>` within a write transaction.
#[elicit_tool(plugin = "redb", name = "write_txn__open_table_str_str",
    description = "Open or create a Table<&str,&str> within a write transaction. Returns a TableStrStr handle.",
    emit = None)]
#[instrument(skip(ctx), fields(name = %p.name))]
pub async fn write_txn_open_table_str_str(
    ctx: Arc<RedbCtx>,
    p: OpenWriteTableParams,
) -> Result<CallToolResult, ErrorData> {
    {
        let txns = ctx.lock_write_txns()?;
        let txn = txns
            .get(&p.txn.0)
            .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn: {}", p.txn.0), None))?;
        let def = redb::TableDefinition::<&str, &str>::new(&p.name);
        txn.open_table(def)
            .map_err(|e| ErrorData::internal_error(format!("open_table: {e}"), None))?;
    }
    let id = Uuid::new_v4();
    lock_table_map(&WRITE_TABLES_STR_STR)?.insert(id, (p.txn.0, p.name));
    ok_json(&TableStrStr(id))
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TableStrStrInsertParams {
    pub table: TableStrStr,
    pub key: String,
    pub value: String,
}
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TableStrStrGetParams {
    pub table: TableStrStr,
    pub key: String,
}
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TableStrStrRemoveParams {
    pub table: TableStrStr,
    pub key: String,
}

/// Insert a key-value pair into a `Table<&str, &str>`.
#[elicit_tool(plugin = "redb", name = "table_str_str__insert",
    description = "Insert or overwrite a &str→&str entry. Returns the previous value if any.",
    emit = None)]
#[instrument(skip(ctx))]
pub async fn table_str_str_insert(
    ctx: Arc<RedbCtx>,
    p: TableStrStrInsertParams,
) -> Result<CallToolResult, ErrorData> {
    let (txn_id, name) = lock_table_map(&WRITE_TABLES_STR_STR)?
        .get(&p.table.0)
        .cloned()
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown table: {}", p.table.0), None))?;
    let txns = ctx.lock_write_txns()?;
    let txn = txns
        .get(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("txn no longer open: {txn_id}"), None))?;
    let def = redb::TableDefinition::<&str, &str>::new(&name);
    let mut tbl = txn
        .open_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_table: {e}"), None))?;
    let prev = tbl
        .insert(p.key.as_str(), p.value.as_str())
        .map_err(|e| ErrorData::internal_error(format!("insert: {e}"), None))?
        .map(|ag| ag.value().to_owned());
    ok_json(&MaybeValue { value: prev })
}

/// Get a value from a `Table<&str, &str>`.
#[elicit_tool(plugin = "redb", name = "table_str_str__get",
    description = "Get a value from a &str→&str table.",
    emit = None)]
#[instrument(skip(ctx))]
pub async fn table_str_str_get(
    ctx: Arc<RedbCtx>,
    p: TableStrStrGetParams,
) -> Result<CallToolResult, ErrorData> {
    let (txn_id, name) = lock_table_map(&WRITE_TABLES_STR_STR)?
        .get(&p.table.0)
        .cloned()
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown table: {}", p.table.0), None))?;
    let txns = ctx.lock_write_txns()?;
    let txn = txns
        .get(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("txn no longer open: {txn_id}"), None))?;
    let def = redb::TableDefinition::<&str, &str>::new(&name);
    let tbl = txn
        .open_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_table: {e}"), None))?;
    let val = tbl
        .get(p.key.as_str())
        .map_err(|e| ErrorData::internal_error(format!("get: {e}"), None))?
        .map(|ag| ag.value().to_owned());
    ok_json(&MaybeValue { value: val })
}

/// Remove a key from a `Table<&str, &str>`.
#[elicit_tool(plugin = "redb", name = "table_str_str__remove",
    description = "Remove a key from a &str→&str table. Returns the removed value if any.",
    emit = None)]
#[instrument(skip(ctx))]
pub async fn table_str_str_remove(
    ctx: Arc<RedbCtx>,
    p: TableStrStrRemoveParams,
) -> Result<CallToolResult, ErrorData> {
    let (txn_id, name) = lock_table_map(&WRITE_TABLES_STR_STR)?
        .get(&p.table.0)
        .cloned()
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown table: {}", p.table.0), None))?;
    let txns = ctx.lock_write_txns()?;
    let txn = txns
        .get(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("txn no longer open: {txn_id}"), None))?;
    let def = redb::TableDefinition::<&str, &str>::new(&name);
    let mut tbl = txn
        .open_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_table: {e}"), None))?;
    let prev = tbl
        .remove(p.key.as_str())
        .map_err(|e| ErrorData::internal_error(format!("remove: {e}"), None))?
        .map(|ag| ag.value().to_owned());
    ok_json(&MaybeValue { value: prev })
}

// ═══════════════════════════════════════════════════════════════════════════════
// Read-only table access (ReadTransaction)
// ═══════════════════════════════════════════════════════════════════════════════

/// Open a `ReadOnlyTable<u64, u64>` within a read transaction.
#[elicit_tool(plugin = "redb", name = "read_txn__open_table_u64_u64",
    description = "Open a ReadOnlyTable<u64,u64> for reading. Returns a ReadTableU64U64 handle.",
    emit = None)]
#[instrument(skip(ctx), fields(name = %p.name))]
pub async fn read_txn_open_table_u64_u64(
    ctx: Arc<RedbCtx>,
    p: OpenReadTableParams,
) -> Result<CallToolResult, ErrorData> {
    {
        let txns = ctx.lock_read_txns()?;
        let txn = txns
            .get(&p.txn.0)
            .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn: {}", p.txn.0), None))?;
        let def = redb::TableDefinition::<u64, u64>::new(&p.name);
        txn.open_table(def)
            .map_err(|e| ErrorData::internal_error(format!("open_table: {e}"), None))?;
    }
    let id = Uuid::new_v4();
    lock_table_map(&READ_TABLES_U64_U64)?.insert(id, (p.txn.0, p.name));
    ok_json(&ReadTableU64U64(id))
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReadTableU64U64GetParams {
    pub table: ReadTableU64U64,
    pub key: u64,
}

/// Get a value from a `ReadOnlyTable<u64, u64>`.
#[elicit_tool(plugin = "redb", name = "read_table_u64_u64__get",
    description = "Get a value from a read-only u64→u64 table.",
    emit = None)]
#[instrument(skip(ctx))]
pub async fn read_table_u64_u64_get(
    ctx: Arc<RedbCtx>,
    p: ReadTableU64U64GetParams,
) -> Result<CallToolResult, ErrorData> {
    let (txn_id, name) = lock_table_map(&READ_TABLES_U64_U64)?
        .get(&p.table.0)
        .cloned()
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown table: {}", p.table.0), None))?;
    let txns = ctx.lock_read_txns()?;
    let txn = txns
        .get(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("txn no longer open: {txn_id}"), None))?;
    let def = redb::TableDefinition::<u64, u64>::new(&name);
    let tbl = txn
        .open_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_table: {e}"), None))?;
    let val = tbl
        .get(p.key)
        .map_err(|e| ErrorData::internal_error(format!("get: {e}"), None))?
        .map(|ag| ag.value());
    ok_json(&MaybeValue { value: val })
}

/// Open a `ReadOnlyTable<&str, &str>` within a read transaction.
#[elicit_tool(plugin = "redb", name = "read_txn__open_table_str_str",
    description = "Open a ReadOnlyTable<&str,&str> for reading. Returns a ReadTableStrStr handle.",
    emit = None)]
#[instrument(skip(ctx), fields(name = %p.name))]
pub async fn read_txn_open_table_str_str(
    ctx: Arc<RedbCtx>,
    p: OpenReadTableParams,
) -> Result<CallToolResult, ErrorData> {
    {
        let txns = ctx.lock_read_txns()?;
        let txn = txns
            .get(&p.txn.0)
            .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn: {}", p.txn.0), None))?;
        let def = redb::TableDefinition::<&str, &str>::new(&p.name);
        txn.open_table(def)
            .map_err(|e| ErrorData::internal_error(format!("open_table: {e}"), None))?;
    }
    let id = Uuid::new_v4();
    lock_table_map(&READ_TABLES_STR_STR)?.insert(id, (p.txn.0, p.name));
    ok_json(&ReadTableStrStr(id))
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReadTableStrStrGetParams {
    pub table: ReadTableStrStr,
    pub key: String,
}

/// Get a value from a `ReadOnlyTable<&str, &str>`.
#[elicit_tool(plugin = "redb", name = "read_table_str_str__get",
    description = "Get a value from a read-only &str→&str table.",
    emit = None)]
#[instrument(skip(ctx))]
pub async fn read_table_str_str_get(
    ctx: Arc<RedbCtx>,
    p: ReadTableStrStrGetParams,
) -> Result<CallToolResult, ErrorData> {
    let (txn_id, name) = lock_table_map(&READ_TABLES_STR_STR)?
        .get(&p.table.0)
        .cloned()
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown table: {}", p.table.0), None))?;
    let txns = ctx.lock_read_txns()?;
    let txn = txns
        .get(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("txn no longer open: {txn_id}"), None))?;
    let def = redb::TableDefinition::<&str, &str>::new(&name);
    let tbl = txn
        .open_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_table: {e}"), None))?;
    let val = tbl
        .get(p.key.as_str())
        .map_err(|e| ErrorData::internal_error(format!("get: {e}"), None))?
        .map(|ag| ag.value().to_owned());
    ok_json(&MaybeValue { value: val })
}
