//! Precise import resolver for proof companion file generators.
//!
//! [`TypeResolver`] parses a VSM source file with `syn` to build a map from
//! bare type names (e.g. `ArchivePanelState`) to their fully-qualified import
//! paths (e.g. `elicit_server::archive::vsm::ArchivePanelState`).
//!
//! Generators collect the bare names they use, then call
//! [`TypeResolver::grouped_imports`] to get precise `use module::{A, B};`
//! lines instead of blanket glob imports.
//!
//! Three information sources, applied in priority order:
//! 1. `pub use` items in parent `mod.rs` / `lib.rs` files (walked up from the
//!    source file to `src/`) — gives the canonical PUBLIC path for types
//!    re-exported through the module hierarchy (e.g. `vsm::ArchivePanelState`
//!    rather than the private `vsm::panel::ArchivePanelState`).
//! 2. `use` items in the source file itself — maps external-crate names like
//!    `elicit_ui::WcagVerified` and `crate::archive::display::DisplayMode`
//!    (with `crate::` → `{crate_name}::`).
//! 3. `pub struct/enum/type` definitions in the same file — fallback for
//!    locally-defined types that are not re-exported anywhere up the hierarchy.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use syn::{Item, UseTree, Visibility};

/// How generated proof files should refer to items from the source crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportStyle {
    /// Generated files live in a separate crate and should use the source
    /// crate's package name, e.g. `valinoreth::vsm::CombatState`.
    ExternalCrate,
    /// Generated files live inside the source crate and should use `crate::...`
    /// paths so they compile as in-crate modules.
    InCrate,
}

/// Resolves bare type names to fully-qualified import paths by parsing the VSM
/// source file.
///
/// Types in the stdlib prelude and types handled by fixed generator imports
/// (`Established`, `kani_label`, `KaniCompose`) are excluded from the output
/// so the grouped imports list stays minimal.
#[derive(Debug, Default)]
pub struct TypeResolver {
    /// Map from bare name to full import path.
    name_to_path: HashMap<String, String>,
}

impl TypeResolver {
    /// Build a resolver by parsing `source_file` and its ancestor modules.
    ///
    /// `import_style` controls whether `crate::` paths resolve to the source
    /// crate's package name or remain `crate::...` for in-crate proof output.
    ///
    /// Resolution priority (first match wins via `or_insert`):
    /// 1. Parent `mod.rs` / `lib.rs` `pub use` re-exports → canonical public path.
    /// 2. Source file's own `use` items → external crate and sibling module types.
    /// 3. Local `pub struct/enum/type` definitions → fallback for types not re-exported.
    pub fn build(source_file: &Path, crate_name: &str, import_style: ImportStyle) -> Self {
        let mut name_to_path: HashMap<String, String> = HashMap::new();
        let import_root = match import_style {
            ImportStyle::ExternalCrate => crate_name.to_string(),
            ImportStyle::InCrate => "crate".to_string(),
        };

        // Pass 1: walk parent mod.rs / lib.rs files upward, extracting pub use
        // re-exports. This gives the canonical public path for locally-defined
        // types (e.g. `vsm::ArchivePanelState` not the private `vsm::panel::…`).
        scan_parent_modules(source_file, &import_root, &mut name_to_path);

        // Pass 2: parse the source file itself.
        let src = match std::fs::read_to_string(source_file) {
            Ok(s) => s,
            Err(_) => return Self { name_to_path },
        };
        let syntax = match syn::parse_file(&src) {
            Ok(f) => f,
            Err(_) => return Self { name_to_path },
        };

        // Derive the module path for types defined in this file:
        //   `src/archive/vsm.rs`      → `archive::vsm`
        //   `src/archive/vsm/mod.rs`  → `archive::vsm`
        //   `src/lib.rs`              → ``
        let module_path = derive_module_path(source_file);

        for item in &syntax.items {
            match item {
                Item::Use(u) => {
                    flatten_use_tree(&u.tree, "", &import_root, &mut name_to_path);
                }
                Item::Struct(s) if is_pub(&s.vis) => {
                    register_local(
                        &s.ident.to_string(),
                        &import_root,
                        &module_path,
                        &mut name_to_path,
                    );
                }
                Item::Enum(e) if is_pub(&e.vis) => {
                    register_local(
                        &e.ident.to_string(),
                        &import_root,
                        &module_path,
                        &mut name_to_path,
                    );
                }
                Item::Type(t) if is_pub(&t.vis) => {
                    register_local(
                        &t.ident.to_string(),
                        &import_root,
                        &module_path,
                        &mut name_to_path,
                    );
                }
                Item::Fn(f) if is_pub(&f.vis) => {
                    register_local(
                        &f.sig.ident.to_string(),
                        &import_root,
                        &module_path,
                        &mut name_to_path,
                    );
                }
                _ => {}
            }
        }

        Self { name_to_path }
    }

