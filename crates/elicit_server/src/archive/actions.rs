//! Platform-neutral action vocabulary and key map for the archive frontends.
//!
//! ## Design
//!
//! All key→action mappings are declared **once** in
//! [`ArchiveKeyMap::default_map`].  Every frontend calls
//! [`ArchiveKeyMap::resolve`] to translate a raw key event into an
//! [`ArchiveAction`], then dispatches via
//! [`ArchiveFrontend::dispatch_action`][crate::archive::frontend_trait::ArchiveFrontend].
//!
//! The exhaustive match required by `dispatch_action` is the compiler-enforced
//! guarantee: every declared action must be handled in every frontend.  Adding
//! a new action to [`ArchiveAction`] produces compile errors in all frontends
//! until they are updated.
//!
//! [`StatusBarDescriptor`] chips and
//! the browser-frontend JavaScript keyboard listener are both **derived** from
//! the key map — single source of truth, no per-frontend duplication.

use elicit_accesskit::{ColorTheme, KeyBinding, StatusBarDescriptor};

// ── Platform-neutral key representation ──────────────────────────────────────

/// Platform-neutral key identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ArchiveKey {
    /// Arrow up.
    Up,
    /// Arrow down.
    Down,
    /// Enter / Return.
    Enter,
    /// Escape.
    Esc,
    /// Backspace.
    Backspace,
    /// Delete.
    Delete,
    /// Tab.
    Tab,
    /// Shift+Tab (BackTab in crossterm).
    BackTab,
    /// Page Down.
    PageDown,
    /// Page Up.
    PageUp,
    /// Home.
    Home,
    /// End.
    End,
    /// Function key Fn.
    F(u8),
    /// Printable character.
    Char(char),
}

/// Platform-neutral key combination.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyCombo {
    /// Base key.
    pub key: ArchiveKey,
    /// Control modifier held.
    pub ctrl: bool,
    /// Shift modifier held.
    pub shift: bool,
    /// Alt modifier held.
    pub alt: bool,
}

impl KeyCombo {
    /// No modifiers.
    pub fn plain(key: ArchiveKey) -> Self {
        Self {
            key,
            ctrl: false,
            shift: false,
            alt: false,
        }
    }

    /// Control held.
    pub fn ctrl(key: ArchiveKey) -> Self {
        Self {
            key,
            ctrl: true,
            shift: false,
            alt: false,
        }
    }

    /// Control + Shift held.
    pub fn ctrl_shift(key: ArchiveKey) -> Self {
        Self {
            key,
            ctrl: true,
            shift: true,
            alt: false,
        }
    }
}

// ── UI mode filter ────────────────────────────────────────────────────────────

/// Which UI mode determines active key bindings.
///
/// [`ArchiveKeyMap::resolve`] uses this to disambiguate keys that do different
/// things in different modes (e.g. `↑` navigates the schema tree in
/// [`Default`][`KeyMapMode::Default`] but scrolls the saved-query list in
/// [`SavedBrowser`][`KeyMapMode::SavedBrowser`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyMapMode {
    /// Normal tree navigation — no overlay active.
    Default,
    /// Search/filter bar is open.
    Filter,
    /// Save-query name prompt is open.
    SavePrompt,
    /// Saved-query browser overlay is open.
    SavedBrowser,
    /// Export format picker overlay is open.
    ExportPicker,
    /// SQL editor panel is active.
    SqlEditor,
}

// ── Action vocabulary ─────────────────────────────────────────────────────────

