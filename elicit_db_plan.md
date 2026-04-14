# Contract-Oriented DB Interface — Standards-Anchored Implementation Plan

## Objective

Anchor DB contracts to **real normative sources**, equivalent in rigor to:

- WCAG → UI
- GAAP → Ledger

We construct a **composite standards stack** and map each layer → `Prop`.

---

# 1. Core SQL Semantics (FOUNDATION)

## Standard

- ISO/IEC 9075 — SQL Standard (multi-part)
  - Part 1: Framework (9075-1)
  - Part 2: Foundation / SQL/Foundation (9075-2)
  - Part 4: Persistent Stored Modules (9075-4)
  - Part 9: Management of External Data (9075-9)
  - Part 14: XML-related (9075-14)

## Public entry point

- <https://www.iso.org/standard/76583.html> (latest catalog entry)

## What it defines

- DDL / DML semantics
- Constraints (PK, FK, CHECK)
- Views
- Transactions (partially)
- Information Schema

## Contract extraction

### Props

```rust
#[derive(Prop)]
pub struct TableCreated {
    pub name: String,
}

#[derive(Prop)]
pub struct ConstraintSatisfied {
    pub constraint: String,
}

#[derive(Prop)]
pub struct ReferentialIntegrityMaintained;
```

## Notes

- ISO text is paywalled → use:
  - PostgreSQL docs (reference implementation)
  - publicly summarized behavior
- Treat ISO as **normative anchor**, not implementation guide

---

# 2. Transaction Semantics (ACID + SQL)

## Standards / Sources

### A. ISO SQL (9075-2)

Defines transactional structure

### B. ANSI/ISO Isolation Levels

Formalized phenomena:

- Dirty read
- Non-repeatable read
- Phantom read

### C. Classic reference (non-ISO but canonical)

- Jim Gray, "Transaction Processing: Concepts and Techniques"

## Contract extraction

```rust
#[derive(Prop)]
pub struct TransactionCommitted;

#[derive(Prop)]
pub struct Atomic;

#[derive(Prop)]
pub struct Durable;

#[derive(Prop)]
pub struct NoDirtyReads;

#[derive(Prop)]
pub struct NoPhantomReads;
```

---

# 3. Isolation Levels (PRECISE BEHAVIORAL LAYER)

## Standard

- Defined in SQL standard + widely summarized here:
  <https://en.wikipedia.org/wiki/Isolation_(database_systems)>

## Formal phenomena model

- P0: Dirty write
- P1: Dirty read
- P2: Non-repeatable read
- P3: Phantom

## Contract extraction

```rust
#[derive(Prop)]
pub struct ReadCommitted;

#[derive(Prop)]
pub struct RepeatableRead;

#[derive(Prop)]
pub struct Serializable;
```

AND optionally:

```rust
#[derive(Prop)]
pub struct PreventsPhenomenon<const P: u8>;
```

---

# 4. PostgreSQL (DE FACTO EXECUTION STANDARD)

## Authoritative docs

- <https://www.postgresql.org/docs/current/index.html>

Critical sections:

- MVCC:
  <https://www.postgresql.org/docs/current/mvcc.html>
- Transactions:
  <https://www.postgresql.org/docs/current/tutorial-transactions.html>
- Explicit locking:
  <https://www.postgresql.org/docs/current/explicit-locking.html>

## Why this matters

ISO defines intent.
PostgreSQL defines **actual behavior you must match**.

## Contract extraction

```rust
#[derive(Prop)]
pub struct MVCCSnapshotValid;

#[derive(Prop)]
pub struct SnapshotIsolation;

#[derive(Prop)]
pub struct AdvisoryLockHeld {
    pub key: i64,
}

#[derive(Prop)]
pub struct RowVisible;
```

---

# 5. Information Schema (INTROSPECTION STANDARD)

## Standard

- ISO/IEC 9075-11 (Information and Definition Schemas)

## PostgreSQL reference

- <https://www.postgresql.org/docs/current/information-schema.html>

## What it gives you

- Portable schema inspection layer

## Contract extraction

