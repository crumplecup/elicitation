//! Disclosure descriptor types — footnotes and disclosure requirements.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ── Footnote ──────────────────────────────────────────────────────────────────

/// Descriptor for a single financial statement footnote.
///
/// The disclosure factory takes a `FootnoteDescriptor` and a specific
/// disclosure-topic factory method and returns an `Established<P>` token
/// proving that the required disclosure is present.
///
/// Source: ASC 235 — Notes to Financial Statements; ASC 205-10-50.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct FootnoteDescriptor {
    /// Unique note identifier (e.g. `"note-1"`, `"note-revenue"`).
    pub note_id: String,
    /// Note title (e.g. `"Significant Accounting Policies"`).
    pub title: String,
    /// Brief content summary (not the full footnote text; used for testing).
    pub content_summary: String,
    /// ASC references addressed by this note (e.g. `["ASC 606", "ASC 842"]`).
    pub asc_references: Vec<String>,
    /// Whether the note is flagged as material.
    pub is_material: bool,
}

// ── Disclosure requirement ────────────────────────────────────────────────────

/// A single required disclosure item, used by `GaapDisclosureMeta` to
/// enumerate outstanding disclosure obligations.
///
/// Source: FASB ASC §50 disclosure paragraphs throughout the Codification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct DisclosureRequirement {
    /// Unique requirement identifier.
    pub requirement_id: String,
    /// ASC citation (e.g. `"ASC 606-10-50-12"`).
    pub asc_reference: String,
    /// Description of what must be disclosed.
    pub description: String,
    /// Whether the requirement is considered material for this entity.
    pub is_material: bool,
    /// Whether this requirement has been satisfied by an existing footnote.
    pub is_satisfied: bool,
}
