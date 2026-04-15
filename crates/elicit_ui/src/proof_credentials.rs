//! Factory-internal proof-minting credentials for WCAG and UI-layout checks.
//!
//! Each type in this module is a **mint witness** — a value that can only be
//! constructed after a specific WCAG runtime check has passed inside the
//! corresponding factory method.  Passing one to
//! [`Established::prove`](elicitation::Established::prove) is the *only* way
//! to produce a proof token for the associated proposition.
//!
//! All types are zero-sized (`pub(crate)`).  External code cannot forge a
//! credential — they must go through the factory, which performs the actual
//! WCAG runtime check first.
//!
//! Each entry is declared with [`proof_credential!`](elicitation::proof_credential),
//! which emits both the ZST struct and its `ProvableFrom` binding in one place,
//! so credential and proposition can never drift apart.

use elicitation::proof_credential;

use crate::{
    FocusVisible, KeyboardAccessible, LevelAaEvidence, NoOverflow, OperableEvidence,
    PerceivedEvidence, RobustEvidence, UnderstandableEvidence, WcagAudioDescriptionPrerecorded,
    WcagCaptionsSynchronized, WcagCharacterShortcutsRemappable, WcagContrastEnhancedLargeText,
    WcagContrastEnhancedNormalText, WcagContrastMinimumLargeText, WcagContrastMinimumNormalText,
    WcagErrorIdentificationDescriptive, WcagErrorPreventionLegal, WcagErrorSuggestionProvided,
    WcagFocusAppearanceEnhancedArea, WcagFocusAppearanceMinimumArea, WcagFocusVisibleKeyboard,
    WcagFormLabelsProgrammatic, WcagHeadingStructureProgrammatic, WcagKeyboardNotTrapped,
    WcagKeyboardOperable, WcagLabelInNameMatch, WcagLabelsOrInstructionsPresent, WcagLevelAAValid,
    WcagListStructureProgrammatic, WcagNamePresent, WcagNonTextContrastMinimum, WcagOperableValid,
    WcagPageLanguageIdentified, WcagPartLanguageIdentified, WcagPerceivedValid,
    WcagPointerCancellationUpEvent, WcagPointerGesturesSimpleAlternative, WcagRobustValid,
    WcagTableHeadersProgrammatic, WcagTargetSizeEnhanced, WcagTargetSizeMinimum, WcagTextResizable,
    WcagTimingAdjustable, WcagUnderstandableValid,
};

// ── Contrast ──────────────────────────────────────────────────────────────────

proof_credential! {
    /// Witness that a colour pair meets the ≥ 4.5:1 ratio for normal text.
    ///
    /// Source: WCAG 2.2 SC 1.4.3 — Contrast (Minimum) Level AA.
    pub(crate) NormalTextContrastVerified => WcagContrastMinimumNormalText;

    /// Witness that a colour pair meets the ≥ 3:1 ratio for large text.
    ///
    /// Source: WCAG 2.2 SC 1.4.3 — Contrast (Minimum) Level AA.
    pub(crate) LargeTextContrastVerified => WcagContrastMinimumLargeText;

    /// Witness that a colour pair meets the ≥ 7:1 ratio for enhanced normal text.
    ///
    /// Source: WCAG 2.2 SC 1.4.6 — Contrast (Enhanced) Level AAA.
    pub(crate) EnhancedNormalTextContrastVerified => WcagContrastEnhancedNormalText;

    /// Witness that a colour pair meets the ≥ 4.5:1 ratio for enhanced large text.
    ///
    /// Source: WCAG 2.2 SC 1.4.6 — Contrast (Enhanced) Level AAA.
    pub(crate) EnhancedLargeTextContrastVerified => WcagContrastEnhancedLargeText;

    /// Witness that a colour pair meets the ≥ 3:1 ratio for non-text components.
    ///
    /// Source: WCAG 2.2 SC 1.4.11 — Non-text Contrast Level AA.
    pub(crate) NonTextContrastVerified => WcagNonTextContrastMinimum;
}

// ── Label ─────────────────────────────────────────────────────────────────────

proof_credential! {
    /// Witness that an element's accessible name is non-empty.
    ///
    /// Source: WCAG 2.2 SC 4.1.2 — Name, Role, Value Level A.
    pub(crate) AccessibleNameVerified => WcagNamePresent;

    /// Witness that a form field has a programmatically associated label.
    ///
    /// Source: WCAG 2.2 SC 1.3.1 / 3.3.2 Level A / AA.
    pub(crate) FormLabelVerified => WcagFormLabelsProgrammatic;

    /// Witness that an element's visible text matches its accessible name.
    ///
    /// Source: WCAG 2.2 SC 2.5.3 — Label in Name Level A.
    pub(crate) LabelInNameVerified => WcagLabelInNameMatch;
}

