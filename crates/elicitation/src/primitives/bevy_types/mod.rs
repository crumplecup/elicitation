//! Bevy game engine type elicitation primitives.
//!
//! Provides [`Elicitation`], [`Select`], and [`Survey`] implementations for
//! Bevy 0.18.x types. All types that cannot implement [`schemars::JsonSchema`]
//! under orphan rules are wrapped with owned trenchcoat structs or enums.
//!
//! Enabled with the `bevy-types` feature.

pub mod affine;
pub mod animation;
pub mod atmosphere;
pub mod audio;
pub mod camera;
pub mod color;
pub mod input;
pub mod mat;
pub mod pbr;
pub mod picking;
pub mod quat;
pub mod ray;
pub mod render_enums;
pub mod shapes;
pub mod sprite;
pub mod text;
pub mod time;
pub mod transform;
pub mod ui;
pub mod vec;
pub mod window;
