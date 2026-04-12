//! Workflow plugins for `elicit_winit`.

mod event_plugin;
mod input_plugin;
mod window_plugin;

pub use event_plugin::{WinitEventLoopScaffolded, WinitEventPlugin};
pub use input_plugin::{WinitInputHandled, WinitInputPlugin};
pub use window_plugin::{WinitWindowConfigured, WinitWindowPlugin};
