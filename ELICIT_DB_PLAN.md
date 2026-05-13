# elicit_db — Database Contract Interface

> **Role:** Interface crate — like `elicit_ui`, not a shadow crate.
> **Purpose:** Define the domain boundary for database interactions. Provides Props,
> typestate markers, and a **complete family of object-safe traits** covering all
> operations of a pgAdmin-style DB management application.
> Traits use contract return types (`Established<P>`) instead of associated types —
> this gives object safety and a common contract language at every call site.
> **No DB driver dependency.** Implementations (sqlx, diesel, sea-orm) live elsewhere.

---

## Design Philosophy

`elicit_db` is the *dictionary*, not the *paragraph*. It defines a comprehensive,
standards-anchored vocabulary of database contracts. Users reach for what they need.

Analogies:

- `elicit_ui` → WCAG Props + accessibility typestate + rendering traits
- `elicit_db` → ISO SQL + ANSI isolation + PostgreSQL + ISO 27001 Props + **management traits**

The multi-standard stack from `elicit_db_plan.md` is the normative grounding for each
Prop. Every Prop maps to either an ISO clause or a documented PostgreSQL guarantee.

### Why contract return types instead of associated types

Traditional trait design uses associated types for result shapes:

```rust
// NOT this — associated types break object safety
trait DbExecutor {
    type Row;
    type Error;
    fn query(&self, sql: &str) -> Result<Vec<Self::Row>, Self::Error>;
}
```

`elicit_db` uses `Established<P>` tuples instead:

```rust
// THIS — object-safe, self-documenting
trait DbQueryExecutor: Send + Sync {
    fn query_rows(
        &self, sql: &str, params: &[DbValue]
    ) -> BoxFuture<DbResult<(DbRows, Established<RowVisible>)>>;
}
```

Benefits:

- `dyn DbTableManager` works — no associated types to bind
- The return type IS the contract: callers know exactly what was proven
- Composable: `(Established<RowInserted>, Established<AuditLogged>)` reads like prose
- All implementations speak the same language regardless of underlying driver

---

## Crate Structure

```
crates/elicit_db/
├── Cargo.toml
├── README.md
└── src/
    ├── lib.rs               # mod + pub use only
    ├── typestate.rs         # Transaction<S>, Query<S> state machine markers
    ├── types.rs             # DbRow, DbRows, DbValue, DbSchema, DbColumn, etc.
    ├── error.rs             # DbError / DbErrorKind
    ├── traits/
    │   ├── mod.rs           # pub use all traits
    │   ├── session.rs       # DbSessionManager — connect/disconnect/list sessions
    │   ├── server.rs        # DbServerAdmin — config, extensions, pg_settings
    │   ├── database.rs      # DbDatabaseManager — create/drop/list/rename databases
    │   ├── schema.rs        # DbSchemaManager — create/drop/list schemas
    │   ├── table.rs         # DbTableManager — create/alter/drop/list/inspect tables
    │   ├── query.rs         # DbQueryExecutor — execute/query/explain/analyze
    │   ├── transaction.rs   # DbTransactor — begin/commit/rollback/savepoint
    │   ├── index.rs         # DbIndexManager — create/drop/list indexes
    │   ├── role.rs          # DbRoleManager — create/drop/grant/revoke/list roles
    │   ├── monitor.rs       # DbMonitor — pg_stat_*, slow queries, bloat, index usage
    │   └── backup.rs        # DbBackupManager — initiate/list/verify backups
    └── contracts/
        ├── mod.rs           # pub use all
        ├── iso_sql.rs       # DDL/DML/constraints (ISO 9075)
        ├── isolation.rs     # ANSI isolation levels + phenomena
        ├── postgres.rs      # MVCC, advisory locks, WAL (PostgreSQL-specific)
        ├── information_schema.rs # Schema introspection (ISO 9075-11)
        ├── security.rs      # Access control, audit, least privilege (ISO 27001)
        ├── recovery.rs      # Backup, WAL replay, PITR
        ├── transport.rs     # Wire protocol, request/response (RFC 7159, PG protocol)
        └── observability.rs # Trace emission, span linking (OpenTelemetry)
```

