# elicit_db

Formally verified database management with compile-time contract enforcement,
built on a proof-carrying trait interface anchored to ISO/IEC 9075, ANSI SQL-92,
PostgreSQL documentation, and ISO/IEC 27001:2022.

## Overview

`elicit_db` models database management as a proof-carrying pipeline. Every
mutating operation is performed through a factory trait that executes the
statement and, on success, returns both the result descriptor **and** a typed
proof token — an `Established<P>` — that records what contract was satisfied.
Proof tokens compose upward through evidence bundles into aggregate proofs
(ACID compliance, schema integrity, security posture), which are the only legal
way to assert those compound invariants.

The compiler enforces this chain. There is no way to produce
`Established<AcidCompliant>` without assembling the leaf isolation and
durability proofs from which it is derived.

This is an **interface crate**, not an implementation. DB drivers (sqlx, diesel,
sea-orm) implement the traits; consumers depend only on this crate. The
reference implementation is `elicit_sqlx::SqlxDbBackend` in the `elicit_sqlx`
crate, which implements all 20 `DbBackend` sub-traits against a `sqlx` connection
pool.

---

## Architecture

```text
  ┌─────────────────────────────────────────────────────────────────────────┐
  │                     Role 1a — Leaf Factory Traits                        │
  │                                                                          │
  │  DbSessionManager · DbServerAdmin · DbDatabaseManager · DbSchemaManager  │
  │  DbTableManager · DbQueryExecutor · DbTransactor · DbIndexManager        │
  │  DbRoleManager · DbBackupManager                                         │
  │  DbRoutineFactory · DbConstraintFactory · DbIsolationFactory             │
  │  DbSecurityFactory · DbReplicationFactory                                │
  └────────────────────────────┬─────────────────────────────────────────────┘
                               │ each method returns (Descriptor, Established<Leaf>)
                               ▼
  ┌─────────────────────────────────────────────────────────────────────────┐
  │              Role 1b — Evidence Bundle Composition                       │
  │                                                                          │
  │  ReadCommittedEvidence   →  Established<PreventsDirtyRead>               │
  │  SerializableEvidence    →  Established<PreventsPhantomRead> + 7 more    │
  │  AcidEvidence            →  Established<AcidCompliant>                   │
  │  SchemaIntegrityEvidence →  Established<SchemaIntegrityEstablished>      │
  │  RecoveryCapabilityEvidence → Established<PointInTimeRecoverable> + 2    │
  │  DataProtectionEvidence  →  Established<EncryptedAtRest> + InTransit     │
  │  StreamingReplicationEvidence → Established<StreamingReplicationConfigured>│
  └────────────────────────────┬─────────────────────────────────────────────┘
                               │ aggregate proofs flow to backend supertrait
                               ▼
  ┌─────────────────────────────────────────────────────────────────────────┐
  │                     Role 2 — Reporter Traits                             │
  │                                                                          │
  │  DbMonitor · DbRoutineMeta · DbConstraintMeta                            │
  │  DbSecurityMeta · DbReplicationMeta                                      │
  │                                                                          │
  │  Returns plain data. No proof tokens consumed or produced.               │
  └─────────────────────────────────────────────────────────────────────────┘
                               │
               ┌───────────────┼───────────────┐
               ▼               ▼               ▼
         elicit_sqlx      elicit_diesel    (your driver)
       (SqlxDbBackend)    (DieselBackend)
```

### Three-role taxonomy

| Role | Description | Return type | Traits |
|------|-------------|-------------|--------|
| **1a** (leaf factory) | Executes an operation; returns a fresh proof token on success | `DbResult<(Descriptor, Established<P>)>` | All factory traits |
| **1b** (section factory) | Consumes an evidence bundle of upstream tokens; mints an aggregate proof | `Established<P::prove(evidence)>` | `contracts::proof_composition` |
| **2** (reporter) | Queries backend state; no proof tokens produced or consumed | `BoxFuture<'_, DbResult<T>>` | `DbMonitor`, `*Meta` traits |

---

## Proof Architecture

### Proposition types

Every verifiable database contract has a corresponding Rust type — a
*proposition* — that implements `elicitation::contracts::Prop`. These types
are zero-cost phantoms that exist only at the type level.

```rust
pub struct TableCreated;         // DDL CREATE TABLE succeeded (ISO 9075-2 §11.3)
pub struct SerializableIsolation; // Transaction at SERIALIZABLE level (ANSI §4.28)
pub struct AuditLogged;          // Operation recorded in audit log (ISO 27001 §A.8.15)
pub struct AcidCompliant;        // Full ACID guarantee assembled from leaf proofs
```

### Proof tokens

`Established<P>` is the proof that proposition `P` holds. It is a zero-sized
type that carries no runtime data — only type-level evidence.

### The `ProvableFrom<C>` evidence path

