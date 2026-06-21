//! Bevy plugin for rendering a validated GIS vector layer.
//!
//! [`BevyGisVectorLayerPlugin`] tessellates `geo_types::Geometry` payloads from a
//! [`elicit_gis::RenderVectorLayerDescriptor`] into Bevy meshes and spawns the
//! resulting entities into the ECS at startup.

use bevy::asset::RenderAssetUsages;
use bevy::render::mesh::{Indices, Mesh, PrimitiveTopology};
use elicit_gis::{
    RenderFeatureSymbolizerDescriptor, RenderVectorLayerDescriptor, RenderVectorLayerPayload,
    RenderableVectorLayerValid,
};
use elicitation::contracts::Established;
use geo::TriangulateDelaunay;
use geo_types::{Geometry, Polygon};
use tracing::{instrument, warn};

/// Bevy startup plugin that tessellates a validated GIS vector layer and spawns mesh entities.
///
/// Construct with [`BevyGisVectorLayerPlugin::new`], which requires an
/// [`Established<RenderableVectorLayerValid>`] proof token to ensure the layer has passed
/// contract validation before rendering.
pub struct BevyGisVectorLayerPlugin {
    descriptor: RenderVectorLayerDescriptor,
}

impl BevyGisVectorLayerPlugin {
    /// Create a new plugin from a validated vector layer descriptor.
    ///
    /// The `_proof` parameter is a ZST proof token — it is zero-cost but
    /// enforces at the type level that the caller has run the contract pipeline.
    pub fn new(
        descriptor: RenderVectorLayerDescriptor,
        _proof: Established<RenderableVectorLayerValid>,
    ) -> Self {
        Self { descriptor }
    }
}

impl crate::Plugin for BevyGisVectorLayerPlugin {
    #[instrument(skip(self, app), fields(layer_id = %self.descriptor.id, layer_name = %self.descriptor.name))]
    fn build(&self, app: &mut crate::App) {
        let descriptor = self.descriptor.clone();
        app.0.add_systems(
            bevy::app::Startup,
            move |mut commands: bevy::ecs::system::Commands,
                  mut meshes: bevy::ecs::system::ResMut<
                bevy::asset::Assets<bevy::render::mesh::Mesh>,
            >,
                  mut materials: bevy::ecs::system::ResMut<
                bevy::asset::Assets<bevy::pbr::StandardMaterial>,
            >| {
                spawn_layer_entities(&mut commands, &mut meshes, &mut materials, &descriptor);
            },
        );
    }
}

/// Tessellate all polygon geometry from the descriptor and spawn Bevy mesh entities.
#[instrument(skip_all, fields(layer_id = %descriptor.id, layer_name = %descriptor.name))]
fn spawn_layer_entities(
    commands: &mut bevy::ecs::system::Commands,
    meshes: &mut bevy::asset::Assets<bevy::render::mesh::Mesh>,
    materials: &mut bevy::asset::Assets<bevy::pbr::StandardMaterial>,
    descriptor: &RenderVectorLayerDescriptor,
) {
    let fill_color =
        symbolizer_fill_color(&descriptor.style.default_symbolizer, descriptor.opacity);
    let material_handle = materials.add(bevy::pbr::StandardMaterial {
        base_color: fill_color,
        unlit: true,
        alpha_mode: if descriptor.opacity < 1.0 {
            bevy::material::AlphaMode::Blend
        } else {
            bevy::material::AlphaMode::Opaque
        },
        ..Default::default()
    });

    let polygons = extract_polygons(&descriptor.payload);
    tracing::debug!(
        count = polygons.len(),
        "Tessellating polygons for vector layer"
    );

    for polygon in &polygons {
        let Some(mesh) = tessellate_polygon(polygon) else {
            warn!("Polygon tessellation failed; skipping");
            continue;
        };
        let mesh_handle = meshes.add(mesh);
        commands.spawn((
            bevy::mesh::Mesh3d(mesh_handle),
            bevy::pbr::MeshMaterial3d(material_handle.clone()),
            bevy::transform::components::Transform::default(),
        ));
        tracing::debug!("Spawned mesh entity for polygon");
    }
}

/// Extract all `Polygon<f64>` from the layer payload across all payload variants.
#[instrument(skip_all)]
fn extract_polygons(payload: &RenderVectorLayerPayload) -> Vec<Polygon<f64>> {
    let mut polygons = Vec::new();
    match payload {
        RenderVectorLayerPayload::Geometries(g) => {
            for geom in &g.0 {
                collect_polygons(geom, &mut polygons);
            }
        }
        RenderVectorLayerPayload::FeatureRecords(records) => {
            for record in &records.0 {
                collect_polygons(&record.geometry.0, &mut polygons);
            }
        }
        RenderVectorLayerPayload::FeatureCollection(_) => {
            warn!("FeatureCollection payload is not yet supported by BevyGisVectorLayerPlugin");
        }
    }
    polygons
}

/// Recursively collect `Polygon<f64>` from any `Geometry<f64>` variant.
fn collect_polygons(geometry: &Geometry<f64>, out: &mut Vec<Polygon<f64>>) {
    match geometry {
        Geometry::Polygon(p) => out.push(p.clone()),
        Geometry::MultiPolygon(mp) => {
            for p in &mp.0 {
                out.push(p.clone());
            }
        }
        Geometry::GeometryCollection(gc) => {
            for g in &gc.0 {
                collect_polygons(g, out);
            }
        }
        _ => {}
    }
}

/// Tessellate a single `Polygon<f64>` into a flat Bevy `Mesh` on the XZ plane.
///
/// Geographic coordinates are mapped as `(lon → X, lat → Z)` with `Y = 0`.
/// Returns `None` if the polygon produces no triangles.
#[instrument(skip_all)]
fn tessellate_polygon(polygon: &Polygon<f64>) -> Option<Mesh> {
    let triangles = match polygon.constrained_triangulation(Default::default()) {
        Ok(t) => t,
        Err(e) => {
            warn!(error = ?e, "Spade triangulation error");
            return None;
        }
    };

    if triangles.is_empty() {
        return None;
    }

    tracing::debug!(triangle_count = triangles.len(), "Tessellated polygon");

    let vertex_count = triangles.len() * 3;
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(vertex_count);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(vertex_count);
    let mut indices: Vec<u32> = Vec::with_capacity(vertex_count);

    for (i, tri) in triangles.iter().enumerate() {
        let base = (i * 3) as u32;
        for coord in [tri.v1(), tri.v2(), tri.v3()] {
            positions.push([coord.x as f32, 0.0, coord.y as f32]);
            normals.push([0.0, 1.0, 0.0]);
        }
        indices.extend_from_slice(&[base, base + 2, base + 1]);
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_indices(Indices::U32(indices));
    Some(mesh)
}

/// Extract the fill color from the default symbolizer, falling back to white.
///
/// Only `peniko::Brush::Solid` is mapped; gradient and image brushes fall back to white.
#[instrument(skip_all)]
fn symbolizer_fill_color(
    symbolizer: &RenderFeatureSymbolizerDescriptor,
    opacity: f32,
) -> bevy::color::Color {
    let Some(fill) = &symbolizer.fill else {
        return bevy::color::Color::WHITE;
    };
    match &fill.brush.0 {
        peniko::Brush::Solid(c) => {
            let [r, g, b, a] = c.components;
            bevy::color::Color::srgba(r, g, b, a * opacity)
        }
        _ => bevy::color::Color::WHITE,
    }
}
