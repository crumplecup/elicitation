//! Leptos descriptor primitives.
//!
//! Available with the `leptos-types` feature.

mod descriptors;
mod enums;

pub use descriptors::{
    LeptosAppDescriptor, LeptosComponentDescriptor, LeptosPropDescriptor, LeptosRouteDescriptor,
    LeptosViewNode,
};
pub use enums::{LeptosHtmlTag, LeptosMode};
