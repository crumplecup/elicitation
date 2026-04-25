//! Static prompt tree for [`Elicitation`][crate::Elicitation] types.
//!
//! # Overview
//!
//! When an agent interacts with an elicitation workflow it receives a sequence
//! of prompts — one for each piece of information required to construct a
//! strongly-typed Rust value. In a small workflow that sequence is obvious at a
//! glance. In a deep, nested one it is not: the full prompt chain is buried
//! across dozens of `#[derive(Elicit)]` types, field attributes, enum variants,
//! and wrapper types, spread throughout the codebase. Without tooling, the only
//! way to see the complete picture is to actually run the elicitation and read
//! the agent's transcript.
//!
//! This module eliminates that problem. It provides a **static, compile-time
//! representation** of the entire prompt structure of any `Elicitation` type —
//! what the agent will be asked, in what order, with what options — without
//! executing a single `async` call or holding a communicator.
//!
//! The central abstraction is [`PromptTree`], a recursive enum that mirrors the
//! shape of the value being elicited:
//!
//! | [`PromptTree`] variant | Elicitation pattern | Rust equivalent |
//! |---|---|---|
//! | [`Leaf`][PromptTree::Leaf] | scalar input | `String`, `u32`, `PathBuf`, … |
//! | [`Select`][PromptTree::Select] | variant choice + optional data | `enum` |
//! | [`Survey`][PromptTree::Survey] | field-by-field input | `struct` |
//! | [`Affirm`][PromptTree::Affirm] | binary yes/no | `bool` |
//!
//! The [`ElicitPromptTree`] trait is the query interface: call
//! `T::prompt_tree()` to obtain the tree, or `T::assembled_prompts()` for a
//! flat, ordered list of every prompt in elicitation sequence.
//!
//! # How the Tree Is Composed
//!
//! `#[derive(Elicit)]` automatically generates an `ElicitPromptTree` impl that
//! recurses into every field and variant type:
//!
//! ```text
//! #[derive(Elicit)]
//! #[prompt("Configure the server:")]
//! struct ServerConfig {
//!     #[prompt("Host name or IP:")]
//!     host: String,
//!     #[prompt("Port number:")]
//!     port: u16,
//!     tls: bool,
//! }
//!
//! // The macro generates (approximately):
//! impl ElicitPromptTree for ServerConfig {
//!     fn prompt_tree() -> PromptTree {
//!         PromptTree::Survey {
//!             prompt: Some("Configure the server:".into()),
//!             type_name: "ServerConfig".into(),
//!             fields: vec![
//!                 ("host".into(), Box::new(
//!                     String::prompt_tree().with_prompt(Some("Host name or IP:".into()))
//!                 )),
//!                 ("port".into(), Box::new(
//!                     u16::prompt_tree().with_prompt(Some("Port number:".into()))
//!                 )),
//!                 ("tls".into(), Box::new(bool::prompt_tree())),
//!             ],
//!         }
//!     }
//! }
//! ```
//!
//! This is the **structural completeness guarantee**: because the macro sees the
//! actual field types at compile time, every field is included automatically.
//! There is no way to accidentally omit a field, and adding a new field to a
//! struct automatically extends its prompt tree. The same holds for enums —
//! adding a variant extends the `Select` node without touching any other code.
//!
//! # Field-Level Prompt Overrides
//!
//! A field-level `#[prompt("...")]` annotation overrides the inner type's
//! default prompt text for that field. The override is applied via
//! [`PromptTree::with_prompt`], which replaces the root prompt on whatever
//! subtree the field type returns:
//!
//! ```text
//! // String::prompt_tree() → Leaf { prompt: "Please enter text:", … }
//! // After with_prompt("Host name or IP:"):
//! //   → Leaf { prompt: "Host name or IP:", … }
//! ```
//!
//! Without a field-level annotation the inner type's default prompt is
//! preserved unchanged. This means primitive types like `String` and `u16`
//! ship with sensible defaults that are automatically used wherever they appear
//! without an override.
//!
//! # Reading the Full Prompt Sequence
//!
//! [`ElicitPromptTree::assembled_prompts`] performs a depth-first traversal of
//! the tree and returns a [`Vec<AssembledPrompt>`] — one entry per question the
//! agent will actually receive, in elicitation order. Each [`AssembledPrompt`]
//! carries:
//!
//! - `text` — the exact prompt string
//! - `kind` — which interaction pattern ([`PromptKind`])
//! - `path` — breadcrumb trail of field/variant names from the root
//!
//! ```
//! use elicitation::{ElicitPromptTree, PromptKind};
//!
//! let prompts = bool::assembled_prompts();
//! assert_eq!(prompts.len(), 1);
//! assert_eq!(prompts[0].kind, PromptKind::Affirm);
//! ```
//!
//! For a nested struct the path tells you *where* in the type hierarchy each
//! prompt originates:
//!
//! ```text
//! // For Deployment { env: Environment, config: ServerConfig { host, port, tls } }
//! // assembled_prompts() returns (approximately):
//! //
//! //   path=["env"]            kind=Select  text="Select deployment environment:"
//! //   path=["config", "host"] kind=Leaf    text="Host name or IP:"
//! //   path=["config", "port"] kind=Leaf    text="Port number:"
//! //   path=["config", "tls"]  kind=Affirm  text="Enable TLS?"
//! ```
//!
//! # AccessKit Bridge
//!
//! Behind the `prompt-tree-accesskit` feature, every [`PromptTree`] can be
//! converted to an [`accesskit::TreeUpdate`] via
//! [`PromptTree::to_accesskit_tree`]. The AccessKit tree is self-contained — no
//! live UI context, no async — and maps each node to the semantically closest
//! accessibility role:
//!
//! | [`PromptTree`] variant | AccessKit [`Role`][accesskit::Role] |
//! |---|---|
//! | `Leaf` | `Role::TextInput` |
//! | `Affirm` | `Role::CheckBox` |
//! | `Select` | `Role::ComboBox` (options as `ListBoxOption` children) |
//! | `Survey` | `Role::Form` (fields wrapped in `Group` children) |
//!
//! This makes the prompt structure consumable by screen readers, visualizers,
//! and any downstream tooling that speaks the AccessKit protocol.
//!
//! # Type Graph Integration
//!
//! The `graph` feature annotates Mermaid and DOT type graph nodes with the
//! prompts carried by this module. Type-level prompts appear in the node label;
//! field-level prompts appear on the edges. This means a single `TypeGraph`
//! render shows the full structural *and* conversational shape of your workflow
//! in one diagram — without running anything.
//!
//! # Feature Flags
//!
//! | Feature | What it enables |
//! |---|---|
//! | `prompt-tree` | This entire module; required for all uses |
//! | `prompt-tree-accesskit` | [`PromptTree::to_accesskit_tree`]; implies `prompt-tree` |
//!
//! # Quick Start
//!
//! ```
//! use elicitation::{ElicitPromptTree, PromptTree, PromptKind};
//!
//! // Any type that derives Elicit (or is a primitive) works:
//! let tree = bool::prompt_tree();
//! assert!(matches!(tree, PromptTree::Affirm { .. }));
//!
//! // Flat traversal — see every question in order:
//! let prompts = bool::assembled_prompts();
//! assert_eq!(prompts.len(), 1);
//! assert_eq!(prompts[0].kind, PromptKind::Affirm);
//! assert!(!prompts[0].text.is_empty());
//! ```

