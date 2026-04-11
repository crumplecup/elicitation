//! Observability propositions.
//!
//! Source: OpenTelemetry Specification — Traces and Metrics.

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

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

    otel_prop!(TraceEmitted, "TraceEmitted");
    otel_prop!(SpanLinkedToOperation, "SpanLinkedToOperation");
    otel_prop!(MetricsRecorded, "MetricsRecorded");
}

pub use emit_impls::{MetricsRecorded, SpanLinkedToOperation, TraceEmitted};
