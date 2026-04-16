//! `elicit_db` — Database contract interface crate.
//!
//! Provides a standards-anchored vocabulary of database propositions ([`contracts`]),
//! typestate markers ([`typestate`]), and a complete family of object-safe async
//! traits ([`traits`]) for pgAdmin-style database management.
//!
//! # Design
//!
//! This is an **interface crate**, not an implementation. DB drivers (sqlx, diesel,
//! sea-orm) implement the traits; consumers depend on this crate only.
//!
//! Traits use [`Established<P>`] contract return types instead of associated types,
//! giving object safety (`dyn DbTableManager`) and a common proof language at call sites.
//!
//! # Standards
//!
//! - ISO/IEC 9075 (SQL semantics)
//! - ANSI isolation model (phenomena P0–P3)
//! - PostgreSQL documentation (execution truth)
//! - ISO/IEC 27001 (security contracts)
//! - OpenTelemetry Specification (observability)
//!
//! # Example
//!
//! ```rust,no_run
//! use elicit_db::{DbTableManager, DbColumn, TableCreated, AuditLogged};
//! use elicitation::Established;
//!
//! async fn ensure_users_table(mgr: &dyn DbTableManager) {
//!     let cols = vec![
//!         DbColumn {
//!             name: "id".into(), ty: "bigint".into(),
//!             nullable: false, default_value: None, primary_key: true,
//!         },
//!         DbColumn {
//!             name: "email".into(), ty: "text".into(),
//!             nullable: false, default_value: None, primary_key: false,
//!         },
//!     ];
//!     let (Established::<TableCreated> { .. }, Established::<AuditLogged> { .. }) =
//!         mgr.create_table("public", "users", cols).await.unwrap();
//! }
//! ```
//!
//! [`Established<P>`]: elicitation::Established

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod contracts;
mod error;
mod traits;
mod types;
mod typestate;

