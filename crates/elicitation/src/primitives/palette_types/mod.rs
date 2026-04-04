//! Elicitation implementations for [`palette`] color types.
//!
//! Provides [`Elicitation`](crate::Elicitation) for palette 0.7 color types
//! that can be interactively constructed from an agent — composite types
//! via Survey (field-by-field elicitation).
//!
//! # Enabled by the `palette` feature
//!
//! ```toml
//! elicitation = { version = "*", features = ["palette"] }
//! ```

// ── Composite struct modules ─────────────────────────────────────────
mod srgb;

// ── Composite struct wrapper re-exports ──────────────────────────────
pub use srgb::{PaletteSrgb, PaletteSrgbStyle};
