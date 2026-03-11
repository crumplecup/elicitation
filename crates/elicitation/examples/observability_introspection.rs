//! Demonstration of observability through ElicitIntrospect.
//!
//! This example shows how to use the `ElicitIntrospect` trait to add
//! structured observability to your elicitation processes. The introspection
//! is **stateless** - no memory overhead, just static metadata queries.
//!
//! # Use Cases
//!
//! - **Tracing**: Add structured metadata to OpenTelemetry/tracing spans
//! - **Metrics**: Instrument with Prometheus counters/histograms
//! - **Debugging**: Visualize elicitation structure before execution
//! - **Agent Guidance**: Query type requirements to plan elicitation
//!
//! # Key Insight
//!
//! The state machine IS the elicitation process. Instead of tracking runtime
//! state, we **introspect the type structure** statically:
//!
//! ```text
//! Compile-Time Structure    Runtime Observation
//! ────────────────────────   ──────────────────────
//! struct Config {           "Eliciting: Config"
//!   timeout: I8Positive,    "Pattern: Survey"
//!   retries: U8NonZero,     "Fields: 2"
//! }                         "Field 1/2: timeout"
//! ```
//!
//! # Running This Example
//!
//! ```bash
//! cargo run --example observability_introspection
//! ```

use elicitation::{Elicit, ElicitIntrospect, PatternDetails, Prompt, Select};

// ============================================================================
// Example Types
// ============================================================================

#[allow(dead_code)]
#[derive(Debug, Clone, Elicit, schemars::JsonSchema)]
struct NetworkConfig {
    host: String,
    port: u16,
    timeout_sec: i32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Elicit, schemars::JsonSchema)]
enum DeploymentMode {
    Development,
    Staging,
    Production { replicas: u8 },
}

#[allow(dead_code)]
#[derive(Debug, Clone, Elicit, schemars::JsonSchema)]
struct ApplicationConfig {
    name: String,
    network: NetworkConfig,
    mode: DeploymentMode,
}

// ============================================================================
// Observability Patterns
// ============================================================================

/// Pattern 1: Tracing Instrumentation
///
/// Add structured metadata to tracing spans for debugging and monitoring.
fn trace_type_structure<T: ElicitIntrospect>() {
    let meta = T::metadata();

    tracing::info!(
        type_name = %meta.type_name,
        pattern = ?meta.pattern(),
        "Type structure"
    );

    match meta.details {
        PatternDetails::Survey { fields } => {
            tracing::info!(
                field_count = fields.len(),
                fields = ?fields.iter().map(|f| f.name).collect::<Vec<_>>(),
                "Survey pattern fields"
            );

            for (i, field) in fields.iter().enumerate() {
                tracing::debug!(
                    field_index = i,
                    field_name = %field.name,
                    field_type = %field.type_name,
                    "Field details"
                );
            }
        }
        PatternDetails::Select { variants } => {
            tracing::info!(
                variant_count = variants.len(),
                labels = ?variants.iter().map(|v| &v.label).collect::<Vec<_>>(),
                "Select pattern variants"
            );
        }
        PatternDetails::Affirm => {
            tracing::info!("Affirm pattern (boolean)");
        }
        PatternDetails::Primitive => {
            tracing::info!("Primitive pattern (direct value)");
        }
    }
}

/// Pattern 2: Pseudo-Metrics (Prometheus-style)
///
/// This shows the pattern for Prometheus metrics. In real code, you'd use
/// the `prometheus` crate, but we demonstrate the pattern here.
struct ElicitationMetrics {
    // In real code: prometheus::Counter
    elicitation_total: std::sync::atomic::AtomicU64,
    // In real code: prometheus::Histogram
    elicitation_duration_seconds: Vec<f64>,
}

impl ElicitationMetrics {
    fn new() -> Self {
        Self {
            elicitation_total: std::sync::atomic::AtomicU64::new(0),
            elicitation_duration_seconds: Vec::new(),
        }
    }

    /// Increment counter for type/pattern combination.
    ///
    /// In real Prometheus:
    /// ```ignore
    /// ELICITATION_COUNTER
    ///     .with_label_values(&[type_name, pattern])
    ///     .inc();
    /// ```
    fn record_elicitation<T: ElicitIntrospect>(&self) {
        let meta = T::metadata();
        self.elicitation_total
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        println!(
            "[METRIC] elicitation_total{{type=\"{}\",pattern=\"{}\"}} +1",
            meta.type_name,
            meta.pattern().as_str()
        );
    }

    /// Record duration for type.
    ///
    /// In real Prometheus:
    /// ```ignore
    /// ELICITATION_DURATION
    ///     .with_label_values(&[type_name])
    ///     .observe(duration.as_secs_f64());
    /// ```
    fn record_duration<T: ElicitIntrospect>(&mut self, duration_secs: f64) {
        let meta = T::metadata();
        self.elicitation_duration_seconds.push(duration_secs);

        println!(
            "[METRIC] elicitation_duration_seconds{{type=\"{}\"}} {:.3}",
            meta.type_name, duration_secs
        );
    }
}

