# elicit_sqlx implementation plan

## Design principles

### Mirror sqlx names exactly

Shadow crates are a dictionary. If sqlx calls it `AnyPool`, we call it
`AnyPool`. If sqlx has `fetch_all()`, we expose `fetch_all()`. Invented
names (`PgRowData`, `QueryStats`) break the contract and teach agents the
wrong vocabulary. The point is that agents compose programs the same way a
human Rust developer would write them.

### Backend-agnostic via `sqlx::Any`

sqlx is explicitly designed for backend-agnostic code. The connection URL
selects the backend at runtime:
- `postgres://...` → Postgres
- `sqlite://...` → SQLite
- `mysql://...` → MySQL

All runtime tools accept `database_url: String` and operate through `AnyPool`.
No Postgres-specific types appear in the tool API. Cargo.toml includes all
three driver features; the URL picks the backend.

### Opaque handles live in PluginContext

`Pool`, `Connection`, `Transaction` are async runtime handles — not
serializable across the MCP boundary. They live server-side in `PluginContext`,
exactly as `reqwest::Client` does today.

`PluginContext` gains two fields gated behind a `sqlx` feature:

```rust
#[cfg(feature = "sqlx")]
pub db: sqlx::any::AnyPool,

#[cfg(feature = "sqlx")]
pub transactions: std::sync::Mutex<
    std::collections::HashMap<uuid::Uuid, sqlx::Transaction<'static, sqlx::Any>>
>,
```

`AnyPool` is `Clone + Send + Sync` (Arc-backed internally) — it shares the
connection pool across every tool invocation on the same server instance.

Transactions use a registry pattern: `sqlx__begin` returns a `transaction_id:
Uuid`; `sqlx__commit` / `sqlx__rollback` accept that ID and remove the entry.
No deferred work — the full architecture is already in place.

---

## sqlx types to shadow

### Serializable representations

| sqlx type | Our newtype | How we serialize it |
|---|---|---|
| `sqlx::Error` | `Error` | Select enum `ErrorKind` of variants |
| `sqlx::any::AnyRow` | `AnyRow` | Materialize as `Vec<(column_name, ColumnValue)>` |
| `sqlx::any::AnyColumn` | `AnyColumn` | `name: String, ordinal: usize, type_name: String` |
| `sqlx::any::AnyQueryResult` | `AnyQueryResult` | `rows_affected: u64, last_insert_id: Option<i64>` |
| `sqlx::any::AnyTypeInfo` | `AnyTypeInfo` | `name: String` |

`ColumnValue` is the natural sqlx value vocabulary for the `Any` backend:
`Text`, `Integer`, `Real`, `Bool`, `Bytes`, `Null`.

### Methods to reflect (mirror sqlx exactly)

**`AnyPool`** (created per-call; not serialized):
- `execute(sql)` → `AnyQueryResult`
- `fetch_all(sql)` → `Vec<AnyRow>`
- `fetch_one(sql)` → `AnyRow`
- `fetch_optional(sql)` → `Option<AnyRow>`
- `size()` → `u32`
- `is_closed()` → `bool`

**`AnyRow`**:
- `columns()` → `Vec<AnyColumn>`
- `len()` → `usize`
- `is_empty()` → `bool`
- typed accessors: `try_get_string(col)`, `try_get_i64(col)`, `try_get_f64(col)`, `try_get_bool(col)`

**`AnyColumn`**:
- `name()` → `&str`
- `ordinal()` → `usize`
- `type_info()` → `AnyTypeInfo`

**`AnyQueryResult`**:
- `rows_affected()` → `u64`
- `last_insert_id()` → `Option<i64>`

**`Error`**:
- `kind()` → `ErrorKind`
- `message()` → `String`

`ErrorKind` Select enum: `Database, PoolTimedOut, PoolClosed, WorkerCrashed,
Protocol, ColumnNotFound, RowNotFound, TypeNotFound, Io, Configuration, Other`

### Fragment tools (compile-time macros — mirror macro names)

| Tool | Wraps | Params |
|---|---|---|
| `sqlx__query` | `query!()` | `{ sql, params: Vec<String> }` |
| `sqlx__query_as` | `query_as!()` | `{ target_type, sql }` |
| `sqlx__query_scalar` | `query_scalar!()` | `{ sql, scalar_type }` |
| `sqlx__migrate` | `migrate!()` | `{ path }` |

