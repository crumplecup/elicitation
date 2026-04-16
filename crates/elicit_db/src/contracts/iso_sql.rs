//! SQL standard propositions — ISO/IEC 9075.
//!
//! Full coverage of DDL, DML, constraints, transactions, access control,
//! set operations, window functions, CTEs, data types, cursors, and query planning.
//!
//! All §references are to ISO/IEC 9075-2:2023 unless stated otherwise.

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    // -- DDL: Table --

    /// Table was successfully created by a `CREATE TABLE` statement.
    ///
    /// Source: ISO/IEC 9075-2 §11.3 — `<table definition>`
    pub struct TableCreated;

    /// Table structure was successfully modified by an `ALTER TABLE` statement.
    ///
    /// Source: ISO/IEC 9075-2 §11.10 — `<alter table statement>`
    pub struct TableAltered;

    /// Table was successfully removed by a `DROP TABLE` statement.
    ///
    /// Source: ISO/IEC 9075-2 §11.21 — `<drop table statement>`
    pub struct TableDropped;

    /// Table was renamed via `ALTER TABLE … RENAME TO`.
    ///
    /// Source: ISO/IEC 9075-2 §11.10 — `<alter table statement>` / PostgreSQL §ALTER TABLE RENAME
    pub struct TableRenamed;

    /// Column was added to a table via `ALTER TABLE … ADD COLUMN`.
    ///
    /// Source: ISO/IEC 9075-2 §11.10 — `<add column definition>`
    pub struct ColumnAdded;

    /// Column was removed from a table via `ALTER TABLE … DROP COLUMN`.
    ///
    /// Source: ISO/IEC 9075-2 §11.10 — `<drop column definition>`
    pub struct ColumnDropped;

    /// Column was renamed via `ALTER TABLE … RENAME COLUMN`.
    ///
    /// Source: ISO/IEC 9075-2 §11.10 — `<alter column definition>` / PostgreSQL §ALTER TABLE RENAME COLUMN
    pub struct ColumnRenamed;

    /// Column default expression was set via `ALTER TABLE … ALTER COLUMN … SET DEFAULT`.
    ///
    /// Source: ISO/IEC 9075-2 §11.10 — `<alter column set default clause>`
    pub struct ColumnDefaultSet;

    /// Column default was removed via `ALTER TABLE … ALTER COLUMN … DROP DEFAULT`.
    ///
    /// Source: ISO/IEC 9075-2 §11.10 — `<alter column drop default clause>`
    pub struct ColumnDefaultDropped;

    // -- DDL: View --

    /// View was successfully created by a `CREATE VIEW` statement.
    ///
    /// Source: ISO/IEC 9075-2 §11.32 — `<view definition>`
    pub struct ViewCreated;

    /// View was successfully removed by a `DROP VIEW` statement.
    ///
    /// Source: ISO/IEC 9075-2 §11.37 — `<drop view statement>`
    pub struct ViewDropped;

    /// The query expression underlying the view is syntactically and semantically valid.
    ///
    /// Source: ISO/IEC 9075-2 §11.32 — `<view definition>` query expression validity
    pub struct ViewQueryValid;

    // -- DDL: Schema --

    /// Schema was successfully created by a `CREATE SCHEMA` statement.
    ///
    /// Source: ISO/IEC 9075-2 §11.1 — `<schema definition>`
    pub struct SchemaCreated;

    /// Schema was successfully removed by a `DROP SCHEMA` statement.
    ///
    /// Source: ISO/IEC 9075-2 §11.2 — `<drop schema statement>`
    pub struct SchemaDropped;

    // -- DDL: Database / Catalog --

    /// Catalog (database) was successfully created.
    ///
    /// Source: ISO/IEC 9075-2 §17 — `<SQL-schema statement>`
    pub struct DatabaseCreated;

    /// Catalog (database) was successfully dropped.
    ///
    /// Source: ISO/IEC 9075-2 §17 — `<SQL-schema statement>`
    pub struct DatabaseDropped;

    // -- DDL: Domain --

    /// Domain was successfully created by a `CREATE DOMAIN` statement.
    ///
    /// Source: ISO/IEC 9075-2 §11.25 — `<domain definition>`
    pub struct DomainCreated;

    /// A value assigned to a domain column satisfies the domain's constraint.
    ///
    /// Source: ISO/IEC 9075-2 §11.25 — `<domain constraint>` evaluation
    pub struct DomainConstraintSatisfied;

    /// Domain was modified via `ALTER DOMAIN`.
    ///
    /// Source: ISO/IEC 9075-2 §11.27 — `<alter domain statement>`
    pub struct DomainAltered;

    /// Domain was removed via `DROP DOMAIN`.
    ///
    /// Source: ISO/IEC 9075-2 §11.26 — `<drop domain statement>`
    pub struct DomainDropped;

    // -- DDL: Sequence --

    /// Sequence generator was successfully created by a `CREATE SEQUENCE` statement.
    ///
    /// Source: ISO/IEC 9075-2 §11.62 — `<sequence generator definition>`
    pub struct SequenceCreated;

    /// `NEXT VALUE FOR` advanced the sequence to the next value in range.
    ///
    /// Source: ISO/IEC 9075-2 §6.13 — `<next value expression>`
    pub struct SequenceAdvanced;

    /// Sequence wrapped around after reaching its maximum (or minimum) value.
    ///
    /// Source: ISO/IEC 9075-2 §11.62 — `CYCLE` option of `<sequence generator definition>`
    pub struct SequenceCycled;

    /// Sequence parameters were changed via `ALTER SEQUENCE`.
    ///
    /// Source: ISO/IEC 9075-2 §11.63 — `<alter sequence generator statement>`
    pub struct SequenceAltered;

    /// Sequence was removed via `DROP SEQUENCE`.
    ///
    /// Source: ISO/IEC 9075-2 §11.64 — `<drop sequence generator statement>`
    pub struct SequenceDropped;

    // -- DDL: Index --

    /// Index was successfully created.
    ///
    /// Source: ISO/IEC 9075-2 §11 — implementation-defined index DDL
    pub struct IndexCreated;

    /// Index was successfully dropped.
    ///
    /// Source: ISO/IEC 9075-2 §11 — implementation-defined index DDL
    pub struct IndexDropped;

    /// Index is internally consistent and reflects the current table contents.
    ///
    /// Source: ISO/IEC 9075-2 §4.15 — integrity constraints and index maintenance
    pub struct IndexValid;

    // -- DDL: User-Defined Type --

    /// User-defined type was successfully created by a `CREATE TYPE` statement.
    ///
    /// Source: ISO/IEC 9075-2 §11.50 — `<user-defined type definition>`
    pub struct TypeCreated;

    /// User-defined type was removed via `DROP TYPE`.
    ///
    /// Source: ISO/IEC 9075-2 §11.56 — `<drop user-defined type statement>`
    pub struct TypeDropped;

    // -- DML: Mutation --

    /// At least one row was inserted by an `INSERT` statement.
    ///
    /// Source: ISO/IEC 9075-2 §14.8 — `<insert statement>`
    pub struct RowInserted;

    /// At least one row was updated by an `UPDATE` statement.
    ///
    /// Source: ISO/IEC 9075-2 §14.11 — `<update statement: searched>`
    pub struct RowUpdated;

    /// At least one row was deleted by a `DELETE` statement.
    ///
    /// Source: ISO/IEC 9075-2 §14.7 — `<delete statement: searched>`
    pub struct RowDeleted;

    /// At least one row was affected by a `MERGE` statement.
    ///
    /// Source: ISO/IEC 9075-2 §14.13 — `<merge statement>`
    pub struct RowMerged;

    // -- DML: Query result cardinality --

    /// Query returned at least one row.
    ///
    /// Source: ISO/IEC 9075-2 §14.1 — `<query expression>`
    pub struct NonEmptyResult;

    /// Query returned exactly zero rows.
    ///
    /// Source: ISO/IEC 9075-2 §14.1 — `<query expression>` empty table result
    pub struct EmptyResult;

    /// The count of affected rows matches the expected value.
    ///
    /// Source: ISO/IEC 9075-2 §14 — row-count semantics of DML statements
    pub struct AffectedRowCountCorrect;

    // -- DML: INSERT correctness --

    /// The values supplied in an `INSERT` match the column list in position and type.
    ///
    /// Source: ISO/IEC 9075-2 §14.8 — `<insert columns and source>` correspondence
    pub struct InsertedValuesMatchColumns;

    // -- DML: UPDATE correctness --

    /// The subset of columns targeted by a `SET` clause is valid for the table.
    ///
    /// Source: ISO/IEC 9075-2 §14.11 — `<set clause list>` validation
    pub struct UpdatedColumnSubsetValid;

    // -- DML: DELETE correctness --

    /// The `WHERE` predicate of a `DELETE` was fully evaluated before row removal.
    ///
    /// Source: ISO/IEC 9075-2 §14.7 — `<delete statement: searched>` search condition
    pub struct DeletePredicateApplied;

    // -- DML: TRUNCATE --

    /// All rows were removed from the table by a `TRUNCATE TABLE` statement.
    ///
    /// Source: ISO/IEC 9075-2 §14.16 — `<truncate table statement>`
    pub struct TruncateRowsRemoved;

    // -- DML: MERGE --

    /// The WHEN MATCHED branch of a `MERGE` was executed for matching rows.
    ///
    /// Source: ISO/IEC 9075-2 §14.13 — `<merge when matched clause>`
    pub struct MergeMatchedApplied;

    /// The WHEN NOT MATCHED branch of a `MERGE` was executed for non-matching rows.
    ///
    /// Source: ISO/IEC 9075-2 §14.13 — `<merge when not matched clause>`
    pub struct MergeNotMatchedApplied;

    // -- DML: SELECT correctness --

    /// The `WHERE` predicate of a `SELECT` was applied to filter result rows.
    ///
    /// Source: ISO/IEC 9075-2 §7.8 — `<where clause>`
    pub struct SelectPredicateApplied;

    /// Every column in the `SELECT` list resolves to a valid column reference.
    ///
    /// Source: ISO/IEC 9075-2 §7.16 — `<select list>` column reference resolution
    pub struct SelectColumnListValid;

    /// `SELECT DISTINCT` eliminated duplicate rows from the result set.
    ///
    /// Source: ISO/IEC 9075-2 §7.17 — `<query specification>` with `DISTINCT`
    pub struct SelectDistinctApplied;

    /// `ORDER BY` clause produced results in the specified sort order.
    ///
    /// Source: ISO/IEC 9075-2 §7.17 — `<order by clause>`
    pub struct OrderByApplied;

    /// `NULLS FIRST` ordering modifier placed null values before non-null values.
    ///
    /// Source: ISO/IEC 9075-2 §10.11 — `<sort specification>` `NULLS FIRST`
    pub struct NullsFirstApplied;

    /// `NULLS LAST` ordering modifier placed null values after non-null values.
    ///
    /// Source: ISO/IEC 9075-2 §10.11 — `<sort specification>` `NULLS LAST`
    pub struct NullsLastApplied;

    /// `FETCH FIRST n ROWS` clause restricted result cardinality.
    ///
    /// Source: ISO/IEC 9075-2 §7.17 — `<fetch first clause>`
    pub struct FetchFirstRowsApplied;

    /// `FETCH FIRST n ROWS` or `LIMIT` clause restricted result cardinality.
    ///
    /// Source: ISO/IEC 9075-2 §7.17 — `<result offset clause>` and `<fetch first clause>`
    pub struct LimitApplied;

    /// `OFFSET` clause skipped the correct number of leading rows.
    ///
    /// Source: ISO/IEC 9075-2 §7.17 — `<result offset clause>`
    pub struct OffsetApplied;

    /// `GROUP BY` clause partitioned rows into the correct grouping sets.
    ///
    /// Source: ISO/IEC 9075-2 §7.14 — `<group by clause>`
    pub struct GroupByApplied;

    /// `GROUP BY ROLLUP(…)` produced the correct superaggregate rows.
    ///
    /// Source: ISO/IEC 9075-2 §7.14 — `<rollup list>`
    pub struct GroupByRollupApplied;

    /// `GROUP BY CUBE(…)` produced all cross-dimensional aggregates.
    ///
    /// Source: ISO/IEC 9075-2 §7.14 — `<cube list>`
    pub struct GroupByCubeApplied;

    /// `GROUP BY GROUPING SETS(…)` produced the declared grouping set combinations.
    ///
    /// Source: ISO/IEC 9075-2 §7.14 — `<grouping sets specification>`
    pub struct GroupingSetsApplied;

    /// `FILTER (WHERE …)` clause correctly restricted the rows fed to an aggregate function.
    ///
    /// Source: ISO/IEC 9075-2 §10.9 — `<aggregate function>` `<filter clause>`
    pub struct AggregateFilterApplied;

    /// `HAVING` clause filtered groups after aggregation was applied.
    ///
    /// Source: ISO/IEC 9075-2 §7.15 — `<having clause>`
    pub struct HavingFilterApplied;

    /// A correlated subquery references columns from its outer query correctly.
    ///
    /// Source: ISO/IEC 9075-2 §8.15 — `<subquery>` with outer reference
    pub struct SubqueryCorrelated;

    /// An uncorrelated subquery produces a result independent of the outer query.
    ///
    /// Source: ISO/IEC 9075-2 §8.15 — `<subquery>` without outer reference
    pub struct SubqueryUncorrelated;

    /// A `LATERAL` join correctly exposes the preceding row source to its table function.
    ///
    /// Source: ISO/IEC 9075-2 §7.7 — `<joined table>` with `LATERAL`
    pub struct LateralJoinValid;

    // -- Constraints: FOREIGN KEY --

    /// Every foreign key value either matches a parent row or is null (if nullable).
    ///
    /// Source: ISO/IEC 9075-2 §11.8 — `<referential constraint definition>` enforcement
    pub struct ReferentialIntegrityMaintained;

    // -- Constraints: Aggregate --

    /// All applicable table and column constraints are satisfied for the row.
    ///
    /// Source: ISO/IEC 9075-2 §11.6 — `<table constraint definition>` aggregate check
    pub struct ConstraintSatisfied;

    /// Constraint has been set to `DEFERRED` mode within the current transaction.
    ///
    /// Source: ISO/IEC 9075-2 §17.10 — `<set constraints mode statement>`
    pub struct ConstraintDeferred;

    /// All schema-level integrity invariants are jointly established: the table exists,
    /// every constraint is satisfied, and referential integrity is intact.
    ///
    /// Composite proposition — requires [`TableCreated`], [`ConstraintSatisfied`], and
    /// [`ReferentialIntegrityMaintained`].  Used as the credential in ProvableFrom chains.
    ///
    /// Source: ISO/IEC 9075-2 §11 — Schema definition; §11.6–§11.8 — Constraint definitions
    pub struct SchemaIntegrityEstablished;

    // -- Transactions --

    /// A transaction has been started (explicitly or implicitly).
    ///
    /// Source: ISO/IEC 9075-2 §17.1 — `<start transaction statement>`
    pub struct TransactionStarted;

    /// Transaction was durably committed via a `COMMIT` statement.
    ///
    /// Source: ISO/IEC 9075-2 §17.3 — `<commit statement>`
    pub struct TransactionCommitted;

    /// Transaction was rolled back via a `ROLLBACK` statement.
    ///
    /// Source: ISO/IEC 9075-2 §17.4 — `<rollback statement>`
    pub struct TransactionRolledBack;

    /// A savepoint was established within the current transaction.
    ///
    /// Source: ISO/IEC 9075-2 §17.5 — `<savepoint statement>`
    pub struct SavepointCreated;

    /// Transaction was partially rolled back to a previously established savepoint.
    ///
    /// Source: ISO/IEC 9075-2 §17.6 — `<rollback statement>` TO SAVEPOINT
    pub struct SavepointRolledBackTo;

    /// A savepoint was released, merging its work into the enclosing transaction.
    ///
    /// Source: ISO/IEC 9075-2 §17.7 — `<release savepoint statement>`
    pub struct SavepointReleased;

    /// Operation is atomic — either fully applied or fully aborted.
    ///
    /// Source: ISO/IEC 9075-2 §4.33 — Atomicity (ACID property)
    pub struct Atomic;

    /// Each transaction leaves the database in a state that satisfies all integrity constraints.
    ///
    /// Source: ISO/IEC 9075-2 §4.33 — Consistency (ACID property)
    pub struct Consistent;

    /// Committed data survives system failure without loss.
    ///
    /// Source: ISO/IEC 9075-2 §4.33 — Durability (ACID property)
    pub struct Durable;

    /// All four ACID properties are jointly established for the completed transaction.
    ///
    /// Composite proposition — requires [`Atomic`], [`Consistent`], [`Durable`], and an
    /// isolation level of at least [`SerializableIsolation`].  Used as the credential in
    /// ProvableFrom chains that enforce full transaction correctness.
    ///
    /// Source: ISO/IEC 9075-2 §4.33 — Atomicity, Consistency, Isolation, Durability
    pub struct AcidCompliant;

    // -- Access Control --

    /// A table-level privilege has been granted to a grantee.
    ///
    /// Source: ISO/IEC 9075-2 §12.3 — `<grant privilege statement>` table privilege
    pub struct TablePrivilegeGranted;

    /// A column-level privilege has been granted to a grantee.
    ///
    /// Source: ISO/IEC 9075-2 §12.3 — `<grant privilege statement>` column privilege
    pub struct ColumnPrivilegeGranted;

    /// A schema-level privilege has been granted to a grantee.
    ///
    /// Source: ISO/IEC 9075-2 §12.3 — `<grant privilege statement>` schema usage
    pub struct SchemaPrivilegeGranted;

    /// A catalog (database) level privilege has been granted to a grantee.
    ///
    /// Source: ISO/IEC 9075-2 §12.3 — `<grant privilege statement>` catalog privilege
    pub struct DatabasePrivilegeGranted;

    /// A previously granted privilege has been revoked from a grantee.
    ///
    /// Source: ISO/IEC 9075-2 §12.6 — `<revoke privilege statement>`
    pub struct PrivilegeRevoked;

    /// A role has been granted to an authorization identifier.
    ///
    /// Source: ISO/IEC 9075-2 §12.5 — `<grant role statement>`
    pub struct RoleGranted;

    /// A role has been revoked from an authorization identifier.
    ///
    /// Source: ISO/IEC 9075-2 §12.7 — `<revoke role statement>`
    pub struct RoleRevoked;

    /// The `WITH GRANT OPTION` privilege was inherited by a downstream grantee.
    ///
    /// Source: ISO/IEC 9075-2 §12.3 — `<grant privilege statement>` WITH GRANT OPTION
    pub struct GrantOptionInherited;

    // -- Set Operations --

    /// `UNION` of two query results contains exactly the rows from both inputs (without duplicates).
    ///
    /// Source: ISO/IEC 9075-2 §7.13 — `<query expression>` UNION
    pub struct UnionResultCorrect;

    /// `INTERSECT` result contains exactly the rows present in both inputs.
    ///
    /// Source: ISO/IEC 9075-2 §7.13 — `<query expression>` INTERSECT
    pub struct IntersectResultCorrect;

    /// `EXCEPT` result contains exactly the rows from the first input absent from the second.
    ///
    /// Source: ISO/IEC 9075-2 §7.13 — `<query expression>` EXCEPT
    pub struct ExceptResultCorrect;

    /// `UNION ALL` result contains all rows from both inputs, preserving duplicates.
    ///
    /// Source: ISO/IEC 9075-2 §7.13 — `<query expression>` UNION ALL
    pub struct UnionAllResultCorrect;

    /// Both operands of a set operation have type-compatible column lists.
    ///
    /// Source: ISO/IEC 9075-2 §7.13 — `<corresponding spec>` type compatibility
    pub struct SetOperationTypeCompatible;

    // -- Window Functions --

    /// A window function is defined with a valid `OVER` clause.
    ///
    /// Source: ISO/IEC 9075-2 §4.15.14 — window functions; §6.10 — `<window function>`
    pub struct WindowFunctionDefined;

    /// `PARTITION BY` correctly partitioned the result set for the window function.
    ///
    /// Source: ISO/IEC 9075-2 §6.10 — `<window partition clause>`
    pub struct PartitionByApplied;

    /// `ORDER BY` inside an `OVER` clause imposed the correct ordering within each partition.
    ///
    /// Source: ISO/IEC 9075-2 §6.10 — `<window order clause>`
    pub struct OrderByInWindowApplied;

    /// The window frame bounds (`ROWS` / `RANGE` / `GROUPS`) are valid and non-empty.
    ///
    /// Source: ISO/IEC 9075-2 §6.10 — `<window frame clause>`
    pub struct WindowFrameBoundsValid;

    /// The window function produced the correct value for every row in its partition.
    ///
    /// Source: ISO/IEC 9075-2 §6.10 — window function result semantics
    pub struct WindowFunctionResultCorrect;

    /// `RANK()` assigned ranks with gaps for tied rows.
    ///
    /// Source: ISO/IEC 9075-2 §6.10 — `<rank function type>` RANK
    pub struct RankFunctionApplied;

    /// `ROW_NUMBER()` assigned a unique sequential integer to every row in the partition.
    ///
    /// Source: ISO/IEC 9075-2 §6.10 — `<rank function type>` ROW_NUMBER
    pub struct RowNumberFunctionApplied;

    /// `DENSE_RANK()` assigned consecutive ranks without gaps for tied rows.
    ///
    /// Source: ISO/IEC 9075-2 §6.10 — `<rank function type>` DENSE_RANK
    pub struct DenseRankApplied;

    /// `LEAD` or `LAG` function accessed the correct offset row within the partition.
    ///
    /// Source: ISO/IEC 9075-2 §6.10 — `<lead or lag function>`
    pub struct LeadLagFunctionApplied;

    /// `NTH_VALUE` returned the value at the specified position within the window frame.
    ///
    /// Source: ISO/IEC 9075-2 §6.10 — `<nth value function>`
    pub struct NthValueFunctionApplied;

    /// An aggregate function was evaluated over a window defined by an `OVER` clause.
    ///
    /// Source: ISO/IEC 9075-2 §6.10 — aggregate window function
    pub struct AggregateOverWindowApplied;

    // -- CTEs --

    /// A `WITH` clause is syntactically and semantically valid.
    ///
    /// Source: ISO/IEC 9075-2 §7.17 — `<with clause>`
    pub struct WithClauseDefined;

    /// A recursive CTE terminates; the recursive union reaches an empty working table.
    ///
    /// Source: ISO/IEC 9075-2 §7.17 — `<with list element>` WITH RECURSIVE termination
    pub struct RecursiveCteTerminates;

    /// A non-recursive CTE result has been fully materialized before the main query runs.
    ///
    /// Source: ISO/IEC 9075-2 §7.17 — `<with list element>` non-recursive materialization
    pub struct NonRecursiveCteResultMaterialized;

    /// The CTE name is referenced at least once in the main query or another CTE.
    ///
    /// Source: ISO/IEC 9075-2 §7.17 — `<with list element>` reference in query body
    pub struct CteReferencedInMainQuery;

    /// The recursive term of a recursive CTE uses `UNION ALL`, not `UNION DISTINCT`.
    ///
    /// Source: ISO/IEC 9075-2 §7.17 — `<with list element>` UNION ALL requirement
    pub struct RecursiveCteUnionAllUsed;

    // -- Data Types --

    /// A numeric value was stored and retrieved without loss of precision.
    ///
    /// Source: ISO/IEC 9075-2 §6.1 — `<data type>` NUMERIC / DECIMAL precision
    pub struct NumericPrecisionMaintained;

    /// A numeric value was stored and retrieved without loss of scale (decimal places).
    ///
    /// Source: ISO/IEC 9075-2 §6.1 — `<data type>` NUMERIC / DECIMAL scale
    pub struct NumericScaleMaintained;

    /// A temporal value (DATE, TIME, TIMESTAMP) is within the valid calendar range.
    ///
    /// Source: ISO/IEC 9075-2 §6.1 — `<datetime type>` value range constraints
    pub struct TemporalValueValid;

    /// A timezone-aware temporal value has been normalized to a canonical offset.
    ///
    /// Source: ISO/IEC 9075-2 §6.1 — `<datetime type>` WITH TIME ZONE normalization
    pub struct TimezoneNormalized;

    /// String comparison was performed using the declared collation of the column.
    ///
    /// Source: ISO/IEC 9075-2 §6.1 — `<character string type>` with `<collate clause>`
    pub struct StringCollationApplied;

    /// A character string value does not exceed the declared maximum length.
    ///
    /// Source: ISO/IEC 9075-2 §6.1 — `<character string type>` length constraint
    pub struct StringLengthWithinBounds;

    /// A BOOLEAN column holds only TRUE, FALSE, or UNKNOWN (null).
    ///
    /// Source: ISO/IEC 9075-2 §6.1 — `<boolean type>`
    pub struct BooleanValueValid;

    /// A binary string value is well-formed and within the declared octet limit.
    ///
    /// Source: ISO/IEC 9075-2 §6.1 — `<binary string type>`
    pub struct BinaryValueValid;

    /// A JSON value is well-formed according to RFC 7159 / SQL/JSON path semantics.
    ///
    /// Source: ISO/IEC 9075-2 §6.1 — `<JSON type>` (SQL:2016 extension)
    pub struct JsonValueValid;

    /// An array value has the expected number of dimensions and element types.
    ///
    /// Source: ISO/IEC 9075-2 §6.1 — `<array type>` dimensionality
    pub struct ArrayDimensionsValid;

    /// All fields of a composite (row) type value are present and of the correct type.
    ///
    /// Source: ISO/IEC 9075-2 §6.1 — `<row type>` field validation
    pub struct CompositeTypeFieldsValid;

    /// An INTERVAL value is within a representable range for its qualifier.
    ///
    /// Source: ISO/IEC 9075-2 §6.1 — `<interval type>` range validation
    pub struct IntervalValueValid;

    /// A UUID value conforms to the RFC 4122 format (8-4-4-4-12 hex groups).
    ///
    /// Source: ISO/IEC 9075-2 §6.1 — implementation-defined UUID type
    pub struct UuidValueValid;

    // -- Cursors --

    /// A cursor has been declared with a valid `DECLARE CURSOR` statement.
    ///
    /// Source: ISO/IEC 9075-2 §14.3 — `<declare cursor>`
    pub struct CursorDeclared;

    /// A previously declared cursor has been opened with an `OPEN` statement.
    ///
    /// Source: ISO/IEC 9075-2 §14.4 — `<open statement>`
    pub struct CursorOpened;

    /// A row was successfully fetched from an open cursor.
    ///
    /// Source: ISO/IEC 9075-2 §14.5 — `<fetch statement>`
    pub struct CursorFetched;

    /// An open cursor has been closed with a `CLOSE` statement.
    ///
    /// Source: ISO/IEC 9075-2 §14.6 — `<close statement>`
    pub struct CursorClosed;

    /// The cursor was declared with `SCROLL`, permitting non-sequential fetch directions.
    ///
    /// Source: ISO/IEC 9075-2 §14.3 — `<declare cursor>` with SCROLL option
    pub struct CursorScrollable;

    // -- Query Planning --

    /// The query executor used an index scan rather than a full table scan.
    ///
    /// Source: ISO/IEC 9075-2 §4.15 — access path selection (implementation-defined)
    pub struct IndexScanUsed;

    /// The query executor performed a full sequential scan of the relation.
    ///
    /// Source: ISO/IEC 9075-2 §4.15 — access path selection (implementation-defined)
    pub struct SequentialScanUsed;

    /// The query executor used a bitmap index scan to combine multiple index results.
    ///
    /// Source: ISO/IEC 9075-2 §4.15 — access path selection (implementation-defined)
    pub struct BitmapIndexScanUsed;

    /// The join was executed using an in-memory hash table (hash join algorithm).
    ///
    /// Source: ISO/IEC 9075-2 §4.15 — join algorithm selection (implementation-defined)
    pub struct HashJoinUsed;

    /// The join was executed using a nested-loop algorithm.
    ///
    /// Source: ISO/IEC 9075-2 §4.15 — join algorithm selection (implementation-defined)
    pub struct NestedLoopJoinUsed;

    /// The join was executed using a sort-merge algorithm on ordered inputs.
    ///
    /// Source: ISO/IEC 9075-2 §4.15 — join algorithm selection (implementation-defined)
    pub struct MergeJoinUsed;

    /// An `EXPLAIN` (or equivalent) plan was generated for the query.
    ///
    /// Source: ISO/IEC 9075-2 §4.15 — implementation-defined query plan introspection
    pub struct ExplainPlanGenerated;

    /// An `EXPLAIN ANALYZE` (or equivalent) plan with runtime statistics was generated.
    ///
    /// Source: ISO/IEC 9075-2 §4.15 — implementation-defined runtime plan introspection
    pub struct AnalyzePlanGenerated;

    // -- NULL Semantics --

    /// `IS NULL` predicate evaluated correctly (§8.8 three-valued logic).
    ///
    /// Source: ISO/IEC 9075-2 §8.8 — `<null predicate>`
    pub struct IsNullPredicateEvaluated;

    /// `IS NOT NULL` predicate evaluated correctly (§8.8 three-valued logic).
    ///
    /// Source: ISO/IEC 9075-2 §8.8 — `<null predicate>`
    pub struct IsNotNullPredicateEvaluated;

    /// A null value propagated correctly through an arithmetic or string expression.
    ///
    /// Source: ISO/IEC 9075-2 §6.27 — `<numeric value expression>` null propagation
    pub struct NullPropagatedInExpression;

    // -- Conditional Expressions --

    /// `CASE` expression evaluated all WHEN/THEN/ELSE branches and returned the correct result.
    ///
    /// Source: ISO/IEC 9075-2 §6.12 — `<case expression>`
    pub struct CaseExpressionEvaluated;

    /// `COALESCE(…)` returned the first non-null value from its argument list.
    ///
    /// Source: ISO/IEC 9075-2 §6.11 — `<coalesce>`
    pub struct CoalesceEvaluated;

    /// `NULLIF(v1, v2)` returned null when the arguments were equal, else the first argument.
    ///
    /// Source: ISO/IEC 9075-2 §6.11 — `<nullif>`
    pub struct NullIfEvaluated;

    macro_rules! structural_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by DDL/DML success */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by DDL/DML success */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by DDL/DML success */ }
                }
            }
        };
    }

    // -- DDL: Table --
    structural_prop!(TableCreated, "TableCreated");
    structural_prop!(TableAltered, "TableAltered");
    structural_prop!(TableDropped, "TableDropped");
    structural_prop!(TableRenamed, "TableRenamed");
    structural_prop!(ColumnAdded, "ColumnAdded");
    structural_prop!(ColumnDropped, "ColumnDropped");
    structural_prop!(ColumnRenamed, "ColumnRenamed");
    structural_prop!(ColumnDefaultSet, "ColumnDefaultSet");
    structural_prop!(ColumnDefaultDropped, "ColumnDefaultDropped");

    // -- DDL: View --
    structural_prop!(ViewCreated, "ViewCreated");
    structural_prop!(ViewDropped, "ViewDropped");
    structural_prop!(ViewQueryValid, "ViewQueryValid");

    // -- DDL: Schema --
    structural_prop!(SchemaCreated, "SchemaCreated");
    structural_prop!(SchemaDropped, "SchemaDropped");

    // -- DDL: Database / Catalog --
    structural_prop!(DatabaseCreated, "DatabaseCreated");
    structural_prop!(DatabaseDropped, "DatabaseDropped");

    // -- DDL: Domain --
    structural_prop!(DomainCreated, "DomainCreated");
    structural_prop!(DomainConstraintSatisfied, "DomainConstraintSatisfied");
    structural_prop!(DomainAltered, "DomainAltered");
    structural_prop!(DomainDropped, "DomainDropped");

    // -- DDL: Sequence --
    structural_prop!(SequenceCreated, "SequenceCreated");
    structural_prop!(SequenceAdvanced, "SequenceAdvanced");
    structural_prop!(SequenceCycled, "SequenceCycled");
    structural_prop!(SequenceAltered, "SequenceAltered");
    structural_prop!(SequenceDropped, "SequenceDropped");

    // -- DDL: Index --
    structural_prop!(IndexCreated, "IndexCreated");
    structural_prop!(IndexDropped, "IndexDropped");
    structural_prop!(IndexValid, "IndexValid");

    // -- DDL: User-Defined Type --
    structural_prop!(TypeCreated, "TypeCreated");
    structural_prop!(TypeDropped, "TypeDropped");

    // -- DML: Mutation --
    structural_prop!(RowInserted, "RowInserted");
    structural_prop!(RowUpdated, "RowUpdated");
    structural_prop!(RowDeleted, "RowDeleted");
    structural_prop!(RowMerged, "RowMerged");

    // -- DML: Query result cardinality --
    structural_prop!(NonEmptyResult, "NonEmptyResult");
    structural_prop!(EmptyResult, "EmptyResult");
    structural_prop!(AffectedRowCountCorrect, "AffectedRowCountCorrect");

    // -- DML: INSERT correctness --
    structural_prop!(InsertedValuesMatchColumns, "InsertedValuesMatchColumns");

    // -- DML: UPDATE correctness --
    structural_prop!(UpdatedColumnSubsetValid, "UpdatedColumnSubsetValid");

    // -- DML: DELETE correctness --
    structural_prop!(DeletePredicateApplied, "DeletePredicateApplied");

    // -- DML: TRUNCATE --
    structural_prop!(TruncateRowsRemoved, "TruncateRowsRemoved");

    // -- DML: MERGE --
    structural_prop!(MergeMatchedApplied, "MergeMatchedApplied");
    structural_prop!(MergeNotMatchedApplied, "MergeNotMatchedApplied");

    // -- DML: SELECT correctness --
    structural_prop!(SelectPredicateApplied, "SelectPredicateApplied");
    structural_prop!(SelectColumnListValid, "SelectColumnListValid");
    structural_prop!(SelectDistinctApplied, "SelectDistinctApplied");
    structural_prop!(OrderByApplied, "OrderByApplied");
    structural_prop!(NullsFirstApplied, "NullsFirstApplied");
    structural_prop!(NullsLastApplied, "NullsLastApplied");
    structural_prop!(LimitApplied, "LimitApplied");
    structural_prop!(OffsetApplied, "OffsetApplied");
    structural_prop!(FetchFirstRowsApplied, "FetchFirstRowsApplied");
    structural_prop!(GroupByApplied, "GroupByApplied");
    structural_prop!(GroupByRollupApplied, "GroupByRollupApplied");
    structural_prop!(GroupByCubeApplied, "GroupByCubeApplied");
    structural_prop!(GroupingSetsApplied, "GroupingSetsApplied");
    structural_prop!(HavingFilterApplied, "HavingFilterApplied");
    structural_prop!(AggregateFilterApplied, "AggregateFilterApplied");
    structural_prop!(SubqueryCorrelated, "SubqueryCorrelated");
    structural_prop!(SubqueryUncorrelated, "SubqueryUncorrelated");
    structural_prop!(LateralJoinValid, "LateralJoinValid");

    // -- NULL semantics --
    structural_prop!(IsNullPredicateEvaluated, "IsNullPredicateEvaluated");
    structural_prop!(IsNotNullPredicateEvaluated, "IsNotNullPredicateEvaluated");
    structural_prop!(NullPropagatedInExpression, "NullPropagatedInExpression");

    // -- Conditional expressions --
    structural_prop!(CaseExpressionEvaluated, "CaseExpressionEvaluated");
    structural_prop!(CoalesceEvaluated, "CoalesceEvaluated");
    structural_prop!(NullIfEvaluated, "NullIfEvaluated");

    // -- Constraints: FOREIGN KEY --
    structural_prop!(
        ReferentialIntegrityMaintained,
        "ReferentialIntegrityMaintained"
    );

    // -- Constraints: Aggregate --
    structural_prop!(ConstraintSatisfied, "ConstraintSatisfied");
    structural_prop!(ConstraintDeferred, "ConstraintDeferred");

    // -- Transactions --
    structural_prop!(TransactionStarted, "TransactionStarted");
    structural_prop!(TransactionCommitted, "TransactionCommitted");
    structural_prop!(TransactionRolledBack, "TransactionRolledBack");
    structural_prop!(SavepointCreated, "SavepointCreated");
    structural_prop!(SavepointRolledBackTo, "SavepointRolledBackTo");
    structural_prop!(SavepointReleased, "SavepointReleased");
    structural_prop!(Atomic, "Atomic");
    structural_prop!(Consistent, "Consistent");
    structural_prop!(Durable, "Durable");
    structural_prop!(SchemaIntegrityEstablished, "SchemaIntegrityEstablished");
    structural_prop!(AcidCompliant, "AcidCompliant");

    // -- Access Control --
    structural_prop!(TablePrivilegeGranted, "TablePrivilegeGranted");
    structural_prop!(ColumnPrivilegeGranted, "ColumnPrivilegeGranted");
    structural_prop!(SchemaPrivilegeGranted, "SchemaPrivilegeGranted");
    structural_prop!(DatabasePrivilegeGranted, "DatabasePrivilegeGranted");
    structural_prop!(PrivilegeRevoked, "PrivilegeRevoked");
    structural_prop!(RoleGranted, "RoleGranted");
    structural_prop!(RoleRevoked, "RoleRevoked");
    structural_prop!(GrantOptionInherited, "GrantOptionInherited");

    // -- Set Operations --
    structural_prop!(UnionResultCorrect, "UnionResultCorrect");
    structural_prop!(IntersectResultCorrect, "IntersectResultCorrect");
    structural_prop!(ExceptResultCorrect, "ExceptResultCorrect");
    structural_prop!(UnionAllResultCorrect, "UnionAllResultCorrect");
    structural_prop!(SetOperationTypeCompatible, "SetOperationTypeCompatible");

    // -- Window Functions --
    structural_prop!(WindowFunctionDefined, "WindowFunctionDefined");
    structural_prop!(PartitionByApplied, "PartitionByApplied");
    structural_prop!(OrderByInWindowApplied, "OrderByInWindowApplied");
    structural_prop!(WindowFrameBoundsValid, "WindowFrameBoundsValid");
    structural_prop!(WindowFunctionResultCorrect, "WindowFunctionResultCorrect");
    structural_prop!(RankFunctionApplied, "RankFunctionApplied");
    structural_prop!(RowNumberFunctionApplied, "RowNumberFunctionApplied");
    structural_prop!(DenseRankApplied, "DenseRankApplied");
    structural_prop!(LeadLagFunctionApplied, "LeadLagFunctionApplied");
    structural_prop!(NthValueFunctionApplied, "NthValueFunctionApplied");
    structural_prop!(AggregateOverWindowApplied, "AggregateOverWindowApplied");

    // -- CTEs --
    structural_prop!(WithClauseDefined, "WithClauseDefined");
    structural_prop!(RecursiveCteTerminates, "RecursiveCteTerminates");
    structural_prop!(
        NonRecursiveCteResultMaterialized,
        "NonRecursiveCteResultMaterialized"
    );
    structural_prop!(CteReferencedInMainQuery, "CteReferencedInMainQuery");
    structural_prop!(RecursiveCteUnionAllUsed, "RecursiveCteUnionAllUsed");

    // -- Data Types --
    structural_prop!(NumericPrecisionMaintained, "NumericPrecisionMaintained");
    structural_prop!(NumericScaleMaintained, "NumericScaleMaintained");
    structural_prop!(TemporalValueValid, "TemporalValueValid");
    structural_prop!(TimezoneNormalized, "TimezoneNormalized");
    structural_prop!(StringCollationApplied, "StringCollationApplied");
    structural_prop!(StringLengthWithinBounds, "StringLengthWithinBounds");
    structural_prop!(BooleanValueValid, "BooleanValueValid");
    structural_prop!(BinaryValueValid, "BinaryValueValid");
    structural_prop!(JsonValueValid, "JsonValueValid");
    structural_prop!(ArrayDimensionsValid, "ArrayDimensionsValid");
    structural_prop!(CompositeTypeFieldsValid, "CompositeTypeFieldsValid");
    structural_prop!(IntervalValueValid, "IntervalValueValid");
    structural_prop!(UuidValueValid, "UuidValueValid");

    // -- Cursors --
    structural_prop!(CursorDeclared, "CursorDeclared");
    structural_prop!(CursorOpened, "CursorOpened");
    structural_prop!(CursorFetched, "CursorFetched");
    structural_prop!(CursorClosed, "CursorClosed");
    structural_prop!(CursorScrollable, "CursorScrollable");

    // -- Query Planning --
    structural_prop!(IndexScanUsed, "IndexScanUsed");
    structural_prop!(SequentialScanUsed, "SequentialScanUsed");
    structural_prop!(BitmapIndexScanUsed, "BitmapIndexScanUsed");
    structural_prop!(HashJoinUsed, "HashJoinUsed");
    structural_prop!(NestedLoopJoinUsed, "NestedLoopJoinUsed");
    structural_prop!(MergeJoinUsed, "MergeJoinUsed");
    structural_prop!(ExplainPlanGenerated, "ExplainPlanGenerated");
    structural_prop!(AnalyzePlanGenerated, "AnalyzePlanGenerated");
}

