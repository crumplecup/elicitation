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
//! | `KANI_FLAGS`          | `""`                                                  | kani           |
//! | `KANI_CSV`            | `kani_verification_results.csv`                       | kani           |
//! | `VERUS_PATH`          | `~/repos/verus/source/target-verus/release/verus`     | verus          |
//! | `VERUS_FILE`          | *(none — required for verus if no `--verus-file`)*   | verus          |
//! | `VERUS_FLAGS`         | `""`                                                  | verus          |
//! | `CREUSOT_PACKAGE`     | falls back to `PROVE_PACKAGE`                         | creusot        |
//! | `CREUSOT_FLAGS`       | `""`                                                  | creusot        |
//! | `CREUSOT_BIN_DIR`     | `~/.local/share/creusot/bin`                          | creusot        |
//! | `WHY3_CONFIG`         | `~/.config/creusot/why3.conf`                         | creusot        |
//! | `DUNE_DIR_LOCATIONS`  | `why3find:lib:~/.local/share/creusot/share/why3find`  | creusot        |

use std::{
    collections::HashSet,
    fs,
    io::Write as _,
    path::PathBuf,
    process::{Command, Stdio},
    time::Instant,
};
use anyhow::Context as _;

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
    /// CSV file path for per-harness results.
    pub kani_csv: PathBuf,
    /// Skip harnesses already recorded as PASS in the CSV.
    pub kani_resume: bool,

    /// Path to the `verus` binary.
    pub verus_path: PathBuf,
    /// Source file (or directory) for Verus.
    pub verus_file: Option<PathBuf>,
    /// Extra flags passed verbatim to the `verus` binary.
    pub verus_flags: Vec<String>,

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

    /// Timeout in seconds per verification unit. 0 means unlimited.
    pub timeout: u64,

    /// Print the commands that would run without executing them.
    pub dry_run: bool,
}

impl ProveConfig {
    /// Build from CLI overrides on top of values loaded from `.env`.
    pub fn resolve(opts: &super::ProveOpts) -> anyhow::Result<Self> {
        // Load .env — walk up from cwd until found; silently skip if absent.
        let _ = dotenvy::dotenv();

        let home = std::env::var("HOME").unwrap_or_default();
        let expand = |s: String| shellexpand::tilde(&s).to_string();

        // ── helpers ──────────────────────────────────────────────────────────
        let env_or = |key: &str, default: &str| -> String {
            std::env::var(key).unwrap_or_else(|_| default.to_string())
        };
        let env_opt = |key: &str| -> Option<String> { std::env::var(key).ok() };

        // ── package resolution ────────────────────────────────────────────────
        let prove_pkg = opts
            .package
            .clone()
            .or_else(|| env_opt("PROVE_PACKAGE"));

        let kani_package = env_opt("KANI_PACKAGE").or_else(|| prove_pkg.clone());
        let creusot_package = env_opt("CREUSOT_PACKAGE").or_else(|| prove_pkg.clone());

        // ── Kani ─────────────────────────────────────────────────────────────
        let kani_flags: Vec<String> = env_or("KANI_FLAGS", "")
            .split_whitespace()
            .map(str::to_string)
            .filter(|s| !s.is_empty())
            .collect();

        let kani_harness = opts.kani_harness.clone();

        let kani_csv = opts
            .csv
            .clone()
            .or_else(|| env_opt("KANI_CSV").map(PathBuf::from))
            .unwrap_or_else(|| PathBuf::from("kani_verification_results.csv"));

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

        let dune_dir_locations = env_opt("DUNE_DIR_LOCATIONS").unwrap_or_else(|| {
            format!("why3find:lib:{home}/.local/share/creusot/share/why3find")
        });

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
            creusot_package,
            creusot_flags,
            creusot_bin_dir,
            why3_config,
            dune_dir_locations,
            timeout: opts.timeout,
            dry_run: opts.dry_run,
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
    let pkg = config
        .kani_package
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!(
            "No package for Kani — set KANI_PACKAGE or PROVE_PACKAGE in .env, or pass --package"
        ))?;