// ── Focus ─────────────────────────────────────────────────────────────────────

proof_credential! {
    /// Witness that a focus indicator satisfies minimum visibility (≥ 3:1 contrast).
    ///
    /// Source: WCAG 2.2 SC 2.4.7 — Focus Visible Level AA.
    pub(crate) FocusVisibleVerified => WcagFocusVisibleKeyboard;

    /// Witness that a focus indicator meets minimum area and contrast thresholds.
    ///
    /// Source: WCAG 2.2 SC 2.4.11 — Focus Appearance (Minimum) Level AA.
    pub(crate) FocusAppearanceMinimumVerified => WcagFocusAppearanceMinimumArea;

    /// Witness that a focus indicator meets enhanced area and contrast thresholds.
    ///
    /// Source: WCAG 2.2 SC 2.4.12 — Focus Appearance (Enhanced) Level AAA.
    pub(crate) FocusAppearanceEnhancedVerified => WcagFocusAppearanceEnhancedArea;
}

// ── Keyboard ──────────────────────────────────────────────────────────────────

proof_credential! {
    /// Witness that a widget is reachable via keyboard navigation.
    ///
    /// Source: WCAG 2.2 SC 2.1.1 — Keyboard Level A.
    pub(crate) KeyboardOperableVerified => WcagKeyboardOperable;

    /// Witness that a keyboard context can be escaped without a trap.
    ///
    /// Source: WCAG 2.2 SC 2.1.2 — No Keyboard Trap Level A.
    pub(crate) KeyboardEscapeVerified => WcagKeyboardNotTrapped;

    /// Witness that a character shortcut is remappable or focus-scoped.
    ///
    /// Source: WCAG 2.2 SC 2.1.4 — Character Key Shortcuts Level A.
    pub(crate) RemappableShortcutVerified => WcagCharacterShortcutsRemappable;
}

// ── Timing ────────────────────────────────────────────────────────────────────

proof_credential! {
    /// Witness that a timed element offers adjustable, pauseable, or disableable
    /// time controls.
    ///
    /// Source: WCAG 2.2 SC 2.2.1 — Timing Adjustable Level A.
    pub(crate) TimingAdjustableVerified => WcagTimingAdjustable;
}

// ── Target size ───────────────────────────────────────────────────────────────

proof_credential! {
    /// Witness that a pointer target meets the 24 × 24 CSS pixel minimum or
    /// adequate adjacent-spacing requirement.
    ///
    /// Source: WCAG 2.2 SC 2.5.8 — Target Size (Minimum) Level AA.
    pub(crate) TargetSizeMinimumVerified => WcagTargetSizeMinimum;

    /// Witness that a pointer target meets the 44 × 44 CSS pixel enhanced
    /// requirement.
    ///
    /// Source: WCAG 2.2 SC 2.5.5 — Target Size (Enhanced) Level AAA.
    pub(crate) TargetSizeEnhancedVerified => WcagTargetSizeEnhanced;

    /// Witness that a dragging gesture has a declared single-pointer alternative.
    ///
    /// Source: WCAG 2.2 SC 2.5.7 — Dragging Movements Level AA.
    pub(crate) PointerGestureAlternativeVerified => WcagPointerGesturesSimpleAlternative;

    /// Witness that a pointer interaction activates only on the up-event or
    /// provides an abort / undo mechanism.
    ///
    /// Source: WCAG 2.2 SC 2.5.2 — Pointer Cancellation Level A.
    pub(crate) PointerCancellationVerified => WcagPointerCancellationUpEvent;
}

// ── Structure ─────────────────────────────────────────────────────────────────

proof_credential! {
    /// Witness that a heading element was created at a valid hierarchical level.
    ///
    /// Source: WCAG 2.2 SC 1.3.1 — Info and Relationships Level A.
    pub(crate) HeadingCreated => WcagHeadingStructureProgrammatic;

    /// Witness that a list structure was created with programmatically
    /// determinable items.
    ///
    /// Source: WCAG 2.2 SC 1.3.1 — Info and Relationships Level A.
    pub(crate) ListCreated => WcagListStructureProgrammatic;

    /// Witness that a table was created with a programmatically associated
    /// caption / header.
    ///
    /// Source: WCAG 2.2 SC 1.3.1 — Info and Relationships Level A.
    pub(crate) TableCreated => WcagTableHeadersProgrammatic;

    /// Witness that a text block was constructed to allow resize up to 200 %
    /// without content or functionality loss.
    ///
    /// Source: WCAG 2.2 SC 1.4.4 — Resize Text Level AA.
    pub(crate) ResizableTextCreated => WcagTextResizable;
}

// ── Media ─────────────────────────────────────────────────────────────────────

