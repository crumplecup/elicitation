//! Shared keyboard-navigation model for the archive frontends.
//!
//! `ArchiveNavModel` holds the flattened tree used by both the ratatui TUI and
//! the egui native frontend.  Keybinding semantics are declared once in
//! [`elicit_accesskit::StatusBarDescriptor::archive_browse`] and every
//! frontend derives its key-to-action mapping from that IR definition.
//!
//! ## Flat-list model
//!
//! The tree (schemas → tables) is projected into a flat `Vec<FlatItem>` that
//! the frontends render as a scrollable list.  A single `usize` cursor tracks
//! the selected row.  Schema rows can be expanded/collapsed; when a schema is
//! collapsed its child table rows are removed from the flat list.

use elicit_accesskit::{KeyBinding, StatusBarDescriptor};

use crate::archive::nav_tree::{NavTree, SchemaEntry};

// ── FlatItem ──────────────────────────────────────────────────────────────────

/// One visible row in the navigation panel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlatItem {
    /// A schema row.  The payload is the schema index into
    /// [`ArchiveNavModel::schemas`].
    Schema(usize),
    /// A table/view row.  The payload is `(schema_idx, table_idx)`.
    Table(usize, usize),
}

// ── SchemaWithExpand ──────────────────────────────────────────────────────────

/// A schema entry combined with its current expand/collapse state.
#[derive(Debug, Clone)]
pub struct SchemaWithExpand {
    /// The underlying schema descriptor.
    pub entry: SchemaEntry,
    /// Whether child tables are currently visible.
    pub expanded: bool,
}

// ── ArchiveNavModel ───────────────────────────────────────────────────────────

/// Frontend-agnostic keyboard-navigation state for the archive tree view.
///
/// Frontends call [`move_up`], [`move_down`], [`toggle_expand`] in response
/// to key events whose identities come from
/// [`StatusBarDescriptor::archive_browse`].
///
/// [`move_up`]: ArchiveNavModel::move_up
/// [`move_down`]: ArchiveNavModel::move_down
/// [`toggle_expand`]: ArchiveNavModel::toggle_expand
#[derive(Debug)]
pub struct ArchiveNavModel {
    /// Database display name.
    pub db_name: String,
    /// Optional server version string.
    pub version: Option<String>,
    /// Backend label (e.g. `"PostgreSQL"`).
    pub backend_label: String,
    /// All schemas with their expand state.
    pub schemas: Vec<SchemaWithExpand>,
    /// Flattened visible rows (rebuilt after every expand/collapse).
    pub flat: Vec<FlatItem>,
    /// Index into [`flat`] of the currently highlighted row.
    ///
    /// [`flat`]: ArchiveNavModel::flat
    pub cursor: usize,
    /// Whether the keybinding help overlay is shown.
    pub show_help: bool,
    /// Ephemeral status flash (e.g. refresh confirmation).
    pub flash: Option<String>,
}

impl ArchiveNavModel {
    /// Build from a [`NavTree`], selecting the first row.
    pub fn new(nav: NavTree) -> Self {
        let schemas: Vec<SchemaWithExpand> = nav
            .schemas
            .into_iter()
            .map(|e| SchemaWithExpand {
                entry: e,
                expanded: false,
            })
            .collect();

        let mut model = Self {
            db_name: nav.db_name,
            version: nav.version,
            backend_label: nav.backend.to_string(),
            schemas,
            flat: Vec::new(),
            cursor: 0,
            show_help: false,
            flash: None,
        };
        model.rebuild_flat();
        model
    }

    /// Rebuild the flat list from the current expand state.
    pub fn rebuild_flat(&mut self) {
        self.flat.clear();
        for (i, s) in self.schemas.iter().enumerate() {
            self.flat.push(FlatItem::Schema(i));
            if s.expanded {
                for j in 0..s.entry.tables.len() {
                    self.flat.push(FlatItem::Table(i, j));
                }
            }
        }
        self.cursor = self.cursor.min(self.flat.len().saturating_sub(1));
    }

    /// Move selection one row up (wraps around).
    pub fn move_up(&mut self) {
        if self.flat.is_empty() {
            return;
        }
        self.cursor = if self.cursor == 0 {
            self.flat.len() - 1
        } else {
            self.cursor - 1
        };
    }

    /// Move selection one row down (wraps around).
    pub fn move_down(&mut self) {
        if self.flat.is_empty() {
            return;
        }
        self.cursor = if self.cursor + 1 >= self.flat.len() {
            0
        } else {
            self.cursor + 1
        };
    }

    /// Expand/collapse a schema row, or select a table row.
    pub fn toggle_expand(&mut self) {
        let Some(&item) = self.flat.get(self.cursor) else {
            return;
        };
        match item {
            FlatItem::Schema(i) => {
                self.schemas[i].expanded = !self.schemas[i].expanded;
                self.rebuild_flat();
                // Keep cursor on the same schema after rebuild.
                if let Some(pos) = self
                    .flat
                    .iter()
                    .position(|f| matches!(f, FlatItem::Schema(j) if *j == i))
                {
                    self.cursor = pos;
                }
            }
            FlatItem::Table(si, ti) => {
                let t = &self.schemas[si].entry.tables[ti];
                let cols = t.columns.len();
                let rows = t
                    .estimated_rows
                    .map(|r| format!("~{r} rows"))
                    .unwrap_or_else(|| "rows: ?".to_string());
                self.flash = Some(format!(
                    "{}.{} — {cols} columns, {rows}",
                    t.schema, t.table_name
                ));
            }
        }
    }

    /// Trigger a refresh (no-op in demo; frontends may override).
    pub fn refresh(&mut self) {
        self.flash = Some("↺ Refreshed (demo)".to_string());
    }

    /// Toggle the help overlay.
    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
        self.flash = None;
    }

    /// Return the currently selected `FlatItem`, if any.
    pub fn selected(&self) -> Option<FlatItem> {
        self.flat.get(self.cursor).copied()
    }

    /// Key bindings as declared in the accesskit IR.
    ///
    /// Every frontend should drive its status-bar rendering and its key-event
    /// dispatch from this single source of truth rather than duplicating the
    /// binding list.
    pub fn bindings() -> Vec<KeyBinding> {
        StatusBarDescriptor::archive_browse().bindings
    }
}