pub use emit_impls::{
    AcidCompliant,
    AffectedRowCountCorrect,
    AggregateFilterApplied,
    AggregateOverWindowApplied,
    AnalyzePlanGenerated,
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
    ColumnPrivilegeGranted,
    ColumnRenamed,
    CompositeTypeFieldsValid,
    Consistent,
    ConstraintDeferred,
    // Constraints: Aggregate
    ConstraintSatisfied,
    CteReferencedInMainQuery,
    CursorClosed,
    // Cursors
    CursorDeclared,
    CursorFetched,
    CursorOpened,
    CursorScrollable,
    // DDL: Database / Catalog
    DatabaseCreated,
    DatabaseDropped,
    DatabasePrivilegeGranted,
    // DML: DELETE correctness
    DeletePredicateApplied,
    DenseRankApplied,
    // DDL: Domain lifecycle
    DomainAltered,
    DomainConstraintSatisfied,
    // DDL: Domain
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
    // DDL: Index
    IndexCreated,
    IndexDropped,
    // Query Planning
    IndexScanUsed,
    IndexValid,
    // DML: INSERT correctness
    InsertedValuesMatchColumns,
    IntersectResultCorrect,
    IntervalValueValid,
    IsNotNullPredicateEvaluated,
    // NULL Semantics
    IsNullPredicateEvaluated,
    JsonValueValid,
    LateralJoinValid,
    LeadLagFunctionApplied,
    LimitApplied,
    MergeJoinUsed,
    // DML: MERGE
    MergeMatchedApplied,
    MergeNotMatchedApplied,
    NestedLoopJoinUsed,
    // DML: Query result cardinality
    NonEmptyResult,
    NonRecursiveCteResultMaterialized,
    NthValueFunctionApplied,
    NullIfEvaluated,
    NullPropagatedInExpression,
    // SELECT: Advanced ordering and grouping
    NullsFirstApplied,
    NullsLastApplied,
    // Data Types
    NumericPrecisionMaintained,
    NumericScaleMaintained,
    OffsetApplied,
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
    // DML: Mutation
    RowInserted,
    RowMerged,
    RowNumberFunctionApplied,
    RowUpdated,
    SavepointCreated,
    SavepointReleased,
    SavepointRolledBackTo,
    // DDL: Schema
    SchemaCreated,
    SchemaDropped,
    // -- Constraints: Schema integrity aggregate
    SchemaIntegrityEstablished,
    SchemaPrivilegeGranted,
    SelectColumnListValid,
    SelectDistinctApplied,
    // DML: SELECT correctness
    SelectPredicateApplied,
    SequenceAdvanced,
    // DDL: Sequence lifecycle
    SequenceAltered,
    // DDL: Sequence
    SequenceCreated,
    SequenceCycled,
    SequenceDropped,
    SequentialScanUsed,
    SetOperationTypeCompatible,
    StringCollationApplied,
    StringLengthWithinBounds,
    SubqueryCorrelated,
    SubqueryUncorrelated,
    TableAltered,
    // DDL: Table
    TableCreated,
    TableDropped,
    // Access Control
    TablePrivilegeGranted,
    // DDL: ALTER TABLE decomposition
    TableRenamed,
    TemporalValueValid,
    TimezoneNormalized,
    TransactionCommitted,
    TransactionRolledBack,
    // Transactions
    TransactionStarted,
    // DML: TRUNCATE
    TruncateRowsRemoved,
    // DDL: User-Defined Type
    TypeCreated,
    // DDL: Type lifecycle
    TypeDropped,
    UnionAllResultCorrect,
    // Set Operations
    UnionResultCorrect,
    // DML: UPDATE correctness
    UpdatedColumnSubsetValid,
    UuidValueValid,
    // DDL: View
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
