//! Kani proof orchestration and tracking.
//!
//! This module provides functionality to run Kani verification proofs individually,
//! track their results in CSV format, and generate summary statistics.

use crate::cli::VerifyAction;
use anyhow::{Context, Result};
use chrono::Utc;
use csv::{Reader, Writer};
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Instant;

/// A Kani proof harness identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, Default, Serialize, Deserialize)]
pub struct ProofHarness {
    /// Module name (e.g., "ipaddr_bytes")
    module: String,
    /// Harness function name (e.g., "verify_ipv4_10_network_is_private")
    name: String,
}

impl ProofHarness {
    /// Create a new proof harness identifier.
    pub fn new(module: impl Into<String>, name: impl Into<String>) -> Self {
        Self { 
            module: module.into(), 
            name: name.into() 
        }
    }

    /// All available proof harnesses.
    pub fn all() -> Vec<Self> {
        vec![
            Self::new("bools", "verify_bool_true"),
            Self::new("bools", "verify_bool_false"),
            Self::new("chars", "verify_char_alphabetic_accepts"),
            Self::new("chars", "verify_char_alphabetic_rejects"),
            Self::new("chars", "verify_char_numeric_accepts"),
            Self::new("chars", "verify_char_numeric_rejects"),
            Self::new("collections", "verify_vec_non_empty"),
            Self::new("collections", "verify_vec_all_satisfy"),
            Self::new("collections", "verify_hashmap_wrapper_logic"),
            Self::new("collections", "verify_btreemap_non_empty"),
            Self::new("collections", "verify_hashset_non_empty"),
            Self::new("collections", "verify_btreeset_non_empty"),
            Self::new("collections", "verify_vecdeque_non_empty"),
            Self::new("collections", "verify_linkedlist_non_empty"),
            Self::new("collections", "verify_box_satisfies"),
            Self::new("collections", "verify_arc_satisfies"),
            Self::new("collections", "verify_rc_satisfies"),
            Self::new("collections", "verify_result_ok"),
            Self::new("collections", "verify_tuple3_composition"),
            Self::new("collections", "verify_tuple4_composition"),
            Self::new("collections", "verify_value_object"),
            Self::new("collections", "verify_value_array"),
            Self::new("collections", "verify_value_non_null"),
            Self::new("collections", "verify_datetime_utc_after"),
            Self::new("collections", "verify_datetime_utc_before"),
            Self::new("collections", "verify_timestamp_after"),
            Self::new("collections", "verify_timestamp_before"),
            Self::new("collections", "verify_offset_datetime_after"),
            Self::new("collections", "verify_offset_datetime_before"),
            Self::new("collections", "verify_i8_range_concrete"),
            Self::new("collections", "verify_i8_range_positive"),
            Self::new("collections", "verify_u8_range_concrete"),
            Self::new("collections", "verify_i8_range_singleton"),
            Self::new("collections", "verify_i16_range_concrete"),
            Self::new("collections", "verify_u16_range_concrete"),
            Self::new("collections", "verify_i32_positive"),
            Self::new("collections", "verify_i32_non_negative"),
            Self::new("collections", "verify_i32_range"),
            Self::new("collections", "verify_i64_positive"),
            Self::new("collections", "verify_i64_non_negative"),
            Self::new("collections", "verify_i64_range"),
            Self::new("collections", "verify_i128_positive"),
            Self::new("collections", "verify_i128_non_negative"),
            Self::new("collections", "verify_i128_range"),
            Self::new("collections", "verify_u32_non_zero"),
            Self::new("collections", "verify_u32_range"),
            Self::new("collections", "verify_u64_non_zero"),
            Self::new("collections", "verify_u64_range"),
            Self::new("collections", "verify_u128_non_zero"),
            Self::new("collections", "verify_u128_range"),
            Self::new("collections", "verify_isize_positive"),
            Self::new("collections", "verify_isize_non_negative"),
            Self::new("collections", "verify_isize_range"),
            Self::new("collections", "verify_usize_non_zero"),
            Self::new("collections", "verify_usize_range"),
            Self::new("durations", "verify_duration_positive"),
            Self::new("durations", "verify_duration_hours_min"),
            Self::new("durations", "verify_duration_hours_max"),
            Self::new("durations", "verify_duration_minutes_min"),
            Self::new("durations", "verify_duration_minutes_max"),
            Self::new("durations", "verify_duration_seconds_min"),
            Self::new("durations", "verify_duration_seconds_max"),
            Self::new("durations", "verify_duration_negative"),
            Self::new("durations", "verify_duration_zero"),
            Self::new("floats", "verify_f32_non_nan"),
            Self::new("floats", "verify_f64_non_nan"),
            Self::new("integers", "verify_port_non_zero"),
            Self::new("integers", "verify_u8_non_zero"),
            Self::new("integers", "verify_u16_non_zero"),
            Self::new("integers", "verify_u32_non_zero"),
            Self::new("integers", "verify_u64_non_zero"),
            Self::new("ipaddr_bytes", "verify_ipv4_10_network_is_private"),
            Self::new("ipaddr_bytes", "verify_ipv4_172_16_31_is_private"),
            Self::new("ipaddr_bytes", "verify_ipv4_172_outside_range_not_private"),
            Self::new("ipaddr_bytes", "verify_ipv4_192_168_is_private"),
            Self::new("ipaddr_bytes", "verify_ipv4_192_non_168_not_private"),
            Self::new("ipaddr_bytes", "verify_ipv4_loopback_not_public"),
            Self::new("ipaddr_bytes", "verify_ipv4_multicast_not_public"),
            Self::new("ipaddr_bytes", "verify_ipv4_private_roundtrip"),
            Self::new("ipaddr_bytes", "verify_ipv4_public_construction"),
            Self::new("ipaddr_bytes", "verify_ipv4_roundtrip"),
            Self::new("ipaddr_bytes", "verify_ipv6_fc00_private"),
            Self::new("ipaddr_bytes", "verify_ipv6_loopback"),
            Self::new("ipaddr_bytes", "verify_ipv6_multicast"),
            Self::new("ipaddr_bytes", "verify_ipv6_outside_fc00_not_private"),
            Self::new("ipaddr_bytes", "verify_ipv6_private_roundtrip"),
            Self::new("ipaddr_bytes", "verify_ipv6_public_construction"),
            Self::new("ipaddr_bytes", "verify_ipv6_roundtrip"),
            Self::new("ipaddr_bytes", "verify_ipv6_unspecified"),
            Self::new("macaddr", "verify_broadcast_is_multicast"),
            Self::new("macaddr", "verify_local_detection"),
            Self::new("macaddr", "verify_local_rejects_universal"),
            Self::new("macaddr", "verify_macaddr_roundtrip"),
            Self::new("macaddr", "verify_multicast_detection"),
            Self::new("macaddr", "verify_multicast_local"),
            Self::new("macaddr", "verify_multicast_rejects_unicast"),
            Self::new("macaddr", "verify_multicast_roundtrip"),
            Self::new("macaddr", "verify_multicast_universal"),
            Self::new("macaddr", "verify_null_is_unicast"),
            Self::new("macaddr", "verify_unicast_detection"),
            Self::new("macaddr", "verify_unicast_local"),
            Self::new("macaddr", "verify_unicast_rejects_multicast"),
            Self::new("macaddr", "verify_unicast_roundtrip"),
            Self::new("macaddr", "verify_unicast_universal"),
            Self::new("macaddr", "verify_universal_detection"),
            Self::new("macaddr", "verify_universal_rejects_local"),
            Self::new("mechanisms", "verify_regex_match_basic"),
            Self::new("mechanisms", "verify_regex_match_complex"),
            Self::new("mechanisms", "verify_regex_no_match"),
            Self::new("mechanisms", "verify_regex_construction"),
            Self::new("mechanisms", "verify_regex_pattern_validation"),
            Self::new("networks", "verify_ip_private"),
            Self::new("networks", "verify_ip_public"),
            Self::new("networks", "verify_ipv4_loopback"),
            Self::new("networks", "verify_ipv6_loopback"),
            Self::new("networks", "verify_uuid_v4"),
            Self::new("networks", "verify_uuid_non_nil"),
            Self::new("networks", "verify_uuid_nil_rejected"),
            Self::new("pathbytes", "verify_path_exists"),
            Self::new("pathbytes", "verify_path_readable"),
            Self::new("pathbytes", "verify_path_is_file"),
            Self::new("pathbytes", "verify_path_is_dir"),
            Self::new("pathbytes", "verify_path_with_current_dir"),
            Self::new("pathbytes", "verify_path_with_parent"),
            Self::new("pathbytes", "verify_path_with_extension"),
            Self::new("pathbytes", "verify_path_without_extension"),
            Self::new("pathbytes", "verify_path_root"),
            Self::new("pathbytes", "verify_path_components"),
            Self::new("pathbytes", "verify_path_join"),
            Self::new("pathbytes", "verify_path_normalization"),
            Self::new("pathbytes", "verify_path_symlink"),
            Self::new("pathbytes", "verify_path_absolute"),
            Self::new("regexbytes", "verify_regex_match_literal"),
            Self::new("regexbytes", "verify_regex_match_pattern"),
            Self::new("regexbytes", "verify_regex_match_unicode"),
            Self::new("regexbytes", "verify_regex_match_empty"),
            Self::new("regexbytes", "verify_regex_no_match"),
            Self::new("regexbytes", "verify_regex_captures"),
            Self::new("regexbytes", "verify_regex_split"),
            Self::new("regexbytes", "verify_regex_replace"),
            Self::new("regexbytes", "verify_regex_invalid"),
            Self::new("regexbytes", "verify_regex_construction"),
            Self::new("regexbytes", "verify_regex_clone"),
            Self::new("regexbytes", "verify_regex_pattern_getter"),
            Self::new("regexbytes", "verify_regex_match_bytes_empty"),
            Self::new("regexbytes", "verify_regex_match_bytes_pattern"),
            Self::new("regexbytes", "verify_regex_match_bytes_literal"),
            Self::new("regexbytes", "verify_regex_match_bytes_invalid_utf8"),
            Self::new("regexbytes", "verify_regex_match_bytes_no_match"),
            Self::new("regexbytes", "verify_regex_match_bytes_captures"),
            Self::new("regexbytes", "verify_regex_match_bytes_split"),
            Self::new("regexbytes", "verify_regex_match_bytes_replace"),
            Self::new("regexbytes", "verify_regex_match_bytes_construction"),
            Self::new("regexes", "verify_regex_match_basic"),
            Self::new("regexes", "verify_regex_match_complex"),
            Self::new("regexes", "verify_regex_no_match"),
            Self::new("regexes", "verify_regex_construction"),
            Self::new("regexes", "verify_regex_pattern_validation"),
            Self::new("regexes", "verify_regex_invalid"),
            Self::new("regexes", "verify_regex_clone"),
            Self::new("socketaddr", "verify_socketaddr_v4_construction"),
            Self::new("socketaddr", "verify_socketaddr_v6_construction"),
            Self::new("socketaddr", "verify_socketaddr_v4_port_non_zero"),
            Self::new("socketaddr", "verify_socketaddr_v6_port_non_zero"),
            Self::new("socketaddr", "verify_socketaddr_v4_port_zero_rejected"),
            Self::new("socketaddr", "verify_socketaddr_v6_port_zero_rejected"),
            Self::new("socketaddr", "verify_socketaddr_v4_private"),
            Self::new("socketaddr", "verify_socketaddr_v6_private"),
            Self::new("socketaddr", "verify_socketaddr_v4_public"),
            Self::new("socketaddr", "verify_socketaddr_v6_public"),
            Self::new("socketaddr", "verify_socketaddr_v4_loopback"),
            Self::new("socketaddr", "verify_socketaddr_v6_loopback"),
            Self::new("socketaddr", "verify_socketaddr_v4_multicast"),
            Self::new("socketaddr", "verify_socketaddr_v6_multicast"),
            Self::new("socketaddr", "verify_socketaddr_v4_unspecified"),
            Self::new("socketaddr", "verify_socketaddr_v6_unspecified"),
            Self::new("socketaddr", "verify_socketaddr_v4_roundtrip"),
            Self::new("socketaddr", "verify_socketaddr_v6_roundtrip"),
            Self::new("socketaddr", "verify_socketaddr_any"),
            Self::new("strings", "verify_string_non_empty"),
            Self::new("urlbytes", "verify_url_construction_http"),
            Self::new("urlbytes", "verify_url_construction_https"),
            Self::new("urlbytes", "verify_url_construction_ftp"),
            Self::new("urlbytes", "verify_url_construction_file"),
            Self::new("urlbytes", "verify_url_scheme_http"),
            Self::new("urlbytes", "verify_url_scheme_https"),
            Self::new("urlbytes", "verify_url_scheme_ftp"),
            Self::new("urlbytes", "verify_url_scheme_file"),
            Self::new("urlbytes", "verify_url_authority"),
            Self::new("urlbytes", "verify_url_path"),
            Self::new("urlbytes", "verify_url_query"),
            Self::new("urlbytes", "verify_url_fragment"),
            Self::new("urlbytes", "verify_url_port"),
            Self::new("urlbytes", "verify_url_without_port"),
            Self::new("urlbytes", "verify_url_relative"),
            Self::new("urlbytes", "verify_url_join"),
            Self::new("urlbytes", "verify_url_invalid_scheme"),
            Self::new("urlbytes", "verify_url_invalid_authority"),
            Self::new("urlbytes", "verify_url_empty"),
            Self::new("urlbytes", "verify_url_roundtrip"),
            Self::new("urls", "verify_url_http_construction"),
            Self::new("urls", "verify_url_https_construction"),
            Self::new("urls", "verify_url_ftp_construction"),
            Self::new("urls", "verify_url_scheme_validation"),
            Self::new("urls", "verify_url_invalid_scheme"),
            Self::new("urls", "verify_url_clone"),
            Self::new("urls", "verify_url_equality"),
            Self::new("uuid_bytes", "verify_uuid_v4_version"),
            Self::new("uuid_bytes", "verify_uuid_v4_variant"),
            Self::new("uuid_bytes", "verify_uuid_v4_non_nil"),
            Self::new("uuid_bytes", "verify_uuid_v4_construction"),
            Self::new("uuid_bytes", "verify_uuid_v4_roundtrip"),
            Self::new("uuid_bytes", "verify_uuid_non_nil_accepts_v4"),
            Self::new("uuid_bytes", "verify_uuid_non_nil_rejects_nil"),
            Self::new("uuid_bytes", "verify_uuid_nil_detection"),
            Self::new("uuid_bytes", "verify_uuid_wrong_version_rejected"),
            Self::new("uuid_bytes", "verify_uuid_invalid_variant_rejected"),
            Self::new("uuid_bytes", "verify_uuid_bytes_equality"),
            Self::new("uuid_bytes", "verify_uuid_bytes_clone"),
            Self::new("uuid_bytes", "verify_uuid_bytes_roundtrip"),
            Self::new("uuid_bytes", "verify_uuid_bytes_as_bytes"),
        ]
    }

