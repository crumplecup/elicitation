//! Command-line interface for elicitation tools.
//!
//! Provides verification orchestration, analysis, and utilities.

pub mod generate;
pub mod prove;

use clap::{Parser, Subcommand};
use derive_getters::Getters;
use std::path::PathBuf;

/// Elicitation library tools and verification
#[derive(Debug, Clone, Parser, Getters)]
#[command(name = "elicitation")]
#[command(about = "Type-safe LLM elicitation with formal verification")]
pub struct Cli {
    /// The command to execute
    #[command(subcommand)]
    command: Commands,
}

/// Available commands
#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    /// Run and manage Kani verification proofs
    Verify {
        /// Action to perform
        #[command(subcommand)]
        action: VerifyAction,
    },

    /// Run and manage Verus verification proofs
    Verus {
        /// Action to perform
        #[command(subcommand)]
        action: VerusAction,
    },

    /// Run and manage Creusot verification modules
    Creusot {
        /// Action to perform
        #[command(subcommand)]
        action: CreusotAction,
    },

    /// Visualize elicitation type structure as a graph
    Graph {
        /// Action to perform
        #[command(subcommand)]
        action: GraphAction,
    },

    /// Scan source files and generate proof companion files
    Generate {
        /// Target verifier(s) to generate for
        #[command(subcommand)]
        target: GenerateTarget,
    },

    /// Run proof backends (Kani / Verus / Creusot) in the current workspace.
    ///
    /// Configuration is read from `.env` in the current directory.
    /// At least one of --kani, --verus, --creusot must be specified.
    Prove(ProveOpts),
}

/// Options for `elicitation prove`.
#[derive(Debug, Clone, clap::Args)]
pub struct ProveOpts {
    /// Run Kani verification (`cargo kani`).
    #[arg(long)]
    pub kani: bool,

    /// Run Verus verification.
    #[arg(long)]
    pub verus: bool,

    /// Run Creusot verification (`cargo creusot prove`).
    #[arg(long)]
    pub creusot: bool,

    /// Cargo package name to verify (overrides PROVE_PACKAGE / KANI_PACKAGE /
    /// CREUSOT_PACKAGE from .env).
    #[arg(short = 'p', long)]
    pub package: Option<String>,

    /// Specific Kani harness to target (Kani only; skips harness discovery).
    #[arg(long)]
    pub kani_harness: Option<String>,

    /// Path to the Verus binary (overrides VERUS_PATH from .env).
    #[arg(long)]
    pub verus_path: Option<PathBuf>,

    /// Verus source file or directory (overrides VERUS_FILE from .env).
    #[arg(long)]
    pub verus_file: Option<PathBuf>,

    /// CSV file for recording per-harness Kani results (overrides KANI_CSV from .env).
    #[arg(long)]
    pub csv: Option<PathBuf>,

    /// Skip Kani harnesses already recorded as PASS in the CSV (resume a previous run).
    #[arg(long)]
    pub resume: bool,

    /// Timeout in seconds per verification unit — 0 means unlimited (default: 300).
    #[arg(long, default_value_t = 300)]
    pub timeout: u64,

    /// Print commands without executing them.
    #[arg(long)]
    pub dry_run: bool,
}

/// Verification actions
#[derive(Debug, Clone, Subcommand)]
pub enum VerifyAction {
    /// List all proof harnesses
    List,

    /// Run all proofs with CSV tracking
    Run {
        /// CSV output file
        #[arg(short, long, default_value = "kani_verification_results.csv")]
        output: PathBuf,

        /// Timeout per test in seconds
        #[arg(short, long, default_value_t = 300)]
        timeout: u64,

        /// Resume mode: skip already-passed tests
        #[arg(short, long)]
        resume: bool,

        /// Only run harnesses whose module path starts with this prefix (e.g. "kani::generated::archive_connection")
        #[arg(short, long)]
        module: Option<String>,
    },

    /// Show summary statistics from CSV
    Summary {
        /// CSV file to analyze
        #[arg(default_value = "kani_verification_results.csv")]
        file: PathBuf,
    },

    /// Show failed tests from CSV
    Failed {
        /// CSV file to analyze
        #[arg(default_value = "kani_verification_results.csv")]
        file: PathBuf,
    },
}

/// Verus verification actions
#[derive(Debug, Clone, Subcommand)]
pub enum VerusAction {
    /// List all proof modules
    List,