---

## Props by Module

### `contracts/iso_sql.rs` — ISO/IEC 9075 (SQL Standard)

```rust
/// Table created by a DDL statement.
/// Source: ISO/IEC 9075-2 §11.3 — <table definition>
pub struct TableCreated { pub name: String }

/// Constraint enforced on a column or table.
/// Source: ISO/IEC 9075-2 §11.6 — <table constraint definition>
pub struct ConstraintSatisfied { pub constraint: String }

/// Referential integrity maintained between tables.
/// Source: ISO/IEC 9075-2 §11.8 — <referential constraint definition>
pub struct ReferentialIntegrityMaintained;

/// View created and queryable.
/// Source: ISO/IEC 9075-2 §11.32 — <view definition>
pub struct ViewCreated { pub name: String }

/// Row inserted into a table.
/// Source: ISO/IEC 9075-2 §14.8 — <insert statement>
pub struct RowInserted { pub table: String }

/// Row updated in a table.
/// Source: ISO/IEC 9075-2 §14.11 — <update statement>
pub struct RowUpdated { pub table: String }

/// Row deleted from a table.
/// Source: ISO/IEC 9075-2 §14.7 — <delete statement>
pub struct RowDeleted { pub table: String }

/// Query returned a non-empty result set.
/// Source: ISO/IEC 9075-2 §14.1 — <query expression>
pub struct NonEmptyResult;

/// Query returned exactly N rows.
/// Source: ISO/IEC 9075-2 §14.1
pub struct ExactRowCount { pub count: usize }
```

### `contracts/isolation.rs` — ANSI Isolation Levels

```rust
/// Transaction committed at Read Uncommitted isolation.
/// Phenomena permitted: P1 (dirty read), P2, P3
pub struct ReadUncommitted;

/// Transaction committed at Read Committed isolation.
/// Phenomena prevented: P1 (dirty read)
/// Source: SQL:1999 §4.32, Table 2
pub struct ReadCommitted;

/// Transaction committed at Repeatable Read isolation.
/// Phenomena prevented: P1, P2 (non-repeatable read)
/// Source: SQL:1999 §4.32, Table 2
pub struct RepeatableRead;

/// Transaction committed at Serializable isolation.
/// Phenomena prevented: P1, P2, P3 (phantom read)
/// Source: SQL:1999 §4.32, Table 2
pub struct Serializable;

/// Specific phenomenon prevented.
/// P0=DirtyWrite, P1=DirtyRead, P2=NonRepeatableRead, P3=Phantom
/// Source: Berenson et al., "A Critique of ANSI SQL Isolation Levels" (1995)
pub struct PreventsPhenomenon<const P: u8>;

/// No dirty reads can occur in this transaction context.
pub type NoDirtyReads = PreventsPhenomenon<1>;

/// No phantom reads can occur in this transaction context.
pub type NoPhantomReads = PreventsPhenomenon<3>;
```

### `contracts/postgres.rs` — PostgreSQL-specific

```rust
/// MVCC snapshot is valid and consistent.
/// Source: PostgreSQL docs §13.1 — Introduction
pub struct MVCCSnapshotValid;

/// Snapshot isolation semantics apply (PostgreSQL's Repeatable Read).
/// Source: PostgreSQL docs §13.2.2 — Repeatable Read Isolation Level
pub struct SnapshotIsolation;

/// Advisory lock is held for the given key.
/// Source: PostgreSQL docs §13.3.5 — Advisory Locks
pub struct AdvisoryLockHeld { pub key: i64 }

/// Row is visible under current snapshot.
/// Source: PostgreSQL docs §13.1 — MVCC visibility rules
pub struct RowVisible;

/// Index exists on the given table+column.
/// Source: PostgreSQL docs §11 — Indexes
pub struct IndexExists { pub table: String, pub column: String }

/// Vacuum has run and table bloat is within bounds.
/// Source: PostgreSQL docs §25.1 — Routine Vacuuming
pub struct VacuumedRecently;
```

