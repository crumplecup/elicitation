//! Prusti proof orchestration and tracking.
//!
//! This module provides functionality to run Prusti verification on proof modules,
//! track their results in CSV format, and generate summary statistics.
//!
//! Unlike Kani (which runs individual harnesses), Prusti verifies all functions
//! with contracts during compilation. Therefore, we track verification per module/file.

use anyhow::{bail, Context, Result};
use chrono::Utc;
use csv::{Reader, Writer};
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// A Prusti proof module identifier.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, Default, Serialize, Deserialize,
)]
pub struct ProofModule {
    /// Module name (e.g., "ipaddr_bytes")
    name: String,
    /// Number of proof functions in this module
    proof_count: usize,
}

impl ProofModule {
    /// Create a new proof module identifier.
    pub fn new(name: impl Into<String>, proof_count: usize) -> Self {
        Self {
            name: name.into(),
            proof_count,
        }
    }
}

/// Status of a Prusti proof verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProofStatus {
    /// Verification succeeded
    Success,
    /// Verification failed
    Failed,
    /// Verification timed out
    Timeout,
}

/// Result of running Prusti verification on a module.
#[derive(Debug, Clone, Getters, Serialize, Deserialize)]
pub struct ProofResult {
    /// Module name
    module: String,
    /// Number of proofs in this module
    proof_count: usize,
    /// Verification status
    status: ProofStatus,
    /// Duration in seconds
    duration_secs: u64,
    /// Timestamp
    timestamp: String,
    /// Error message (if failed)
    error_message: Option<String>,
}

/// Summary statistics for a verification run.
#[derive(Debug, Clone, Default, Getters)]
pub struct Summary {
    /// Total modules run
    total_modules: usize,
    /// Total proofs across all modules
    total_proofs: usize,
    /// Number of modules passed
    modules_passed: usize,
    /// Number of modules failed
    modules_failed: usize,
    /// Number of modules skipped (resume mode)
    modules_skipped: usize,
    /// Number of proofs in passed modules
    proofs_passed: usize,
    /// Number of proofs in failed modules
    proofs_failed: usize,
}

impl Summary {
    fn update(&mut self, result: &ProofResult) {
        self.total_modules += 1;
        self.total_proofs += result.proof_count;

        match result.status {
            ProofStatus::Success => {
                self.modules_passed += 1;
                self.proofs_passed += result.proof_count;
            }
            ProofStatus::Failed | ProofStatus::Timeout => {
                self.modules_failed += 1;
                self.proofs_failed += result.proof_count;
            }
        }
    }
}

/// List all proof modules.
#[tracing::instrument]
pub fn list_modules() -> Result<()> {
    tracing::info!("Listing Prusti proof modules");

    for module in all_modules() {
        println!("{},{}", module.name(), module.proof_count());
    }

    let total_proofs: usize = all_modules().iter().map(|m| m.proof_count()).sum();
    tracing::info!(
        modules = all_modules().len(),
        proofs = total_proofs,
        "Listed modules"
    );

    println!();
    println!("Total modules: {}", all_modules().len());
    println!("Total proofs: {}", total_proofs);

    Ok(())
}

/// Run Prusti verification and save results to CSV.
///
/// Note: Checks edition compatibility and runs verification if on prusti-verification branch.
#[tracing::instrument]
pub fn run_all(output: &Path, timeout: u64) -> Result<()> {
    // Check if we're on a compatible edition (2021) by trying to detect workspace edition
    let workspace_toml = std::path::Path::new("Cargo.toml");
    if workspace_toml.exists() {
        let content = std::fs::read_to_string(workspace_toml)?;
        if content.contains("edition = \"2024\"") {
            // On main branch with edition 2024 - show guidance
            show_branch_guidance(output)?;
            bail!("Prusti verification requires prusti-verification branch")
        }
    }

    // Edition 2021 detected - proceed with verification
    tracing::info!(
        output = %output.display(),
        timeout,
        "Running Prusti verification"
    );

    run_verification_impl(output, timeout)
}

