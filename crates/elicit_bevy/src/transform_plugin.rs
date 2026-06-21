//! `BevyTransformPlugin` — static transform constructors for agents.
//!
//! The `#[reflect_methods]` approach only covers instance methods, so static
//! constructors like `Transform::from_xyz` are unreachable without a prior
//! instance.  This plugin exposes them as standalone `#[elicit_tool]` callables
//! that the agent can invoke without any existing `Transform` in hand.

use crate::Transform;
use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

/// Parameters for `bevy_transform__from_xyz`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TransformFromXyzParams {
    /// World-space X position.
    pub x: f32,
    /// World-space Y position.
    pub y: f32,
    /// World-space Z position.
    pub z: f32,
}

/// Parameters for `bevy_transform__from_scale`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TransformFromScaleParams {
    /// Uniform scale factor applied to all axes.
    pub scale: f32,
}

/// Parameters for `bevy_transform__from_xyz_looking_at`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TransformFromXyzLookingAtParams {
    /// World-space X position for the transform origin.
    pub x: f32,
    /// World-space Y position for the transform origin.
    pub y: f32,
    /// World-space Z position for the transform origin.
    pub z: f32,
    /// X component of the target point the transform should face.
    pub target_x: f32,
    /// Y component of the target point the transform should face.
    pub target_y: f32,
    /// Z component of the target point the transform should face.
    pub target_z: f32,
}

fn transform_result(t: Transform) -> Result<CallToolResult, ErrorData> {
    match serde_json::to_string(&t) {
        Ok(s) => Ok(CallToolResult::success(vec![Content::text(s)])),
        Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
            "serialize error: {e}"
        ))])),
    }
}

/// Plugin registering static `Transform` constructor tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "bevy_transform")]
pub struct BevyTransformPlugin;

impl BevyTransformPlugin {
    /// Creates a new plugin instance.
    #[instrument]
    pub fn new() -> Self {
        Self
    }
}

impl Default for BevyTransformPlugin {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a `Transform` at the given world-space position with identity rotation and scale 1.
#[elicit_tool(
    plugin = "bevy_transform",
    emit = None,
    name = "from_xyz",
    description = "Create a Transform positioned at (x, y, z) with identity rotation and uniform scale 1. \
                   Returns a serialized Transform that can be passed to SceneSetupDescriptor or other APIs."
)]
#[instrument]
async fn from_xyz(p: TransformFromXyzParams) -> Result<CallToolResult, ErrorData> {
    let t = Transform::from(bevy::transform::components::Transform::from_xyz(
        p.x, p.y, p.z,
    ));
    transform_result(t)
}

/// Create a `Transform` with uniform scale and identity translation and rotation.
#[elicit_tool(
    plugin = "bevy_transform",
    emit = None,
    name = "from_scale",
    description = "Create a Transform with a uniform scale factor and identity translation and rotation. \
                   Returns a serialized Transform."
)]
#[instrument]
async fn from_scale(p: TransformFromScaleParams) -> Result<CallToolResult, ErrorData> {
    let t = Transform::from(bevy::transform::components::Transform::from_scale(
        bevy::math::Vec3::splat(p.scale),
    ));
    transform_result(t)
}

/// Create a `Transform` positioned at `(x, y, z)` and oriented toward `(target_x, target_y, target_z)`.
///
/// Uses `Vec3::Y` as the up direction.  This is the one-shot equivalent of
/// `Transform::from_xyz(x, y, z).looking_at(target, Vec3::Y)`.
#[elicit_tool(
    plugin = "bevy_transform",
    emit = None,
    name = "from_xyz_looking_at",
    description = "Create a Transform at (x, y, z) oriented toward a target point, using Y-up. \
                   Equivalent to Transform::from_xyz(x,y,z).looking_at(target, Vec3::Y). \
                   Returns a serialized Transform."
)]
#[instrument]
async fn from_xyz_looking_at(
    p: TransformFromXyzLookingAtParams,
) -> Result<CallToolResult, ErrorData> {
    let bevy_t = bevy::transform::components::Transform::from_xyz(p.x, p.y, p.z).looking_at(
        bevy::math::Vec3::new(p.target_x, p.target_y, p.target_z),
        bevy::math::Vec3::Y,
    );
    transform_result(Transform::from(bevy_t))
}
