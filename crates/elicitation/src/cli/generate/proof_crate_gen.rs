//! Proof-crate generator: `elicitation generate proof-crate`.
//!
//! Emits a complete, compilable Cargo crate containing all boilerplate
//! scaffolding (`Cargo.toml`, `src/lib.rs`, backend `mod.rs` files) plus
//! every generated proof companion file for every VSM in the source crate.
//!
//! This eliminates the hand-written minefield that users previously had to
//! maintain: `Cargo.toml`, `src/lib.rs` (including the all-important
//! `#![allow(unexpected_cfgs)]`), and six `mod.rs` shims — all derived
//! mechanically from the VSM descriptors and the source-crate name.

use std::path::{Path, PathBuf};

use anyhow::Context;

use crate::cli::generate::{
    ImportStyle, creusot_gen, find_crate_name, find_crate_root, kani_gen,
    scanner::VsmDescriptor, verus_gen,
};

// ─── Public API ───────────────────────────────────────────────────────────────

/// Generate a complete proof companion crate at `out_dir`.
///
/// `source_crate_path` is the root (or any directory inside) the crate being
/// verified.  `out_dir` is the root of the proof crate to create or update.
/// `proof_crate_name` overrides the generated crate name; by default it is
/// `{source_crate_name}_proofs`.
///
/// All files that already exist are overwritten; new directories are created
/// as needed.
#[tracing::instrument(skip(vsms), fields(vsms = vsms.len(), out = %out_dir.display()))]
pub fn generate_proof_crate(
    vsms: &[VsmDescriptor],
    source_crate_path: &Path,
    out_dir: &Path,
    proof_crate_name: Option<&str>,
) -> anyhow::Result<()> {
    let source_name = find_crate_name(source_crate_path);

    let proof_name = proof_crate_name
        .map(str::to_string)
        .unwrap_or_else(|| format!("{source_name}_proofs"));

    let source_root = find_crate_root(source_crate_path).with_context(|| {
        format!(
            "No Cargo.toml found above {}",
            source_crate_path.display()
        )
    })?;

    let source_rel = relative_path(out_dir, &source_root)?;
    let (elicitation_rel, creusot_std_rel) = find_elicitation_paths(&source_root, out_dir)?;
    let kani_skip = read_kani_skip_features(&source_root);
    let source_dep_extras = build_source_dep_extras(&source_root, &kani_skip);

    // ── Cargo.toml ───────────────────────────────────────────────────────────
    write_file(
        out_dir,
        "Cargo.toml",
        &render_cargo_toml(
            &proof_name,
            &source_name,
            &source_rel,
            elicitation_rel.as_deref(),
            creusot_std_rel.as_deref(),
            &source_dep_extras,
        ),
    )?;

    // ── src/lib.rs ───────────────────────────────────────────────────────────
    write_file(
        &out_dir.join("src"),
        "lib.rs",
        &render_lib_rs(&source_name),
    )?;

    // ── src/{kani,creusot,verus}/mod.rs ──────────────────────────────────────
    for backend in ["kani", "creusot", "verus"] {
        write_file(
            &out_dir.join("src").join(backend),
            "mod.rs",
            &render_backend_mod_rs(),
        )?;
    }

    // ── Generated proof files + generated/mod.rs per backend ─────────────────
    write_kani_files(vsms, source_crate_path, out_dir)?;
    write_creusot_files(vsms, source_crate_path, out_dir)?;
    write_verus_files(vsms, source_crate_path, out_dir)?;

    // ── why3find.json at workspace root (needed by `cargo creusot prove`) ────
    let workspace_root = find_workspace_root(&source_root).unwrap_or(source_root);
    write_why3find_json(&workspace_root)?;

    Ok(())
}

// ─── Per-backend writers ──────────────────────────────────────────────────────

#[tracing::instrument(skip(vsms, source, out))]
fn write_kani_files(vsms: &[VsmDescriptor], source: &Path, out: &Path) -> anyhow::Result<()> {
    let dir = out.join("src/kani/generated");
    for vsm in vsms {
        let filename = format!("{}.rs", machine_to_filename(&vsm.machine));
        let content =
            kani_gen::generate_kani_file_with_style(vsm, source, ImportStyle::ExternalCrate)
                .map_err(std::io::Error::other)?;
        write_file(&dir, &filename, &content)?;
    }
    write_file(
        &dir,
        "mod.rs",
        &render_generated_mod_rs(vsms, "kani", false),
    )?;
    Ok(())
}

