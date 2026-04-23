//! AccessKit display for [`StagedEdit`].
use elicitation::Elicit;

use accesskit::Role as AkRole;
use elicit_accesskit::{NodeId, NodeJson, Role};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::archive::{RowEditKind, StagedEdit};

use super::ArchiveDisplay;

/// Display strategies for a [`StagedEdit`].
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, JsonSchema, Elicit,
)]
pub enum StagedEditMode {
    /// A single row describing the pending mutation.
    #[default]
    Row,
}

impl ArchiveDisplay for StagedEdit {
    type Mode = StagedEditMode;

    fn root_role(_mode: &Self::Mode) -> Role {
        Role(AkRole::Row)
    }

    fn to_ak_nodes(&self, _mode: &Self::Mode, id_base: u64) -> (NodeId, Vec<(NodeId, NodeJson)>) {
        let root_id = NodeId::from(id_base);
        let kind_label = match &self.kind {
            RowEditKind::Update {
                column, new_value, ..
            } => {
                format!(
                    "UPDATE {}.{} SET {}={}",
                    self.schema, self.table, column, new_value
                )
            }
            RowEditKind::Insert { row } => {
                let cols: Vec<&str> = row.iter().map(|(c, _)| c.as_str()).collect();
                format!(
                    "INSERT INTO {}.{} ({})",
                    self.schema,
                    self.table,
                    cols.join(", ")
                )
            }
            RowEditKind::Delete { pk_values } => {
                let pk: Vec<String> = pk_values.iter().map(|(c, v)| format!("{c}={v}")).collect();
                format!(
                    "DELETE FROM {}.{} WHERE {}",
                    self.schema,
                    self.table,
                    pk.join(" AND ")
                )
            }
        };
        (
            root_id,
            vec![(
                root_id,
                NodeJson::new(Role(AkRole::Row)).with_label(kind_label),
            )],
        )
    }
}