    /// Run this proof harness with cargo kani.
    #[tracing::instrument(skip(self), fields(module = %self.module, harness = %self.name))]
    pub fn run(&self, timeout_secs: u64) -> Result<ProofResult> {
        tracing::info!("Running Kani proof");
        let start = Instant::now();
        let timestamp = Utc::now();

        let output = Command::new("timeout")
            .args([
                &format!("{}s", timeout_secs),
                "cargo",
                "kani",
                "--lib",
                "--features",
                "verify-kani,serde_json,chrono,jiff,time,uuid,url,regex",
                "--harness",
                &self.name,
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .context("Failed to execute cargo kani")?;

        let duration = start.elapsed();
        
        // Check if timeout killed the process (exit code 124)
        let status = if output.status.code() == Some(124) {
            tracing::warn!(duration_secs = duration.as_secs(), "Proof timed out");
            ProofStatus::Timeout
        } else if output.status.success() {
            tracing::info!(duration_secs = duration.as_secs(), "Proof succeeded");
            ProofStatus::Success
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            tracing::warn!(
                duration_secs = duration.as_secs(),
                error = %stderr.lines().next().unwrap_or("Unknown error"),
                "Proof failed"
            );
            ProofStatus::Failed
        };

        let error_message = match status {
            ProofStatus::Timeout => Some("Timeout exceeded".to_string()),
            ProofStatus::Failed => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                stderr
                    .lines()
                    .find(|line| line.contains("FAILED") || line.contains("error"))
                    .map(|s| s.trim().to_string())
            }
            _ => None,
        };

        Ok(ProofResult {
            module: self.module.clone(),
            harness_name: self.name.clone(),
            status,
            duration_secs: duration.as_secs(),
            timestamp: timestamp.to_rfc3339(),
            error_message,
        })
    }
}

/// Proof execution result.
#[derive(Debug, Clone, Getters, Serialize, Deserialize)]
pub struct ProofResult {
    /// Module name
    #[serde(rename = "Module")]
    module: String,
    /// Harness name
    #[serde(rename = "Harness")]
    harness_name: String,
    /// Execution status
    #[serde(rename = "Status")]
    status: ProofStatus,
    /// Duration in seconds
    #[serde(rename = "Time_Seconds")]
    duration_secs: u64,
    /// ISO 8601 timestamp
    #[serde(rename = "Timestamp")]
    timestamp: String,
    /// Error message if failed
    #[serde(rename = "Error_Message")]
    error_message: Option<String>,
}

/// Proof execution status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProofStatus {
    /// Proof succeeded
    #[serde(rename = "SUCCESS")]
    Success,
    /// Proof failed
    #[serde(rename = "FAILED")]
    Failed,
    /// Proof timed out
    #[serde(rename = "TIMEOUT")]
    Timeout,
}

