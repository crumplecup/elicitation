//! `elicitation prove` — invoke proof backends from the current workspace.
//!
//! Configuration is read from `.env` in the current directory (or the first
//! `.env` found walking upward).  CLI flags override env values.
//!
//! ## Environment variables
//!
//! | Variable              | Default                                               | Used by        |
//! |-----------------------|-------------------------------------------------------|----------------|
//! | `PROVE_PACKAGE`       | *(none — required if no `-p` flag)*                  | all backends   |
//! | `KANI_PACKAGE`        | falls back to `PROVE_PACKAGE`                         | kani           |
//! | `KANI_FLAGS`          | `""` (`elicit_proofs` needs `"--lib --features kani -Z function-contracts -Z stubbing"`) | kani |
//! | `KANI_CSV`            | `kani_verification_results.csv`                       | kani           |
//! | `VERUS_PATH`          | `~/repos/verus/source/target-verus/release/verus`     | verus          |
//! | `VERUS_FILE`          | *(none — required for verus if no `--verus-file`)*   | verus          |
//! |                       | May be a **directory**: all `*.rs` files (excluding   | verus          |
//! |                       | `mod.rs`) are verified individually, matching the     | verus          |
//! |                       | `generate verus --out <dir>` output path.             | verus          |
//! |                       | In this repo, point Verus at `crates/elicitation_verus/src/lib.rs`, not `elicit_proofs`. | verus |
//! | `VERUS_FLAGS`         | `""`                                                  | verus          |
//! | `CREUSOT_PACKAGE`     | falls back to `PROVE_PACKAGE`                         | creusot        |
//! | `CREUSOT_FLAGS`       | `""`                                                  | creusot        |
//! | `CREUSOT_BIN_DIR`     | `~/.local/share/creusot/bin`                          | creusot        |
//! | `WHY3_CONFIG`         | `~/.config/creusot/why3.conf`                         | creusot        |
//! | `DUNE_DIR_LOCATIONS`  | `why3find:lib:~/.local/share/creusot/share/why3find`  | creusot        |
//! | `PROVE_LOG_DIR`       | `.` (current directory)                               | all backends   |