The evidence-bundle minting path is `Established::prove`:

```rust
impl Established<P> {
    pub fn prove<C>(_: &C) -> Self  where P: ProvableFrom<C> { … }
}
```

`ProvableFrom<C>` declares "evidence bundle `C` proves proposition `P`". The
`proof_composition` module provides 17 evidence bundle types with 40+
`ProvableFrom` impls. For example:

```rust
pub struct SerializableEvidence {
    pub isolation: Established<SerializableIsolation>,
}

impl ProvableFrom<SerializableEvidence> for PreventsDirtyWrite {}
impl ProvableFrom<SerializableEvidence> for PreventsDirtyRead {}
impl ProvableFrom<SerializableEvidence> for PreventsNonRepeatableRead {}
impl ProvableFrom<SerializableEvidence> for PreventsPhantomRead {}
impl ProvableFrom<SerializableEvidence> for PreventsLostUpdate {}
impl ProvableFrom<SerializableEvidence> for PreventsWriteSkew {}
impl ProvableFrom<SerializableEvidence> for PreventsSerializationAnomaly {}
impl ProvableFrom<SerializableEvidence> for PreventsCircularInformationFlow {}
```

### The `Established::assert()` escape hatch

`Established::assert()` is `pub` as a general escape hatch, but any call to
`assert()` on a database proposition is immediately visible in code review and
audit tooling as an explicit bypass of the credential path.

---

## Proof Composition

Proofs compose bottom-up via the evidence bundle types in
`contracts::proof_composition`. The compiler rejects any gap in the chain.

```rust
// 1. Begin a typed transaction — leaf factory returns isolation proof
let (handle, marker, iso_proof) = db
    .begin_serializable()
    .await?;
// iso_proof: Established<SerializableIsolation>

// 2. Execute work, collect more leaf proofs
let (_, inserted) = db.insert_rows("public", "events", rows).await?;
// inserted: Established<RowInserted>

// 3. Commit — leaf factory returns durability proofs
let (_, committed, durable, audit) = db.commit(handle).await?;
// committed: Established<TransactionCommitted>
// durable:   Established<Durable>

// 4. Assemble ACID evidence bundle — all fields are required
let acid_evidence = AcidEvidence {
    atomicity:   Established::assert(),  // role 1a: DbTransactor covers A
    consistency: Established::assert(),  // role 1a: constraint checks cover C
    isolation:   Established::prove(&SerializableEvidence { isolation: iso_proof }),
    durability:  durable,
};

// 5. Mint the composite ACID proof
let acid_proof: Established<AcidCompliant> =
    Established::prove(&acid_evidence);

// If any leaf is missing, the evidence struct literal won't compile —
// there is no API to skip a field.
```

---

## Trait Interface

### Role 1a — Leaf factory traits (15 traits)

```rust
// Object administration
pub trait DbSessionManager: Send + Sync { /* connect, list_sessions, terminate */ }
pub trait DbServerAdmin: Send + Sync    { /* server_version, reload_config, extensions */ }
pub trait DbDatabaseManager: Send + Sync { /* create_database, drop_database, list_databases */ }
pub trait DbSchemaManager: Send + Sync  { /* create_schema, drop_schema, list_schemas */ }
pub trait DbTableManager: Send + Sync   { /* create_table, alter_table, drop_table */ }
pub trait DbQueryExecutor: Send + Sync  { /* execute, query_rows, explain */ }
pub trait DbTransactor: Send + Sync     { /* begin, commit, rollback, savepoint */ }
pub trait DbIndexManager: Send + Sync   { /* create_index, drop_index, reindex */ }
pub trait DbRoleManager: Send + Sync    { /* create_role, grant, revoke */ }
pub trait DbBackupManager: Send + Sync  { /* initiate_backup, list_backups, verify */ }

// Typed factories (new)
pub trait DbRoutineFactory: Send + Sync   { /* create_function, drop_function, alter_function, ... */ }
pub trait DbConstraintFactory: Send + Sync { /* add_check, add_primary_key, verify_constraints */ }
pub trait DbIsolationFactory: Send + Sync  { /* begin_read_committed, begin_serializable, ... */ }
pub trait DbSecurityFactory: Send + Sync   { /* enforce_tls, enable_rls, define_rls_policy, ... */ }
pub trait DbReplicationFactory: Send + Sync { /* create_publication, create_subscription, ... */ }
```

### Role 2 — Reporter traits (5 traits)

```rust
pub trait DbMonitor: Send + Sync         { /* active_sessions, slow_queries, table_bloat, locks */ }
pub trait DbRoutineMeta: Send + Sync     { /* list_functions, list_procedures, routine_info */ }
pub trait DbConstraintMeta: Send + Sync  { /* list_constraints, verify_constraints */ }
pub trait DbSecurityMeta: Send + Sync    { /* tls_status, hba_rules, security_settings */ }
pub trait DbReplicationMeta: Send + Sync { /* slot_lag, list_publications, streaming_status */ }
```

