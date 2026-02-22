//! Creusot proof orchestration and tracking.
//!
//! This module provides functionality to track Creusot proof compilation status.
//! Unlike Kani/Verus which run actual proofs, Creusot proofs are marked #[trusted]
//! and verification happens at compile time. This runner tracks module compilation.

use anyhow::{Context, Result};
use chrono::Utc;
use csv::{Reader, Writer};
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Instant;

/// A Creusot proof module identifier.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, Default, Serialize, Deserialize,
)]
pub struct CreusotModule {
    /// Module name (e.g., "bools", "integers")
    name: String,
    /// Optional feature requirement (e.g., Some("uuid"), None for core)
    feature: Option<String>,
    /// Whether this is unix-only
    unix_only: bool,
}

impl CreusotModule {
    /// Create a new module identifier.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            feature: None,
            unix_only: false,
        }
    }

    /// Create a feature-gated module.
    pub fn with_feature(name: impl Into<String>, feature: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            feature: Some(feature.into()),
            unix_only: false,
        }
    }

    /// Create a unix-only module.
    pub fn unix_only(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            feature: None,
            unix_only: true,
        }
    }

    /// All available Creusot proof modules.
    pub fn all() -> Vec<Self> {
        vec![
            // Core contract modules (10)
            Self::new("bools"),
            Self::new("chars"),
            Self::new("collections"),
            Self::new("durations"),
            Self::new("floats"),
            Self::new("integers"),
            Self::new("networks"),
            Self::new("paths"),
            Self::new("strings"),
            Self::new("tuples"),
            // Trenchcoat wrapper modules (6 core + 1 unix)
            Self::new("ipaddr_bytes"),
            Self::new("macaddr"),
            Self::new("mechanisms"),
            Self::new("socketaddr"),
            Self::new("utf8"),
            Self::unix_only("pathbytes"),
            // Feature-gated contract modules (7)
            Self::with_feature("uuids", "uuid"),
            Self::with_feature("uuid_bytes", "uuid"),
            Self::with_feature("values", "serde_json"),
            Self::with_feature("urls", "url"),
            Self::with_feature("urlbytes", "url"),
            Self::with_feature("regexes", "regex"),
            Self::with_feature("regexbytes", "regex"),
            Self::with_feature("datetimes_chrono", "chrono"),
            Self::with_feature("datetimes_time", "time"),
            Self::with_feature("datetimes_jiff", "jiff"),
        ]
    }

    /// Check if this module should be compiled on current platform.
    pub fn is_available(&self) -> bool {
        if self.unix_only && !cfg!(unix) {
            return false;
        }
        true
    }
}

impl std::fmt::Display for CreusotModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(feature) = &self.feature {
            write!(f, "{} (feature: {})", self.name, feature)
        } else if self.unix_only {
            write!(f, "{} (unix)", self.name)
        } else {
            write!(f, "{}", self.name)
        }
    }
}

/// Status of a Creusot module compilation check.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompilationStatus {
    /// Module compiled successfully
    Success,
    /// Module compilation failed
    Failed,
    /// Module skipped (platform or feature unavailable)
    Skipped,
}

/// Result of running a Creusot module compilation check.
#[derive(Debug, Clone, Getters, Serialize, Deserialize)]
pub struct CreusotModuleResult {
    /// Module name
    module: String,
    /// Compilation status
    status: CompilationStatus,
    /// Time taken in seconds
    time_seconds: u64,
    /// Timestamp
    timestamp: chrono::DateTime<Utc>,
    /// Error message if failed
    error_message: String,
}

impl CreusotModuleResult {
    /// Create a new module result.
    pub fn new(
        module: impl Into<String>,
        status: CompilationStatus,
        time_seconds: u64,
        error_message: impl Into<String>,
    ) -> Self {
        Self {
            module: module.into(),
            status,
            time_seconds,
            timestamp: Utc::now(),
            error_message: error_message.into(),
        }
    }

    /// Check if compilation succeeded.
    pub fn is_success(&self) -> bool {
        self.status == CompilationStatus::Success
    }
}

/// Run a Creusot module compilation check.
pub fn run_creusot_module(module: &CreusotModule) -> Result<CreusotModuleResult> {
    let start = Instant::now();

    // Check if module is available on this platform
    if !module.is_available() {
        return Ok(CreusotModuleResult::new(
            module.name(),
            CompilationStatus::Skipped,
            0,
            "Module not available on this platform",
        ));
    }

    // Build cargo check command for elicitation_creusot crate
    let mut cmd = Command::new("cargo");
    cmd.arg("check")
        .arg("-p")
        .arg("elicitation_creusot")
        .arg("--lib");

    // Add feature flag if needed
    if let Some(feature) = module.feature() {
        cmd.arg("--features").arg(feature);
    }

    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

    let output = cmd
        .output()
        .with_context(|| format!("Failed to execute cargo check for module {}", module.name()))?;

    let elapsed = start.elapsed().as_secs();
    let stderr = String::from_utf8_lossy(&output.stderr);

    let (status, error_message) = if output.status.success() {
        (CompilationStatus::Success, String::new())
    } else {
        (
            CompilationStatus::Failed,
            format!("Compilation failed:\n{}", stderr),
        )
    };

    Ok(CreusotModuleResult::new(
        module.name(),
        status,
        elapsed,
        error_message,
    ))
}

