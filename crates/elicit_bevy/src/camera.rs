//! Bevy camera shadow types.
//!
//! Covers [`Camera`], [`Projection`], [`OrthographicProjection`], [`PerspectiveProjection`],
//! [`ScalingMode`], [`Visibility`], [`InheritedVisibility`], and [`ViewVisibility`].

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;
use std::sync::Arc;

// ── ScalingMode ───────────────────────────────────────────────────────────────

elicit_newtype!(bevy::camera::ScalingMode, as ScalingMode);
elicit_newtype_traits!(ScalingMode, bevy::camera::ScalingMode, []);

impl serde::Serialize for ScalingMode {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}

impl<'de> serde::Deserialize<'de> for ScalingMode {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::camera::ScalingMode::deserialize(d).map(|v| ScalingMode(Arc::new(v)))
    }
}

impl From<ScalingMode> for bevy::camera::ScalingMode {
    fn from(v: ScalingMode) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl ScalingMode {
    /// Variant name, e.g. `"WindowSize"`, `"Fixed"`, `"AutoMin"`.
    #[tracing::instrument(skip(self))]
    pub fn variant_name(&self) -> &'static str {
        use bevy::camera::ScalingMode as S;
        match *self.0 {
            S::Fixed { .. } => "Fixed",
            S::WindowSize => "WindowSize",
            S::AutoMin { .. } => "AutoMin",
            S::AutoMax { .. } => "AutoMax",
            S::FixedVertical { .. } => "FixedVertical",
            S::FixedHorizontal { .. } => "FixedHorizontal",
        }
    }
}

mod emit_scalingmode {
    use super::ScalingMode;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for ScalingMode {
        fn to_code_literal(&self) -> TokenStream {
            let json = serde_json::to_string(&**self).unwrap_or_default();
            quote::quote! {
                ::serde_json::from_str::<::bevy::camera::ScalingMode>(#json).unwrap()
            }
        }
    }
}

impl elicitation::ElicitComplete for ScalingMode {}

// ── OrthographicProjection ────────────────────────────────────────────────────

elicit_newtype!(bevy::camera::OrthographicProjection, as OrthographicProjection);
elicit_newtype_traits!(
    OrthographicProjection,
    bevy::camera::OrthographicProjection,
    []
);

impl serde::Serialize for OrthographicProjection {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = s.serialize_map(Some(4))?;
        map.serialize_entry("near", &self.0.near)?;
        map.serialize_entry("far", &self.0.far)?;
        map.serialize_entry("scale", &self.0.scale)?;
        map.serialize_entry(
            "viewport_origin",
            &[self.0.viewport_origin.x, self.0.viewport_origin.y],
        )?;
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for OrthographicProjection {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = OrthographicProjection;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "OrthographicProjection")
            }
            fn visit_map<A: MapAccess<'de>>(
                self,
                mut map: A,
            ) -> Result<OrthographicProjection, A::Error> {
                let mut proj = bevy::camera::OrthographicProjection::default_3d();
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "near" => proj.near = map.next_value()?,
                        "far" => proj.far = map.next_value()?,
                        "scale" => proj.scale = map.next_value()?,
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(OrthographicProjection(Arc::new(proj)))
            }
        }
        d.deserialize_map(V)
    }
}

