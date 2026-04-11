# elicit_db

**Database contract interface** — Props, typestate markers, and a complete family of object-safe async traits for pgAdmin-style database management.

This is an **interface crate**, not an implementation. DB drivers (sqlx, diesel, sea-orm) implement the traits; consumers depend on this crate only.

---

## Design

`elicit_db` is the *dictionary*, not the *paragraph*. It defines a comprehensive, standards-anchored vocabulary of database contracts.

### Why contract return types instead of associated types

Traditional trait design uses associated types for result shapes, which breaks object safety:

```rust
// NOT this — associated types break object safety
trait DbExecutor {
    type Row;
    fn query(&self, sql: &str) -> Result<Vec<Self::Row>, Error>;
}
```

`elicit_db` uses `Established<P>` tuples instead:

```rust
// THIS — object-safe, self-documenting
trait DbQueryExecutor: Send + Sync {
    fn query_rows(
        &self, sql: &str, params: &[DbValue],
    ) -> BoxFuture<'_, DbResult<(DbRows, Established<RowVisible>)>>;
}
```

Benefits:
- `dyn DbTableManager` works — no associated types to bind
- The return type IS the contract: callers know exactly what was proven
- Composable: `(Established<RowInserted>, Established<AuditLogged>)` reads like prose
- All implementations speak the same language regardless of underlying driver

---

## Usage

```toml
[dependencies]
elicit_db = { workspace = true }
```

### Object-safe trait dispatch

```rust
use elicit_db::{DbTableManager, DbColumn, TableCreated, AuditLogged};
use elicitation::Established;

async fn ensure_users_table(mgr: &dyn DbTableManager) {
    let cols = vec![
        DbColumn {
            name: "id".into(), ty: "bigint".into(),
            nullable: false, default_value: None, primary_key: true,
        },
        DbColumn {
            name: "email".into(), ty: "text".into(),
            nullable: false, default_value: None, primary_key: false,
        },
    ];
    let (_, _): (Established<TableCreated>, Established<AuditLogged>) =
        mgr.create_table("public", "users", cols).await.unwrap();
}
```

### Transaction typestate

```rust
use elicit_db::{DbTransactor, IsolationLevel, TxMarker, Open, Committed};

async fn run_transaction(db: &dyn DbTransactor) {
    let (handle, marker) = db.begin(IsolationLevel::Serializable).await.unwrap();
    assert_eq!(marker.isolation, IsolationLevel::Serializable);

    let (committed_marker, _, _) = db.commit(handle).await.unwrap();
    let _: TxMarker<Committed> = committed_marker;
}
```

### Proof token destructuring

```rust
use elicit_db::{DbRoleManager, AccessAuthorized, AuditLogged};
use elicitation::Established;

async fn create_app_role(roles: &dyn DbRoleManager) {
    let (Established::<AccessAuthorized> { .. }, Established::<AuditLogged> { .. }) =
        roles.create_role("app_user", true, false).await.unwrap();
}
```

---

## Trait Family Reference

| Trait | Purpose | Source |
|-------|---------|--------|
| [`DbSessionManager`] | Connect, disconnect, list/terminate sessions | PG §55.2, §28.2 |
| [`DbServerAdmin`] | Server version, settings, extensions | PG §20, §54.16 |
| [`DbDatabaseManager`] | Create/drop/list/rename databases | ISO 9075-2 §17 |
| [`DbSchemaManager`] | Create/drop/list schemas | ISO 9075-2 §11.1 |
| [`DbTableManager`] | Create/alter/drop/inspect tables and columns | ISO 9075-2 §11 |
| [`DbQueryExecutor`] | Execute SQL, query rows, EXPLAIN | ISO 9075-2 §14 |
| [`DbTransactor`] | Begin/commit/rollback/savepoint | ISO 9075-2 §17 |
| [`DbIndexManager`] | Create/drop/list/reindex | PG §11 |
| [`DbRoleManager`] | Create/drop roles, grant/revoke privileges | PG §22.1, ISO 9075-2 §12 |
| [`DbMonitor`] | Sessions, slow queries, bloat, locks, cache | PG §28 |
| [`DbBackupManager`] | Backup initiation, listing, verification, WAL | PG §26, §30 |
| [`DbBackend`] | Blanket supertrait for all 11 above | — |

---

## Contract Module Reference

### `contracts::iso_sql` — ISO/IEC 9075

| Prop | Standard | Meaning |
|------|----------|---------|
| `TableCreated` | §11.3 | DDL `CREATE TABLE` succeeded |
| `ConstraintSatisfied` | §11.6 | All table constraints hold |
| `ReferentialIntegrityMaintained` | §11.8 | No dangling foreign keys |
| `ViewCreated` | §11.32 | DDL `CREATE VIEW` succeeded |
| `RowInserted` | §14.8 | At least one row was inserted |
| `RowUpdated` | §14.11 | At least one row was updated |
| `RowDeleted` | §14.7 | At least one row was deleted |
| `NonEmptyResult` | §14.1 | Query returned ≥1 row |
| `TransactionCommitted` | §17.3 | Transaction committed durably |
| `Atomic` | §4.33 | Operation is atomic (A in ACID) |
| `Durable` | §4.33 | Committed data survives failure (D in ACID) |
| `DatabaseCreated` | §17 | `CREATE DATABASE` succeeded |
| `SchemaCreated` | §11.1 | `CREATE SCHEMA` succeeded |

