//! `ArchiveDisplay` — the core display trait for archive descriptor types.

use elicit_accesskit::{NodeId, NodeJson, Role};

/// Trait implemented by every archive descriptor type to produce an AccessKit
/// node tree.
///
/// The `Mode` associated type captures the competing display strategies for
/// a given descriptor.  Types with only one obvious layout omit the enum and
/// use `Mode = ()`.
pub trait ArchiveDisplay {
    /// The set of supported display strategies for this type.
    type Mode: Default;

    /// AccessKit `Role` used for the root node of this type's subtree.
    fn root_role(mode: &Self::Mode) -> Role;

    /// Build the AccessKit node list for this descriptor in the given mode.
    ///
    /// `id_base` is the starting `u64` for allocating `NodeId`s; callers
    /// must pass a value that does not overlap with other nodes in the same
    /// tree update.  Returns `(root_id, nodes)`.
    fn to_ak_nodes(&self, mode: &Self::Mode, id_base: u64) -> (NodeId, Vec<(NodeId, NodeJson)>);
}