pub use contracts::access_control::{
    // Column-level privilege propositions
    ColumnInsertPrivilegeGranted,
    ColumnPrivilegeRevoked,
    ColumnSelectPrivilegeGranted,
    ColumnUpdatePrivilegeGranted,
    // Database-level privilege propositions
    DatabaseConnectPrivilegeGranted,
    DatabaseCreatePrivilegeGranted,
    DatabasePrivilegeRevoked,
    DatabaseTempPrivilegeGranted,
    // Object ownership
    DefaultPrivilegesApplied,
    ForeignDataWrapperUsagePrivilegeGranted,
    ForeignServerUsagePrivilegeGranted,
    // Function privilege propositions
    FunctionExecutePrivilegeGranted,
    FunctionExecutePrivilegeRevoked,
    ObjectOwnershipTransferred,
    ObjectOwnershipVerified,
    // Grant option and revoke cascade/restrict
    PrivilegeGrantedWithGrantOption,
    PrivilegeRevokedCascade,
    PrivilegeRevokedRestrict,
    // Public role access control
    PublicConnectRevokedFromDatabase,
    PublicRolePrivilegeLimited,
    // Row-level security policies
    RlsBypassRoleExcluded,
    RlsDeletePolicyApplied,
    RlsInsertPolicyApplied,
    RlsSelectPolicyApplied,
    RlsUpdatePolicyApplied,
    // Role membership and options
    RoleAdminOptionGranted,
    RoleGrantOptionInherited,
    RoleInheritanceActive,
    RoleMembershipGranted,
    RoleMembershipRevoked,
    RoleResetApplied,
    RoleSetRoleApplied,
    // Schema privilege propositions
    SchemaCreatePrivilegeGranted,
    SchemaPrivilegeRevoked,
    SchemaUsagePrivilegeGranted,
    // Sequence privilege propositions
    SequenceSelectPrivilegeGranted,
    SequenceUpdatePrivilegeGranted,
    SequenceUsagePrivilegeGranted,
    // Table privilege propositions
    TableAllPrivilegesGranted,
    TableDeletePrivilegeGranted,
    TableInsertPrivilegeGranted,
    TablePrivilegeRevoked,
    TableReferencesPrivilegeGranted,
    TableSelectPrivilegeGranted,
    TableTriggerPrivilegeGranted,
    TableTruncatePrivilegeGranted,
    TableUpdatePrivilegeGranted,
    // Type privilege propositions
    TypeUsagePrivilegeGranted,
};
pub use contracts::constraints::{
    // CHECK constraint propositions
    CheckConstraintDefined,
    CheckConstraintDropped,
    CheckConstraintEvaluatesTrue,
    CheckConstraintViolationRejected,
    // Deferrable constraint propositions
    ConstraintCheckedAtCommit,
    ConstraintDeferrable,
    ConstraintDeferredInTransaction,
    ConstraintInitiallyDeferred,
    ConstraintInitiallyImmediate,
    // EXCLUSION constraint propositions
    ExclusionConstraintDefined,
    ExclusionConstraintEnforced,
    // FOREIGN KEY constraint propositions
    ForeignKeyDefined,
    ForeignKeyDropped,
    ForeignKeyEnforced,
    ForeignKeyOnDeleteCascade,
    ForeignKeyOnDeleteNoAction,
    ForeignKeyOnDeleteRestrict,
    ForeignKeyOnDeleteSetDefault,
    ForeignKeyOnDeleteSetNull,
    ForeignKeyOnUpdateCascade,
    ForeignKeyOnUpdateNoAction,
    ForeignKeyOnUpdateRestrict,
    ForeignKeyOnUpdateSetDefault,
    ForeignKeyOnUpdateSetNull,
    // Generated columns
    GeneratedColumnComputedOnInsert,
    GeneratedColumnComputedOnUpdate,
    GeneratedColumnDefined,
    // Identity columns
    IdentityAlwaysEnforced,
    IdentityByDefaultDefined,
    IdentityColumnDefined,
    // NOT NULL constraint propositions
    NotNullConstraintDefined,
    NotNullConstraintDropped,
    NotNullConstraintEnforced,
    // Partial index constraint propositions
    PartialIndexConstraintDefined,
    PartialIndexConstraintPredicateValid,
    // PRIMARY KEY constraint propositions
    PrimaryKeyDefined,
    PrimaryKeyDropped,
    PrimaryKeyEnforced,
    PrimaryKeyMultiColumn,
    PrimaryKeySingleColumn,
    // UNIQUE constraint propositions
    UniqueConstraintDefined,
    UniqueConstraintDropped,
    UniqueConstraintEnforced,
};
pub use contracts::information_schema::{
    // Table type propositions
    BaseTableExists,
    // Character set / collation propositions
    CharacterSetExists,
    CheckConstraintClauseDeclared,
    CheckConstraintNotNullClause,
    CheckConstraintRecorded,
    CollationExists,
    ColumnCharacterMaximumLengthSet,
    ColumnDataTypeDeclared,
    ColumnDatetimePrecisionSet,
    ColumnDefaultValueDeclared,
    // Column propositions
    ColumnExists,
    ColumnGenerationExpressionDeclared,
    ColumnIsNotNullable,
    ColumnIsNullable,
    ColumnNumericPrecisionSet,
    ColumnOrdinalPositionCorrect,
    ColumnPrivilegeRecorded,
    // Constraint propositions (INFORMATION_SCHEMA view)
    ConstraintEnforced,
    DomainConstraintRecorded,
    DomainDataTypeDeclared,
    // Domain propositions
    DomainExists,
    ForeignKeyColumnPositionMapped,
    ForeignKeyConstraintRecorded,
    ForeignKeyExists,
    ForeignTableExists,
    KeyColumnUsageRecorded,
    PrimaryKeyColumnOrdinalCorrect,
    PrimaryKeyConstraintRecorded,
    ReferentialConstraintDeleteRuleDeclared,
    ReferentialConstraintMatchOptionDeclared,
    ReferentialConstraintUpdateRuleDeclared,
    RoutineDataTypeDeclared,
    // Routine propositions
    RoutineExists,
    RoutineIsFunction,
    RoutineIsProcedure,
    RoutineSqlDataAccessDeclared,
    SchemaDefaultCharacterSetDeclared,
    SchemaDefaultCollationDeclared,
    // Schema propositions
    SchemaExists,
    SchemaSqlPathDeclared,
    // Sequence propositions
    SequenceExists,
    TableCommitActionDeclared,
    TableConstraintExists,
    TableExists,
    TableIsInsertableInto,
    // Privilege propositions
    TablePrivilegeRecorded,
    TriggerEventManipulationCorrect,
    // Trigger propositions (INFORMATION_SCHEMA view)
    TriggerExistsInSchema,
    TriggerTimingCorrect,
    UniqueConstraintRecorded,
    UsagePrivilegeRecorded,
    ViewCheckOptionDeclared,
    ViewDefinitionAccessible,
    ViewExists,
    ViewIsUpdatable,
    ViewTableExists,
};
pub use contracts::iso_sql::{
    AcidCompliant,
    AffectedRowCountCorrect,
    AggregateFilterApplied,
    // DML — aggregates / analytics
    AggregateOverWindowApplied,
    // Query planning
    AnalyzePlanGenerated,
    // Data types / value correctness
    ArrayDimensionsValid,
    Atomic,
    BinaryValueValid,
    BitmapIndexScanUsed,
    BooleanValueValid,
    // Conditional Expressions
    CaseExpressionEvaluated,
    CoalesceEvaluated,
    ColumnAdded,
    ColumnDefaultDropped,
    ColumnDefaultSet,
    ColumnDropped,
    // Access control (ISO SQL-level privilege propositions)
    ColumnPrivilegeGranted,
    ColumnRenamed,
    CompositeTypeFieldsValid,
    Consistent,
    ConstraintDeferred,
    // Constraints (ISO SQL-level aggregate propositions)
    ConstraintSatisfied,
    CteReferencedInMainQuery,
    CursorClosed,
    // Cursors
    CursorDeclared,
    CursorFetched,
    CursorOpened,
    CursorScrollable,
    DatabaseCreated,
    DatabaseDropped,
    DatabasePrivilegeGranted,
    DeletePredicateApplied,
    DenseRankApplied,
    // DDL: Domain lifecycle
    DomainAltered,
    DomainConstraintSatisfied,
    DomainCreated,
    DomainDropped,
    Durable,
    EmptyResult,
    ExceptResultCorrect,
    ExplainPlanGenerated,
    FetchFirstRowsApplied,
    GrantOptionInherited,
    GroupByApplied,
    GroupByCubeApplied,
    GroupByRollupApplied,
    GroupingSetsApplied,
    HashJoinUsed,
    HavingFilterApplied,
    // DDL — Index
    IndexCreated,
    IndexDropped,
    IndexScanUsed,
    IndexValid,
    InsertedValuesMatchColumns,
    IntersectResultCorrect,
    IntervalValueValid,
    IsNotNullPredicateEvaluated,
    // NULL Semantics
    IsNullPredicateEvaluated,
    JsonValueValid,
    // DML — JOINs / subqueries
    LateralJoinValid,
    LeadLagFunctionApplied,
    LimitApplied,
    MergeJoinUsed,
    MergeMatchedApplied,
    MergeNotMatchedApplied,
    NestedLoopJoinUsed,
    NonEmptyResult,
    NonRecursiveCteResultMaterialized,
    NthValueFunctionApplied,
    NullIfEvaluated,
    NullPropagatedInExpression,
    // SELECT: Advanced ordering and grouping
    NullsFirstApplied,
    NullsLastApplied,
    NumericPrecisionMaintained,
    NumericScaleMaintained,
    OffsetApplied,
    // DML — ordering / pagination
    OrderByApplied,
    OrderByInWindowApplied,
    PartitionByApplied,
    PrivilegeRevoked,
    RankFunctionApplied,
    RecursiveCteTerminates,
    RecursiveCteUnionAllUsed,
    ReferentialIntegrityMaintained,
    RoleGranted,
    RoleRevoked,
    RowDeleted,
    // DML — INSERT / UPDATE / DELETE / MERGE / TRUNCATE
    RowInserted,
    RowMerged,
    RowNumberFunctionApplied,
    RowUpdated,
    SavepointCreated,
    SavepointReleased,
    SavepointRolledBackTo,
    // DDL — Schema / Database
    SchemaCreated,
    SchemaDropped,
    SchemaIntegrityEstablished,
    SchemaPrivilegeGranted,
    // DML — SELECT
    SelectColumnListValid,
    SelectDistinctApplied,
    SelectPredicateApplied,
    SequenceAdvanced,
    // DDL: Sequence lifecycle
    SequenceAltered,
    // DDL — Sequence / Domain / Type
    SequenceCreated,
    SequenceCycled,
    SequenceDropped,
    SequentialScanUsed,
    // Set operations
    SetOperationTypeCompatible,
    StringCollationApplied,
    StringLengthWithinBounds,
    SubqueryCorrelated,
    SubqueryUncorrelated,
    TableAltered,
    // DDL — Table
    TableCreated,
    TableDropped,
    TablePrivilegeGranted,
    // DDL: ALTER TABLE decomposition
    TableRenamed,
    TemporalValueValid,
    TimezoneNormalized,
    TransactionCommitted,
    TransactionRolledBack,
    // Transactions
    TransactionStarted,
    TruncateRowsRemoved,
    TypeCreated,
    // DDL: Type lifecycle
    TypeDropped,
    UnionAllResultCorrect,
    UnionResultCorrect,
    UpdatedColumnSubsetValid,
    UuidValueValid,
    // DDL — View
    ViewCreated,
    ViewDropped,
    ViewQueryValid,
    WindowFrameBoundsValid,
    // Window Functions
    WindowFunctionDefined,
    WindowFunctionResultCorrect,
    // CTEs
    WithClauseDefined,
};
pub use contracts::isolation::{
    AccessExclusiveLockAcquired, AdvisoryLockReleased, AdvisoryLockSessionHeld,
    AdvisoryLockTransactionHeld, DeadlockDetected, DeadlockResolved, DeadlockTimeoutRespected,
    DeferrableConstraintChecked, ExclusiveLockAcquired, ForShareAcquired, ForUpdateAcquired,
    IsolationLevelUpgraded, NoDirtyReads, NoLostUpdates, NoPhantomReads, NoWaitRespected,
    NoWriteSkew, PreventsCircularInformationFlow, PreventsDirtyRead, PreventsDirtyWrite,
    PreventsLostUpdate, PreventsNonRepeatableRead, PreventsPhantomRead, PreventsReadSkew,
    PreventsSerializationAnomaly, PreventsWriteSkew, ReadCommittedIsolation,
    ReadUncommittedIsolation, RepeatableReadIsolation, RowExclusiveLockAcquired,
    RowShareLockAcquired, SavepointEstablished, SerializableIsolation, SessionIsolationLevelSet,
    ShareLockAcquired, ShareRowExclusiveLockAcquired, ShareUpdateExclusiveLockAcquired,
    SkipLockedApplied, SsiDangerousStructureAvoided, SsiPredicateLockHeld,
    SsiRwAntiDependencyTracked, StatementTimeoutRespected, TransactionIsolationLevelSet,
    TransactionReadOnly, TransactionReadWrite, TwoPhaseCommitFinalized, TwoPhaseCommitPrepared,
};
pub use contracts::observability::{
    // Metric propositions
    ActiveBackendsMetricRecorded,
    // Trace / span propositions
    BaggagePropagated,
    CacheHitRateMetricRecorded,
    ChildSpanLinkedToParent,
    ConnectionPoolMetricRecorded,
    CorrelationIdAttached,
    // Semantic attribute propositions (OpenTelemetry DB semantic conventions)
    DbConnectionStringAttributeSet,
    DbNameAttributeSet,
    DbOperationAttributeSet,
    DbRowsAffectedAttributeSet,
    DbStatementAttributeSet,
    DbSystemAttributeSet,
    DeadTuplesMetricRecorded,
    DeadlockEventLogged,
    EnvironmentAttributeSet,
    // Structured log propositions
    ErrorLogged,
    ErrorRateMetricRecorded,
    LockWaitTimeMetricRecorded,
    LogLevelConsistentWithSeverity,
    MetricsRecorded,
    QueryDurationHistogramPopulated,
    ReplicationLagMetricRecorded,
    SamplerConfigurationRespected,
    ServiceNameAttributeSet,
    SlowQueryLogged,
    SpanAttributesComplete,
    SpanContextPropagated,
    SpanErrorRecorded,
    SpanLinkedToOperation,
    SpanStatusCodeSet,
    StatementLogged,
    StructuredLogFormatUsed,
    TempFilesMetricRecorded,
    TraceEmitted,
    TraceIdAttachedToQuery,
};
pub use contracts::postgres::{
    // Advisory locks
    AdvisoryLockHeld,
    // Autovacuum / VACUUM
    AutovacuumLaunched,
    BTreeIndexCreated,
    // Index types
    BrinIndexCreated,
    // Checkpoint
    CheckpointCompleted,
    // Type system
    CompositeTypeCreated,
    // Connection pooling
    ConnectionPoolerActive,
    ConnectionPoolerModeSatisfied,
    DeadTupleFractionLow,
    // Partitioning
    DefaultPartitionExists,
    EnumTypeCreated,
    // Extensions
    ExtensionInstalled,
    ExtensionVersionMet,
    // Foreign data wrappers
    FdwOptionsValid,
    ForeignDataWrapperInstalled,
    ForeignServerDefined,
    ForeignTableAccessible,
    GinIndexCreated,
    GistIndexCreated,
    HashIndexCreated,
    HashPartitionAttached,
    HorizonAdvanced,
    // MVCC / visibility
    HotUpdateChainValid,
    IndexBloatAcceptable,
    IndexConcurrentlyBuilt,
    IndexExists,
    ListPartitionAttached,
    MVCCSnapshotValid,
    // Materialized views
    MaterializedViewCreated,
    MaterializedViewRefreshed,
    MultiRangeTypeCreated,
    MultiXactWraparoundPrevented,
    ParallelQueryPlanGenerated,
    PartialIndexDefined,
    PartitionBoundNonOverlapping,
    PartitionDetachedSafely,
    PartitionPruningApplied,
    PgTrgmExtensionActive,
    PgcryptoExtensionActive,
    PostgisExtensionActive,
    // Privilege (PG-specific)
    PrivilegeGranted,
    RangePartitionAttached,
    // PostgreSQL-specific types
    RangeTypeCreated,
    // Row-level security (PG-specific policies)
    RlsPolicyCreated,
    RlsPolicyEnablesDelete,
    RlsPolicyEnablesInsert,
    RlsPolicyEnablesRead,
    RlsPolicyEnablesUpdate,
    RowVisible,
    // Sequence bounds
    SequenceCycledSafely,
    SequenceOwnedByColumn,
    SequenceValueWithinBounds,
    SerialColumnSequenceBound,
    SharedBufferHitRateHigh,
    SnapshotIsolation,
    SpGistIndexCreated,
    StoredGeneratedColumnDefined,
    TableAnalyzed,
    TableBloatAcceptable,
    TablePartitioned,
    // Storage
    TablespaceCreated,
    ToastRelationExists,
    TransactionIdWraparoundPrevented,
    // Triggers
    TriggerCreated,
    UserMappingDefined,
    UuidOsspExtensionActive,
    VacuumFreezeApplied,
    VacuumedRecently,
    // WAL
    WalLevelSufficient,
};
pub use contracts::psm::{
    // Aggregate functions
    AggregateFinalFunctionCorrect,
    AggregateFunctionCreated,
    AggregateHypotheticalSetDefined,
    AggregateSortedSetDefined,
    AggregateStateFunctionCorrect,
    // Anonymous blocks
    AnonymousBlockExecuted,
    // Function creation and properties
    CFunctionCreated,
    FunctionAltered,
    FunctionCostEstimated,
    FunctionCreated,
    // Function/Procedure lifecycle
    FunctionDropped,
    FunctionIsImmutable,
    FunctionIsStable,
    FunctionIsStrict,
    FunctionIsVolatile,
    FunctionLanguageDeclared,
    FunctionParallelRestricted,
    // Parallel safety
    FunctionParallelSafe,
    FunctionParallelUnsafe,
    FunctionReturnTypeCorrect,
    FunctionRowsEstimated,
    FunctionSearchPathSet,
    FunctionSecurityDefiner,
    FunctionSecurityInvoker,
    // PL/pgSQL exception handling
    PlpgsqlExceptionHandled,
    PlpgsqlFunctionCreated,
    PlpgsqlRaisedWithSqlstate,
    // Procedures
    ProcedureCalledViaCAll,
    ProcedureCreated,
    ProcedureDropped,
    ProcedureTransactionControlAllowed,
    // Set-returning functions
    SetReturningFunctionDefined,
    SqlFunctionCreated,
    // Triggers
    TriggerBoundToTable,
    TriggerConditionPassed,
    TriggerDisabled,
    TriggerDropped,
    TriggerEnabled,
    TriggerFiredAfter,
    TriggerFiredBefore,
    TriggerFiredInsteadOf,
    TriggerFiredOnDelete,
    TriggerFiredOnInsert,
    TriggerFiredOnTruncate,
    TriggerFiredOnUpdate,
    TriggerFiredPerRow,
    TriggerFiredPerStatement,
    TriggerFunctionCreated,
    // Trigger conditions
    TriggerWhenConditionDefined,
    // Window functions
    UserDefinedWindowFunctionCreated,
    WindowFunctionOverClauseValid,
    WindowFunctionResultDeterministic,
};
pub use contracts::recovery::{
    // Backup propositions
    BackupConsistent,
    BackupManifestGenerated,
    BackupManifestVerified,
    BackupRetentionPolicyMet,
    BaseBackupChecksumVerified,
    BaseBackupCompleted,
    BaseBackupEncrypted,
    BaseBackupInitiated,
    // pg_dump propositions
    CustomFormatDumpUsed,
    DumpEncryptionApplied,
    // Standby / HA propositions
    FailoverCompleted,
    ParallelRestoreCompleted,
    PgDumpAllCompleted,
    PgDumpCompleted,
    // Point-in-time recovery propositions
    PitrTestPassed,
    PointInTimeRecoverable,
    PromotedToStandalone,
    QuorumCommitAcknowledged,
    RecoveryCompletionEstimated,
    RecoveryPauseCleared,
    RecoveryTargetLsn,
    RecoveryTargetName,
    RecoveryTargetReached,
    // PITR target type propositions
    RecoveryTargetTime,
    RecoveryTargetTransaction,
    ReplicationSlotCreated,
    StandbyConnectedToPrimary,
    StandbyLagBelowThreshold,
    StandbyReceivingWal,
    SwitchoverCompleted,
    SynchronousStandbyAcknowledged,
    TimelineAdvanced,
    // WAL propositions
    WALReplayable,
    WalArchiveAccessible,
    WalArchiveCommandSucceeded,
    WalArchiveRetentionMet,
    WalArchivingEnabled,
    WalChecksumEnabled,
    WalFsyncConfirmed,
    WalLevelSufficientForArchiving,
    WalRestoreCommandTested,
    WalRetentionPolicySatisfied,
    WalSegmentIntact,
};
pub use contracts::replication::{
    // High-availability lifecycle propositions
    AutomaticFailoverTriggered,
    // Logical replication propositions
    CdcChangeEventProduced,
    FailoverSlotCreated,
    HotStandbyEnabled,
    HotStandbyQueryExecuted,
    LogicalDecodingActive,
    LogicalDecodingPluginRegistered,
    LogicalReplicationConfigured,
    LogicalReplicationSlotCreated,
    MaxReplicationSlotsConfigured,
    MaxWalSendersConfigured,
    // Replication slot propositions
    PhysicalReplicationSlotCreated,
    // Streaming replication propositions
    PrimaryWalsenderActive,
    PublicationAllTablesScope,
    PublicationColumnListDefined,
    PublicationCreated,
    PublicationIncludesTable,
    PublicationRowFilterDefined,
    // Synchronous replication propositions
    QuorumSynchronousCommitAcknowledged,
    RecoveryModeExited,
    // Replication lag propositions
    ReplicationLagWithinSla,
    ReplicationOriginCreated,
    ReplicationOriginProgressTracked,
    ReplicationSlotActive,
    ReplicationSlotDropped,
    ReplicationSlotLagAcceptable,
    StandbyApplyingWal,
    StandbyPromotedToPrimary,
    StandbySwitchoverCompleted,
    StandbyWalreceiverActive,
    StreamingReplicationConfigured,
    SubscriptionActive,
    SubscriptionConflictResolved,
    SubscriptionCreated,
    SubscriptionTableCopied,
    SynchronousReplicationConfigured,
    SynchronousReplicationModeLocal,
    SynchronousReplicationModeRemoteApply,
    SynchronousReplicationModeRemoteWrite,
    SynchronousStandbyAcknowledgedApply,
    SynchronousStandbyAcknowledgedWrite,
    TimelineIdAdvanced,
    // WAL level configuration
    WalLevelLogical,
    WalLevelMinimal,
    WalLevelReplica,
};
pub use contracts::security::{
    // §A.5.15 Access control
    AccessAuthorized,
    AccessDeniedCorrectly,
    // §A.8.15 Logging
    AuditLogRetentionMet,
    AuditLogTamperEvident,
    AuditLogged,
    // Authentication
    AuthenticationSucceeded,
    CertificateAuthenticationUsed,
    // Connection security
    ConnectionLimitEnforced,
    // Data protection
    DataClassificationTagApplied,
    // §A.8.24 Cryptography
    EncryptedAtRest,
    EncryptedInTransit,
    EncryptionAlgorithmApproved,
    IpAllowlistEnforced,
    KeyManagementPolicyApplied,
    KeyRotationPerformed,
    LeastPrivilegeEnforced,
    // Role-Based Access Control
    LoginRolePasswordSet,
    Md5AuthenticationDeprecated,
    // Additional ISO 27001:2022 security controls
    MultiFactorAuthEnforced,
    NeedToKnowEnforced,
    PasswordNotStoredInPlaintext,
    PasswordPolicyEnforced,
    PeerAuthenticationLocal,
    PgHbaRuleMatched,
    PiiNotExposedInLogs,
    PrivilegeSeparationMaintained,
    PrivilegedActionLogged,
    // Row-Level Security
    RlsForcedForTableOwner,
    RlsUsingClauseCorrect,
    RlsWithCheckClauseCorrect,
    RoleCannotLoginUnexpectedly,
    RoleConnectionLimitEnforced,
    RoleCreated,
    RoleDropped,
    RoleValidUntilEnforced,
    RowLevelSecurityBypassDenied,
    RowLevelSecurityEnabled,
    RowLevelSecurityPolicyApplied,
    RowLevelSecurityPolicyDefined,
    ScramSha256AuthenticationUsed,
    SecurityEventLogged,
    SensitiveColumnMasked,
    SessionTimeoutEnforced,
    SqlInjectionPrevented,
    SslCertificateVerified,
    SslModeRequired,
    SuperuserPrivilegeRestricted,
    TlsSessionActive,
    TrustAuthenticationLimited,
};
pub use contracts::transport::{
    AuthenticationOkReceived,
    // Connection setup
    AuthenticationRequestReceived,
    BackendKeyDataReceived,
    BindCompleteReceived,
    BindMessageSent,
    CloseMessageSent,
    // Message framing
    CommandCompleteReceived,
    // Connection lifecycle
    ConnectionClosedGracefully,
    ConnectionEstablished,
    ConnectionPoolLimitRespected,
    ConnectionPoolSlotAvailable,
    ConnectionTimeoutRespected,
    CopyDataSent,
    CopyDoneReceived,
    // COPY protocol
    CopyInResponseReceived,
    CopyOutResponseReceived,
    DescribeMessageSent,
    ErrorResponseReceived,
    ExecuteMessageSent,
    // JSON serialization
    JsonNestingDepthSafe,
    JsonSchemaValid,
    JsonUtf8Encoded,
    MessageLengthPrefixCorrect,
    NoticeResponseReceived,
    ParameterStatusReceived,
    ParseCompleteReceived,
    // Extended query protocol
    ParseMessageSent,
    ProtocolVersionNegotiated,
    ReadyForQueryReceived,
    RequestWellFormed,
    ResponseFullyReceived,
    ResponseSerializable,
    ScramClientFinalSent,
    // SCRAM authentication
    ScramClientFirstSent,
    ScramServerFinalReceived,
    ScramServerFirstReceived,
    SslNegotiationSucceeded,
    SslRequestSent,
    StartupMessageSent,
    SyncMessageSent,
    // Graceful termination
    TerminateMessageSent,
    // TLS
    Tls12Supported,
    Tls13Preferred,
    TlsCertificateChainValid,
    TlsCipherSuiteApproved,
    TlsClientCertificatePresented,
    TlsHostnameVerified,
    TlsRenegotiationDisabled,
    TlsSessionResumptionSupported,
};