    // Single-harness shortcut (--kani-harness flag).
    if let Some(harness) = &config.kani_harness {
        let cmd = build_kani_harness_cmd(config, pkg, harness);
        return execute_timed("cargo kani", cmd, config.timeout, config.dry_run)
            .map(|_| ());
    }

    if config.dry_run {
        println!("🔍 [dry-run] would discover harnesses via `cargo kani list`, then run each with timeout {}s → {}", config.timeout, config.kani_csv.display());
        return Ok(());
    }

    // Discover harnesses.
    let harnesses = list_kani_harnesses(pkg, &config.kani_flags)?;
    let total = harnesses.len();
    println!("🔬 Running {total} Kani harnesses → {}", config.kani_csv.display());

    // Load already-passed harnesses when resuming.
    let passed_set: HashSet<String> = if config.kani_resume && config.kani_csv.exists() {
        load_passed_harnesses(&config.kani_csv)?
    } else {
        HashSet::new()
    };

    // Write (or append) CSV header.
    write_csv_header(&config.kani_csv, config.kani_resume)?;

    let mut pass = 0usize;
    let mut fail = 0usize;

    for (idx, harness) in harnesses.iter().enumerate() {
        if passed_set.contains(harness) {
            println!("  [{}/{}] {:<60} ⏭  SKIP", idx + 1, total, short_name(harness));
            pass += 1;
            continue;
        }

        print!("  [{}/{}] {:<60}", idx + 1, total, short_name(harness));
        let _ = std::io::stdout().flush();

        let cmd = build_kani_harness_cmd(config, pkg, harness);
        let (status_str, elapsed) =
            run_harness_timed(cmd, config.timeout)?;
        let elapsed_s = elapsed.as_secs();

        append_csv_row(&config.kani_csv, pkg, harness, &status_str, elapsed_s)?;

        match status_str.as_str() {
            "PASS" => {
                println!("✅ PASS ({elapsed_s}s)");
                pass += 1;
            }
            "TIMEOUT" => {
                println!("⏱  TIMEOUT ({elapsed_s}s)");
                fail += 1;
            }
            _ => {
                println!("❌ FAIL ({elapsed_s}s)");
                fail += 1;
            }
        }
    }

    println!("\nResults: {pass}/{total} passed, {fail} failed");
    println!("CSV:     {}", config.kani_csv.display());

    if fail > 0 {
        anyhow::bail!("{fail} Kani harness(es) failed or timed out");
    }
    Ok(())
}

/// Build a `cargo kani` command targeting a single harness by exact qualified name.
fn build_kani_harness_cmd(config: &ProveConfig, pkg: &str, harness: &str) -> Command {
    let mut cmd = Command::new("cargo");
    cmd.arg("kani")
        .arg("-p").arg(pkg)
        .arg("--harness").arg(harness)
        .arg("--exact");
    for flag in &config.kani_flags {
        cmd.arg(flag);
    }
    cmd
}

/// Run `cargo kani list` from the package directory and return fully-qualified harness names.
fn list_kani_harnesses(pkg: &str, extra_flags: &[String]) -> anyhow::Result<Vec<String>> {
    let pkg_dir = find_package_dir(pkg)?;

    let mut cmd = Command::new("cargo");
    cmd.arg("kani").arg("list");
    cmd.current_dir(&pkg_dir);
    for flag in extra_flags {
        cmd.arg(flag);
    }

    // kani list writes everything to stderr; stdout may be empty or contain the table.
    let output = cmd
        .output()
        .context("Failed to run `cargo kani list`")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("`cargo kani list` failed:\n{stderr}");
    }

    // Table rows appear in stderr in Kani 0.67.
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    parse_kani_list_output(&combined)
}