impl From<OrthographicProjection> for bevy::camera::OrthographicProjection {
    fn from(v: OrthographicProjection) -> Self {
        Arc::try_unwrap(v.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

#[reflect_methods]
impl OrthographicProjection {
    /// Near clipping plane distance.
    #[tracing::instrument(skip(self))]
    pub fn near(&self) -> f32 {
        self.0.near
    }

    /// Far clipping plane distance.
    #[tracing::instrument(skip(self))]
    pub fn far(&self) -> f32 {
        self.0.far
    }

    /// Orthographic scale factor.
    #[tracing::instrument(skip(self))]
    pub fn scale(&self) -> f32 {
        self.0.scale
    }

    /// Viewport origin X.
    #[tracing::instrument(skip(self))]
    pub fn viewport_origin_x(&self) -> f32 {
        self.0.viewport_origin.x
    }

    /// Viewport origin Y.
    #[tracing::instrument(skip(self))]
    pub fn viewport_origin_y(&self) -> f32 {
        self.0.viewport_origin.y
    }

    /// Scaling mode variant name.
    #[tracing::instrument(skip(self))]
    pub fn scaling_mode(&self) -> ScalingMode {
        ScalingMode::from(self.0.scaling_mode)
    }
}

mod emit_ortho {
    use super::OrthographicProjection;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for OrthographicProjection {
        fn to_code_literal(&self) -> TokenStream {
            let (near, far, scale) = (self.0.near, self.0.far, self.0.scale);
            quote::quote! {{
                let mut p = ::bevy::camera::OrthographicProjection::default_3d();
                p.near = #near;
                p.far = #far;
                p.scale = #scale;
                p
            }}
        }
    }
}

impl elicitation::ElicitComplete for OrthographicProjection {}

// ── PerspectiveProjection ─────────────────────────────────────────────────────

elicit_newtype!(bevy::camera::PerspectiveProjection, as PerspectiveProjection);
elicit_newtype_traits!(
    PerspectiveProjection,
    bevy::camera::PerspectiveProjection,
    []
);

impl serde::Serialize for PerspectiveProjection {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = s.serialize_map(Some(4))?;
        map.serialize_entry("fov", &self.0.fov)?;
        map.serialize_entry("aspect_ratio", &self.0.aspect_ratio)?;
        map.serialize_entry("near", &self.0.near)?;
        map.serialize_entry("far", &self.0.far)?;
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for PerspectiveProjection {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = PerspectiveProjection;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "PerspectiveProjection")
            }
            fn visit_map<A: MapAccess<'de>>(
                self,
                mut map: A,
            ) -> Result<PerspectiveProjection, A::Error> {
                let mut proj = bevy::camera::PerspectiveProjection::default();
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "fov" => proj.fov = map.next_value()?,
                        "aspect_ratio" => proj.aspect_ratio = map.next_value()?,
                        "near" => proj.near = map.next_value()?,
                        "far" => proj.far = map.next_value()?,
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(PerspectiveProjection(Arc::new(proj)))
            }
        }
        d.deserialize_map(V)
    }
}

impl From<PerspectiveProjection> for bevy::camera::PerspectiveProjection {
    fn from(v: PerspectiveProjection) -> Self {
        Arc::try_unwrap(v.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

#[reflect_methods]
impl PerspectiveProjection {
    /// Vertical field of view in radians.
    #[tracing::instrument(skip(self))]
    pub fn fov(&self) -> f32 {
        self.0.fov
    }

    /// Aspect ratio (width / height).
    #[tracing::instrument(skip(self))]
    pub fn aspect_ratio(&self) -> f32 {
        self.0.aspect_ratio
    }

    /// Near clipping plane distance.
    #[tracing::instrument(skip(self))]
    pub fn near(&self) -> f32 {
        self.0.near
    }

    /// Far clipping plane distance.
    #[tracing::instrument(skip(self))]
    pub fn far(&self) -> f32 {
        self.0.far
    }
}

mod emit_persp {
    use super::PerspectiveProjection;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for PerspectiveProjection {
        fn to_code_literal(&self) -> TokenStream {
            let (fov, ar, near, far) = (self.0.fov, self.0.aspect_ratio, self.0.near, self.0.far);
            quote::quote! {
                ::bevy::camera::PerspectiveProjection {
                    fov: #fov, aspect_ratio: #ar, near: #near, far: #far,
                }
            }
        }
    }
}

impl elicitation::ElicitComplete for PerspectiveProjection {}

// ── Projection ────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::camera::Projection, as Projection);
elicit_newtype_traits!(Projection, bevy::camera::Projection, []);

impl serde::Serialize for Projection {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use bevy::camera::Projection as P;
        use serde::ser::SerializeMap;
        match &*self.0 {
            P::Perspective(p) => {
                let mut map = s.serialize_map(Some(2))?;
                map.serialize_entry("type", "Perspective")?;
                map.serialize_entry("fov", &p.fov)?;
                map.end()
            }
            P::Orthographic(o) => {
                let mut map = s.serialize_map(Some(2))?;
                map.serialize_entry("type", "Orthographic")?;
                map.serialize_entry("scale", &o.scale)?;
                map.end()
            }
            P::Custom(_) => {
                // Custom projections are not serializable; fall back to perspective.
                let default_p = bevy::camera::PerspectiveProjection::default();
                let mut map = s.serialize_map(Some(2))?;
                map.serialize_entry("type", "Perspective")?;
                map.serialize_entry("fov", &default_p.fov)?;
                map.end()
            }
        }
    }
}

impl<'de> serde::Deserialize<'de> for Projection {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = Projection;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "Projection")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Projection, A::Error> {
                let mut typ = "Perspective".to_string();
                let mut fov = std::f32::consts::PI / 4.0;
                let mut scale = 1.0f32;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "type" => typ = map.next_value()?,
                        "fov" => fov = map.next_value()?,
                        "scale" => scale = map.next_value()?,
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                let inner = if typ == "Orthographic" {
                    let mut o = bevy::camera::OrthographicProjection::default_3d();
                    o.scale = scale;
                    bevy::camera::Projection::Orthographic(o)
                } else {
                    bevy::camera::Projection::Perspective(bevy::camera::PerspectiveProjection {
                        fov,
                        ..Default::default()
                    })
                };
                Ok(Projection(Arc::new(inner)))
            }
        }
        d.deserialize_map(V)
    }
}