```rust
#[derive(Prop)]
pub struct TableExists {
    pub name: String,
}

#[derive(Prop)]
pub struct ColumnExists {
    pub table: String,
    pub column: String,
}
```

---

# 6. Security & Access Control

## Standard

- ISO/IEC 27001 — Information Security Management
  <https://www.iso.org/isoiec-27001-information-security.html>

## Supporting standard

- ISO/IEC 27002 (controls catalogue)

## What it defines

- Access control
- Audit logging
- Least privilege
- Risk controls

## Contract extraction

```rust
#[derive(Prop)]
pub struct AccessAuthorized {
    pub action: String,
}

#[derive(Prop)]
pub struct AuditLogged;

#[derive(Prop)]
pub struct LeastPrivilegeEnforced;
```

---

# 7. Backup / Durability / Recovery

## Standards / references

### A. ISO SQL (partial)

- Backup not deeply standardized

### B. PostgreSQL WAL + backup docs

- <https://www.postgresql.org/docs/current/continuous-archiving.html>

### C. Industry practice (de facto)

- Point-in-time recovery (PITR)

## Contract extraction

```rust
#[derive(Prop)]
pub struct BackupConsistent;

#[derive(Prop)]
pub struct WALReplayable;

#[derive(Prop)]
pub struct PointInTimeRecoverable;
```

---

# 8. Networking / Wire Protocol (OPTIONAL BUT STRONG)

If your traits cross process boundaries:

## Standards

### A. PostgreSQL Wire Protocol

- <https://www.postgresql.org/docs/current/protocol.html>

### B. IETF RFCs (if HTTP/JSON transport used)

- :contentReference[oaicite:0]{index=0}
- :contentReference[oaicite:1]{index=1}

## Contract extraction

```rust
#[derive(Prop)]
pub struct RequestWellFormed;

#[derive(Prop)]
pub struct ResponseSerializable;
```

---

# 9. Observability (OPTIONAL BUT POWERFUL)

## Standard / Spec

- :contentReference[oaicite:2]{index=2}
  <https://opentelemetry.io/docs/>

## Contract extraction

```rust
#[derive(Prop)]
pub struct TraceEmitted;

#[derive(Prop)]
pub struct SpanLinkedToOperation;
```

---

# 10. How This Maps to Your Trait System

## Example: create_database

```rust
fn create_database(
    &self,
    input: CreateDatabase,
) -> Result<
    (
        Established<DatabaseCreated>,
        Established<AuditLogged>,
        Established<AccessAuthorized>,
    ),
    DbError,
>;
```

Each returned Prop is backed by:

| Prop              | Standard Source              |
|------------------|-----------------------------|
| DatabaseCreated  | ISO SQL (DDL semantics)     |
| AuditLogged      | ISO 27001                   |
| AccessAuthorized | ISO 27001                   |

---

# 11. Implementation Rule (CRITICAL)

Every Prop MUST:

1. Map to a standard clause OR
2. Map to a documented PostgreSQL guarantee

If neither:
→ it is not a valid contract

---

# 12. Suggested Crate Structure (Standards-Aligned)

db-contracts/
├── iso_sql.rs
├── isolation.rs
├── postgres.rs
├── information_schema.rs
├── security_iso27001.rs
├── recovery.rs
├── transport_rfc.rs
└── observability.rs

Each file:

- cites standard
- defines Props
- documents invariants

---

# 13. Traceability Requirement

Every Prop should include doc comments:

```rust
/// Established when a transaction is durably committed.
/// Source:
/// - ISO/IEC 9075-2
/// - PostgreSQL WAL documentation
#[derive(Prop)]
pub struct TransactionCommitted;
```

---

# Final Position

There is no single “GAAP for databases.”

Instead, the correct move is:

→ Compose a **multi-standard contract surface**

Stack:

- ISO/IEC 9075 (SQL semantics)
- ANSI isolation model (behavior)
- PostgreSQL docs (execution truth)
- ISO 27001 (security)
- RFCs (transport)
- OpenTelemetry (observability)

This gives you:

- Normative grounding
- Practical implementability
- Verifiable semantics

Anything less is not strong enough to support your architecture.