### `DbBackend` supertrait

```rust
pub trait DbBackend:
    DbSessionManager + DbServerAdmin + DbDatabaseManager + DbSchemaManager
    + DbTableManager + DbQueryExecutor + DbTransactor + DbIndexManager
    + DbRoleManager + DbMonitor + DbBackupManager
    + DbRoutineFactory + DbRoutineMeta
    + DbReplicationFactory + DbReplicationMeta
    + DbSecurityFactory + DbSecurityMeta
    + DbConstraintFactory + DbConstraintMeta
    + DbIsolationFactory
    + Send + Sync
{}
```

`DbBackend` is not itself object-safe (it is a supertrait of 20 traits), but
each sub-trait is individually object-safe: use `dyn DbTableManager`,
`dyn DbRoutineFactory`, etc. for dynamic dispatch at architectural boundaries.

---

## Contract Module Reference (668 propositions)

### `contracts::iso_sql` — ISO/IEC 9075-2:2023 (136 propositions)

Covers the full SQL standard: DDL lifecycle, DML operations, query semantics,
set operations, window functions, CTEs, sequences, and type management.

| Group | Representative propositions |
|-------|----------------------------|
| Table DDL | `TableCreated`, `TableAltered`, `TableDropped`, `TableRenamed` |
| Column DDL | `ColumnAdded`, `ColumnDropped`, `ColumnRenamed`, `ColumnDefaultSet` |
| View DDL | `ViewCreated`, `ViewDropped`, `ViewQueryValid` |
| Schema/DB | `SchemaCreated`, `SchemaDropped`, `DatabaseCreated`, `DatabaseDropped` |
| Domain/Sequence | `DomainCreated`, `SequenceCreated`, `SequenceAdvanced`, `SequenceCycled` |
| Type DDL | `TypeCreated`, `TypeDropped`, `IndexCreated`, `IndexDropped`, `IndexValid` |
| DML insert/update/delete | `RowInserted`, `RowUpdated`, `RowDeleted`, `RowMerged`, `TruncateRowsRemoved` |
| DML predicates | `SelectPredicateApplied`, `DeletePredicateApplied`, `UpdatedColumnSubsetValid` |
| Query semantics | `NonEmptyResult`, `EmptyResult`, `OrderByApplied`, `LimitApplied`, `OffsetApplied` |
| Aggregation | `GroupByApplied`, `GroupByRollupApplied`, `GroupByCubeApplied`, `HavingFilterApplied` |
| Subqueries | `SubqueryCorrelated`, `SubqueryUncorrelated` |
| ACID primitives | `Atomic`, `Consistent`, `Durable`, `TransactionCommitted`, `AcidCompliant` |
| Integrity | `ConstraintSatisfied`, `ReferentialIntegrityMaintained`, `AuditLogged` |

### `contracts::isolation` — ANSI SQL-92 + Berenson et al. 1995 (43 propositions)

| Group | Representative propositions |
|-------|----------------------------|
| Isolation levels | `ReadUncommittedIsolation`, `ReadCommittedIsolation`, `RepeatableReadIsolation`, `SerializableIsolation` |
| Phenomena prevention | `PreventsDirtyRead`, `PreventsDirtyWrite`, `PreventsNonRepeatableRead`, `PreventsPhantomRead`, `PreventsWriteSkew` |
| SSI (Serializable Snapshot) | `SsiPredicateLockHeld`, `SsiRwAntiDependencyTracked`, `SsiDangerousStructureAvoided` |
| Lock modes | `RowShareLockAcquired`, `ExclusiveLockAcquired`, `AccessExclusiveLockAcquired` |
| Lock behavior | `NoWaitRespected`, `SkipLockedApplied`, `DeadlockDetected`, `DeadlockResolved` |
| Timeouts | `StatementTimeoutRespected`, `DeadlockTimeoutRespected` |
| Savepoints | `SavepointEstablished`, `DeferrableConstraintChecked` |
| 2PC | `TwoPhaseCommitPrepared`, `TwoPhaseCommitFinalized` |
| Session/tx settings | `TransactionReadOnly`, `TransactionReadWrite`, `SessionIsolationLevelSet` |

### `contracts::constraints` — ISO/IEC 9075-2 §11.6 (43 propositions)