proof_credential! {
    /// Witness that a media element was constructed with verified synchronised
    /// captions.
    ///
    /// Source: WCAG 2.2 SC 1.2.2 — Captions (Prerecorded) Level A.
    pub(crate) CaptionsVerified => WcagCaptionsSynchronized;

    /// Witness that a media element was constructed with a verified prerecorded
    /// audio description track.
    ///
    /// Source: WCAG 2.2 SC 1.2.5 — Audio Description (Prerecorded) Level AA.
    pub(crate) AudioDescriptionVerified => WcagAudioDescriptionPrerecorded;
}

// ── Language ──────────────────────────────────────────────────────────────────

proof_credential! {
    /// Witness that the page language has been programmatically identified.
    ///
    /// Source: WCAG 2.2 SC 3.1.1 — Language of Page Level A.
    pub(crate) PageLanguageVerified => WcagPageLanguageIdentified;

    /// Witness that a document part's language has been programmatically
    /// identified.
    ///
    /// Source: WCAG 2.2 SC 3.1.2 — Language of Parts Level AA.
    pub(crate) PartLanguageVerified => WcagPartLanguageIdentified;
}

// ── Error ─────────────────────────────────────────────────────────────────────

proof_credential! {
    /// Witness that an input error was verified to carry descriptive text.
    ///
    /// Source: WCAG 2.2 SC 3.3.1 — Error Identification Level A.
    pub(crate) ErrorIdentifiedVerified => WcagErrorIdentificationDescriptive;

    /// Witness that a form field carries labels or instructions.
    ///
    /// Source: WCAG 2.2 SC 3.3.2 — Labels or Instructions Level A.
    pub(crate) LabelsAndInstructionsVerified => WcagLabelsOrInstructionsPresent;

    /// Witness that an error suggestion was verified to be present and non-empty.
    ///
    /// Source: WCAG 2.2 SC 3.3.3 — Error Suggestion Level AA.
    pub(crate) ErrorSuggestionVerified => WcagErrorSuggestionProvided;

    /// Witness that an error-prevention mechanism (review, confirm, or reverse)
    /// has been declared for a submission.
    ///
    /// Source: WCAG 2.2 SC 3.3.4 — Error Prevention (Legal, Financial, Data) Level AA.
    pub(crate) ErrorPreventionVerified => WcagErrorPreventionLegal;
}

// ── Layout ────────────────────────────────────────────────────────────────────

proof_credential! {
    /// Witness that a layout container was created and children are constrained
    /// within bounds that prevent viewport overflow.
    ///
    /// Source: WCAG 2.2 SC 1.4.10 — Reflow Level AA.
    pub(crate) LayoutContainerCreated => NoOverflow;
}

// ── Navigation ────────────────────────────────────────────────────────────────

proof_credential! {
    /// Witness that a keyboard focus order was explicitly set on the surface.
    ///
    /// Source: WCAG 2.2 SC 2.4.3 — Focus Order Level A.
    pub(crate) FocusOrderSet => KeyboardAccessible;

    /// Witness that keyboard focus was directed to a specific widget, making its
    /// focus indicator visible.
    ///
    /// Source: WCAG 2.2 SC 2.4.7 — Focus Visible Level AA.
    pub(crate) FocusActivated => FocusVisible;

    /// Witness that a keyboard shortcut was registered with an accessible label.
    ///
    /// Source: WCAG 2.2 SC 2.1.4 — Character Key Shortcuts Level A.
    pub(crate) ShortcutRegistered => KeyboardAccessible;

    /// Witness that a skip-navigation link was created pointing to a valid target.
    ///
    /// Source: WCAG 2.2 SC 2.4.1 — Bypass Blocks Level A.
    pub(crate) SkipLinkAdded => KeyboardAccessible;
}

// ── Section aggregates (evidence bundles are the credentials) ─────────────────
//
// These five impls use the evidence bundle structs directly as credentials —
// no separate ZST is needed since the bundle is itself the proof of completeness.

use elicitation::contracts::ProvableFrom;

/// A full `PerceivedEvidence` bundle proves WCAG Principle 1 conformance.
impl ProvableFrom<PerceivedEvidence> for WcagPerceivedValid {}

/// A full `OperableEvidence` bundle proves WCAG Principle 2 conformance.
impl ProvableFrom<OperableEvidence> for WcagOperableValid {}

/// A full `UnderstandableEvidence` bundle proves WCAG Principle 3 conformance.
impl ProvableFrom<UnderstandableEvidence> for WcagUnderstandableValid {}

/// A full `RobustEvidence` bundle proves WCAG Principle 4 conformance.
impl ProvableFrom<RobustEvidence> for WcagRobustValid {}

/// A full `LevelAaEvidence` bundle (all four principles) proves WCAG Level AA.
impl ProvableFrom<LevelAaEvidence> for WcagLevelAAValid {}