use crate::Prompt;

// ============================================================================
// Core types
// ============================================================================

/// The prompt structure of a type, as a static owned tree.
///
/// Built from `String` values and `Vec` allocations — no communicator, no
/// async, no runtime elicitation state required.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PromptTree {
    /// A scalar value with a single prompt (bool uses [`PromptTree::Affirm`]).
    Leaf {
        /// The prompt text sent to the agent.
        prompt: String,
        /// The Rust type name, for display.
        type_name: String,
    },

    /// An enum — the agent picks one variant from a finite set.
    Select {
        /// The base prompt text.
        prompt: String,
        /// The Rust type name.
        type_name: String,
        /// Variant labels in declaration order.
        options: Vec<String>,
        /// For variants that carry fields, the sub-tree elicited after
        /// selection. `None` for unit variants.
        branches: Vec<Option<Box<PromptTree>>>,
    },

    /// A struct — the agent answers a sequence of field prompts.
    Survey {
        /// The top-level prompt for this struct, if any.
        prompt: Option<String>,
        /// The Rust type name.
        type_name: String,
        /// Ordered list of `(field_name, sub-tree)` pairs.
        fields: Vec<(String, Box<PromptTree>)>,
    },

    /// A binary yes/no step.
    Affirm {
        /// The prompt text.
        prompt: String,
        /// The Rust type name.
        type_name: String,
    },
}

impl PromptTree {
    /// The prompt text for the root node, if any.
    pub fn prompt(&self) -> Option<&str> {
        match self {
            Self::Leaf { prompt, .. } => Some(prompt),
            Self::Select { prompt, .. } => Some(prompt),
            Self::Survey { prompt, .. } => prompt.as_deref(),
            Self::Affirm { prompt, .. } => Some(prompt),
        }
    }

    /// The type name for the root node.
    pub fn type_name(&self) -> &str {
        match self {
            Self::Leaf { type_name, .. } => type_name,
            Self::Select { type_name, .. } => type_name,
            Self::Survey { type_name, .. } => type_name,
            Self::Affirm { type_name, .. } => type_name,
        }
    }

    /// Override the prompt at the root node of this tree.
    ///
    /// If `prompt` is `None`, the existing prompt is kept unchanged.  Used
    /// when a field-level `#[prompt("...")]` annotation should override the
    /// default prompt coming from the field type's own `ElicitPromptTree`
    /// implementation.
    #[must_use]
    pub fn with_prompt(self, prompt: Option<String>) -> Self {
        match prompt {
            None => self,
            Some(p) => match self {
                Self::Leaf { type_name, .. } => Self::Leaf {
                    type_name,
                    prompt: p,
                },
                Self::Affirm { type_name, .. } => Self::Affirm {
                    type_name,
                    prompt: p,
                },
                Self::Select {
                    type_name,
                    options,
                    branches,
                    ..
                } => Self::Select {
                    type_name,
                    options,
                    branches,
                    prompt: p,
                },
                Self::Survey {
                    type_name, fields, ..
                } => Self::Survey {
                    type_name,
                    fields,
                    prompt: Some(p),
                },
            },
        }
    }

    /// The depth of this tree (1 for a leaf).
    pub fn depth(&self) -> usize {
        match self {
            Self::Leaf { .. } | Self::Affirm { .. } => 1,
            Self::Select { branches, .. } => {
                1 + branches
                    .iter()
                    .filter_map(|b| b.as_deref())
                    .map(|t| t.depth())
                    .max()
                    .unwrap_or(0)
            }
            Self::Survey { fields, .. } => {
                1 + fields.iter().map(|(_, t)| t.depth()).max().unwrap_or(0)
            }
        }
    }

    /// Convert to an AccessKit `TreeUpdate` for use in a visualizer or
    /// assistive technology bridge.
    ///
    /// The root node receives `NodeId(0)`.  All descendant nodes receive
    /// auto-incremented IDs.  The returned `TreeUpdate` is self-contained
    /// and has no dependency on a live UI context.
    ///
    /// Role mapping:
    /// - `Leaf`   → `Role::TextField`
    /// - `Select` → `Role::ComboBox` (options as `Role::ListBoxOption` children)
    /// - `Survey` → `Role::Form`     (fields as named children)
    /// - `Affirm` → `Role::CheckBox`
    ///
    /// Prompt text is placed in `node.label`; the type name in
    /// `node.description`.
    #[cfg(feature = "prompt-tree-accesskit")]
    pub fn to_accesskit_tree(&self) -> accesskit::TreeUpdate {
        let mut nodes: Vec<(accesskit::NodeId, accesskit::Node)> = Vec::new();
        let mut counter: u64 = 0;
        let root_id = build_accesskit_nodes(self, &mut counter, &mut nodes);
        let tree = accesskit::Tree::new(root_id);
        accesskit::TreeUpdate {
            nodes,
            tree: Some(tree),
            tree_id: accesskit::TreeId::ROOT,
            focus: root_id,
        }
    }
}

