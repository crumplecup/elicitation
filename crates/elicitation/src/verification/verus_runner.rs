//! Verus proof orchestration and tracking.
//!
//! This module provides functionality to run Verus verification proofs individually,
//! track their results in CSV format, and generate summary statistics.

use anyhow::{Context, Result};
use chrono::Utc;
use csv::{Reader, Writer};
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Instant;

/// A Verus proof module identifier.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, Default, Serialize, Deserialize,
)]
pub struct VerusProof {
    /// Module name (e.g., "bools", "integers")
    module: String,
    /// Proof identifier (e.g., "verify_bool_true")
    name: String,
}

impl VerusProof {
    /// Create a new proof identifier.
    pub fn new(module: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            module: module.into(),
            name: name.into(),
        }
    }

    /// All available Verus proofs from the elicitation_verus crate.
    pub fn all() -> Vec<Self> {
        vec![
            // external_types (25 proofs)
            Self::new("external_types", "verify_datetime_after_construction"),
            Self::new("external_types", "verify_datetime_before_construction"),
            Self::new("external_types", "verify_datetime_construction"),
            Self::new("external_types", "verify_duration_construction"),
            Self::new("external_types", "verify_duration_positive_construction"),
            Self::new("external_types", "verify_ip_private_construction"),
            Self::new("external_types", "verify_ip_public_construction"),
            Self::new("external_types", "verify_ipaddr_construction"),
            Self::new("external_types", "verify_ipv4_construction"),
            Self::new("external_types", "verify_ipv6_construction"),
            Self::new("external_types", "verify_json_array_construction"),
            Self::new("external_types", "verify_json_non_null_construction"),
            Self::new("external_types", "verify_json_object_construction"),
            Self::new("external_types", "verify_json_value_construction"),
            Self::new("external_types", "verify_path_absolute_construction"),
            Self::new("external_types", "verify_path_relative_construction"),
            Self::new("external_types", "verify_pathbuf_construction"),
            Self::new("external_types", "verify_regex_case_insensitive_construction"),
            Self::new("external_types", "verify_regex_construction"),
            Self::new("external_types", "verify_url_construction"),
            Self::new("external_types", "verify_url_http_construction"),
            Self::new("external_types", "verify_url_https_construction"),
            Self::new("external_types", "verify_uuid_construction"),
            Self::new("external_types", "verify_uuid_non_nil_construction"),
            Self::new("external_types", "verify_uuid_v4_construction"),
            // primitives (13 proofs)
            Self::new("primitives", "verify_bool_construction"),
            Self::new("primitives", "verify_char_construction"),
            Self::new("primitives", "verify_f32_construction"),
            Self::new("primitives", "verify_f64_construction"),
            Self::new("primitives", "verify_i16_construction"),
            Self::new("primitives", "verify_i32_construction"),
            Self::new("primitives", "verify_i64_construction"),
            Self::new("primitives", "verify_i8_construction"),
            Self::new("primitives", "verify_u16_construction"),
            Self::new("primitives", "verify_u32_construction"),
            Self::new("primitives", "verify_u64_construction"),
            Self::new("primitives", "verify_u8_construction"),
            Self::new("primitives", "verify_unit_construction"),
            // stdlib_collections (11 proofs)
            Self::new("stdlib_collections", "verify_option_is_none_true"),
            Self::new("stdlib_collections", "verify_option_is_some_true"),
            Self::new("stdlib_collections", "verify_option_none"),
            Self::new("stdlib_collections", "verify_option_some"),
            Self::new("stdlib_collections", "verify_result_err"),
            Self::new("stdlib_collections", "verify_result_is_err_true"),
            Self::new("stdlib_collections", "verify_result_is_ok_true"),
            Self::new("stdlib_collections", "verify_result_ok"),
            Self::new("stdlib_collections", "verify_tuple2_construction"),
            Self::new("stdlib_collections", "verify_tuple3_construction"),
            Self::new("stdlib_collections", "verify_tuple4_construction"),
        ]
    }
}

impl std::fmt::Display for VerusProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}::{}", self.module, self.name)
    }
}

/// Result of running a single Verus proof.
#[derive(Debug, Clone, Serialize, Deserialize, Getters)]
pub struct VerusProofResult {
    /// Module name
    #[serde(rename = "Module")]
    module: String,
    /// Proof name
    #[serde(rename = "Proof")]
    proof: String,
    /// Verification status
    #[serde(rename = "Status")]
    status: VerificationStatus,
    /// Time taken in seconds
    #[serde(rename = "Time_Seconds")]
    time_seconds: u64,
    /// Timestamp of verification
    #[serde(rename = "Timestamp")]
    timestamp: String,
    /// Error message (if failed)
    #[serde(rename = "Error_Message")]
    error_message: String,
}