Descriptions note: emitted binary requires `DATABASE_URL` at **build time**
(sqlx's own compile-time contract).

### Trait factories

| Trait | Pattern | Generates |
|---|---|---|
| `sqlx::FromRow` | `#[reflect_trait]` | `sqlx__from_row::<T>` per registered type |

Deferred: `Encode<DB>`, `Decode<DB>`, `Type<DB>`.

---

## Phase 1 — Workspace root

Add to `Cargo.toml`:
```toml
sqlx = { version = "0.8", features = [
    "runtime-tokio", "tls-native-tls",
    "any", "postgres", "sqlite", "mysql", "macros"
] }
elicit_sqlx = { path = "crates/elicit_sqlx", version = "0.9.1" }
```
Add `"crates/elicit_sqlx"` to `members`.

---

## Phase 2 — Elicitation primitives

Feature flag: `sqlx-types`

Files under `src/primitives/sqlx_types/`:

- `error.rs` — `ErrorKind` Select enum (11 variants)
- `column_value.rs` — `ColumnValue` Select enum (Text/Integer/Real/Bool/Bytes/Null)
- `row.rs` — `AnyColumn` + `AnyRow` Survey types
- `query_result.rs` — `AnyQueryResult` Survey type
- `type_info.rs` — `AnyTypeInfo` primitive

Wire: `primitives/mod.rs` + `lib.rs` + `type_spec/sqlx_specs.rs`.

---

## Phase 3 — `crates/elicit_sqlx/`

### Structure

```
src/
  lib.rs
  error.rs             AnyPool, Error newtype + ErrorKind mapping
  any_row.rs           AnyRow newtype + materialize from sqlx::any::AnyRow
  any_column.rs        AnyColumn newtype
  any_query_result.rs  AnyQueryResult newtype
  any_type_info.rs     AnyTypeInfo newtype
  workflow.rs          SqlxPlugin — 5 runtime tools
  fragments/
    mod.rs
    query.rs           sqlx__query
    query_as.rs        sqlx__query_as
    query_scalar.rs    sqlx__query_scalar
    migrate.rs         sqlx__migrate
  plugin.rs            top-level SqlxPlugin
tests/
  types_test.rs        serde roundtrip, schema, method reflection
  fragment_test.rs     EmitCode output + dispatch_emit_from (no DB)
  runtime_test.rs      live DB (#[cfg_attr(not(feature="api"), ignore)])
```

### Workflow plugin

Propositions mirroring sqlx lifecycle:
```rust
pub struct Executed;     impl Prop for Executed {}
pub struct RowsReturned; impl Prop for RowsReturned {}
pub struct InTransaction; impl Prop for InTransaction {}
```

`SqlxPlugin(Arc<PluginContext>)` — newtype wrapping the shared context (same
pattern as `SecureFetchPlugin`). Handlers receive `ctx.db` for the pool and
`ctx.transactions` for the registry.

Tools:
| Tool | sqlx API mirrored | Notes |
|---|---|---|
| `sqlx__execute` | `pool.execute(sql)` | Uses `ctx.db` |
| `sqlx__fetch_all` | `pool.fetch_all(sql)` | Uses `ctx.db` |
| `sqlx__fetch_one` | `pool.fetch_one(sql)` | Uses `ctx.db` |
| `sqlx__fetch_optional` | `pool.fetch_optional(sql)` | Uses `ctx.db` |
| `sqlx__begin` | `pool.begin()` | Returns `transaction_id: Uuid` |
| `sqlx__commit` | `tx.commit()` | Accepts `transaction_id` |
| `sqlx__rollback` | `tx.rollback()` | Accepts `transaction_id` |

---

## Phase 3B — `FromRow` trait factory

```rust
#[reflect_trait]
trait FromRow {
    fn from_row(row: AnyRow) -> Result<Self, Error>;
}
```

`type_map` bridges `sqlx::any::AnyRow` → `elicit_sqlx::AnyRow`.

---

## Phase 4 — Kani harnesses

`crates/elicitation_kani/src/sqlx_types.rs`:
- `column_value_roundtrip`
- `error_kind_labels_roundtrip`
- `error_kind_unknown_rejected`
- `any_query_result_roundtrip`
- `any_row_column_count`

---

## Phase 5 — Creusot

`crates/elicitation_creusot/src/sqlx_types.rs`:
- `error_kind_from_label` requires/ensures
- `column_value_is_null` logic predicate
- `any_row_len_matches_columns` ensures

---

## Milestones

1. **Types + runtime tools** — Phase 1 → 2 → 3 (types + workflow)
2. **Fragment tools** — Phase 3 fragments/
3. **Trait factory + verification** — Phase 3B → 4 → 5

## Deferred

- `Encode/Decode/Type<DB>` trait factories — complex associated types
- `Pool::acquire()` — returns opaque `Connection` (use pool methods directly instead)