/// Recursively build AccessKit nodes, returning the `NodeId` of the root.
#[cfg(feature = "prompt-tree-accesskit")]
fn build_accesskit_nodes(
    tree: &PromptTree,
    counter: &mut u64,
    nodes: &mut Vec<(accesskit::NodeId, accesskit::Node)>,
) -> accesskit::NodeId {
    let id = accesskit::NodeId(*counter);
    *counter += 1;

    match tree {
        PromptTree::Leaf { prompt, type_name } => {
            let mut node = accesskit::Node::new(accesskit::Role::TextInput);
            node.set_label(prompt.as_str());
            node.set_description(type_name.as_str());
            nodes.push((id, node));
        }
        PromptTree::Affirm { prompt, type_name } => {
            let mut node = accesskit::Node::new(accesskit::Role::CheckBox);
            node.set_label(prompt.as_str());
            node.set_description(type_name.as_str());
            nodes.push((id, node));
        }
        PromptTree::Select {
            prompt,
            type_name,
            options,
            branches,
        } => {
            // Build option + branch children first so we can reference their IDs.
            let mut child_ids: Vec<accesskit::NodeId> = Vec::new();

            for (option_label, branch) in options.iter().zip(branches.iter()) {
                // Each option gets a ListBoxOption node.
                let opt_id = accesskit::NodeId(*counter);
                *counter += 1;
                let mut opt_node = accesskit::Node::new(accesskit::Role::ListBoxOption);
                opt_node.set_label(option_label.as_str());
                // If the variant has a branch, nest it as a child.
                if let Some(subtree) = branch {
                    let branch_id = build_accesskit_nodes(subtree, counter, nodes);
                    opt_node.push_child(branch_id);
                }
                nodes.push((opt_id, opt_node));
                child_ids.push(opt_id);
            }

            let mut node = accesskit::Node::new(accesskit::Role::ComboBox);
            node.set_label(prompt.as_str());
            node.set_description(type_name.as_str());
            for child_id in child_ids {
                node.push_child(child_id);
            }
            nodes.push((id, node));
        }
        PromptTree::Survey {
            prompt,
            type_name,
            fields,
        } => {
            let mut child_ids: Vec<accesskit::NodeId> = Vec::new();
            for (field_name, subtree) in fields {
                let field_id = build_accesskit_nodes(subtree, counter, nodes);
                // Wrap each field in a Group node labelled with the field name.
                let wrapper_id = accesskit::NodeId(*counter);
                *counter += 1;
                let mut wrapper = accesskit::Node::new(accesskit::Role::Group);
                wrapper.set_label(field_name.as_str());
                wrapper.push_child(field_id);
                nodes.push((wrapper_id, wrapper));
                child_ids.push(wrapper_id);
            }
            let mut node = accesskit::Node::new(accesskit::Role::Form);
            if let Some(p) = prompt {
                node.set_label(p.as_str());
            }
            node.set_description(type_name.as_str());
            for child_id in child_ids {
                node.push_child(child_id);
            }
            nodes.push((id, node));
        }
    }

    id
}

// ============================================================================
// Trait
// ============================================================================

/// Types that can describe their prompt structure statically.
///
/// Implemented automatically by [`crate::Elicit`] derive for user types.
/// Also implemented for all primitives in this crate.
pub trait ElicitPromptTree {
    /// Return the static prompt tree for this type.
    ///
    /// Pure function: no side effects, same result every call. Safe to call
    /// at startup, in tests, or in a visualizer without running an
    /// elicitation.
    fn prompt_tree() -> PromptTree;

    /// Return the complete assembled prompts in elicitation order.
    ///
    /// Each element is the exact string that would be passed to
    /// `communicator.send_prompt()` during a real elicitation. For `Survey`
    /// types this yields one entry per field; for `Select` types it yields
    /// the base prompt with the numbered options list appended.
    fn assembled_prompts() -> Vec<AssembledPrompt> {
        collect_assembled_prompts(&Self::prompt_tree(), &[])
    }
}

// ============================================================================
// AssembledPrompt
// ============================================================================

/// A single assembled prompt, exactly as the agent would receive it.
#[derive(Debug, Clone)]
pub struct AssembledPrompt {
    /// The full prompt string, including options list for `Select` nodes.
    pub text: String,
    /// The path through the type tree to this step (e.g. `["address", "port"]`).
    pub path: Vec<String>,
    /// Which interaction paradigm this step uses.
    pub kind: PromptKind,
}

/// The interaction paradigm for a single elicitation step.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptKind {
    /// Scalar value entry.
    Leaf,
    /// Pick one from a finite set.
    Select,
    /// Sequential multi-field elicitation (top-level node).
    Survey,
    /// Binary yes/no.
    Affirm,
}

