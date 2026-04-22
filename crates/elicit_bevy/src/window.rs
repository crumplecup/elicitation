//! Bevy window shadow types.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;
use std::sync::Arc;

macro_rules! window_enum {
    ($name:ident, $upstream:path) => {
        elicit_newtype!($upstream, as $name);
        // Use [eq] only — not all window enums implement Hash.
        elicit_newtype_traits!($name, $upstream, [eq]);
        impl serde::Serialize for $name {
            fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                (*self.0).serialize(s)
            }
        }
        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                <$upstream>::deserialize(d).map(|v| $name(Arc::new(v)))
            }
        }
        impl From<$name> for $upstream {
            fn from(v: $name) -> Self { *v.0 }
        }
        impl elicitation::ElicitComplete for $name {}
    };
}

// ── WindowMode ────────────────────────────────────────────────────────────────

window_enum!(WindowMode, bevy::window::WindowMode);

#[reflect_methods]
impl WindowMode {
    /// Variant name: `"Windowed"`, `"BorderlessFullscreen"`, or `"Fullscreen"`.
    #[tracing::instrument(skip(self))]
    pub fn variant_name(&self) -> &'static str {
        use bevy::window::WindowMode as W;
        match *self.0 {
            W::Windowed => "Windowed",
            W::BorderlessFullscreen(_) => "BorderlessFullscreen",
            W::Fullscreen(_, _) => "Fullscreen",
        }
    }

    /// Returns `true` if the window is in windowed (non-fullscreen) mode.
    #[tracing::instrument(skip(self))]
    pub fn is_windowed(&self) -> bool {
        matches!(*self.0, bevy::window::WindowMode::Windowed)
    }

    /// Returns `true` if the window is in borderless fullscreen mode.
    #[tracing::instrument(skip(self))]
    pub fn is_borderless_fullscreen(&self) -> bool {
        matches!(*self.0, bevy::window::WindowMode::BorderlessFullscreen(_))
    }

    /// Returns `true` if the window is in exclusive fullscreen mode.
    #[tracing::instrument(skip(self))]
    pub fn is_fullscreen(&self) -> bool {
        matches!(*self.0, bevy::window::WindowMode::Fullscreen(_, _))
    }
}

mod emit_windowmode {
    use super::WindowMode;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for WindowMode {
        fn to_code_literal(&self) -> TokenStream {
            use bevy::window::WindowMode as W;
            match *self.0 {
                W::Windowed => quote::quote! { ::bevy::window::WindowMode::Windowed },
                W::BorderlessFullscreen(_) => quote::quote! {
                    ::bevy::window::WindowMode::BorderlessFullscreen(Default::default())
                },
                W::Fullscreen(_, _) => quote::quote! {
                    ::bevy::window::WindowMode::Fullscreen(
                        Default::default(), Default::default(),
                    )
                },
            }
        }
    }
}

// ── WindowLevel ───────────────────────────────────────────────────────────────

window_enum!(WindowLevel, bevy::window::WindowLevel);

#[reflect_methods]
impl WindowLevel {
    /// Variant name: `"AlwaysOnBottom"`, `"Normal"`, or `"AlwaysOnTop"`.
    #[tracing::instrument(skip(self))]
    pub fn variant_name(&self) -> &'static str {
        use bevy::window::WindowLevel as W;
        match *self.0 {
            W::AlwaysOnBottom => "AlwaysOnBottom",
            W::Normal => "Normal",
            W::AlwaysOnTop => "AlwaysOnTop",
        }
    }

    /// Returns `true` if the window is at the top of the z-order stack.
    #[tracing::instrument(skip(self))]
    pub fn is_always_on_top(&self) -> bool {
        matches!(*self.0, bevy::window::WindowLevel::AlwaysOnTop)
    }

    /// Returns `true` if the window is at the normal z-order level.
    #[tracing::instrument(skip(self))]
    pub fn is_normal(&self) -> bool {
        matches!(*self.0, bevy::window::WindowLevel::Normal)
    }
}

mod emit_windowlevel {
    use super::WindowLevel;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for WindowLevel {
        fn to_code_literal(&self) -> TokenStream {
            let v = proc_macro2::Ident::new(self.variant_name(), proc_macro2::Span::call_site());
            quote::quote! { ::bevy::window::WindowLevel::#v }
        }
    }
}

// ── PresentMode ───────────────────────────────────────────────────────────────

window_enum!(PresentMode, bevy::window::PresentMode);

#[reflect_methods]
impl PresentMode {
    /// Variant name, e.g. `"Fifo"`, `"Mailbox"`, `"AutoVsync"`.
    #[tracing::instrument(skip(self))]
    pub fn variant_name(&self) -> &'static str {
        use bevy::window::PresentMode as P;
        match *self.0 {
            P::AutoVsync => "AutoVsync",
            P::AutoNoVsync => "AutoNoVsync",
            P::Fifo => "Fifo",
            P::FifoRelaxed => "FifoRelaxed",
            P::Immediate => "Immediate",
            P::Mailbox => "Mailbox",
        }
    }

    /// Returns `true` if this is a vsync-enabled mode.
    #[tracing::instrument(skip(self))]
    pub fn is_vsync(&self) -> bool {
        use bevy::window::PresentMode as P;
        matches!(*self.0, P::AutoVsync | P::Fifo | P::FifoRelaxed)
    }

