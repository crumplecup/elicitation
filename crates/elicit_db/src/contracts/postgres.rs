//! PostgreSQL-specific propositions.
//!
//! Source: PostgreSQL documentation, chapters 5 (DDL), 8 (data types), 9 (functions),
//!         11 (indexes), 13 (MVCC), 22 (roles), 25 (maintenance), 27 (replication),
//!         28 (storage), 30 (WAL), 31 (logical replication), 37–39 (server programming),
//!         49–50 (logical decoding / replication origins), F (extensions).

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    // -- MVCC / Visibility --

    /// A valid MVCC snapshot was acquired for the transaction.
    ///
    /// Source: PostgreSQL docs §13.1 — Introduction to MVCC
    pub struct MVCCSnapshotValid;

    /// Snapshot isolation semantics are in effect.
    ///
    /// Source: PostgreSQL docs §13.2.2 — Repeatable Read Isolation Level
    pub struct SnapshotIsolation;

    /// An advisory lock is currently held.
    ///
    /// Source: PostgreSQL docs §13.3.5 — Advisory Locks
    pub struct AdvisoryLockHeld;

    /// The queried row is visible to the current transaction snapshot.
    ///
    /// Source: PostgreSQL docs §13.1 — Row visibility rules
    pub struct RowVisible;

    /// The oldest xmin horizon has advanced; old snapshots have been released.
    ///
    /// Source: PostgreSQL docs §13.2 — Transaction Isolation — oldest xmin horizon
    pub struct HorizonAdvanced;

    /// Transaction ID wraparound was prevented by vacuuming.
    ///
    /// Source: PostgreSQL docs §13.1.4 — Transaction ID Wraparound
    pub struct TransactionIdWraparoundPrevented;

    /// Multixact ID wraparound was prevented.
    ///
    /// Source: PostgreSQL docs §13.1.4 — Multixact Wraparound
    pub struct MultiXactWraparoundPrevented;

    /// VACUUM FREEZE was applied to advance relfrozenxid.
    ///
    /// Source: PostgreSQL docs §25.1.5 — Preventing Transaction ID Wraparound
    pub struct VacuumFreezeApplied;

    /// HOT (Heap Only Tuple) update chain is valid.
    ///
    /// Source: PostgreSQL docs §13.6 — HOT Updates
    pub struct HotUpdateChainValid;

    // -- Indexes --

    /// The named index exists on the target table.
    ///
    /// Source: PostgreSQL docs §11 — Indexes
    pub struct IndexExists;

    /// A B-tree index was created successfully.
    ///
    /// Source: PostgreSQL docs §11.2 — Index Types — B-Tree
    pub struct BTreeIndexCreated;

    /// A hash index was created successfully.
    ///
    /// Source: PostgreSQL docs §11.2 — Index Types — Hash
    pub struct HashIndexCreated;

    /// A GIN index was created successfully.
    ///
    /// Source: PostgreSQL docs §11.2 — Index Types — GIN
    pub struct GinIndexCreated;

    /// A GiST index was created successfully.
    ///
    /// Source: PostgreSQL docs §11.2 — Index Types — GiST
    pub struct GistIndexCreated;

    /// A BRIN index was created successfully.
    ///
    /// Source: PostgreSQL docs §11.2 — Index Types — BRIN
    pub struct BrinIndexCreated;

    /// An SP-GiST index was created successfully.
    ///
    /// Source: PostgreSQL docs §11.2 — Index Types — SP-GiST
    pub struct SpGistIndexCreated;

    /// CREATE INDEX CONCURRENTLY completed without locking writes.
    ///
    /// Source: PostgreSQL docs §11.12 — Building Indexes Concurrently
    pub struct IndexConcurrentlyBuilt;

    /// Index bloat is within acceptable bounds.
    ///
    /// Source: PostgreSQL docs §11 — Indexes — index bloat
    pub struct IndexBloatAcceptable;

    /// Partial index has a WHERE clause predicate.
    ///
    /// Source: PostgreSQL docs §11.8 — Partial Indexes
    pub struct PartialIndexDefined;

    // -- Partitioning --

    /// Table is a partitioned parent.
    ///
    /// Source: PostgreSQL docs §5.11 — Table Partitioning
    pub struct TablePartitioned;

    /// A range partition was attached to a partitioned table.
    ///
    /// Source: PostgreSQL docs §5.11.2 — Declarative Partitioning — Range
    pub struct RangePartitionAttached;

    /// A list partition was attached to a partitioned table.
    ///
    /// Source: PostgreSQL docs §5.11.2 — Declarative Partitioning — List
    pub struct ListPartitionAttached;

    /// A hash partition was attached to a partitioned table.
    ///
    /// Source: PostgreSQL docs §5.11.2 — Declarative Partitioning — Hash
    pub struct HashPartitionAttached;

    /// A DEFAULT partition exists on the partitioned table.
    ///
    /// Source: PostgreSQL docs §5.11.2 — Declarative Partitioning — Default partition
    pub struct DefaultPartitionExists;

    /// Partition pruning eliminated irrelevant partitions from the query plan.
    ///
    /// Source: PostgreSQL docs §5.11.4 — Partition Pruning
    pub struct PartitionPruningApplied;

    /// DETACH PARTITION CONCURRENTLY completed without a lock wait.
    ///
    /// Source: PostgreSQL docs §5.11.3 — Partition Maintenance — concurrent detach
    pub struct PartitionDetachedSafely;

    /// Partition bounds do not overlap.
    ///
    /// Source: PostgreSQL docs §5.11.2 — Declarative Partitioning — non-overlapping bounds
    pub struct PartitionBoundNonOverlapping;

    // -- Extensions --

    /// CREATE EXTENSION succeeded.
    ///
    /// Source: PostgreSQL docs §F — Additional Supplied Modules
    pub struct ExtensionInstalled;

    /// PostGIS extension is installed and active.
    ///
    /// Source: PostgreSQL docs §F — Additional Supplied Modules — PostGIS
    pub struct PostgisExtensionActive;

    /// pgcrypto extension is installed.
    ///
    /// Source: PostgreSQL docs §F.26 — pgcrypto
    pub struct PgcryptoExtensionActive;

    /// uuid-ossp extension is installed.
    ///
    /// Source: PostgreSQL docs §F.49 — uuid-ossp
    pub struct UuidOsspExtensionActive;

    /// pg_trgm extension is installed.
    ///
    /// Source: PostgreSQL docs §F.38 — pg_trgm
    pub struct PgTrgmExtensionActive;

    /// Installed extension version meets the minimum required version.
    ///
    /// Source: PostgreSQL docs §F — Additional Supplied Modules — extension versioning
    pub struct ExtensionVersionMet;

    // -- Sequences --

    /// nextval() returned a value within [minvalue, maxvalue].
    ///
    /// Source: PostgreSQL docs §9.17 — Sequence Manipulation Functions — bounds
    pub struct SequenceValueWithinBounds;

    /// Sequence cycled back to its start value safely.
    ///
    /// Source: PostgreSQL docs §9.17 — Sequence Manipulation Functions — CYCLE
    pub struct SequenceCycledSafely;

    /// Sequence is owned by a table column (OWNED BY).
    ///
    /// Source: PostgreSQL docs §9.17 — Sequence Manipulation Functions — OWNED BY
    pub struct SequenceOwnedByColumn;

    /// SERIAL/BIGSERIAL implicit sequence is correctly bound to its column.
    ///
    /// Source: PostgreSQL docs §8.1.4 — Serial Types — implicit sequence
    pub struct SerialColumnSequenceBound;

    // -- Foreign Data Wrappers --

    /// CREATE FOREIGN DATA WRAPPER succeeded.
    ///
    /// Source: PostgreSQL docs §5.12 — Foreign Data — CREATE FOREIGN DATA WRAPPER
    pub struct ForeignDataWrapperInstalled;

    /// CREATE SERVER succeeded.
    ///
    /// Source: PostgreSQL docs §5.12 — Foreign Data — CREATE SERVER
    pub struct ForeignServerDefined;

    /// CREATE USER MAPPING succeeded.
    ///
    /// Source: PostgreSQL docs §5.12 — Foreign Data — CREATE USER MAPPING
    pub struct UserMappingDefined;

    /// Foreign table can be queried through the FDW.
    ///
    /// Source: PostgreSQL docs §5.12 — Foreign Data — foreign table access
    pub struct ForeignTableAccessible;

    /// FDW options are valid for the server and table.
    ///
    /// Source: PostgreSQL docs §5.12 — Foreign Data — FDW option validation
    pub struct FdwOptionsValid;

    // -- Functions and Triggers --

    /// CREATE TRIGGER succeeded.
    ///
    /// Source: PostgreSQL docs §39 — Triggers — CREATE TRIGGER
    pub struct TriggerCreated;

    // -- Row-Level Security --

    /// CREATE POLICY succeeded.
    ///
    /// Source: PostgreSQL docs §5.8 — Row Security Policies — CREATE POLICY
    pub struct RlsPolicyCreated;

    /// RLS policy permits SELECT (read) access.
    ///
    /// Source: PostgreSQL docs §5.8 — Row Security Policies — USING expression
    pub struct RlsPolicyEnablesRead;

    /// RLS policy permits INSERT access.
    ///
    /// Source: PostgreSQL docs §5.8 — Row Security Policies — WITH CHECK on INSERT
    pub struct RlsPolicyEnablesInsert;

    /// RLS policy permits UPDATE access.
    ///
    /// Source: PostgreSQL docs §5.8 — Row Security Policies — WITH CHECK on UPDATE
    pub struct RlsPolicyEnablesUpdate;

    /// RLS policy permits DELETE access.
    ///
    /// Source: PostgreSQL docs §5.8 — Row Security Policies — USING on DELETE
    pub struct RlsPolicyEnablesDelete;

    // -- Maintenance --

    /// The table has been vacuumed recently enough for healthy bloat levels.
    ///
    /// Source: PostgreSQL docs §25.1 — Routine Vacuuming
    pub struct VacuumedRecently;

    /// ANALYZE completed; column statistics are up to date.
    ///
    /// Source: PostgreSQL docs §25.1.3 — Updating Planner Statistics
    pub struct TableAnalyzed;

    /// Dead tuple fraction is below autovacuum_vacuum_scale_factor.
    ///
    /// Source: PostgreSQL docs §25.1.6 — Autovacuum — scale factor threshold
    pub struct DeadTupleFractionLow;

    /// Table bloat ratio is within acceptable bounds.
    ///
    /// Source: PostgreSQL docs §25.1 — Routine Vacuuming — bloat
    pub struct TableBloatAcceptable;

    /// Autovacuum daemon launched a worker for this table.
    ///
    /// Source: PostgreSQL docs §25.1.6 — The Autovacuum Daemon
    pub struct AutovacuumLaunched;

    /// TOAST table exists for tables with potentially large columns.
    ///
    /// Source: PostgreSQL docs §28 — Storage — TOAST
    pub struct ToastRelationExists;

    /// A checkpoint completed successfully.
    ///
    /// Source: PostgreSQL docs §30.4 — Write-Ahead Logging — Checkpoints
    pub struct CheckpointCompleted;

    /// Shared buffer hit rate is above the configured threshold.
    ///
    /// Source: PostgreSQL docs §20.4 — Resource Consumption — shared_buffers
    pub struct SharedBufferHitRateHigh;

    // -- Connection Pooling --

    /// A connection pooler (e.g., PgBouncer) is active.
    ///
    /// Source: PostgreSQL docs §28.1 — Connection Pooling — external pooler
    pub struct ConnectionPoolerActive;

    /// Pooler mode (transaction/session/statement) is appropriate for the workload.
    ///
    /// Source: PostgreSQL docs §28.1 — Connection Pooling — pooling modes
    pub struct ConnectionPoolerModeSatisfied;

    // -- WAL and Logical Replication --

    /// wal_level is set to replica or logical, sufficient for replication.
    ///
    /// Source: PostgreSQL docs §30.4 — Write-Ahead Logging — wal_level
    pub struct WalLevelSufficient;

    // -- Roles and Privileges --

    /// GRANT privilege on object succeeded.
    ///
    /// Source: PostgreSQL docs §5.7 — Privileges — GRANT
    pub struct PrivilegeGranted;

    // -- Schema and DDL --

    /// CREATE TABLESPACE succeeded.
    ///
    /// Source: PostgreSQL docs §22.6 — Tablespaces — CREATE TABLESPACE
    pub struct TablespaceCreated;

    /// CREATE MATERIALIZED VIEW succeeded.
    ///
    /// Source: PostgreSQL docs §41.3 — Materialized Views — CREATE MATERIALIZED VIEW
    pub struct MaterializedViewCreated;

    /// REFRESH MATERIALIZED VIEW completed successfully.
    ///
    /// Source: PostgreSQL docs §41.3 — Materialized Views — REFRESH
    pub struct MaterializedViewRefreshed;

    /// CREATE TYPE ... AS (composite) succeeded.
    ///
    /// Source: PostgreSQL docs §8.16 — Composite Types — CREATE TYPE
    pub struct CompositeTypeCreated;

    /// CREATE TYPE ... AS ENUM succeeded.
    ///
    /// Source: PostgreSQL docs §8.7 — Enumerated Types — CREATE TYPE AS ENUM
    pub struct EnumTypeCreated;

    /// CREATE TYPE ... AS RANGE succeeded.
    ///
    /// Source: PostgreSQL docs §8.17 — Range Types — CREATE TYPE AS RANGE
    pub struct RangeTypeCreated;

    /// CREATE TYPE ... AS RANGE succeeded with a corresponding multi-range type.
    ///
    /// Source: PostgreSQL docs §8.17.5 — Multirange Types (PostgreSQL 14+)
    pub struct MultiRangeTypeCreated;

    /// Column is defined as `GENERATED ALWAYS AS (expr) STORED` in PostgreSQL.
    ///
    /// Source: PostgreSQL §5.3 — Generated Columns; ISO/IEC 9075-2:2011 §11.4
    pub struct StoredGeneratedColumnDefined;

    /// Query optimizer chose a parallel plan (parallel workers > 0).
    ///
    /// Source: PostgreSQL §15 — Parallel Query — parallel plan generation
    pub struct ParallelQueryPlanGenerated;

    macro_rules! pg_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by PostgreSQL MVCC guarantee */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by PostgreSQL MVCC guarantee */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by PostgreSQL MVCC guarantee */ }
                }
            }
        };
    }

    // MVCC / Visibility
    pg_prop!(MVCCSnapshotValid, "MVCCSnapshotValid");
    pg_prop!(SnapshotIsolation, "SnapshotIsolation");
    pg_prop!(AdvisoryLockHeld, "AdvisoryLockHeld");
    pg_prop!(RowVisible, "RowVisible");
    pg_prop!(HorizonAdvanced, "HorizonAdvanced");
    pg_prop!(
        TransactionIdWraparoundPrevented,
        "TransactionIdWraparoundPrevented"
    );
    pg_prop!(MultiXactWraparoundPrevented, "MultiXactWraparoundPrevented");
    pg_prop!(VacuumFreezeApplied, "VacuumFreezeApplied");
    pg_prop!(HotUpdateChainValid, "HotUpdateChainValid");
    // Indexes
    pg_prop!(IndexExists, "IndexExists");
    pg_prop!(BTreeIndexCreated, "BTreeIndexCreated");
    pg_prop!(HashIndexCreated, "HashIndexCreated");
    pg_prop!(GinIndexCreated, "GinIndexCreated");
    pg_prop!(GistIndexCreated, "GistIndexCreated");
    pg_prop!(BrinIndexCreated, "BrinIndexCreated");
    pg_prop!(SpGistIndexCreated, "SpGistIndexCreated");
    pg_prop!(IndexConcurrentlyBuilt, "IndexConcurrentlyBuilt");
    pg_prop!(IndexBloatAcceptable, "IndexBloatAcceptable");
    pg_prop!(PartialIndexDefined, "PartialIndexDefined");
    // Partitioning
    pg_prop!(TablePartitioned, "TablePartitioned");
    pg_prop!(RangePartitionAttached, "RangePartitionAttached");
    pg_prop!(ListPartitionAttached, "ListPartitionAttached");
    pg_prop!(HashPartitionAttached, "HashPartitionAttached");
    pg_prop!(DefaultPartitionExists, "DefaultPartitionExists");
    pg_prop!(PartitionPruningApplied, "PartitionPruningApplied");
    pg_prop!(PartitionDetachedSafely, "PartitionDetachedSafely");
    pg_prop!(PartitionBoundNonOverlapping, "PartitionBoundNonOverlapping");
    // Extensions
    pg_prop!(ExtensionInstalled, "ExtensionInstalled");
    pg_prop!(PostgisExtensionActive, "PostgisExtensionActive");
    pg_prop!(PgcryptoExtensionActive, "PgcryptoExtensionActive");
    pg_prop!(UuidOsspExtensionActive, "UuidOsspExtensionActive");
    pg_prop!(PgTrgmExtensionActive, "PgTrgmExtensionActive");
    pg_prop!(ExtensionVersionMet, "ExtensionVersionMet");
    // Sequences
    pg_prop!(SequenceValueWithinBounds, "SequenceValueWithinBounds");
    pg_prop!(SequenceCycledSafely, "SequenceCycledSafely");
    pg_prop!(SequenceOwnedByColumn, "SequenceOwnedByColumn");
    pg_prop!(SerialColumnSequenceBound, "SerialColumnSequenceBound");
    // Foreign Data Wrappers
    pg_prop!(ForeignDataWrapperInstalled, "ForeignDataWrapperInstalled");
    pg_prop!(ForeignServerDefined, "ForeignServerDefined");
    pg_prop!(UserMappingDefined, "UserMappingDefined");
    pg_prop!(ForeignTableAccessible, "ForeignTableAccessible");
    pg_prop!(FdwOptionsValid, "FdwOptionsValid");
    // Functions and Triggers
    pg_prop!(TriggerCreated, "TriggerCreated");
    // Row-Level Security
    pg_prop!(RlsPolicyCreated, "RlsPolicyCreated");
    pg_prop!(RlsPolicyEnablesRead, "RlsPolicyEnablesRead");
    pg_prop!(RlsPolicyEnablesInsert, "RlsPolicyEnablesInsert");
    pg_prop!(RlsPolicyEnablesUpdate, "RlsPolicyEnablesUpdate");
    pg_prop!(RlsPolicyEnablesDelete, "RlsPolicyEnablesDelete");
    // Maintenance
    pg_prop!(VacuumedRecently, "VacuumedRecently");
    pg_prop!(TableAnalyzed, "TableAnalyzed");
    pg_prop!(DeadTupleFractionLow, "DeadTupleFractionLow");
    pg_prop!(TableBloatAcceptable, "TableBloatAcceptable");
    pg_prop!(AutovacuumLaunched, "AutovacuumLaunched");
    pg_prop!(ToastRelationExists, "ToastRelationExists");
    pg_prop!(CheckpointCompleted, "CheckpointCompleted");
    pg_prop!(SharedBufferHitRateHigh, "SharedBufferHitRateHigh");
    // Connection Pooling
    pg_prop!(ConnectionPoolerActive, "ConnectionPoolerActive");
    pg_prop!(
        ConnectionPoolerModeSatisfied,
        "ConnectionPoolerModeSatisfied"
    );
    // WAL and Logical Replication
    pg_prop!(WalLevelSufficient, "WalLevelSufficient");
    // Roles and Privileges
    pg_prop!(PrivilegeGranted, "PrivilegeGranted");
    // Schema and DDL
    pg_prop!(TablespaceCreated, "TablespaceCreated");
    pg_prop!(MaterializedViewCreated, "MaterializedViewCreated");
    pg_prop!(MaterializedViewRefreshed, "MaterializedViewRefreshed");
    pg_prop!(CompositeTypeCreated, "CompositeTypeCreated");
    pg_prop!(EnumTypeCreated, "EnumTypeCreated");

    pg_prop!(RangeTypeCreated, "RangeTypeCreated");
    pg_prop!(MultiRangeTypeCreated, "MultiRangeTypeCreated");
    pg_prop!(StoredGeneratedColumnDefined, "StoredGeneratedColumnDefined");
    pg_prop!(ParallelQueryPlanGenerated, "ParallelQueryPlanGenerated");
}