/// All possible user-initiated actions in the archive application.
///
/// This is the canonical action vocabulary.  Every archive frontend must
/// implement [`ArchiveFrontend::dispatch_action`][crate::archive::frontend_trait::ArchiveFrontend]
/// with an **exhaustive match** over this enum — the compiler enforces that no
/// action variant is silently unhandled.
///
/// To add a new action: add a variant here, add entries to
/// [`ArchiveKeyMap::default_map`], then fix the compile errors in each
/// frontend's `dispatch_action` impl.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArchiveAction {
    // ── Navigation (Default mode) ─────────────────────────────────────────
    /// Move the selection up one row.
    MoveUp,
    /// Move the selection down one row.
    MoveDown,
    /// Toggle-expand / select the focused item.
    Select,
    /// Re-query the database and rebuild the navigation tree.
    Refresh,
    /// Toggle the keybinding help overlay.
    ToggleHelp,
    /// Open the search/filter bar.
    OpenFilter,
    /// Open the SQL editor panel.
    OpenSqlEditor,
    /// Open or close the saved-query browser.
    OpenSavedBrowser,
    /// Open or close the live monitor panel.
    OpenMonitor,
    /// Open or close the admin panel (roles, backups, settings).
    OpenAdmin,
    /// Open or close the ERD (entity-relationship diagram) for the selected schema.
    OpenErd,
    /// Open or close the constraint panel for the selected table.
    OpenConstraints,
    /// Open or close the index panel for the selected table.
    OpenIndexes,
    /// Open or close the connection profile editor for the active connection.
    EditConnection,
    /// Cycle forward to the next admin panel tab.
    AdminTabNext,
    /// Cycle backward to the previous admin panel tab.
    AdminTabPrev,
    /// Toggle the export format picker (effective only in DataGrid mode).
    ToggleExportPicker,
    /// Request DDL source for the selected table.
    RequestDdl,
    /// Request an EXPLAIN plan for the last query on the selected table.
    RequestExplain,
    /// Clear the side-by-side EXPLAIN comparison and return to single-plan view.
    ClearExplainCompare,
    /// Advance the data grid to the next page.
    PageNext,
    /// Return the data grid to the previous page.
    PagePrev,
    /// Jump to the first page of the data grid.
    PageFirst,
    /// Jump to the last page of the data grid.
    PageLast,
    /// Cycle forward to the next database connection.
    ConnNext,
    /// Cycle backward to the previous database connection.
    ConnPrev,
    /// Quit the application.
    Quit,

    // ── Filter mode ────────────────────────────────────────────────────────
    /// Close the filter bar (restores the full unfiltered list).
    FilterClose,
    /// Delete the last character in the filter input.
    FilterBackspace,

    // ── Save-prompt mode ───────────────────────────────────────────────────
    /// Close the save-query name prompt without saving.
    SavePromptClose,
    /// Delete the last character in the save-name input.
    SavePromptBackspace,
    /// Confirm and persist the current SQL under the entered name.
    SavePromptConfirm,

    // ── Saved-browser mode ─────────────────────────────────────────────────
    /// Close the saved-query browser.
    SavedBrowserClose,
    /// Move the saved-query selection up.
    SavedBrowserUp,
    /// Move the saved-query selection down.
    SavedBrowserDown,
    /// Load the focused saved query into the SQL editor.
    SavedBrowserSelect,
    /// Delete the focused saved query.
    SavedBrowserDelete,

    // ── Export-picker mode ─────────────────────────────────────────────────
    /// Close the export format picker without exporting.
    ExportPickerClose,
    /// Move the format selection up.
    ExportPickerUp,
    /// Move the format selection down.
    ExportPickerDown,
    /// Export in the selected format.
    ExportPickerConfirm,

    // ── SQL-editor mode ────────────────────────────────────────────────────
    /// Execute the current SQL.
    SqlRun,
    /// Navigate to the previous SQL history entry.
    SqlHistoryPrev,
    /// Navigate to the next SQL history entry.
    SqlHistoryNext,
    /// Close the SQL editor (return to column detail view).
    SqlClose,
    /// Open the save-query name prompt for the current SQL.
    SqlSave,
    /// Delete the last character in the SQL editor.
    SqlBackspace,
    /// Insert a newline in the SQL editor.
    SqlNewline,
}

// ── Key map entry ─────────────────────────────────────────────────────────────

/// A single binding in the archive key map.
pub struct KeyMapEntry {
    /// Key combination that triggers this action.
    pub combo: KeyCombo,
    /// UI mode in which this binding is active (`None` = all modes).
    pub mode: Option<KeyMapMode>,
    /// The action to dispatch.
    pub action: ArchiveAction,
    /// Whether to surface this binding as a status bar chip.
    pub show_in_status_bar: bool,
    /// Short human-readable key label (e.g. `"Ctrl+Tab"`).  Empty → not shown.
    pub label: &'static str,
    /// Human-readable action description (e.g. `"Next connection"`).
    pub description: &'static str,
}

impl KeyMapEntry {
    fn nav(
        combo: KeyCombo,
        action: ArchiveAction,
        label: &'static str,
        description: &'static str,
        show: bool,
    ) -> Self {
        Self {
            combo,
            mode: Some(KeyMapMode::Default),
            action,
            show_in_status_bar: show,
            label,
            description,
        }
    }

    fn modal(
        mode: KeyMapMode,
        combo: KeyCombo,
        action: ArchiveAction,
        label: &'static str,
        description: &'static str,
        show: bool,
    ) -> Self {
        Self {
            combo,
            mode: Some(mode),
            action,
            show_in_status_bar: show,
            label,
            description,
        }
    }
}