    /// Run Verus verification with CSV tracking
    Run {
        /// CSV output file
        #[arg(short, long, default_value = "verus_verification_results.csv")]
        output: PathBuf,

        /// Timeout per proof in seconds
        #[arg(short, long, default_value_t = 600)]
        timeout: u64,

        /// Resume mode: skip already-passed proofs
        #[arg(short, long)]
        resume: bool,

        /// Path to Verus binary (defaults to VERUS_PATH env var or ~/repos/verus/source/target-verus/release/verus)
        #[arg(long)]
        verus_path: Option<PathBuf>,
    },

    /// Show summary statistics from CSV
    Summary {
        /// CSV file to analyze
        #[arg(short, long, default_value = "verus_verification_results.csv")]
        file: PathBuf,
    },

    /// Show failed proofs from CSV
    Failed {
        /// CSV file to analyze
        #[arg(short, long, default_value = "verus_verification_results.csv")]
        file: PathBuf,
    },
}

/// Creusot verification actions
#[derive(Debug, Clone, Subcommand)]
pub enum CreusotAction {
    /// List all Creusot modules
    List,

    /// Run Creusot verification on all modules
    Run {
        /// CSV output file
        #[arg(short, long, default_value = "creusot_verification_results.csv")]
        output: PathBuf,

        /// Resume mode: skip already-passed modules
        #[arg(short, long)]
        resume: bool,
    },

    /// Run SMT provers and track per-goal results with timestamps
    Prove {
        /// Module-level CSV output file
        #[arg(long, default_value = "creusot_module_results.csv")]
        output: PathBuf,

        /// Per-goal CSV output file
        #[arg(long, default_value = "creusot_goal_results.csv")]
        goals: PathBuf,

        /// Resume mode: skip already-passed modules
        #[arg(long)]
        resume: bool,
    },

    /// Show summary statistics from CSV
    Summary {
        /// CSV file to analyze
        #[arg(short, long, default_value = "creusot_verification_results.csv")]
        file: PathBuf,
    },
}

/// Graph visualization actions
#[derive(Debug, Clone, Subcommand)]
pub enum GraphAction {
    /// List all registered graphable types
    List,