use anyhow::Context as _;
use std::{
    collections::HashSet,
    fs,
    io::{BufRead as _, Read, Write as _},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

use indicatif::{ProgressBar, ProgressStyle};

// ── Config ────────────────────────────────────────────────────────────────────

/// Resolved configuration for the `prove` command.
#[derive(Debug)]
pub struct ProveConfig {
    /// Run the Kani backend.
    pub run_kani: bool,
    /// Run the Verus backend.
    pub run_verus: bool,
    /// Run the Creusot backend.
    pub run_creusot: bool,

    /// `-p <package>` for Kani.
    pub kani_package: Option<String>,
    /// Extra flags passed verbatim to `cargo kani`.
    pub kani_flags: Vec<String>,
    /// Single harness to target instead of discovering all (Kani only).
    pub kani_harness: Option<String>,
    /// CSV file path for per-harness results (None = no CSV).
    pub kani_csv: Option<PathBuf>,
    /// Skip harnesses already recorded as PASS in the CSV.
    pub kani_resume: bool,

    /// Path to the `verus` binary.
    pub verus_path: PathBuf,
    /// Source file (or directory) for Verus.
    pub verus_file: Option<PathBuf>,
    /// Extra flags passed verbatim to the `verus` binary.
    pub verus_flags: Vec<String>,
    /// CSV file path for Verus results (None = no CSV).
    pub verus_csv: Option<PathBuf>,

    /// `-p <package>` for Creusot.
    pub creusot_package: Option<String>,
    /// Extra flags passed verbatim to `cargo creusot prove`.
    pub creusot_flags: Vec<String>,
    /// Directory prepended to `PATH` containing `why3find` etc.
    pub creusot_bin_dir: PathBuf,
    /// Path to the Why3 config file.
    pub why3_config: PathBuf,
    /// `DUNE_DIR_LOCATIONS` value for why3find.
    pub dune_dir_locations: String,
    /// CSV file path for Creusot results (None = no CSV).
    pub creusot_csv: Option<PathBuf>,
    /// Timeout for the entire `cargo creusot prove` invocation. 0 means unlimited.
    /// Defaults to 0 because creusot compiles the whole workspace before proving,
    /// so a short global timeout would fire during compilation on a cold cache.
    pub creusot_timeout: u64,

    /// Timeout in seconds per verification unit. 0 means unlimited.
    pub timeout: u64,

    /// Print the commands that would run without executing them.
    pub dry_run: bool,

    /// Directory for per-backend log files (prove_kani.log, prove_verus.log, prove_creusot.log).
    pub log_dir: PathBuf,
}

impl ProveConfig {
    /// Build from CLI overrides on top of values loaded from `.env`.
    #[tracing::instrument(skip(opts))]
    pub fn resolve(opts: &super::ProveOpts) -> anyhow::Result<Self> {
        // Load .env — walk up from cwd until found; silently skip if absent.
        // Use dotenv_override so explicit `.env` values take precedence over
        // empty/unset shell environment variables (e.g. KANI_FLAGS='').
        match dotenvy::dotenv_override() {
            Ok(path) => tracing::debug!(dotenv_path = %path.display(), "Loaded .env"),
            Err(e) => tracing::debug!(error = %e, "No .env loaded"),
        }

        let home = std::env::var("HOME").unwrap_or_default();
        let expand = |s: String| shellexpand::tilde(&s).to_string();

        // ── helpers ──────────────────────────────────────────────────────────
        let env_or = |key: &str, default: &str| -> String {
            std::env::var(key).unwrap_or_else(|_| default.to_string())
        };
        let env_opt = |key: &str| -> Option<String> { std::env::var(key).ok() };

        // ── package resolution ────────────────────────────────────────────────
        let prove_pkg = opts.package.clone().or_else(|| env_opt("PROVE_PACKAGE"));

        let kani_package = env_opt("KANI_PACKAGE").or_else(|| prove_pkg.clone());
        let creusot_package = env_opt("CREUSOT_PACKAGE").or_else(|| prove_pkg.clone());

        // ── Kani ─────────────────────────────────────────────────────────────
        let kani_flags: Vec<String> = env_or("KANI_FLAGS", "")
            .split_whitespace()
            .map(str::to_string)
            .filter(|s| !s.is_empty())
            .collect();
        tracing::debug!(kani_flags = ?kani_flags, "Resolved KANI_FLAGS");

        let kani_harness = opts.kani_harness.clone();

        // Derive per-backend CSV paths from the user-supplied stem (or env var).
        // --csv bare → Some("verification_results.csv"); not passed → None.
        // Each backend prefixes its name: kani_verification_results.csv, etc.
        let csv_stem: Option<PathBuf> = opts.csv.clone();
        let backend_csv = |prefix: &str, env_key: &str| -> Option<PathBuf> {
            if let Some(p) = csv_stem.as_ref() {
                let stem = p
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("verification_results");
                let ext = p.extension().and_then(|s| s.to_str()).unwrap_or("csv");
                Some(PathBuf::from(format!("{prefix}_{stem}.{ext}")))
            } else {
                env_opt(env_key).map(PathBuf::from)
            }
        };

        let kani_csv = backend_csv("kani", "KANI_CSV");
        let kani_resume = opts.resume;

        // ── Verus ─────────────────────────────────────────────────────────────
        let verus_path = opts
            .verus_path
            .clone()
            .or_else(|| env_opt("VERUS_PATH").map(|p| PathBuf::from(expand(p))))
            .unwrap_or_else(|| {
                PathBuf::from(expand(format!(
                    "{home}/repos/verus/source/target-verus/release/verus"
                )))
            });

        let verus_file = opts
            .verus_file
            .clone()
            .or_else(|| env_opt("VERUS_FILE").map(PathBuf::from));

        let verus_flags: Vec<String> = env_or("VERUS_FLAGS", "")
            .split_whitespace()
            .map(str::to_string)
            .filter(|s| !s.is_empty())
            .collect();

        let verus_csv = backend_csv("verus", "VERUS_CSV");

        // ── Creusot ───────────────────────────────────────────────────────────
        let creusot_flags: Vec<String> = env_or("CREUSOT_FLAGS", "")
            .split_whitespace()
            .map(str::to_string)
            .filter(|s| !s.is_empty())
            .collect();

        let creusot_bin_dir = env_opt("CREUSOT_BIN_DIR")
            .map(|p| PathBuf::from(expand(p)))
            .unwrap_or_else(|| PathBuf::from(expand(format!("{home}/.local/share/creusot/bin"))));

        let why3_config = env_opt("WHY3_CONFIG")
            .map(|p| PathBuf::from(expand(p)))
            .unwrap_or_else(|| PathBuf::from(expand(format!("{home}/.config/creusot/why3.conf"))));

        let dune_dir_locations = env_opt("DUNE_DIR_LOCATIONS")
            .unwrap_or_else(|| format!("why3find:lib:{home}/.local/share/creusot/share/why3find"));

        let creusot_csv = backend_csv("creusot", "CREUSOT_CSV");

        let creusot_timeout = env_opt("CREUSOT_TIMEOUT")
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(0); // 0 = unlimited: creusot compiles the whole workspace first

        let log_dir = opts
            .log_dir
            .clone()
            .or_else(|| env_opt("PROVE_LOG_DIR").map(|p| PathBuf::from(expand(p))))
            .unwrap_or_else(|| PathBuf::from("."));

        Ok(Self {
            run_kani: opts.kani,
            run_verus: opts.verus,
            run_creusot: opts.creusot,
            kani_package,
            kani_flags,
            kani_harness,
            kani_csv,
            kani_resume,
            verus_path,
            verus_file,
            verus_flags,
            verus_csv,
            creusot_package,
            creusot_flags,
            creusot_bin_dir,
            why3_config,
            dune_dir_locations,
            creusot_csv,
            creusot_timeout,
            timeout: env_opt("KANI_TIMEOUT")
                .and_then(|v| v.parse::<u64>().ok())
                .unwrap_or(opts.timeout),
            dry_run: opts.dry_run,
            log_dir,
        })
    }
}

// ── Runner ────────────────────────────────────────────────────────────────────

/// Execute the backends selected in `config`.
///
/// Returns `Ok(())` only when **all** selected backends succeed.
#[tracing::instrument(skip(config))]
pub fn run(config: &ProveConfig) -> anyhow::Result<()> {
    if !config.run_kani && !config.run_verus && !config.run_creusot {
        anyhow::bail!("No backend selected — use --kani, --verus, and/or --creusot");
    }

    let mut failed: Vec<&str> = Vec::new();

    if config.run_kani {
        if let Err(e) = run_kani(config) {
            eprintln!("❌ Kani failed: {e}");
            failed.push("kani");
        }
    }

    if config.run_verus {
        if let Err(e) = run_verus(config) {
            eprintln!("❌ Verus failed: {e}");
            failed.push("verus");
        }
    }

    if config.run_creusot {
        if let Err(e) = run_creusot(config) {
            eprintln!("❌ Creusot failed: {e}");
            failed.push("creusot");
        }
    }

    if failed.is_empty() {
        Ok(())
    } else {
        anyhow::bail!("Proof backends failed: {}", failed.join(", "))
    }
}

// ── Kani ──────────────────────────────────────────────────────────────────────

fn run_kani(config: &ProveConfig) -> anyhow::Result<()> {
    let pkg = config.kani_package.as_deref().ok_or_else(|| {
        anyhow::anyhow!(
            "No package for Kani — set KANI_PACKAGE or PROVE_PACKAGE in .env, or pass --package"
        )
    })?;

    // Single-harness shortcut (--kani-harness flag).
    if let Some(harness) = &config.kani_harness {
        let cmd = build_kani_harness_cmd(config, pkg, harness);
        let log = config.log_dir.join("prove_kani.log");
        return execute_timed("cargo kani", cmd, config.timeout, config.dry_run, &log).map(|_| ());
    }

    if config.dry_run {
        let csv_info = config
            .kani_csv
            .as_ref()
            .map(|p| format!(" → {}", p.display()))
            .unwrap_or_default();
        println!(
            "🔍 [dry-run] would discover harnesses via `cargo kani list`, then run each with timeout {}s{csv_info}",
            config.timeout
        );
        return Ok(());
    }

    // Warm up the codegen cache so the per-harness 300s timeout applies only
    // to CBMC model-checking, not compilation.
    let mut warm_cmd = Command::new("cargo");
    warm_cmd
        .arg("kani")
        .arg("-p")
        .arg(pkg)
        .arg("--only-codegen");
    for flag in &config.kani_flags {
        warm_cmd.arg(flag);
    }
    let kani_log = config.log_dir.join("prove_kani.log");
    println!("🔬 Building Kani model…");
    println!("📝 Logging to {}", kani_log.display());
    let build_bar = spinner("Starting codegen…");
    let build_sink = kani_build_sink(build_bar.clone());
    execute_with_progress(
        "cargo kani --only-codegen",
        warm_cmd,
        0,
        false,
        &kani_log,
        build_sink,
    )
    .context(
        "`cargo kani --only-codegen` failed — fix compilation errors before running harnesses",
    )?;
    build_bar.finish_and_clear();

    // Discover harnesses.
    let harnesses = list_kani_harnesses(pkg, &config.kani_flags)?;
    let total = harnesses.len();
    let csv_suffix = config
        .kani_csv
        .as_ref()
        .map(|p| format!(" → {}", p.display()))
        .unwrap_or_default();
    println!("🔬 Running {total} Kani harnesses{csv_suffix}");

    // Load already-passed harnesses when resuming.
    let passed_set: HashSet<String> = if config.kani_resume {
        if let Some(csv) = &config.kani_csv {
            if csv.exists() {
                load_passed_harnesses(csv)?
            } else {
                HashSet::new()
            }
        } else {
            HashSet::new()
        }
    } else {
        HashSet::new()
    };

    // Write (or append) CSV header.
    if let Some(csv) = &config.kani_csv {
        write_csv_header(csv, config.kani_resume)?;
    }

    let mut pass = 0usize;
    let mut fail = 0usize;

    let bar = ProgressBar::new(total as u64);
    bar.set_style(
        ProgressStyle::with_template("{spinner:.blue} [{pos}/{len}] {wide_msg} {elapsed_precise}")
            .expect("valid template")
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    bar.enable_steady_tick(std::time::Duration::from_millis(80));

    for (idx, harness) in harnesses.iter().enumerate() {
        let short = short_name(harness);
        if passed_set.contains(harness) {
            bar.println(format!("  [{}/{}] {:<60} ⏭  SKIP", idx + 1, total, short));
            bar.inc(1);
            pass += 1;
            continue;
        }

        bar.set_message(format!("{short}…"));

        let cmd = build_kani_harness_cmd(config, pkg, harness);
        let (status_str, elapsed) = run_harness_timed(cmd, config.timeout)?;
        let elapsed_s = elapsed.as_secs();

        if let Some(csv) = &config.kani_csv {
            append_csv_row(csv, pkg, harness, &status_str, elapsed_s)?;
        }

        match status_str.as_str() {
            "PASS" => {
                bar.println(format!(
                    "  [{}/{}] {:<60} ✅ PASS ({elapsed_s}s)",
                    idx + 1,
                    total,
                    short
                ));
                pass += 1;
            }
            "TIMEOUT" => {
                bar.println(format!(
                    "  [{}/{}] {:<60} ⏱  TIMEOUT ({elapsed_s}s)",
                    idx + 1,
                    total,
                    short
                ));
                fail += 1;
            }
            _ => {
                bar.println(format!(
                    "  [{}/{}] {:<60} ❌ FAIL ({elapsed_s}s)",
                    idx + 1,
                    total,
                    short
                ));
                fail += 1;
            }
        }
        bar.inc(1);
    }

    bar.finish_and_clear();
    println!("\nResults: {pass}/{total} passed, {fail} failed");
    if let Some(csv) = &config.kani_csv {
        println!("CSV:     {}", csv.display());
    }

    if fail > 0 {
        anyhow::bail!("{fail} Kani harness(es) failed or timed out");
    }
    Ok(())
}

/// Build a `cargo kani` command targeting a single harness by exact qualified name.
fn build_kani_harness_cmd(config: &ProveConfig, pkg: &str, harness: &str) -> Command {
    let mut cmd = Command::new("cargo");
    cmd.arg("kani")
        .arg("-p")
        .arg(pkg)
        .arg("--harness")
        .arg(harness)
        .arg("--exact");
    for flag in &config.kani_flags {
        cmd.arg(flag);
    }
    // Let kani manage its own per-harness timeout so it can clean up cbmc children
    // gracefully. --harness-timeout is experimental and requires unstable-options.
    if config.timeout > 0 {
        cmd.arg("-Z")
            .arg("unstable-options")
            .arg(format!("--harness-timeout={}s", config.timeout));
    }
    cmd
}

/// Extract the subset of `KANI_FLAGS` that `cargo kani list` accepts.
///
/// In practice this is the unstable `-Z <feature>` pairs such as
/// `-Z function-contracts` and `-Z stubbing`. Cargo-target flags like `--lib`
/// or `--features kani` are rejected by the `list` subcommand.
fn kani_list_flags(extra_flags: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < extra_flags.len() {
        if extra_flags[i] == "-Z" {
            out.push(extra_flags[i].clone());
            if let Some(next) = extra_flags.get(i + 1) {
                out.push(next.clone());
                i += 2;
                continue;
            }
        }
        i += 1;
    }
    out
}

/// Run `cargo kani list` from the package directory and return fully-qualified harness names.
#[tracing::instrument(skip(extra_flags), fields(package = %pkg))]
fn list_kani_harnesses(pkg: &str, extra_flags: &[String]) -> anyhow::Result<Vec<String>> {
    let pkg_dir = find_package_dir(pkg)?;
    let list_flags = kani_list_flags(extra_flags);

    let mut cmd = Command::new("cargo");
    cmd.arg("kani").arg("list");
    cmd.current_dir(&pkg_dir);
    for flag in &list_flags {
        cmd.arg(flag);
    }
    tracing::debug!(
        cwd = %pkg_dir.display(),
        list_flags = ?list_flags,
        command = %format_command(&cmd),
        "Running `cargo kani list`"
    );

    // kani list writes everything to stderr; stdout may be empty or contain the table.
    let output = cmd.output().context("Failed to run `cargo kani list`")?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    tracing::debug!(
        success = output.status.success(),
        status = ?output.status.code(),
        stdout = %stdout,
        stderr = %stderr,
        "`cargo kani list` completed"
    );

    if !output.status.success() {
        anyhow::bail!("`cargo kani list` failed:\n{stderr}");
    }

    // Table rows appear in stderr in Kani 0.67.
    let combined = format!("{stdout}{stderr}");
    parse_kani_list_output(&combined)
}

/// Parse `cargo kani list` table output and return fully-qualified harness names.
///
/// The table has two row shapes:
/// - Regular harness:  `| | <crate> | <qualified::harness> |`
/// - Contract harness: `| | <crate> | <function> | <qualified::harness> |`
///
/// We take the last `::` column of each data row.
#[tracing::instrument(skip(output))]
pub fn parse_kani_list_output(output: &str) -> anyhow::Result<Vec<String>> {
    let mut harnesses = Vec::new();

    for line in output.lines() {
        if !line.starts_with('|') || line.contains("---") {
            continue;
        }
        let cols: Vec<&str> = line
            .split('|')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .collect();
        // Skip headers and totals.
        if cols.iter().any(|c| {
            matches!(
                *c,
                "Crate" | "Function" | "Total" | "Contract Harnesses (#[kani::proof_for_contract])"
            )
        }) {
            continue;
        }
        // The harness to run is the last column that contains `::`.
        if let Some(harness) = cols.iter().rev().find(|c| c.contains("::")) {
            harnesses.push(harness.to_string());
        }
    }

    if harnesses.is_empty() {
        anyhow::bail!(
            "No harnesses found — is the package compiled and does it contain `#[kani::proof]` harnesses?"
        );
    }

    // Deduplicate (contract rows can appear in both sections).
    harnesses.sort();
    harnesses.dedup();
    Ok(harnesses)
}

/// Locate the directory of a workspace package via `cargo metadata`.
fn find_package_dir(package: &str) -> anyhow::Result<PathBuf> {
    let output = Command::new("cargo")
        .args(["metadata", "--no-deps", "--format-version", "1"])
        .output()
        .context("Failed to run `cargo metadata`")?;

    let json: serde_json::Value =
        serde_json::from_slice(&output.stdout).context("Failed to parse `cargo metadata`")?;

    for pkg in json["packages"].as_array().unwrap_or(&vec![]) {
        if pkg["name"].as_str() == Some(package) {
            let manifest = pkg["manifest_path"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("Missing manifest_path in metadata"))?;
            return PathBuf::from(manifest)
                .parent()
                .map(|p| p.to_path_buf())
                .ok_or_else(|| anyhow::anyhow!("Could not determine package directory"));
        }
    }

    anyhow::bail!("Package '{package}' not found in workspace")
}

/// Run a command with a simple wait (no external timeout — kani manages its own
/// harness timeout via `--harness-timeout`). Returns (status_str, elapsed).
fn run_harness_timed(
    mut cmd: Command,
    _timeout_secs: u64,
) -> anyhow::Result<(String, std::time::Duration)> {
    cmd.stdout(Stdio::null()).stderr(Stdio::null());

    let start = Instant::now();
    let mut child = cmd.spawn().context("Failed to spawn cargo kani")?;
    let status = child.wait().context("Failed to wait for cargo kani")?;
    let elapsed = start.elapsed();
    // Kani exits with a non-success status when a harness times out or fails.
    Ok((
        if status.success() { "PASS" } else { "FAIL" }.to_string(),
        elapsed,
    ))
}

/// Load harness names already recorded as PASS in an existing CSV.
fn load_passed_harnesses(csv: &PathBuf) -> anyhow::Result<HashSet<String>> {
    let content = fs::read_to_string(csv)
        .with_context(|| format!("Failed to read CSV: {}", csv.display()))?;
    let passed = content
        .lines()
        .skip(1) // header
        .filter_map(|line| {
            let mut cols = line.splitn(5, ',');
            let _module = cols.next()?;
            let harness = cols.next()?.to_string();
            let status = cols.next()?;
            if status == "PASS" {
                Some(harness)
            } else {
                None
            }
        })
        .collect();
    Ok(passed)
}

/// Write (or append to) the CSV header.
fn write_csv_header(csv: &PathBuf, append: bool) -> anyhow::Result<()> {
    if append && csv.exists() {
        return Ok(());
    }
    let mut f = fs::File::create(csv)
        .with_context(|| format!("Failed to create CSV: {}", csv.display()))?;
    writeln!(f, "module,harness,status,duration_secs,timestamp")?;
    Ok(())
}

/// Append one result row to the CSV.
fn append_csv_row(
    csv: &PathBuf,
    module: &str,
    harness: &str,
    status: &str,
    elapsed_s: u64,
) -> anyhow::Result<()> {
    let mut f = fs::OpenOptions::new()
        .append(true)
        .open(csv)
        .with_context(|| format!("Failed to open CSV: {}", csv.display()))?;
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    writeln!(f, "{module},{harness},{status},{elapsed_s},{ts}")?;
    Ok(())
}

/// Return the last path segment of a qualified Rust name for display.
fn short_name(qualified: &str) -> &str {
    qualified.rsplit("::").next().unwrap_or(qualified)
}

// ── Verus ─────────────────────────────────────────────────────────────────────

fn run_verus(config: &ProveConfig) -> anyhow::Result<()> {
    if !config.verus_path.exists() {
        anyhow::bail!(
            "Verus not found at: {}\nSet VERUS_PATH in .env or pass --verus-path",
            config.verus_path.display()
        );
    }

    let Some(verus_file) = &config.verus_file else {
        anyhow::bail!("No Verus source file — set VERUS_FILE in .env or pass --verus-file");
    };

    if verus_file.is_dir() {
        run_verus_dir(config, verus_file)
    } else {
        run_verus_file(config, verus_file)
    }
}

/// Verify a single Verus source file.
fn run_verus_file(config: &ProveConfig, verus_file: &Path) -> anyhow::Result<()> {
    let mut cmd = Command::new(&config.verus_path);
    cmd.arg("--crate-type=lib").arg(verus_file);
    for flag in &config.verus_flags {
        cmd.arg(flag);
    }

    let log = config.log_dir.join("prove_verus.log");
    println!("🔬 Running verus…");
    println!("📝 Logging to {}", log.display());
    let bar = spinner("Verifying…");
    let sink = verus_sink(bar.clone());
    let start = Instant::now();
    let status = execute_with_progress("verus", cmd, config.timeout, config.dry_run, &log, sink)?;
    bar.finish_and_clear();
    let elapsed_s = start.elapsed().as_secs();

    if !config.dry_run {
        if let Some(csv) = &config.verus_csv {
            write_csv_header(csv, false)?;
            append_csv_row(
                csv,
                "verus",
                verus_file.to_string_lossy().as_ref(),
                &status,
                elapsed_s,
            )?;
        }
    }

    if status == "PASS" || status == "DRY-RUN" {
        println!("✅ verus PASS ({elapsed_s}s) — see {}", log.display());
        Ok(())
    } else {
        println!("❌ verus FAIL ({elapsed_s}s) — see {}", log.display());
        anyhow::bail!("verus verification failed")
    }
}

/// Verify all `*.rs` files (excluding `mod.rs`) in a directory, one by one.
///
/// This matches the `generate verus --out <dir>` output layout: each generated
/// file is self-contained and can be verified independently.
fn run_verus_dir(config: &ProveConfig, dir: &Path) -> anyhow::Result<()> {
    let mut files: Vec<PathBuf> = fs::read_dir(dir)
        .with_context(|| format!("Cannot read Verus dir: {}", dir.display()))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| {
            p.extension().map(|e| e == "rs").unwrap_or(false)
                && p.file_name().map(|n| n != "mod.rs").unwrap_or(false)
        })
        .collect();
    files.sort();

    let total = files.len();
    if total == 0 {
        anyhow::bail!(
            "No .rs files (other than mod.rs) found in {}",
            dir.display()
        );
    }

    let csv_suffix = config
        .verus_csv
        .as_ref()
        .map(|p| format!(" → {}", p.display()))
        .unwrap_or_default();
    println!(
        "🔬 Running verus on {total} files in {}{csv_suffix}",
        dir.display()
    );

    if let Some(csv) = &config.verus_csv {
        write_csv_header(csv, false)?;
    }

    let bar = ProgressBar::new(total as u64);
    bar.set_style(
        ProgressStyle::with_template("{spinner:.blue} [{pos}/{len}] {wide_msg} {elapsed_precise}")
            .expect("valid template")
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    bar.enable_steady_tick(std::time::Duration::from_millis(80));

    let mut pass = 0usize;
    let mut fail = 0usize;

    for (idx, file) in files.iter().enumerate() {
        let stem = file
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| file.display().to_string());

        bar.set_message(format!("{stem}…"));

        let log_name = format!("prove_verus_{stem}.log");
        let log = config.log_dir.join(&log_name);

        let mut cmd = Command::new(&config.verus_path);
        cmd.arg("--crate-type=lib").arg(file);
        for flag in &config.verus_flags {
            cmd.arg(flag);
        }

        let sink = verus_sink_quiet();
        let start = Instant::now();
        let status =
            execute_with_progress("verus", cmd, config.timeout, config.dry_run, &log, sink)?;
        let elapsed_s = start.elapsed().as_secs();

        if let Some(csv) = &config.verus_csv {
            if !config.dry_run {
                append_csv_row(csv, "verus", &stem, &status, elapsed_s)?;
            }
        }

        match status.as_str() {
            "PASS" => {
                bar.println(format!(
                    "  [{}/{}] {:<55} ✅ PASS ({elapsed_s}s)",
                    idx + 1,
                    total,
                    stem
                ));
                pass += 1;
            }
            "DRY-RUN" => {
                bar.println(format!("  [{}/{}] {stem} 🔍 DRY-RUN", idx + 1, total));
                pass += 1;
            }
            "TIMEOUT" => {
                bar.println(format!(
                    "  [{}/{}] {:<55} ⏱  TIMEOUT ({elapsed_s}s) — see {log_name}",
                    idx + 1,
                    total,
                    stem
                ));
                fail += 1;
            }
            _ => {
                bar.println(format!(
                    "  [{}/{}] {:<55} ❌ FAIL ({elapsed_s}s) — see {log_name}",
                    idx + 1,
                    total,
                    stem
                ));
                fail += 1;
            }
        }
        bar.inc(1);
    }

    bar.finish_and_clear();
    println!("\nResults: {pass}/{total} passed, {fail} failed");
    if let Some(csv) = &config.verus_csv {
        println!("CSV:     {}", csv.display());
    }

    if fail > 0 {
        anyhow::bail!("{fail} Verus file(s) failed or timed out");
    }
    Ok(())
}

// ── Creusot ───────────────────────────────────────────────────────────────────

fn run_creusot(config: &ProveConfig) -> anyhow::Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.arg("creusot").arg("prove");

    for flag in &config.creusot_flags {
        cmd.arg(flag);
    }

    cmd.arg("--");

    if let Some(pkg) = &config.creusot_package {
        cmd.arg("-p").arg(pkg);
    } else {
        anyhow::bail!(
            "No package for Creusot — set CREUSOT_PACKAGE or PROVE_PACKAGE in .env, or pass --package"
        );
    }

    cmd.arg("--features").arg("creusot");

    // Augment PATH so why3find is discoverable.
    let current_path = std::env::var("PATH").unwrap_or_default();
    let new_path = format!("{}:{}", config.creusot_bin_dir.display(), current_path);
    cmd.env("PATH", &new_path);
    cmd.env("WHY3CONFIG", &config.why3_config);
    cmd.env("DUNE_DIR_LOCATIONS", &config.dune_dir_locations);

    let log = config.log_dir.join("prove_creusot.log");
    println!("🔬 Running cargo creusot prove…");
    println!("📝 Logging to {}", log.display());
    let bar = spinner("Starting…");
    let sink = creusot_sink(bar.clone());
    let start = Instant::now();
    let status = execute_with_progress(
        "cargo creusot prove",
        cmd,
        config.creusot_timeout,
        config.dry_run,
        &log,
        sink,
    )?;
    bar.finish_and_clear();
    let elapsed_s = start.elapsed().as_secs();

    if !config.dry_run {
        if let Some(csv) = &config.creusot_csv {
            write_csv_header(csv, false)?;
            append_csv_row(csv, "creusot", "creusot", &status, elapsed_s)?;
        }
    }

    if status == "PASS" || status == "DRY-RUN" {
        println!(
            "✅ cargo creusot prove PASS ({elapsed_s}s) — see {}",
            log.display()
        );
        Ok(())
    } else {
        println!(
            "❌ cargo creusot prove FAIL ({elapsed_s}s) — see {}",
            log.display()
        );
        anyhow::bail!("cargo creusot prove failed")
    }
}