    /// Resolve a bare name to its full import path.
    ///
    /// Returns `None` for stdlib/prelude types and names not found in the
    /// source file.
    pub fn resolve(&self, name: &str) -> Option<&str> {
        if is_prelude_type(name) {
            return None;
        }
        self.name_to_path.get(name).map(String::as_str)
    }

    /// Extract bare identifier names from a Rust type string.
    ///
    /// Splits on every character that cannot appear in an identifier, so
    /// generics, references, and punctuation are handled automatically:
    ///
    /// - `"Vec<SavedQuery>"` → `["Vec", "SavedQuery"]`
    /// - `"Option<&str>"` → `["Option", "str"]`
    /// - `"Established<ArchiveConnectionConsistent>"` →
    ///   `["Established", "ArchiveConnectionConsistent"]`
    pub fn extract_bare_names(ty_str: &str) -> Vec<String> {
        ty_str
            .split(|c: char| !c.is_alphanumeric() && c != '_')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    }

    /// Add every bare name from `ty_str` to `needed`.
    pub fn collect_type(ty_str: &str, needed: &mut BTreeSet<String>) {
        for name in Self::extract_bare_names(ty_str) {
            needed.insert(name);
        }
    }

    /// Given a set of needed bare names, resolve and group them by module.
    ///
    /// Returns sorted `use module::{A, B, C};` strings (or `use module::A;`
    /// for singletons), ready to emit directly as `use` statements.
    ///
    /// Names that cannot be resolved (not in the source file and not stdlib)
    /// are silently omitted — the caller should handle any remaining
    /// unresolvable types separately.
    pub fn grouped_imports(&self, needed: &BTreeSet<String>) -> Vec<String> {
        let mut by_module: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

        for name in needed {
            if let Some(path) = self.resolve(name)
                && let Some(pos) = path.rfind("::")
            {
                let module = path[..pos].to_string();
                let ty = path[pos + 2..].to_string();
                by_module.entry(module).or_default().insert(ty);
            }
            // Top-level names (no `::`) are already in scope — skip.
        }

        let mut result: Vec<String> = Vec::new();
        for (module, names) in &by_module {
            if names.len() == 1 {
                let n = names.iter().next().unwrap();
                result.push(format!("{module}::{n}"));
            } else {
                let list = names.iter().cloned().collect::<Vec<_>>().join(", ");
                result.push(format!("{module}::{{{list}}}"));
            }
        }
        result
    }
}

// ─── Private helpers ──────────────────────────────────────────────────────────

/// Derive the Rust module path from a source file path.
///
/// Locates the `src/` component and converts the remaining path to a
/// `::` separated module path, stripping `.rs` and normalising `mod`/`lib`
/// terminal segments.
pub(crate) fn derive_module_path(source_file: &Path) -> String {
    let mut segs: Vec<String> = Vec::new();
    let mut after_src = false;

    for component in source_file.components() {
        let s = component.as_os_str().to_string_lossy().to_string();
        if after_src {
            segs.push(s);
        } else if s == "src" {
            after_src = true;
        }
    }

    if segs.is_empty() {
        return String::new();
    }

    if let Some(last) = segs.last_mut()
        && last.ends_with(".rs")
    {
        *last = last[..last.len() - 3].to_string();
    }

    // `mod.rs` and `lib.rs` map to the enclosing module, not a sub-module.
    while segs.last().is_some_and(|s| s == "mod" || s == "lib") {
        segs.pop();
    }

    segs.join("::")
}

/// Flatten a `syn::UseTree` into `name → full_path` pairs.
///
/// `crate::` is substituted with `{import_root}::` so paths work from either an
/// external proof crate or in-crate generated module.
fn flatten_use_tree(
    tree: &UseTree,
    prefix: &str,
    import_root: &str,
    out: &mut HashMap<String, String>,
) {
    match tree {
        UseTree::Path(p) => {
            let seg = p.ident.to_string();
            let new_prefix = if seg == "crate" {
                import_root.to_string()
            } else if prefix.is_empty() {
                seg
            } else {
                format!("{prefix}::{seg}")
            };
            flatten_use_tree(&p.tree, &new_prefix, import_root, out);
        }
        UseTree::Name(n) => {
            let name = n.ident.to_string();
            if name != "self" {
                let full = if prefix.is_empty() {
                    name.clone()
                } else {
                    format!("{prefix}::{name}")
                };
                out.entry(name).or_insert(full);
            }
        }
        UseTree::Rename(r) => {
            // `use foo::Bar as Baz` — record alias pointing to the original path.
            let orig = r.ident.to_string();
            let alias = r.rename.to_string();
            let full = if prefix.is_empty() {
                orig
            } else {
                format!("{prefix}::{orig}")
            };
            out.entry(alias).or_insert(full);
        }
        UseTree::Glob(_) => {
            // Cannot enumerate glob imports statically — skip.
        }
        UseTree::Group(g) => {
            for item in &g.items {
                flatten_use_tree(item, prefix, import_root, out);
            }
        }
    }
}