| Group | Representative propositions |
|-------|----------------------------|
| NOT NULL | `NotNullConstraintDefined`, `NotNullConstraintEnforced`, `NotNullConstraintDropped` |
| UNIQUE | `UniqueConstraintDefined`, `UniqueConstraintEnforced`, `UniqueConstraintDropped` |
| PRIMARY KEY | `PrimaryKeyDefined`, `PrimaryKeySingleColumn`, `PrimaryKeyMultiColumn`, `PrimaryKeyEnforced` |
| FOREIGN KEY | `ForeignKeyDefined`, `ForeignKeyEnforced`, `ForeignKeyOnDeleteCascade`, `ForeignKeyOnDeleteSetNull` |
| CHECK | `CheckConstraintDefined`, `CheckConstraintEvaluatesTrue`, `CheckConstraintViolationRejected` |
| Deferral | `ConstraintDeferrable`, `ConstraintInitiallyDeferred`, `ConstraintCheckedAtCommit` |
| Advanced | `ExclusionConstraintDefined`, `PartialIndexConstraintDefined`, `IdentityColumnDefined`, `GeneratedColumnDefined` |

### `contracts::access_control` — ISO/IEC 9075-2 §12 (48 propositions)

| Group | Representative propositions |
|-------|----------------------------|
| Table privileges | `TableSelectPrivilegeGranted`, `TableInsertPrivilegeGranted`, `TableUpdatePrivilegeGranted`, `TableAllPrivilegesGranted` |
| Column privileges | `ColumnSelectPrivilegeGranted`, `ColumnInsertPrivilegeGranted`, `ColumnUpdatePrivilegeGranted` |
| Schema/Sequence | `SchemaUsagePrivilegeGranted`, `SequenceUsagePrivilegeGranted`, `FunctionExecutePrivilegeGranted` |
| Role management | `RoleMembershipGranted`, `RoleMembershipRevoked`, `RoleAdminOptionGranted`, `RoleInheritanceActive` |
| Ownership | `ObjectOwnershipVerified`, `ObjectOwnershipTransferred`, `DefaultPrivilegesApplied` |
| Public role safety | `PublicRolePrivilegeLimited`, `PublicConnectRevokedFromDatabase` |
| RLS (Row Level Security) | `RlsSelectPolicyApplied`, `RlsInsertPolicyApplied`, `RlsBypassRoleExcluded` |
| Grant options | `PrivilegeGrantedWithGrantOption`, `PrivilegeRevokedCascade`, `PrivilegeRevokedRestrict` |

### `contracts::psm` — ISO/IEC 9075-4 (PSM) + PostgreSQL §39 (52 propositions)

| Group | Representative propositions |
|-------|----------------------------|
| Function lifecycle | `FunctionCreated`, `FunctionDropped`, `FunctionAltered`, `FunctionReturnTypeCorrect` |
| Language variants | `PlpgsqlFunctionCreated`, `SqlFunctionCreated`, `CFunctionCreated` |
| Volatility | `FunctionIsImmutable`, `FunctionIsStable`, `FunctionIsVolatile` |
| Security | `FunctionSecurityDefiner`, `FunctionSecurityInvoker`, `FunctionParallelSafe` |
| Procedure | `ProcedureCreated`, `ProcedureDropped`, `ProcedureTransactionControlAllowed` |
| Triggers | `TriggerFunctionCreated`, `TriggerBoundToTable`, `TriggerFiredOnInsert`, `TriggerEnabled` |
| Trigger timing | `TriggerFiredBefore`, `TriggerFiredAfter`, `TriggerFiredInsteadOf`, `TriggerFiredPerRow` |
| Aggregate | `AggregateFunctionCreated`, `AggregateSortedSetDefined` |
| PL/pgSQL | `PlpgsqlExceptionHandled`, `PlpgsqlRaisedWithSqlstate`, `AnonymousBlockExecuted` |

### `contracts::replication` — PostgreSQL §27 + §29 (45 propositions)

| Group | Representative propositions |
|-------|----------------------------|
| Streaming | `StreamingReplicationConfigured`, `PrimaryWalsenderActive`, `StandbyWalreceiverActive` |
| Synchronous | `SynchronousReplicationConfigured`, `SynchronousStandbyAcknowledgedWrite`, `QuorumSynchronousCommitAcknowledged` |
| Slots | `PhysicalReplicationSlotCreated`, `LogicalReplicationSlotCreated`, `ReplicationSlotActive`, `ReplicationSlotLagAcceptable` |
| Logical replication | `LogicalReplicationConfigured`, `LogicalDecodingActive`, `CdcChangeEventProduced` |
| Publication | `PublicationCreated`, `PublicationIncludesTable`, `PublicationRowFilterDefined` |
| Subscription | `SubscriptionCreated`, `SubscriptionActive`, `SubscriptionTableCopied` |
| Failover | `StandbyPromotedToPrimary`, `TimelineIdAdvanced`, `AutomaticFailoverTriggered` |
| WAL level | `WalLevelLogical`, `WalLevelReplica`, `WalLevelMinimal` |

### `contracts::security` — ISO/IEC 27001:2022 + PostgreSQL §19, §21 (49 propositions)

