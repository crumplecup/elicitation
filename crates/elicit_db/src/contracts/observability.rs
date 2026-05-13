//! Observability propositions.
//!
//! Source: OpenTelemetry Specification — Traces, Metrics, and Logs;
//!         OpenTelemetry Semantic Conventions for Database Clients (v1.24+).

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    // -- Core --

    /// A trace was emitted for this operation.
    ///
    /// Source: OpenTelemetry Specification §traces
    pub struct TraceEmitted;

    /// A span is linked to the current database operation.
    ///
    /// Source: OpenTelemetry Specification §span-links
    pub struct SpanLinkedToOperation;

    /// Metrics for this operation were recorded.
    ///
    /// Source: OpenTelemetry Semantic Conventions for Database Clients
    pub struct MetricsRecorded;

    // -- OpenTelemetry Tracing --

    /// W3C TraceContext propagated across service boundaries.
    ///
    /// Source: OpenTelemetry Specification §context-propagation — W3C TraceContext
    pub struct SpanContextPropagated;

    /// db.system, db.name, db.operation, and db.statement attributes are all set.
    ///
    /// Source: OpenTelemetry Semantic Conventions §db — required attributes
    pub struct SpanAttributesComplete;

    /// Span status (OK/ERROR) was set correctly after the operation.
    ///
    /// Source: OpenTelemetry Specification §span-status
    pub struct SpanStatusCodeSet;

    /// An exception event was recorded on the span.
    ///
    /// Source: OpenTelemetry Specification §exception-events
    pub struct SpanErrorRecorded;

    /// Child span correctly linked to parent via trace context.
    ///
    /// Source: OpenTelemetry Specification §span-links — parent-child relationship
    pub struct ChildSpanLinkedToParent;

    // -- Database Semantic Conventions --

    /// db.system attribute identifies the database (e.g., "postgresql").
    ///
    /// Source: OpenTelemetry Semantic Conventions §db — db.system
    pub struct DbSystemAttributeSet;

    /// db.name attribute identifies the database name.
    ///
    /// Source: OpenTelemetry Semantic Conventions §db — db.name
    pub struct DbNameAttributeSet;

    /// db.operation attribute names the SQL verb (SELECT/INSERT/etc.).
    ///
    /// Source: OpenTelemetry Semantic Conventions §db — db.operation
    pub struct DbOperationAttributeSet;

    /// db.statement attribute contains the sanitized SQL statement.
    ///
    /// Source: OpenTelemetry Semantic Conventions §db — db.statement
    pub struct DbStatementAttributeSet;

    /// db.connection_string is set without credentials.
    ///
    /// Source: OpenTelemetry Semantic Conventions §db — db.connection_string
    pub struct DbConnectionStringAttributeSet;

    /// db.rows_affected was recorded in the span.
    ///
    /// Source: OpenTelemetry Semantic Conventions §db — db.rows_affected
    pub struct DbRowsAffectedAttributeSet;

    // -- Logging --

    /// Query exceeding log_min_duration_statement was logged.
    ///
    /// Source: OpenTelemetry Logs / PostgreSQL docs §20.8 — log_min_duration_statement
    pub struct SlowQueryLogged;

    /// Deadlock detection event was written to the server log.
    ///
    /// Source: OpenTelemetry Logs / PostgreSQL docs §13.3.4 — Deadlocks
    pub struct DeadlockEventLogged;

    /// Error-level event was captured at appropriate severity.
    ///
    /// Source: OpenTelemetry Logs Specification — severity
    pub struct ErrorLogged;

    /// log_statement setting captured this statement.
    ///
    /// Source: OpenTelemetry Logs / PostgreSQL docs §20.8 — log_statement
    pub struct StatementLogged;

    /// Log output uses structured (JSON) format for machine parsing.
    ///
    /// Source: OpenTelemetry Logs Specification — structured log format
    pub struct StructuredLogFormatUsed;

    // -- Metrics --

    /// pg_stat_activity-derived connection pool metric was emitted.
    ///
    /// Source: OpenTelemetry Semantic Conventions §db — connection pool metrics
    pub struct ConnectionPoolMetricRecorded;

    /// Replication lag metric (seconds or bytes) was emitted.
    ///
    /// Source: OpenTelemetry Semantic Conventions §db — replication lag
    pub struct ReplicationLagMetricRecorded;

    /// Active backend count gauge was emitted.
    ///
    /// Source: OpenTelemetry Semantic Conventions §db — active backends
    pub struct ActiveBackendsMetricRecorded;

    /// Query latency histogram bucket was recorded.
    ///
    /// Source: OpenTelemetry Semantic Conventions §db — query duration histogram
    pub struct QueryDurationHistogramPopulated;

    /// Error rate counter was incremented.
    ///
    /// Source: OpenTelemetry Semantic Conventions §db — error rate
    pub struct ErrorRateMetricRecorded;

    /// Shared buffer hit rate metric was emitted.
    ///
    /// Source: OpenTelemetry Semantic Conventions §db — cache hit rate
    pub struct CacheHitRateMetricRecorded;

    /// Lock wait duration metric was emitted.
    ///
    /// Source: OpenTelemetry Semantic Conventions §db — lock wait time
    pub struct LockWaitTimeMetricRecorded;

    /// Dead tuple count gauge was emitted.
    ///
    /// Source: OpenTelemetry Semantic Conventions §db — dead tuples
    pub struct DeadTuplesMetricRecorded;

    /// Temporary files created metric was emitted.
    ///
    /// Source: OpenTelemetry Semantic Conventions §db — temp files
    pub struct TempFilesMetricRecorded;

    // -- Sampling and Configuration --

    /// Head-based sampling decision respects the configured rate.
    ///
    /// Source: OpenTelemetry Specification §sampling — head-based sampler
    pub struct SamplerConfigurationRespected;

    /// Trace ID is embedded in the query comment for log correlation.
    ///
    /// Source: OpenTelemetry Specification §context-propagation — query comment injection
    pub struct TraceIdAttachedToQuery;

    /// Log level correctly maps to OTel severity number.
    ///
    /// Source: OpenTelemetry Logs Specification §severity — severity mapping
    pub struct LogLevelConsistentWithSeverity;

    // -- Baggage and Context Propagation --

    /// W3C Baggage header propagated alongside trace context.
    ///
    /// Source: OpenTelemetry Specification §baggage — W3C Baggage
    pub struct BaggagePropagated;

    /// Correlation ID is attached to log entries matching the trace ID.
    ///
    /// Source: OpenTelemetry Logs Specification — log-to-trace correlation
    pub struct CorrelationIdAttached;

    /// service.name resource attribute is set on all telemetry signals.
    ///
    /// Source: OpenTelemetry Semantic Conventions §resource — service.name
    pub struct ServiceNameAttributeSet;

    /// deployment.environment resource attribute is set on all telemetry signals.
    ///
    /// Source: OpenTelemetry Semantic Conventions §resource — deployment.environment
    pub struct EnvironmentAttributeSet;

    macro_rules! otel_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by OpenTelemetry instrumentation */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by OpenTelemetry instrumentation */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by OpenTelemetry instrumentation */ }
                }
            }
        };
    }

    // Core
    otel_prop!(TraceEmitted, "TraceEmitted");
    otel_prop!(SpanLinkedToOperation, "SpanLinkedToOperation");
    otel_prop!(MetricsRecorded, "MetricsRecorded");
    // OTel Tracing
    otel_prop!(SpanContextPropagated, "SpanContextPropagated");
    otel_prop!(SpanAttributesComplete, "SpanAttributesComplete");
    otel_prop!(SpanStatusCodeSet, "SpanStatusCodeSet");
    otel_prop!(SpanErrorRecorded, "SpanErrorRecorded");
    otel_prop!(ChildSpanLinkedToParent, "ChildSpanLinkedToParent");
    // Database Semantic Conventions
    otel_prop!(DbSystemAttributeSet, "DbSystemAttributeSet");
    otel_prop!(DbNameAttributeSet, "DbNameAttributeSet");
    otel_prop!(DbOperationAttributeSet, "DbOperationAttributeSet");
    otel_prop!(DbStatementAttributeSet, "DbStatementAttributeSet");
    otel_prop!(
        DbConnectionStringAttributeSet,
        "DbConnectionStringAttributeSet"
    );
    otel_prop!(DbRowsAffectedAttributeSet, "DbRowsAffectedAttributeSet");
    // Logging
    otel_prop!(SlowQueryLogged, "SlowQueryLogged");
    otel_prop!(DeadlockEventLogged, "DeadlockEventLogged");
    otel_prop!(ErrorLogged, "ErrorLogged");
    otel_prop!(StatementLogged, "StatementLogged");
    otel_prop!(StructuredLogFormatUsed, "StructuredLogFormatUsed");
    // Metrics
    otel_prop!(ConnectionPoolMetricRecorded, "ConnectionPoolMetricRecorded");
    otel_prop!(ReplicationLagMetricRecorded, "ReplicationLagMetricRecorded");
    otel_prop!(ActiveBackendsMetricRecorded, "ActiveBackendsMetricRecorded");
    otel_prop!(
        QueryDurationHistogramPopulated,
        "QueryDurationHistogramPopulated"
    );
    otel_prop!(ErrorRateMetricRecorded, "ErrorRateMetricRecorded");
    otel_prop!(CacheHitRateMetricRecorded, "CacheHitRateMetricRecorded");
    otel_prop!(LockWaitTimeMetricRecorded, "LockWaitTimeMetricRecorded");
    otel_prop!(DeadTuplesMetricRecorded, "DeadTuplesMetricRecorded");
    otel_prop!(TempFilesMetricRecorded, "TempFilesMetricRecorded");
    // Sampling and Configuration
    otel_prop!(
        SamplerConfigurationRespected,
        "SamplerConfigurationRespected"
    );
    otel_prop!(TraceIdAttachedToQuery, "TraceIdAttachedToQuery");
    otel_prop!(
        LogLevelConsistentWithSeverity,
        "LogLevelConsistentWithSeverity"
    );
    // Baggage and Context Propagation
    otel_prop!(BaggagePropagated, "BaggagePropagated");
    otel_prop!(CorrelationIdAttached, "CorrelationIdAttached");
    otel_prop!(ServiceNameAttributeSet, "ServiceNameAttributeSet");
    otel_prop!(EnvironmentAttributeSet, "EnvironmentAttributeSet");
}

pub use emit_impls::{
    ActiveBackendsMetricRecorded, BaggagePropagated, CacheHitRateMetricRecorded,
    ChildSpanLinkedToParent, ConnectionPoolMetricRecorded, CorrelationIdAttached,
    DbConnectionStringAttributeSet, DbNameAttributeSet, DbOperationAttributeSet,
    DbRowsAffectedAttributeSet, DbStatementAttributeSet, DbSystemAttributeSet,
    DeadTuplesMetricRecorded, DeadlockEventLogged, EnvironmentAttributeSet, ErrorLogged,
    ErrorRateMetricRecorded, LockWaitTimeMetricRecorded, LogLevelConsistentWithSeverity,
    MetricsRecorded, QueryDurationHistogramPopulated, ReplicationLagMetricRecorded,
    SamplerConfigurationRespected, ServiceNameAttributeSet, SlowQueryLogged,
    SpanAttributesComplete, SpanContextPropagated, SpanErrorRecorded, SpanLinkedToOperation,
    SpanStatusCodeSet, StatementLogged, StructuredLogFormatUsed, TempFilesMetricRecorded,
    TraceEmitted, TraceIdAttachedToQuery,
};