### `contracts/information_schema.rs` — ISO 9075-11

```rust
/// Table exists and is queryable via information_schema.
/// Source: ISO/IEC 9075-11 §TABLES view
pub struct TableExists { pub name: String }

/// Column exists on the given table.
/// Source: ISO/IEC 9075-11 §COLUMNS view
pub struct ColumnExists { pub table: String, pub column: String }

/// Schema exists in the catalog.
/// Source: ISO/IEC 9075-11 §SCHEMATA view
pub struct SchemaExists { pub name: String }

/// Foreign key relationship exists between two tables.
/// Source: ISO/IEC 9075-11 §REFERENTIAL_CONSTRAINTS view
pub struct ForeignKeyExists { pub from_table: String, pub to_table: String }
```

### `contracts/security.rs` — ISO/IEC 27001

```rust
/// Action was authorized for the current principal.
/// Source: ISO/IEC 27001:2022 §A.5.15 — Access control
pub struct AccessAuthorized { pub action: String }

/// Operation was recorded in the audit log.
/// Source: ISO/IEC 27001:2022 §A.8.15 — Logging
pub struct AuditLogged;

/// Least privilege principle enforced for this operation.
/// Source: ISO/IEC 27001:2022 §A.5.15
pub struct LeastPrivilegeEnforced;

/// Data is encrypted at rest.
/// Source: ISO/IEC 27001:2022 §A.8.24 — Use of cryptography
pub struct EncryptedAtRest;

/// Connection is encrypted in transit (TLS).
/// Source: ISO/IEC 27001:2022 §A.8.24
pub struct EncryptedInTransit;
```

### `contracts/recovery.rs` — Durability & Recovery

```rust
/// Backup is consistent and restorable.
/// Source: PostgreSQL docs §26 — Backup and Restore
pub struct BackupConsistent;

/// WAL segment is replayable.
/// Source: PostgreSQL docs §30 — Write-Ahead Logging (WAL)
pub struct WALReplayable;

/// Point-in-time recovery is possible to this moment.
/// Source: PostgreSQL docs §26.3 — Continuous Archiving and PITR
pub struct PointInTimeRecoverable;
```

### `contracts/transport.rs` — Wire Protocol

```rust
/// Request is well-formed per PostgreSQL wire protocol.
/// Source: PostgreSQL docs §55 — Frontend/Backend Protocol
pub struct RequestWellFormed;

/// Response is serializable to JSON (RFC 7159).
/// Source: IETF RFC 7159
pub struct ResponseSerializable;

/// Connection is established and authenticated.
/// Source: PostgreSQL docs §55.2 — Message Flow
pub struct ConnectionEstablished;
```

### `contracts/observability.rs` — OpenTelemetry

```rust
/// Trace was emitted for this operation.
/// Source: OpenTelemetry Specification §traces
pub struct TraceEmitted;

/// Span is linked to the parent operation.
/// Source: OpenTelemetry Specification §span-links
pub struct SpanLinkedToOperation;

/// Database metrics recorded (query duration, row count).
/// Source: OpenTelemetry Semantic Conventions for databases
pub struct MetricsRecorded;
```

---

## Typestate: `typestate.rs`

Mirrors `elicit_ui`'s `Layout<Pending/Verified/Rendered>` pattern.

```rust
/// Typestate: transaction is open but not yet committed.
pub struct Open;

/// Typestate: transaction has been committed.
pub struct Committed;

/// Typestate: transaction has been rolled back.
pub struct RolledBack;

/// Typestate: query is built but not yet executed.
pub struct Prepared;

/// Typestate: query has been executed, results available.
pub struct Executed;

/// A transaction handle with typestate tracking.
pub struct Transaction<S> {
    isolation: IsolationLevel,
    audit: bool,
    _state: PhantomData<S>,
}

impl Transaction<Open> {
    pub fn commit(self) -> Transaction<Committed> { ... }
    pub fn rollback(self) -> Transaction<RolledBack> { ... }
}
```

