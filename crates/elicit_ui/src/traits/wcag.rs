//! WCAG 2.2 factory and reporter traits.
//!
//! # Three-role taxonomy
//!
//! ## Role 1a — Leaf factories
//!
//! Each leaf factory takes raw descriptor data and either returns a validated
//! construct plus a proof token (`Established<P>`) or returns an error.
//!
//! | Trait | Domain | Key SC |
//! |---|---|---|
//! | [`WcagContrastFactory`] | Colour contrast | 1.4.3, 1.4.6, 1.4.11 |
//! | [`WcagLabelFactory`] | Accessible names | 1.1.1, 4.1.2 |
//! | [`WcagFocusFactory`] | Focus visibility | 2.4.7, 2.4.11, 2.4.12 |
//! | [`WcagKeyboardFactory`] | Keyboard nav | 2.1.x, 2.4.1, 2.4.3 |
//! | [`WcagTimingFactory`] | Time limits | 2.2.x |
//! | [`WcagTargetFactory`] | Pointer targets | 2.5.5, 2.5.8 |
//! | [`WcagStructureFactory`] | Adaptable structure | 1.3.x |
//! | [`WcagMediaFactory`] | Time-based media | 1.2.x |
//! | [`WcagLanguageFactory`] | Language of page/parts | 3.1.x |
//! | [`WcagErrorFactory`] | Input assistance | 3.3.x |
//!
//! ## Role 1b — Section factories
//!
//! Section factories accept an *evidence bundle* — a struct of
//! `Established<P>` proof tokens obtained from leaf factories — and convert
//! it into a principle-level proof.  This is the compositionality mechanism:
//! the type system prevents calling a section factory unless all required
//! leaf-factory proofs are in hand.
//!
//! | Trait | Principle | Output proof |
//! |---|---|---|
//! | [`WcagPerceivedFactory`] | Principle 1 | `Established<WcagPerceivedValid>` |
//! | [`WcagOperableFactory`] | Principle 2 | `Established<WcagOperableValid>` |
//! | [`WcagUnderstandableFactory`] | Principle 3 | `Established<WcagUnderstandableValid>` |
//! | [`WcagRobustFactory`] | Principle 4 | `Established<WcagRobustValid>` |
//!
//! ## Role 2 — Orthogonal reporters
//!
//! Reporters query accessibility metadata without requiring or producing proof
//! tokens.  They capture a concern that is orthogonal to geometric correctness:
//! you can ask for an element's ARIA role whether or not the element is valid.
//!
//! | Trait | Concern |
//! |---|---|
//! | [`WcagElementMeta`] | Per-element ARIA attributes |
//! | [`WcagPageMeta`] | Page-level language and navigation structure |
//!
//! ## Role 3 — Backend supertrait
//!
//! [`WcagBackend`] is the supertrait that blanket-impls when all 16 constituent
//! traits are implemented.  Its [`WcagBackend::build_level_aa`] method is the
//! top of the proof chain: it consumes a [`LevelAaEvidence`] bundle and returns
//! `Established<WcagLevelAAValid>`.

use elicitation::Established;

use crate::{
    CaptionedMedia, ContrastDescriptor, ContrastPair, ErrorDescriptor, ErrorField, FocusDescriptor,
    FocusIndicator, KeyboardDescriptor, KeyboardPath, LabelDescriptor, LabeledElement,
    LanguageDescriptor, LanguagePage, LevelAaEvidence, MediaDescriptor, OperableEvidence,
    OperableInterface, PerceivedEvidence, PerceivedSection, PointerTarget, RobustEvidence,
    RobustWidget, StructureDescriptor, StructuredElement, TargetDescriptor, TimedElement,
    TimingDescriptor, UiResult, UnderstandableEvidence, UnderstandableInterface,
    WcagAudioDescriptionPrerecorded, WcagCaptionsSynchronized, WcagCharacterShortcutsRemappable,
    WcagContrastEnhancedLargeText, WcagContrastEnhancedNormalText, WcagContrastMinimumLargeText,
    WcagContrastMinimumNormalText, WcagErrorIdentificationDescriptive, WcagErrorPreventionLegal,
    WcagErrorSuggestionProvided, WcagFocusAppearanceEnhancedArea, WcagFocusAppearanceMinimumArea,
    WcagFocusVisibleKeyboard, WcagFormLabelsProgrammatic, WcagHeadingStructureProgrammatic,
    WcagKeyboardNotTrapped, WcagKeyboardOperable, WcagLabelInNameMatch,
    WcagLabelsOrInstructionsPresent, WcagLevelAAValid, WcagListStructureProgrammatic,
    WcagNamePresent, WcagNonTextContrastMinimum, WcagOperableValid, WcagPageLanguageIdentified,
    WcagPartLanguageIdentified, WcagPerceivedValid, WcagPointerCancellationUpEvent,
    WcagPointerGesturesSimpleAlternative, WcagRobustValid, WcagTableHeadersProgrammatic,
    WcagTargetSizeEnhanced, WcagTargetSizeMinimum, WcagTextResizable, WcagTimingAdjustable,
    WcagUnderstandableValid, WidgetId,
};