/// Summary statistics for a verification run.
#[derive(Debug, Clone, Default, Getters)]
pub struct Summary {
    /// Total proofs run
    total: usize,
    /// Number of passed proofs
    passed: usize,
    /// Number of failed proofs
    failed: usize,
    /// Number of skipped proofs (resume mode)
    skipped: usize,
}

impl Summary {
    fn update(&mut self, result: &ProofResult) {
        self.total += 1;
        match result.status {
            ProofStatus::Success => self.passed += 1,
            ProofStatus::Failed | ProofStatus::Timeout => self.failed += 1,
        }
    }
}

/// Handle CLI verify commands.
#[tracing::instrument(skip(action))]
pub fn handle(action: &VerifyAction) -> Result<()> {
    tracing::debug!(action = ?action, "Handling verify command");
    
    match action {
        VerifyAction::List => list_harnesses(),
        VerifyAction::Run {
            output,
            timeout,
            resume,
        } => run_all(output, *timeout, *resume),
        VerifyAction::Summary { file } => show_summary(file),
        VerifyAction::Failed { file } => show_failed(file),
    }
}

/// List all proof harnesses.
#[tracing::instrument]
fn list_harnesses() -> Result<()> {
    tracing::info!("Listing proof harnesses");
    
    for harness in ProofHarness::all() {
        println!("{},{}", harness.module(), harness.name());
    }
    
    tracing::info!(count = ProofHarness::all().len(), "Listed harnesses");
    Ok(())
}

