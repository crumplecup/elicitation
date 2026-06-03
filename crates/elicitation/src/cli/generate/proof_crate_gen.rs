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

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use anyhow::Context;

use crate::cli::generate::{
    ImportStyle, creusot_gen, find_crate_name, find_crate_root, kani_gen, scanner::VsmDescriptor,
    type_resolver::derive_module_path, verus_gen,
};

// ─── Public API ───────────────────────────────────────────────────────────────

/// Generate a complete proof companion crate at `out_dir`.
///
/// `source_crate_paths` lists one or more source crate roots (or directories
/// inside them) to scan for VSMs.  Multiple paths are needed when VSMs live
/// in separate workspace member crates (e.g. a game suite with one crate per
/// game).  All source crates must belong to the same workspace.
///
/// `proof_crate_name` overrides the generated crate name; by default it is
/// `{first_source}_proofs`.  When supplying multiple source paths the name
/// should always be set explicitly via `--crate-name`.
///
/// All files that already exist are overwritten; new directories are created
/// as needed.
#[tracing::instrument(skip(vsms), fields(vsms = vsms.len(), out = %out_dir.display()))]
pub fn generate_proof_crate(
    vsms: &[VsmDescriptor],
    source_crate_paths: &[&Path],
    out_dir: &Path,
    proof_crate_name: Option<&str>,
) -> anyhow::Result<()> {
    anyhow::ensure!(
        !source_crate_paths.is_empty(),
        "at least one --crate-path must be provided"
    );

    // ── Per-source metadata ───────────────────────────────────────────────────
    struct SourceInfo {
        name: String,
        root: PathBuf,
        dep_line: String,
    }

    let mut sources: Vec<SourceInfo> = Vec::new();
    for &path in source_crate_paths {
        let name = find_crate_name(path);
        let root = find_crate_root(path)
            .with_context(|| format!("No Cargo.toml found above {}", path.display()))?;
        // Workspace dep lookup deferred until workspace_deps is available below.
        sources.push(SourceInfo {
            name,
            root,
            dep_line: String::new(), // filled in after workspace_deps resolved
        });
    }

    let first_root = sources[0].root.clone();
    let proof_name = proof_crate_name
        .map(str::to_string)
        .unwrap_or_else(|| format!("{}_proofs", sources[0].name));

    let (elicitation_rel, creusot_std_rel) = find_elicitation_paths(&first_root, out_dir)?;
    let workspace_root = find_workspace_root(&first_root);
    let workspace_pkg_fields = workspace_root
        .as_deref()
        .map(read_workspace_package_fields)
        .unwrap_or_default();
    let workspace_deps = workspace_root
        .as_deref()
        .map(read_workspace_dep_names)
        .unwrap_or_default();
    let source_version = read_crate_version(&first_root);

    // Build dep lines now that workspace_deps is known.
    let in_workspace = !workspace_pkg_fields.is_empty() || !workspace_deps.is_empty();
    for src in &mut sources {
        let kani_skip = read_kani_skip_features(&src.root);
        let dep_extras = build_source_dep_extras(&src.root, &kani_skip);
        src.dep_line = if in_workspace && workspace_deps.contains(&src.name) {
            format!("{} = {{ workspace = true{dep_extras} }}", src.name)
        } else {
            let rel = relative_path(out_dir, &src.root)?;
            format!(
                "{} = {{ path = \"{}\"{dep_extras} }}",
                src.name,
                rel.display()
            )
        };
    }

    let source_names: Vec<&str> = sources.iter().map(|s| s.name.as_str()).collect();
    // Deduplicate: multiple paths may resolve to the same crate (e.g. src/vsm and src/ui/vsm).
    // Keep only the first dep line per unique crate name.
    let mut seen_names: std::collections::HashSet<&str> = std::collections::HashSet::new();
    let source_dep_lines: Vec<&str> = sources
        .iter()
        .filter(|s| seen_names.insert(s.name.as_str()))
        .map(|s| s.dep_line.as_str())
        .collect();
    let extra_workspace_deps = collect_extra_deps(vsms, &source_names, &workspace_deps);

    // ── Cargo.toml ───────────────────────────────────────────────────────────
    write_file(
        out_dir,
        "Cargo.toml",
        &render_cargo_toml(
            &proof_name,
            &sources[0].name,
            &source_dep_lines,
            elicitation_rel.as_deref(),
            creusot_std_rel.as_deref(),
            &workspace_pkg_fields,
            &workspace_deps,
            source_version.as_deref(),
            &extra_workspace_deps,
        ),
    )?;

    // ── src/lib.rs ───────────────────────────────────────────────────────────
    write_file(&out_dir.join("src"), "lib.rs", &render_lib_rs(&proof_name))?;

    // ── src/{kani,creusot,verus}/mod.rs ──────────────────────────────────────
    for backend in ["kani", "creusot", "verus"] {
        write_file(
            &out_dir.join("src").join(backend),
            "mod.rs",
            &render_backend_mod_rs(),
        )?;
    }

    // ── Generated proof files + generated/mod.rs per backend ─────────────────
    // crate_root is derived from vsm.source_file inside each generator, so a
    // placeholder is fine here.
    let placeholder = Path::new(".");
    write_kani_files(vsms, placeholder, out_dir)?;
    write_creusot_files(vsms, placeholder, out_dir)?;
    write_verus_files(vsms, placeholder, out_dir)?;

    // ── kani reexports patched into each source crate's module tree ─────────
    for src in &sources {
        let src_vsms: Vec<&VsmDescriptor> = vsms
            .iter()
            .filter(|v| find_crate_name(&v.source_file) == src.name)
            .collect();
        if src_vsms.iter().any(|v| v.invariant.is_some()) {
            patch_module_tree_kani_reexports(&src.root, &src_vsms)?;
        }
    }

    // ── why3find.json at workspace root (needed by `cargo creusot prove`) ────
    let ws_root = workspace_root.unwrap_or_else(|| first_root.clone());
    write_why3find_json(&ws_root)?;

    // ── README.md (scaffold once; skip if already customized) ────────────────
    write_readme_if_absent(out_dir, &proof_name, &sources[0].name)?;

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
        let content =
            creusot_gen::generate_creusot_file_with_style(vsm, source, ImportStyle::ExternalCrate)
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

// ─── kani_reexports helpers ───────────────────────────────────────────────────

/// Patch the module tree of a source crate to re-export all `_kani_contracted`
/// transition wrappers and invariant predicates up to the crate root.
///
/// Each module in the path from a VSM file to the crate root gets a sentinel
/// block added/updated in its `mod.rs` (or `lib.rs` for depth 0) with:
///
/// ```rust
/// pub use next_mod::fn_kani_contracted;
/// ```
///
/// This follows the standard `pub use` chain already used for public API items —
/// only the contracted/invariant fns are added, no `pub mod` is introduced.
///
/// For `lib.rs` specifically: if a symbol is already exported outside the
/// sentinel block, it is skipped to avoid duplicate definition errors.
///
/// Also removes any orphaned `src/kani_reexports.rs` from the previous approach.
fn patch_module_tree_kani_reexports(
    source_root: &Path,
    vsms: &[&VsmDescriptor],
) -> anyhow::Result<()> {
    use std::collections::{BTreeMap, BTreeSet};

    const BEGIN: &str = "// BEGIN ELICITATION KANI REEXPORTS — DO NOT EDIT";
    const END: &str = "// END ELICITATION KANI REEXPORTS";

    // mod_file_path → sorted set of "pub use next_mod::fn;" lines
    let mut patches: BTreeMap<PathBuf, BTreeSet<String>> = BTreeMap::new();

    for vsm in vsms {
        let Some(inv) = &vsm.invariant else { continue };
        let Some(inv_fn) = &inv.kani_fn else { continue };

        // All items are re-exported unconditionally: contracted fns have both cfg
        // branches inside their compat mods; the invariant fn is a plain pub fn
        // (no cfg gate on the declaration), so it is always available.
        let items: Vec<String> = vsm
            .transitions
            .iter()
            .map(|t| format!("{t}_kani_contracted"))
            .chain(std::iter::once(inv_fn.clone()))
            .collect();

        let module_path = derive_module_path(&vsm.source_file);
        let segments: Vec<&str> = module_path.split("::").collect();

        // Level 0 → lib.rs:                  pub use seg[0]::item
        // Level 1 → src/seg[0]/mod.rs:       pub use seg[1]::item
        // Level L → src/seg[0]/../mod.rs:    pub use seg[L]::item
        for level in 0..segments.len() {
            let mod_file = if level == 0 {
                source_root.join("src/lib.rs")
            } else {
                let mut p = source_root.join("src");
                for &part in &segments[..level] {
                    p = p.join(part);
                }
                p.join("mod.rs")
            };

            let next_mod = segments[level];
            for item in &items {
                let line = format!("pub use {next_mod}::{item};");
                patches.entry(mod_file.clone()).or_default().insert(line);
            }
        }
    }

    // Remove orphaned kani_reexports.rs from the previous generation approach.
    let orphan = source_root.join("src/kani_reexports.rs");
    if orphan.exists() {
        std::fs::remove_file(&orphan)
            .with_context(|| format!("Cannot remove orphan {}", orphan.display()))?;
        tracing::info!("Removed orphan {}", orphan.display());
    }

    // Apply patches to each mod file.
    for (mod_file, new_lines) in &patches {
        let existing = std::fs::read_to_string(mod_file)
            .with_context(|| format!("Cannot read {}", mod_file.display()))?;

        // For lib.rs: skip symbols already exported outside the sentinel block.
        let outside_sentinel = if existing.contains(BEGIN) {
            let start = existing.find(BEGIN).unwrap();
            let end = existing.find(END).unwrap() + END.len();
            format!("{}{}", &existing[..start], &existing[end..])
        } else {
            existing.clone()
        };

        let filtered: Vec<String> = new_lines
            .iter()
            .filter(|line| {
                // Extract symbol: "pub use mod::symbol;" → "symbol"
                let sym = line.trim_end_matches(';').rsplit("::").next().unwrap_or("");
                // Skip if the symbol already appears anywhere outside the sentinel
                // (handles both single-line and multi-line pub use blocks).
                !outside_sentinel.contains(sym)
            })
            .cloned()
            .collect();

        if filtered.is_empty() {
            continue;
        }

        let inner = filtered.join("\n");
        let block = format!("{BEGIN}\n{inner}\n{END}\n");

        let new_content = if existing.contains(BEGIN) {
            let start = existing.find(BEGIN).unwrap();
            let end = existing.find(END).unwrap() + END.len();
            // Preserve any trailing newline after END marker.
            let after = &existing[end..];
            let after = after.strip_prefix('\n').unwrap_or(after);
            format!("{}{}{}", &existing[..start], block, after)
        } else {
            format!("{}\n{}", existing.trim_end(), block)
        };

        std::fs::write(mod_file, &new_content)
            .with_context(|| format!("Cannot write {}", mod_file.display()))?;
        let _ = std::process::Command::new("rustfmt")
            .arg("--edition=2024")
            .arg(mod_file)
            .status();
        tracing::info!("Patched {}", mod_file.display());
    }

    Ok(())
}

// ─── Content renderers ────────────────────────────────────────────────────────

fn render_cargo_toml(
    proof_name: &str,
    primary_source: &str,
    source_dep_lines: &[&str],
    elicitation_rel: Option<&Path>,
    creusot_std_rel: Option<&Path>,
    workspace_pkg_fields: &[String],
    workspace_deps: &[String],
    source_version: Option<&str>,
    extra_workspace_deps: &[String],
) -> String {
    let has_wpkg = |field: &str| workspace_pkg_fields.iter().any(|f| f == field);

    let version_field = if has_wpkg("version") {
        "version.workspace = true".to_string()
    } else {
        format!("version = \"{}\"", source_version.unwrap_or("0.1.0"))
    };
    let edition_field = if has_wpkg("edition") {
        "edition.workspace = true".to_string()
    } else {
        "edition = \"2024\"".to_string()
    };

    // description is required by crates.io; inherit from workspace or synthesize
    let description_field = if has_wpkg("description") {
        "description.workspace = true".to_string()
    } else {
        format!(
            "description = \"Auto-generated Kani, Creusot, and Verus proof harnesses for {}, produced by elicitation\"",
            primary_source
        )
    };

    // Optional publishable fields — inherit from workspace when present.
    // readme is intentionally excluded: we always emit a local readme = "README.md"
    // so the proof crate's README reflects its own purpose, not the workspace's.
    let optional_fields: &[&str] = &[
        "license",
        "license-file",
        "repository",
        "authors",
        "homepage",
        "documentation",
        "keywords",
        "categories",
    ];
    let inherited: Vec<String> = optional_fields
        .iter()
        .filter(|&&f| has_wpkg(f))
        .map(|&f| format!("{f}.workspace = true"))
        .collect();

    let pkg_fields = {
        let mut parts = vec![version_field, edition_field, description_field];
        parts.extend(inherited);
        parts.push("readme = \"README.md\"".to_string());
        parts.join("\n")
    };

    let in_workspace = !workspace_pkg_fields.is_empty() || !workspace_deps.is_empty();

    let elicitation_dep = if in_workspace && workspace_deps.contains(&"elicitation".to_string()) {
        "elicitation = { workspace = true }".to_string()
    } else {
        match elicitation_rel {
            Some(p) => format!("elicitation = {{ path = \"{}\" }}", p.display()),
            None => {
                tracing::warn!("elicitation path not found; proof crate will lack it");
                "# elicitation = { path = \"<path-to-elicitation>\" }  # TODO: set this".to_string()
            }
        }
    };

    let creusot_std_dep = if in_workspace && workspace_deps.contains(&"creusot-std".to_string()) {
        "creusot-std = { workspace = true, optional = true }".to_string()
    } else {
        match creusot_std_rel {
            Some(p) => format!(
                "creusot-std = {{ path = \"{}\", optional = true }}",
                p.display()
            ),
            None => {
                tracing::warn!("creusot-std vendor path not found; proof crate will lack it");
                "# creusot-std = { path = \"<path-to-creusot-std>\", optional = true }  # TODO: set this"
                    .to_string()
            }
        }
    };

    let sources_str = source_dep_lines.join("\n");

    let extra_deps_str = if extra_workspace_deps.is_empty() {
        String::new()
    } else {
        extra_workspace_deps
            .iter()
            .map(|name| format!("{name} = {{ workspace = true }}\n"))
            .collect::<String>()
    };

    format!(
        "# AUTO-GENERATED by `elicitation generate proof-crate` — DO NOT EDIT\n\
[package]\n\
name = \"{proof_name}\"\n\
{pkg_fields}\n\
\n\
[features]\n\
kani    = []\n\
creusot = [\"dep:creusot-std\"]\n\
verus   = []\n\
\n\
[lints.rust]\n\
unexpected_cfgs = {{ level = \"warn\", check-cfg = [\'cfg(creusot)\', \'cfg(kani)\', \'cfg(verus)\'] }}\n\
\n\
[dependencies]\n\
{sources_str}\n\
{elicitation_dep}\n\
{creusot_std_dep}\n\
{extra_deps_str}"
    )
}

fn render_lib_rs(proof_name: &str) -> String {
    format!(
        r#"//! Formal verification proof harnesses for `{proof_name}`.
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
    let mut out =
        format!("// AUTO-GENERATED by `elicitation generate proof-crate` — DO NOT EDIT\n\n");
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

/// Read all dependency names from `[workspace.dependencies]` in the workspace
/// root `Cargo.toml`.  Returns an empty vec if the section is absent.
fn read_workspace_dep_names(workspace_root: &Path) -> Vec<String> {
    let path = workspace_root.join("Cargo.toml");
    let text = match std::fs::read_to_string(&path) {
        Ok(t) => t,
        Err(_) => return vec![],
    };
    let table: toml::Table = match text.parse() {
        Ok(t) => t,
        Err(_) => return vec![],
    };
    table
        .get("workspace")
        .and_then(|w| w.get("dependencies"))
        .and_then(|d| d.as_table())
        .map(|t| t.keys().cloned().collect())
        .unwrap_or_default()
}

/// Read all field names from `[workspace.package]` in the workspace root
/// `Cargo.toml`.  Returns an empty vec if the section is absent.
fn read_workspace_package_fields(workspace_root: &Path) -> Vec<String> {
    let path = workspace_root.join("Cargo.toml");
    let text = match std::fs::read_to_string(&path) {
        Ok(t) => t,
        Err(_) => return vec![],
    };
    let table: toml::Table = match text.parse() {
        Ok(t) => t,
        Err(_) => return vec![],
    };
    table
        .get("workspace")
        .and_then(|w| w.get("package"))
        .and_then(|p| p.as_table())
        .map(|t| t.keys().cloned().collect())
        .unwrap_or_default()
}

/// Read the `version` field from a crate's own `[package]` section.
fn read_crate_version(crate_root: &Path) -> Option<String> {
    let path = crate_root.join("Cargo.toml");
    let text = std::fs::read_to_string(&path).ok()?;
    let table: toml::Table = text.parse().ok()?;
    table
        .get("package")
        .and_then(|p| p.get("version"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
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

/// Write `README.md` to `dir` if it does not already exist.
///
/// On first generation the stub README explains that this is an auto-generated
/// proof crate and how to regenerate it.  On subsequent runs it is left alone
/// so users can customize it freely.
fn write_readme_if_absent(dir: &Path, proof_name: &str, primary_source: &str) -> anyhow::Result<()> {
    let path = dir.join("README.md");
    if path.exists() {
        println!("Skipped (exists): {}", path.display());
        return Ok(());
    }
    let content = render_readme(proof_name, primary_source);
    std::fs::write(&path, content).with_context(|| format!("write {}", path.display()))?;
    println!("Written: {}", path.display());
    Ok(())
}

fn render_readme(proof_name: &str, primary_source: &str) -> String {
    format!(
        r#"# {proof_name}

Kani, Creusot, and Verus proof harnesses for [`{primary_source}`],
generated by [elicitation](https://crates.io/crates/elicitation).

This README was scaffolded on first generation and can be freely edited —
subsequent runs of `elicitation generate proof-crate` will not overwrite it.

## Contents

| Directory | Description |
|-----------|-------------|
| `src/kani/generated/` | Kani proof harnesses, one per VSM transition |
| `src/creusot/generated/` | Creusot logic predicates and extern specs |
| `src/verus/generated/` | Verus proof harnesses |

## Running proofs

```sh
# Kani
elicitation prove --kani --csv

# Creusot
elicitation prove --creusot --csv

# Verus
elicitation prove --verus --csv
```

## Regenerating

```sh
elicitation generate proof-crate \
    --crate-path <path-to-{primary_source}> \
    --crate-name {proof_name} \
    --out <path-to-this-crate>
```
"#
    )
}
"#
    )
}


/// return the names of any external crates that appear in `workspace_deps` but
/// are not already emitted by `render_cargo_toml` as explicit deps.
///
/// This handles the case where VSM source files (and thus their generated
/// proof companions) import types from workspace crates other than the source
/// crate itself (e.g. `use elicit_server::gaap::...`).
fn collect_extra_deps(
    vsms: &[VsmDescriptor],
    source_names: &[&str],
    workspace_deps: &[String],
) -> Vec<String> {
    // Crates already emitted explicitly by render_cargo_toml.
    let always_handled: &[&str] = &[
        "crate",
        "self",
        "super",
        "std",
        "core",
        "alloc",
        "elicitation",
        "creusot_std",
        "creusot-std",
        "kani",
        "creusot",
        "verus",
        "verus_builtin",
    ];

    let mut found: BTreeSet<String> = BTreeSet::new();

    for vsm in vsms {
        let src = match std::fs::read_to_string(&vsm.source_file) {
            Ok(s) => s,
            Err(_) => continue,
        };
        let syntax = match syn::parse_file(&src) {
            Ok(f) => f,
            Err(_) => continue,
        };
        for item in &syntax.items {
            if let syn::Item::Use(u) = item {
                if let Some(root) = use_tree_root_crate(&u.tree) {
                    let cargo_name = root.replace('_', "-");
                    let is_source = source_names.contains(&root.as_str())
                        || source_names
                            .iter()
                            .any(|n| n.replace('_', "-") == cargo_name);
                    let skip = is_source
                        || always_handled.contains(&root.as_str())
                        || always_handled.contains(&cargo_name.as_str());
                    if !skip {
                        found.insert(root);
                    }
                }
            }
        }
    }

    found
        .into_iter()
        .filter(|name| {
            let cargo_name = name.replace('_', "-");
            workspace_deps.contains(name) || workspace_deps.contains(&cargo_name)
        })
        .collect()
}

/// Extract the root crate name from a `UseTree`.
///
/// For `use foo::bar::Baz` returns `"foo"`.
/// For `use {foo::X, bar::Y}` returns `None` (group at root level is
/// ambiguous; those are handled by recursing into `use` items individually).
fn use_tree_root_crate(tree: &syn::UseTree) -> Option<String> {
    match tree {
        syn::UseTree::Path(p) => Some(p.ident.to_string()),
        syn::UseTree::Name(n) => Some(n.ident.to_string()),
        syn::UseTree::Rename(r) => Some(r.ident.to_string()),
        syn::UseTree::Glob(_) => None,
        syn::UseTree::Group(_) => None,
    }
}

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
///
/// # Known limitation: uniform feature filtering across backends
///
/// Cargo does not support per-feature-flag dep feature sets on a single
/// dependency entry, so `kani_skip_features` is applied to the source dep
/// unconditionally — it affects Creusot and Verus builds too.
///
/// In practice this is fine for generated harnesses, which only import
/// core VSM types that are always available regardless of optional features.
/// It would break only if a user's Creusot/Verus harnesses import types
/// gated behind a skipped feature.
///
/// ## Proposed alternative (if this becomes a problem)
///
/// Use Cargo's `package = "..."` rename trick to declare the source crate
/// three times under different names, each activated by its backend feature:
///
/// ```toml
/// [features]
/// kani    = ["dep:source_kani"]
/// creusot = ["dep:source_creusot", "dep:creusot-std"]
/// verus   = ["dep:source_verus"]
///
/// [dependencies]
/// source_kani    = { package = "source_crate", optional = true,
///                    default-features = false, features = [...] }
/// source_creusot = { package = "source_crate", optional = true }
/// source_verus   = { package = "source_crate", optional = true }
/// ```
///
/// Downside: generated `use source_crate::...` imports would need to be
/// rewritten to use the renamed dep name, coupling the code generator to
/// the Cargo.toml structure.
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
    std::fs::create_dir_all(dir).with_context(|| format!("create_dir_all {}", dir.display()))?;
    let path = dir.join(filename);
    std::fs::write(&path, content).with_context(|| format!("write {}", path.display()))?;
    if path.extension().and_then(|e| e.to_str()) == Some("rs") {
        let _ = std::process::Command::new("rustfmt")
            .arg("--edition=2024")
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
    // Prefer canonicalize (resolves `..` and symlinks) when the path exists.
    // Fall back to a simple cwd-join for paths that don't exist yet (e.g. a
    // proof-crate output directory that hasn't been created yet).
    if let Ok(canonical) = path.canonicalize() {
        return canonical;
    }
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(path)
    }
}
