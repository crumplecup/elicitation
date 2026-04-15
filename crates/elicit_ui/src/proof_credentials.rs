//! Factory-internal proof-minting credentials for WCAG and UI-layout checks.
//!
//! Each type in this module is a **mint witness** — a value that can only be
//! constructed after a specific WCAG runtime check has passed inside the
//! corresponding factory method.  Passing one to
//! [`Established::prove`](elicitation::Established::prove) is the *only* way
//! to produce a proof token for the associated proposition.
//!
//! All types are zero-sized; the validated value is returned separately in the
//! factory result tuple.  These types exist purely to enforce the type-level
//! chain: only the method that performed the check can construct the credential,
//! and only `prove` can accept it.
//!
//! # Visibility
//!
//! All types are `pub(crate)`.  External code cannot forge a credential — they
//! must go through the factory, which performs the actual WCAG runtime check
//! first.

// ── Contrast credentials ──────────────────────────────────────────────────────

/// Witness that a colour pair meets the ≥ 4.5:1 ratio for normal text.
///
/// Source: WCAG 2.2 SC 1.4.3 — Contrast (Minimum) Level AA.
pub(crate) struct NormalTextContrastVerified;

/// Witness that a colour pair meets the ≥ 3:1 ratio for large text.
///
/// Source: WCAG 2.2 SC 1.4.3 — Contrast (Minimum) Level AA.
pub(crate) struct LargeTextContrastVerified;

/// Witness that a colour pair meets the ≥ 7:1 ratio for enhanced normal text.
///
/// Source: WCAG 2.2 SC 1.4.6 — Contrast (Enhanced) Level AAA.
pub(crate) struct EnhancedNormalTextContrastVerified;

/// Witness that a colour pair meets the ≥ 4.5:1 ratio for enhanced large text.
///
/// Source: WCAG 2.2 SC 1.4.6 — Contrast (Enhanced) Level AAA.
pub(crate) struct EnhancedLargeTextContrastVerified;

/// Witness that a colour pair meets the ≥ 3:1 ratio for non-text components.
///
/// Source: WCAG 2.2 SC 1.4.11 — Non-text Contrast Level AA.
pub(crate) struct NonTextContrastVerified;

// ── Label credentials ─────────────────────────────────────────────────────────

/// Witness that an element's accessible name is non-empty.
///
/// Source: WCAG 2.2 SC 4.1.2 — Name, Role, Value Level A.
pub(crate) struct AccessibleNameVerified;

/// Witness that a form field has a programmatically associated label.
///
/// Source: WCAG 2.2 SC 1.3.1 / 3.3.2 Level A / AA.
pub(crate) struct FormLabelVerified;

/// Witness that an element's visible text matches its accessible name.
///
/// Source: WCAG 2.2 SC 2.5.3 — Label in Name Level A.
pub(crate) struct LabelInNameVerified;

// ── Focus credentials ─────────────────────────────────────────────────────────

/// Witness that a focus indicator satisfies minimum visibility (≥ 3:1 contrast).
///
/// Source: WCAG 2.2 SC 2.4.7 — Focus Visible Level AA.
pub(crate) struct FocusVisibleVerified;

/// Witness that a focus indicator meets minimum area and contrast thresholds.
///
/// Source: WCAG 2.2 SC 2.4.11 — Focus Appearance (Minimum) Level AA.
pub(crate) struct FocusAppearanceMinimumVerified;

/// Witness that a focus indicator meets enhanced area and contrast thresholds.
///
/// Source: WCAG 2.2 SC 2.4.12 — Focus Appearance (Enhanced) Level AAA.
pub(crate) struct FocusAppearanceEnhancedVerified;

// ── Keyboard credentials ──────────────────────────────────────────────────────

/// Witness that a widget is reachable via keyboard navigation.
///
/// Source: WCAG 2.2 SC 2.1.1 — Keyboard Level A.
pub(crate) struct KeyboardOperableVerified;

/// Witness that a keyboard context can be escaped without a trap.
///
/// Source: WCAG 2.2 SC 2.1.2 — No Keyboard Trap Level A.
pub(crate) struct KeyboardEscapeVerified;

/// Witness that a character shortcut is remappable or focus-scoped.
///
/// Source: WCAG 2.2 SC 2.1.4 — Character Key Shortcuts Level A.
pub(crate) struct RemappableShortcutVerified;

// ── Timing credentials ────────────────────────────────────────────────────────

/// Witness that a timed element offers adjustable, pauseable, or disableable
/// time controls.
///
/// Source: WCAG 2.2 SC 2.2.1 — Timing Adjustable Level A.
pub(crate) struct TimingAdjustableVerified;

// ── Target size credentials ───────────────────────────────────────────────────

/// Witness that a pointer target meets the 24 × 24 CSS pixel minimum or
/// adequate adjacent-spacing requirement.
///
/// Source: WCAG 2.2 SC 2.5.8 — Target Size (Minimum) Level AA.
pub(crate) struct TargetSizeMinimumVerified;