/// Walk `tree` depth-first and collect one [`AssembledPrompt`] per
/// `send_prompt` call the real `elicit()` would make.
pub fn collect_assembled_prompts(tree: &PromptTree, path: &[String]) -> Vec<AssembledPrompt> {
    match tree {
        PromptTree::Leaf { prompt, .. } => vec![AssembledPrompt {
            text: prompt.clone(),
            path: path.to_vec(),
            kind: PromptKind::Leaf,
        }],

        PromptTree::Affirm { prompt, .. } => vec![AssembledPrompt {
            text: prompt.clone(),
            path: path.to_vec(),
            kind: PromptKind::Affirm,
        }],

        PromptTree::Select {
            prompt,
            options,
            branches,
            ..
        } => {
            // Assemble exactly as generate_elicit_impl does in enum_impl.rs
            let options_text = options
                .iter()
                .enumerate()
                .map(|(i, label)| format!("{}. {}", i + 1, label))
                .collect::<Vec<_>>()
                .join("\n");
            let full_prompt = format!(
                "{}\n\nOptions:\n{}\n\nRespond with the number (1-{}) or exact label:",
                prompt,
                options_text,
                options.len()
            );

            let mut out = vec![AssembledPrompt {
                text: full_prompt,
                path: path.to_vec(),
                kind: PromptKind::Select,
            }];

            // Include the first non-unit branch as the representative path
            // (branches are enumerated per-variant; the first with fields wins)
            for (label, branch) in options.iter().zip(branches.iter()) {
                if let Some(sub) = branch {
                    let mut child_path = path.to_vec();
                    child_path.push(label.clone());
                    out.extend(collect_assembled_prompts(sub, &child_path));
                    break;
                }
            }

            out
        }

        PromptTree::Survey { fields, .. } => fields
            .iter()
            .flat_map(|(field_name, sub)| {
                let mut child_path = path.to_vec();
                child_path.push(field_name.clone());
                collect_assembled_prompts(sub, &child_path)
            })
            .collect(),
    }
}

// ============================================================================
// Blanket impls — primitives
// ============================================================================

macro_rules! leaf_impl {
    ($ty:ty, $type_name:literal) => {
        impl ElicitPromptTree for $ty {
            fn prompt_tree() -> PromptTree {
                PromptTree::Leaf {
                    prompt: <$ty as Prompt>::prompt().unwrap_or($type_name).to_string(),
                    type_name: $type_name.to_string(),
                }
            }
        }
    };
}

macro_rules! affirm_impl {
    ($ty:ty, $type_name:literal) => {
        impl ElicitPromptTree for $ty {
            fn prompt_tree() -> PromptTree {
                PromptTree::Affirm {
                    prompt: <$ty as Prompt>::prompt().unwrap_or($type_name).to_string(),
                    type_name: $type_name.to_string(),
                }
            }
        }
    };
}

affirm_impl!(bool, "bool");

leaf_impl!(i8, "i8");
leaf_impl!(i16, "i16");
leaf_impl!(i32, "i32");
leaf_impl!(i64, "i64");
leaf_impl!(i128, "i128");
leaf_impl!(isize, "isize");
leaf_impl!(u8, "u8");
leaf_impl!(u16, "u16");
leaf_impl!(u32, "u32");
leaf_impl!(u64, "u64");
leaf_impl!(u128, "u128");
leaf_impl!(usize, "usize");
leaf_impl!(f32, "f32");
leaf_impl!(f64, "f64");
leaf_impl!(char, "char");
leaf_impl!(String, "String");
leaf_impl!(std::path::PathBuf, "PathBuf");
leaf_impl!(std::time::Duration, "Duration");
leaf_impl!(std::time::SystemTime, "SystemTime");
leaf_impl!((), "()");
leaf_impl!(std::net::IpAddr, "IpAddr");
leaf_impl!(std::net::Ipv4Addr, "Ipv4Addr");
leaf_impl!(std::net::Ipv6Addr, "Ipv6Addr");
leaf_impl!(std::net::SocketAddr, "SocketAddr");
leaf_impl!(std::net::SocketAddrV4, "SocketAddrV4");
leaf_impl!(std::net::SocketAddrV6, "SocketAddrV6");

// Generic containers — delegate to inner type
impl<T: ElicitPromptTree> ElicitPromptTree for Vec<T> {
    fn prompt_tree() -> PromptTree {
        T::prompt_tree()
    }
}

impl<T: ElicitPromptTree> ElicitPromptTree for Option<T> {
    fn prompt_tree() -> PromptTree {
        T::prompt_tree()
    }
}

impl<T: ElicitPromptTree, E> ElicitPromptTree for Result<T, E> {
    fn prompt_tree() -> PromptTree {
        T::prompt_tree()
    }
}

impl<T: ElicitPromptTree> ElicitPromptTree for Box<T> {
    fn prompt_tree() -> PromptTree {
        T::prompt_tree()
    }
}

impl<T: ElicitPromptTree> ElicitPromptTree for std::sync::Arc<T> {
    fn prompt_tree() -> PromptTree {
        T::prompt_tree()
    }
}

impl<T: ElicitPromptTree> ElicitPromptTree for std::rc::Rc<T> {
    fn prompt_tree() -> PromptTree {
        T::prompt_tree()
    }
}

impl<T: ElicitPromptTree, const N: usize> ElicitPromptTree for [T; N] {
    fn prompt_tree() -> PromptTree {
        T::prompt_tree()
    }
}

// ============================================================================
// Standard library collections — delegate to element / value type
// ============================================================================

impl<K, V: ElicitPromptTree> ElicitPromptTree for std::collections::HashMap<K, V> {
    fn prompt_tree() -> PromptTree {
        V::prompt_tree()
    }
}

impl<K, V: ElicitPromptTree> ElicitPromptTree for std::collections::BTreeMap<K, V> {
    fn prompt_tree() -> PromptTree {
        V::prompt_tree()
    }
}

impl<T: ElicitPromptTree> ElicitPromptTree for std::collections::HashSet<T> {
    fn prompt_tree() -> PromptTree {
        T::prompt_tree()
    }
}

impl<T: ElicitPromptTree> ElicitPromptTree for std::collections::BTreeSet<T> {
    fn prompt_tree() -> PromptTree {
        T::prompt_tree()
    }
}

impl<T: ElicitPromptTree> ElicitPromptTree for std::collections::VecDeque<T> {
    fn prompt_tree() -> PromptTree {
        T::prompt_tree()
    }
}

impl<T: ElicitPromptTree> ElicitPromptTree for std::collections::LinkedList<T> {
    fn prompt_tree() -> PromptTree {
        T::prompt_tree()
    }
}