    /// Render a type's structural graph
    Render {
        /// Root type name to render (e.g. `ApplicationConfig`)
        #[arg(short, long)]
        root: String,

        /// Output format
        #[arg(short, long, default_value = "mermaid", value_parser = ["mermaid", "dot"])]
        format: String,

        /// Include primitive/generic leaf nodes in output
        #[arg(long)]
        include_primitives: bool,

        /// Write output to file instead of stdout
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

/// Proof generation targets.
#[derive(Debug, Clone, Subcommand)]
pub enum GenerateTarget {
    /// Generate Kani proof harnesses for all VSMs found in `crate_path`.
    Kani {
        /// Root directory to scan for VSM source files.
        #[arg(short = 'p', long, default_value = ".")]
        crate_path: PathBuf,

        /// Output directory for generated files (defaults to stdout preview).
        #[arg(short, long)]
        out: Option<PathBuf>,
    },

    /// Generate Verus companion proofs for all VSMs found in `crate_path`.
    Verus {
        /// Root directory to scan for VSM source files.
        #[arg(short = 'p', long, default_value = ".")]
        crate_path: PathBuf,

        /// Output directory for generated files (defaults to stdout preview).
        #[arg(short, long)]
        out: Option<PathBuf>,
    },

    /// Generate Creusot companions for all VSMs found in `crate_path`.
    Creusot {
        /// Root directory to scan for VSM source files.
        #[arg(short = 'p', long, default_value = ".")]
        crate_path: PathBuf,

        /// Output directory for generated files (defaults to stdout preview).
        #[arg(short, long)]
        out: Option<PathBuf>,
    },

    /// Generate all verifier companions for all VSMs found in `crate_path`.
    All {
        /// Root directory to scan for VSM source files.
        #[arg(short = 'p', long, default_value = ".")]
        crate_path: PathBuf,

        /// Output directory for generated files (defaults to stdout preview).
        #[arg(short, long)]
        out: Option<PathBuf>,
    },

    /// Scan `crate_path` and list discovered VSMs without generating files.
    Scan {
        /// Root directory to scan for VSM source files.
        #[arg(short = 'p', long, default_value = ".")]
        crate_path: PathBuf,
    },

    /// Generate Kani constructibility harnesses for all `#[derive(Elicit)]` types.
    Foundation {
        /// Root directory to scan for `#[derive(Elicit)]` source files.
        #[arg(short = 'p', long, default_value = ".")]
        crate_path: PathBuf,

        /// Output directory for `foundation.rs` (defaults to stdout preview).
        #[arg(short, long)]
        out: Option<PathBuf>,
    },
}

/// Execute the CLI command.
#[tracing::instrument(skip(cli))]
pub fn execute(cli: Cli) -> anyhow::Result<()> {
    tracing::debug!("Executing CLI command");

    match cli.command() {
        Commands::Verify { action } => crate::verification::runner::handle(action),
        Commands::Verus { action } => handle_verus(action),
        Commands::Creusot { action } => handle_creusot(action),
        Commands::Graph { action } => handle_graph(action),
        Commands::Generate { target } => handle_generate(target),
        Commands::Prove(opts) => {
            let config = prove::ProveConfig::resolve(opts)?;
            prove::run(&config)
        }
    }
}

/// Handle Verus verification commands.
#[tracing::instrument(skip(action))]
fn handle_verus(action: &VerusAction) -> anyhow::Result<()> {
    tracing::debug!(action = ?action, "Handling Verus command");

    match action {
        VerusAction::List => list_verus_proofs(),
        VerusAction::Run {
            output,
            timeout,
            resume,
            verus_path,
        } => run_verus_proofs(output, *timeout, *resume, verus_path.as_deref()),
        VerusAction::Summary { file } => show_verus_summary(file),
        VerusAction::Failed { file } => show_verus_failed(file),
    }
}

/// Handle Creusot verification commands.
#[tracing::instrument(skip(action))]
fn handle_creusot(action: &CreusotAction) -> anyhow::Result<()> {
    tracing::debug!(action = ?action, "Handling Creusot command");

    match action {
        CreusotAction::List => crate::verification::creusot_runner::list_modules(),
        CreusotAction::Run { output, resume } => {
            let summary = crate::verification::creusot_runner::run_all_modules(output, *resume)?;
            println!();
            println!("✅ Creusot verification complete!");
            println!("   Total: {}", summary.total());
            println!("   Passed: {}", summary.passed());
            println!("   Failed: {}", summary.failed());
            Ok(())
        }
        CreusotAction::Prove {
            output,
            goals,
            resume,
        } => {
            let workspace_root = std::env::current_dir()?;
            let summary = crate::verification::creusot_runner::run_all_modules_prove(
                output,
                goals,
                &workspace_root,
                *resume,
            )?;
            println!();
            println!("✅ Creusot prove complete!");
            println!("   Total: {}", summary.total());
            println!("   Passed: {}", summary.passed());
            println!("   Failed: {}", summary.failed());
            Ok(())
        }
        CreusotAction::Summary { file } => crate::verification::creusot_runner::show_summary(file),
    }
}

/// List all Verus proofs.
fn list_verus_proofs() -> anyhow::Result<()> {
    use crate::verification::verus_runner::VerusProof;

    let proofs = VerusProof::all();
    println!("📋 Available Verus Proofs ({} total):", proofs.len());
    println!();

    let mut by_module: std::collections::BTreeMap<String, Vec<String>> =
        std::collections::BTreeMap::new();
    for proof in proofs {
        by_module
            .entry(proof.module().to_string())
            .or_default()
            .push(proof.name().to_string());
    }

    for (module, proofs) in by_module {
        println!("  {}:", module);
        for proof in proofs {
            println!("    - {}", proof);
        }
        println!();
    }

    Ok(())
}

/// Run all Verus proofs with tracking.
fn run_verus_proofs(
    output: &std::path::Path,
    timeout: u64,
    resume: bool,
    verus_path: Option<&std::path::Path>,
) -> anyhow::Result<()> {
    use crate::verification::verus_runner;

    // Determine Verus path
    let verus_path = if let Some(path) = verus_path {
        path.to_path_buf()
    } else if let Ok(env_path) = std::env::var("VERUS_PATH") {
        // Try environment variable
        std::path::PathBuf::from(shellexpand::tilde(&env_path).to_string())
    } else {
        // Try default location
        let default_path =
            shellexpand::tilde("~/repos/verus/source/target-verus/release/verus").to_string();
        std::path::PathBuf::from(default_path)
    };

    if !verus_path.exists() {
        anyhow::bail!(
            "Verus not found at: {}\nSet VERUS_PATH environment variable or use --verus-path",
            verus_path.display()
        );
    }

    verus_runner::run_all_proofs(&verus_path, output, Some(timeout), resume)?;
    Ok(())
}

/// Show summary of Verus verification results.
fn show_verus_summary(file: &std::path::Path) -> anyhow::Result<()> {
    use crate::verification::verus_runner;

    let summary = verus_runner::summarize_results(file)?;

    println!("📊 Verus Verification Summary");
    println!("============================");
    println!();
    println!("  Total:   {}", summary.total());
    println!("  Passed:  {} ✅", summary.passed());
    println!("  Failed:  {} ❌", summary.failed());
    println!("  Errors:  {} 🔥", summary.errors());
    println!();
    println!("  Success Rate: {:.1}%", summary.success_rate());

    Ok(())
}

/// Show failed Verus proofs.
fn show_verus_failed(file: &std::path::Path) -> anyhow::Result<()> {
    use crate::verification::verus_runner;

    let failed = verus_runner::list_failed_proofs(file)?;

    if failed.is_empty() {
        println!("✅ No failed proofs!");
        return Ok(());
    }

    println!("❌ Failed Verus Proofs ({} total):", failed.len());
    println!();

    for result in failed {
        println!("  {}::{}", result.module(), result.proof());
        println!("    Status: {:?}", result.status());
        println!("    Time: {}s", result.time_seconds());
        if !result.error_message().is_empty() {
            println!(
                "    Error: {}",
                result.error_message().lines().next().unwrap_or("")
            );
        }
        println!();
    }

    Ok(())
}

/// Handle graph visualization commands.
#[tracing::instrument(skip(action))]
fn handle_graph(action: &GraphAction) -> anyhow::Result<()> {
    use crate::type_graph::{
        DotRenderer, GraphRenderer, MermaidDirection, MermaidRenderer, TypeGraph,
    };
    use std::io::Write;

    tracing::debug!(action = ?action, "Handling graph command");

    match action {
        GraphAction::List => {
            let types = TypeGraph::registered_types();
            if types.is_empty() {
                println!("No graphable types registered.");
                println!("Enable the `graph` feature and use `#[derive(Elicit)]` on your types.");
            } else {
                println!("{} registered graphable type(s):\n", types.len());
                for name in types {
                    println!("  {name}");
                }
            }
            Ok(())
        }

        GraphAction::Render {
            root,
            format,
            include_primitives,
            output,
        } => {
            tracing::debug!(root, format, "Rendering type graph");

            let graph = TypeGraph::from_root(root).map_err(|e| anyhow::anyhow!("{e}"))?;

            let rendered = match format.as_str() {
                "dot" => DotRenderer {
                    include_primitives: *include_primitives,
                    ..Default::default()
                }
                .render(&graph),
                _ => MermaidRenderer {
                    direction: MermaidDirection::TopDown,
                    include_primitives: *include_primitives,
                }
                .render(&graph),
            };

            match output {
                Some(path) => {
                    let mut file = std::fs::File::create(path)
                        .map_err(|e| anyhow::anyhow!("Cannot write to {}: {e}", path.display()))?;
                    file.write_all(rendered.as_bytes())?;
                    println!("Written to {}", path.display());
                    tracing::info!(path = %path.display(), format, "Graph written to file");
                }
                None => print!("{rendered}"),
            }

            Ok(())
        }
    }
}

/// Handle `elicitation generate` subcommands.
#[tracing::instrument(skip(target))]
fn handle_generate(target: &GenerateTarget) -> anyhow::Result<()> {
    use crate::cli::generate::{creusot_gen, foundation_gen, kani_gen, scan_elicit_types, scan_vsms, verus_gen};
    use std::io::Write;

    let (crate_path, label) = match target {
        GenerateTarget::Scan { crate_path } => {
            let vsms = scan_vsms(crate_path);
            if vsms.is_empty() {
                println!(
                    "No VerifiedStateMachine structs found in {}",
                    crate_path.display()
                );
            } else {
                println!("Found {} VSM(s):", vsms.len());
                for vsm in &vsms {
                    println!(
                        "  {} ({} transitions, {} signatures found)",
                        vsm.machine,
                        vsm.transitions.len(),
                        vsm.transition_fns.len(),
                    );
                    for t in &vsm.transitions {
                        let sig = vsm.transition_fns.iter().find(|tf| &tf.name == t);
                        match sig {
                            Some(tf) => {
                                let extra: Vec<_> =
                                    tf.extra_args().map(|a| a.name.as_str()).collect();
                                if extra.is_empty() {
                                    println!("    - {t}(state, proof)");
                                } else {
                                    println!("    - {t}(state, proof, {})", extra.join(", "));
                                }
                            }
                            None => println!("    - {t}  [signature not found]"),
                        }
                    }
                    if let Some(inv) = &vsm.invariant {
                        println!(
                            "  invariant: {} (kani={}, verus={}, body={})",
                            inv.name,
                            inv.kani_fn.as_deref().unwrap_or("?"),
                            inv.verus_fn.as_deref().unwrap_or("?"),
                            if inv.verus_inv_body.is_some() {
                                "✓"
                            } else {
                                "missing"
                            },
                        );
                    }
                }
            }
            return Ok(());
        }
        GenerateTarget::Kani { crate_path, .. } => (crate_path, "kani"),
        GenerateTarget::Verus { crate_path, .. } => (crate_path, "verus"),
        GenerateTarget::Creusot { crate_path, .. } => (crate_path, "creusot"),
        GenerateTarget::All { crate_path, .. } => (crate_path, "all"),
        GenerateTarget::Foundation { crate_path, out } => {
            let types = scan_elicit_types(crate_path);
            if types.is_empty() {
                println!("No #[derive(Elicit)] types found in {}", crate_path.display());
                return Ok(());
            }
            let content = foundation_gen::generate_foundation_file(&types, crate_path);
            emit_content(&content, "foundation.rs", out.as_deref())?;
            return Ok(());
        }
    };

    let vsms = scan_vsms(crate_path);
    tracing::info!(target = label, vsms = vsms.len(), "generating proofs");

    /// Derive a snake_case filename stem from a machine name, e.g.
    /// `ArchiveNavMachine` → `archive_nav`.
    fn machine_to_filename(machine: &str) -> String {
        machine
            .trim_end_matches("Machine")
            .chars()
            .fold(String::new(), |mut acc, c| {
                if c.is_uppercase() && !acc.is_empty() {
                    acc.push('_');
                }
                acc.push(c.to_ascii_lowercase());
                acc
            })
    }

    /// Write content to `<dir>/<filename>` or print to stdout if dir is None.
    fn emit_content(
        content: &str,
        filename: &str,
        out_dir: Option<&std::path::Path>,
    ) -> anyhow::Result<()> {
        match out_dir {
            Some(dir) => {
                std::fs::create_dir_all(dir)?;
                let path = dir.join(filename);
                std::fs::write(&path, content)?;
                println!("Written: {}", path.display());
            }
            None => {
                println!("// ── {filename} ──");
                std::io::stdout().write_all(content.as_bytes())?;
                println!();
            }
        }
        Ok(())
    }

    if matches!(label, "kani" | "all") {
        let out_dir = match target {
            GenerateTarget::Kani { out, .. } | GenerateTarget::All { out, .. } => out.as_deref(),
            _ => None,
        };
        for vsm in &vsms {
            let content = kani_gen::generate_kani_file(vsm, crate_path)
                .map_err(|e| std::io::Error::other(e))?;
            let filename = format!("{}.rs", machine_to_filename(&vsm.machine));
            emit_content(&content, &filename, out_dir)?;
        }
    }

    if matches!(label, "verus" | "all") {
        let out_dir = match target {
            GenerateTarget::Verus { out, .. } | GenerateTarget::All { out, .. } => out.as_deref(),
            _ => None,
        };
        for vsm in &vsms {
            let content = verus_gen::generate_verus_file(vsm, crate_path)
                .map_err(|e| std::io::Error::other(e))?;
            let filename = format!("{}.rs", machine_to_filename(&vsm.machine));
            emit_content(&content, &filename, out_dir)?;
        }
    }

    if matches!(label, "creusot" | "all") {
        let out_dir = match target {
            GenerateTarget::Creusot { out, .. } | GenerateTarget::All { out, .. } => out.as_deref(),
            _ => None,
        };
        for vsm in &vsms {
            let content = creusot_gen::generate_creusot_file(vsm, crate_path)
                .map_err(|e| std::io::Error::other(e))?;
            let filename = format!("{}.rs", machine_to_filename(&vsm.machine));
            emit_content(&content, &filename, out_dir)?;
        }
    }

    Ok(())
}