/// Witness that a pointer target meets the 44 × 44 CSS pixel enhanced
/// requirement.
///
/// Source: WCAG 2.2 SC 2.5.5 — Target Size (Enhanced) Level AAA.
pub(crate) struct TargetSizeEnhancedVerified;

/// Witness that a dragging gesture has a declared single-pointer alternative.
///
/// Source: WCAG 2.2 SC 2.5.7 — Dragging Movements Level AA.
pub(crate) struct PointerGestureAlternativeVerified;

/// Witness that a pointer interaction activates only on the up-event or
/// provides an abort / undo mechanism.
///
/// Source: WCAG 2.2 SC 2.5.2 — Pointer Cancellation Level A.
pub(crate) struct PointerCancellationVerified;

// ── Structure credentials ─────────────────────────────────────────────────────

/// Witness that a heading element was created at a valid hierarchical level.
///
/// Source: WCAG 2.2 SC 1.3.1 — Info and Relationships Level A.
pub(crate) struct HeadingCreated;

/// Witness that a list structure was created with programmatically
/// determinable items.
///
/// Source: WCAG 2.2 SC 1.3.1 — Info and Relationships Level A.
pub(crate) struct ListCreated;

/// Witness that a table was created with a programmatically associated
/// caption / header.
///
/// Source: WCAG 2.2 SC 1.3.1 — Info and Relationships Level A.
pub(crate) struct TableCreated;

/// Witness that a text block was constructed to allow resize up to 200 %
/// without content or functionality loss.
///
/// Source: WCAG 2.2 SC 1.4.4 — Resize Text Level AA.
pub(crate) struct ResizableTextCreated;

// ── Media credentials ─────────────────────────────────────────────────────────

/// Witness that a media element was constructed with verified synchronised
/// captions.
///
/// Source: WCAG 2.2 SC 1.2.2 — Captions (Prerecorded) Level A.
pub(crate) struct CaptionsVerified;

/// Witness that a media element was constructed with a verified prerecorded
/// audio description track.
///
/// Source: WCAG 2.2 SC 1.2.5 — Audio Description (Prerecorded) Level AA.
pub(crate) struct AudioDescriptionVerified;

// ── Language credentials ──────────────────────────────────────────────────────

/// Witness that the page language has been programmatically identified.
///
/// Source: WCAG 2.2 SC 3.1.1 — Language of Page Level A.
pub(crate) struct PageLanguageVerified;

/// Witness that a document part's language has been programmatically
/// identified.
///
/// Source: WCAG 2.2 SC 3.1.2 — Language of Parts Level AA.
pub(crate) struct PartLanguageVerified;

// ── Error credentials ─────────────────────────────────────────────────────────

/// Witness that an input error was verified to carry descriptive text.
///
/// Source: WCAG 2.2 SC 3.3.1 — Error Identification Level A.
pub(crate) struct ErrorIdentifiedVerified;

/// Witness that a form field carries labels or instructions.
///
/// Source: WCAG 2.2 SC 3.3.2 — Labels or Instructions Level A.
pub(crate) struct LabelsAndInstructionsVerified;

/// Witness that an error suggestion was verified to be present and non-empty.
///
/// Source: WCAG 2.2 SC 3.3.3 — Error Suggestion Level AA.
pub(crate) struct ErrorSuggestionVerified;

/// Witness that an error-prevention mechanism (review, confirm, or reverse)
/// has been declared for a submission.
///
/// Source: WCAG 2.2 SC 3.3.4 — Error Prevention (Legal, Financial, Data) Level AA.
pub(crate) struct ErrorPreventionVerified;

// ── Layout credentials ────────────────────────────────────────────────────────

/// Witness that a layout container was created and children are constrained
/// within bounds that prevent viewport overflow.
///
/// Source: WCAG 2.2 SC 1.4.10 — Reflow Level AA.
pub(crate) struct LayoutContainerCreated;

// ── Navigation credentials ────────────────────────────────────────────────────

/// Witness that a keyboard focus order was explicitly set on the surface.
///
/// Source: WCAG 2.2 SC 2.4.3 — Focus Order Level A.
pub(crate) struct FocusOrderSet;

/// Witness that keyboard focus was directed to a specific widget, making its
/// focus indicator visible.
///
/// Source: WCAG 2.2 SC 2.4.7 — Focus Visible Level AA.
pub(crate) struct FocusActivated;

/// Witness that a keyboard shortcut was registered with an accessible label.
///
/// Source: WCAG 2.2 SC 2.1.4 — Character Key Shortcuts Level A.
pub(crate) struct ShortcutRegistered;

/// Witness that a skip-navigation link was created pointing to a valid target.
///
/// Source: WCAG 2.2 SC 2.4.1 — Bypass Blocks Level A.
pub(crate) struct SkipLinkAdded;