/// Register a locally-defined `pub` type at its canonical path.
fn register_local(
    name: &str,
    import_root: &str,
    module_path: &str,
    out: &mut HashMap<String, String>,
) {
    let full = if module_path.is_empty() {
        format!("{import_root}::{name}")
    } else {
        format!("{import_root}::{module_path}::{name}")
    };
    out.entry(name.to_string()).or_insert(full);
}

/// Walk parent directories from `source_file` up to `src/`, parsing each
/// `mod.rs` / `lib.rs` for `pub use` items.
///
/// The leaf names of those items are registered at the parent module's path —
/// i.e. the canonical PUBLIC re-export path — so private submodule paths
/// (e.g. `vsm::panel::ArchivePanelState`) are never used when the type is
/// already accessible via the public path (`vsm::ArchivePanelState`).
///
/// Outermost parent (lib.rs / crate root) wins: we collect all parent module
/// files from inner to outer, then process them outer-first with `or_insert`.
/// This means `lib.rs`'s flat `pub use vsm::{CombatState}` registers as
/// `valinoreth::CombatState`, taking priority over the deeper
/// `valinoreth::vsm::CombatState` that `vsm/mod.rs` would produce.
fn scan_parent_modules(source_file: &Path, import_root: &str, out: &mut HashMap<String, String>) {
    let mut dir = match source_file.parent() {
        Some(p) => p,
        None => return,
    };

    // Collect (parent_file, module_path) pairs from inner to outer.
    let mut parents: Vec<(PathBuf, String)> = Vec::new();

    loop {
        let mod_file = dir.join("mod.rs");
        let lib_file = dir.join("lib.rs");

        let parent_file = if mod_file.exists() {
            mod_file
        } else if lib_file.exists() {
            lib_file
        } else {
            match dir.parent() {
                Some(p) => {
                    dir = p;
                    continue;
                }
                None => break,
            }
        };

        let module_path = derive_module_path(&parent_file);
        parents.push((parent_file, module_path));

        let dir_name = dir.file_name().map_or("", |s| s.to_str().unwrap_or(""));
        if dir_name == "src" {
            break;
        }

        match dir.parent() {
            Some(p) => dir = p,
            None => break,
        }
    }

    // Process outer-first so lib.rs flat re-exports take priority over deeper paths.
    for (parent_file, module_path) in parents.into_iter().rev() {
        if let Ok(src) = std::fs::read_to_string(&parent_file)
            && let Ok(syntax) = syn::parse_file(&src)
        {
            for item in &syntax.items {
                if let Item::Use(u) = item
                    && is_pub(&u.vis)
                {
                    let mut names: Vec<String> = Vec::new();
                    extract_leaf_names(&u.tree, &mut names);
                    for name in names {
                        let full = if module_path.is_empty() {
                            format!("{import_root}::{name}")
                        } else {
                            format!("{import_root}::{module_path}::{name}")
                        };
                        out.entry(name).or_insert(full);
                    }
                }
            }
        }
    }
}

/// Extract all leaf (terminal) names from a `syn::UseTree`.
///
/// Glob trees (`use foo::*`) are skipped — their contents cannot be
/// enumerated statically.
fn extract_leaf_names(tree: &UseTree, out: &mut Vec<String>) {
    match tree {
        UseTree::Path(p) => extract_leaf_names(&p.tree, out),
        UseTree::Name(n) => {
            let name = n.ident.to_string();
            if name != "self" {
                out.push(name);
            }
        }
        UseTree::Rename(r) => out.push(r.rename.to_string()),
        UseTree::Glob(_) => {}
        UseTree::Group(g) => {
            for item in &g.items {
                extract_leaf_names(item, out);
            }
        }
    }
}

fn is_pub(vis: &Visibility) -> bool {
    matches!(vis, Visibility::Public(_))
}

/// Returns `true` for types that are always in scope and never need an
/// explicit import: Rust primitives, std-prelude items, and the fixed imports
/// emitted by each generator (`Established`, `kani_label`, `KaniCompose`).
fn is_prelude_type(name: &str) -> bool {
    matches!(
        name,
        // Primitives
        "bool"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "u128"
            | "i8"
            | "i16"
            | "i32"
            | "i64"
            | "i128"
            | "usize"
            | "isize"
            | "f32"
            | "f64"
            | "char"
            | "str"
            // std prelude
            | "String"
            | "Vec"
            | "Option"
            | "Result"
            | "Box"
            | "Rc"
            | "Arc"
            | "Some"
            | "None"
            | "Ok"
            | "Err"
            | "Copy"
            | "Clone"
            | "Drop"
            | "Send"
            | "Sync"
            | "Sized"
            | "Default"
            | "PartialEq"
            | "Eq"
            | "PartialOrd"
            | "Ord"
            | "Hash"
            | "Iterator"
            | "IntoIterator"
            | "From"
            | "Into"
            | "ToString"
            | "AsRef"
            | "AsMut"
            // Fixed imports handled separately in each generator
            | "Established"
            | "kani_label"
            | "KaniCompose"
            // Proof macro keywords (not types)
            | "pearlite"
            | "verus"
            | "prophetic"
    )
}