// ── Key map ───────────────────────────────────────────────────────────────────

/// Canonical key map for the archive application.
///
/// The key map is the **single source of truth** for:
/// - Runtime key→action dispatch in all three frontends
/// - [`StatusBarDescriptor`] chip labels (derived via [`to_status_bar`][Self::to_status_bar])
/// - JavaScript keyboard listener for the browser frontend (derived via
///   [`to_js_listener`][Self::to_js_listener])
pub struct ArchiveKeyMap(Vec<KeyMapEntry>);

impl ArchiveKeyMap {
    /// The complete canonical key map for the archive application.
    pub fn default_map() -> Self {
        use ArchiveAction as A;
        use ArchiveKey as K;
        use KeyMapMode as M;
        let p = KeyCombo::plain;
        let c = KeyCombo::ctrl;
        let cs = KeyCombo::ctrl_shift;

        Self(vec![
            // ── Default / navigation mode ──────────────────────────────────
            KeyMapEntry::nav(p(K::Up), A::MoveUp, "↑/k", "Move up", true),
            KeyMapEntry::nav(p(K::Char('k')), A::MoveUp, "", "", false),
            KeyMapEntry::nav(p(K::Down), A::MoveDown, "↓/j", "Move down", true),
            KeyMapEntry::nav(p(K::Char('j')), A::MoveDown, "", "", false),
            KeyMapEntry::nav(p(K::Enter), A::Select, "Enter", "Select", true),
            KeyMapEntry::nav(p(K::Char('r')), A::Refresh, "r", "Refresh", true),
            KeyMapEntry::nav(p(K::Char('?')), A::ToggleHelp, "?", "Help", true),
            KeyMapEntry::nav(p(K::Char('q')), A::Quit, "q", "Quit", true),
            KeyMapEntry::nav(p(K::Esc), A::Quit, "", "", false),
            KeyMapEntry::nav(p(K::Char('/')), A::OpenFilter, "/", "Filter", true),
            KeyMapEntry::nav(p(K::Char('d')), A::RequestDdl, "d", "DDL", true),
            KeyMapEntry::nav(p(K::Char('e')), A::RequestExplain, "e", "Explain", true),
            KeyMapEntry::nav(
                cs(K::Char('e')),
                A::ClearExplainCompare,
                "⇧e",
                "Clear compare",
                true,
            ),
            KeyMapEntry::nav(p(K::Char('s')), A::OpenSqlEditor, "s", "SQL", true),
            KeyMapEntry::nav(p(K::F(2)), A::OpenSavedBrowser, "F2", "Saved", true),
            KeyMapEntry::nav(p(K::Char('m')), A::OpenMonitor, "m", "Monitor", true),
            KeyMapEntry::nav(p(K::Char('a')), A::OpenAdmin, "a", "Admin", true),
            KeyMapEntry::nav(p(K::Char('g')), A::OpenErd, "g", "Graph/ERD", true),
            KeyMapEntry::nav(
                p(K::Char('c')),
                A::OpenConstraints,
                "c",
                "Constraints",
                true,
            ),
            KeyMapEntry::nav(p(K::Char('i')), A::OpenIndexes, "i", "Indexes", true),
            KeyMapEntry::nav(
                p(K::Char('o')),
                A::EditConnection,
                "o",
                "Edit connection",
                true,
            ),
            KeyMapEntry::nav(p(K::Char(']')), A::AdminTabNext, "]", "Next tab", true),
            KeyMapEntry::nav(p(K::Char('[')), A::AdminTabPrev, "[", "Prev tab", true),
            KeyMapEntry::nav(p(K::Char('x')), A::ToggleExportPicker, "x", "Export", true),
            KeyMapEntry::nav(p(K::PageDown), A::PageNext, "PgDn", "Next page", true),
            KeyMapEntry::nav(p(K::PageUp), A::PagePrev, "PgUp", "Prev page", true),
            KeyMapEntry::nav(p(K::Home), A::PageFirst, "Home", "First page", true),
            KeyMapEntry::nav(p(K::End), A::PageLast, "End", "Last page", true),
            KeyMapEntry::nav(c(K::Tab), A::ConnNext, "Ctrl+Tab", "Next conn.", true),
            KeyMapEntry::nav(cs(K::Tab), A::ConnPrev, "Ctrl+⇧Tab", "Prev conn.", true),
            // BackTab is the crossterm encoding of Shift+Tab
            KeyMapEntry::nav(c(K::BackTab), A::ConnPrev, "", "", false),
            // ── Filter mode ────────────────────────────────────────────────
            KeyMapEntry::modal(
                M::Filter,
                p(K::Esc),
                A::FilterClose,
                "Esc",
                "Close filter",
                false,
            ),
            KeyMapEntry::modal(
                M::Filter,
                p(K::Backspace),
                A::FilterBackspace,
                "",
                "",
                false,
            ),
            // ── Save-prompt mode ───────────────────────────────────────────
            KeyMapEntry::modal(
                M::SavePrompt,
                p(K::Esc),
                A::SavePromptClose,
                "Esc",
                "Cancel",
                false,
            ),
            KeyMapEntry::modal(
                M::SavePrompt,
                p(K::Backspace),
                A::SavePromptBackspace,
                "",
                "",
                false,
            ),
            KeyMapEntry::modal(
                M::SavePrompt,
                p(K::Enter),
                A::SavePromptConfirm,
                "Enter",
                "Save",
                false,
            ),
            // ── Saved-browser mode ─────────────────────────────────────────
            KeyMapEntry::modal(
                M::SavedBrowser,
                p(K::Esc),
                A::SavedBrowserClose,
                "Esc",
                "Close",
                false,
            ),
            KeyMapEntry::modal(
                M::SavedBrowser,
                p(K::Char('q')),
                A::SavedBrowserClose,
                "",
                "",
                false,
            ),
            KeyMapEntry::modal(M::SavedBrowser, p(K::Up), A::SavedBrowserUp, "", "", false),
            KeyMapEntry::modal(
                M::SavedBrowser,
                p(K::Char('k')),
                A::SavedBrowserUp,
                "",
                "",
                false,
            ),
            KeyMapEntry::modal(
                M::SavedBrowser,
                p(K::Down),
                A::SavedBrowserDown,
                "",
                "",
                false,
            ),
            KeyMapEntry::modal(
                M::SavedBrowser,
                p(K::Char('j')),
                A::SavedBrowserDown,
                "",
                "",
                false,
            ),
            KeyMapEntry::modal(
                M::SavedBrowser,
                p(K::Enter),
                A::SavedBrowserSelect,
                "Enter",
                "Load",
                false,
            ),
            KeyMapEntry::modal(
                M::SavedBrowser,
                p(K::Char('d')),
                A::SavedBrowserDelete,
                "d",
                "Delete",
                false,
            ),
            KeyMapEntry::modal(
                M::SavedBrowser,
                p(K::Delete),
                A::SavedBrowserDelete,
                "",
                "",
                false,
            ),
            // ── Export-picker mode ─────────────────────────────────────────
            KeyMapEntry::modal(
                M::ExportPicker,
                p(K::Esc),
                A::ExportPickerClose,
                "Esc",
                "Cancel",
                false,
            ),
            KeyMapEntry::modal(M::ExportPicker, p(K::Up), A::ExportPickerUp, "", "", false),
            KeyMapEntry::modal(
                M::ExportPicker,
                p(K::Char('k')),
                A::ExportPickerUp,
                "",
                "",
                false,
            ),
            KeyMapEntry::modal(
                M::ExportPicker,
                p(K::Down),
                A::ExportPickerDown,
                "",
                "",
                false,
            ),
            KeyMapEntry::modal(
                M::ExportPicker,
                p(K::Char('j')),
                A::ExportPickerDown,
                "",
                "",
                false,
            ),
            KeyMapEntry::modal(
                M::ExportPicker,
                p(K::Enter),
                A::ExportPickerConfirm,
                "Enter",
                "Export",
                false,
            ),
            // ── SQL-editor mode ────────────────────────────────────────────
            KeyMapEntry::modal(M::SqlEditor, c(K::Enter), A::SqlRun, "Ctrl+↵", "Run", true),
            KeyMapEntry::modal(M::SqlEditor, p(K::F(5)), A::SqlRun, "F5", "Run", false),
            KeyMapEntry::modal(
                M::SqlEditor,
                c(K::Up),
                A::SqlHistoryPrev,
                "Ctrl+↑",
                "Prev",
                false,
            ),
            KeyMapEntry::modal(
                M::SqlEditor,
                c(K::Down),
                A::SqlHistoryNext,
                "Ctrl+↓",
                "Next",
                false,
            ),
            KeyMapEntry::modal(M::SqlEditor, p(K::Esc), A::SqlClose, "Esc", "Close", false),
            KeyMapEntry::modal(
                M::SqlEditor,
                c(K::Char('s')),
                A::SqlSave,
                "Ctrl+s",
                "Save",
                false,
            ),
            KeyMapEntry::modal(
                M::SqlEditor,
                p(K::Backspace),
                A::SqlBackspace,
                "",
                "",
                false,
            ),
            KeyMapEntry::modal(M::SqlEditor, p(K::Enter), A::SqlNewline, "", "", false),
        ])
    }