impl From<Projection> for bevy::camera::Projection {
    fn from(v: Projection) -> Self {
        Arc::try_unwrap(v.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

#[reflect_methods]
impl Projection {
    /// Returns `true` if this is a perspective projection.
    #[tracing::instrument(skip(self))]
    pub fn is_perspective(&self) -> bool {
        matches!(&*self.0, bevy::camera::Projection::Perspective(_))
    }

    /// Returns `true` if this is an orthographic projection.
    #[tracing::instrument(skip(self))]
    pub fn is_orthographic(&self) -> bool {
        matches!(&*self.0, bevy::camera::Projection::Orthographic(_))
    }

    /// Near clipping plane distance.
    #[tracing::instrument(skip(self))]
    pub fn near(&self) -> f32 {
        use bevy::camera::Projection as P;
        match &*self.0 {
            P::Perspective(p) => p.near,
            P::Orthographic(o) => o.near,
            P::Custom(_) => 0.1,
        }
    }

    /// Far clipping plane distance.
    #[tracing::instrument(skip(self))]
    pub fn far(&self) -> f32 {
        use bevy::camera::Projection as P;
        match &*self.0 {
            P::Perspective(p) => p.far,
            P::Orthographic(o) => o.far,
            P::Custom(_) => 1000.0,
        }
    }

    /// Returns the inner perspective projection (if any).
    #[tracing::instrument(skip(self))]
    pub fn as_perspective(&self) -> Option<PerspectiveProjection> {
        if let bevy::camera::Projection::Perspective(p) = &*self.0 {
            Some(PerspectiveProjection::from(p.clone()))
        } else {
            Option::None
        }
    }

    /// Returns the inner orthographic projection (if any).
    #[tracing::instrument(skip(self))]
    pub fn as_orthographic(&self) -> Option<OrthographicProjection> {
        if let bevy::camera::Projection::Orthographic(o) = &*self.0 {
            Some(OrthographicProjection::from(o.clone()))
        } else {
            Option::None
        }
    }
}

mod emit_projection {
    use super::Projection;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Projection {
        fn to_code_literal(&self) -> TokenStream {
            use bevy::camera::Projection as P;
            match &*self.0 {
                P::Perspective(p) => {
                    let fov = p.fov;
                    quote::quote! {
                        ::bevy::camera::Projection::Perspective(
                            ::bevy::camera::PerspectiveProjection {
                                fov: #fov, ..Default::default()
                            }
                        )
                    }
                }
                P::Orthographic(o) => {
                    let scale = o.scale;
                    quote::quote! {
                        ::bevy::camera::Projection::Orthographic({
                            let mut p = ::bevy::camera::OrthographicProjection::default_3d();
                            p.scale = #scale;
                            p
                        })
                    }
                }
                P::Custom(_) => {
                    quote::quote! {
                        ::bevy::camera::Projection::Perspective(Default::default())
                    }
                }
            }
        }
    }
}

impl elicitation::ElicitComplete for Projection {}

// ── Visibility ────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::camera::visibility::Visibility, as Visibility);
// Visibility has PartialEq + Eq but no Hash (it's a simple 3-variant enum with no Hash derive).
elicit_newtype_traits!(Visibility, bevy::camera::visibility::Visibility, [eq]);

impl serde::Serialize for Visibility {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.variant_name())
    }
}

impl<'de> serde::Deserialize<'de> for Visibility {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        let v = match s.as_str() {
            "Inherited" => bevy::camera::visibility::Visibility::Inherited,
            "Hidden" => bevy::camera::visibility::Visibility::Hidden,
            "Visible" => bevy::camera::visibility::Visibility::Visible,
            other => {
                return Err(serde::de::Error::unknown_variant(
                    other,
                    &["Inherited", "Hidden", "Visible"],
                ));
            }
        };
        Ok(Visibility(Arc::new(v)))
    }
}

