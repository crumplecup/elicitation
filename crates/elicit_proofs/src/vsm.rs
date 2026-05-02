//! VSM Kani proof harness runner.
//!
//! Reads the generated `manifest.json` (embedded at compile time), runs each
//! harness individually with a per-harness timeout, and tracks results in a CSV.

use anyhow::{Context, Result};
use chrono::Utc;
use clap::Subcommand;
use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Instant;

/// Manifest JSON embedded at compile time — always in sync with build.rs output.
const MANIFEST: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/kani/generated/manifest.json"
));

// ─────────────────────────────────────────────────────────────────────────────
//  CLI subcommands
// ─────────────────────────────────────────────────────────────────────────────

/// VSM proof subcommands.
#[derive(Debug, Clone, Subcommand)]
pub enum VsmAction {
    /// List all VSM harnesses from the manifest.
    List,

    /// Run VSM harnesses with per-harness timeout and CSV tracking.
    Run {
        /// CSV output file.
        #[arg(short, long, default_value = "vsm_kani_results.csv")]
        csv: PathBuf,

        /// Timeout per harness in seconds.
        #[arg(short, long, default_value_t = 300)]
        timeout: u64,

        /// Skip harnesses already recorded as SUCCESS in the CSV.
        #[arg(long)]
        resume: bool,

        /// Only run harnesses whose module or name contains this substring.
        #[arg(long)]
        filter: Option<String>,
    },

    /// Show pass/fail/timeout summary from a results CSV.
    Summary {
        /// CSV results file.
        #[arg(short, long, default_value = "vsm_kani_results.csv")]
        csv: PathBuf,
    },

    /// Show all failing harnesses from a results CSV.
    Failed {
        /// CSV results file.
        #[arg(short, long, default_value = "vsm_kani_results.csv")]
        csv: PathBuf,
    },
}

// ─────────────────────────────────────────────────────────────────────────────
//  Data types
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
struct ManifestEntry {
    module: String,
    name: String,
}

#[derive(Debug, Clone)]
struct VsmHarness {
    module: String,
    name: String,
}

/// Proof execution status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VsmStatus {
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "TIMEOUT")]
    Timeout,
}

impl std::fmt::Display for VsmStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => write!(f, "SUCCESS"),
            Self::Failed => write!(f, "FAILED"),
            Self::Timeout => write!(f, "TIMEOUT"),
        }
    }
}

/// One row in the results CSV.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct VsmResult {
    #[serde(rename = "Module")]
    module: String,
    #[serde(rename = "Harness")]
    harness_name: String,
    #[serde(rename = "Status")]
    status: VsmStatus,
    #[serde(rename = "Time_Seconds")]
    duration_secs: u64,
    #[serde(rename = "Timestamp")]
    timestamp: String,
    #[serde(rename = "Error_Message")]
    error_message: Option<String>,
}

// ─────────────────────────────────────────────────────────────────────────────
//  Public entry point
// ─────────────────────────────────────────────────────────────────────────────