// ── Role 1a: Leaf factories ───────────────────────────────────────────────────

/// Constructs contrast-valid colour pairs from raw foreground/background data.
///
/// Methods perform the WCAG 2.1 relative-luminance calculation and return a
/// [`ContrastPair`] plus a proof token only when the ratio meets the threshold.
///
/// Source: WCAG 2.2 Guideline 1.4 — Distinguishable
pub trait WcagContrastFactory: Send + Sync {
    /// Build a colour pair satisfying the 4.5:1 normal-text threshold.
    ///
    /// Source: WCAG 2.2 SC 1.4.3 — Contrast (Minimum), Level AA
    fn build_contrast_minimum(
        &self,
        input: ContrastDescriptor,
    ) -> UiResult<(ContrastPair, Established<WcagContrastMinimumNormalText>)>;

    /// Build a colour pair satisfying the 3:1 large-text threshold.
    ///
    /// Source: WCAG 2.2 SC 1.4.3 — Contrast (Minimum), Level AA
    fn build_contrast_minimum_large(
        &self,
        input: ContrastDescriptor,
    ) -> UiResult<(ContrastPair, Established<WcagContrastMinimumLargeText>)>;

    /// Build a colour pair satisfying the 7:1 enhanced normal-text threshold.
    ///
    /// Source: WCAG 2.2 SC 1.4.6 — Contrast (Enhanced), Level AAA
    fn build_contrast_enhanced(
        &self,
        input: ContrastDescriptor,
    ) -> UiResult<(ContrastPair, Established<WcagContrastEnhancedNormalText>)>;

    /// Build a colour pair satisfying the 4.5:1 enhanced large-text threshold.
    ///
    /// Source: WCAG 2.2 SC 1.4.6 — Contrast (Enhanced), Level AAA
    fn build_contrast_enhanced_large(
        &self,
        input: ContrastDescriptor,
    ) -> UiResult<(ContrastPair, Established<WcagContrastEnhancedLargeText>)>;

    /// Build a colour pair satisfying the 3:1 non-text-component threshold.
    ///
    /// Source: WCAG 2.2 SC 1.4.11 — Non-text Contrast, Level AA
    fn build_non_text_contrast(
        &self,
        input: ContrastDescriptor,
    ) -> UiResult<(ContrastPair, Established<WcagNonTextContrastMinimum>)>;
}

/// Constructs labeled elements, establishing the presence of an accessible name.
///
/// Source: WCAG 2.2 SC 1.1.1 — Non-text Content; SC 4.1.2 — Name, Role, Value
pub trait WcagLabelFactory: Send + Sync {
    /// Assign an accessible name to a widget.
    ///
    /// Returns an error if `name` is empty.
    ///
    /// Source: WCAG 2.2 SC 4.1.2 — Name, Role, Value, Level A
    fn build_labeled_element(
        &self,
        input: LabelDescriptor,
    ) -> UiResult<(LabeledElement, Established<WcagNamePresent>)>;

    /// Build a labeled form field, establishing both a name and an instructions
    /// association.
    ///
    /// Source: WCAG 2.2 SC 1.3.1 — Info and Relationships, Level A;
    /// SC 3.3.2 — Labels or Instructions, Level A
    fn build_labeled_form_field(
        &self,
        input: LabelDescriptor,
    ) -> UiResult<(LabeledElement, Established<WcagFormLabelsProgrammatic>)>;

    /// Build a labeled element whose visible label matches its accessible name.
    ///
    /// Source: WCAG 2.2 SC 2.5.3 — Label in Name, Level A
    fn build_label_in_name(
        &self,
        input: LabelDescriptor,
    ) -> UiResult<(LabeledElement, Established<WcagLabelInNameMatch>)>;
}