/// Show guidance for switching to prusti-verification branch.
fn show_branch_guidance(output: &Path) -> Result<()> {
    let modules = all_modules();
    let total_proofs: usize = modules.iter().map(|m| m.proof_count()).sum();

    println!("⚠️  Prusti Verification Edition Compatibility Notice");
    println!("====================================================");
    println!();
    println!("Prusti uses rustc 1.74 (2023) which only supports edition 2021.");
    println!("This repository uses edition 2024 on the main development branch.");
    println!();
    println!("To run Prusti verification:");
    println!("  1. Switch to the prusti-verification branch:");
    println!("     $ git checkout prusti-verification");
    println!();
    println!("  2. Run verification:");
    println!("     $ cd crates/elicitation_prusti && cargo prusti --all-features");
    println!();
    println!("  3. Or use justfile:");
    println!("     $ just verify-prusti");
    println!();
    println!("The prusti-verification branch:");
    println!("  - Uses edition 2021 (Prusti compatible)");
    println!("  - Contains identical code to main branch");
    println!("  - Automatically runs CI verification");
    println!("  - Periodically syncs from dev/main");
    println!();
    println!("See PRUSTI_BRANCH.md for full documentation.");
    println!();
    
    // Write CSV indicating verification requires branch switch
    let mut writer = Writer::from_path(output).context("Failed to create CSV file")?;
    
    writer.write_record(&[
        "module",
        "proof_count", 
        "status",
        "duration_secs",
        "timestamp",
        "error_message",
    ])?;
    
    let timestamp = Utc::now().to_rfc3339();
    for module in &modules {
        writer.write_record(&[
            module.name(),
            &module.proof_count().to_string(),
            "RequiresBranch",
            "0",
            &timestamp,
            "Use prusti-verification branch - see PRUSTI_BRANCH.md",
        ])?;
    }
    
    writer.flush()?;
    
    println!("📊 Status written to: {}", output.display());
    println!("Total modules: {}", modules.len());
    println!("Total proofs: {}", total_proofs);
    
    Ok(())
}

/// Implementation of Prusti verification (runs on edition 2021).
fn run_verification_impl(output: &Path, timeout: u64) -> Result<()> {
    use std::process::{Command, Stdio};
    use std::time::Instant;
    
    let mut writer = Writer::from_path(output).context("Failed to create CSV file")?;
    let modules = all_modules();
    let total_proofs: usize = modules.iter().map(|m| m.proof_count()).sum();

    println!("🔬 Prusti Verification Tracking");
    println!("================================");
    println!("Total modules: {}", modules.len());
    println!("Total proofs: {}", total_proofs);
    println!("CSV output: {}", output.display());
    println!("Timeout: {}s", timeout);
    println!();
    println!("Running cargo prusti on elicitation_prusti crate...");
    println!();

    let start = Instant::now();
    let timestamp = Utc::now().to_rfc3339();

    let output_result = Command::new("cargo")
        .args(["prusti", "--package", "elicitation_prusti", "--all-features"])
        .env("PRUSTI_CHECK_PANICS", "true")
        .env("PRUSTI_CHECK_OVERFLOWS", "true")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("Failed to spawn cargo prusti")?
        .wait_with_output()
        .context("Failed to run cargo prusti")?;

    let duration = start.elapsed();
    let duration_secs = duration.as_secs();

    let status = if duration_secs >= timeout {
        ProofStatus::Timeout
    } else if output_result.status.success() {
        ProofStatus::Success
    } else {
        ProofStatus::Failed
    };

    let stderr = String::from_utf8_lossy(&output_result.stderr);
    let error_message = if status != ProofStatus::Success {
        Some(stderr.to_string())
    } else {
        None
    };

    // Write one result entry per module
    let mut summary = Summary::default();
    for module in &modules {
        let result = ProofResult {
            module: module.name().clone(),
            proof_count: *module.proof_count(),
            status,
            duration_secs,
            timestamp: timestamp.clone(),
            error_message: error_message.clone(),
        };

        writer.serialize(&result).context("Failed to write CSV row")?;
        summary.update(&result);
    }

    writer.flush().context("Failed to flush CSV")?;

    // Print summary
    println!();
    println!("================================");
    match status {
        ProofStatus::Success => println!("✅ All Verifications Passed"),
        ProofStatus::Failed => println!("❌ Verification Failed"),
        ProofStatus::Timeout => println!("⏱️  Verification Timeout"),
    }
    println!();
    println!("Modules: {}", modules.len());
    println!("Proofs: {}", total_proofs);
    println!("Duration: {}s", duration_secs);
    println!();
    println!("📊 Results saved to: {}", output.display());

    if status != ProofStatus::Success {
        if let Some(err) = &error_message {
            println!();
            println!("Error output:");
            println!("{}", err);
        }
        bail!("Verification failed");
    }

    Ok(())
}