/// Dispatch a `vsm` subcommand.
#[tracing::instrument(skip(action))]
pub fn handle(action: &VsmAction) -> Result<()> {
    match action {
        VsmAction::List => list(),
        VsmAction::Run {
            csv,
            timeout,
            resume,
            filter,
        } => run_all(csv, *timeout, *resume, filter.as_deref()),
        VsmAction::Summary { csv } => show_summary(csv),
        VsmAction::Failed { csv } => show_failed(csv),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
//  Internals
// ─────────────────────────────────────────────────────────────────────────────

fn load_harnesses() -> Result<Vec<VsmHarness>> {
    let entries: Vec<ManifestEntry> =
        serde_json::from_str(MANIFEST).context("Failed to parse embedded VSM manifest")?;
    Ok(entries
        .into_iter()
        .map(|e| VsmHarness {
            module: e.module,
            name: e.name,
        })
        .collect())
}

fn list() -> Result<()> {
    let harnesses = load_harnesses()?;
    for h in &harnesses {
        println!("{}::{}", h.module, h.name);
    }
    println!("\nTotal: {}", harnesses.len());
    Ok(())
}

fn run_all(csv: &Path, timeout: u64, resume: bool, filter: Option<&str>) -> Result<()> {
    let all = load_harnesses()?;
    let harnesses: Vec<_> = match filter {
        Some(pat) => all
            .into_iter()
            .filter(|h| h.module.contains(pat) || h.name.contains(pat))
            .collect(),
        None => all,
    };

    let mut writer = Writer::from_path(csv).context("Failed to create CSV file")?;

    let total = harnesses.len();
    println!("🔬 VSM Kani Verification");
    println!("========================");
    println!("Total harnesses : {total}");
    println!("CSV output      : {}", csv.display());
    println!("Timeout/harness : {timeout}s");
    println!("Resume mode     : {resume}");
    println!();

    let mut passed = 0usize;
    let mut failed = 0usize;
    let mut skipped = 0usize;

    for (idx, h) in harnesses.iter().enumerate() {
        let current = idx + 1;

        if resume && already_passed(csv, &h.module, &h.name)? {
            println!("[{current}/{total}] ⏭️  {}", h.name);
            skipped += 1;
            continue;
        }

        print!("[{current}/{total}] 🔬 {}  ", h.name);
        let _ = std::io::stdout().flush();

        let result = run_harness(h, timeout);

        match result.status {
            VsmStatus::Success => {
                println!("✅");
                passed += 1;
            }
            VsmStatus::Failed => {
                println!("❌");
                failed += 1;
            }
            VsmStatus::Timeout => {
                println!("⏱️  TIMEOUT");
                failed += 1;
            }
        }

        writer
            .serialize(&result)
            .context("Failed to write CSV row")?;
        writer.flush().context("Failed to flush CSV")?;
    }

    println!();
    println!("Results: {passed} passed, {failed} failed, {skipped} skipped / {total} total");
    Ok(())
}

fn run_harness(h: &VsmHarness, timeout_secs: u64) -> VsmResult {
    let start = Instant::now();
    let timestamp = Utc::now().to_rfc3339();

    let output = Command::new("timeout")
        .args([
            &format!("{timeout_secs}s"),
            "cargo",
            "kani",
            "-p",
            "elicit_proofs",
            "--lib",
            "--features",
            "kani",
            "-Z",
            "function-contracts",
            "--harness",
            &h.name,
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();

    let duration = start.elapsed().as_secs();

    let (status, error_message) = match output {
        Err(e) => (VsmStatus::Failed, Some(e.to_string())),
        Ok(out) => {
            if out.status.code() == Some(124) {
                (VsmStatus::Timeout, Some("Timeout exceeded".to_string()))
            } else if out.status.success() {
                (VsmStatus::Success, None)
            } else {
                let stderr = String::from_utf8_lossy(&out.stderr);
                let msg = stderr
                    .lines()
                    .find(|l| l.contains("FAILED") || l.contains("error"))
                    .unwrap_or("Unknown error")
                    .trim()
                    .to_string();
                (VsmStatus::Failed, Some(msg))
            }
        }
    };

    VsmResult {
        module: h.module.clone(),
        harness_name: h.name.clone(),
        status,
        duration_secs: duration,
        timestamp,
        error_message,
    }
}

fn already_passed(csv: &Path, module: &str, name: &str) -> Result<bool> {
    if !csv.exists() {
        return Ok(false);
    }
    let mut reader = Reader::from_path(csv).context("Failed to read CSV")?;
    for record in reader.deserialize::<VsmResult>() {
        if let Ok(r) = record {
            if r.module == module && r.harness_name == name && r.status == VsmStatus::Success {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn show_summary(csv: &Path) -> Result<()> {
    if !csv.exists() {
        println!("No results file: {}", csv.display());
        return Ok(());
    }
    let mut reader = Reader::from_path(csv).context("Failed to read CSV")?;
    let (mut passed, mut failed, mut timeout) = (0usize, 0usize, 0usize);
    for record in reader.deserialize::<VsmResult>() {
        let r = record.context("Failed to read CSV record")?;
        match r.status {
            VsmStatus::Success => passed += 1,
            VsmStatus::Failed => failed += 1,
            VsmStatus::Timeout => timeout += 1,
        }
    }
    let total = passed + failed + timeout;
    println!("VSM Kani Summary");
    println!("================");
    println!("Total   : {total}");
    println!("✅ Pass  : {passed}");
    println!("❌ Fail  : {failed}");
    println!("⏱️  Timeout: {timeout}");
    Ok(())
}

fn show_failed(csv: &Path) -> Result<()> {
    if !csv.exists() {
        println!("No results file: {}", csv.display());
        return Ok(());
    }
    let mut reader = Reader::from_path(csv).context("Failed to read CSV")?;
    let mut count = 0usize;
    for record in reader.deserialize::<VsmResult>() {
        let r = record.context("Failed to read CSV record")?;
        if r.status != VsmStatus::Success {
            println!("[{}] {}::{}", r.status, r.module, r.harness_name);
            if let Some(msg) = &r.error_message {
                println!("     → {msg}");
            }
            count += 1;
        }
    }
    if count == 0 {
        println!("No failures.");
    }
    Ok(())
}
