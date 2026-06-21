//! Bevy application and exit status wrappers.
//!
//! [`App`] is a newtype around `bevy::app::App` that can be constructed and
//! queried over MCP.  Because `bevy::app::App` is `!Send + !Sync` (its runner
//! closure is not thread-safe), MCP tool methods on `App` are synchronous —
//! the agent calls them on the same thread that owns the app.
//!
//! [`AppExit`] wraps `bevy::app::AppExit` via `Arc` and is fully MCP-capable.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;
use std::sync::Arc;

// ── App ───────────────────────────────────────────────────────────────────────

/// Shadow of `bevy::app::App`.
///
/// Newtype wrapper that owns `bevy::app::App` and exposes its builder API as
/// MCP tools via `#[reflect_methods]`.  Builder methods consume `self` and
/// return the updated wrapper, matching Bevy's own convention.
///
/// `#[repr(transparent)]` makes it safe to cast `&bevy::app::App` ↔ `&App`
/// via [`ref_cast::RefCast`], which the blanket `bevy::app::Plugin` impl uses
/// for shared-reference parameters (e.g., `ready`).
///
/// For descriptor-based, fully async MCP app construction use
/// [`BevyAppPlugin`](crate::BevyAppPlugin).
#[repr(transparent)]
#[derive(ref_cast::RefCast)]
pub struct App(pub bevy::app::App);

impl App {
    /// Create a new Bevy application.
    #[tracing::instrument]
    pub fn new() -> Self {
        App(bevy::app::App::new())
    }

    /// Add a plugin or plugin group.
    ///
    /// This is not exposed as an MCP tool because `Plugins<M>` is generic.
    /// Use the typed variants (`add_gis_plugin`, etc.) for MCP access.
    pub fn add_plugins<M>(mut self, plugins: impl bevy::app::Plugins<M>) -> Self {
        self.0.add_plugins(plugins);
        self
    }

    /// Add a system to the `Startup` schedule.
    ///
    /// Hides `bevy::app::Startup` from callers; use this instead of
    /// `add_systems(Startup, ...)` to avoid a direct bevy dep.
    pub fn add_startup_system<M>(
        mut self,
        system: impl bevy::ecs::schedule::IntoScheduleConfigs<bevy::ecs::system::ScheduleSystem, M>,
    ) -> Self {
        self.0.add_systems(bevy::app::Startup, system);
        self
    }

    /// Add a system to the `Update` schedule.
    ///
    /// Hides `bevy::app::Update` from callers; use this instead of
    /// `add_systems(Update, ...)` to avoid a direct bevy dep.
    pub fn add_update_system<M>(
        mut self,
        system: impl bevy::ecs::schedule::IntoScheduleConfigs<bevy::ecs::system::ScheduleSystem, M>,
    ) -> Self {
        self.0.add_systems(bevy::app::Update, system);
        self
    }

    /// Run the application to completion and return the exit status.
    pub fn run(mut self) -> bevy::app::AppExit {
        self.0.run()
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("App").finish_non_exhaustive()
    }
}

impl Clone for App {
    /// Returns a fresh `App`; Bevy's `App` is not `Clone`, so no ECS state is preserved.
    ///
    /// This matches the round-trip behaviour of our `Deserialize` impl (which also
    /// creates a fresh app from any JSON input), so no information is lost beyond
    /// what serialisation already loses.
    fn clone(&self) -> Self {
        App(bevy::app::App::new())
    }
}

impl From<bevy::app::App> for App {
    fn from(app: bevy::app::App) -> Self {
        App(app)
    }
}

impl From<App> for bevy::app::App {
    fn from(wrapper: App) -> Self {
        wrapper.0
    }
}

impl serde::Serialize for App {
    /// Serializes as `{}`.  `bevy::app::App` carries runtime ECS state that
    /// has no meaningful JSON representation.
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        serializer.serialize_map(Some(0))?.end()
    }
}

impl<'de> serde::Deserialize<'de> for App {
    /// Deserializes from any map object by constructing a fresh
    /// `bevy::app::App::new()`.
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        struct AppVisitor;
        impl<'de> Visitor<'de> for AppVisitor {
            type Value = App;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "an empty object {{}} for a Bevy App handle")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<App, A::Error> {
                while map.next_entry::<String, serde::de::IgnoredAny>()?.is_some() {}
                Ok(App(bevy::app::App::new()))
            }
        }
        deserializer.deserialize_map(AppVisitor)
    }
}

impl schemars::JsonSchema for App {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("App")
    }
    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({ "type": "object" })
    }
}

#[reflect_methods]
impl App {
    /// Construct a fresh Bevy application.
    ///
    /// The `&self` receiver is ignored; this is a factory constructor.
    #[tracing::instrument(skip(self))]
    pub fn new_app(&self) -> App {
        App(bevy::app::App::new())
    }

