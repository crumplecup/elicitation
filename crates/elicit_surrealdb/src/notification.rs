//! [`surrealdb_types::Notification`] and [`surrealdb_types::Action`] shadow types.

use elicitation::Elicit;
use elicitation_derive::reflect_methods;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ── Action ───────────────────────────────────────────────────────────────────

/// The action that caused a live-query notification.
///
/// Mirrors `surrealdb_types::Action`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Elicit)]
#[serde(rename_all = "UPPERCASE")]
pub enum Action {
    /// A record was created.
    Create,
    /// A record was updated.
    Update,
    /// A record was deleted.
    Delete,
    /// The live query was killed.
    Killed,
}

impl From<surrealdb_types::Action> for Action {
    fn from(a: surrealdb_types::Action) -> Self {
        match a {
            surrealdb_types::Action::Create => Action::Create,
            surrealdb_types::Action::Update => Action::Update,
            surrealdb_types::Action::Delete => Action::Delete,
            surrealdb_types::Action::Killed => Action::Killed,
        }
    }
}

impl From<Action> for surrealdb_types::Action {
    fn from(a: Action) -> Self {
        match a {
            Action::Create => surrealdb_types::Action::Create,
            Action::Update => surrealdb_types::Action::Update,
            Action::Delete => surrealdb_types::Action::Delete,
            Action::Killed => surrealdb_types::Action::Killed,
        }
    }
}

// ── Notification ─────────────────────────────────────────────────────────────

/// A live-query notification from SurrealDB.
///
/// Mirrors `surrealdb_types::Notification` with elicitation-enabled field types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct Notification {
    /// The UUID of the LIVE query that produced this notification.
    pub id: crate::Uuid,
    /// The session UUID, if present.
    pub session: Option<crate::Uuid>,
    /// The action that triggered this notification.
    pub action: Action,
    /// The record ID that was affected.
    pub record: crate::Value,
    /// The resulting document content.
    pub result: crate::Value,
}

impl From<surrealdb_types::Notification> for Notification {
    fn from(n: surrealdb_types::Notification) -> Self {
        Self {
            id: n.id.into(),
            session: n.session.map(Into::into),
            action: n.action.into(),
            record: n.record.into(),
            result: n.result.into(),
        }
    }
}

impl From<Notification> for surrealdb_types::Notification {
    fn from(n: Notification) -> Self {
        surrealdb_types::Notification::new(
            surrealdb_types::Uuid::from(n.id),
            n.session.map(surrealdb_types::Uuid::from),
            surrealdb_types::Action::from(n.action),
            surrealdb_types::Value::from(n.record),
            surrealdb_types::Value::from(n.result),
        )
    }
}

#[reflect_methods]
impl Notification {
    /// Returns `true` if this notification is a CREATE event.
    #[tracing::instrument(skip(self))]
    pub fn is_create(&self) -> bool {
        self.action == Action::Create
    }

    /// Returns `true` if this notification is an UPDATE event.
    #[tracing::instrument(skip(self))]
    pub fn is_update(&self) -> bool {
        self.action == Action::Update
    }

    /// Returns `true` if this notification is a DELETE event.
    #[tracing::instrument(skip(self))]
    pub fn is_delete(&self) -> bool {
        self.action == Action::Delete
    }

    /// Returns `true` if the live query was killed.
    #[tracing::instrument(skip(self))]
    pub fn is_killed(&self) -> bool {
        self.action == Action::Killed
    }
}