---

## Trait Family: `traits/`

All traits are **object-safe** (`dyn DbTableManager` works). Async methods return
`BoxFuture`. No associated types — result shapes are expressed as contract tuples.

### Common types used in trait signatures (`types.rs`)

```rust
pub struct DbRow(pub IndexMap<String, DbValue>);
pub struct DbRows(pub Vec<DbRow>);
pub struct DbColumn { pub name: String, pub ty: String, pub nullable: bool, pub default: Option<String> }
pub struct DbSchema { pub name: String, pub tables: Vec<DbTableInfo> }
pub struct DbTableInfo { pub name: String, pub schema: String, pub columns: Vec<DbColumn> }
pub struct DbIndexInfo { pub name: String, pub table: String, pub columns: Vec<String>, pub unique: bool }
pub struct DbRoleInfo { pub name: String, pub superuser: bool, pub can_login: bool }
pub struct DbSessionInfo { pub pid: i32, pub app_name: String, pub state: String, pub query: Option<String> }
pub struct DbStatActivity { pub sessions: Vec<DbSessionInfo>, pub idle_count: usize, pub active_count: usize }
pub struct DbExplain { pub plan: String, pub cost: Option<f64>, pub actual_rows: Option<i64> }

pub enum DbValue { Null, Bool(bool), Int(i64), Float(f64), Text(String), Bytes(Vec<u8>), Json(serde_json::Value) }
pub type DbResult<T> = Result<T, DbError>;
```

### `traits/session.rs` — Connection & Session Management

```rust
/// Manage database sessions and connections.
/// Covers: connect, disconnect, list active backends.
pub trait DbSessionManager: Send + Sync {
    fn connect(&self, url: &str)
        -> BoxFuture<DbResult<(ConnectionId, Established<ConnectionEstablished>)>>;

    fn disconnect(&self, id: ConnectionId)
        -> BoxFuture<DbResult<()>>;

    fn list_sessions(&self)
        -> BoxFuture<DbResult<DbStatActivity>>;

    fn terminate_session(&self, pid: i32)
        -> BoxFuture<DbResult<Established<AuditLogged>>>;
}
```

### `traits/server.rs` — Server Administration

```rust
/// PostgreSQL server-level administration.
/// Covers: config inspection, extension management, version info.
pub trait DbServerAdmin: Send + Sync {
    fn server_version(&self)
        -> BoxFuture<DbResult<String>>;

    fn list_settings(&self)
        -> BoxFuture<DbResult<Vec<(String, String)>>>;  // pg_settings

    fn list_extensions(&self)
        -> BoxFuture<DbResult<Vec<String>>>;

    fn install_extension(&self, name: &str)
        -> BoxFuture<DbResult<Established<AuditLogged>>>;

    fn reload_config(&self)
        -> BoxFuture<DbResult<Established<AuditLogged>>>;
}
```

### `traits/database.rs` — Database Lifecycle

```rust
/// Manage the set of databases on a server.
/// Source: ISO/IEC 9075-2 + PostgreSQL CREATE/DROP DATABASE
pub trait DbDatabaseManager: Send + Sync {
    fn create_database(&self, name: &str)
        -> BoxFuture<DbResult<(Established<TableCreated>, Established<AuditLogged>)>>;
        // TableCreated is the closest DDL prop; DatabaseCreated is a valid alias

    fn drop_database(&self, name: &str)
        -> BoxFuture<DbResult<Established<AuditLogged>>>;

    fn list_databases(&self)
        -> BoxFuture<DbResult<Vec<String>>>;

    fn rename_database(&self, from: &str, to: &str)
        -> BoxFuture<DbResult<Established<AuditLogged>>>;

    fn database_size(&self, name: &str)
        -> BoxFuture<DbResult<u64>>;  // bytes
}
```

