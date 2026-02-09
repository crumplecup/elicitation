//! Kani proof orchestration for rand integration.

use chrono::Utc;
use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Instant;

/// A Kani proof harness identifier for rand proofs.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub struct ProofHarness {
    /// Module name (e.g., "kani_proofs")
    pub module: String,
    /// Harness function name (e.g., "verify_random_generator_u8_construction")
    pub name: String,
}

impl ProofHarness {
    /// Create a new proof harness identifier.
    pub fn new(module: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            module: module.into(),
            name: name.into(),
        }
    }

    /// All available proof harnesses for rand integration.
    ///
    /// Note: Limited to logic-only proofs due to rand's use of inline assembly.
    /// WeightedGenerator and RNG construction cannot be verified by Kani.
    pub fn all() -> Vec<Self> {
        vec![
            // Uniform bounds logic proofs (inline asm prevents construction testing)
            Self::new("kani_proofs", "verify_uniform_bounds_ordering"),
            Self::new("kani_proofs", "verify_uniform_f64_finite"),
            Self::new("kani_proofs", "verify_uniform_i32_negative_range"),
        ]
    }
}

/// A single proof execution result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofResult {
    /// Module name
    pub module: String,
    /// Harness name
    pub harness: String,
    /// Success/failure status
    pub status: String,
    /// Duration in seconds
    pub duration_secs: f64,
    /// Timestamp of execution
    pub timestamp: String,
}

impl ProofResult {
    /// Create a new successful proof result.
    pub fn success(harness: &ProofHarness, duration_secs: f64) -> Self {
        Self {
            module: harness.module.clone(),
            harness: harness.name.clone(),
            status: "PASS".to_string(),
            duration_secs,
            timestamp: Utc::now().to_rfc3339(),
        }
    }

    /// Create a new failed proof result.
    pub fn failure(harness: &ProofHarness, duration_secs: f64) -> Self {
        Self {
            module: harness.module.clone(),
            harness: harness.name.clone(),
            status: "FAIL".to_string(),
            duration_secs,
            timestamp: Utc::now().to_rfc3339(),
        }
    }
}

/// Run a single Kani proof and return the result.
pub fn run_proof(harness: &ProofHarness) -> std::io::Result<ProofResult> {
    println!("Running proof: {}::{}", harness.module, harness.name);

    let start = Instant::now();

    let output = Command::new("cargo")
        .args(["kani", "--harness", &harness.name, "-p", "elicitation_rand"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    let duration = start.elapsed();
    let duration_secs = duration.as_secs_f64();

    let result = if output.status.success() {
        ProofResult::success(harness, duration_secs)
    } else {
        eprintln!("Proof failed: {}::{}", harness.module, harness.name);
        eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        ProofResult::failure(harness, duration_secs)
    };

    println!("  {} in {:.2}s", result.status, result.duration_secs);

    Ok(result)
}

/// Run all proofs and write results to CSV.
pub fn run_all_proofs(output_path: &Path) -> std::io::Result<Vec<ProofResult>> {
    let harnesses = ProofHarness::all();
    let mut results = Vec::new();

    println!("Running {} rand proofs...", harnesses.len());

    for harness in &harnesses {
        let result = run_proof(harness)?;
        results.push(result);
    }

    // Write results to CSV
    write_results_csv(&results, output_path)?;

    // Print summary
    print_summary(&results);

    Ok(results)
}

/// Write proof results to CSV file.
fn write_results_csv(results: &[ProofResult], output_path: &Path) -> std::io::Result<()> {
    let mut writer = Writer::from_path(output_path)?;

    for result in results {
        writer
            .serialize(result)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    }

    writer.flush()?;
    println!("\nResults written to: {}", output_path.display());

    Ok(())
}

/// Print summary statistics.
fn print_summary(results: &[ProofResult]) {
    let total = results.len();
    let passed = results.iter().filter(|r| r.status == "PASS").count();
    let failed = total - passed;
    let total_time: f64 = results.iter().map(|r| r.duration_secs).sum();

    println!("\n{}", "=".repeat(60));
    println!("RAND PROOF SUMMARY");
    println!("{}", "=".repeat(60));
    println!("Total proofs:  {}", total);
    println!(
        "Passed:        {} ({:.1}%)",
        passed,
        (passed as f64 / total as f64) * 100.0
    );
    println!("Failed:        {}", failed);
    println!("Total time:    {:.2}s", total_time);
    println!("Average time:  {:.2}s", total_time / total as f64);
    println!("{}", "=".repeat(60));

    if failed > 0 {
        println!("\nFailed proofs:");
        for result in results.iter().filter(|r| r.status == "FAIL") {
            println!("  - {}::{}", result.module, result.harness);
        }
    }
}

/// Read existing results from CSV.
pub fn read_results_csv(input_path: &Path) -> std::io::Result<Vec<ProofResult>> {
    let mut reader = Reader::from_path(input_path)?;

    let mut results = Vec::new();
    for record in reader.deserialize::<ProofResult>() {
        let result = record.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        results.push(result);
    }

    Ok(results)
}