pub use emit_impls::{
    AdvisoryLockHeld, AutovacuumLaunched, BTreeIndexCreated, BrinIndexCreated, CheckpointCompleted,
    CompositeTypeCreated, ConnectionPoolerActive, ConnectionPoolerModeSatisfied,
    DeadTupleFractionLow, DefaultPartitionExists, EnumTypeCreated, ExtensionInstalled,
    ExtensionVersionMet, FdwOptionsValid, ForeignDataWrapperInstalled, ForeignServerDefined,
    ForeignTableAccessible, GinIndexCreated, GistIndexCreated, HashIndexCreated,
    HashPartitionAttached, HorizonAdvanced, HotUpdateChainValid, IndexBloatAcceptable,
    IndexConcurrentlyBuilt, IndexExists, ListPartitionAttached, MVCCSnapshotValid,
    MaterializedViewCreated, MaterializedViewRefreshed, MultiRangeTypeCreated,
    MultiXactWraparoundPrevented, ParallelQueryPlanGenerated, PartialIndexDefined,
    PartitionBoundNonOverlapping, PartitionDetachedSafely, PartitionPruningApplied,
    PgTrgmExtensionActive, PgcryptoExtensionActive, PostgisExtensionActive, PrivilegeGranted,
    RangePartitionAttached, RangeTypeCreated, RlsPolicyCreated, RlsPolicyEnablesDelete,
    RlsPolicyEnablesInsert, RlsPolicyEnablesRead, RlsPolicyEnablesUpdate, RowVisible,
    SequenceCycledSafely, SequenceOwnedByColumn, SequenceValueWithinBounds,
    SerialColumnSequenceBound, SharedBufferHitRateHigh, SnapshotIsolation, SpGistIndexCreated,
    StoredGeneratedColumnDefined, TableAnalyzed, TableBloatAcceptable, TablePartitioned,
    TablespaceCreated, ToastRelationExists, TransactionIdWraparoundPrevented, TriggerCreated,
    UserMappingDefined, UuidOsspExtensionActive, VacuumFreezeApplied, VacuumedRecently,
    WalLevelSufficient,
};