    /// Returns `true` if this is the immediate (no vsync) mode.
    #[tracing::instrument(skip(self))]
    pub fn is_immediate(&self) -> bool {
        matches!(*self.0, bevy::window::PresentMode::Immediate)
    }
}

mod emit_presentmode {
    use super::PresentMode;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for PresentMode {
        fn to_code_literal(&self) -> TokenStream {
            let v = proc_macro2::Ident::new(self.variant_name(), proc_macro2::Span::call_site());
            quote::quote! { ::bevy::window::PresentMode::#v }
        }
    }
}

// ── CursorGrabMode ────────────────────────────────────────────────────────────

window_enum!(CursorGrabMode, bevy::window::CursorGrabMode);

#[reflect_methods]
impl CursorGrabMode {
    /// Variant name: `"None"`, `"Confined"`, or `"Locked"`.
    #[tracing::instrument(skip(self))]
    pub fn variant_name(&self) -> &'static str {
        use bevy::window::CursorGrabMode as C;
        match *self.0 {
            C::None => "None",
            C::Confined => "Confined",
            C::Locked => "Locked",
        }
    }

    /// Returns `true` if the cursor is not grabbed.
    #[tracing::instrument(skip(self))]
    pub fn is_none(&self) -> bool {
        matches!(*self.0, bevy::window::CursorGrabMode::None)
    }

    /// Returns `true` if the cursor is confined to the window.
    #[tracing::instrument(skip(self))]
    pub fn is_confined(&self) -> bool {
        matches!(*self.0, bevy::window::CursorGrabMode::Confined)
    }

    /// Returns `true` if the cursor is locked in place.
    #[tracing::instrument(skip(self))]
    pub fn is_locked(&self) -> bool {
        matches!(*self.0, bevy::window::CursorGrabMode::Locked)
    }
}

mod emit_cursorgrab {
    use super::CursorGrabMode;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for CursorGrabMode {
        fn to_code_literal(&self) -> TokenStream {
            let v = proc_macro2::Ident::new(self.variant_name(), proc_macro2::Span::call_site());
            quote::quote! { ::bevy::window::CursorGrabMode::#v }
        }
    }
}

// ── WindowTheme ───────────────────────────────────────────────────────────────

window_enum!(WindowTheme, bevy::window::WindowTheme);

#[reflect_methods]
impl WindowTheme {
    /// Variant name: `"Light"` or `"Dark"`.
    #[tracing::instrument(skip(self))]
    pub fn variant_name(&self) -> &'static str {
        match *self.0 {
            bevy::window::WindowTheme::Light => "Light",
            bevy::window::WindowTheme::Dark => "Dark",
        }
    }

    /// Returns `true` if the light theme is active.
    #[tracing::instrument(skip(self))]
    pub fn is_light(&self) -> bool {
        matches!(*self.0, bevy::window::WindowTheme::Light)
    }

    /// Returns `true` if the dark theme is active.
    #[tracing::instrument(skip(self))]
    pub fn is_dark(&self) -> bool {
        matches!(*self.0, bevy::window::WindowTheme::Dark)
    }
}

mod emit_windowtheme {
    use super::WindowTheme;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for WindowTheme {
        fn to_code_literal(&self) -> TokenStream {
            let v = proc_macro2::Ident::new(self.variant_name(), proc_macro2::Span::call_site());
            quote::quote! { ::bevy::window::WindowTheme::#v }
        }
    }
}

// ── CompositeAlphaMode ────────────────────────────────────────────────────────

window_enum!(CompositeAlphaMode, bevy::window::CompositeAlphaMode);

#[reflect_methods]
impl CompositeAlphaMode {
    /// Variant name.
    #[tracing::instrument(skip(self))]
    pub fn variant_name(&self) -> &'static str {
        use bevy::window::CompositeAlphaMode as C;
        match *self.0 {
            C::Auto => "Auto",
            C::Opaque => "Opaque",
            C::PreMultiplied => "PreMultiplied",
            C::PostMultiplied => "PostMultiplied",
            C::Inherit => "Inherit",
        }
    }

    /// Returns `true` if this is the `Auto` mode.
    #[tracing::instrument(skip(self))]
    pub fn is_auto(&self) -> bool {
        matches!(*self.0, bevy::window::CompositeAlphaMode::Auto)
    }

    /// Returns `true` if this is the `Opaque` mode.
    #[tracing::instrument(skip(self))]
    pub fn is_opaque(&self) -> bool {
        matches!(*self.0, bevy::window::CompositeAlphaMode::Opaque)
    }
}