| Group | Representative propositions |
|-------|----------------------------|
| Access control | `AccessAuthorized`, `AccessDeniedCorrectly`, `LeastPrivilegeEnforced`, `NeedToKnowEnforced` |
| Audit | `AuditLogged`, `AuditLogTamperEvident`, `AuditLogRetentionMet`, `SecurityEventLogged` |
| Encryption | `EncryptedAtRest`, `EncryptedInTransit`, `EncryptionAlgorithmApproved`, `KeyRotationPerformed` |
| Row Level Security | `RowLevelSecurityEnabled`, `RowLevelSecurityPolicyDefined`, `RlsUsingClauseCorrect`, `RlsForcedForTableOwner` |
| Authentication | `AuthenticationSucceeded`, `ScramSha256AuthenticationUsed`, `CertificateAuthenticationUsed`, `MultiFactorAuthEnforced` |
| HBA / TLS | `SslModeRequired`, `SslCertificateVerified`, `TlsSessionActive`, `PgHbaRuleMatched`, `IpAllowlistEnforced` |
| Policy enforcement | `PasswordPolicyEnforced`, `SessionTimeoutEnforced`, `SqlInjectionPrevented` |
| Data protection | `SensitiveColumnMasked`, `PiiNotExposedInLogs`, `DataClassificationTagApplied` |

### `contracts::recovery` — PostgreSQL §26 + §30 (43 propositions)

| Group | Representative propositions |
|-------|----------------------------|
| Base backup | `BackupConsistent`, `BaseBackupCompleted`, `BaseBackupChecksumVerified`, `BaseBackupEncrypted` |
| Manifest | `BackupManifestGenerated`, `BackupManifestVerified` |
| pg_dump | `PgDumpCompleted`, `PgDumpAllCompleted`, `CustomFormatDumpUsed`, `DumpEncryptionApplied` |
| WAL archiving | `WALReplayable`, `WalArchivingEnabled`, `WalArchiveCommandSucceeded`, `WalFsyncConfirmed` |
| WAL safety | `WalChecksumEnabled`, `WalLevelSufficientForArchiving`, `WalRetentionPolicySatisfied` |
| PITR | `PointInTimeRecoverable`, `RecoveryTargetReached`, `PitrTestPassed` |
| Failover / recovery | `FailoverCompleted`, `SwitchoverCompleted`, `TimelineAdvanced`, `RecoveryCompletionEstimated` |

### `contracts::postgres` — PostgreSQL documentation (69 propositions)

| Group | Representative propositions |
|-------|----------------------------|
| MVCC | `MVCCSnapshotValid`, `SnapshotIsolation`, `RowVisible`, `HorizonAdvanced` |
| Wraparound prevention | `TransactionIdWraparoundPrevented`, `MultiXactWraparoundPrevented`, `VacuumFreezeApplied` |
| Indexes | `IndexExists`, `BTreeIndexCreated`, `GinIndexCreated`, `GistIndexCreated`, `IndexConcurrentlyBuilt` |
| Partitioning | `TablePartitioned`, `RangePartitionAttached`, `PartitionPruningApplied`, `PartitionBoundNonOverlapping` |
| Extensions | `ExtensionInstalled`, `PostgisExtensionActive`, `PgcryptoExtensionActive` |
| Advisory locks | `AdvisoryLockHeld`, `AdvisoryLockSessionHeld`, `AdvisoryLockReleased` |
| Autovacuum / health | `VacuumedRecently`, `TableAnalyzed`, `DeadTupleFractionLow`, `AutovacuumLaunched` |
| FDW | `ForeignDataWrapperInstalled`, `ForeignServerDefined`, `ForeignTableAccessible` |
| Misc DDL | `TriggerCreated`, `RlsPolicyCreated`, `MaterializedViewCreated`, `CompositeTypeCreated`, `EnumTypeCreated` |

### `contracts::information_schema` — ISO/IEC 9075-11 (56 propositions)

| Group | Representative propositions |
|-------|----------------------------|
| Schema catalog | `SchemaExists`, `SchemaDefaultCharacterSetDeclared`, `SchemaSqlPathDeclared` |
| Table catalog | `TableExists`, `BaseTableExists`, `ViewTableExists`, `TableIsInsertableInto` |
| Column catalog | `ColumnExists`, `ColumnDataTypeDeclared`, `ColumnIsNullable`, `ColumnDefaultValueDeclared` |
| View catalog | `ViewExists`, `ViewIsUpdatable`, `ViewCheckOptionDeclared` |
| Constraint catalog | `TableConstraintExists`, `PrimaryKeyConstraintRecorded`, `ForeignKeyConstraintRecorded`, `CheckConstraintRecorded` |
| Referential constraints | `ForeignKeyExists`, `ReferentialConstraintUpdateRuleDeclared`, `KeyColumnUsageRecorded` |
| Privilege catalog | `TablePrivilegeRecorded`, `ColumnPrivilegeRecorded`, `UsagePrivilegeRecorded` |
| Routine catalog | `RoutineExists`, `RoutineIsFunction`, `RoutineDataTypeDeclared`, `TriggerExistsInSchema` |
| Domain/Sequence | `DomainExists`, `DomainConstraintRecorded`, `SequenceExists`, `CollationExists` |