// ============================================================================
// Verification types — require the `verification` feature
//
// All the contract/validation types in `crate::verification::types` have
// hand-written `Elicitation` impls and therefore do NOT get `ElicitPromptTree`
// from `#[derive(Elicit)]`.  We centrally provide `Leaf` (or delegate) impls
// here so that any struct with a verification-type field can be derived.
// ============================================================================

mod verification_impls {
    use super::*;
    use crate::verification::types::{
        // Collections (generic)
        ArcNonNull,
        ArcSatisfies,
        ArrayAllSatisfy,
        BTreeMapNonEmpty,
        BTreeSetNonEmpty,
        // Bool
        BoolDefault,
        BoolFalse,
        BoolTrue,
        BoxNonNull,
        BoxSatisfies,
        // Char
        CharAlphabetic,
        CharAlphanumeric,
        CharNumeric,
        // Duration
        DurationPositive,
        // Float
        F32Default,
        F32Finite,
        F32NonNegative,
        F32Positive,
        F64Default,
        F64Finite,
        F64NonNegative,
        F64Positive,
        HashMapNonEmpty,
        HashSetNonEmpty,
        // Integers — i8 family
        I8Default,
        I8NonNegative,
        I8NonZero,
        I8Positive,
        I8Range,
        I8RangeStyle,
        // i16
        I16Default,
        I16NonNegative,
        I16NonZero,
        I16Positive,
        I16Range,
        I16RangeStyle,
        // i32
        I32Default,
        I32NonNegative,
        I32NonZero,
        I32Positive,
        I32Range,
        // i64 and wider
        I64Default,
        I64NonNegative,
        I64NonZero,
        I64Positive,
        I64Range,
        I128Default,
        I128NonNegative,
        I128NonZero,
        I128Positive,
        I128Range,
        // Network
        IpPrivate,
        IpPublic,
        IpV4,
        IpV6,
        Ipv4Loopback,
        Ipv6Loopback,
        IsizeDefault,
        IsizeNonNegative,
        IsizeNonZero,
        IsizePositive,
        IsizeRange,
        LinkedListNonEmpty,
        OptionSome,
        // Path
        PathBufExists,
        PathBufIsDir,
        PathBufIsFile,
        PathBufReadable,
        RcNonNull,
        RcSatisfies,
        ResultOk,
        // String
        StringDefault,
        StringNonEmpty,
        // Tuples (generic)
        Tuple2,
        Tuple3,
        Tuple4,
        // u8
        U8Default,
        U8NonZero,
        U8Positive,
        U8Range,
        U8RangeStyle,
        // u16
        U16Default,
        U16NonZero,
        U16Positive,
        U16Range,
        U16RangeStyle,
        // u32
        U32Default,
        U32NonZero,
        U32Positive,
        U32Range,
        // u64 and wider
        U64Default,
        U64NonZero,
        U64Positive,
        U64Range,
        U128Default,
        U128NonZero,
        U128Positive,
        U128Range,
        UsizeDefault,
        UsizeNonZero,
        UsizePositive,
        UsizeRange,
        VecAllSatisfy,
        VecDequeNonEmpty,
        VecNonEmpty,
    };

    // ---- Integers: concrete leaf types ----

    leaf_impl!(I8Positive, "I8Positive");
    leaf_impl!(I8NonNegative, "I8NonNegative");
    leaf_impl!(I8NonZero, "I8NonZero");
    leaf_impl!(I8Default, "I8Default");
    leaf_impl!(I8RangeStyle, "I8RangeStyle");

    leaf_impl!(I16Positive, "I16Positive");
    leaf_impl!(I16NonNegative, "I16NonNegative");
    leaf_impl!(I16NonZero, "I16NonZero");
    leaf_impl!(I16Default, "I16Default");
    leaf_impl!(I16RangeStyle, "I16RangeStyle");

    leaf_impl!(I32Positive, "I32Positive");
    leaf_impl!(I32NonNegative, "I32NonNegative");
    leaf_impl!(I32NonZero, "I32NonZero");
    leaf_impl!(I32Default, "I32Default");

    leaf_impl!(I64Positive, "I64Positive");
    leaf_impl!(I64NonNegative, "I64NonNegative");
    leaf_impl!(I64NonZero, "I64NonZero");
    leaf_impl!(I64Default, "I64Default");

    leaf_impl!(I128Positive, "I128Positive");
    leaf_impl!(I128NonNegative, "I128NonNegative");
    leaf_impl!(I128NonZero, "I128NonZero");
    leaf_impl!(I128Default, "I128Default");

    leaf_impl!(IsizePositive, "IsizePositive");
    leaf_impl!(IsizeNonNegative, "IsizeNonNegative");
    leaf_impl!(IsizeNonZero, "IsizeNonZero");
    leaf_impl!(IsizeDefault, "IsizeDefault");

    leaf_impl!(U8Positive, "U8Positive");
    leaf_impl!(U8NonZero, "U8NonZero");
    leaf_impl!(U8Default, "U8Default");
    leaf_impl!(U8RangeStyle, "U8RangeStyle");

    leaf_impl!(U16Positive, "U16Positive");
    leaf_impl!(U16NonZero, "U16NonZero");
    leaf_impl!(U16Default, "U16Default");
    leaf_impl!(U16RangeStyle, "U16RangeStyle");

    leaf_impl!(U32Positive, "U32Positive");
    leaf_impl!(U32NonZero, "U32NonZero");
    leaf_impl!(U32Default, "U32Default");

    leaf_impl!(U64Positive, "U64Positive");
    leaf_impl!(U64NonZero, "U64NonZero");
    leaf_impl!(U64Default, "U64Default");

    leaf_impl!(U128Positive, "U128Positive");
    leaf_impl!(U128NonZero, "U128NonZero");
    leaf_impl!(U128Default, "U128Default");

    leaf_impl!(UsizePositive, "UsizePositive");
    leaf_impl!(UsizeNonZero, "UsizeNonZero");
    leaf_impl!(UsizeDefault, "UsizeDefault");

    // ---- Integers: Range types (const-generic) ----