    /// Resolve a key combination in the given mode to an [`ArchiveAction`].
    ///
    /// Mode-specific entries are checked first; only if none match does
    /// resolution fall back to entries with `mode = None` (global bindings).
    pub fn resolve(&self, combo: &KeyCombo, mode: KeyMapMode) -> Option<ArchiveAction> {
        self.0
            .iter()
            .find(|e| e.combo == *combo && e.mode == Some(mode))
            .or_else(|| {
                self.0
                    .iter()
                    .find(|e| e.combo == *combo && e.mode.is_none())
            })
            .map(|e| e.action)
    }

    /// Derive a [`StatusBarDescriptor`] for the given mode.
    ///
    /// Only entries with `show_in_status_bar = true` and a non-empty `label`
    /// that are active in `mode` (or globally) are included.  This is the
    /// single source of truth for the status bar — do not hard-code chips
    /// in any frontend.
    pub fn to_status_bar(&self, mode: KeyMapMode) -> StatusBarDescriptor {
        let bindings = self
            .0
            .iter()
            .filter(|e| {
                e.show_in_status_bar
                    && !e.label.is_empty()
                    && (e.mode == Some(mode) || e.mode.is_none())
            })
            .map(|e| KeyBinding::new(e.label, e.description))
            .collect();
        StatusBarDescriptor::new(bindings, ColorTheme::Dark)
    }

