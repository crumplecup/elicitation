//! Select-trenchcoat wrappers for winit enum types.
//!
//! Most wrappers use the `serde` delegation variant — the underlying winit
//! types derive `Serialize`/`Deserialize` when `winit/serde` is enabled.
//! Exceptions:
//! - `WindowLevel` does not have serde or `Hash` in winit 0.30, so we use
//!   the manual-serde (label-based) form and omit `hash`.
//! - `Theme` has serde but lacks `Hash`, so we omit `hash`.

// ── Window configuration enums ────────────────────────────────────────────────

// WindowLevel: no serde, no Hash — use manual-serde (label-based) form.
crate::select_trenchcoat!(winit::window::WindowLevel, as WinitWindowLevelSelect);
crate::select_trenchcoat_traits!(
    WinitWindowLevelSelect,
    winit::window::WindowLevel,
    [copy, eq]
);

// Theme: has serde but no Hash.
crate::select_trenchcoat!(winit::window::Theme, as WinitThemeSelect, serde);
crate::select_trenchcoat_traits!(WinitThemeSelect, winit::window::Theme, [copy, eq]);

// CursorIcon is re-exported from the cursor-icon crate via winit::window
crate::select_trenchcoat!(winit::window::CursorIcon, as WinitCursorIconSelect, serde);
crate::select_trenchcoat_traits!(
    WinitCursorIconSelect,
    winit::window::CursorIcon,
    [copy, eq, hash]
);

// ── Input / event enums ───────────────────────────────────────────────────────

crate::select_trenchcoat!(winit::event::ElementState, as WinitElementStateSelect, serde);
crate::select_trenchcoat_traits!(
    WinitElementStateSelect,
    winit::event::ElementState,
    [copy, eq, hash]
);

crate::select_trenchcoat!(winit::event::MouseButton, as WinitMouseButtonSelect, serde);
crate::select_trenchcoat_traits!(
    WinitMouseButtonSelect,
    winit::event::MouseButton,
    [copy, eq, hash]
);

crate::select_trenchcoat!(winit::event::TouchPhase, as WinitTouchPhaseSelect, serde);
crate::select_trenchcoat_traits!(
    WinitTouchPhaseSelect,
    winit::event::TouchPhase,
    [copy, eq, hash]
);

// ── Keyboard ──────────────────────────────────────────────────────────────────

// KeyCode has ~100 variants (physical key scan codes). All are unit variants
// so they serialize as plain strings, making them ideal for MCP tool params.
crate::select_trenchcoat!(winit::keyboard::KeyCode, as WinitKeyCodeSelect, serde);
crate::select_trenchcoat_traits!(
    WinitKeyCodeSelect,
    winit::keyboard::KeyCode,
    [copy, eq, hash]
);
