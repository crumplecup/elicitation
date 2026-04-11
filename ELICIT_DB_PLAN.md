# elicit_db — Database Contract Interface

> **Role:** Interface crate — like `elicit_ui`, not a shadow crate.
> **Purpose:** Define the domain boundary for database interactions. Provides Props,
> typestate markers, traits, and composite contract types that both DB implementations
> and consumers (axum handlers, leptos server fns, UI layers) program against.
> **No DB driver dependency.** Implementations (sqlx, diesel, sea-orm) live elsewhere.

---

## Design Philosophy

`elicit_db` is the *dictionary*, not the *paragraph*. It defines a comprehensive,
standards-anchored vocabulary of database contracts. Users reach for what they need.

Analogies:
- `elicit_ui` → WCAG Props + accessibility typestate
- `elicit_db` → ISO SQL + ANSI isolation + PostgreSQL + ISO 27001 Props + DB typestate

The multi-standard stack from `elicit_db_plan.md` is the normative grounding for each
Prop. Every Prop maps to either an ISO clause or a documented PostgreSQL guarantee.

---

## Crate Structure

```
crates/elicit_db/
├── Cargo.toml
├── README.md
└── src/
    ├── lib.rs               # mod + pub use only
    ├── typestate.rs         # Transaction<S>, Query<S> state machine markers
    ├── traits.rs            # DbConnection, DbTransaction, DbQuery traits
    ├── error.rs             # DbError / DbErrorKind
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

## Traits: `traits.rs`

```rust
/// A database connection capable of beginning transactions.
pub trait DbConnection: Send + Sync {
    type Transaction<'a>: DbTransaction where Self: 'a;
    fn begin(&mut self) -> impl Future<Output = DbResult<Transaction<Open>>>;
}

/// A transaction that can execute queries and commit/rollback.
pub trait DbTransaction: Send + Sync {
    fn execute(&mut self, sql: &str, params: &[DbValue])
        -> impl Future<Output = DbResult<u64>>;
    fn query(&mut self, sql: &str, params: &[DbValue])
        -> impl Future<Output = DbResult<Vec<DbRow>>>;
    fn commit(self) -> impl Future<Output = DbResult<()>>;
    fn rollback(self) -> impl Future<Output = DbResult<()>>;
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
5. Tests: verify Props compile, typestate transitions are correct
6. README: contract reference with standard citations

---

## Non-Goals (explicitly deferred)

- `elicit_sqlx` — MCP tools wrapping sqlx (future crate, depends on elicit_db)
- Migration tooling (sqlx migrate / refinery) — deferred
- Query builder DSL — deferred
- Connection pooling traits — deferred (deadpool / bb8 are runtime concerns)