/// Show summary statistics from CSV.
#[tracing::instrument]
pub fn show_summary(file: &Path) -> Result<()> {
    tracing::info!(file = %file.display(), "Showing summary");

    if !file.exists() {
        anyhow::bail!("❌ No results found: {}", file.display());
    }

    let mut reader = Reader::from_path(file).context("Failed to open CSV file")?;
    let mut summary = Summary::default();

    for result in reader.deserialize::<ProofResult>() {
        let record = result.context("Failed to parse CSV record")?;
        summary.update(&record);
    }

    println!("📊 Prusti Verification Summary");
    println!("===============================");
    println!("Source: {}", file.display());
    println!();
    println!("Modules total: {}", summary.total_modules);
    println!("✅ Modules passed: {}", summary.modules_passed);
    println!("❌ Modules failed: {}", summary.modules_failed);
    println!();
    println!("Proofs total: {}", summary.total_proofs);
    println!("✅ Proofs passed: {}", summary.proofs_passed);
    println!("❌ Proofs failed: {}", summary.proofs_failed);

    if summary.total_modules > 0 {
        let module_pass_rate =
            (summary.modules_passed as f64 / summary.total_modules as f64) * 100.0;
        let proof_pass_rate = (summary.proofs_passed as f64 / summary.total_proofs as f64) * 100.0;
        println!();
        println!("Module pass rate: {:.1}%", module_pass_rate);
        println!("Proof pass rate: {:.1}%", proof_pass_rate);
    }

    Ok(())
}

/// Show failed modules from CSV.
#[tracing::instrument]
pub fn show_failed(file: &Path) -> Result<()> {
    tracing::info!(file = %file.display(), "Showing failures");

    if !file.exists() {
        anyhow::bail!("❌ No results found: {}", file.display());
    }

    let mut reader = Reader::from_path(file).context("Failed to open CSV file")?;
    let mut failures = Vec::new();

    for result in reader.deserialize::<ProofResult>() {
        let record: ProofResult = result.context("Failed to parse CSV record")?;
        if record.status != ProofStatus::Success {
            failures.push(record);
        }
    }

    if failures.is_empty() {
        println!("✅ No failures! All modules passed.");
        return Ok(());
    }

    println!("❌ Failed Prusti Verifications");
    println!("===============================");
    println!();

    for failure in &failures {
        println!("Module: {}", failure.module);
        println!("Proofs: {}", failure.proof_count);
        println!("Status: {:?}", failure.status);
        println!("Time: {}s", failure.duration_secs);
        if let Some(err) = &failure.error_message {
            println!("Error: {}", err);
        }
        println!();
    }

    let total_failed_proofs: usize = failures.iter().map(|f| f.proof_count).sum();
    println!("Total failed modules: {}", failures.len());
    println!("Total failed proofs: {}", total_failed_proofs);

    Ok(())
}

/// All available Prusti proof modules with their proof counts.
fn all_modules() -> Vec<ProofModule> {
    vec![
        ProofModule::new("bools", 4),
        ProofModule::new("chars", 4),
        ProofModule::new("collections", 20),
        ProofModule::new("durations", 2),
        ProofModule::new("floats", 7),
        ProofModule::new("integers", 73),
        ProofModule::new("ipaddr_bytes", 43),
        ProofModule::new("macaddr", 27),
        ProofModule::new("mechanisms", 7),
        ProofModule::new("networks", 12),
        ProofModule::new("pathbytes", 33),
        ProofModule::new("regexbytes", 45),
        ProofModule::new("regexes", 6),
        ProofModule::new("socketaddr", 30),
        ProofModule::new("strings", 8),
        ProofModule::new("urls", 10),
        ProofModule::new("urlbytes", 46),
        ProofModule::new("utf8", 17),
        ProofModule::new("uuid_bytes", 33),
    ]
}