### `contracts::transport` — PostgreSQL wire protocol §55 + RFC 7159 (49 propositions)

| Group | Representative propositions |
|-------|----------------------------|
| Connection handshake | `ConnectionEstablished`, `ProtocolVersionNegotiated`, `SslNegotiationSucceeded`, `AuthenticationOkReceived` |
| TLS | `Tls13Preferred`, `TlsCertificateChainValid`, `TlsHostnameVerified`, `TlsCipherSuiteApproved` |
| JSON | `ResponseSerializable`, `JsonSchemaValid`, `JsonUtf8Encoded`, `JsonNestingDepthSafe` |
| Extended query | `ParseMessageSent`, `BindMessageSent`, `ExecuteMessageSent`, `ParseCompleteReceived` |
| Copy protocol | `CopyInResponseReceived`, `CopyOutResponseReceived`, `CopyDoneReceived` |
| SCRAM auth | `ScramClientFirstSent`, `ScramServerFirstReceived`, `ScramClientFinalSent`, `ScramServerFinalReceived` |
| Connection pool | `ConnectionPoolSlotAvailable`, `ConnectionPoolLimitRespected`, `ConnectionTimeoutRespected` |

### `contracts::observability` — OpenTelemetry Specification (35 propositions)

| Group | Representative propositions |
|-------|----------------------------|
| Tracing | `TraceEmitted`, `SpanLinkedToOperation`, `SpanContextPropagated`, `ChildSpanLinkedToParent` |
| Span attributes | `DbSystemAttributeSet`, `DbNameAttributeSet`, `DbStatementAttributeSet`, `DbRowsAffectedAttributeSet` |
| Events | `SlowQueryLogged`, `DeadlockEventLogged`, `ErrorLogged`, `StatementLogged` |
| Metrics | `MetricsRecorded`, `ConnectionPoolMetricRecorded`, `ReplicationLagMetricRecorded`, `QueryDurationHistogramPopulated` |
| Context | `TraceIdAttachedToQuery`, `CorrelationIdAttached`, `ServiceNameAttributeSet`, `BaggagePropagated` |

---

## Proof Composition Reference

The `contracts::proof_composition` module defines 17 evidence bundle types.
Each struct's fields are `Established<LeafProposition>` tokens — all must be
present for the bundle struct literal to compile.

### Isolation level chains

| Evidence bundle | Leaf field required | Propositions it proves |
|---|---|---|
| `ReadUncommittedEvidence` | `Established<ReadUncommittedIsolation>` | `PreventsDirtyWrite` |
| `ReadCommittedEvidence` | `Established<ReadCommittedIsolation>` | `PreventsDirtyWrite`, `PreventsDirtyRead` |
| `RepeatableReadEvidence` | `Established<RepeatableReadIsolation>` | `PreventsDirtyWrite`, `PreventsDirtyRead`, `PreventsNonRepeatableRead`, `PreventsLostUpdate` |
| `SnapshotIsolationEvidence` | `Established<SnapshotIsolation>` | Same as RepeatableRead |
| `SerializableEvidence` | `Established<SerializableIsolation>` | All 8 phenomena prevented, including `PreventsWriteSkew` + `PreventsSerializationAnomaly` |

### Aggregate chains

| Evidence bundle | Fields required | Proposition proved |
|---|---|---|
| `AcidEvidence` | `atomicity`, `consistency`, `isolation`, `durability` | `AcidCompliant` |
| `SchemaIntegrityEvidence` | `table_created`, `constraints_satisfied`, `referential_integrity` | `SchemaIntegrityEstablished` |
| `SecureAccessEvidence` | `access_authorized`, `least_privilege`, `audit_logged` | `AccessAuthorized` |
| `DataProtectionEvidence` | `encrypted_at_rest`, `encrypted_in_transit` | `EncryptedAtRest`, `EncryptedInTransit` |
| `AuthenticatedConnectionEvidence` | `connection`, `authentication` | `ConnectionEstablished`, `AccessAuthorized` |
| `RecoveryCapabilityEvidence` | `backup`, `wal`, `pitr` | `BackupConsistent`, `WALReplayable`, `PointInTimeRecoverable` |
| `TwoPhaseCommitFinalityEvidence` | `prepared`, `finalized` | `TwoPhaseCommitFinalized` |
| `StreamingReplicationEvidence` | `wal_sender`, `wal_receiver`, `applying` | `StreamingReplicationConfigured` |
| `LogicalReplicationEvidence` | `publication`, `subscription` | `LogicalReplicationConfigured` |
| `PrimaryKeyEnforcementEvidence` | `defined`, `enforced` | `PrimaryKeyEnforced` |
| `ForeignKeyEnforcementEvidence` | `defined`, `enforced` | `ForeignKeyEnforced` |