mod emit_composite {
    use super::CompositeAlphaMode;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for CompositeAlphaMode {
        fn to_code_literal(&self) -> TokenStream {
            let v = proc_macro2::Ident::new(self.variant_name(), proc_macro2::Span::call_site());
            quote::quote! { ::bevy::window::CompositeAlphaMode::#v }
        }
    }
}

// ── WindowResolution ──────────────────────────────────────────────────────────

elicit_newtype!(bevy::window::WindowResolution, as WindowResolution);
elicit_newtype_traits!(WindowResolution, bevy::window::WindowResolution, []);

impl serde::Serialize for WindowResolution {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = s.serialize_map(Some(2))?;
        map.serialize_entry("width", &self.0.width())?;
        map.serialize_entry("height", &self.0.height())?;
        map.end()
    }
}
impl<'de> serde::Deserialize<'de> for WindowResolution {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = WindowResolution;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "WindowResolution {{ width, height }}")
            }
            fn visit_map<A: MapAccess<'de>>(
                self,
                mut map: A,
            ) -> Result<WindowResolution, A::Error> {
                let mut w = 1280.0f32;
                let mut h = 720.0f32;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "width" => w = map.next_value()?,
                        "height" => h = map.next_value()?,
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(WindowResolution(Arc::new(
                    bevy::window::WindowResolution::new(w as u32, h as u32),
                )))
            }
        }
        d.deserialize_map(V)
    }
}
impl From<WindowResolution> for bevy::window::WindowResolution {
    fn from(v: WindowResolution) -> Self {
        Arc::try_unwrap(v.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

#[reflect_methods]
impl WindowResolution {
    /// Physical width in pixels.
    #[tracing::instrument(skip(self))]
    pub fn width(&self) -> f32 {
        self.0.width()
    }

    /// Physical height in pixels.
    #[tracing::instrument(skip(self))]
    pub fn height(&self) -> f32 {
        self.0.height()
    }

    /// Physical width in integer pixels.
    #[tracing::instrument(skip(self))]
    pub fn physical_width(&self) -> u32 {
        self.0.physical_width()
    }

    /// Physical height in integer pixels.
    #[tracing::instrument(skip(self))]
    pub fn physical_height(&self) -> u32 {
        self.0.physical_height()
    }

    /// Scale factor.
    #[tracing::instrument(skip(self))]
    pub fn scale_factor(&self) -> f32 {
        self.0.scale_factor()
    }

    /// Override scale factor, if one has been set.
    #[tracing::instrument(skip(self))]
    pub fn scale_factor_override(&self) -> Option<f32> {
        self.0.scale_factor_override()
    }

    /// Aspect ratio (width / height).
    #[tracing::instrument(skip(self))]
    pub fn aspect_ratio(&self) -> f32 {
        self.0.width() / self.0.height().max(1.0)
    }

    /// Constructs a new resolution (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn new_resolution(&self, width: f32, height: f32) -> WindowResolution {
        WindowResolution::from(bevy::window::WindowResolution::new(
            width as u32,
            height as u32,
        ))
    }
}

mod emit_resolution {
    use super::WindowResolution;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for WindowResolution {
        fn to_code_literal(&self) -> TokenStream {
            let (w, h) = (self.0.physical_width(), self.0.physical_height());
            quote::quote! { ::bevy::window::WindowResolution::new(#w, #h) }
        }
    }
}
impl elicitation::ElicitComplete for WindowResolution {}

// ── Window ────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::window::Window, as Window);
elicit_newtype_traits!(Window, bevy::window::Window, []);

impl serde::Serialize for Window {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = s.serialize_map(Some(4))?;
        map.serialize_entry("title", &self.0.title)?;
        map.serialize_entry("width", &self.0.resolution.width())?;
        map.serialize_entry("height", &self.0.resolution.height())?;
        map.serialize_entry("resizable", &self.0.resizable)?;
        map.end()
    }
}
impl<'de> serde::Deserialize<'de> for Window {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = Window;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "Window object")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Window, A::Error> {
                let mut title = "App".to_string();
                let mut w = 1280.0f32;
                let mut h = 720.0f32;
                let mut resizable = true;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "title" => title = map.next_value()?,
                        "width" => w = map.next_value()?,
                        "height" => h = map.next_value()?,
                        "resizable" => resizable = map.next_value()?,
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Window(Arc::new(bevy::window::Window {
                    title,
                    resolution: bevy::window::WindowResolution::new(w as u32, h as u32),
                    resizable,
                    ..Default::default()
                })))
            }
        }
        d.deserialize_map(V)
    }
}
impl From<Window> for bevy::window::Window {
    fn from(v: Window) -> Self {
        Arc::try_unwrap(v.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

#[reflect_methods]
impl Window {
    /// Window title.
    #[tracing::instrument(skip(self))]
    pub fn title(&self) -> String {
        self.0.title.clone()
    }

    /// Window width in logical pixels.
    #[tracing::instrument(skip(self))]
    pub fn width(&self) -> f32 {
        self.0.resolution.width()
    }

    /// Window height in logical pixels.
    #[tracing::instrument(skip(self))]
    pub fn height(&self) -> f32 {
        self.0.resolution.height()
    }

    /// Returns `true` if the window is resizable.
    #[tracing::instrument(skip(self))]
    pub fn resizable(&self) -> bool {
        self.0.resizable
    }

    /// Returns `true` if decorations (title bar, borders) are shown.
    #[tracing::instrument(skip(self))]
    pub fn decorations(&self) -> bool {
        self.0.decorations
    }

    /// Returns `true` if the window has a transparent background.
    #[tracing::instrument(skip(self))]
    pub fn transparent(&self) -> bool {
        self.0.transparent
    }

    /// Returns `true` if the window is currently focused.
    #[tracing::instrument(skip(self))]
    pub fn focused(&self) -> bool {
        self.0.focused
    }

    /// Returns the cursor resolution wrapping the window resolution.
    #[tracing::instrument(skip(self))]
    pub fn resolution(&self) -> WindowResolution {
        WindowResolution::from(self.0.resolution.clone())
    }

    /// Current window mode.
    #[tracing::instrument(skip(self))]
    pub fn mode(&self) -> WindowMode {
        WindowMode::from(self.0.mode)
    }

    /// Present mode.
    #[tracing::instrument(skip(self))]
    pub fn present_mode(&self) -> PresentMode {
        PresentMode::from(self.0.present_mode)
    }
}

mod emit_window {
    use super::Window;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Window {
        fn to_code_literal(&self) -> TokenStream {
            let title = &self.0.title;
            let w = self.0.resolution.physical_width();
            let h = self.0.resolution.physical_height();
            let resizable = self.0.resizable;
            quote::quote! {
                ::bevy::window::Window {
                    title: #title.to_string(),
                    resolution: ::bevy::window::WindowResolution::new(#w, #h),
                    resizable: #resizable,
                    ..::std::default::Default::default()
                }
            }
        }
    }
}
impl elicitation::ElicitComplete for Window {}

// ── PrimaryWindow ─────────────────────────────────────────────────────────────

elicit_newtype!(bevy::window::PrimaryWindow, as PrimaryWindow);
elicit_newtype_traits!(PrimaryWindow, bevy::window::PrimaryWindow, [eq]);

impl serde::Serialize for PrimaryWindow {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str("PrimaryWindow")
    }
}
impl<'de> serde::Deserialize<'de> for PrimaryWindow {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        String::deserialize(d)?;
        Ok(PrimaryWindow(std::sync::Arc::new(
            bevy::window::PrimaryWindow,
        )))
    }
}

mod emit_primary_window {
    use super::PrimaryWindow;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for PrimaryWindow {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::bevy::window::PrimaryWindow }
        }
    }
}
impl elicitation::ElicitComplete for PrimaryWindow {}