impl From<Visibility> for bevy::camera::visibility::Visibility {
    fn from(v: Visibility) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Visibility {
    /// Variant name: `"Inherited"`, `"Hidden"`, or `"Visible"`.
    #[tracing::instrument(skip(self))]
    pub fn variant_name(&self) -> &'static str {
        use bevy::camera::visibility::Visibility as V;
        match *self.0 {
            V::Inherited => "Inherited",
            V::Hidden => "Hidden",
            V::Visible => "Visible",
        }
    }

    /// Returns `true` if this entity is visible (unconditionally).
    #[tracing::instrument(skip(self))]
    pub fn is_visible(&self) -> bool {
        matches!(*self.0, bevy::camera::visibility::Visibility::Visible)
    }

    /// Returns `true` if this entity is hidden.
    #[tracing::instrument(skip(self))]
    pub fn is_hidden(&self) -> bool {
        matches!(*self.0, bevy::camera::visibility::Visibility::Hidden)
    }

    /// Returns `true` if this entity inherits visibility from its parent.
    #[tracing::instrument(skip(self))]
    pub fn is_inherited(&self) -> bool {
        matches!(*self.0, bevy::camera::visibility::Visibility::Inherited)
    }
}

mod emit_visibility {
    use super::Visibility;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Visibility {
        fn to_code_literal(&self) -> TokenStream {
            let v = proc_macro2::Ident::new(self.variant_name(), proc_macro2::Span::call_site());
            quote::quote! { ::bevy::camera::visibility::Visibility::#v }
        }
    }
}

impl elicitation::ElicitComplete for Visibility {}

// ── InheritedVisibility ───────────────────────────────────────────────────────

elicit_newtype!(
    bevy::camera::visibility::InheritedVisibility,
    as InheritedVisibility
);
elicit_newtype_traits!(
    InheritedVisibility,
    bevy::camera::visibility::InheritedVisibility,
    [eq]
);

impl serde::Serialize for InheritedVisibility {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_bool(self.0.get())
    }
}

impl<'de> serde::Deserialize<'de> for InheritedVisibility {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let b = bool::deserialize(d)?;
        Ok(InheritedVisibility(Arc::new(if b {
            bevy::camera::visibility::InheritedVisibility::VISIBLE
        } else {
            bevy::camera::visibility::InheritedVisibility::HIDDEN
        })))
    }
}

impl From<InheritedVisibility> for bevy::camera::visibility::InheritedVisibility {
    fn from(v: InheritedVisibility) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl InheritedVisibility {
    /// Returns the computed visibility value.
    #[tracing::instrument(skip(self))]
    pub fn get(&self) -> bool {
        self.0.get()
    }

    /// Returns `true` if the entity is visible (computed from hierarchy).
    #[tracing::instrument(skip(self))]
    pub fn is_visible(&self) -> bool {
        self.0.get()
    }
}

mod emit_inherited_vis {
    use super::InheritedVisibility;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for InheritedVisibility {
        fn to_code_literal(&self) -> TokenStream {
            let b = self.0.get();
            quote::quote! {
                if #b {
                    ::bevy::camera::visibility::InheritedVisibility::VISIBLE
                } else {
                    ::bevy::camera::visibility::InheritedVisibility::HIDDEN
                }
            }
        }
    }
}

impl elicitation::ElicitComplete for InheritedVisibility {}