---

## Typestate Reference

Transaction lifecycle is encoded in the type parameter of `TxMarker<S>`:

```text
TxMarker<Open>  →  .commit()    →  TxMarker<Committed>
TxMarker<Open>  →  .rollback()  →  TxMarker<RolledBack>
```

| Marker | Source | Meaning |
|--------|--------|---------|
| `Open` | ISO 9075-2 §17.1 | Transaction active, awaiting commit or rollback |
| `Committed` | ISO 9075-2 §17.3 | Transaction durably committed |
| `RolledBack` | ISO 9075-2 §17.4 | Transaction rolled back, changes discarded |
| `Prepared` | — | Query built but not yet executed |
| `Executed` | — | Query executed, results available |

---

## Descriptor Types

`elicit_db::types` provides the data-carrying companions to proof tokens:

| Type | Purpose |
|------|---------|
| `DbColumn` | Column definition (name, type, nullability, primary key, default) |
| `DbTableInfo` | Table introspection result (schema, name, columns, indexes, constraints) |
| `DbSchema` | Schema metadata (name, owner, comment) |
| `DbIndexInfo` | Index metadata (name, table, columns, unique, method) |
| `DbRoleInfo` | Role metadata (name, superuser, login, connection_limit) |
| `DbSessionInfo` | Active session (pid, user, database, state, query) |
| `DbStatActivity` | Extended session stats from `pg_stat_activity` |
| `DbExplain` | EXPLAIN/EXPLAIN ANALYZE output |
| `TransactionHandle` | Opaque handle returned by `begin`; consumed by `commit`/`rollback` |
| `IsolationLevel` | Runtime enum (`ReadCommitted`, `RepeatableRead`, `Serializable`, `ReadUncommitted`) |
| `DbRoutineDescriptor` | Function/procedure definition (schema, name, language, body, return_type) |
| `DbReplicationSlotDescriptor` | Replication slot (name, kind, plugin) |
| `DbPublicationDescriptor` | Logical publication (name, tables, all_tables) |
| `DbSubscriptionDescriptor` | Logical subscription (name, connection, publications) |
| `DbRow` / `DbRows` | Query result rows with typed `DbValue` cells |
| `DbValue` | Column value: Null, Bool, Int, BigInt, Float, Text, Bytes, Uuid, Json, Array, Spatial |

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

async fn ensure_users_table(mgr: &dyn DbTableManager) -> DbResult<()> {
    let cols = vec![
        DbColumn { name: "id".into(), ty: "bigint".into(),
                   nullable: false, default_value: None, primary_key: true },
        DbColumn { name: "email".into(), ty: "text".into(),
                   nullable: false, default_value: None, primary_key: false },
    ];
    let (_, _table, _audit): (_, Established<TableCreated>, Established<AuditLogged>) =
        mgr.create_table("public", "users", cols).await?;
    Ok(())
}
```

### Typed isolation transactions

```rust
use elicit_db::{DbIsolationFactory, DbTransactor, SerializableIsolation, TxMarker, Open};

async fn transfer_funds(db: &dyn DbIsolationFactory) -> DbResult<()> {
    let (handle, _marker, iso_proof) = db.begin_serializable().await?;
    // iso_proof: Established<SerializableIsolation>
    // Compose into ACID / phenomena-prevention bundles as needed
    Ok(())
}
```

### Security posture assertion

```rust
use elicit_db::{DbSecurityFactory, SslModeRequired, EncryptedInTransit};

