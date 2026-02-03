//! Command-line interface for elicitation tools.
//!
//! Provides verification orchestration, analysis, and utilities.

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

    /// Run and manage Prusti verification proofs
    Prusti {
        /// Action to perform
        #[command(subcommand)]
        action: PrustiAction,
    },
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

/// Prusti verification actions
#[derive(Debug, Clone, Subcommand)]
pub enum PrustiAction {
    /// List all proof modules
    List,

    /// Run Prusti verification with CSV tracking
    Run {
        /// CSV output file
        #[arg(short, long, default_value = "prusti_verification_results.csv")]
        output: PathBuf,

        /// Timeout in seconds
        #[arg(short, long, default_value_t = 600)]
        timeout: u64,
    },

    /// Show summary statistics from CSV
    Summary {
        /// CSV file to analyze
        #[arg(short, long, default_value = "prusti_verification_results.csv")]
        file: PathBuf,
    },

    /// Show failed modules from CSV
    Failed {
        /// CSV file to analyze
        #[arg(short, long, default_value = "prusti_verification_results.csv")]
        file: PathBuf,
    },
}

/// Execute the CLI command.
#[tracing::instrument(skip(cli))]
pub fn execute(cli: Cli) -> anyhow::Result<()> {
    tracing::debug!("Executing CLI command");

    match cli.command() {
        Commands::Verify { action } => crate::verification::runner::handle(action),
        Commands::Prusti { action } => handle_prusti(action),
    }
}

/// Handle Prusti verification commands.
#[tracing::instrument(skip(action))]
fn handle_prusti(action: &PrustiAction) -> anyhow::Result<()> {
    tracing::debug!(action = ?action, "Handling Prusti command");

    match action {
        PrustiAction::List => crate::verification::prusti_runner::list_modules(),
        PrustiAction::Run { output, timeout } => {
            crate::verification::prusti_runner::run_all(output, *timeout)
        }
        PrustiAction::Summary { file } => crate::verification::prusti_runner::show_summary(file),
        PrustiAction::Failed { file } => crate::verification::prusti_runner::show_failed(file),
    }
}