/// Constructs focus-visible elements and validated focus indicators.
///
/// Source: WCAG 2.2 Guideline 2.4 — Navigable
pub trait WcagFocusFactory: Send + Sync {
    /// Build a focusable widget with a visible focus indicator.
    ///
    /// Source: WCAG 2.2 SC 2.4.7 — Focus Visible, Level AA
    fn build_focus_visible(
        &self,
        input: FocusDescriptor,
    ) -> UiResult<(FocusIndicator, Established<WcagFocusVisibleKeyboard>)>;

    /// Build a focus indicator meeting the minimum area and contrast thresholds.
    ///
    /// Source: WCAG 2.2 SC 2.4.11 — Focus Appearance (Minimum), Level AA
    fn build_focus_appearance_minimum(
        &self,
        input: FocusDescriptor,
    ) -> UiResult<(FocusIndicator, Established<WcagFocusAppearanceMinimumArea>)>;

    /// Build a focus indicator meeting the enhanced area and contrast thresholds.
    ///
    /// Source: WCAG 2.2 SC 2.4.12 — Focus Appearance (Enhanced), Level AAA
    fn build_focus_appearance_enhanced(
        &self,
        input: FocusDescriptor,
    ) -> UiResult<(FocusIndicator, Established<WcagFocusAppearanceEnhancedArea>)>;
}

/// Constructs keyboard-accessible widgets and navigation paths.
///
/// Source: WCAG 2.2 Guideline 2.1 — Keyboard Accessible
pub trait WcagKeyboardFactory: Send + Sync {
    /// Place a widget in the keyboard navigation order without a timing
    /// requirement.
    ///
    /// Source: WCAG 2.2 SC 2.1.1 — Keyboard, Level A
    fn build_keyboard_accessible(
        &self,
        input: KeyboardDescriptor,
    ) -> UiResult<(KeyboardPath, Established<WcagKeyboardOperable>)>;

    /// Establish that a component can be exited via keyboard without a trap.
    ///
    /// Source: WCAG 2.2 SC 2.1.2 — No Keyboard Trap, Level A
    fn build_keyboard_escape(
        &self,
        input: KeyboardDescriptor,
    ) -> UiResult<(KeyboardPath, Established<WcagKeyboardNotTrapped>)>;

    /// Register a single-character shortcut as remappable or focus-restricted.
    ///
    /// Source: WCAG 2.2 SC 2.1.4 — Character Key Shortcuts, Level A
    fn build_remappable_shortcut(
        &self,
        input: KeyboardDescriptor,
    ) -> UiResult<(KeyboardPath, Established<WcagCharacterShortcutsRemappable>)>;
}

/// Constructs timing-controlled elements that satisfy WCAG time-limit requirements.
///
/// Source: WCAG 2.2 Guideline 2.2 — Enough Time
pub trait WcagTimingFactory: Send + Sync {
    /// Build a timed element with adjustable or disableable time limits.
    ///
    /// Source: WCAG 2.2 SC 2.2.1 — Timing Adjustable, Level A
    fn build_timed_element(
        &self,
        input: TimingDescriptor,
    ) -> UiResult<(TimedElement, Established<WcagTimingAdjustable>)>;
}

/// Constructs pointer targets that satisfy WCAG touch/mouse size requirements.
///
/// Source: WCAG 2.2 Guideline 2.5 — Input Modalities
pub trait WcagTargetFactory: Send + Sync {
    /// Build a pointer target meeting the 24×24 CSS pixel minimum size or
    /// spacing requirement.
    ///
    /// Source: WCAG 2.2 SC 2.5.8 — Target Size (Minimum), Level AA
    fn build_target_minimum(
        &self,
        input: TargetDescriptor,
    ) -> UiResult<(PointerTarget, Established<WcagTargetSizeMinimum>)>;

    /// Build a pointer target meeting the 44×44 CSS pixel enhanced size.
    ///
    /// Source: WCAG 2.2 SC 2.5.5 — Target Size (Enhanced), Level AAA
    fn build_target_enhanced(
        &self,
        input: TargetDescriptor,
    ) -> UiResult<(PointerTarget, Established<WcagTargetSizeEnhanced>)>;

