//! Runtime bridges for `Res<T>` and `ResMut<T>`.
//!
//! [`ResPlugin<T>`] adds an `Update` system that snapshots the current resource
//! value into `Arc<RwLock<Option<T>>>` each frame.
//!
//! [`ResMutPlugin<T>`] does the same but also accepts pending writes via an
//! `Arc<Mutex<Option<T>>>` queue, applying them to the live resource before
//! taking the snapshot.

use std::sync::{Arc, Mutex, RwLock};

/// Shared handle for reading the most-recent resource snapshot.
pub type ResSnapshot<T> = Arc<RwLock<Option<T>>>;

/// Queue for pending resource writes; the plugin applies them next `Update`.
pub type ResPending<T> = Arc<Mutex<Option<T>>>;

// ── ResPlugin ─────────────────────────────────────────────────────────────────

/// Runtime bridge for read-only access to a bevy `Resource`.
///
/// The shared [`ResSnapshot<T>`] is `None` until the first `Update` tick
/// after the resource is inserted into the world.
///
/// # Example
///
/// ```rust,ignore
/// let (plugin, state) = ResPlugin::<GameSettings>::new();
/// app.add_plugins(plugin);
/// // From an MCP tool:
/// let guard = state.read().unwrap();
/// if let Some(settings) = guard.as_ref() { println!("{:?}", settings); }
/// ```
pub struct ResPlugin<T: bevy::ecs::resource::Resource + Clone> {
    state: ResSnapshot<T>,
}

impl<T: bevy::ecs::resource::Resource + Clone> ResPlugin<T> {
    /// Create the plugin and return a handle to the shared resource snapshot.
    pub fn new() -> (Self, ResSnapshot<T>) {
        let state = Arc::new(RwLock::new(None));
        (
            Self {
                state: state.clone(),
            },
            state,
        )
    }
}

impl<T: bevy::ecs::resource::Resource + Clone> bevy::app::Plugin for ResPlugin<T> {
    #[tracing::instrument(skip_all, name = "ResPlugin::build")]
    fn build(&self, app: &mut bevy::app::App) {
        let state = self.state.clone();
        app.add_systems(
            bevy::app::Update,
            move |res: Option<bevy::ecs::system::Res<T>>| {
                tracing::trace!(present = res.is_some(), "Res frame sync");
                if let Ok(mut guard) = state.write() {
                    *guard = res.as_deref().cloned();
                }
            },
        );
    }
}

// ── ResMutPlugin ──────────────────────────────────────────────────────────────

/// Runtime bridge for bidirectional access to a bevy `Resource`.
///
/// Returns two handles:
/// - [`ResSnapshot<T>`] — read the last snapshotted value
/// - [`ResPending<T>`] — place a pending write; applied next `Update` tick
///
/// # Example
///
/// ```rust,ignore
/// let (plugin, state, pending) = ResMutPlugin::<GameSettings>::new();
/// app.add_plugins(plugin);
/// // Read:
/// let guard = state.read().unwrap();
/// // Write (applied next frame):
/// *pending.lock().unwrap() = Some(new_settings);
/// ```
pub struct ResMutPlugin<T>
where
    T: bevy::ecs::resource::Resource
        + bevy::ecs::component::Component<Mutability = bevy::ecs::component::Mutable>
        + Clone,
{
    state: ResSnapshot<T>,
    pending: ResPending<T>,
}

impl<T> ResMutPlugin<T>
where
    T: bevy::ecs::resource::Resource
        + bevy::ecs::component::Component<Mutability = bevy::ecs::component::Mutable>
        + Clone,
{
    /// Create the plugin and return handles to the shared state and write queue.
    pub fn new() -> (Self, ResSnapshot<T>, ResPending<T>) {
        let state = Arc::new(RwLock::new(None));
        let pending = Arc::new(Mutex::new(None));
        (
            Self {
                state: state.clone(),
                pending: pending.clone(),
            },
            state,
            pending,
        )
    }
}

impl<T> bevy::app::Plugin for ResMutPlugin<T>
where
    T: bevy::ecs::resource::Resource
        + bevy::ecs::component::Component<Mutability = bevy::ecs::component::Mutable>
        + Clone,
{
    #[tracing::instrument(skip_all, name = "ResMutPlugin::build")]
    fn build(&self, app: &mut bevy::app::App) {
        let state = self.state.clone();
        let pending = self.pending.clone();
        app.add_systems(
            bevy::app::Update,
            move |mut resource: Option<bevy::ecs::system::ResMut<T>>| {
                if let Ok(mut pw) = pending.lock()
                    && let Some(new_val) = pw.take()
                {
                    if let Some(ref mut res) = resource {
                        use std::ops::DerefMut;
                        *res.deref_mut() = new_val;
                        tracing::debug!("ResMut write applied");
                    } else {
                        tracing::warn!("ResMut write dropped: resource not present in world");
                    }
                }
                if let Ok(mut guard) = state.write() {
                    *guard = resource.as_ref().map(|r| (**r).clone());
                }
                tracing::trace!(present = resource.is_some(), "ResMut frame sync");
            },
        );
    }
}
