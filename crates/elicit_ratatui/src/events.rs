//! Trenchcoat types for crossterm terminal events.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ── KeyModifiers ──────────────────────────────────────────────────────────────

/// Active keyboard modifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
pub struct KeyModifiers {
    /// Shift key held.
    pub shift: bool,
    /// Control key held.
    pub control: bool,
    /// Alt key held.
    pub alt: bool,
    /// Super/Windows/Command key held.
    pub super_key: bool,
    /// Hyper key held.
    pub hyper: bool,
    /// Meta key held.
    pub meta: bool,
}

// ── KeyCode ───────────────────────────────────────────────────────────────────

/// A keyboard key code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum KeyCode {
    /// A printable character.
    Char {
        /// The character.
        c: char,
    },
    /// Backspace key.
    Backspace,
    /// Enter / Return key.
    Enter,
    /// Left arrow.
    Left,
    /// Right arrow.
    Right,
    /// Up arrow.
    Up,
    /// Down arrow.
    Down,
    /// Home key.
    Home,
    /// End key.
    End,
    /// Page Up key.
    PageUp,
    /// Page Down key.
    PageDown,
    /// Tab key.
    Tab,
    /// Back-tab (Shift+Tab).
    BackTab,
    /// Delete key.
    Delete,
    /// Insert key.
    Insert,
    /// Escape key.
    Esc,
    /// Function key F1–F12.
    F {
        /// Function key number (1–12).
        n: u8,
    },
    /// Null / NUL character.
    Null,
    /// A modifier key pressed alone.
    Modifier,
}

// ── KeyEvent ──────────────────────────────────────────────────────────────────

/// A keyboard event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct KeyEvent {
    /// The key that was pressed.
    pub code: KeyCode,
    /// Active modifiers at the time of the event.
    pub modifiers: KeyModifiers,
}

// ── MouseButton ───────────────────────────────────────────────────────────────

/// A mouse button.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum MouseButton {
    /// Primary (usually left) button.
    Left,
    /// Secondary (usually right) button.
    Right,
    /// Middle (scroll wheel click) button.
    Middle,
    /// Unknown button.
    Unknown,
}

// ── MouseEventKind ────────────────────────────────────────────────────────────

/// The kind of mouse event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum MouseEventKind {
    /// Button pressed.
    Down {
        /// Which button.
        button: MouseButton,
    },
    /// Button released.
    Up {
        /// Which button.
        button: MouseButton,
    },
    /// Dragged while button held.
    Drag {
        /// Which button.
        button: MouseButton,
    },
    /// Mouse moved (no button pressed).
    Moved,
    /// Scroll wheel up.
    ScrollUp,
    /// Scroll wheel down.
    ScrollDown,
    /// Scroll wheel left.
    ScrollLeft,
    /// Scroll wheel right.
    ScrollRight,
}

// ── MouseEvent ────────────────────────────────────────────────────────────────

/// A mouse event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct MouseEvent {
    /// The kind of mouse event.
    pub kind: MouseEventKind,
    /// Column of the event (0-based).
    pub column: u16,
    /// Row of the event (0-based).
    pub row: u16,
    /// Active modifiers.
    pub modifiers: KeyModifiers,
}

// ── Event ─────────────────────────────────────────────────────────────────────

/// A terminal event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum Event {
    /// A keyboard event.
    Key(KeyEvent),
    /// A mouse event.
    Mouse(MouseEvent),
    /// The terminal was resized.
    Resize {
        /// New width in columns.
        columns: u16,
        /// New height in rows.
        rows: u16,
    },
    /// Focus gained.
    FocusGained,
    /// Focus lost.
    FocusLost,
    /// Paste event with pasted text.
    Paste {
        /// Pasted text.
        text: String,
    },
}