// ── WindowResizeConstraints ───────────────────────────────────────────────────

elicit_newtype!(bevy::window::WindowResizeConstraints, as WindowResizeConstraints);
elicit_newtype_traits!(
    WindowResizeConstraints,
    bevy::window::WindowResizeConstraints,
    [eq]
);

impl serde::Serialize for WindowResizeConstraints {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut st = s.serialize_struct("WindowResizeConstraints", 4)?;
        st.serialize_field("min_width", &self.0.min_width)?;
        st.serialize_field("min_height", &self.0.min_height)?;
        st.serialize_field("max_width", &self.0.max_width)?;
        st.serialize_field("max_height", &self.0.max_height)?;
        st.end()
    }
}
impl<'de> serde::Deserialize<'de> for WindowResizeConstraints {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let v: serde_json::Value = serde_json::Value::deserialize(d)?;
        let min_width = v["min_width"].as_f64().unwrap_or(180.0) as f32;
        let min_height = v["min_height"].as_f64().unwrap_or(120.0) as f32;
        let max_width = v["max_width"].as_f64().unwrap_or(f32::INFINITY as f64) as f32;
        let max_height = v["max_height"].as_f64().unwrap_or(f32::INFINITY as f64) as f32;
        Ok(WindowResizeConstraints(std::sync::Arc::new(
            bevy::window::WindowResizeConstraints {
                min_width,
                min_height,
                max_width,
                max_height,
            },
        )))
    }
}

#[reflect_methods]
impl WindowResizeConstraints {
    /// Returns the minimum width.
    #[tracing::instrument(skip(self))]
    pub fn min_width(&self) -> f32 {
        self.0.min_width
    }
    /// Returns the minimum height.
    #[tracing::instrument(skip(self))]
    pub fn min_height(&self) -> f32 {
        self.0.min_height
    }
}

mod emit_window_resize_constraints {
    use super::WindowResizeConstraints;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for WindowResizeConstraints {
        fn to_code_literal(&self) -> TokenStream {
            let min_w = self.0.min_width;
            let min_h = self.0.min_height;
            let max_w = self.0.max_width;
            let max_h = self.0.max_height;
            quote::quote! {
                ::bevy::window::WindowResizeConstraints {
                    min_width: #min_w,
                    min_height: #min_h,
                    max_width: #max_w,
                    max_height: #max_h,
                }
            }
        }
    }
}
impl elicitation::ElicitComplete for WindowResizeConstraints {}

// ── ScreenEdge ────────────────────────────────────────────────────────────────

window_enum!(ScreenEdge, bevy::window::ScreenEdge);

#[reflect_methods]
impl ScreenEdge {
    /// Returns `true` if no edge is selected.
    #[tracing::instrument(skip(self))]
    pub fn is_none(&self) -> bool {
        matches!(*self.0, bevy::window::ScreenEdge::None)
    }

    /// Returns `true` if all edges are selected.
    #[tracing::instrument(skip(self))]
    pub fn is_all(&self) -> bool {
        matches!(*self.0, bevy::window::ScreenEdge::All)
    }
}

mod emit_impls_screen_edge {
    use super::ScreenEdge;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for ScreenEdge {
        fn to_code_literal(&self) -> TokenStream {
            use bevy::window::ScreenEdge as S;
            match *self.0 {
                S::None => quote::quote! { ::bevy::window::ScreenEdge::None },
                S::Top => quote::quote! { ::bevy::window::ScreenEdge::Top },
                S::Left => quote::quote! { ::bevy::window::ScreenEdge::Left },
                S::Bottom => quote::quote! { ::bevy::window::ScreenEdge::Bottom },
                S::Right => quote::quote! { ::bevy::window::ScreenEdge::Right },
                S::All => quote::quote! { ::bevy::window::ScreenEdge::All },
            }
        }
    }
}