/// Run all proofs and save results to CSV.
#[tracing::instrument]
fn run_all(output: &Path, timeout: u64, resume: bool) -> Result<()> {
    tracing::info!(
        output = %output.display(),
        timeout,
        resume,
        "Running all proofs"
    );

    let mut writer = Writer::from_path(output).context("Failed to create CSV file")?;
    let harnesses = ProofHarness::all();
    let mut summary = Summary::default();

    println!("🔬 Kani Verification Tracking");
    println!("==============================");
    println!("Total harnesses: {}", harnesses.len());
    println!("CSV output: {}", output.display());
    println!("Timeout per test: {}s", timeout);
    println!("Resume mode: {}", resume);
    println!();

    for (idx, harness) in harnesses.iter().enumerate() {
        let current = idx + 1;
        let total = harnesses.len();

        // Check if already passed (resume mode)
        if resume && already_passed(output, harness)? {
            println!("[{}/{}] ⏭️  Skipped (cached): {}::{}", current, total, harness.module(), harness.name());
            summary.skipped += 1;
            continue;
        }

        println!(
            "[{}/{}] 🔬 Running: {}::{}",
            current,
            total,
            harness.module(),
            harness.name()
        );
        let _ = std::io::stdout().flush();

        let result = harness.run(timeout)?;
        writer.serialize(&result).context("Failed to write CSV row")?;
        writer.flush().context("Failed to flush CSV")?;

        match result.status {
            ProofStatus::Success => {
                println!("[{}/{}] ✅ Passed", current, total);
            }
            ProofStatus::Failed => {
                println!("[{}/{}] ❌ Failed", current, total);
                if let Some(err) = &result.error_message {
                    println!("    Error: {}", err);
                }
            }
            ProofStatus::Timeout => {
                println!("[{}/{}] ⏱️  Timeout", current, total);
            }
        }

        summary.update(&result);
        println!();
    }

    println!("==============================");
    println!("✅ Passed: {}", summary.passed);
    println!("❌ Failed: {}", summary.failed);
    println!("⏭️  Skipped: {}", summary.skipped);
    println!();
    println!("📊 Results saved to: {}", output.display());

    if summary.failed > 0 {
        anyhow::bail!("{} proofs failed", summary.failed);
    }

    Ok(())
}