#[tracing::instrument(skip(vsms, source, out))]
fn write_creusot_files(vsms: &[VsmDescriptor], source: &Path, out: &Path) -> anyhow::Result<()> {
    let dir = out.join("src/creusot/generated");
    for vsm in vsms {
        let filename = format!("{}.rs", machine_to_filename(&vsm.machine));
        let content = creusot_gen::generate_creusot_file_with_style(
            vsm,
            source,
            ImportStyle::ExternalCrate,
        )
        .map_err(std::io::Error::other)?;
        write_file(&dir, &filename, &content)?;
    }
    write_file(
        &dir,
        "elicitation_specs.rs",
        &creusot_gen::generate_creusot_shared_file(),
    )?;
    write_file(
        &dir,
        "mod.rs",
        &render_generated_mod_rs(vsms, "creusot", true),
    )?;
    Ok(())
}

#[tracing::instrument(skip(vsms, source, out))]
fn write_verus_files(vsms: &[VsmDescriptor], source: &Path, out: &Path) -> anyhow::Result<()> {
    let dir = out.join("src/verus/generated");
    for vsm in vsms {
        let filename = format!("{}.rs", machine_to_filename(&vsm.machine));
        let content = verus_gen::generate_verus_file(vsm, source).map_err(std::io::Error::other)?;
        write_file(&dir, &filename, &content)?;
    }
    write_file(
        &dir,
        "mod.rs",
        &render_generated_mod_rs(vsms, "verus", false),
    )?;
    Ok(())
}

// ─── Content renderers ────────────────────────────────────────────────────────