    /// Add the GIS render backend plugin and return the updated app.
    ///
    /// Creates a fresh [`BevyGisRenderCtx`](crate::BevyGisRenderCtx) and
    /// registers [`BevyGisBackend`](crate::BevyGisBackend) as a Bevy ECS
    /// resource accessible in systems via `Res<BevyGisBackend>`.
    ///
    /// This is a consuming method: the input `App` is consumed.
    #[tracing::instrument(skip(self))]
    pub fn add_gis_plugin(mut self) -> App {
        use crate::{BevyGisBackend, BevyGisPlugin, BevyGisRenderCtx};
        let ctx = Arc::new(BevyGisRenderCtx::new());
        let backend = BevyGisBackend::new(ctx);
        self.0.add_plugins(BevyGisPlugin::new(backend));
        self
    }
}

mod emit_impls_app {
    use super::App;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for App {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::bevy::app::App::new() }
        }
    }
}

// ── AppExit ───────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::app::AppExit, as AppExit);
elicit_newtype_traits!(AppExit, bevy::app::AppExit, [eq]);

impl serde::Serialize for AppExit {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(2))?;
        match *self.0 {
            bevy::app::AppExit::Success => {
                map.serialize_entry("type", "Success")?;
            }
            bevy::app::AppExit::Error(code) => {
                map.serialize_entry("type", "Error")?;
                map.serialize_entry("code", &code.get())?;
            }
        }
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for AppExit {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{self, MapAccess, Visitor};
        struct AppExitVisitor;
        impl<'de> Visitor<'de> for AppExitVisitor {
            type Value = AppExit;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, r#"an object with "type": "Success" or "type": "Error""#)
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<AppExit, A::Error> {
                let mut ty: Option<String> = None;
                let mut code: Option<u8> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "type" => ty = Some(map.next_value()?),
                        "code" => code = Some(map.next_value()?),
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                let ty = ty.ok_or_else(|| de::Error::missing_field("type"))?;
                let inner = match ty.as_str() {
                    "Success" => bevy::app::AppExit::Success,
                    "Error" => {
                        let c = code.unwrap_or(1);
                        bevy::app::AppExit::from_code(c)
                    }
                    other => {
                        return Err(de::Error::unknown_variant(other, &["Success", "Error"]));
                    }
                };
                Ok(AppExit(Arc::new(inner)))
            }
        }
        deserializer.deserialize_map(AppExitVisitor)
    }
}

impl From<AppExit> for bevy::app::AppExit {
    fn from(v: AppExit) -> Self {
        Arc::try_unwrap(v.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

#[reflect_methods]
impl AppExit {
    /// Returns `true` if this is `AppExit::Success`.
    #[tracing::instrument(skip(self))]
    pub fn is_success(&self) -> bool {
        self.0.is_success()
    }

    /// Returns `true` if this is `AppExit::Error`.
    #[tracing::instrument(skip(self))]
    pub fn is_error(&self) -> bool {
        self.0.is_error()
    }

    /// Returns the error code if this is `AppExit::Error`, otherwise `None`.
    #[tracing::instrument(skip(self))]
    pub fn error_code(&self) -> Option<u8> {
        match *self.0 {
            bevy::app::AppExit::Error(code) => Some(code.get()),
            bevy::app::AppExit::Success => None,
        }
    }

    /// Construct an `AppExit::Success` value.
    ///
    /// The `&self` receiver is ignored; this is a factory constructor.
    #[tracing::instrument(skip(self))]
    pub fn success_constructor(&self) -> AppExit {
        AppExit(Arc::new(bevy::app::AppExit::Success))
    }

    /// Construct an `AppExit::Error` with exit code 1.
    ///
    /// The `&self` receiver is ignored; this is a factory constructor.
    #[tracing::instrument(skip(self))]
    pub fn error_constructor(&self) -> AppExit {
        AppExit(Arc::new(bevy::app::AppExit::error()))
    }

    /// Construct an `AppExit` from a numeric exit code.
    ///
    /// Code 0 maps to `AppExit::Success`; non-zero maps to
    /// `AppExit::Error` carrying that code.
    ///
    /// The `&self` receiver is ignored; this is a factory constructor.
    #[tracing::instrument(skip(self))]
    pub fn from_code_constructor(&self, code: u8) -> AppExit {
        AppExit(Arc::new(bevy::app::AppExit::from_code(code)))
    }
}

// ── ElicitComplete + ToCodeLiteral ───────────────────────────────────────────

mod emit_impls {
    use super::AppExit;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for AppExit {
        fn to_code_literal(&self) -> TokenStream {
            match *self.0 {
                bevy::app::AppExit::Success => quote::quote! {
                    ::elicit_bevy::AppExit::from(::bevy::app::AppExit::Success)
                },
                bevy::app::AppExit::Error(code) => {
                    let c = code.get();
                    quote::quote! {
                        ::elicit_bevy::AppExit::from(
                            ::bevy::app::AppExit::from_code(#c)
                        )
                    }
                }
            }
        }
    }
}

impl elicitation::ElicitComplete for AppExit {}