pub use contracts::proof_composition::{
    AcidEvidence, AuthenticatedConnectionEvidence, DataProtectionEvidence,
    ForeignKeyEnforcementEvidence, LogicalReplicationEvidence, PrimaryKeyEnforcementEvidence,
    ReadCommittedEvidence, ReadUncommittedEvidence, RecoveryCapabilityEvidence,
    RepeatableReadEvidence, SchemaIntegrityEvidence, SecureAccessEvidence, SerializableEvidence,
    SnapshotIsolationEvidence, SsiCompletenessEvidence, StreamingReplicationEvidence,
    TwoPhaseCommitFinalityEvidence,
};

pub use elicitation::ElicitComplete;
pub use error::{DbError, DbErrorKind, DbResult};
pub use traits::{
    DbBackend, DbBackupManager, DbConstraintFactory, DbConstraintMeta, DbDatabaseManager,
    DbIndexManager, DbIsolationFactory, DbMonitor, DbQueryExecutor, DbReplicationFactory,
    DbReplicationMeta, DbRoleManager, DbRoutineFactory, DbRoutineMeta, DbSchemaManager,
    DbSecurityFactory, DbSecurityMeta, DbServerAdmin, DbSessionManager, DbTableManager,
    DbTransactor,
};
pub use types::{
    ConnectionId, DbColumn, DbCommitResult, DbExecuteResult, DbExplain, DbIndexInfo,
    DbPublicationDescriptor, DbQueryRowsResult, DbReplicationSlotDescriptor, DbRoleInfo,
    DbRoutineDescriptor, DbRow, DbRows, DbSchema, DbSessionInfo, DbSpatialValue, DbStatActivity,
    DbSubscriptionDescriptor, DbTableInfo, DbTransactionalExecuteResult, DbValue, IsolationLevel,
    ParallelSafety, ReplicationSlotKind, RoutineKind, SecurityMode, TransactionHandle,
    VolatilityKind,
};
pub use typestate::{Committed, Executed, Open, Prepared, RolledBack, TxMarker};