### `contracts::isolation` — ANSI SQL-92

| Prop | Standard | Meaning |
|------|----------|---------|
| `ReadUncommittedIsolation` | ANSI Table 2 | Transaction at READ UNCOMMITTED |
| `ReadCommittedIsolation` | ANSI Table 2 | Transaction at READ COMMITTED |
| `RepeatableReadIsolation` | ANSI Table 2 | Transaction at REPEATABLE READ |
| `SerializableIsolation` | ANSI Table 2 | Transaction at SERIALIZABLE |
| `PreventsDirtyRead` / `NoDirtyReads` | §4.28 P1 | P1 phenomenon prevented |
| `PreventsNonRepeatableRead` | §4.28 P2 | P2 phenomenon prevented |
| `PreventsPhantomRead` / `NoPhantomReads` | §4.28 P3 | P3 phenomenon prevented |
| `PreventsDirtyWrite` | Berenson P0 | P0 phenomenon prevented |

### `contracts::postgres` — PostgreSQL

| Prop | Source | Meaning |
|------|--------|---------|
| `MVCCSnapshotValid` | §13.1 | Valid MVCC snapshot acquired |
| `SnapshotIsolation` | §13.2.2 | Snapshot isolation in effect |
| `AdvisoryLockHeld` | §13.3.5 | Advisory lock currently held |
| `RowVisible` | §13.1 | Row visible to current snapshot |
| `IndexExists` | §11 | Named index exists |
| `VacuumedRecently` | §25.1 | Table vacuumed recently |

### `contracts::information_schema` — ISO/IEC 9075-11

| Prop | View | Meaning |
|------|------|---------|
| `TableExists` | `TABLES` | Table present in catalog |
| `ColumnExists` | `COLUMNS` | Column present on table |
| `SchemaExists` | `SCHEMATA` | Schema present in catalog |
| `ForeignKeyExists` | `REFERENTIAL_CONSTRAINTS` | Foreign key constraint exists |

### `contracts::security` — ISO/IEC 27001:2022

| Prop | Clause | Meaning |
|------|--------|---------|
| `AccessAuthorized` | §A.5.15 | Access control check passed |
| `AuditLogged` | §A.8.15 | Operation recorded in audit log |
| `LeastPrivilegeEnforced` | §A.5.15 | Minimum necessary privileges applied |
| `EncryptedAtRest` | §A.8.24 | Data at rest encrypted |
| `EncryptedInTransit` | §A.8.24 | Data in transit encrypted |

### `contracts::recovery` — PostgreSQL docs §26, §30

| Prop | Source | Meaning |
|------|--------|---------|
| `BackupConsistent` | §26 | Backup is consistent and restorable |
| `WALReplayable` | §30 | WAL segment intact and replayable |
| `PointInTimeRecoverable` | §26.3 | PITR is possible from this state |

### `contracts::transport` — PG Protocol §55, RFC 7159

| Prop | Source | Meaning |
|------|--------|---------|
| `RequestWellFormed` | PG §55 | Request conforms to wire protocol |
| `ResponseSerializable` | RFC 7159 | Response is valid JSON |
| `ConnectionEstablished` | PG §55.2 | TCP/TLS connection established |

### `contracts::observability` — OpenTelemetry

| Prop | Source | Meaning |
|------|--------|---------|
| `TraceEmitted` | OTel §traces | Trace emitted for this operation |
| `SpanLinkedToOperation` | OTel §span-links | Span linked to DB operation |
| `MetricsRecorded` | OTel DB Conventions | Metrics recorded |

---

## Typestate Reference

```
TxMarker<Open>  →  .commit()   →  TxMarker<Committed>
TxMarker<Open>  →  .rollback() →  TxMarker<RolledBack>
```

| Marker | Meaning |
|--------|---------|
| `Open` | Transaction active, awaiting commit/rollback |
| `Committed` | Transaction durably committed |
| `RolledBack` | Transaction rolled back, changes discarded |
| `Prepared` | Query built but not yet executed |
| `Executed` | Query executed, results available |

---

## Standards Grounding

| Standard | Coverage |
|----------|----------|
| ISO/IEC 9075-2 (SQL) | DDL, DML, transactions, constraints |
| ISO/IEC 9075-11 (SQL/Schemata) | INFORMATION_SCHEMA introspection |
| ANSI SQL-92 | Isolation levels, read phenomena |
| Berenson et al. 1995 | Extended isolation phenomena (P0) |
| PostgreSQL documentation | MVCC, advisory locks, monitoring, backup/WAL |
| ISO/IEC 27001:2022 | Access control, audit, encryption |
| OpenTelemetry Specification | Traces, spans, metrics |
| IETF RFC 7159 | JSON wire format |

## License

Apache-2.0 OR MIT