### `traits/schema.rs` — Schema Management

```rust
/// Manage schemas within a database.
/// Source: ISO/IEC 9075-2 §11.1 — <schema definition>
pub trait DbSchemaManager: Send + Sync {
    fn create_schema(&self, name: &str)
        -> BoxFuture<DbResult<(Established<SchemaExists>, Established<AuditLogged>)>>;

    fn drop_schema(&self, name: &str, cascade: bool)
        -> BoxFuture<DbResult<Established<AuditLogged>>>;

    fn list_schemas(&self)
        -> BoxFuture<DbResult<Vec<String>>>;

    fn schema_info(&self, name: &str)
        -> BoxFuture<DbResult<DbSchema>>;
}
```

### `traits/table.rs` — Table Management

```rust
/// Full table lifecycle: create, alter, drop, inspect.
/// Source: ISO/IEC 9075-2 §11.3 — <table definition>
pub trait DbTableManager: Send + Sync {
    fn create_table(&self, schema: &str, name: &str, columns: Vec<DbColumn>)
        -> BoxFuture<DbResult<(Established<TableCreated>, Established<ReferentialIntegrityMaintained>)>>;

    fn drop_table(&self, schema: &str, name: &str, cascade: bool)
        -> BoxFuture<DbResult<Established<AuditLogged>>>;

    fn list_tables(&self, schema: &str)
        -> BoxFuture<DbResult<Vec<DbTableInfo>>>;

    fn inspect_table(&self, schema: &str, name: &str)
        -> BoxFuture<DbResult<(DbTableInfo, Established<TableExists>)>>;

    fn add_column(&self, schema: &str, table: &str, column: DbColumn)
        -> BoxFuture<DbResult<(Established<ColumnExists>, Established<AuditLogged>)>>;

    fn drop_column(&self, schema: &str, table: &str, column: &str)
        -> BoxFuture<DbResult<Established<AuditLogged>>>;

    fn rename_table(&self, schema: &str, from: &str, to: &str)
        -> BoxFuture<DbResult<Established<AuditLogged>>>;

    fn truncate_table(&self, schema: &str, name: &str)
        -> BoxFuture<DbResult<Established<AuditLogged>>>;
}
```

### `traits/query.rs` — Query Execution

```rust
/// Execute SQL queries and DML. The core data-plane trait.
/// Source: ISO/IEC 9075-2 §14 — Data manipulation
pub trait DbQueryExecutor: Send + Sync {
    /// Execute a DML statement (INSERT/UPDATE/DELETE), return affected row count.
    fn execute(&self, sql: &str, params: &[DbValue])
        -> BoxFuture<DbResult<(u64, Established<AuditLogged>)>>;

    /// Execute a SELECT, return rows.
    fn query_rows(&self, sql: &str, params: &[DbValue])
        -> BoxFuture<DbResult<(DbRows, Established<RowVisible>)>>;

    /// EXPLAIN a query — no rows modified.
    fn explain(&self, sql: &str, analyze: bool)
        -> BoxFuture<DbResult<DbExplain>>;

    /// Execute inside a transaction, returning commit proof.
    fn execute_in_transaction(
        &self,
        sql: &str,
        params: &[DbValue],
        isolation: IsolationLevel,
    ) -> BoxFuture<DbResult<(u64, Established<TransactionCommitted>, Established<AuditLogged>)>>;
}
```

### `traits/transaction.rs` — Transaction Control

```rust
/// Fine-grained transaction management with savepoints.
/// Source: ISO/IEC 9075-2 §17 — Transaction management
pub trait DbTransactor: Send + Sync {
    fn begin(&self, isolation: IsolationLevel)
        -> BoxFuture<DbResult<TransactionHandle>>;

    fn commit(&self, handle: TransactionHandle)
        -> BoxFuture<DbResult<(Established<TransactionCommitted>, Established<Durable>)>>;

    fn rollback(&self, handle: TransactionHandle)
        -> BoxFuture<DbResult<()>>;

    fn savepoint(&self, handle: &TransactionHandle, name: &str)
        -> BoxFuture<DbResult<()>>;

    fn rollback_to_savepoint(&self, handle: &TransactionHandle, name: &str)
        -> BoxFuture<DbResult<()>>;
}
```