    /// Build an alternative to a dragging operation that uses a single pointer.
    ///
    /// Source: WCAG 2.2 SC 2.5.7 — Dragging Movements, Level AA
    fn build_pointer_gesture_alternative(
        &self,
        input: TargetDescriptor,
    ) -> UiResult<(
        PointerTarget,
        Established<WcagPointerGesturesSimpleAlternative>,
    )>;

    /// Build a pointer interaction that does not activate on the down-event.
    ///
    /// Source: WCAG 2.2 SC 2.5.2 — Pointer Cancellation, Level A
    fn build_pointer_cancellation(
        &self,
        input: TargetDescriptor,
    ) -> UiResult<(PointerTarget, Established<WcagPointerCancellationUpEvent>)>;
}

/// Constructs elements with programmatically determinable semantic structure.
///
/// Source: WCAG 2.2 Guideline 1.3 — Adaptable
pub trait WcagStructureFactory: Send + Sync {
    /// Build a heading element at a valid hierarchical level.
    ///
    /// Source: WCAG 2.2 SC 1.3.1 — Info and Relationships, Level A
    fn build_heading(
        &self,
        input: StructureDescriptor,
    ) -> UiResult<(
        StructuredElement,
        Established<WcagHeadingStructureProgrammatic>,
    )>;

    /// Build a list structure with programmatically determinable items.
    ///
    /// Source: WCAG 2.2 SC 1.3.1 — Info and Relationships, Level A
    fn build_list(
        &self,
        input: StructureDescriptor,
    ) -> UiResult<(
        StructuredElement,
        Established<WcagListStructureProgrammatic>,
    )>;

    /// Build a table with programmatically associated headers.
    ///
    /// Source: WCAG 2.2 SC 1.3.1 — Info and Relationships, Level A
    fn build_table(
        &self,
        input: StructureDescriptor,
    ) -> UiResult<(StructuredElement, Established<WcagTableHeadersProgrammatic>)>;

    /// Build an element that can be resized up to 200 % without content loss.
    ///
    /// Source: WCAG 2.2 SC 1.4.4 — Resize Text, Level AA
    fn build_resizable_text(
        &self,
        input: StructureDescriptor,
    ) -> UiResult<(StructuredElement, Established<WcagTextResizable>)>;
}

/// Constructs accessible media elements with captions and audio descriptions.
///
/// Source: WCAG 2.2 Guideline 1.2 — Time-based Media
pub trait WcagMediaFactory: Send + Sync {
    /// Build a media element with synchronised captions.
    ///
    /// Source: WCAG 2.2 SC 1.2.2 — Captions (Prerecorded), Level A
    fn build_captioned_media(
        &self,
        input: MediaDescriptor,
    ) -> UiResult<(CaptionedMedia, Established<WcagCaptionsSynchronized>)>;

    /// Build a media element with a prerecorded audio description.
    ///
    /// Source: WCAG 2.2 SC 1.2.5 — Audio Description (Prerecorded), Level AA
    fn build_audio_described_media(
        &self,
        input: MediaDescriptor,
    ) -> UiResult<(CaptionedMedia, Established<WcagAudioDescriptionPrerecorded>)>;
}

/// Constructs language-identified pages and elements.
///
/// Source: WCAG 2.2 Guideline 3.1 — Readable
pub trait WcagLanguageFactory: Send + Sync {
    /// Build a page with a programmatically identified default language.
    ///
    /// Source: WCAG 2.2 SC 3.1.1 — Language of Page, Level A
    fn build_language_page(
        &self,
        input: LanguageDescriptor,
    ) -> UiResult<(LanguagePage, Established<WcagPageLanguageIdentified>)>;

    /// Build an element whose language differs from the page default and is
    /// identified programmatically.
    ///
    /// Source: WCAG 2.2 SC 3.1.2 — Language of Parts, Level AA
    fn build_language_element(
        &self,
        input: LanguageDescriptor,
    ) -> UiResult<(LanguagePage, Established<WcagPartLanguageIdentified>)>;
}

/// Constructs error-identified form fields with suggestions.
///
/// Source: WCAG 2.2 Guideline 3.3 — Input Assistance
pub trait WcagErrorFactory: Send + Sync {
    /// Build a form field with a descriptive error message.
    ///
    /// Source: WCAG 2.2 SC 3.3.1 — Error Identification, Level A
    fn build_identified_error(
        &self,
        input: ErrorDescriptor,
    ) -> UiResult<(ErrorField, Established<WcagErrorIdentificationDescriptive>)>;