// ── ViewVisibility ────────────────────────────────────────────────────────────

elicit_newtype!(
    bevy::camera::visibility::ViewVisibility,
    as ViewVisibility
);
elicit_newtype_traits!(
    ViewVisibility,
    bevy::camera::visibility::ViewVisibility,
    [eq]
);

impl serde::Serialize for ViewVisibility {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_bool(self.0.get())
    }
}

impl<'de> serde::Deserialize<'de> for ViewVisibility {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let _b = bool::deserialize(d)?;
        Ok(ViewVisibility(Arc::new(
            bevy::camera::visibility::ViewVisibility::default(),
        )))
    }
}

impl From<ViewVisibility> for bevy::camera::visibility::ViewVisibility {
    fn from(v: ViewVisibility) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl ViewVisibility {
    /// Returns the computed render-side visibility value.
    #[tracing::instrument(skip(self))]
    pub fn get(&self) -> bool {
        self.0.get()
    }

    /// Returns `true` if the entity will be rendered this frame.
    #[tracing::instrument(skip(self))]
    pub fn is_visible(&self) -> bool {
        self.0.get()
    }
}

mod emit_view_vis {
    use super::ViewVisibility;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for ViewVisibility {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::bevy::camera::visibility::ViewVisibility::default() }
        }
    }
}

impl elicitation::ElicitComplete for ViewVisibility {}

// ── Camera ────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::camera::Camera, as Camera);
elicit_newtype_traits!(Camera, bevy::camera::Camera, []);

impl serde::Serialize for Camera {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = s.serialize_map(Some(2))?;
        map.serialize_entry("order", &self.0.order)?;
        map.serialize_entry("is_active", &self.0.is_active)?;
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for Camera {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = Camera;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "Camera")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Camera, A::Error> {
                let mut cam = bevy::camera::Camera::default();
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "order" => cam.order = map.next_value()?,
                        "is_active" => cam.is_active = map.next_value()?,
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Camera(Arc::new(cam)))
            }
        }
        d.deserialize_map(V)
    }
}

impl From<Camera> for bevy::camera::Camera {
    fn from(v: Camera) -> Self {
        Arc::try_unwrap(v.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

#[reflect_methods]
impl Camera {
    /// Returns `true` if this camera is active and will render.
    #[tracing::instrument(skip(self))]
    pub fn is_active(&self) -> bool {
        self.0.is_active
    }

    /// Render order; lower values render first.
    #[tracing::instrument(skip(self))]
    pub fn order(&self) -> isize {
        self.0.order
    }

    /// Constructs a default camera (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn new_camera(&self) -> Camera {
        Camera::from(bevy::camera::Camera::default())
    }
}

mod emit_camera {
    use super::Camera;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Camera {
        fn to_code_literal(&self) -> TokenStream {
            let order = self.0.order;
            let active = self.0.is_active;
            quote::quote! {{
                let mut cam = ::bevy::camera::Camera::default();
                cam.order = #order;
                cam.is_active = #active;
                cam
            }}
        }
    }
}

impl elicitation::ElicitComplete for Camera {}

// ── shadow_elicitation macro ──────────────────────────────────────────────────

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

// ── unit_elicitation macro ────────────────────────────────────────────────────

macro_rules! unit_elicitation {
    ($name:ident, $inner_path:path) => {
        impl elicitation::Prompt for $name {
            fn prompt() -> Option<&'static str> {
                None
            }
        }
        impl elicitation::Elicitation for $name {
            type Style = ();
            async fn elicit<C: elicitation::ElicitCommunicator>(
                _communicator: &C,
            ) -> elicitation::ElicitResult<Self> {
                Ok(Self)
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
                    .summary(
                        concat!(
                            "Marker component shadow for `",
                            stringify!($inner_path),
                            "`."
                        )
                        .to_string(),
                    )
                    .build()
                    .expect("valid TypeSpec")
            }
        }
        impl elicitation::ElicitComplete for $name {}
    };
}

// ── Camera2d ──────────────────────────────────────────────────────────────────

/// Shadow of [`bevy::camera::Camera2d`].
///
/// Marker component enabling the main 2D render graph for a [`Camera`].
#[derive(
    Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct Camera2d;

impl From<Camera2d> for bevy::camera::Camera2d {
    fn from(_: Camera2d) -> Self {
        bevy::camera::Camera2d
    }
}

mod emit_impls_camera2d {
    use super::Camera2d;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Camera2d {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::bevy::camera::Camera2d }
        }
    }
}