/// Check if a harness already passed in the CSV.
#[tracing::instrument]
fn already_passed(csv_path: &Path, harness: &ProofHarness) -> Result<bool> {
    if !csv_path.exists() {
        return Ok(false);
    }

    let mut reader = Reader::from_path(csv_path).context("Failed to open CSV file")?;

    for result in reader.deserialize::<ProofResult>() {
        let record = result.context("Failed to parse CSV record")?;
        if &record.module == harness.module() && &record.harness_name == harness.name()
            && record.status == ProofStatus::Success
        {
            return Ok(true);
        }
    }

    Ok(false)
}

/// Show summary statistics from CSV.
#[tracing::instrument]
fn show_summary(file: &Path) -> Result<()> {
    tracing::info!(file = %file.display(), "Showing summary");

    if !file.exists() {
        anyhow::bail!("❌ No results found: {}", file.display());
    }

    let mut reader = Reader::from_path(file).context("Failed to open CSV file")?;
    let mut total = 0;
    let mut passed = 0;
    let mut failed = 0;

    for result in reader.deserialize::<ProofResult>() {
        let record = result.context("Failed to parse CSV record")?;
        total += 1;
        match record.status {
            ProofStatus::Success => passed += 1,
            ProofStatus::Failed | ProofStatus::Timeout => failed += 1,
        }
    }

    println!("📊 Kani Verification Summary");
    println!("=============================");
    println!("Source: {}", file.display());
    println!();
    println!("Total runs: {}", total);
    println!("✅ Passed: {}", passed);
    println!("❌ Failed: {}", failed);

    if total > 0 {
        let pass_rate = (passed as f64 / total as f64) * 100.0;
        println!();
        println!("Pass rate: {:.1}%", pass_rate);
    }

    Ok(())
}

/// Show failed tests from CSV.
#[tracing::instrument]
fn show_failed(file: &Path) -> Result<()> {
    tracing::info!(file = %file.display(), "Showing failures");

    if !file.exists() {
        anyhow::bail!("❌ No results found: {}", file.display());
    }

    let mut reader = Reader::from_path(file).context("Failed to open CSV file")?;
    let mut failures = Vec::new();

    for result in reader.deserialize::<ProofResult>() {
        let record = result.context("Failed to parse CSV record")?;
        if record.status != ProofStatus::Success {
            failures.push(record);
        }
    }

    if failures.is_empty() {
        println!("✅ No failures! All tests passed.");
        return Ok(());
    }

    println!("❌ Failed Kani Verifications");
    println!("=============================");
    println!();

    for failure in &failures {
        println!("Module: {}", failure.module);
        println!("Harness: {}", failure.harness_name);
        println!("Status: {:?}", failure.status);
        println!("Time: {}s", failure.duration_secs);
        if let Some(err) = &failure.error_message {
            println!("Error: {}", err);
        }
        println!();
    }

    println!("Total failures: {}", failures.len());

    Ok(())
}
