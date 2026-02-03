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

/// Execute the CLI command.
#[tracing::instrument(skip(cli))]
pub fn execute(cli: Cli) -> anyhow::Result<()> {
    tracing::debug!("Executing CLI command");

    match cli.command() {
        Commands::Verify { action } => crate::verification::runner::handle(action),
    }
}