unit_elicitation!(Camera2d, bevy::camera::Camera2d);

// ── Camera3dDepthLoadOp ───────────────────────────────────────────────────────

/// Shadow of [`bevy::camera::Camera3dDepthLoadOp`].
///
/// Depth clear operation for the main 3D pass.
/// `Clear(0.0)` is the default (far plane due to Bevy's reverse-Z projection).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub enum Camera3dDepthLoadOp {
    /// Clear depth with the given value. Use `0.0` for the far plane.
    Clear(f32),
    /// Load depth from memory (no clear).
    Load,
}

impl Default for Camera3dDepthLoadOp {
    fn default() -> Self {
        Camera3dDepthLoadOp::Clear(0.0)
    }
}

impl From<Camera3dDepthLoadOp> for bevy::camera::Camera3dDepthLoadOp {
    fn from(v: Camera3dDepthLoadOp) -> Self {
        match v {
            Camera3dDepthLoadOp::Clear(x) => bevy::camera::Camera3dDepthLoadOp::Clear(x),
            Camera3dDepthLoadOp::Load => bevy::camera::Camera3dDepthLoadOp::Load,
        }
    }
}

mod emit_impls_camera3d_depth_load_op {
    use super::Camera3dDepthLoadOp;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Camera3dDepthLoadOp {
        fn to_code_literal(&self) -> TokenStream {
            match self {
                Camera3dDepthLoadOp::Clear(x) => {
                    quote::quote! { ::bevy::camera::Camera3dDepthLoadOp::Clear(#x) }
                }
                Camera3dDepthLoadOp::Load => {
                    quote::quote! { ::bevy::camera::Camera3dDepthLoadOp::Load }
                }
            }
        }
    }
}

shadow_elicitation!(Camera3dDepthLoadOp);

// ── ScreenSpaceTransmissionQuality ────────────────────────────────────────────

/// Shadow of [`bevy::camera::ScreenSpaceTransmissionQuality`].
///
/// Quality of the screen-space specular transmission blur effect applied behind
/// transmissive objects. Higher quality is more GPU-intensive.
#[derive(
    Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub enum ScreenSpaceTransmissionQuality {
    /// Best performance, lowest quality (suitable for mobile/lower-end GPUs).
    Low,
    /// Medium quality (default).
    #[default]
    Medium,
    /// High quality.
    High,
    /// Highest quality, most GPU-intensive.
    Ultra,
}

impl From<ScreenSpaceTransmissionQuality> for bevy::camera::ScreenSpaceTransmissionQuality {
    fn from(v: ScreenSpaceTransmissionQuality) -> Self {
        match v {
            ScreenSpaceTransmissionQuality::Low => {
                bevy::camera::ScreenSpaceTransmissionQuality::Low
            }
            ScreenSpaceTransmissionQuality::Medium => {
                bevy::camera::ScreenSpaceTransmissionQuality::Medium
            }
            ScreenSpaceTransmissionQuality::High => {
                bevy::camera::ScreenSpaceTransmissionQuality::High
            }
            ScreenSpaceTransmissionQuality::Ultra => {
                bevy::camera::ScreenSpaceTransmissionQuality::Ultra
            }
        }
    }
}

mod emit_impls_sstq {
    use super::ScreenSpaceTransmissionQuality;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for ScreenSpaceTransmissionQuality {
        fn to_code_literal(&self) -> TokenStream {
            match self {
                ScreenSpaceTransmissionQuality::Low => {
                    quote::quote! { ::bevy::camera::ScreenSpaceTransmissionQuality::Low }
                }
                ScreenSpaceTransmissionQuality::Medium => {
                    quote::quote! { ::bevy::camera::ScreenSpaceTransmissionQuality::Medium }
                }
                ScreenSpaceTransmissionQuality::High => {
                    quote::quote! { ::bevy::camera::ScreenSpaceTransmissionQuality::High }
                }
                ScreenSpaceTransmissionQuality::Ultra => {
                    quote::quote! { ::bevy::camera::ScreenSpaceTransmissionQuality::Ultra }
                }
            }
        }
    }
}

shadow_elicitation!(ScreenSpaceTransmissionQuality);

// ── Camera3d ──────────────────────────────────────────────────────────────────

/// Shadow of [`bevy::camera::Camera3d`].
///
/// Component enabling the main 3D render graph for a [`Camera`].
/// Serializes `depth_load_op`, `depth_texture_usages` (raw bitflags u32),
/// `screen_space_specular_transmission_steps`, and
/// `screen_space_specular_transmission_quality`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct Camera3d {
    /// Depth clear operation for the main 3D pass.
    pub depth_load_op: Camera3dDepthLoadOp,
    /// Raw [`TextureUsages`] bitflags for the depth texture.
    pub depth_texture_usages: u32,
    /// Number of transmissive-pass steps (layers of transparency).
    pub screen_space_specular_transmission_steps: usize,
    /// Quality of the screen-space specular transmission blur.
    pub screen_space_specular_transmission_quality: ScreenSpaceTransmissionQuality,
}

impl Default for Camera3d {
    fn default() -> Self {
        use bevy::render::render_resource::TextureUsages;
        Self {
            depth_load_op: Camera3dDepthLoadOp::default(),
            depth_texture_usages: TextureUsages::RENDER_ATTACHMENT.bits(),
            screen_space_specular_transmission_steps: 1,
            screen_space_specular_transmission_quality: ScreenSpaceTransmissionQuality::default(),
        }
    }
}

impl From<Camera3d> for bevy::camera::Camera3d {
    fn from(v: Camera3d) -> Self {
        bevy::camera::Camera3d {
            depth_load_op: v.depth_load_op.into(),
            depth_texture_usages: bevy::camera::Camera3dDepthTextureUsage(v.depth_texture_usages),
            screen_space_specular_transmission_steps: v.screen_space_specular_transmission_steps,
            screen_space_specular_transmission_quality: v
                .screen_space_specular_transmission_quality
                .into(),
        }
    }
}

mod emit_impls_camera3d {
    use super::Camera3d;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Camera3d {
        fn to_code_literal(&self) -> TokenStream {
            let depth_op = self.depth_load_op.to_code_literal();
            let tex_usages = self.depth_texture_usages;
            let steps = self.screen_space_specular_transmission_steps;
            let quality = self
                .screen_space_specular_transmission_quality
                .to_code_literal();
            quote::quote! {
                ::bevy::camera::Camera3d {
                    depth_load_op: #depth_op,
                    depth_texture_usages: ::bevy::camera::Camera3dDepthTextureUsage(#tex_usages),
                    screen_space_specular_transmission_steps: #steps,
                    screen_space_specular_transmission_quality: #quality,
                }
            }
        }
    }
}

shadow_elicitation!(Camera3d);

// ── ClearColor ────────────────────────────────────────────────────────────────

// Shadow of [`bevy::camera::ClearColor`].
//
// World resource storing the default clear color for cameras.
// Serialized/deserialized as the inner [`Color`].
elicit_newtype!(bevy::camera::ClearColor, as ClearColor);
elicit_newtype_traits!(ClearColor, bevy::camera::ClearColor, []);

impl From<ClearColor> for bevy::camera::ClearColor {
    fn from(v: ClearColor) -> Self {
        (*v.0).clone()
    }
}

impl serde::Serialize for ClearColor {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.0.0.serialize(s)
    }
}

impl<'de> serde::Deserialize<'de> for ClearColor {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::color::Color::deserialize(d)
            .map(|c| ClearColor(Arc::new(bevy::camera::ClearColor(c))))
    }
}

