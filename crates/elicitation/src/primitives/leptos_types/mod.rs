//! Leptos descriptor primitives.
//!
//! Available with the `leptos-types` feature.

mod axum_ssr;
mod descriptors;
mod enums;

pub use axum_ssr::{
    LeptosAxumDescriptor, LeptosAxumMode, LeptosCustomRouteDescriptor,
    LeptosResponseHeaderDescriptor,
};
pub use descriptors::{
    LeptosAppDescriptor, LeptosComponentDescriptor, LeptosPropDescriptor, LeptosRouteDescriptor,
    LeptosViewNode,
};
pub use enums::{LeptosHtmlTag, LeptosMode};