async fn harden_connection(sec: &dyn DbSecurityFactory) -> DbResult<()> {
    let (ssl_proof, tls_proof) = sec.enforce_tls().await?;
    // ssl_proof:  Established<SslModeRequired>
    // tls_proof: Established<EncryptedInTransit>
    // Compose into DataProtectionEvidence for aggregate proof
    Ok(())
}
```

---

## Compile-Time Guarantee Summary

| What is guaranteed | Mechanism |
|---|---|
| DDL statement was executed before proof minted | Factory method SQL runs before `Established::assert()` is called |
| Isolation level matches proof token type | `begin_serializable()` returns `Established<SerializableIsolation>` — no cast possible |
| ACID proof requires all four components | `AcidEvidence` struct fields are non-optional `Established<_>` tokens |
| Phenomena prevention follows isolation level | `ProvableFrom` impls encode the ANSI + Berenson matrix at the type level |
| Security controls were activated | `DbSecurityFactory` methods call `SET ssl = on` before minting `SslModeRequired` |
| `assert()` bypasses are audit-visible | Any `Established::assert()` on a DB proposition stands out immediately in review |

---

## Implementing a Custom Backend

To implement `DbBackend` for a new driver:

1. Implement the 15 Role 1a factory traits. Each method must execute the
   statement, then call `Established::assert()` after success. The proof token
   is the *authority* that the SQL was executed — not a passive label.

2. Implement the 5 Role 2 reporter traits. These run queries and return plain
   data; no proof tokens are involved.

3. `DbBackend` has a blanket impl — it is satisfied automatically when all 20
   sub-traits are implemented.

> **Note:** `Established::assert()` is the correct constructor for backend
> implementations. The credential-gated `Established::prove()` path is reserved
> for evidence-bundle composition in `proof_composition`, not leaf factories.
> All calls to `assert()` in a backend impl are expected, auditable, and
> intentional: the factory *is* the authority that the operation succeeded.

---

## Standards Grounding

| Standard | Coverage |
|----------|----------|
| ISO/IEC 9075-2:2023 (SQL Foundation) | DDL, DML, transactions, queries, aggregates, CTEs, window functions |
| ISO/IEC 9075-4:2023 (PSM) | Stored routines — functions, procedures, triggers |
| ISO/IEC 9075-11:2023 (SQL/Schemata) | INFORMATION_SCHEMA catalog views |
| ANSI X3.135-1992 (SQL-92) | Isolation levels — READ COMMITTED through SERIALIZABLE |
| Berenson et al. 1995 | Extended isolation phenomena: P0 dirty write, write skew, serialization anomaly |
| PostgreSQL 16 documentation | MVCC, advisory locks, partitioning, extensions, replication, backup/WAL, FDW |
| ISO/IEC 27001:2022 | Access control, audit logging, encryption, key management, authentication policy |
| OpenTelemetry Specification | Distributed tracing, span attributes, database conventions, metrics |
| PostgreSQL wire protocol §55 | Connection handshake, message framing, SCRAM-SHA-256, extended query |
| IETF RFC 7159 | JSON wire format for response serialization |

---

## Crate Layout

```text
src/
├── lib.rs                       pub use surface
├── types.rs                     Descriptor types, DbValue, IsolationLevel, TxMarker
├── typestate.rs                 Open, Committed, RolledBack, Prepared, Executed
├── contracts/
│   ├── mod.rs
│   ├── iso_sql.rs               136 props — DDL, DML, query, ACID primitives
│   ├── isolation.rs              43 props — isolation levels, phenomena, locks, 2PC
│   ├── constraints.rs            43 props — NOT NULL, UNIQUE, PK, FK, CHECK, EXCLUSION
│   ├── access_control.rs         48 props — GRANT/REVOKE, RLS policies, ownership
│   ├── psm.rs                    52 props — functions, procedures, triggers, aggregates
│   ├── replication.rs            45 props — streaming, logical, slots, pub/sub, failover
│   ├── security.rs               49 props — TLS, audit, encryption, HBA, authentication
│   ├── recovery.rs               43 props — backup, WAL archiving, PITR, failover
│   ├── postgres.rs               69 props — MVCC, indexes, partitioning, extensions, FDW
│   ├── information_schema.rs     56 props — INFORMATION_SCHEMA catalog propositions
│   ├── transport.rs              49 props — PG wire protocol, TLS, JSON, connection pool
│   ├── observability.rs          35 props — OTel traces, spans, metrics, attributes
│   └── proof_composition.rs      17 evidence bundles + 40+ ProvableFrom impls
├── traits/
│   ├── mod.rs                   Three-role taxonomy docs + DbBackend supertrait
│   ├── session.rs               DbSessionManager
│   ├── server.rs                DbServerAdmin
│   ├── database.rs              DbDatabaseManager
│   ├── schema.rs                DbSchemaManager
│   ├── table.rs                 DbTableManager
│   ├── query.rs                 DbQueryExecutor
│   ├── transaction.rs           DbTransactor
│   ├── index.rs                 DbIndexManager
│   ├── role.rs                  DbRoleManager
│   ├── monitor.rs               DbMonitor
│   ├── backup.rs                DbBackupManager
│   ├── routine.rs               DbRoutineFactory + DbRoutineMeta
│   ├── constraint.rs            DbConstraintFactory + DbConstraintMeta
│   ├── isolation.rs             DbIsolationFactory
│   ├── security.rs              DbSecurityFactory + DbSecurityMeta
│   └── replication.rs           DbReplicationFactory + DbReplicationMeta
```

---

## Formal Verification

The proof architecture is designed for downstream formal verification.
Each proposition type implements `elicitation::contracts::Prop`, which
exposes a `kani_proof()` method for generating verification harnesses.

- **Kani** — bounded model checking on factory proof-minting paths
- **Creusot** — deductive verification that factory methods execute SQL before
  calling `Established::assert()`
- **Verus** — SMT-based proofs of evidence bundle composition totality

---

## License

Apache-2.0 OR MIT