// ── Shared execute helpers ────────────────────────────────────────────────────

/// Callback type shared between the stdout and stderr reader threads.
///
/// Each line of output is passed to the handler; the handler drives the
/// indicatif progress bar and decides whether to surface anything to the
/// terminal (via `bar.println`).  All raw output is always written to the log
/// regardless of what the handler does.
type LineSink = Arc<Mutex<dyn FnMut(&str) + Send>>;

/// Create a spinning progress bar on stderr.
fn spinner(msg: impl Into<String>) -> ProgressBar {
    let bar = ProgressBar::new_spinner();
    bar.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .expect("valid template")
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    bar.set_message(msg.into());
    bar.enable_steady_tick(std::time::Duration::from_millis(80));
    bar
}

/// Build a `LineSink` that drives a spinner during `cargo creusot prove`.
///
/// Two phases are detected from the output stream:
/// - **Compile** (`Compiling …`): spinner message shows the crate count.
/// - **Prove** (`Goal Coma.vc_…`): spinner message counts proved goals.
///
/// Dangling-file warnings, `cargo creusot clean` hints, and the final
/// `Proved (N files)` summary are surfaced above the bar via `bar.println`.
fn creusot_sink(bar: ProgressBar) -> LineSink {
    let b = bar.clone();
    let mut crates = 0usize;
    let mut goals = 0usize;
    let mut proving = false;
    Arc::new(Mutex::new(move |line: &str| {
        let t = line.trim();
        if !proving && t.starts_with("Compiling ") {
            crates += 1;
            b.set_message(format!("🔨 Compiling ({crates} crates)…"));
        } else if t.starts_with("Finished ") {
            proving = true;
            b.set_message("🔬 Proving…");
        } else if proving && t.starts_with("Goal ") {
            goals += 1;
            b.set_message(format!("🔬 Proving [{goals} goals]…"));
        } else if t.starts_with("Warning:") || t.contains("cargo creusot clean") {
            b.println(line);
        } else if t.starts_with("Proved ") || t.starts_with("Error ") || t.contains("FAILED") {
            b.println(line);
        }
    }))
}

