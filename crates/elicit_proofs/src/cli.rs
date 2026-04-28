//! CLI entry point for the `elicit_proofs` runner binary.

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::vsm::VsmAction;

/// Formal verification proof runner for the elicitation ecosystem.
#[derive(Debug, Parser)]
#[command(name = "elicit_proofs", about = "Run and track formal verification proof harnesses")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Top-level subcommands — one per verifier/proof family.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Run and manage VSM (Verified State Machine) Kani proof harnesses.
    Vsm {
        #[command(subcommand)]
        action: VsmAction,
    },
}

/// Dispatch a parsed CLI command.
pub fn execute(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Vsm { action } => crate::vsm::handle(&action),
    }
}