    impl<const MIN: i8, const MAX: i8> ElicitPromptTree for I8Range<MIN, MAX> {
        fn prompt_tree() -> PromptTree {
            PromptTree::Leaf {
                prompt: <Self as Prompt>::prompt().unwrap_or("I8Range").to_string(),
                type_name: format!("I8Range<{MIN},{MAX}>"),
            }
        }
    }

    impl<const MIN: i16, const MAX: i16> ElicitPromptTree for I16Range<MIN, MAX> {
        fn prompt_tree() -> PromptTree {
            PromptTree::Leaf {
                prompt: <Self as Prompt>::prompt().unwrap_or("I16Range").to_string(),
                type_name: format!("I16Range<{MIN},{MAX}>"),
            }
        }
    }

    impl<const MIN: i32, const MAX: i32> ElicitPromptTree for I32Range<MIN, MAX> {
        fn prompt_tree() -> PromptTree {
            PromptTree::Leaf {
                prompt: <Self as Prompt>::prompt().unwrap_or("I32Range").to_string(),
                type_name: format!("I32Range<{MIN},{MAX}>"),
            }
        }
    }

    impl<const MIN: i64, const MAX: i64> ElicitPromptTree for I64Range<MIN, MAX> {
        fn prompt_tree() -> PromptTree {
            PromptTree::Leaf {
                prompt: <Self as Prompt>::prompt().unwrap_or("I64Range").to_string(),
                type_name: format!("I64Range<{MIN},{MAX}>"),
            }
        }
    }

    impl<const MIN: i128, const MAX: i128> ElicitPromptTree for I128Range<MIN, MAX> {
        fn prompt_tree() -> PromptTree {
            PromptTree::Leaf {
                prompt: <Self as Prompt>::prompt()
                    .unwrap_or("I128Range")
                    .to_string(),
                type_name: format!("I128Range<{MIN},{MAX}>"),
            }
        }
    }

    impl<const MIN: isize, const MAX: isize> ElicitPromptTree for IsizeRange<MIN, MAX> {
        fn prompt_tree() -> PromptTree {
            PromptTree::Leaf {
                prompt: <Self as Prompt>::prompt()
                    .unwrap_or("IsizeRange")
                    .to_string(),
                type_name: format!("IsizeRange<{MIN},{MAX}>"),
            }
        }
    }

    impl<const MIN: u8, const MAX: u8> ElicitPromptTree for U8Range<MIN, MAX> {
        fn prompt_tree() -> PromptTree {
            PromptTree::Leaf {
                prompt: <Self as Prompt>::prompt().unwrap_or("U8Range").to_string(),
                type_name: format!("U8Range<{MIN},{MAX}>"),
            }
        }
    }

    impl<const MIN: u16, const MAX: u16> ElicitPromptTree for U16Range<MIN, MAX> {
        fn prompt_tree() -> PromptTree {
            PromptTree::Leaf {
                prompt: <Self as Prompt>::prompt().unwrap_or("U16Range").to_string(),
                type_name: format!("U16Range<{MIN},{MAX}>"),
            }
        }
    }

    impl<const MIN: u32, const MAX: u32> ElicitPromptTree for U32Range<MIN, MAX> {
        fn prompt_tree() -> PromptTree {
            PromptTree::Leaf {
                prompt: <Self as Prompt>::prompt().unwrap_or("U32Range").to_string(),
                type_name: format!("U32Range<{MIN},{MAX}>"),
            }
        }
    }

    impl<const MIN: u64, const MAX: u64> ElicitPromptTree for U64Range<MIN, MAX> {
        fn prompt_tree() -> PromptTree {
            PromptTree::Leaf {
                prompt: <Self as Prompt>::prompt().unwrap_or("U64Range").to_string(),
                type_name: format!("U64Range<{MIN},{MAX}>"),
            }
        }
    }

    impl<const MIN: u128, const MAX: u128> ElicitPromptTree for U128Range<MIN, MAX> {
        fn prompt_tree() -> PromptTree {
            PromptTree::Leaf {
                prompt: <Self as Prompt>::prompt()
                    .unwrap_or("U128Range")
                    .to_string(),
                type_name: format!("U128Range<{MIN},{MAX}>"),
            }
        }
    }

    impl<const MIN: usize, const MAX: usize> ElicitPromptTree for UsizeRange<MIN, MAX> {
        fn prompt_tree() -> PromptTree {
            PromptTree::Leaf {
                prompt: <Self as Prompt>::prompt()
                    .unwrap_or("UsizeRange")
                    .to_string(),
                type_name: format!("UsizeRange<{MIN},{MAX}>"),
            }
        }
    }

    // ---- Floats ----

    leaf_impl!(F32Positive, "F32Positive");
    leaf_impl!(F32NonNegative, "F32NonNegative");
    leaf_impl!(F32Finite, "F32Finite");
    leaf_impl!(F32Default, "F32Default");
    leaf_impl!(F64Positive, "F64Positive");
    leaf_impl!(F64NonNegative, "F64NonNegative");
    leaf_impl!(F64Finite, "F64Finite");
    leaf_impl!(F64Default, "F64Default");

    // ---- Bools ----

    leaf_impl!(BoolTrue, "BoolTrue");
    leaf_impl!(BoolFalse, "BoolFalse");
    leaf_impl!(BoolDefault, "BoolDefault");

    // ---- Chars ----

    leaf_impl!(CharAlphabetic, "CharAlphabetic");
    leaf_impl!(CharAlphanumeric, "CharAlphanumeric");
    leaf_impl!(CharNumeric, "CharNumeric");

    // ---- Strings ----

    leaf_impl!(StringDefault, "StringDefault");

    impl<const MAX_LEN: usize> ElicitPromptTree for StringNonEmpty<MAX_LEN> {
        fn prompt_tree() -> PromptTree {
            PromptTree::Leaf {
                prompt: <Self as Prompt>::prompt()
                    .unwrap_or("StringNonEmpty")
                    .to_string(),
                type_name: "StringNonEmpty".to_string(),
            }
        }
    }