/// Verification status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum VerificationStatus {
    /// Proof verified successfully
    Success,
    /// Proof failed verification
    Failed,
    /// Verification timed out
    Timeout,
    /// Error running verifier
    Error,
}

impl VerusProofResult {
    /// Create a new proof result.
    pub fn new(
        module: impl Into<String>,
        proof: impl Into<String>,
        status: VerificationStatus,
        time_seconds: u64,
        error_message: impl Into<String>,
    ) -> Self {
        Self {
            module: module.into(),
            proof: proof.into(),
            status,
            time_seconds,
            timestamp: Utc::now().to_rfc3339(),
            error_message: error_message.into(),
        }
    }

    /// Whether the proof succeeded.
    pub fn is_success(&self) -> bool {
        self.status == VerificationStatus::Success
    }
}

/// Run a single Verus proof by running entire elicitation_verus crate and extracting results.
pub fn run_verus_proof(
    proof: &VerusProof,
    verus_path: &Path,
    timeout_secs: Option<u64>,
) -> Result<VerusProofResult> {
    let start = Instant::now();

    // Run Verus on the entire elicitation_verus crate with JSON output
    let crate_path = Path::new("crates/elicitation_verus/src/lib.rs");
    
    let mut cmd = Command::new(verus_path);
    cmd.arg("--crate-type=lib")
        .arg("--output-json")
        .arg(crate_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    if let Some(timeout) = timeout_secs {
        cmd.env("VERUS_TIMEOUT", timeout.to_string());
    }

    let output = cmd
        .output()
        .with_context(|| format!("Failed to execute Verus on {}", proof))?;

    let elapsed = start.elapsed().as_secs();

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Parse JSON output to extract result for this specific proof
    let (status, error_message) = if output.status.success() {
        match parse_verus_json_for_proof(&stdout, proof) {
            Ok(true) => (VerificationStatus::Success, String::new()),
            Ok(false) => (
                VerificationStatus::Failed,
                "Proof verification failed".to_string(),
            ),
            Err(e) => (
                VerificationStatus::Failed,
                format!("JSON parsing error: {}\nstderr: {}", e, stderr),
            ),
        }
    } else if stderr.contains("timeout") || stdout.contains("timeout") {
        (
            VerificationStatus::Timeout,
            "Verification timed out".to_string(),
        )
    } else {
        (
            VerificationStatus::Failed,
            format!("Verus execution failed\nstderr: {}", stderr),
        )
    };

    Ok(VerusProofResult::new(
        proof.module(),
        proof.name(),
        status,
        elapsed,
        error_message,
    ))
}

/// Parse Verus JSON output to check if a specific proof passed.
fn parse_verus_json_for_proof(output: &str, proof: &VerusProof) -> Result<bool> {
    // Find the JSON object in the output (starts after text output)
    let lines: Vec<&str> = output.lines().collect();
    let mut json_lines = Vec::new();
    let mut in_json = false;
    let mut brace_count = 0;

    for line in lines {
        if !in_json && line.trim().starts_with('{') {
            in_json = true;
        }

        if in_json {
            json_lines.push(line);
            brace_count += line.matches('{').count() as i32;
            brace_count -= line.matches('}').count() as i32;

            if brace_count == 0 {
                break;
            }
        }
    }

    if json_lines.is_empty() {
        anyhow::bail!("No JSON found in Verus output");
    }

    let json_str = json_lines.join("\n");
    let data: Value = serde_json::from_str(&json_str)
        .context("Failed to parse Verus JSON output")?;

    let func_details = data
        .get("func-details")
        .and_then(|v| v.as_object())
        .context("Missing func-details in JSON")?;

    // Build the expected function name: lib::module::function_name
    let expected_name = format!("lib::{}::{}", proof.module(), proof.name());

    // Find the proof in func-details
    if let Some(details) = func_details.get(&expected_name) {
        let failed_notes = details
            .get("failed_proof_notes")
            .and_then(|v| v.as_array())
            .context("Missing failed_proof_notes")?;

        // If failed_proof_notes is empty, the proof passed
        Ok(failed_notes.is_empty())
    } else {
        anyhow::bail!("Proof {} not found in Verus output", expected_name);
    }
}

/// Run all Verus proofs and track results.
pub fn run_all_proofs(
    verus_path: &Path,
    output_csv: &Path,
    timeout_secs: Option<u64>,
    resume: bool,
) -> Result<VerusSummary> {
    println!("🔬 Running Verus verification proofs...");
    println!("   Verus: {}", verus_path.display());
    println!("   Output: {}", output_csv.display());
    if let Some(t) = timeout_secs {
        println!("   Timeout: {}s per proof", t);
    }
    println!();

    // Load existing results if resuming
    let mut completed_proofs = std::collections::HashSet::new();
    if resume && output_csv.exists() {
        println!("📂 Loading existing results...");
        let mut reader = Reader::from_path(output_csv)
            .with_context(|| format!("Failed to read CSV: {}", output_csv.display()))?;
        for result in reader.deserialize::<VerusProofResult>() {
            if let Ok(result) = result {
                if result.is_success() {
                    completed_proofs.insert(format!("{}::{}", result.module(), result.proof()));
                }
            }
        }
        println!("   Found {} completed proofs", completed_proofs.len());
        println!();
    }

    // Create CSV writer
    let mut writer = Writer::from_path(output_csv)
        .with_context(|| format!("Failed to create CSV: {}", output_csv.display()))?;

    let proofs = VerusProof::all();
    let total = proofs.len();
    let mut passed = 0;
    let mut failed = 0;
    let mut errors = 0;
    let mut skipped = 0;

    for (i, proof) in proofs.iter().enumerate() {
        let proof_id = format!("{}::{}", proof.module(), proof.name());

        if completed_proofs.contains(&proof_id) {
            println!(
                "[{:3}/{:3}] ⏭️  Skipping {} (already passed)",
                i + 1,
                total,
                proof_id
            );
            skipped += 1;
            continue;
        }

        print!("[{:3}/{:3}] 🔬 Verifying {}... ", i + 1, total, proof_id);
        std::io::stdout().flush().ok();

        match run_verus_proof(proof, verus_path, timeout_secs) {
            Ok(result) => {
                match result.status() {
                    VerificationStatus::Success => {
                        println!("✅ PASS ({}s)", result.time_seconds());
                        passed += 1;
                    }
                    VerificationStatus::Failed => {
                        println!("❌ FAIL ({}s)", result.time_seconds());
                        failed += 1;
                    }
                    VerificationStatus::Timeout => {
                        println!("⏱️  TIMEOUT ({}s)", result.time_seconds());
                        errors += 1;
                    }
                    VerificationStatus::Error => {
                        println!("🔥 ERROR ({}s)", result.time_seconds());
                        errors += 1;
                    }
                }

                writer
                    .serialize(&result)
                    .with_context(|| format!("Failed to write result for {}", proof))?;
                writer.flush()?;
            }
            Err(e) => {
                println!("🔥 ERROR: {}", e);
                errors += 1;
            }
        }
    }

    println!();
    println!("📊 Summary:");
    println!("   Total:   {}", total);
    println!("   Passed:  {} ✅", passed);
    println!("   Failed:  {} ❌", failed);
    println!("   Errors:  {} 🔥", errors);
    println!("   Skipped: {} ⏭️", skipped);
    println!();
    println!("Results saved to: {}", output_csv.display());

    Ok(VerusSummary::new(total, passed, failed, errors, skipped))
}

/// Summary statistics for Verus verification.
#[derive(Debug, Clone, Getters)]
pub struct VerusSummary {
    /// Total number of proofs
    total: usize,
    /// Number of passed proofs
    passed: usize,
    /// Number of failed proofs
    failed: usize,
    /// Number of errors
    errors: usize,
    /// Number of skipped proofs
    skipped: usize,
}

impl VerusSummary {
    /// Create a new summary.
    pub fn new(total: usize, passed: usize, failed: usize, errors: usize, skipped: usize) -> Self {
        Self {
            total,
            passed,
            failed,
            errors,
            skipped,
        }
    }

    /// Calculate success rate percentage.
    pub fn success_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.passed as f64 / self.total as f64) * 100.0
        }
    }
}

/// Load and summarize results from CSV.
pub fn summarize_results(csv_path: &Path) -> Result<VerusSummary> {
    let mut reader = Reader::from_path(csv_path)
        .with_context(|| format!("Failed to read CSV: {}", csv_path.display()))?;

    let mut passed = 0;
    let mut failed = 0;
    let mut errors = 0;

    for result in reader.deserialize::<VerusProofResult>() {
        let result = result?;
        match result.status() {
            VerificationStatus::Success => passed += 1,
            VerificationStatus::Failed => failed += 1,
            VerificationStatus::Timeout | VerificationStatus::Error => errors += 1,
        }
    }

    let total = passed + failed + errors;

    Ok(VerusSummary::new(total, passed, failed, errors, 0))
}

/// List all failed proofs from CSV.
pub fn list_failed_proofs(csv_path: &Path) -> Result<Vec<VerusProofResult>> {
    let mut reader = Reader::from_path(csv_path)
        .with_context(|| format!("Failed to read CSV: {}", csv_path.display()))?;

    let mut failed = Vec::new();

    for result in reader.deserialize::<VerusProofResult>() {
        let result = result?;
        if !result.is_success() {
            failed.push(result);
        }
    }

    Ok(failed)
}