fn render_cargo_toml(
    proof_name: &str,
    source_name: &str,
    source_rel: &Path,
    elicitation_rel: Option<&Path>,
    creusot_std_rel: Option<&Path>,
    source_dep_extras: &str,
) -> String {
    let elicitation_dep = match elicitation_rel {
        Some(p) => format!(r#"elicitation = {{ path = "{}" }}"#, p.display()),
        None => {
            tracing::warn!("elicitation path not found; proof crate will lack it");
            "# elicitation = { path = \"<path-to-elicitation>\" }  # TODO: set this".to_string()
        }
    };
    let creusot_std_dep = match creusot_std_rel {
        Some(p) => format!(r#"creusot-std = {{ path = "{}" }}"#, p.display()),
        None => {
            tracing::warn!("creusot-std vendor path not found; proof crate will lack it");
            "# creusot-std = { path = \"<path-to-creusot-std>\" }  # TODO: set this".to_string()
        }
    };
    format!(
        r#"# AUTO-GENERATED by `elicitation generate proof-crate` — DO NOT EDIT
[package]
name = "{proof_name}"
version = "0.1.0"
edition = "2021"

[features]
kani    = []
creusot = []
verus   = []

[dependencies]
{source_name} = {{ path = "{source_rel}"{source_dep_extras} }}
{elicitation_dep}
{creusot_std_dep}
"#,
        source_rel = source_rel.display()
    )
}

fn render_lib_rs(source_name: &str) -> String {
    format!(
        r#"//! Formal verification proof harnesses for `{source_name}`.
//!
//! AUTO-GENERATED by `elicitation generate proof-crate` — DO NOT EDIT.
//!
//! Regenerate with:
//! ```sh
//! elicitation generate proof-crate --crate-path <source-crate> --out <this-dir>
//! ```
#![allow(unexpected_cfgs)]

#[cfg(kani)]
pub mod kani;

#[cfg(creusot)]
pub mod creusot;

#[cfg(verus)]
pub mod verus;
"#
    )
}

fn render_backend_mod_rs() -> String {
    "// AUTO-GENERATED by `elicitation generate proof-crate` — DO NOT EDIT\n\npub mod generated;\n"
        .to_string()
}

fn render_generated_mod_rs(
    vsms: &[VsmDescriptor],
    cfg_key: &str,
    include_elicitation_specs: bool,
) -> String {
    let mut out = format!(
        "// AUTO-GENERATED by `elicitation generate proof-crate` — DO NOT EDIT\n\n"
    );
    for vsm in vsms {
        let name = machine_to_filename(&vsm.machine);
        out.push_str(&format!("#[cfg({cfg_key})]\npub mod {name};\n"));
    }
    if include_elicitation_specs {
        out.push_str(&format!("#[cfg({cfg_key})]\npub mod elicitation_specs;\n"));
    }
    out
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

/// Locate the elicitation crate and vendored `creusot-std` by reading the
/// source crate's `Cargo.toml` and following the `elicitation` path dep.
///
/// Returns `(elicitation_rel, creusot_std_rel)` — both as paths relative to
/// `from_dir` suitable for embedding in the generated `Cargo.toml`.
fn find_elicitation_paths(
    source_crate_root: &Path,
    from_dir: &Path,
) -> anyhow::Result<(Option<PathBuf>, Option<PathBuf>)> {
    let cargo_toml_path = source_crate_root.join("Cargo.toml");
    let cargo_toml_text = match std::fs::read_to_string(&cargo_toml_path) {
        Ok(t) => t,
        Err(_) => return Ok((None, None)),
    };

    let table: toml::Table = match cargo_toml_text.parse() {
        Ok(t) => t,
        Err(_) => return Ok((None, None)),
    };

    let elicitation_abs = table
        .get("dependencies")
        .and_then(|d| d.get("elicitation"))
        .and_then(|e| e.get("path"))
        .and_then(|p| p.as_str())
        .map(|p| {
            let raw = Path::new(p);
            if raw.is_absolute() {
                raw.to_path_buf()
            } else {
                source_crate_root.join(raw)
            }
        });

    let Some(elicitation_crate_dir) = elicitation_abs else {
        return Ok((None, None));
    };

    let elicitation_rel = relative_path(from_dir, &elicitation_crate_dir).ok();

    // Walk up from the elicitation crate dir to find `vendor/creusot-std`.
    let mut candidate = elicitation_crate_dir.as_path();
    let creusot_std_rel = loop {
        let vendor = candidate.join("vendor").join("creusot-std");
        if vendor.exists() {
            break relative_path(from_dir, &vendor).ok();
        }
        match candidate.parent() {
            Some(p) => candidate = p,
            None => {
                tracing::warn!(
                    elicitation_crate = %elicitation_crate_dir.display(),
                    "vendor/creusot-std not found walking up from elicitation crate"
                );
                break None;
            }
        }
    };

    Ok((elicitation_rel, creusot_std_rel))
}

/// Walk up from `crate_root` to find the workspace root: the highest ancestor
/// directory whose `Cargo.toml` contains a `[workspace]` table.
///
/// Falls back to `crate_root` itself if no workspace is found.
fn find_workspace_root(crate_root: &Path) -> Option<PathBuf> {
    let mut workspace_root: Option<PathBuf> = None;
    let mut dir: &Path = crate_root;
    loop {
        let manifest = dir.join("Cargo.toml");
        if manifest.exists() {
            if let Ok(text) = std::fs::read_to_string(&manifest) {
                if text.contains("[workspace]") {
                    workspace_root = Some(dir.to_path_buf());
                }
            }
        }
        match dir.parent() {
            Some(p) => dir = p,
            None => break,
        }
    }
    workspace_root
}

/// Write `why3find.json` to `dir` if it does not already exist.
///
/// This file is required by `cargo creusot prove`; without it why3find
/// refuses to run.  The content mirrors what `cargo creusot init` generates.
fn write_why3find_json(dir: &Path) -> anyhow::Result<()> {
    let path = dir.join("why3find.json");
    if path.exists() {
        println!("Skipped (exists): {}", path.display());
        return Ok(());
    }
    let content = r#"{
  "fast": 0.2,
  "time": 1,
  "depth": 6,
  "packages": [ "creusot" ],
  "provers": [ "alt-ergo", "z3", "cvc5", "cvc4" ],
  "tactics": [ "compute_specified", "split_vc" ],
  "drivers": [],
  "warnoff": [ "unused_variable", "axiom_abstract" ],
  "profile": [
    { "prover": "cvc5@1.3.1", "size": 45, "time": 0.473 },
    { "prover": "z3@4.15.3", "size": 30, "time": 0.165 },
    { "prover": "alt-ergo@2.6.2", "size": 16, "time": 0.232 },
    { "prover": "cvc4@1.8", "size": 40, "time": 0.506 }
  ]
}
"#;
    std::fs::write(&path, content).with_context(|| format!("write {}", path.display()))?;
    println!("Written: {}", path.display());
    Ok(())
}

/// Read `[package.metadata.elicitation] kani_skip_features` from the source
/// crate's `Cargo.toml`.  Returns an empty vec if absent or unparseable.
fn read_kani_skip_features(source_crate_root: &Path) -> Vec<String> {
    let cargo_toml_path = source_crate_root.join("Cargo.toml");
    let text = match std::fs::read_to_string(&cargo_toml_path) {
        Ok(t) => t,
        Err(_) => return vec![],
    };
    let table: toml::Table = match text.parse() {
        Ok(t) => t,
        Err(_) => return vec![],
    };
    table
        .get("package")
        .and_then(|p| p.get("metadata"))
        .and_then(|m| m.get("elicitation"))
        .and_then(|e| e.get("kani_skip_features"))
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

/// Read the `[features] default` list from the source crate's `Cargo.toml`.
/// Returns an empty vec if absent or unparseable.
fn read_default_features(source_crate_root: &Path) -> Vec<String> {
    let cargo_toml_path = source_crate_root.join("Cargo.toml");
    let text = match std::fs::read_to_string(&cargo_toml_path) {
        Ok(t) => t,
        Err(_) => return vec![],
    };
    let table: toml::Table = match text.parse() {
        Ok(t) => t,
        Err(_) => return vec![],
    };
    table
        .get("features")
        .and_then(|f| f.get("default"))
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

/// Build the extra fragment for the source dep line.
///
/// If `kani_skip_features` is non-empty, emit `default-features = false` plus
/// all declared `default` features minus the ones to skip.  This prevents the
/// proof crate from pulling in Kani-incompatible transitive deps that the
/// source crate gates behind optional features.
///
/// If the skip list is empty the returned string is empty (normal dep).
fn build_source_dep_extras(source_crate_root: &Path, kani_skip: &[String]) -> String {
    if kani_skip.is_empty() {
        return String::new();
    }
    let defaults = read_default_features(source_crate_root);
    let remaining: Vec<&String> = defaults.iter().filter(|f| !kani_skip.contains(f)).collect();
    tracing::info!(
        skip = ?kani_skip,
        defaults = ?defaults,
        remaining = ?remaining,
        "Applying kani_skip_features to source dep"
    );
    if remaining.is_empty() {
        r#", default-features = false"#.to_string()
    } else {
        let list = remaining
            .iter()
            .map(|f| format!(r#""{f}""#))
            .collect::<Vec<_>>()
            .join(", ");
        format!(r#", default-features = false, features = [{list}]"#)
    }
}


///
/// `CombatMachine` → `combat`, `ArchiveNavMachine` → `archive_nav`.
fn machine_to_filename(machine: &str) -> String {
    machine
        .trim_end_matches("Machine")
        .chars()
        .fold(String::new(), |mut acc, c| {
            if c.is_uppercase() && !acc.is_empty() {
                acc.push('_');
            }
            acc.push(c.to_ascii_lowercase());
            acc
        })
}

/// Write `content` to `dir/filename`, creating `dir` if needed.
///
/// `.rs` files are formatted with `rustfmt --edition=2021` afterwards.
fn write_file(dir: &Path, filename: &str, content: &str) -> anyhow::Result<()> {
    std::fs::create_dir_all(dir)
        .with_context(|| format!("create_dir_all {}", dir.display()))?;
    let path = dir.join(filename);
    std::fs::write(&path, content).with_context(|| format!("write {}", path.display()))?;
    if path.extension().and_then(|e| e.to_str()) == Some("rs") {
        let _ = std::process::Command::new("rustfmt")
            .arg("--edition=2021")
            .arg(&path)
            .status();
    }
    println!("Written: {}", path.display());
    Ok(())
}

/// Compute the relative path from `from_dir` to `to_dir`.
///
/// Both paths are absolutized before comparison so that relative inputs
/// (e.g. `"."`) work correctly.
fn relative_path(from_dir: &Path, to_dir: &Path) -> anyhow::Result<PathBuf> {
    let from_abs = absolutize(from_dir);
    let to_abs = absolutize(to_dir);

    let from_parts: Vec<_> = from_abs.components().collect();
    let to_parts: Vec<_> = to_abs.components().collect();

    let common = from_parts
        .iter()
        .zip(to_parts.iter())
        .take_while(|(a, b)| a == b)
        .count();

    let mut result = PathBuf::new();
    for _ in 0..(from_parts.len() - common) {
        result.push("..");
    }
    for part in &to_parts[common..] {
        result.push(part.as_os_str());
    }
    if result.as_os_str().is_empty() {
        result.push(".");
    }
    Ok(result)
}

fn absolutize(path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(path)
    }
}