    // ---- Duration ----

    leaf_impl!(DurationPositive, "DurationPositive");

    // ---- Network ----

    leaf_impl!(IpPrivate, "IpPrivate");
    leaf_impl!(IpPublic, "IpPublic");
    leaf_impl!(IpV4, "IpV4");
    leaf_impl!(IpV6, "IpV6");
    leaf_impl!(Ipv4Loopback, "Ipv4Loopback");
    leaf_impl!(Ipv6Loopback, "Ipv6Loopback");

    // ---- Paths ----

    leaf_impl!(PathBufExists, "PathBufExists");
    leaf_impl!(PathBufReadable, "PathBufReadable");
    leaf_impl!(PathBufIsDir, "PathBufIsDir");
    leaf_impl!(PathBufIsFile, "PathBufIsFile");

    // ---- Collection contract types (generic) ----

    impl<T: ElicitPromptTree> ElicitPromptTree for VecNonEmpty<T> {
        fn prompt_tree() -> PromptTree {
            T::prompt_tree()
        }
    }

    impl<C: ElicitPromptTree> ElicitPromptTree for VecAllSatisfy<C> {
        fn prompt_tree() -> PromptTree {
            C::prompt_tree()
        }
    }

    impl<T: ElicitPromptTree> ElicitPromptTree for OptionSome<T> {
        fn prompt_tree() -> PromptTree {
            T::prompt_tree()
        }
    }

    impl<T: ElicitPromptTree> ElicitPromptTree for ResultOk<T> {
        fn prompt_tree() -> PromptTree {
            T::prompt_tree()
        }
    }

    impl<C: ElicitPromptTree> ElicitPromptTree for BoxSatisfies<C> {
        fn prompt_tree() -> PromptTree {
            C::prompt_tree()
        }
    }

    impl<C: ElicitPromptTree> ElicitPromptTree for ArcSatisfies<C> {
        fn prompt_tree() -> PromptTree {
            C::prompt_tree()
        }
    }

    impl<C: ElicitPromptTree> ElicitPromptTree for RcSatisfies<C> {
        fn prompt_tree() -> PromptTree {
            C::prompt_tree()
        }
    }

    impl<K, V: ElicitPromptTree> ElicitPromptTree for HashMapNonEmpty<K, V> {
        fn prompt_tree() -> PromptTree {
            V::prompt_tree()
        }
    }

    impl<K, V: ElicitPromptTree> ElicitPromptTree for BTreeMapNonEmpty<K, V> {
        fn prompt_tree() -> PromptTree {
            V::prompt_tree()
        }
    }

    impl<T: ElicitPromptTree> ElicitPromptTree for HashSetNonEmpty<T> {
        fn prompt_tree() -> PromptTree {
            T::prompt_tree()
        }
    }

    impl<T: ElicitPromptTree> ElicitPromptTree for BTreeSetNonEmpty<T> {
        fn prompt_tree() -> PromptTree {
            T::prompt_tree()
        }
    }

    impl<T: ElicitPromptTree> ElicitPromptTree for VecDequeNonEmpty<T> {
        fn prompt_tree() -> PromptTree {
            T::prompt_tree()
        }
    }

    impl<T: ElicitPromptTree> ElicitPromptTree for LinkedListNonEmpty<T> {
        fn prompt_tree() -> PromptTree {
            T::prompt_tree()
        }
    }

    impl<C: ElicitPromptTree, const N: usize> ElicitPromptTree for ArrayAllSatisfy<C, N> {
        fn prompt_tree() -> PromptTree {
            C::prompt_tree()
        }
    }

    impl<T: ElicitPromptTree> ElicitPromptTree for BoxNonNull<T> {
        fn prompt_tree() -> PromptTree {
            T::prompt_tree()
        }
    }

    impl<T: ElicitPromptTree> ElicitPromptTree for ArcNonNull<T> {
        fn prompt_tree() -> PromptTree {
            T::prompt_tree()
        }
    }

    impl<T: ElicitPromptTree> ElicitPromptTree for RcNonNull<T> {
        fn prompt_tree() -> PromptTree {
            T::prompt_tree()
        }
    }

    // ---- Tuples (Survey: each component is a named positional field) ----

    impl<A: ElicitPromptTree, B: ElicitPromptTree> ElicitPromptTree for Tuple2<A, B> {
        fn prompt_tree() -> PromptTree {
            PromptTree::Survey {
                prompt: Some("Eliciting tuple with 2 elements:".to_string()),
                type_name: "Tuple2".to_string(),
                fields: vec![
                    ("_0".to_string(), Box::new(A::prompt_tree())),
                    ("_1".to_string(), Box::new(B::prompt_tree())),
                ],
            }
        }
    }

    impl<A: ElicitPromptTree, B: ElicitPromptTree, C: ElicitPromptTree> ElicitPromptTree
        for Tuple3<A, B, C>
    {
        fn prompt_tree() -> PromptTree {
            PromptTree::Survey {
                prompt: Some("Eliciting tuple with 3 elements:".to_string()),
                type_name: "Tuple3".to_string(),
                fields: vec![
                    ("_0".to_string(), Box::new(A::prompt_tree())),
                    ("_1".to_string(), Box::new(B::prompt_tree())),
                    ("_2".to_string(), Box::new(C::prompt_tree())),
                ],
            }
        }
    }

    impl<A: ElicitPromptTree, B: ElicitPromptTree, C: ElicitPromptTree, D: ElicitPromptTree>
        ElicitPromptTree for Tuple4<A, B, C, D>
    {
        fn prompt_tree() -> PromptTree {
            PromptTree::Survey {
                prompt: Some("Eliciting tuple with 4 elements:".to_string()),
                type_name: "Tuple4".to_string(),
                fields: vec![
                    ("_0".to_string(), Box::new(A::prompt_tree())),
                    ("_1".to_string(), Box::new(B::prompt_tree())),
                    ("_2".to_string(), Box::new(C::prompt_tree())),
                    ("_3".to_string(), Box::new(D::prompt_tree())),
                ],
            }
        }
    }