mod emit_impls_clear_color {
    use super::ClearColor;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for ClearColor {
        fn to_code_literal(&self) -> TokenStream {
            let color = crate::Color::from(self.0.0);
            let color_tokens = color.to_code_literal();
            quote::quote! { ::elicit_bevy::ClearColor::from(#color_tokens) }
        }
    }
}

impl elicitation::ElicitComplete for ClearColor {}

// ── ClearColorConfig ──────────────────────────────────────────────────────────

// Shadow of [`bevy::camera::ClearColorConfig`].
//
// Per-camera clear color configuration. Has derived `Serialize`/`Deserialize` upstream.
elicit_newtype!(bevy::camera::ClearColorConfig, as ClearColorConfig, forward_serde);
elicit_newtype_traits!(ClearColorConfig, bevy::camera::ClearColorConfig, []);

impl From<ClearColorConfig> for bevy::camera::ClearColorConfig {
    fn from(v: ClearColorConfig) -> Self {
        *v.0
    }
}

mod emit_impls_clear_color_config {
    use super::ClearColorConfig;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for ClearColorConfig {
        fn to_code_literal(&self) -> TokenStream {
            let json = serde_json::to_string(&*self.0).unwrap_or_default();
            quote::quote! {
                ::serde_json::from_str::<::bevy::camera::ClearColorConfig>(#json).unwrap()
            }
        }
    }
}

impl elicitation::ElicitComplete for ClearColorConfig {}

// ── MsaaWriteback ─────────────────────────────────────────────────────────────

// Shadow of [`bevy::camera::MsaaWriteback`].
//
// Controls when MSAA writeback occurs for a camera. Has derived
// `Serialize`/`Deserialize` upstream.
elicit_newtype!(bevy::camera::MsaaWriteback, as MsaaWriteback, forward_serde);
elicit_newtype_traits!(MsaaWriteback, bevy::camera::MsaaWriteback, [eq]);

impl From<MsaaWriteback> for bevy::camera::MsaaWriteback {
    fn from(v: MsaaWriteback) -> Self {
        *v.0
    }
}

mod emit_impls_msaa_writeback {
    use super::MsaaWriteback;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for MsaaWriteback {
        fn to_code_literal(&self) -> TokenStream {
            let json = serde_json::to_string(&*self.0).unwrap_or_default();
            quote::quote! {
                ::serde_json::from_str::<::bevy::camera::MsaaWriteback>(#json).unwrap()
            }
        }
    }
}

impl elicitation::ElicitComplete for MsaaWriteback {}

// ── PhysicalCameraParameters ──────────────────────────────────────────────────

/// Shadow of [`bevy::camera::PhysicalCameraParameters`].
///
/// Physical camera parameters used for computing EV100 and depth-of-field values.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct PhysicalCameraParameters {
    /// Aperture in f-stops (e.g. 1.4, 2.8, 5.6).
    pub aperture_f_stops: f32,
    /// Shutter speed in seconds (e.g. 1.0/125.0).
    pub shutter_speed_s: f32,
    /// Sensor sensitivity in ISO (e.g. 100.0).
    pub sensitivity_iso: f32,
    /// Image sensor height in meters (default 0.01866 for Super 35).
    pub sensor_height: f32,
}

impl Default for PhysicalCameraParameters {
    fn default() -> Self {
        let d = bevy::camera::PhysicalCameraParameters::default();
        Self {
            aperture_f_stops: d.aperture_f_stops,
            shutter_speed_s: d.shutter_speed_s,
            sensitivity_iso: d.sensitivity_iso,
            sensor_height: d.sensor_height,
        }
    }
}

impl From<PhysicalCameraParameters> for bevy::camera::PhysicalCameraParameters {
    fn from(v: PhysicalCameraParameters) -> Self {
        bevy::camera::PhysicalCameraParameters {
            aperture_f_stops: v.aperture_f_stops,
            shutter_speed_s: v.shutter_speed_s,
            sensitivity_iso: v.sensitivity_iso,
            sensor_height: v.sensor_height,
        }
    }
}

mod emit_impls_physical_camera_parameters {
    use super::PhysicalCameraParameters;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for PhysicalCameraParameters {
        fn to_code_literal(&self) -> TokenStream {
            let aperture = self.aperture_f_stops;
            let shutter = self.shutter_speed_s;
            let iso = self.sensitivity_iso;
            let sensor = self.sensor_height;
            quote::quote! {
                ::bevy::camera::PhysicalCameraParameters {
                    aperture_f_stops: #aperture,
                    shutter_speed_s: #shutter,
                    sensitivity_iso: #iso,
                    sensor_height: #sensor,
                }
            }
        }
    }
}

shadow_elicitation!(PhysicalCameraParameters);