/// Build a `LineSink` that drives a spinner during the Kani codegen warmup.
fn kani_build_sink(bar: ProgressBar) -> LineSink {
    let b = bar.clone();
    let mut crates = 0usize;
    Arc::new(Mutex::new(move |line: &str| {
        let t = line.trim();
        if t.starts_with("Compiling ") {
            crates += 1;
            b.set_message(format!("🔨 Building model ({crates} crates)…"));
        } else if t.starts_with("error") {
            b.println(line);
        }
    }))
}

/// Build a `LineSink` that drives a spinner during Verus.
fn verus_sink(bar: ProgressBar) -> LineSink {
    let b = bar.clone();
    Arc::new(Mutex::new(move |line: &str| {
        let t = line.trim();
        // Surface the summary line ("N verified, M errors") above the bar.
        if t.contains("verified,") || t.starts_with("error") {
            b.println(line);
        }
    }))
}

/// A silent `LineSink` used in directory mode — results are shown per-file via
/// the outer progress bar rather than by surfacing individual Verus output lines.
fn verus_sink_quiet() -> LineSink {
    Arc::new(Mutex::new(|_line: &str| {}))
}

/// Format a command for tracing and log headers.
fn format_command(cmd: &Command) -> String {
    let program = cmd.get_program().to_string_lossy();
    let args = cmd
        .get_args()
        .map(|arg| arg.to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    if args.is_empty() {
        program.into_owned()
    } else {
        format!("{program} {}", args.join(" "))
    }
}

/// Record the command, cwd, and explicit env overrides before spawning.
fn log_command_context(
    label: &str,
    cmd: &Command,
    log: &Arc<Mutex<fs::File>>,
) -> anyhow::Result<()> {
    let command = format_command(cmd);
    let cwd = cmd
        .get_current_dir()
        .map(Path::to_path_buf)
        .unwrap_or(std::env::current_dir().context("Failed to resolve current directory")?);
    let env_overrides = cmd
        .get_envs()
        .map(|(key, value)| match value {
            Some(value) => format!("{}={}", key.to_string_lossy(), value.to_string_lossy()),
            None => format!("{}=<removed>", key.to_string_lossy()),
        })
        .collect::<Vec<_>>();

    tracing::debug!(
        label,
        command = %command,
        cwd = %cwd.display(),
        env_overrides = ?env_overrides,
        "Spawning proof command"
    );

    let mut file = log
        .lock()
        .map_err(|_| anyhow::anyhow!("Failed to lock command log"))?;
    writeln!(file, "# command: {command}")?;
    writeln!(file, "# cwd: {}", cwd.display())?;
    if env_overrides.is_empty() {
        writeln!(file, "# env overrides: <none>")?;
    } else {
        writeln!(file, "# env overrides:")?;
        for entry in &env_overrides {
            writeln!(file, "#   {entry}")?;
        }
    }
    Ok(())
}

/// Execute a command, driving an indicatif `LineSink` instead of echoing raw
/// output to the terminal.  All stdout+stderr is still written to `log_path`.
/// Returns `"PASS"` / `"FAIL"` / `"TIMEOUT"` / `"DRY-RUN"`.
fn execute_with_progress(
    label: &str,
    mut cmd: Command,
    timeout_secs: u64,
    dry_run: bool,
    log_path: &Path,
    sink: LineSink,
) -> anyhow::Result<String> {
    if dry_run {
        println!("🔍 [dry-run] {label}: {:?}", cmd);
        return Ok("DRY-RUN".to_string());
    }

    let log = Arc::new(Mutex::new(
        fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(log_path)
            .with_context(|| format!("Failed to open log: {}", log_path.display()))?,
    ));

    log_command_context(label, &cmd, &log)?;
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

    let mut child = cmd
        .spawn()
        .with_context(|| format!("Failed to spawn {label}"))?;

    // Stdout reader: log every line + call sink.
    let stdout_pipe = child.stdout.take();
    let sink_out = Arc::clone(&sink);
    let log_out = Arc::clone(&log);
    let stdout_thread = stdout_pipe.map(|pipe| {
        thread::spawn(move || {
            let reader = std::io::BufReader::new(pipe);
            for line in reader.lines().map_while(Result::ok) {
                if let Ok(mut f) = log_out.lock() {
                    let _ = writeln!(f, "{line}");
                }
                if let Ok(mut h) = sink_out.lock() {
                    h(&line);
                }
            }
        })
    });

    // Stderr reader: same.
    let stderr_pipe = child.stderr.take();
    let sink_err = Arc::clone(&sink);
    let log_err = Arc::clone(&log);
    let stderr_thread = stderr_pipe.map(|pipe| {
        thread::spawn(move || {
            let reader = std::io::BufReader::new(pipe);
            for line in reader.lines().map_while(Result::ok) {
                if let Ok(mut f) = log_err.lock() {
                    let _ = writeln!(f, "{line}");
                }
                if let Ok(mut h) = sink_err.lock() {
                    h(&line);
                }
            }
        })
    });

    let join_threads = |st: Option<thread::JoinHandle<()>>, et: Option<thread::JoinHandle<()>>| {
        if let Some(t) = st {
            let _ = t.join();
        }
        if let Some(t) = et {
            let _ = t.join();
        }
    };

    let status = if timeout_secs == 0 {
        let s = child.wait().context("Failed to wait for child")?;
        join_threads(stdout_thread, stderr_thread);
        s
    } else {
        let deadline = Instant::now() + std::time::Duration::from_secs(timeout_secs);
        loop {
            match child.try_wait()? {
                Some(s) => {
                    join_threads(stdout_thread, stderr_thread);
                    break s;
                }
                None => {
                    if Instant::now() >= deadline {
                        let _ = child.kill();
                        let _ = child.wait();
                        join_threads(stdout_thread, stderr_thread);
                        anyhow::bail!(
                            "{label} timed out after {timeout_secs}s — see {}",
                            log_path.display()
                        );
                    }
                    std::thread::sleep(std::time::Duration::from_millis(200));
                }
            }
        }
    };

    if status.success() {
        Ok("PASS".to_string())
    } else {
        Ok("FAIL".to_string())
    }
}

/// Run a command with a timeout, teeing stdout+stderr to both the terminal and
/// `log_path`. Returns the status string ("PASS"/"FAIL"/"TIMEOUT").
fn execute_timed(
    label: &str,
    mut cmd: Command,
    timeout_secs: u64,
    dry_run: bool,
    log_path: &Path,
) -> anyhow::Result<String> {
    if dry_run {
        println!("🔍 [dry-run] {label}: {:?}", cmd);
        return Ok("DRY-RUN".to_string());
    }

    println!("🔬 Running {label}…");
    println!("📝 Logging to {}", log_path.display());

    let log = Arc::new(Mutex::new(
        fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(log_path)
            .with_context(|| format!("Failed to open log: {}", log_path.display()))?,
    ));

    log_command_context(label, &cmd, &log)?;
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

    let mut child = cmd
        .spawn()
        .with_context(|| format!("Failed to spawn {label}"))?;

    // Tee stdout → terminal + log.
    let stdout_pipe = child.stdout.take();
    let log_out = Arc::clone(&log);
    let stdout_thread = stdout_pipe.map(|pipe| {
        thread::spawn(move || {
            let mut buf = [0u8; 8192];
            let mut reader = pipe;
            loop {
                match reader.read(&mut buf) {
                    Ok(0) | Err(_) => break,
                    Ok(n) => {
                        let _ = std::io::stdout().write_all(&buf[..n]);
                        let _ = std::io::stdout().flush();
                        if let Ok(mut f) = log_out.lock() {
                            let _ = f.write_all(&buf[..n]);
                        }
                    }
                }
            }
        })
    });

    // Tee stderr → terminal + log.
    let stderr_pipe = child.stderr.take();
    let log_err = Arc::clone(&log);
    let stderr_thread = stderr_pipe.map(|pipe| {
        thread::spawn(move || {
            let mut buf = [0u8; 8192];
            let mut reader = pipe;
            loop {
                match reader.read(&mut buf) {
                    Ok(0) | Err(_) => break,
                    Ok(n) => {
                        let _ = std::io::stderr().write_all(&buf[..n]);
                        let _ = std::io::stderr().flush();
                        if let Ok(mut f) = log_err.lock() {
                            let _ = f.write_all(&buf[..n]);
                        }
                    }
                }
            }
        })
    });

    let join_threads = |st: Option<thread::JoinHandle<()>>, et: Option<thread::JoinHandle<()>>| {
        if let Some(t) = st {
            let _ = t.join();
        }
        if let Some(t) = et {
            let _ = t.join();
        }
    };

    let status = if timeout_secs == 0 {
        let s = child.wait().context("Failed to wait for child")?;
        join_threads(stdout_thread, stderr_thread);
        s
    } else {
        let deadline = Instant::now() + std::time::Duration::from_secs(timeout_secs);
        loop {
            match child.try_wait()? {
                Some(s) => {
                    join_threads(stdout_thread, stderr_thread);
                    break s;
                }
                None => {
                    if Instant::now() >= deadline {
                        let _ = child.kill();
                        let _ = child.wait();
                        join_threads(stdout_thread, stderr_thread);
                        anyhow::bail!(
                            "{label} timed out after {timeout_secs}s — see {}",
                            log_path.display()
                        );
                    }
                    std::thread::sleep(std::time::Duration::from_millis(500));
                }
            }
        }
    };

    if status.success() {
        println!("✅ {label} passed");
        Ok("PASS".to_string())
    } else {
        anyhow::bail!("{label} exited with {status} — see {}", log_path.display());
    }
}