    // ---- Plain Rust tuples (Survey: each component is a named positional field) ----

    impl<A: ElicitPromptTree, B: ElicitPromptTree> ElicitPromptTree for (A, B) {
        fn prompt_tree() -> PromptTree {
            PromptTree::Survey {
                prompt: Some("Eliciting 2-tuple:".to_string()),
                type_name: "(_, _)".to_string(),
                fields: vec![
                    ("_0".to_string(), Box::new(A::prompt_tree())),
                    ("_1".to_string(), Box::new(B::prompt_tree())),
                ],
            }
        }
    }

    impl<A: ElicitPromptTree, B: ElicitPromptTree, C: ElicitPromptTree> ElicitPromptTree for (A, B, C) {
        fn prompt_tree() -> PromptTree {
            PromptTree::Survey {
                prompt: Some("Eliciting 3-tuple:".to_string()),
                type_name: "(_, _, _)".to_string(),
                fields: vec![
                    ("_0".to_string(), Box::new(A::prompt_tree())),
                    ("_1".to_string(), Box::new(B::prompt_tree())),
                    ("_2".to_string(), Box::new(C::prompt_tree())),
                ],
            }
        }
    }

    impl<A: ElicitPromptTree, B: ElicitPromptTree, C: ElicitPromptTree, D: ElicitPromptTree>
        ElicitPromptTree for (A, B, C, D)
    {
        fn prompt_tree() -> PromptTree {
            PromptTree::Survey {
                prompt: Some("Eliciting 4-tuple:".to_string()),
                type_name: "(_, _, _, _)".to_string(),
                fields: vec![
                    ("_0".to_string(), Box::new(A::prompt_tree())),
                    ("_1".to_string(), Box::new(B::prompt_tree())),
                    ("_2".to_string(), Box::new(C::prompt_tree())),
                    ("_3".to_string(), Box::new(D::prompt_tree())),
                ],
            }
        }
    }

    // ---- Feature-gated types ----

    #[cfg(feature = "uuid")]
    mod uuid_impls {
        use super::*;
        use crate::verification::types::{UuidNonNil, UuidV4};
        leaf_impl!(UuidV4, "UuidV4");
        leaf_impl!(UuidNonNil, "UuidNonNil");
    }

    #[cfg(all(feature = "chrono", not(kani)))]
    mod chrono_impls {
        use super::*;
        use crate::verification::types::{DateTimeUtcAfter, DateTimeUtcBefore, NaiveDateTimeAfter};
        use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
        leaf_impl!(DateTime<Utc>, "DateTime<Utc>");
        leaf_impl!(DateTime<FixedOffset>, "DateTime<FixedOffset>");
        leaf_impl!(NaiveDateTime, "NaiveDateTime");
        leaf_impl!(DateTimeUtcAfter, "DateTimeUtcAfter");
        leaf_impl!(DateTimeUtcBefore, "DateTimeUtcBefore");
        leaf_impl!(NaiveDateTimeAfter, "NaiveDateTimeAfter");
    }

    #[cfg(all(feature = "time", not(kani)))]
    mod time_impls {
        use super::*;
        use crate::verification::types::{OffsetDateTimeAfter, OffsetDateTimeBefore};
        leaf_impl!(OffsetDateTimeAfter, "OffsetDateTimeAfter");
        leaf_impl!(OffsetDateTimeBefore, "OffsetDateTimeBefore");
    }

    #[cfg(all(feature = "jiff", not(kani)))]
    mod jiff_impls {
        use super::*;
        use crate::verification::types::{TimestampAfter, TimestampBefore};
        leaf_impl!(TimestampAfter, "TimestampAfter");
        leaf_impl!(TimestampBefore, "TimestampBefore");
    }

    #[cfg(feature = "url")]
    mod url_impls {
        use super::*;
        use crate::verification::types::{UrlCanBeBase, UrlHttp, UrlHttps, UrlValid, UrlWithHost};
        leaf_impl!(UrlValid, "UrlValid");
        leaf_impl!(UrlHttps, "UrlHttps");
        leaf_impl!(UrlHttp, "UrlHttp");
        leaf_impl!(UrlWithHost, "UrlWithHost");
        leaf_impl!(UrlCanBeBase, "UrlCanBeBase");
    }

    #[cfg(all(feature = "serde_json", not(kani)))]
    mod serde_json_impls {
        use super::*;
        use crate::verification::types::{ValueArray, ValueNonNull, ValueObject};
        leaf_impl!(ValueObject, "ValueObject");
        leaf_impl!(ValueArray, "ValueArray");
        leaf_impl!(ValueNonNull, "ValueNonNull");
        leaf_impl!(serde_json::Value, "serde_json::Value");
    }

    // Under kani, ValueObject/Array/NonNull use PhantomData stubs that don't
    // expose Prompt, so we skip them. serde_json::Value::prompt() is
    // unconditional and safe to use under kani.
    #[cfg(all(feature = "serde_json", kani))]
    mod serde_json_kani_impls {
        use super::*;
        leaf_impl!(serde_json::Value, "serde_json::Value");
    }

    #[cfg(feature = "regex")]
    mod regex_impls {
        use super::*;
        use crate::verification::types::{
            RegexCaseInsensitive, RegexMultiline, RegexSetNonEmpty, RegexSetValid, RegexValid,
        };
        leaf_impl!(RegexValid, "RegexValid");
        leaf_impl!(RegexSetValid, "RegexSetValid");
        leaf_impl!(RegexCaseInsensitive, "RegexCaseInsensitive");
        leaf_impl!(RegexMultiline, "RegexMultiline");
        leaf_impl!(RegexSetNonEmpty, "RegexSetNonEmpty");
    }

    #[cfg(feature = "reqwest")]
    mod reqwest_impls {
        use super::*;
        use crate::verification::types::StatusCodeValid;
        leaf_impl!(StatusCodeValid, "StatusCodeValid");
    }
}