    /// Build a form field with labels or instructions present.
    ///
    /// Source: WCAG 2.2 SC 3.3.2 — Labels or Instructions, Level A
    fn build_labeled_field(
        &self,
        input: ErrorDescriptor,
    ) -> UiResult<(ErrorField, Established<WcagLabelsOrInstructionsPresent>)>;

    /// Build a form field with an actionable correction suggestion.
    ///
    /// Source: WCAG 2.2 SC 3.3.3 — Error Suggestion, Level AA
    fn build_error_suggestion(
        &self,
        input: ErrorDescriptor,
    ) -> UiResult<(ErrorField, Established<WcagErrorSuggestionProvided>)>;

    /// Build a form that prevents, checks, or reverses submissions for legal
    /// and financial transactions.
    ///
    /// Source: WCAG 2.2 SC 3.3.4 — Error Prevention (Legal, Financial, Data),
    /// Level AA
    fn build_error_prevention(
        &self,
        input: ErrorDescriptor,
    ) -> UiResult<(ErrorField, Established<WcagErrorPreventionLegal>)>;
}

// ── Role 1b: Section factories ────────────────────────────────────────────────

/// Converts leaf-factory evidence into a Principle 1 (Perceivable) proof.
///
/// # Evidence chain
///
/// The caller must supply a [`PerceivedEvidence`] bundle — a struct of
/// `Established<P>` tokens — which was obtained by running all required leaf
/// factories.  Having the bundle proves the preconditions; the method produces
/// the principle-level proof.
///
/// Source: WCAG 2.2 Principle 1 — Perceivable
pub trait WcagPerceivedFactory: Send + Sync {
    /// Combine Principle 1 evidence into a perceivable-section construct.
    ///
    /// Because the evidence bundle can only exist when all required leaf proofs
    /// have been established, this method is infallible: the type system has
    /// already enforced the preconditions.
    ///
    /// Source: WCAG 2.2 Principle 1 — Perceivable
    fn build_perceivable(
        &self,
        evidence: PerceivedEvidence,
    ) -> (PerceivedSection, Established<WcagPerceivedValid>);
}

/// Converts leaf-factory evidence into a Principle 2 (Operable) proof.
///
/// Source: WCAG 2.2 Principle 2 — Operable
pub trait WcagOperableFactory: Send + Sync {
    /// Combine Principle 2 evidence into an operable-interface construct.
    ///
    /// Source: WCAG 2.2 Principle 2 — Operable
    fn build_operable(
        &self,
        evidence: OperableEvidence,
    ) -> (OperableInterface, Established<WcagOperableValid>);
}

/// Converts leaf-factory evidence into a Principle 3 (Understandable) proof.
///
/// Source: WCAG 2.2 Principle 3 — Understandable
pub trait WcagUnderstandableFactory: Send + Sync {
    /// Combine Principle 3 evidence into an understandable-interface construct.
    ///
    /// Source: WCAG 2.2 Principle 3 — Understandable
    fn build_understandable(
        &self,
        evidence: UnderstandableEvidence,
    ) -> (
        UnderstandableInterface,
        Established<WcagUnderstandableValid>,
    );
}

/// Converts leaf-factory evidence into a Principle 4 (Robust) proof.
///
/// Source: WCAG 2.2 Principle 4 — Robust
pub trait WcagRobustFactory: Send + Sync {
    /// Combine Principle 4 evidence into a robust-widget construct.
    ///
    /// Source: WCAG 2.2 Principle 4 — Robust
    fn build_robust(
        &self,
        evidence: RobustEvidence,
    ) -> (RobustWidget, Established<WcagRobustValid>);
}

// ── Role 2: Orthogonal reporters ─────────────────────────────────────────────

/// Queries per-element ARIA accessibility attributes.
///
/// Reporter methods do not produce or require proof tokens; they surface
/// metadata about any element regardless of its validity.  Useful for
/// diagnostics, audit reports, and dev-tool introspection.
///
/// Source: WCAG 2.2 SC 4.1.2 — Name, Role, Value
pub trait WcagElementMeta: Send + Sync {
    /// Return the programmatic role of a widget (e.g., `"button"`, `"heading"`).
    ///
    /// Source: WCAG 2.2 SC 4.1.2 — Name, Role, Value, Level A
    fn element_role(&self, id: WidgetId) -> UiResult<Option<String>>;