/// Run all Creusot module compilation checks and track results.
pub fn run_all_modules(output_csv: &Path, resume: bool) -> Result<CreusotSummary> {
    println!("🔬 Running Creusot module compilation checks...");
    println!("   Output: {}", output_csv.display());
    println!();

    // Load existing results if resuming
    let mut completed_modules = std::collections::HashSet::new();
    if resume && output_csv.exists() {
        println!("📂 Loading existing results...");
        let mut reader = Reader::from_path(output_csv)
            .with_context(|| format!("Failed to read CSV: {}", output_csv.display()))?;
        for result in reader.deserialize::<CreusotModuleResult>() {
            if let Ok(result) = result {
                if result.is_success() {
                    completed_modules.insert(result.module().clone());
                }
            }
        }
        println!("   Found {} completed modules", completed_modules.len());
        println!();
    }

    // Create CSV writer
    let mut writer = Writer::from_path(output_csv)
        .with_context(|| format!("Failed to create CSV: {}", output_csv.display()))?;

    let modules = CreusotModule::all();
    let total = modules.len();
    let mut passed = 0;
    let mut failed = 0;
    let mut skipped = 0;

    for (i, module) in modules.iter().enumerate() {
        if completed_modules.contains(module.name()) {
            println!(
                "[{:2}/{:2}] ⏭️  Skipping {} (already passed)",
                i + 1,
                total,
                module
            );
            skipped += 1;
            continue;
        }

        print!("[{:2}/{:2}] 🔬 Checking {}... ", i + 1, total, module);
        std::io::stdout().flush().ok();

        match run_creusot_module(module) {
            Ok(result) => {
                match result.status() {
                    CompilationStatus::Success => {
                        println!("✅ PASS ({}s)", result.time_seconds());
                        passed += 1;
                    }
                    CompilationStatus::Failed => {
                        println!("❌ FAIL ({}s)", result.time_seconds());
                        if !result.error_message().is_empty() {
                            println!("   Error: {}", result.error_message());
                        }
                        failed += 1;
                    }
                    CompilationStatus::Skipped => {
                        println!("⏭️  SKIPPED");
                        skipped += 1;
                    }
                }

                writer
                    .serialize(&result)
                    .context("Failed to write result to CSV")?;
                writer.flush().context("Failed to flush CSV writer")?;
            }
            Err(e) => {
                println!("🔥 ERROR");
                println!("   {:#}", e);
                failed += 1;
            }
        }
    }

    writer.flush().context("Failed to flush CSV")?;

    println!();
    println!("📊 Compilation Summary:");
    println!("   Total:   {}", total);
    println!("   Passed:  {} ✅", passed);
    println!("   Failed:  {} ❌", failed);
    println!("   Skipped: {} ⏭️", skipped);

    Ok(CreusotSummary {
        total,
        passed,
        failed,
        skipped,
    })
}

/// Summary statistics for Creusot module compilation.
#[derive(Debug, Clone)]
pub struct CreusotSummary {
    /// Total number of modules
    pub total: usize,
    /// Number passed
    pub passed: usize,
    /// Number failed
    pub failed: usize,
    /// Number skipped
    pub skipped: usize,
}

impl CreusotSummary {
    /// Calculate success rate.
    pub fn success_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.passed as f64 / (self.total - self.skipped) as f64) * 100.0
        }
    }
}

/// Show summary statistics from CSV file.
pub fn show_summary(csv_path: &Path) -> Result<()> {
    let mut reader = Reader::from_path(csv_path)
        .with_context(|| format!("Failed to read CSV: {}", csv_path.display()))?;

    let mut total = 0;
    let mut passed = 0;
    let mut failed = 0;
    let mut skipped = 0;

    for result in reader.deserialize::<CreusotModuleResult>() {
        let result = result.context("Failed to parse CSV row")?;
        total += 1;
        match result.status() {
            CompilationStatus::Success => passed += 1,
            CompilationStatus::Failed => failed += 1,
            CompilationStatus::Skipped => skipped += 1,
        }
    }

    let summary = CreusotSummary {
        total,
        passed,
        failed,
        skipped,
    };

    println!("📊 Creusot Module Compilation Summary");
    println!("============================");
    println!();
    println!("  Total:   {}", summary.total);
    println!("  Passed:  {} ✅", summary.passed);
    println!("  Failed:  {} ❌", summary.failed);
    println!("  Skipped: {} ⏭️", summary.skipped);
    println!();
    println!("  Success Rate: {:.1}%", summary.success_rate());

    Ok(())
}

/// List all Creusot modules.
pub fn list_modules() {
    let modules = CreusotModule::all();

    println!("📋 Available Creusot Modules ({} total):", modules.len());
    println!();

    // Group by category
    let core: Vec<_> = modules
        .iter()
        .filter(|m| m.feature().is_none() && !m.unix_only())
        .collect();
    let unix: Vec<_> = modules.iter().filter(|m| m.unix_only()).collect();
    let featured: Vec<_> = modules.iter().filter(|m| m.feature().is_some()).collect();

    if !core.is_empty() {
        println!("  Core modules ({}):", core.len());
        for module in core {
            println!("    - {}", module.name());
        }
        println!();
    }

    if !unix.is_empty() {
        println!("  Unix-only modules ({}):", unix.len());
        for module in unix {
            println!("    - {}", module.name());
        }
        println!();
    }

    if !featured.is_empty() {
        println!("  Feature-gated modules ({}):", featured.len());
        for module in featured {
            println!(
                "    - {} (requires: {})",
                module.name(),
                module.feature().as_ref().unwrap()
            );
        }
    }
}
