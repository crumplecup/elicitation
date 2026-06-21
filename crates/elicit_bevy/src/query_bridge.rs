//! Runtime bridge for `Query<(Entity, &C)>`.
//!
//! [`QueryPlugin<C>`] adds an `Update` system that snapshots all
//! `(entity_bits, component_clone)` pairs for component `C` into a shared
//! [`QuerySnapshot<C>`] each frame.

use std::sync::{Arc, RwLock};

/// Shared handle to the per-frame `(entity_bits, component)` snapshot.
pub type QuerySnapshot<C> = Arc<RwLock<Vec<(u64, C)>>>;

/// Runtime bridge that snapshots `Query<(Entity, &C)>` each `Update` frame.
///
/// # Example
///
/// ```rust,ignore
/// let (plugin, snapshot) = QueryPlugin::<Transform>::new();
/// app.add_plugins(plugin);
/// // From an MCP tool:
/// let guard = snapshot.read().unwrap();
/// for (bits, transform) in guard.iter() {
///     println!("entity {bits}: {transform:?}");
/// }
/// ```
pub struct QueryPlugin<C: bevy::ecs::component::Component + Clone> {
    snapshot: QuerySnapshot<C>,
}

impl<C: bevy::ecs::component::Component + Clone> QueryPlugin<C> {
    /// Create the plugin and return a handle to the shared snapshot.
    pub fn new() -> (Self, QuerySnapshot<C>) {
        let snapshot = Arc::new(RwLock::new(Vec::new()));
        (
            Self {
                snapshot: snapshot.clone(),
            },
            snapshot,
        )
    }
}

impl<C: bevy::ecs::component::Component + Clone> bevy::app::Plugin for QueryPlugin<C> {
    #[tracing::instrument(skip_all, name = "QueryPlugin::build")]
    fn build(&self, app: &mut bevy::app::App) {
        let snapshot = self.snapshot.clone();
        app.add_systems(
            bevy::app::Update,
            move |query: bevy::ecs::system::Query<(bevy::ecs::entity::Entity, &C)>| {
                let count = query.iter().count();
                tracing::trace!(count, "Query snapshot");
                if let Ok(mut guard) = snapshot.write() {
                    *guard = query
                        .iter()
                        .map(|(entity, comp)| (entity.to_bits(), comp.clone()))
                        .collect();
                }
            },
        );
    }
}
