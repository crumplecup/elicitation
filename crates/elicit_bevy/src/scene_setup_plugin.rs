//! `BevySceneSetupPlugin` — bootstrap a lit 3D scene without importing bevy.
//!
//! Formalware and app crates that depend on `elicit_bevy` but not on `bevy`
//! directly can use this plugin to spawn a camera and directional light at
//! startup by supplying a [`SceneSetupDescriptor`] built entirely from
//! shadow types.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{Camera3d, DirectionalLight, GlobalAmbientLight, Transform};

/// Descriptor for a lit 3D scene bootstrap.
///
/// All fields are optional; omitted fields use bevy's defaults.  Construct
/// via `SceneSetupDescriptor::default()` and override only what you need.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct SceneSetupDescriptor {
    /// `Camera3d` component overrides applied to the spawned camera entity.
    #[serde(default)]
    pub camera: Option<Camera3d>,
    /// World-space transform for the camera entity.
    #[serde(default)]
    pub camera_transform: Option<Transform>,
    /// `DirectionalLight` component for the spawned light entity.
    #[serde(default)]
    pub directional_light: Option<DirectionalLight>,
    /// World-space transform for the directional light entity (controls direction).
    #[serde(default)]
    pub directional_light_transform: Option<Transform>,
    /// Global ambient light resource inserted into the world.
    #[serde(default)]
    pub global_ambient_light: Option<GlobalAmbientLight>,
}

/// Bevy plugin that spawns a camera and directional light on `Startup`.
///
/// # Example
///
/// ```rust,ignore
/// use elicit_bevy::{BevySceneSetupPlugin, SceneSetupDescriptor, Transform};
///
/// app.add_plugins(BevySceneSetupPlugin::new(SceneSetupDescriptor {
///     camera_transform: Some(Transform::from_xyz(0.0, 5.0, 10.0)),
///     ..Default::default()
/// }));
/// ```
#[derive(Debug, Clone)]
pub struct BevySceneSetupPlugin {
    descriptor: SceneSetupDescriptor,
}

impl BevySceneSetupPlugin {
    /// Create a new plugin from a scene descriptor.
    #[instrument]
    pub fn new(descriptor: SceneSetupDescriptor) -> Self {
        Self { descriptor }
    }
}

impl crate::Plugin for BevySceneSetupPlugin {
    #[instrument(skip(self, app))]
    fn build(&self, app: &mut crate::App) {
        let descriptor = self.descriptor.clone();
        app.0.add_systems(
            bevy::app::Startup,
            move |mut commands: bevy::ecs::system::Commands| {
                let camera = descriptor
                    .camera
                    .clone()
                    .map(bevy::camera::Camera3d::from)
                    .unwrap_or_default();
                let camera_transform = descriptor
                    .camera_transform
                    .clone()
                    .map(bevy::transform::components::Transform::from)
                    .unwrap_or_default();
                tracing::debug!(
                    tx = camera_transform.translation.x,
                    ty = camera_transform.translation.y,
                    tz = camera_transform.translation.z,
                    "Spawned camera"
                );
                commands.spawn((camera, camera_transform, bevy::ui::IsDefaultUiCamera));

                let light = descriptor
                    .directional_light
                    .clone()
                    .map(bevy::light::DirectionalLight::from)
                    .unwrap_or_default();
                let light_transform = descriptor
                    .directional_light_transform
                    .clone()
                    .map(bevy::transform::components::Transform::from)
                    .unwrap_or_default();
                commands.spawn((light, light_transform));

                if let Some(ambient) = descriptor.global_ambient_light.clone() {
                    commands.insert_resource(bevy::light::GlobalAmbientLight::from(ambient));
                }
            },
        );
    }
}