### `traits/index.rs` — Index Management

```rust
/// Create, drop, and inspect indexes.
/// Source: PostgreSQL docs §11 — Indexes
pub trait DbIndexManager: Send + Sync {
    fn create_index(&self, table: &str, columns: &[&str], unique: bool)
        -> BoxFuture<DbResult<(Established<IndexExists>, Established<AuditLogged>)>>;

    fn drop_index(&self, name: &str)
        -> BoxFuture<DbResult<Established<AuditLogged>>>;

    fn list_indexes(&self, table: &str)
        -> BoxFuture<DbResult<Vec<DbIndexInfo>>>;

    fn reindex(&self, table: &str)
        -> BoxFuture<DbResult<Established<AuditLogged>>>;
}
```

### `traits/role.rs` — Role & Privilege Management

```rust
/// PostgreSQL role management and GRANT/REVOKE.
/// Source: ISO/IEC 9075-2 §12 — Access control + ISO 27001 §A.5.15
pub trait DbRoleManager: Send + Sync {
    fn create_role(&self, name: &str, can_login: bool, superuser: bool)
        -> BoxFuture<DbResult<(Established<AccessAuthorized>, Established<AuditLogged>)>>;

    fn drop_role(&self, name: &str)
        -> BoxFuture<DbResult<Established<AuditLogged>>>;

    fn list_roles(&self)
        -> BoxFuture<DbResult<Vec<DbRoleInfo>>>;

    fn grant(&self, privilege: &str, on: &str, to: &str)
        -> BoxFuture<DbResult<(Established<AccessAuthorized>, Established<AuditLogged>)>>;

    fn revoke(&self, privilege: &str, on: &str, from: &str)
        -> BoxFuture<DbResult<(Established<LeastPrivilegeEnforced>, Established<AuditLogged>)>>;
}
```

### `traits/monitor.rs` — Performance Monitoring

```rust
/// Runtime monitoring: sessions, locks, slow queries, bloat.
/// Source: PostgreSQL pg_stat_* system views
pub trait DbMonitor: Send + Sync {
    fn active_sessions(&self)
        -> BoxFuture<DbResult<DbStatActivity>>;

    fn slow_queries(&self, threshold_ms: u64)
        -> BoxFuture<DbResult<Vec<DbSessionInfo>>>;

    fn table_bloat(&self, schema: &str)
        -> BoxFuture<DbResult<Vec<(String, f64)>>>;  // (table, bloat_ratio)

    fn index_usage(&self, schema: &str)
        -> BoxFuture<DbResult<Vec<(String, u64)>>>;  // (index, scans)

    fn lock_waits(&self)
        -> BoxFuture<DbResult<Vec<(i32, i32)>>>;  // (waiting_pid, blocking_pid)

    fn cache_hit_ratio(&self)
        -> BoxFuture<DbResult<f64>>;  // 0.0–1.0
}
```

### `traits/backup.rs` — Backup & Recovery

```rust
/// Backup initiation, listing, and verification.
/// Source: PostgreSQL docs §26 + ISO SQL (durability)
pub trait DbBackupManager: Send + Sync {
    fn initiate_backup(&self, label: &str)
        -> BoxFuture<DbResult<(Established<BackupConsistent>, Established<AuditLogged>)>>;

    fn list_backups(&self)
        -> BoxFuture<DbResult<Vec<String>>>;

    fn verify_backup(&self, label: &str)
        -> BoxFuture<DbResult<Established<BackupConsistent>>>;

    fn wal_status(&self)
        -> BoxFuture<DbResult<Established<WALReplayable>>>;
}
```

### Composite capability trait

A full pgAdmin-style backend implements all of the above:

```rust
/// Complete database management backend.
///
/// Implement this for any database driver (sqlx, diesel, tiberius, libpq).
/// All consumers depend only on this interface — not the driver.
pub trait DbBackend:
    DbSessionManager
    + DbServerAdmin
    + DbDatabaseManager
    + DbSchemaManager
    + DbTableManager
    + DbQueryExecutor
    + DbTransactor
    + DbIndexManager
    + DbRoleManager
    + DbMonitor
    + DbBackupManager
    + Send
    + Sync
{}

// Blanket impl: anything that impls all sub-traits gets DbBackend for free
impl<T> DbBackend for T
where
    T: DbSessionManager + DbServerAdmin + DbDatabaseManager + DbSchemaManager
     + DbTableManager + DbQueryExecutor + DbTransactor + DbIndexManager
     + DbRoleManager + DbMonitor + DbBackupManager + Send + Sync
{}
```

Usage at application level:

```rust
async fn get_table_info(db: &dyn DbTableManager, schema: &str, name: &str) {
    let (info, Established::<TableExists> { .. }) = db.inspect_table(schema, name).await?;
    // The destructured proof token is evidence — caller doesn't need to recheck
}
```

---

## Composite Props (convenience aliases)

```rust
/// Full ACID transaction contract.
/// Source: ISO SQL + Gray & Reuter "Transaction Processing"
pub type AcidCommitted = (
    Established<TransactionCommitted>,
    Established<Atomic>,
    Established<Durable>,
    Established<Serializable>,
);

/// PostgreSQL production-ready write.
pub type PgSafeWrite = (
    Established<RowInserted>,
    Established<AuditLogged>,
    Established<MVCCSnapshotValid>,
    Established<WALReplayable>,
);
```

---

## elicitation Primitives (feature: `db-types`)

```
crates/elicitation/src/primitives/db_types/
├── mod.rs
├── enums.rs        # IsolationLevel, DbOperation, DbValueKind
└── descriptors.rs  # DbQueryDescriptor, DbSchemaDescriptor, DbMigrationDescriptor
```

These feed into MCP tools in a future `elicit_sqlx` or `elicit_diesel` shadow crate.
`elicit_db` itself has no MCP tools — it is a pure interface/contracts crate.

---

## What elicit_db is NOT

- Not a query builder (that's `elicit_sqlx`)
- Not an ORM wrapper (that's `elicit_sea_orm` or `elicit_diesel`)
- Not an MCP tool crate — no `ElicitPlugin` impls here
- Not tied to any async runtime or DB driver

---

## Cargo.toml

```toml
[package]
name = "elicit_db"
version.workspace = true
edition.workspace = true
license.workspace = true
description = "Database contract interface — Props, traits, and typestate for SQL boundaries"
keywords = ["database", "contracts", "typestate", "sql", "elicitation"]
categories = ["database", "rust-patterns"]

[dependencies]
elicitation = { workspace = true, features = ["emit"] }
serde = { workspace = true }
schemars = { workspace = true }
derive_more = { workspace = true }
thiserror = { workspace = true, optional = true }

[features]
default = []
emit = ["elicitation/emit"]
```

---

## Implementation Order

1. `error.rs` + `contracts/` (8 modules, all Props)
2. `typestate.rs` (Transaction/Query state machines)
3. `traits.rs` (DbConnection, DbTransaction, DbQuery)
4. `crates/elicitation` primitives (`db-types` feature)
5. Tests: verify Props compile, typestate transitions correct, `dyn DbBackend` is object-safe
6. README: trait family reference with standard citations per method

---

## Non-Goals (explicitly deferred)

- `elicit_sqlx` — MCP tools wrapping sqlx (future crate, depends on `elicit_db`)
- Migration tooling (sqlx migrate / refinery) — deferred
- Query builder DSL — deferred
- Connection pooling traits — deferred (deadpool / bb8 are runtime concerns)
- Actual sqlx/diesel/sea-orm implementations — separate impl crates, not here
