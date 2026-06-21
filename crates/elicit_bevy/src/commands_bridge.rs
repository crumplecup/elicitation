//! Runtime bridge for bevy `Commands`.
//!
//! [`CommandsPlugin`] queues [`CommandRequest`]s from outside the ECS and
//! drains them via a bevy `Update` system each frame.  Spawned entity bits are
//! accumulated in a shared `Vec` that callers can drain with
//! [`CommandsHandle::take_spawned`].

use std::sync::{Arc, Mutex};

// ── CommandRequest ────────────────────────────────────────────────────────────

/// An entity operation to be executed inside the next bevy `Update` tick.
#[derive(Debug, Clone)]
pub enum CommandRequest {
    /// Spawn a new entity with a `Name` component and accumulate its bits.
    SpawnNamed {
        /// Value passed to `bevy::ecs::name::Name`.
        name: String,
    },
    /// Spawn a new entity with no components and accumulate its bits.
    SpawnEmpty,
    /// Despawn the entity identified by these raw bits.
    Despawn {
        /// `Entity::to_bits()` value.
        entity_bits: u64,
    },
}

// ── CommandsHandle ────────────────────────────────────────────────────────────

/// Caller-held handle for enqueuing commands and collecting results.
#[derive(Debug, Clone)]
pub struct CommandsHandle {
    queue: Arc<Mutex<Vec<CommandRequest>>>,
    spawned: Arc<Mutex<Vec<u64>>>,
}

impl CommandsHandle {
    /// Enqueue a spawn of a named entity.  The entity bits appear in
    /// [`take_spawned`](Self::take_spawned) after the next `Update` tick.
    pub fn spawn_named(&self, name: impl Into<String>) {
        if let Ok(mut q) = self.queue.lock() {
            q.push(CommandRequest::SpawnNamed { name: name.into() });
        }
    }

    /// Enqueue a spawn of an unnamed entity.
    pub fn spawn_empty(&self) {
        if let Ok(mut q) = self.queue.lock() {
            q.push(CommandRequest::SpawnEmpty);
        }
    }

    /// Enqueue a despawn by raw entity bits.
    pub fn despawn(&self, entity_bits: u64) {
        if let Ok(mut q) = self.queue.lock() {
            q.push(CommandRequest::Despawn { entity_bits });
        }
    }

    /// Drain and return the bits of all entities spawned since the last call.
    pub fn take_spawned(&self) -> Vec<u64> {
        self.spawned
            .lock()
            .map(|mut s| s.drain(..).collect())
            .unwrap_or_default()
    }
}

// ── CommandsPlugin ────────────────────────────────────────────────────────────

/// Runtime bridge that drains [`CommandRequest`]s via bevy `Commands` each frame.
///
/// # Example
///
/// ```rust,ignore
/// let (plugin, handle) = CommandsPlugin::new();
/// app.add_plugins(plugin);
/// // From an MCP tool:
/// handle.spawn_named("player");
/// // Next frame the entity exists; bits are in handle.take_spawned().
/// ```
pub struct CommandsPlugin {
    queue: Arc<Mutex<Vec<CommandRequest>>>,
    spawned: Arc<Mutex<Vec<u64>>>,
}

impl CommandsPlugin {
    /// Create the plugin and return a caller handle.
    pub fn new() -> (Self, CommandsHandle) {
        let queue = Arc::new(Mutex::new(Vec::new()));
        let spawned = Arc::new(Mutex::new(Vec::new()));
        let handle = CommandsHandle {
            queue: queue.clone(),
            spawned: spawned.clone(),
        };
        (Self { queue, spawned }, handle)
    }
}

impl bevy::app::Plugin for CommandsPlugin {
    #[tracing::instrument(skip_all, name = "CommandsPlugin::build")]
    fn build(&self, app: &mut bevy::app::App) {
        let queue = self.queue.clone();
        let spawned = self.spawned.clone();
        app.add_systems(
            bevy::app::Update,
            move |mut commands: bevy::ecs::system::Commands| {
                let requests: Vec<CommandRequest> = queue
                    .lock()
                    .map(|mut q| q.drain(..).collect())
                    .unwrap_or_default();

                if requests.is_empty() {
                    return;
                }

                tracing::debug!(count = requests.len(), "Draining CommandsPlugin queue");

                for request in requests {
                    match request {
                        CommandRequest::SpawnNamed { name } => {
                            let entity = commands
                                .spawn(bevy::ecs::name::Name::new(name.clone()))
                                .id();
                            tracing::debug!(
                                entity_bits = entity.to_bits(),
                                %name,
                                "Spawned named entity"
                            );
                            if let Ok(mut s) = spawned.lock() {
                                s.push(entity.to_bits());
                            }
                        }
                        CommandRequest::SpawnEmpty => {
                            let entity = commands.spawn_empty().id();
                            tracing::debug!(entity_bits = entity.to_bits(), "Spawned empty entity");
                            if let Ok(mut s) = spawned.lock() {
                                s.push(entity.to_bits());
                            }
                        }
                        CommandRequest::Despawn { entity_bits } => {
                            let entity = bevy::ecs::entity::Entity::from_bits(entity_bits);
                            commands.entity(entity).despawn();
                            tracing::debug!(entity_bits, "Despawned entity");
                        }
                    }
                }
            },
        );
    }
}
