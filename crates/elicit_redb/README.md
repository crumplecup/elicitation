# elicit_redb

MCP tools for redb 4.x — embedded key-value store code generation.

`elicit_redb` is part of the [elicitation](../../README.md) workspace. It exposes the
[redb](https://docs.rs/redb) 4.x API surface as MCP tools, letting an AI model author
correct, idiomatic redb Rust code without memorising API details.

---

## Coverage at a Glance

| Plugin | Tool prefix | Tools | Coverage category |
|--------|-------------|------:|-------------------|
| `RedbDatabasePlugin` | `redb_database__*` | 9 | Database / Builder creation and management |
| `RedbTablePlugin` | `redb_table__*` | 14 | TableDefinition, typed CRUD, iteration |
| `RedbSavepointPlugin` | `redb_savepoint__*` | 5 | Savepoint create / restore / delete |
| `RedbTransactionPlugin` | `redb_txn__*` | 6 | Stateful write-transaction builder |
| `RedbMultimapPlugin` | `redb_multimap__*` | 8 | MultimapTableDefinition and multimap CRUD |
| `RedbTypesPlugin` | `redb_types__*` | 7 | Key / Value / MutInPlaceValue trait skeletons |
| `RedbBackendPlugin` | `redb_backend__*` | 4 | StorageBackend implementation skeleton |
| **Total** | | **53** | |

---

## Why This Crate Exists

redb has a concise but precise API: tables are typed by `(K, V)` type parameters,
transactions are lexically scoped, and the `Key`/`Value` traits require careful byte-level
encoding. An AI model without this crate would need to recall exact method signatures,
trait bounds, and lifetime constraints from memory. This crate packages all of that as
53 MCP tools with schema-validated parameters and sensible defaults.

---

## Quick Start

Add the plugins to your MCP server:

```rust
use elicit_redb::{
    RedbDatabasePlugin, RedbTablePlugin, RedbSavepointPlugin,
    RedbTransactionPlugin, RedbMultimapPlugin, RedbTypesPlugin, RedbBackendPlugin,
};
use elicitation::ElicitPlugin;

let plugins: Vec<Box<dyn ElicitPlugin>> = vec![
    Box::new(RedbDatabasePlugin),
    Box::new(RedbTablePlugin),
    Box::new(RedbSavepointPlugin),
    Box::new(RedbTransactionPlugin::new()),
    Box::new(RedbMultimapPlugin),
    Box::new(RedbTypesPlugin),
    Box::new(RedbBackendPlugin),
];
```

---

## Plugin Reference

### `RedbDatabasePlugin` (9 tools)

Stateless tools for database lifecycle management.

| Tool | Emits |
|------|-------|
| `redb_database__create` | `Database::create(path)?` |
| `redb_database__open` | `Database::open(path)?` |
| `redb_database__open_read_only` | `ReadOnlyDatabase::open(path)?` |
| `redb_database__builder_new` | `Builder::new()` chain with optional `set_page_size` / `set_cache_size` |
| `redb_database__begin_write` | `db.begin_write()` + commit/abort block |
| `redb_database__begin_read` | `db.begin_read()` binding |
| `redb_database__compact` | `db.compact()?` with return-value note |
| `redb_database__check_integrity` | `db.check_integrity(…)` with repair session choice |
| `redb_database__stats` | `write_txn.stats()?` with field printout |

---

### `RedbTablePlugin` (14 tools)

Stateless tools for typed table operations.

| Tool | Emits |
|------|-------|
| `redb_table__define` | `const TABLE: TableDefinition<K, V> = TableDefinition::new("name")` |
| `redb_table__open_write` | `write_txn.open_table(TABLE)?` |
| `redb_table__open_read` | `read_txn.open_table(TABLE)?` |
| `redb_table__insert` | `table.insert(key, value)?` with optional old-value capture |
| `redb_table__get` | `table.get(key)?` with `Option<AccessGuard>` pattern |
| `redb_table__remove` | `table.remove(key)?` with optional old-value capture |
| `redb_table__iter` | `for item in table.iter()?` loop skeleton |
| `redb_table__range` | `table.range(lo..=hi)?` loop skeleton |
| `redb_table__pop` | `table.pop_first()` or `pop_last()` snippet |
| `redb_table__retain` | `table.retain(\|k, v\| predicate)?` |
| `redb_table__len` | `table.len()?` + `table.is_empty()?` |
| `redb_table__rename` | `write_txn.rename_table(&old, &new)?` |
| `redb_table__delete` | `write_txn.delete_table(TABLE)?` |
| `redb_table__list` | `write_txn.list_tables()` iteration snippet |

---

### `RedbSavepointPlugin` (5 tools)

Stateless tools for savepoint lifecycle.

| Tool | Emits |
|------|-------|
| `redb_savepoint__create_persistent` | `write_txn.create_savepoint()?` |
| `redb_savepoint__create_ephemeral` | `write_txn.ephemeral_savepoint()?` |
| `redb_savepoint__restore` | `write_txn.restore_savepoint(&sp)?` |
| `redb_savepoint__list` | `write_txn.list_persistent_savepoints()` loop |
| `redb_savepoint__delete` | `write_txn.delete_savepoint(sp)?` |

---

### `RedbTransactionPlugin` (6 tools) — stateful

Maintains a UUID-keyed map of transaction descriptors. Useful for building a multi-table
write transaction incrementally across multiple tool calls.

| Tool | Action |
|------|--------|
| `redb_txn__start` | Create a new descriptor; returns UUID |
| `redb_txn__add_op` | Append a Rust statement to the transaction body |
| `redb_txn__set_durability` | Set `Durability::None` or `Durability::Immediate` |
| `redb_txn__set_two_phase` | Enable/disable two-phase commit |
| `redb_txn__inspect` | Show the current descriptor as JSON |
| `redb_txn__emit` | Emit the complete `begin_write` + ops + `commit`/`abort` block |

---

### `RedbMultimapPlugin` (8 tools)

Stateless tools for multimap tables (one key → multiple values).

| Tool | Emits |
|------|-------|
| `redb_multimap__define` | `const MM: MultimapTableDefinition<K, V> = …` |
| `redb_multimap__open_write` | `write_txn.open_multimap_table(MM)?` |
| `redb_multimap__open_read` | `read_txn.open_multimap_table(MM)?` |
| `redb_multimap__insert` | `table.insert(&key, &value)?` |
| `redb_multimap__get` | `for v in table.get(&key)?` iteration pattern |
| `redb_multimap__remove` | `table.remove(&key, &value)?` |
| `redb_multimap__remove_all` | `table.remove_all(&key)?` |
| `redb_multimap__iter` | `for item in table.iter()?` loop skeleton |

---

### `RedbTypesPlugin` (7 tools)

Stateless tools that emit trait implementation skeletons. Useful when a user type needs to
be stored directly in a redb table without a serialisation wrapper.

| Tool | Emits |
|------|-------|
| `redb_types__impl_key` | `impl redb::Key for T` skeleton (Ord + byte conversions) |
| `redb_types__impl_value` | `impl redb::Value for T` skeleton |
| `redb_types__impl_mut_in_place` | `impl redb::MutInPlaceValue for T` skeleton |
| `redb_types__derive_key_bincode` | bincode-based `Key` impl using `encode_to_vec`/`decode_from_slice` |
| `redb_types__derive_value_json` | `serde_json`-based `Value` impl |
| `redb_types__type_name` | `const NAME: &str = "..."` type-name const |
| `redb_types__fixed_width_key` | Fixed-width key struct + `Key` impl (e.g., `u64` as 8-byte array) |

---

### `RedbBackendPlugin` (4 tools)

Stateless tools for custom `StorageBackend` implementations.

| Tool | Emits |
|------|-------|
| `redb_backend__impl_storage` | `impl StorageBackend for T` skeleton with all required method signatures |
| `redb_backend__read_impl` | `fn read` body for an in-memory backend |
| `redb_backend__write_impl` | `fn write` / `fn sync_data` body for an in-memory backend |
| `redb_backend__in_memory_struct` | Complete in-memory `StorageBackend` struct + impl |

---

## Design Notes

### Pure code generation

No redb runtime types are instantiated. Every tool returns a Rust code snippet as a
`String`. This means `elicit_redb` has no transitive dependency on redb itself — the
generated code is what the user compiles against their own redb dependency.

### Stateless vs stateful plugins

Six of the seven plugins are stateless unit structs (`RedbDatabasePlugin`, etc.) — each
tool is a pure function from parameters to a code string. `RedbTransactionPlugin` is the
exception: it holds an `Arc<Mutex<HashMap<Uuid, TxnDescriptor>>>` to track in-progress
transaction descriptors, matching the `SurrealSelectPlugin` / `SurrealTransactionPlugin`
pattern from `elicit_surrealdb`.

### Unique params structs per tool

Every tool uses its own `*Params` struct. Two tools sharing a struct would cause
conflicting `EmitCode` impls generated by `#[elicit_tool]`.