/// Pattern 3: Agent Guidance
///
/// Query type structure to plan elicitation strategy before execution.
fn plan_elicitation_strategy<T: ElicitIntrospect>() {
    let meta = T::metadata();

    println!("\n=== Elicitation Plan for {} ===", meta.type_name);
    println!("Pattern: {:?}", meta.pattern());

    if let Some(desc) = meta.description {
        println!("Description: {}", desc);
    }

    match meta.details {
        PatternDetails::Survey { fields } => {
            println!("Strategy: Sequential field elicitation");
            println!("Steps required: {}", fields.len());
            println!("\nExecution plan:");
            for (i, field) in fields.iter().enumerate() {
                println!(
                    "  {}. Elicit {}: {} ({})",
                    i + 1,
                    field.name,
                    field.type_name,
                    field.prompt.unwrap_or("no prompt")
                );
            }
        }
        PatternDetails::Select { variants } => {
            println!("Strategy: Variant selection followed by field elicitation");
            println!("Choices: {}", variants.len());
            println!("\nAvailable variants:");
            for (i, v) in variants.iter().enumerate() {
                if v.fields.is_empty() {
                    println!("  {}. {}", i + 1, v.label);
                } else {
                    println!("  {}. {} ({} fields)", i + 1, v.label, v.fields.len());
                }
            }
        }
        PatternDetails::Affirm => {
            println!("Strategy: Yes/no confirmation");
            println!("Steps required: 1");
        }
        PatternDetails::Primitive => {
            println!("Strategy: Direct value input");
            println!("Steps required: 1");
        }
    }
}

/// Pattern 4: Nested Introspection
///
/// Show how to recursively introspect nested structures.
fn introspect_nested<T: ElicitIntrospect>(depth: usize) {
    let meta = T::metadata();
    let indent = "  ".repeat(depth);

    println!(
        "{}├─ {} [{}]",
        indent,
        meta.type_name,
        meta.pattern().as_str()
    );

    if let PatternDetails::Survey { fields } = meta.details {
        for field in fields {
            println!(
                "{}│  ├─ field: {} ({})",
                indent, field.name, field.type_name
            );
        }
    }
}

// ============================================================================
// Main Example
// ============================================================================

fn main() {
    // Initialize tracing subscriber
    use tracing_subscriber::fmt::format::FmtSpan;
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::ACTIVE)
        .with_target(false)
        .init();

    println!("🔍 ElicitIntrospect Observability Examples\n");
    println!("═══════════════════════════════════════════════\n");

    // Pattern 1: Tracing
    println!("📊 Pattern 1: Structured Tracing");
    println!("───────────────────────────────────────────────");
    trace_type_structure::<ApplicationConfig>();
    trace_type_structure::<NetworkConfig>();
    trace_type_structure::<DeploymentMode>();
    trace_type_structure::<bool>();

    // Pattern 2: Metrics
    println!("\n📈 Pattern 2: Metrics (Prometheus-style)");
    println!("───────────────────────────────────────────────");
    let mut metrics = ElicitationMetrics::new();
    metrics.record_elicitation::<ApplicationConfig>();
    metrics.record_elicitation::<NetworkConfig>();
    metrics.record_elicitation::<bool>();
    metrics.record_duration::<ApplicationConfig>(1.234);
    metrics.record_duration::<NetworkConfig>(0.567);

    // Pattern 3: Agent Guidance
    println!("\n🤖 Pattern 3: Agent Planning");
    println!("───────────────────────────────────────────────");
    plan_elicitation_strategy::<ApplicationConfig>();
    plan_elicitation_strategy::<DeploymentMode>();
    plan_elicitation_strategy::<bool>();

    // Pattern 4: Nested Introspection
    println!("\n🌳 Pattern 4: Nested Structure");
    println!("───────────────────────────────────────────────");
    println!("ApplicationConfig structure:");
    introspect_nested::<ApplicationConfig>(0);

    // Show the key insight
    println!("\n💡 Key Insights");
    println!("═══════════════════════════════════════════════");
    println!("✓ Zero state tracking - all metadata is static");
    println!("✓ O(1) memory usage - no stack traces or history");
    println!("✓ Compose with tracing/metrics without overhead");
    println!("✓ Perfect for Prometheus, OpenTelemetry, Grafana");

    println!("\n🎯 Production Usage Pattern");
    println!("═══════════════════════════════════════════════");
    println!(
        r#"
// In your production code:
#[tracing::instrument(
    skip(communicator),
    fields(
        type_name = %T::metadata().type_name,
        pattern = ?T::pattern(),
    )
)]
async fn elicit_with_observability<T: ElicitIntrospect>(
    communicator: &impl ElicitCommunicator
) -> ElicitResult<T> {{
    // Record start
    ELICITATION_COUNTER
        .with_label_values(&[T::metadata().type_name])
        .inc();

    let timer = ELICITATION_DURATION.start_timer();

    // Execute
    let result = T::elicit(communicator).await;

    // Record completion
    timer.observe_duration();

    result
}}
    "#
    );

    println!("\n✅ Example complete!");
    println!("\nFor more details, see:");
    println!("  - traits.rs:337-481 (ElicitIntrospect trait)");
    println!("  - FORMAL_VERIFICATION_LEGOS.md (proof chain)");
}