    /// Generate a JavaScript `keydown` listener for the browser frontend.
    ///
    /// The generated script POSTs `{ "action": "<VariantName>" }` to
    /// `/api/action` whenever a registered key combo for `mode` is pressed.
    /// This keeps the browser frontend's keyboard behaviour in sync with the
    /// IR-declared key map without per-action JavaScript duplication.
    pub fn to_js_listener(&self, mode: KeyMapMode) -> String {
        let mut lines = vec![
            "document.addEventListener('keydown', function(e) {".to_string(),
            "  var ctrl = e.ctrlKey, shift = e.shiftKey, action = null;".to_string(),
        ];
        for entry in &self.0 {
            if entry.mode != Some(mode) && entry.mode.is_some() {
                continue;
            }
            let cond = js_combo_condition(&entry.combo);
            let name = format!("{:?}", entry.action);
            lines.push(format!("  if ({cond}) action = '{name}';"));
        }
        lines.push("  if (action) {".to_string());
        lines.push("    e.preventDefault();".to_string());
        lines.push(
            "    fetch('/api/action', { method: 'POST', \
             headers: {'Content-Type': 'application/json'}, \
             body: JSON.stringify({action: action}) });"
                .to_string(),
        );
        lines.push("  }".to_string());
        lines.push("});".to_string());
        lines.join("\n")
    }

    /// All entries in declaration order.
    pub fn entries(&self) -> &[KeyMapEntry] {
        &self.0
    }
}

fn js_key_str(key: &ArchiveKey) -> String {
    match key {
        ArchiveKey::Up => "ArrowUp".to_string(),
        ArchiveKey::Down => "ArrowDown".to_string(),
        ArchiveKey::Enter => "Enter".to_string(),
        ArchiveKey::Esc => "Escape".to_string(),
        ArchiveKey::Backspace => "Backspace".to_string(),
        ArchiveKey::Delete => "Delete".to_string(),
        ArchiveKey::Tab | ArchiveKey::BackTab => "Tab".to_string(),
        ArchiveKey::F(n) => format!("F{n}"),
        ArchiveKey::Char(c) => c.to_string(),
        ArchiveKey::PageDown => "PageDown".to_string(),
        ArchiveKey::PageUp => "PageUp".to_string(),
        ArchiveKey::Home => "Home".to_string(),
        ArchiveKey::End => "End".to_string(),
    }
}

fn js_combo_condition(combo: &KeyCombo) -> String {
    let shift = combo.shift || combo.key == ArchiveKey::BackTab;
    let ctrl_part = if combo.ctrl { "ctrl" } else { "!ctrl" };
    let shift_part = if shift { "shift" } else { "!shift" };
    let key = js_key_str(&combo.key);
    let key_part = if combo.key == ArchiveKey::Tab || combo.key == ArchiveKey::BackTab {
        format!("e.key==='{key}'")
    } else {
        format!("e.key==='{}' || e.code==='Key{}'", key, key.to_uppercase())
    };
    format!("{ctrl_part} && {shift_part} && ({key_part})")
}
