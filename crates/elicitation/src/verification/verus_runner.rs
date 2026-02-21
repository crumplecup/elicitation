//! Verus proof orchestration and tracking.
//!
//! This module provides functionality to run Verus verification proofs individually,
//! track their results in CSV format, and generate summary statistics.

use anyhow::{Context, Result};
use chrono::Utc;
use csv::{Reader, Writer};
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
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

    /// All available Verus proofs.
    pub fn all() -> Vec<Self> {
        vec![
            // Bools
            Self::new("bools", "verify_bool_true"),
            Self::new("bools", "verify_bool_false"),
            // Chars
            Self::new("chars", "verify_char_alphabetic_accepts"),
            Self::new("chars", "verify_char_alphabetic_rejects"),
            Self::new("chars", "verify_char_numeric_accepts"),
            Self::new("chars", "verify_char_numeric_rejects"),
            // Collections
            Self::new("collections", "verify_vec_non_empty"),
            Self::new("collections", "verify_vec_all_satisfy"),
            Self::new("collections", "verify_option_some"),
            Self::new("collections", "verify_option_some_rejects_none"),
            Self::new("collections", "verify_result_ok"),
            Self::new("collections", "verify_hashmap_non_empty"),
            Self::new("collections", "verify_btreemap_non_empty"),
            Self::new("collections", "verify_hashset_non_empty"),
            Self::new("collections", "verify_btreeset_non_empty"),
            Self::new("collections", "verify_vecdeque_non_empty"),
            Self::new("collections", "verify_linkedlist_non_empty"),
            Self::new("collections", "verify_box_satisfies"),
            Self::new("collections", "verify_arc_satisfies"),
            Self::new("collections", "verify_rc_satisfies"),
            Self::new("collections", "verify_array_all_satisfy"),
            // Durations
            Self::new("durations", "verify_duration_positive"),
            // Floats
            Self::new("floats", "verify_f32_finite"),
            Self::new("floats", "verify_f32_positive"),
            Self::new("floats", "verify_f32_non_negative"),
            Self::new("floats", "verify_f64_finite"),
            Self::new("floats", "verify_f64_positive"),
            Self::new("floats", "verify_f64_non_negative"),
            // Integers
            Self::new("integers", "verify_i8_range_concrete"),
            Self::new("integers", "verify_i8_range_positive"),
            Self::new("integers", "verify_i8_range_singleton"),
            Self::new("integers", "verify_i16_range_concrete"),
            Self::new("integers", "verify_i32_positive"),
            Self::new("integers", "verify_i32_non_negative"),
            Self::new("integers", "verify_i32_range"),
            Self::new("integers", "verify_i64_positive"),
            Self::new("integers", "verify_i64_non_negative"),
            Self::new("integers", "verify_i64_range"),
            Self::new("integers", "verify_i128_positive"),
            Self::new("integers", "verify_i128_non_negative"),
            Self::new("integers", "verify_i128_range"),
            Self::new("integers", "verify_isize_positive"),
            Self::new("integers", "verify_isize_non_negative"),
            Self::new("integers", "verify_isize_range"),
            Self::new("integers", "verify_u8_range_concrete"),
            Self::new("integers", "verify_u16_range_concrete"),
            Self::new("integers", "verify_u32_non_zero"),
            Self::new("integers", "verify_u32_range"),
            Self::new("integers", "verify_u64_non_zero"),
            Self::new("integers", "verify_u64_range"),
            Self::new("integers", "verify_u128_non_zero"),
            Self::new("integers", "verify_u128_range"),
            Self::new("integers", "verify_usize_non_zero"),
            Self::new("integers", "verify_usize_range"),
            // Mechanisms
            Self::new("mechanisms", "verify_affirm_returns_boolean"),
            Self::new("mechanisms", "verify_survey_returns_valid_variant"),
            Self::new("mechanisms", "verify_select_returns_valid_option"),
            Self::new("mechanisms", "verify_input_contracts"),
            Self::new("mechanisms", "verify_slider_validates"),
            // Networks
            Self::new("networks", "verify_ip_private"),
            Self::new("networks", "verify_ip_public"),
            Self::new("networks", "verify_ipv4_loopback"),
            Self::new("networks", "verify_ipv6_loopback"),
            Self::new("networks", "verify_ipv4"),
            Self::new("networks", "verify_ipv6"),
            Self::new("networks", "verify_uuid_v4"),
            Self::new("networks", "verify_uuid_non_nil"),
            Self::new("networks", "verify_pathbuf_contracts"),
            // Strings
            Self::new("strings", "verify_string_non_empty"),
            Self::new("strings", "verify_string_alphabetic_accepts"),
            Self::new("strings", "verify_string_alphabetic_rejects"),
            Self::new("strings", "verify_string_numeric_accepts"),
            Self::new("strings", "verify_string_numeric_rejects"),
            Self::new("strings", "verify_string_length_min"),
            Self::new("strings", "verify_string_length_max"),
            Self::new("strings", "verify_string_length_range"),
            Self::new("strings", "verify_string_starts_with"),
            Self::new("strings", "verify_string_ends_with"),
            Self::new("strings", "verify_string_contains"),
            // Regexes (if feature enabled)
            #[cfg(feature = "regex")]
            Self::new("regexes", "verify_regex_matches"),
            #[cfg(feature = "regex")]
            Self::new("regexes", "verify_regex_rejects"),
            // URLs (if feature enabled)
            #[cfg(feature = "url")]
            Self::new("urls", "verify_url_http_scheme"),
            #[cfg(feature = "url")]
            Self::new("urls", "verify_url_https_scheme"),
            #[cfg(feature = "url")]
            Self::new("urls", "verify_url_host_present"),
            #[cfg(feature = "url")]
            Self::new("urls", "verify_url_port_present"),
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

/// Run a single Verus proof.
pub fn run_verus_proof(
    proof: &VerusProof,
    verus_path: &Path,
    timeout_secs: Option<u64>,
) -> Result<VerusProofResult> {
    let start = Instant::now();

    // Create a temporary source file that imports and calls the specific proof
    let temp_src = format!(
        r#"
#![feature(register_tool)]
#![register_tool(verus)]
#![feature(custom_inner_attributes)]

use elicitation::verification::types::verus_proofs::{}::{}; 

fn main() {{
    {}();
}}
"#,
        proof.module(),
        proof.name(),
        proof.name()
    );

    let temp_dir = std::env::temp_dir();
    let temp_file = temp_dir.join(format!("verus_{}_{}.rs", proof.module(), proof.name()));
    std::fs::write(&temp_file, temp_src)
        .with_context(|| format!("Failed to write temp file: {}", temp_file.display()))?;

    // Run Verus on the temp file
    let mut cmd = Command::new(verus_path);
    cmd.arg("--crate-type=bin")
        .arg(&temp_file)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    if let Some(timeout) = timeout_secs {
        // Note: This is a simple timeout - you might want to use a crate like `wait-timeout`
        cmd.env("VERUS_TIMEOUT", timeout.to_string());
    }

    let output = cmd
        .output()
        .with_context(|| format!("Failed to execute Verus on {}", proof))?;

    let elapsed = start.elapsed().as_secs();

    // Clean up temp file
    let _ = std::fs::remove_file(&temp_file);

    // Parse output
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let (status, error_message) = if output.status.success() {
        if stdout.contains("verification results:: ") && stdout.contains(" verified, 0 errors") {
            (VerificationStatus::Success, String::new())
        } else {
            (
                VerificationStatus::Failed,
                format!("stdout: {}\nstderr: {}", stdout, stderr),
            )
        }
    } else if stderr.contains("timeout") || stdout.contains("timeout") {
        (VerificationStatus::Timeout, "Verification timed out".to_string())
    } else {
        (
            VerificationStatus::Failed,
            format!("stdout: {}\nstderr: {}", stdout, stderr),
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
    pub fn new(
        total: usize,
        passed: usize,
        failed: usize,
        errors: usize,
        skipped: usize,
    ) -> Self {
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
