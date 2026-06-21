//! Runtime bridge for `bevy::input::ButtonInput<T>`.
//!
//! [`ButtonInputPlugin`] and [`MouseButtonInputPlugin`] each add an `Update`
//! system that snapshots key / button state into a shared
//! `Arc<RwLock<ButtonInputState<T>>>` every frame, so MCP tools and other
//! non-ECS code can read current input without touching the live world.

use std::collections::HashSet;
use std::sync::{Arc, RwLock};

use crate::KeyCode;
use crate::input::MouseButton;

/// Snapshot of button input state captured at the last frame boundary.
#[derive(Debug, Clone)]
pub struct ButtonInputState<T: Eq + std::hash::Hash + Clone> {
    pressed: HashSet<T>,
    just_pressed: HashSet<T>,
    just_released: HashSet<T>,
}

impl<T: Eq + std::hash::Hash + Clone> Default for ButtonInputState<T> {
    fn default() -> Self {
        Self {
            pressed: HashSet::new(),
            just_pressed: HashSet::new(),
            just_released: HashSet::new(),
        }
    }
}

impl<T: Eq + std::hash::Hash + Clone> ButtonInputState<T> {
    /// Returns `true` if `key` is currently held.
    pub fn pressed(&self, key: &T) -> bool {
        self.pressed.contains(key)
    }

    /// Returns `true` if `key` was pressed this frame.
    pub fn just_pressed(&self, key: &T) -> bool {
        self.just_pressed.contains(key)
    }

    /// Returns `true` if `key` was released this frame.
    pub fn just_released(&self, key: &T) -> bool {
        self.just_released.contains(key)
    }

    /// Iterator over all currently pressed keys.
    pub fn get_pressed(&self) -> impl Iterator<Item = &T> {
        self.pressed.iter()
    }

    /// Iterator over keys pressed this frame.
    pub fn get_just_pressed(&self) -> impl Iterator<Item = &T> {
        self.just_pressed.iter()
    }

    /// Iterator over keys released this frame.
    pub fn get_just_released(&self) -> impl Iterator<Item = &T> {
        self.just_released.iter()
    }
}

// ── KeyCode bridge ────────────────────────────────────────────────────────────

/// Runtime bridge that syncs `Res<ButtonInput<KeyCode>>` into a shared `Arc` each frame.
///
/// # Example
///
/// ```rust,ignore
/// let (plugin, state) = ButtonInputPlugin::new();
/// app.add_plugins(plugin);
/// // Later, from an MCP tool:
/// let guard = state.read().unwrap();
/// if guard.just_pressed(&KeyCode::from(bevy::input::keyboard::KeyCode::Space)) { ... }
/// ```
pub struct ButtonInputPlugin {
    state: Arc<RwLock<ButtonInputState<KeyCode>>>,
}

impl ButtonInputPlugin {
    /// Create the plugin and return a handle to the shared state.
    pub fn new() -> (Self, Arc<RwLock<ButtonInputState<KeyCode>>>) {
        let state = Arc::new(RwLock::new(ButtonInputState::default()));
        (
            Self {
                state: state.clone(),
            },
            state,
        )
    }
}

impl bevy::app::Plugin for ButtonInputPlugin {
    #[tracing::instrument(skip_all, name = "ButtonInputPlugin::build")]
    fn build(&self, app: &mut bevy::app::App) {
        let state = self.state.clone();
        app.add_systems(
            bevy::app::Update,
            move |input: bevy::ecs::system::Res<
                bevy::input::ButtonInput<bevy::input::keyboard::KeyCode>,
            >| {
                tracing::trace!(
                    pressed = input.get_pressed().count(),
                    just_pressed = input.get_just_pressed().count(),
                    just_released = input.get_just_released().count(),
                    "ButtonInput<KeyCode> frame sync"
                );
                if let Ok(mut guard) = state.write() {
                    guard.pressed = input.get_pressed().copied().map(KeyCode::from).collect();
                    guard.just_pressed = input
                        .get_just_pressed()
                        .copied()
                        .map(KeyCode::from)
                        .collect();
                    guard.just_released = input
                        .get_just_released()
                        .copied()
                        .map(KeyCode::from)
                        .collect();
                }
            },
        );
    }
}

// ── MouseButton bridge ────────────────────────────────────────────────────────

/// Runtime bridge that syncs `Res<ButtonInput<MouseButton>>` into a shared `Arc` each frame.
pub struct MouseButtonInputPlugin {
    state: Arc<RwLock<ButtonInputState<MouseButton>>>,
}

impl MouseButtonInputPlugin {
    /// Create the plugin and return a handle to the shared state.
    pub fn new() -> (Self, Arc<RwLock<ButtonInputState<MouseButton>>>) {
        let state = Arc::new(RwLock::new(ButtonInputState::default()));
        (
            Self {
                state: state.clone(),
            },
            state,
        )
    }
}

impl bevy::app::Plugin for MouseButtonInputPlugin {
    #[tracing::instrument(skip_all, name = "MouseButtonInputPlugin::build")]
    fn build(&self, app: &mut bevy::app::App) {
        let state = self.state.clone();
        app.add_systems(
            bevy::app::Update,
            move |input: bevy::ecs::system::Res<
                bevy::input::ButtonInput<bevy::input::mouse::MouseButton>,
            >| {
                tracing::trace!(
                    pressed = input.get_pressed().count(),
                    just_pressed = input.get_just_pressed().count(),
                    just_released = input.get_just_released().count(),
                    "ButtonInput<MouseButton> frame sync"
                );
                if let Ok(mut guard) = state.write() {
                    guard.pressed = input
                        .get_pressed()
                        .copied()
                        .map(MouseButton::from)
                        .collect();
                    guard.just_pressed = input
                        .get_just_pressed()
                        .copied()
                        .map(MouseButton::from)
                        .collect();
                    guard.just_released = input
                        .get_just_released()
                        .copied()
                        .map(MouseButton::from)
                        .collect();
                }
            },
        );
    }
}
