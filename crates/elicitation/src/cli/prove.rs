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
//! | `VERUS_PATH`          | `~/repos/verus/source/target-verus/release/verus`     | verus          |
//! | `VERUS_FILE`          | *(none — required for verus if no `--verus-file`)*   | verus          |
//! | `VERUS_FLAGS`         | `""`                                                  | verus          |
//! | `CREUSOT_PACKAGE`     | falls back to `PROVE_PACKAGE`                         | creusot        |
//! | `CREUSOT_FLAGS`       | `""`                                                  | creusot        |
//! | `CREUSOT_BIN_DIR`     | `~/.local/share/creusot/bin`                          | creusot        |
//! | `WHY3_CONFIG`         | `~/.config/creusot/why3.conf`                         | creusot        |
//! | `DUNE_DIR_LOCATIONS`  | `why3find:lib:~/.local/share/creusot/share/why3find`  | creusot        |

use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
use anyhow::Context as _;

// ── Config ────────────────────────────────────────────────────────────────────

/// Resolved configuration for the `prove` command.
#[derive(Debug)]
pub struct ProveConfig {
    pub run_kani: bool,
    pub run_verus: bool,
    pub run_creusot: bool,

    /// `-p <package>` for Kani.
    pub kani_package: Option<String>,
    /// Extra flags passed verbatim to `cargo kani`.
    pub kani_flags: Vec<String>,
    /// Specific harness to target (Kani only).
    pub kani_harness: Option<String>,

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

    /// Timeout in seconds for each individual backend invocation.
    pub timeout: Option<u64>,

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
    let mut cmd = Command::new("cargo");
    cmd.arg("kani");

    if let Some(pkg) = &config.kani_package {
        cmd.arg("-p").arg(pkg);
    } else {
        anyhow::bail!(
            "No package for Kani — set KANI_PACKAGE or PROVE_PACKAGE in .env, or pass --package"
        );
    }

    if let Some(harness) = &config.kani_harness {
        cmd.arg("--harness").arg(harness);
    }

    for flag in &config.kani_flags {
        cmd.arg(flag);
    }

    execute("cargo kani", cmd, config)
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

    execute("verus", cmd, config)
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

    execute("cargo creusot prove", cmd, config)
}

// ── Shared execute helper ─────────────────────────────────────────────────────

fn execute(label: &str, mut cmd: Command, config: &ProveConfig) -> anyhow::Result<()> {
    if config.dry_run {
        println!("🔍 [dry-run] {label}: {:?}", cmd);
        return Ok(());
    }

    println!("🔬 Running {label}…");

    cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());

    let status = if let Some(secs) = config.timeout {
        // Spawn with a timeout by wrapping in a thread.
        let mut child = cmd
            .spawn()
            .with_context(|| format!("Failed to spawn {label}"))?;
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(secs);
        loop {
            match child.try_wait()? {
                Some(s) => break s,
                None => {
                    if std::time::Instant::now() >= deadline {
                        let _ = child.kill();
                        anyhow::bail!("{label} timed out after {secs}s");
                    }
                    std::thread::sleep(std::time::Duration::from_millis(500));
                }
            }
        }
    } else {
        cmd.status()
            .with_context(|| format!("Failed to execute {label}"))?
    };

    if status.success() {
        println!("✅ {label} passed");
        Ok(())
    } else {
        anyhow::bail!("{label} exited with {status}")
    }
}