// ── EnabledButtons ────────────────────────────────────────────────────────────

elicit_newtype!(bevy::window::EnabledButtons, as EnabledButtons, forward_serde);
elicit_newtype_traits!(EnabledButtons, bevy::window::EnabledButtons, [eq]);

impl From<EnabledButtons> for bevy::window::EnabledButtons {
    fn from(v: EnabledButtons) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl EnabledButtons {
    /// Returns `true` if the minimize button is enabled.
    #[tracing::instrument(skip(self))]
    pub fn minimize(&self) -> bool {
        self.0.minimize
    }

    /// Returns `true` if the maximize button is enabled.
    #[tracing::instrument(skip(self))]
    pub fn maximize(&self) -> bool {
        self.0.maximize
    }
}

mod emit_impls_enabled_buttons {
    use super::EnabledButtons;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for EnabledButtons {
        fn to_code_literal(&self) -> TokenStream {
            let min = self.0.minimize;
            let max = self.0.maximize;
            quote::quote! {
                ::bevy::window::EnabledButtons {
                    minimize: #min,
                    maximize: #max,
                }
            }
        }
    }
}

impl elicitation::ElicitComplete for EnabledButtons {}

// ── MonitorSelection ──────────────────────────────────────────────────────────

elicit_newtype!(bevy::window::MonitorSelection, as MonitorSelection, forward_serde);
elicit_newtype_traits!(MonitorSelection, bevy::window::MonitorSelection, [eq]);

impl From<MonitorSelection> for bevy::window::MonitorSelection {
    fn from(v: MonitorSelection) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl MonitorSelection {
    /// Returns `true` if this selects the primary monitor.
    #[tracing::instrument(skip(self))]
    pub fn is_primary(&self) -> bool {
        matches!(*self.0, bevy::window::MonitorSelection::Primary)
    }

    /// Returns the index if `Index` variant.
    #[tracing::instrument(skip(self))]
    pub fn get_index(&self) -> Option<usize> {
        match *self.0 {
            bevy::window::MonitorSelection::Index(i) => Some(i),
            _ => None,
        }
    }
}

mod emit_impls_monitor_selection {
    use super::MonitorSelection;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for MonitorSelection {
        fn to_code_literal(&self) -> TokenStream {
            use bevy::window::MonitorSelection as M;
            match *self.0 {
                M::Current => quote::quote! { ::bevy::window::MonitorSelection::Current },
                M::Primary => quote::quote! { ::bevy::window::MonitorSelection::Primary },
                M::Index(i) => quote::quote! { ::bevy::window::MonitorSelection::Index(#i) },
                M::Entity(e) => {
                    let bits = e.to_bits();
                    quote::quote! {
                        ::bevy::window::MonitorSelection::Entity(
                            ::bevy::ecs::entity::Entity::from_bits(#bits)
                        )
                    }
                }
            }
        }
    }
}

impl elicitation::ElicitComplete for MonitorSelection {}

// ── CursorOptions ─────────────────────────────────────────────────────────────

elicit_newtype!(bevy::window::CursorOptions, as CursorOptions, forward_serde);
elicit_newtype_traits!(CursorOptions, bevy::window::CursorOptions, []);

impl From<CursorOptions> for bevy::window::CursorOptions {
    fn from(v: CursorOptions) -> Self {
        (*v.0).clone()
    }
}

#[reflect_methods]
impl CursorOptions {
    /// Returns whether the cursor is visible.
    #[tracing::instrument(skip(self))]
    pub fn visible(&self) -> bool {
        self.0.visible
    }

    /// Returns whether hit-testing is enabled.
    #[tracing::instrument(skip(self))]
    pub fn hit_test(&self) -> bool {
        self.0.hit_test
    }
}

mod emit_impls_cursor_options {
    use super::CursorOptions;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for CursorOptions {
        fn to_code_literal(&self) -> TokenStream {
            let json = serde_json::to_string(&*self.0).unwrap_or_default();
            quote::quote! {
                ::elicit_bevy::CursorOptions::from(
                    ::serde_json::from_str::<::bevy::window::CursorOptions>(#json).unwrap()
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for CursorOptions {}

// ── shadow_elicitation (module-local) ────────────────────────────────────────

/// Generates the full `Elicitation` trait chain for a shadow type with fields.
macro_rules! shadow_elicitation {
    ($name:ident) => {
        impl elicitation::Prompt for $name {
            fn prompt() -> Option<&'static str> {
                None
            }
        }

        impl elicitation::Elicitation for $name {
            type Style = ();

            async fn elicit<C: elicitation::ElicitCommunicator>(
                communicator: &C,
            ) -> elicitation::ElicitResult<Self> {
                let response = communicator
                    .send_prompt(concat!("Enter value for ", stringify!($name)))
                    .await?;
                serde_json::from_str(&response)
                    .or_else(|_| serde_json::from_str::<Self>(&format!("\"{}\"", response)))
                    .map_err(|e| {
                        elicitation::ElicitError::new(elicitation::ElicitErrorKind::ParseError(
                            format!("Invalid {}: {}", stringify!($name), e),
                        ))
                    })
            }

            fn kani_proof() -> elicitation::proc_macro2::TokenStream {
                elicitation::verification::proof_helpers::kani_trusted_opaque(stringify!($name))
            }

            fn verus_proof() -> elicitation::proc_macro2::TokenStream {
                elicitation::verification::proof_helpers::verus_trusted_opaque(stringify!($name))
            }

            fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
                elicitation::verification::proof_helpers::creusot_trusted_opaque(stringify!($name))
            }
        }

        impl elicitation::ElicitIntrospect for $name {
            fn pattern() -> elicitation::ElicitationPattern {
                elicitation::ElicitationPattern::Primitive
            }

            fn metadata() -> elicitation::TypeMetadata {
                elicitation::TypeMetadata {
                    type_name: stringify!($name),
                    description: None,
                    details: elicitation::PatternDetails::Primitive,
                }
            }
        }

        impl elicitation::ElicitPromptTree for $name {
            fn prompt_tree() -> elicitation::PromptTree {
                elicitation::PromptTree::Leaf {
                    prompt: stringify!($name).to_string(),
                    type_name: stringify!($name).to_string(),
                }
            }
        }

        impl elicitation::ElicitSpec for $name {
            fn type_spec() -> elicitation::TypeSpec {
                elicitation::TypeSpecBuilder::default()
                    .type_name(stringify!($name).to_string())
                    .summary(concat!("Shadow type for `", stringify!($name), "`.").to_string())
                    .build()
                    .expect("valid TypeSpec")
            }
        }

        impl elicitation::ElicitComplete for $name {}
    };
}

// ── SystemCursorIcon ──────────────────────────────────────────────────────────

/// Shadow of [`bevy::window::SystemCursorIcon`].
///
/// All variants correspond directly to standard OS cursor shapes.
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub enum SystemCursorIcon {
    /// Platform-default arrow cursor.
    #[default]
    Default,
    /// Context menu cursor.
    ContextMenu,
    /// Help/question cursor.
    Help,
    /// Pointer (hand) cursor for links.
    Pointer,
    /// Progress indicator cursor.
    Progress,
    /// Busy/wait cursor.
    Wait,
    /// Spreadsheet cell selection cursor.
    Cell,
    /// Crosshair cursor.
    Crosshair,
    /// Text insertion cursor (I-beam).
    Text,
    /// Vertical text insertion cursor.
    VerticalText,
    /// Alias/shortcut cursor.
    Alias,
    /// Copy cursor.
    Copy,
    /// Move cursor.
    Move,
    /// No-drop cursor.
    NoDrop,
    /// Not-allowed cursor.
    NotAllowed,
    /// Grab cursor.
    Grab,
    /// Grabbing cursor.
    Grabbing,
    /// East resize cursor.
    EResize,
    /// North resize cursor.
    NResize,
    /// North-east resize cursor.
    NeResize,
    /// North-west resize cursor.
    NwResize,
    /// South resize cursor.
    SResize,
    /// South-east resize cursor.
    SeResize,
    /// South-west resize cursor.
    SwResize,
    /// West resize cursor.
    WResize,
    /// East-west resize cursor.
    EwResize,
    /// North-south resize cursor.
    NsResize,
    /// North-east/south-west resize cursor.
    NeswResize,
    /// North-west/south-east resize cursor.
    NwseResize,
    /// Column resize cursor.
    ColResize,
    /// Row resize cursor.
    RowResize,
    /// All-scroll cursor.
    AllScroll,
    /// Zoom-in cursor.
    ZoomIn,
    /// Zoom-out cursor.
    ZoomOut,
}

impl From<SystemCursorIcon> for bevy::window::SystemCursorIcon {
    fn from(v: SystemCursorIcon) -> Self {
        use SystemCursorIcon as S;
        use bevy::window::SystemCursorIcon as B;
        match v {
            S::Default => B::Default,
            S::ContextMenu => B::ContextMenu,
            S::Help => B::Help,
            S::Pointer => B::Pointer,
            S::Progress => B::Progress,
            S::Wait => B::Wait,
            S::Cell => B::Cell,
            S::Crosshair => B::Crosshair,
            S::Text => B::Text,
            S::VerticalText => B::VerticalText,
            S::Alias => B::Alias,
            S::Copy => B::Copy,
            S::Move => B::Move,
            S::NoDrop => B::NoDrop,
            S::NotAllowed => B::NotAllowed,
            S::Grab => B::Grab,
            S::Grabbing => B::Grabbing,
            S::EResize => B::EResize,
            S::NResize => B::NResize,
            S::NeResize => B::NeResize,
            S::NwResize => B::NwResize,
            S::SResize => B::SResize,
            S::SeResize => B::SeResize,
            S::SwResize => B::SwResize,
            S::WResize => B::WResize,
            S::EwResize => B::EwResize,
            S::NsResize => B::NsResize,
            S::NeswResize => B::NeswResize,
            S::NwseResize => B::NwseResize,
            S::ColResize => B::ColResize,
            S::RowResize => B::RowResize,
            S::AllScroll => B::AllScroll,
            S::ZoomIn => B::ZoomIn,
            S::ZoomOut => B::ZoomOut,
        }
    }
}

impl From<bevy::window::SystemCursorIcon> for SystemCursorIcon {
    fn from(v: bevy::window::SystemCursorIcon) -> Self {
        use SystemCursorIcon as S;
        use bevy::window::SystemCursorIcon as B;
        match v {
            B::Default => S::Default,
            B::ContextMenu => S::ContextMenu,
            B::Help => S::Help,
            B::Pointer => S::Pointer,
            B::Progress => S::Progress,
            B::Wait => S::Wait,
            B::Cell => S::Cell,
            B::Crosshair => S::Crosshair,
            B::Text => S::Text,
            B::VerticalText => S::VerticalText,
            B::Alias => S::Alias,
            B::Copy => S::Copy,
            B::Move => S::Move,
            B::NoDrop => S::NoDrop,
            B::NotAllowed => S::NotAllowed,
            B::Grab => S::Grab,
            B::Grabbing => S::Grabbing,
            B::EResize => S::EResize,
            B::NResize => S::NResize,
            B::NeResize => S::NeResize,
            B::NwResize => S::NwResize,
            B::SResize => S::SResize,
            B::SeResize => S::SeResize,
            B::SwResize => S::SwResize,
            B::WResize => S::WResize,
            B::EwResize => S::EwResize,
            B::NsResize => S::NsResize,
            B::NeswResize => S::NeswResize,
            B::NwseResize => S::NwseResize,
            B::ColResize => S::ColResize,
            B::RowResize => S::RowResize,
            B::AllScroll => S::AllScroll,
            B::ZoomIn => S::ZoomIn,
            B::ZoomOut => S::ZoomOut,
        }
    }
}

mod emit_system_cursor_icon {
    use super::SystemCursorIcon;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for SystemCursorIcon {
        fn to_code_literal(&self) -> TokenStream {
            let variant = format!("{:?}", self);
            let ident = syn::parse_str::<syn::Ident>(&variant).unwrap();
            quote::quote! { ::bevy::window::SystemCursorIcon::#ident }
        }
    }
}

shadow_elicitation!(SystemCursorIcon);

// ── CursorIcon ────────────────────────────────────────────────────────────────

/// Shadow of [`bevy::window::CursorIcon`].
///
/// Component placed on a window entity to set the visible cursor.
/// Only the `System` variant is supported here; custom image cursors
/// are behind the `custom_cursor` Bevy feature and are not covered.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub enum CursorIcon {
    /// System-provided cursor icon.
    #[default]
    Default,
    /// Named system cursor.
    System(SystemCursorIcon),
}

impl From<CursorIcon> for bevy::window::CursorIcon {
    fn from(v: CursorIcon) -> Self {
        match v {
            CursorIcon::Default => bevy::window::CursorIcon::default(),
            CursorIcon::System(s) => bevy::window::CursorIcon::System(s.into()),
        }
    }
}

impl From<bevy::window::CursorIcon> for CursorIcon {
    fn from(v: bevy::window::CursorIcon) -> Self {
        match v {
            bevy::window::CursorIcon::System(s) => CursorIcon::System(s.into()),
        }
    }
}

mod emit_cursor_icon {
    use super::CursorIcon;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for CursorIcon {
        fn to_code_literal(&self) -> TokenStream {
            match self {
                CursorIcon::Default => {
                    quote::quote! { ::bevy::window::CursorIcon::default() }
                }
                CursorIcon::System(s) => {
                    let s = s.to_code_literal();
                    quote::quote! { ::bevy::window::CursorIcon::System(#s) }
                }
            }
        }
    }
}

shadow_elicitation!(CursorIcon);

// ── WindowPosition ────────────────────────────────────────────────────────────

/// Shadow of [`bevy::window::WindowPosition`].
///
/// Controls where a window is placed on screen at creation time.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub enum WindowPosition {
    /// Window manager decides the position (default).
    #[default]
    Automatic,
    /// Center the window on the given monitor.
    Centered(crate::MonitorSelection),
    /// Place the top-left corner at the given physical-pixel coordinates `[x, y]`.
    At([i32; 2]),
}

impl From<WindowPosition> for bevy::window::WindowPosition {
    fn from(v: WindowPosition) -> Self {
        match v {
            WindowPosition::Automatic => bevy::window::WindowPosition::Automatic,
            WindowPosition::Centered(m) => {
                bevy::window::WindowPosition::Centered(bevy::window::MonitorSelection::from(m))
            }
            WindowPosition::At(pos) => {
                bevy::window::WindowPosition::At(bevy::math::IVec2::new(pos[0], pos[1]))
            }
        }
    }
}

impl From<bevy::window::WindowPosition> for WindowPosition {
    fn from(v: bevy::window::WindowPosition) -> Self {
        match v {
            bevy::window::WindowPosition::Automatic => WindowPosition::Automatic,
            bevy::window::WindowPosition::Centered(m) => {
                WindowPosition::Centered(crate::MonitorSelection::from(m))
            }
            bevy::window::WindowPosition::At(p) => WindowPosition::At([p.x, p.y]),
        }
    }
}

mod emit_window_position {
    use super::WindowPosition;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for WindowPosition {
        fn to_code_literal(&self) -> TokenStream {
            match self {
                WindowPosition::Automatic => {
                    quote::quote! { ::bevy::window::WindowPosition::Automatic }
                }
                WindowPosition::Centered(m) => {
                    let m = m.to_code_literal();
                    quote::quote! { ::bevy::window::WindowPosition::Centered(#m) }
                }
                WindowPosition::At(pos) => {
                    let x = pos[0];
                    let y = pos[1];
                    quote::quote! {
                        ::bevy::window::WindowPosition::At(
                            ::bevy::math::IVec2::new(#x, #y)
                        )
                    }
                }
            }
        }
    }
}

shadow_elicitation!(WindowPosition);

// ── VideoMode ─────────────────────────────────────────────────────────────────

// Shadow of [`bevy::window::VideoMode`].
//
// Describes a monitor video mode: resolution, bit depth, and refresh rate.
elicit_newtype!(bevy::window::VideoMode, as VideoMode);
elicit_newtype_traits!(VideoMode, bevy::window::VideoMode, [eq]);

impl From<VideoMode> for bevy::window::VideoMode {
    fn from(v: VideoMode) -> Self {
        Arc::try_unwrap(v.0).unwrap_or_else(|arc| *arc)
    }
}

impl serde::Serialize for VideoMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let v = &*self.0;
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("physical_size", &[v.physical_size.x, v.physical_size.y])?;
        map.serialize_entry("bit_depth", &v.bit_depth)?;
        map.serialize_entry("refresh_rate_millihertz", &v.refresh_rate_millihertz)?;
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for VideoMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = VideoMode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "a VideoMode JSON object")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<VideoMode, A::Error> {
                let mut physical_size: Option<[u32; 2]> = None;
                let mut bit_depth: Option<u16> = None;
                let mut refresh_rate_millihertz: Option<u32> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "physical_size" => physical_size = Some(map.next_value()?),
                        "bit_depth" => bit_depth = Some(map.next_value()?),
                        "refresh_rate_millihertz" => {
                            refresh_rate_millihertz = Some(map.next_value()?)
                        }
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                let [px, py] = physical_size.unwrap_or([0, 0]);
                Ok(VideoMode(Arc::new(bevy::window::VideoMode {
                    physical_size: bevy::math::UVec2::new(px, py),
                    bit_depth: bit_depth.unwrap_or(32),
                    refresh_rate_millihertz: refresh_rate_millihertz.unwrap_or(60_000),
                })))
            }
        }
        deserializer.deserialize_map(V)
    }
}

#[reflect_methods]
impl VideoMode {
    /// Returns the physical resolution as `[width, height]`.
    #[tracing::instrument(skip(self))]
    pub fn physical_size(&self) -> [u32; 2] {
        [self.0.physical_size.x, self.0.physical_size.y]
    }

    /// Returns the color bit depth.
    #[tracing::instrument(skip(self))]
    pub fn bit_depth(&self) -> u16 {
        self.0.bit_depth
    }

    /// Returns the refresh rate in millihertz.
    #[tracing::instrument(skip(self))]
    pub fn refresh_rate_millihertz(&self) -> u32 {
        self.0.refresh_rate_millihertz
    }
}

mod emit_impls_video_mode {
    use super::VideoMode;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for VideoMode {
        fn to_code_literal(&self) -> TokenStream {
            let px = self.0.physical_size.x;
            let py = self.0.physical_size.y;
            let bd = self.0.bit_depth;
            let rr = self.0.refresh_rate_millihertz;
            quote::quote! {
                ::bevy::window::VideoMode {
                    physical_size: ::bevy::math::UVec2::new(#px, #py),
                    bit_depth: #bd,
                    refresh_rate_millihertz: #rr,
                }
            }
        }
    }
}

impl elicitation::ElicitComplete for VideoMode {}

// ── VideoModeSelection ────────────────────────────────────────────────────────

/// Shadow of [`bevy::window::VideoModeSelection`].
///
/// Selects the video mode for a fullscreen window.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub enum VideoModeSelection {
    /// Use the monitor's current video mode.
    Current,
    /// Use a specific video mode.
    Specific(VideoMode),
}

impl From<bevy::window::VideoModeSelection> for VideoModeSelection {
    fn from(v: bevy::window::VideoModeSelection) -> Self {
        match v {
            bevy::window::VideoModeSelection::Current => VideoModeSelection::Current,
            bevy::window::VideoModeSelection::Specific(m) => {
                VideoModeSelection::Specific(VideoMode::from(m))
            }
        }
    }
}

impl From<VideoModeSelection> for bevy::window::VideoModeSelection {
    fn from(v: VideoModeSelection) -> Self {
        match v {
            VideoModeSelection::Current => bevy::window::VideoModeSelection::Current,
            VideoModeSelection::Specific(m) => {
                bevy::window::VideoModeSelection::Specific(bevy::window::VideoMode::from(m))
            }
        }
    }
}

mod emit_impls_video_mode_selection {
    use super::VideoModeSelection;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for VideoModeSelection {
        fn to_code_literal(&self) -> TokenStream {
            match self {
                VideoModeSelection::Current => {
                    quote::quote! { ::bevy::window::VideoModeSelection::Current }
                }
                VideoModeSelection::Specific(m) => {
                    let m = m.to_code_literal();
                    quote::quote! {
                        ::bevy::window::VideoModeSelection::Specific(#m)
                    }
                }
            }
        }
    }
}

shadow_elicitation!(VideoModeSelection);