    /// Return the accessible name of a widget, if present.
    ///
    /// Source: WCAG 2.2 SC 4.1.2 — Name, Role, Value, Level A
    fn element_label(&self, id: WidgetId) -> UiResult<Option<String>>;

    /// Return the accessible description of a widget, if present.
    ///
    /// Source: WCAG 2.2 SC 4.1.2 — Name, Role, Value, Level A
    fn element_description(&self, id: WidgetId) -> UiResult<Option<String>>;

    /// Return whether a widget currently has keyboard focus.
    ///
    /// Source: WCAG 2.2 SC 2.4.7 — Focus Visible, Level AA
    fn element_has_focus(&self, id: WidgetId) -> UiResult<bool>;

    /// Return the current programmatic state as a human-readable string
    /// (e.g., `"checked"`, `"expanded"`, `"disabled"`).
    ///
    /// Source: WCAG 2.2 SC 4.1.2 — Name, Role, Value, Level A
    fn element_state(&self, id: WidgetId) -> UiResult<Option<String>>;
}

/// Queries page-level accessibility metadata.
///
/// Reporter methods do not produce or require proof tokens.
///
/// Source: WCAG 2.2 Guideline 3.1 — Readable; Guideline 2.4 — Navigable
pub trait WcagPageMeta: Send + Sync {
    /// Return the page title, if present.
    ///
    /// Source: WCAG 2.2 SC 2.4.2 — Page Titled, Level A
    fn page_title(&self) -> UiResult<Option<String>>;

    /// Return the BCP-47 language tag of the page, if present.
    ///
    /// Source: WCAG 2.2 SC 3.1.1 — Language of Page, Level A
    fn page_language(&self) -> UiResult<Option<String>>;

    /// Return the IDs of all navigation landmark widgets.
    ///
    /// Source: WCAG 2.2 SC 2.4.1 — Bypass Blocks, Level A
    fn navigation_landmarks(&self) -> UiResult<Vec<WidgetId>>;

    /// Return all heading widgets in document order.
    ///
    /// Source: WCAG 2.2 SC 2.4.6 — Headings and Labels, Level AA
    fn page_headings(&self) -> UiResult<Vec<WidgetId>>;
}

// ── Role 3: Backend supertrait ────────────────────────────────────────────────

/// Full WCAG 2.2 backend — blanket impl for anything implementing all 16 traits.
///
/// `WcagBackend::build_level_aa` is the top of the proof chain: it accepts a
/// [`LevelAaEvidence`] bundle (itself requiring principle-level proofs from all
/// four section factories) and returns `Established<WcagLevelAAValid>`.
///
/// Source: WCAG 2.2 — full conformance
pub trait WcagBackend:
    WcagContrastFactory
    + WcagLabelFactory
    + WcagFocusFactory
    + WcagKeyboardFactory
    + WcagTimingFactory
    + WcagTargetFactory
    + WcagStructureFactory
    + WcagMediaFactory
    + WcagLanguageFactory
    + WcagErrorFactory
    + WcagPerceivedFactory
    + WcagOperableFactory
    + WcagUnderstandableFactory
    + WcagRobustFactory
    + WcagElementMeta
    + WcagPageMeta
    + Send
    + Sync
{
    /// Convert four principle-level proofs into a full Level AA conformance proof.
    ///
    /// The [`LevelAaEvidence`] bundle can only be constructed when all four
    /// section factories have been run successfully, making this method
    /// infallible.
    ///
    /// Source: WCAG 2.2 — Level AA conformance
    fn build_level_aa(&self, evidence: LevelAaEvidence) -> Established<WcagLevelAAValid>;
}

impl<T> WcagBackend for T
where
    T: WcagContrastFactory
        + WcagLabelFactory
        + WcagFocusFactory
        + WcagKeyboardFactory
        + WcagTimingFactory
        + WcagTargetFactory
        + WcagStructureFactory
        + WcagMediaFactory
        + WcagLanguageFactory
        + WcagErrorFactory
        + WcagPerceivedFactory
        + WcagOperableFactory
        + WcagUnderstandableFactory
        + WcagRobustFactory
        + WcagElementMeta
        + WcagPageMeta
        + Send
        + Sync,
{
    fn build_level_aa(&self, evidence: LevelAaEvidence) -> Established<WcagLevelAAValid> {
        Established::prove(&evidence)
    }
}