/// Parse `cargo kani list` table output and return fully-qualified harness names.
///
/// The table has two row shapes:
/// - Regular harness:  `| | <crate> | <qualified::harness> |`
/// - Contract harness: `| | <crate> | <function> | <qualified::harness> |`
///
/// We take the last `::` column of each data row.
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
        anyhow::bail!("No harnesses found — is the package compiled and does it contain `#[kani::proof]` harnesses?");
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

/// Run a command with a per-invocation timeout. Returns (status_str, elapsed).
fn run_harness_timed(
    mut cmd: Command,
    timeout_secs: u64,
) -> anyhow::Result<(String, std::time::Duration)> {
    cmd.stdout(Stdio::null()).stderr(Stdio::null());
    let start = Instant::now();
    let mut child = cmd.spawn().context("Failed to spawn cargo kani")?;

    if timeout_secs == 0 {
        let status = child.wait().context("Failed to wait for cargo kani")?;
        let elapsed = start.elapsed();
        return Ok((if status.success() { "PASS" } else { "FAIL" }.to_string(), elapsed));
    }

    let deadline = start + std::time::Duration::from_secs(timeout_secs);
    loop {
        match child.try_wait()? {
            Some(s) => {
                let elapsed = start.elapsed();
                return Ok((
                    if s.success() { "PASS" } else { "FAIL" }.to_string(),
                    elapsed,
                ));
            }
            None => {
                if Instant::now() >= deadline {
                    let _ = child.kill();
                    return Ok(("TIMEOUT".to_string(), start.elapsed()));
                }
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
        }
    }
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
            if status == "PASS" { Some(harness) } else { None }
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
        anyhow::bail!(
            "No Verus source file — set VERUS_FILE in .env or pass --verus-file"
        );
    };

    let mut cmd = Command::new(&config.verus_path);
    cmd.arg("--crate-type=lib").arg(verus_file);

    for flag in &config.verus_flags {
        cmd.arg(flag);
    }

    execute_timed("verus", cmd, config.timeout, config.dry_run).map(|_| ())
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

    // Augment PATH so why3find is discoverable.
    let current_path = std::env::var("PATH").unwrap_or_default();
    let new_path = format!("{}:{}", config.creusot_bin_dir.display(), current_path);
    cmd.env("PATH", &new_path);
    cmd.env("WHY3CONFIG", &config.why3_config);
    cmd.env("DUNE_DIR_LOCATIONS", &config.dune_dir_locations);

    execute_timed("cargo creusot prove", cmd, config.timeout, config.dry_run).map(|_| ())
}

// ── Shared execute helpers ────────────────────────────────────────────────────

/// Run a command with inherited stdio, enforcing an optional timeout.
/// Returns the exit status string ("PASS"/"FAIL"/"TIMEOUT").
fn execute_timed(
    label: &str,
    mut cmd: Command,
    timeout_secs: u64,
    dry_run: bool,
) -> anyhow::Result<String> {
    if dry_run {
        println!("🔍 [dry-run] {label}: {:?}", cmd);
        return Ok("DRY-RUN".to_string());
    }

    println!("🔬 Running {label}…");
    cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());

    if timeout_secs == 0 {
        let status = cmd
            .status()
            .with_context(|| format!("Failed to execute {label}"))?;
        if status.success() {
            println!("✅ {label} passed");
            return Ok("PASS".to_string());
        } else {
            anyhow::bail!("{label} exited with {status}");
        }
    }

    let mut child = cmd
        .spawn()
        .with_context(|| format!("Failed to spawn {label}"))?;
    let deadline = Instant::now() + std::time::Duration::from_secs(timeout_secs);
    loop {
        match child.try_wait()? {
            Some(s) => {
                if s.success() {
                    println!("✅ {label} passed");
                    return Ok("PASS".to_string());
                } else {
                    anyhow::bail!("{label} exited with {s}");
                }
            }
            None => {
                if Instant::now() >= deadline {
                    let _ = child.kill();
                    anyhow::bail!("{label} timed out after {timeout_secs}s");
                }
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
        }
    }
}

