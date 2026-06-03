//! WCAG 2.2 proof-carrying contract propositions.
//!
//! Source: Web Content Accessibility Guidelines (WCAG) 2.2
//! W3C Recommendation 05 October 2023.
//! <https://www.w3.org/TR/WCAG22/>
//!
//! One proposition (proof token) per Success Criterion, plus sub-invariant
//! propositions for criteria that decompose into independently verifiable
//! numeric or structural conditions.
//!
//! # Three-tier taxonomy
//!
//! | Token kind | Naming pattern | Notes |
//! |---|---|---|
//! | SC-level aggregate | `Wcag<Description>` | One per criterion |
//! | Sub-invariant | `Wcag<Criterion><SubCondition>` | Multiple per criterion |
//! | Level aggregate | `WcagLevel{A,AA,AAA}` | Composite conformance seams |
//! | Legacy aliases | camelCase short forms | Kept for API back-compat |

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    macro_rules! structural_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by WCAG 2.2 contract */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by WCAG 2.2 contract */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by WCAG 2.2 contract */ }
                }
            }
        };
    }

    // ── Principle 1: Perceivable ─────────────────────────────────────────────

    // §1.1 Text Alternatives ─────────────────────────────────────────────────

    /// Every non-text content element carries a text alternative that serves
    /// an equivalent purpose.
    ///
    /// Source: WCAG 2.2 SC 1.1.1 — Non-text Content (Level A).
    pub struct WcagNonTextContentAltPresent;
    structural_prop!(WcagNonTextContentAltPresent, "WcagNonTextContentAltPresent");

    /// Text alternative for a non-text element is non-empty (not just
    /// whitespace).
    ///
    /// Source: WCAG 2.2 SC 1.1.1 — Non-text Content (Level A).
    pub struct WcagNonTextContentAltNonEmpty;
    structural_prop!(
        WcagNonTextContentAltNonEmpty,
        "WcagNonTextContentAltNonEmpty"
    );

    /// Text alternative adequately describes the purpose or content of the
    /// non-text element.
    ///
    /// Source: WCAG 2.2 SC 1.1.1 — Non-text Content (Level A).
    pub struct WcagNonTextContentAltDescriptive;
    structural_prop!(
        WcagNonTextContentAltDescriptive,
        "WcagNonTextContentAltDescriptive"
    );

    /// Purely decorative image carries an empty (`""`) alt attribute so
    /// assistive technology skips it.
    ///
    /// Source: WCAG 2.2 SC 1.1.1 — Non-text Content (Level A).
    pub struct WcagDecorativeImageAltEmpty;
    structural_prop!(WcagDecorativeImageAltEmpty, "WcagDecorativeImageAltEmpty");

    /// CAPTCHA provides at least two sensory modalities (visual + audio, etc.)
    /// so users with sensory disabilities can still pass.
    ///
    /// Source: WCAG 2.2 SC 1.1.1 — Non-text Content, CAPTCHA clause (Level A).
    pub struct WcagCaptchaMultipleModalities;
    structural_prop!(
        WcagCaptchaMultipleModalities,
        "WcagCaptchaMultipleModalities"
    );

    // §1.2 Time-based Media ───────────────────────────────────────────────────

    /// Audio-only prerecorded content has a text transcript or equivalent.
    ///
    /// Source: WCAG 2.2 SC 1.2.1 — Audio-only and Video-only (Prerecorded) (Level A).
    pub struct WcagAudioOnlyAlternativeProvided;
    structural_prop!(
        WcagAudioOnlyAlternativeProvided,
        "WcagAudioOnlyAlternativeProvided"
    );

    /// Video-only prerecorded content has an audio track or text equivalent.
    ///
    /// Source: WCAG 2.2 SC 1.2.1 — Audio-only and Video-only (Prerecorded) (Level A).
    pub struct WcagVideoOnlyAlternativeProvided;
    structural_prop!(
        WcagVideoOnlyAlternativeProvided,
        "WcagVideoOnlyAlternativeProvided"
    );

    /// Captions for prerecorded audio in synchronized media are present and
    /// synchronized to the audio track.
    ///
    /// Source: WCAG 2.2 SC 1.2.2 — Captions (Prerecorded) (Level A).
    pub struct WcagCaptionsSynchronized;
    structural_prop!(WcagCaptionsSynchronized, "WcagCaptionsSynchronized");

    /// Prerecorded synchronized media provides either an audio description
    /// or a full media alternative (transcript + description).
    ///
    /// Source: WCAG 2.2 SC 1.2.3 — Audio Description or Media Alternative (Level A).
    pub struct WcagAudioDescriptionOrMediaAlt;
    structural_prop!(
        WcagAudioDescriptionOrMediaAlt,
        "WcagAudioDescriptionOrMediaAlt"
    );

    /// Live synchronized media provides real-time captions.
    ///
    /// Source: WCAG 2.2 SC 1.2.4 — Captions (Live) (Level AA).
    pub struct WcagCaptionsLiveProvided;
    structural_prop!(WcagCaptionsLiveProvided, "WcagCaptionsLiveProvided");

    /// Prerecorded video in synchronized media includes an audio description
    /// track for visual-only information.
    ///
    /// Source: WCAG 2.2 SC 1.2.5 — Audio Description (Prerecorded) (Level AA).
    pub struct WcagAudioDescriptionPrerecorded;
    structural_prop!(
        WcagAudioDescriptionPrerecorded,
        "WcagAudioDescriptionPrerecorded"
    );

    /// Prerecorded synchronized media provides a sign language interpretation.
    ///
    /// Source: WCAG 2.2 SC 1.2.6 — Sign Language (Prerecorded) (Level AAA).
    pub struct WcagSignLanguagePrerecorded;
    structural_prop!(WcagSignLanguagePrerecorded, "WcagSignLanguagePrerecorded");

    /// When standard audio description cannot cover all pauses, an extended
    /// audio description is provided.
    ///
    /// Source: WCAG 2.2 SC 1.2.7 — Extended Audio Description (Prerecorded) (Level AAA).
    pub struct WcagExtendedAudioDescription;
    structural_prop!(WcagExtendedAudioDescription, "WcagExtendedAudioDescription");

    /// Prerecorded synchronized media provides a full text media alternative
    /// for both audio and visual content.
    ///
    /// Source: WCAG 2.2 SC 1.2.8 — Media Alternative (Prerecorded) (Level AAA).
    pub struct WcagMediaAlternativePrerecorded;
    structural_prop!(
        WcagMediaAlternativePrerecorded,
        "WcagMediaAlternativePrerecorded"
    );

    /// Live audio-only content provides a real-time text alternative.
    ///
    /// Source: WCAG 2.2 SC 1.2.9 — Audio-only (Live) (Level AAA).
    pub struct WcagAudioOnlyLiveAlternative;
    structural_prop!(WcagAudioOnlyLiveAlternative, "WcagAudioOnlyLiveAlternative");

    // §1.3 Adaptable ──────────────────────────────────────────────────────────

    /// Information, structure, and relationships conveyed visually are
    /// programmatically determinable.
    ///
    /// Source: WCAG 2.2 SC 1.3.1 — Info and Relationships (Level A).
    pub struct WcagInfoAndRelationshipsProgrammatic;
    structural_prop!(
        WcagInfoAndRelationshipsProgrammatic,
        "WcagInfoAndRelationshipsProgrammatic"
    );

    /// Headings are marked with programmatic heading roles or semantics.
    ///
    /// Source: WCAG 2.2 SC 1.3.1 — Info and Relationships (Level A).
    pub struct WcagHeadingStructureProgrammatic;
    structural_prop!(
        WcagHeadingStructureProgrammatic,
        "WcagHeadingStructureProgrammatic"
    );

    /// List structure is programmatically encoded (not just visual indentation).
    ///
    /// Source: WCAG 2.2 SC 1.3.1 — Info and Relationships (Level A).
    pub struct WcagListStructureProgrammatic;
    structural_prop!(
        WcagListStructureProgrammatic,
        "WcagListStructureProgrammatic"
    );

    /// Table headers are programmatically associated with their data cells.
    ///
    /// Source: WCAG 2.2 SC 1.3.1 — Info and Relationships (Level A).
    pub struct WcagTableHeadersProgrammatic;
    structural_prop!(WcagTableHeadersProgrammatic, "WcagTableHeadersProgrammatic");

    /// Form labels are programmatically associated with their controls.
    ///
    /// Source: WCAG 2.2 SC 1.3.1 — Info and Relationships (Level A).
    pub struct WcagFormLabelsProgrammatic;
    structural_prop!(WcagFormLabelsProgrammatic, "WcagFormLabelsProgrammatic");

    /// Content reading and operation order is determinable from structure
    /// alone (not layout or presentation).
    ///
    /// Source: WCAG 2.2 SC 1.3.2 — Meaningful Sequence (Level A).
    pub struct WcagMeaningfulSequencePreservable;
    structural_prop!(
        WcagMeaningfulSequencePreservable,
        "WcagMeaningfulSequencePreservable"
    );

    /// Instructions do not rely solely on sensory characteristics such as shape,
    /// color, size, visual location, orientation, or sound.
    ///
    /// Source: WCAG 2.2 SC 1.3.3 — Sensory Characteristics (Level A).
    pub struct WcagSensoryNotExclusive;
    structural_prop!(WcagSensoryNotExclusive, "WcagSensoryNotExclusive");

    /// Content does not lock display orientation to portrait or landscape.
    ///
    /// Source: WCAG 2.2 SC 1.3.4 — Orientation (Level AA).
    pub struct WcagOrientationNotRestricted;
    structural_prop!(WcagOrientationNotRestricted, "WcagOrientationNotRestricted");

    /// The purpose of each input field collecting user information can be
    /// programmatically determined (autocomplete).
    ///
    /// Source: WCAG 2.2 SC 1.3.5 — Identify Input Purpose (Level AA).
    pub struct WcagInputPurposeIdentifiable;
    structural_prop!(WcagInputPurposeIdentifiable, "WcagInputPurposeIdentifiable");

    /// The purpose of UI components, icons, and regions can be determined by
    /// assistive technology.
    ///
    /// Source: WCAG 2.2 SC 1.3.6 — Identify Purpose (Level AAA).
    pub struct WcagComponentPurposeIdentifiable;
    structural_prop!(
        WcagComponentPurposeIdentifiable,
        "WcagComponentPurposeIdentifiable"
    );

    // §1.4 Distinguishable ────────────────────────────────────────────────────

    /// Color is not the only visual means of conveying information, indicating
    /// an action, prompting a response, or distinguishing an element.
    ///
    /// Source: WCAG 2.2 SC 1.4.1 — Use of Color (Level A).
    pub struct WcagColorNotSoleConveyor;
    structural_prop!(WcagColorNotSoleConveyor, "WcagColorNotSoleConveyor");

    /// A mechanism is available to pause, stop, or adjust audio that plays
    /// automatically for more than 3 seconds.
    ///
    /// Source: WCAG 2.2 SC 1.4.2 — Audio Control (Level A).
    pub struct WcagAudioControlAvailable;
    structural_prop!(WcagAudioControlAvailable, "WcagAudioControlAvailable");

    /// Normal-size text (below 18 pt or 14 pt bold) meets a 4.5:1 contrast
    /// ratio against its background.
    ///
    /// Source: WCAG 2.2 SC 1.4.3 — Contrast (Minimum) (Level AA).
    pub struct WcagContrastMinimumNormalText;
    structural_prop!(
        WcagContrastMinimumNormalText,
        "WcagContrastMinimumNormalText"
    );

    /// Large text (18 pt+ or 14 pt+ bold) meets a 3:1 contrast ratio.
    ///
    /// Source: WCAG 2.2 SC 1.4.3 — Contrast (Minimum) (Level AA).
    pub struct WcagContrastMinimumLargeText;
    structural_prop!(WcagContrastMinimumLargeText, "WcagContrastMinimumLargeText");

    /// Logotype text or text in inactive UI components has no contrast
    /// requirement enforced.
    ///
    /// Source: WCAG 2.2 SC 1.4.3 — Contrast (Minimum), exception clause (Level AA).
    pub struct WcagContrastMinimumLogotypeExcepted;
    structural_prop!(
        WcagContrastMinimumLogotypeExcepted,
        "WcagContrastMinimumLogotypeExcepted"
    );

    /// Text can be resized up to 200% without loss of content or functionality
    /// (no assistive technology required).
    ///
    /// Source: WCAG 2.2 SC 1.4.4 — Resize Text (Level AA).
    pub struct WcagTextResizable;
    structural_prop!(WcagTextResizable, "WcagTextResizable");

    /// Images of text are avoided in favor of real text.
    ///
    /// Source: WCAG 2.2 SC 1.4.5 — Images of Text (Level AA).
    pub struct WcagImagesOfTextAvoided;
    structural_prop!(WcagImagesOfTextAvoided, "WcagImagesOfTextAvoided");

    /// Where images of text are used, they can be visually customized to
    /// the user's requirements.
    ///
    /// Source: WCAG 2.2 SC 1.4.5 — Images of Text (Level AA).
    pub struct WcagImagesOfTextCustomizable;
    structural_prop!(WcagImagesOfTextCustomizable, "WcagImagesOfTextCustomizable");

    /// Normal-size text meets a 7:1 enhanced contrast ratio.
    ///
    /// Source: WCAG 2.2 SC 1.4.6 — Contrast (Enhanced) (Level AAA).
    pub struct WcagContrastEnhancedNormalText;
    structural_prop!(
        WcagContrastEnhancedNormalText,
        "WcagContrastEnhancedNormalText"
    );

    /// Large text meets a 4.5:1 enhanced contrast ratio.
    ///
    /// Source: WCAG 2.2 SC 1.4.6 — Contrast (Enhanced) (Level AAA).
    pub struct WcagContrastEnhancedLargeText;
    structural_prop!(
        WcagContrastEnhancedLargeText,
        "WcagContrastEnhancedLargeText"
    );

    /// Background audio is at least 20 dB lower than foreground speech, or
    /// can be turned off.
    ///
    /// Source: WCAG 2.2 SC 1.4.7 — Low or No Background Audio (Level AAA).
    pub struct WcagLowBackgroundAudio;
    structural_prop!(WcagLowBackgroundAudio, "WcagLowBackgroundAudio");

    /// Visual presentation of text blocks is fully user-customizable
    /// (colors, width, alignment, spacing).
    ///
    /// Source: WCAG 2.2 SC 1.4.8 — Visual Presentation (Level AAA).
    pub struct WcagVisualPresentationCustomizable;
    structural_prop!(
        WcagVisualPresentationCustomizable,
        "WcagVisualPresentationCustomizable"
    );

    /// Images of text are not used except where essential (e.g., logotype).
    ///
    /// Source: WCAG 2.2 SC 1.4.9 — Images of Text (No Exception) (Level AAA).
    pub struct WcagImagesOfTextNoException;
    structural_prop!(WcagImagesOfTextNoException, "WcagImagesOfTextNoException");

    /// Content can reflow into a single column at 320 CSS px without losing
    /// information or requiring two-dimensional scrolling.
    ///
    /// Source: WCAG 2.2 SC 1.4.10 — Reflow (Level AA).
    pub struct WcagContentReflowable;
    structural_prop!(WcagContentReflowable, "WcagContentReflowable");

    /// Vertical-text content does not require horizontal scrolling at 256 CSS px.
    ///
    /// Source: WCAG 2.2 SC 1.4.10 — Reflow (Level AA).
    pub struct WcagNoHorizontalScrollVerticalText;
    structural_prop!(
        WcagNoHorizontalScrollVerticalText,
        "WcagNoHorizontalScrollVerticalText"
    );

    /// Text qualifies as "large" under WCAG definitions:
    /// at least 18 pt at any weight, or at least 14 pt when bold (≥700 weight).
    ///
    /// Obtaining this token through [`WcagContrastFactory::classify_large_text`](crate::WcagContrastFactory::classify_large_text)
    /// is required before calling the large-text contrast threshold methods, ensuring
    /// the 3:1 / 4.5:1 distinction is validated by the type system rather than
    /// asserted by the caller.
    ///
    /// Source: WCAG 2.2 SC 1.4.3 — Contrast (Minimum), large-text definition.
    pub struct WcagLargeTextClassified;
    structural_prop!(WcagLargeTextClassified, "WcagLargeTextClassified");

    /// User interface components and graphical objects have at least 3:1
    /// contrast against adjacent colors.
    ///
    /// Source: WCAG 2.2 SC 1.4.11 — Non-text Contrast (Level AA).
    pub struct WcagNonTextContrastMinimum;
    structural_prop!(WcagNonTextContrastMinimum, "WcagNonTextContrastMinimum");

    /// The keyboard focus indicator meets the 3:1 contrast requirement.
    ///
    /// Source: WCAG 2.2 SC 1.4.11 — Non-text Contrast (Level AA).
    pub struct WcagFocusIndicatorContrast;
    structural_prop!(WcagFocusIndicatorContrast, "WcagFocusIndicatorContrast");

    /// Content does not lose information when all of the following are applied:
    /// line height ≥1.5, letter spacing ≥0.12 em, word spacing ≥0.16 em,
    /// paragraph spacing ≥2 em.
    ///
    /// Source: WCAG 2.2 SC 1.4.12 — Text Spacing (Level AA).
    pub struct WcagTextSpacingAdjustable;
    structural_prop!(WcagTextSpacingAdjustable, "WcagTextSpacingAdjustable");

    /// Line height is adjustable to at least 1.5× the font size without loss.
    ///
    /// Source: WCAG 2.2 SC 1.4.12 — Text Spacing (Level AA).
    pub struct WcagLineHeightAdjustable;
    structural_prop!(WcagLineHeightAdjustable, "WcagLineHeightAdjustable");

    /// Letter spacing is adjustable to at least 0.12 em without loss.
    ///
    /// Source: WCAG 2.2 SC 1.4.12 — Text Spacing (Level AA).
    pub struct WcagLetterSpacingAdjustable;
    structural_prop!(WcagLetterSpacingAdjustable, "WcagLetterSpacingAdjustable");

    /// Word spacing is adjustable to at least 0.16 em without loss.
    ///
    /// Source: WCAG 2.2 SC 1.4.12 — Text Spacing (Level AA).
    pub struct WcagWordSpacingAdjustable;
    structural_prop!(WcagWordSpacingAdjustable, "WcagWordSpacingAdjustable");

    /// Paragraph spacing is adjustable to at least 2 em without loss.
    ///
    /// Source: WCAG 2.2 SC 1.4.12 — Text Spacing (Level AA).
    pub struct WcagParagraphSpacingAdjustable;
    structural_prop!(
        WcagParagraphSpacingAdjustable,
        "WcagParagraphSpacingAdjustable"
    );

    /// Hover- or focus-triggered additional content can be dismissed without
    /// moving the pointer or keyboard focus.
    ///
    /// Source: WCAG 2.2 SC 1.4.13 — Content on Hover or Focus (Level AA).
    pub struct WcagHoverContentDismissible;
    structural_prop!(WcagHoverContentDismissible, "WcagHoverContentDismissible");

    /// If a trigger is hovered, the pointer can be moved to the additional
    /// content without it disappearing.
    ///
    /// Source: WCAG 2.2 SC 1.4.13 — Content on Hover or Focus (Level AA).
    pub struct WcagHoverContentHoverable;
    structural_prop!(WcagHoverContentHoverable, "WcagHoverContentHoverable");

    /// Hover- or focus-triggered additional content remains visible until the
    /// trigger loses hover/focus or the user dismisses it.
    ///
    /// Source: WCAG 2.2 SC 1.4.13 — Content on Hover or Focus (Level AA).
    pub struct WcagHoverContentPersistent;
    structural_prop!(WcagHoverContentPersistent, "WcagHoverContentPersistent");

    // ── Principle 2: Operable ────────────────────────────────────────────────

    // §2.1 Keyboard Accessible ────────────────────────────────────────────────

    /// All functionality is operable through a keyboard interface without
    /// requiring specific timing for keystrokes.
    ///
    /// Source: WCAG 2.2 SC 2.1.1 — Keyboard (Level A).
    pub struct WcagKeyboardOperable;
    structural_prop!(WcagKeyboardOperable, "WcagKeyboardOperable");

    /// No keyboard path requires a particular timing between keystrokes.
    ///
    /// Source: WCAG 2.2 SC 2.1.1 — Keyboard (Level A).
    pub struct WcagKeyboardNoTimingPath;
    structural_prop!(WcagKeyboardNoTimingPath, "WcagKeyboardNoTimingPath");

    /// Keyboard focus is never trapped; the user can navigate away from any
    /// component using standard keys.
    ///
    /// Source: WCAG 2.2 SC 2.1.2 — No Keyboard Trap (Level A).
    pub struct WcagKeyboardNotTrapped;
    structural_prop!(WcagKeyboardNotTrapped, "WcagKeyboardNotTrapped");

    /// A standard key (Escape, Tab, etc.) moves focus out of the component.
    ///
    /// Source: WCAG 2.2 SC 2.1.2 — No Keyboard Trap (Level A).
    pub struct WcagKeyboardEscapeFromComponent;
    structural_prop!(
        WcagKeyboardEscapeFromComponent,
        "WcagKeyboardEscapeFromComponent"
    );

    /// All functionality is keyboard accessible with no exception.
    ///
    /// Source: WCAG 2.2 SC 2.1.3 — Keyboard (No Exception) (Level AAA).
    pub struct WcagAllFunctionalityKeyboard;
    structural_prop!(WcagAllFunctionalityKeyboard, "WcagAllFunctionalityKeyboard");

    /// Single-character key shortcuts can be remapped or disabled.
    ///
    /// Source: WCAG 2.2 SC 2.1.4 — Character Key Shortcuts (Level A).
    pub struct WcagCharacterShortcutsRemappable;
    structural_prop!(
        WcagCharacterShortcutsRemappable,
        "WcagCharacterShortcutsRemappable"
    );

    /// Single-character key shortcuts can be entirely turned off.
    ///
    /// Source: WCAG 2.2 SC 2.1.4 — Character Key Shortcuts (Level A).
    pub struct WcagCharacterShortcutsDisableable;
    structural_prop!(
        WcagCharacterShortcutsDisableable,
        "WcagCharacterShortcutsDisableable"
    );

    /// A mechanism exists to activate shortcut keys only when the component
    /// has keyboard focus.
    ///
    /// Source: WCAG 2.2 SC 2.1.4 — Character Key Shortcuts (Level A).
    pub struct WcagCharacterShortcutsFocusOnly;
    structural_prop!(
        WcagCharacterShortcutsFocusOnly,
        "WcagCharacterShortcutsFocusOnly"
    );

    // §2.2 Enough Time ────────────────────────────────────────────────────────

    /// Each time limit can be turned off, adjusted, or extended before it
    /// expires (with real-time and 20-hour exceptions).
    ///
    /// Source: WCAG 2.2 SC 2.2.1 — Timing Adjustable (Level A).
    pub struct WcagTimingAdjustable;
    structural_prop!(WcagTimingAdjustable, "WcagTimingAdjustable");

    /// A mechanism is provided to turn off the time limit entirely.
    ///
    /// Source: WCAG 2.2 SC 2.2.1 — Timing Adjustable (Level A).
    pub struct WcagTimingTurnOffAvailable;
    structural_prop!(WcagTimingTurnOffAvailable, "WcagTimingTurnOffAvailable");

    /// A mechanism is provided to adjust (extend) the time limit by at least
    /// 10× the default duration.
    ///
    /// Source: WCAG 2.2 SC 2.2.1 — Timing Adjustable (Level A).
    pub struct WcagTimingAdjustTenX;
    structural_prop!(WcagTimingAdjustTenX, "WcagTimingAdjustTenX");

    /// When a time limit is about to expire, the user is warned at least
    /// 20 seconds in advance and given a simple way to extend it.
    ///
    /// Source: WCAG 2.2 SC 2.2.1 — Timing Adjustable (Level A).
    pub struct WcagTimingExtendWarning;
    structural_prop!(WcagTimingExtendWarning, "WcagTimingExtendWarning");

    /// Moving, blinking, scrolling, or auto-updating content that lasts more
    /// than 5 seconds can be paused, stopped, or hidden.
    ///
    /// Source: WCAG 2.2 SC 2.2.2 — Pause, Stop, Hide (Level A).
    pub struct WcagPauseStopHideAvailable;
    structural_prop!(WcagPauseStopHideAvailable, "WcagPauseStopHideAvailable");

    /// Auto-updating content can be paused so the user can read it.
    ///
    /// Source: WCAG 2.2 SC 2.2.2 — Pause, Stop, Hide (Level A).
    pub struct WcagAutoUpdatePausable;
    structural_prop!(WcagAutoUpdatePausable, "WcagAutoUpdatePausable");

    /// No time limits are imposed, except for real-time events and where
    /// a 20-hour minimum cannot be achieved.
    ///
    /// Source: WCAG 2.2 SC 2.2.3 — No Timing (Level AAA).
    pub struct WcagNoTimingRequired;
    structural_prop!(WcagNoTimingRequired, "WcagNoTimingRequired");

    /// Interruptions can be postponed or suppressed (except for emergencies).
    ///
    /// Source: WCAG 2.2 SC 2.2.4 — Interruptions (Level AAA).
    pub struct WcagInterruptionsPostponable;
    structural_prop!(WcagInterruptionsPostponable, "WcagInterruptionsPostponable");

    /// After re-authentication following a session timeout, data already
    /// entered is preserved.
    ///
    /// Source: WCAG 2.2 SC 2.2.5 — Re-authenticating (Level AAA).
    pub struct WcagReauthWithoutDataLoss;
    structural_prop!(WcagReauthWithoutDataLoss, "WcagReauthWithoutDataLoss");

    /// Users are warned about inactivity timeouts that could cause data loss
    /// (minimum 20-hour threshold).
    ///
    /// Source: WCAG 2.2 SC 2.2.6 — Timeouts (Level AAA).
    pub struct WcagTimeoutWarningProvided;
    structural_prop!(WcagTimeoutWarningProvided, "WcagTimeoutWarningProvided");

    // §2.3 Seizures and Physical Reactions ────────────────────────────────────

    /// Content does not flash more than three times per second, or the flash
    /// is below the general and red flash thresholds.
    ///
    /// Source: WCAG 2.2 SC 2.3.1 — Three Flashes or Below Threshold (Level A).
    pub struct WcagThreeFlashBelowThreshold;
    structural_prop!(WcagThreeFlashBelowThreshold, "WcagThreeFlashBelowThreshold");

    /// No single flash occupies more than 25% of the viewport (1024×768 or
    /// 10-degree visual field threshold).
    ///
    /// Source: WCAG 2.2 SC 2.3.1 — Three Flashes or Below Threshold (Level A).
    pub struct WcagFlashAreaBelowThreshold;
    structural_prop!(WcagFlashAreaBelowThreshold, "WcagFlashAreaBelowThreshold");

    /// No content flashes more than three times per second under any
    /// circumstances.
    ///
    /// Source: WCAG 2.2 SC 2.3.2 — Three Flashes (Level AAA).
    pub struct WcagNoThreeFlashAbsolute;
    structural_prop!(WcagNoThreeFlashAbsolute, "WcagNoThreeFlashAbsolute");

    /// Animation triggered by interaction can be disabled via a
    /// `prefers-reduced-motion` media query or equivalent control.
    ///
    /// Source: WCAG 2.2 SC 2.3.3 — Animation from Interactions (Level AAA).
    pub struct WcagReducedMotionRespected;
    structural_prop!(WcagReducedMotionRespected, "WcagReducedMotionRespected");

    // §2.4 Navigable ──────────────────────────────────────────────────────────

    /// A skip-to-main-content mechanism or landmark structure allows users to
    /// bypass repeated navigation blocks.
    ///
    /// Source: WCAG 2.2 SC 2.4.1 — Bypass Blocks (Level A).
    pub struct WcagBypassBlocksMechanism;
    structural_prop!(WcagBypassBlocksMechanism, "WcagBypassBlocksMechanism");

    /// Each page or view has a descriptive title that identifies its topic or
    /// purpose.
    ///
    /// Source: WCAG 2.2 SC 2.4.2 — Page Titled (Level A).
    pub struct WcagPageTitled;
    structural_prop!(WcagPageTitled, "WcagPageTitled");

    /// The page title is unique within the set of pages, or contextually
    /// distinguishable.
    ///
    /// Source: WCAG 2.2 SC 2.4.2 — Page Titled (Level A).
    pub struct WcagPageTitleDescriptive;
    structural_prop!(WcagPageTitleDescriptive, "WcagPageTitleDescriptive");

    /// If a Web page can be navigated sequentially and navigation sequences
    /// affect meaning or operation, the focus order preserves meaning.
    ///
    /// Source: WCAG 2.2 SC 2.4.3 — Focus Order (Level A).
    pub struct WcagFocusOrderLogical;
    structural_prop!(WcagFocusOrderLogical, "WcagFocusOrderLogical");

    /// The purpose of each link can be determined from its link text alone
    /// or from surrounding context.
    ///
    /// Source: WCAG 2.2 SC 2.4.4 — Link Purpose (In Context) (Level A).
    pub struct WcagLinkPurposeFromContext;
    structural_prop!(WcagLinkPurposeFromContext, "WcagLinkPurposeFromContext");

    /// More than one way to locate a web page within a set exists (site map,
    /// search, related links, etc.).
    ///
    /// Source: WCAG 2.2 SC 2.4.5 — Multiple Ways (Level AA).
    pub struct WcagMultiplePathsToContent;
    structural_prop!(WcagMultiplePathsToContent, "WcagMultiplePathsToContent");

    /// Headings and labels describe their topic or purpose.
    ///
    /// Source: WCAG 2.2 SC 2.4.6 — Headings and Labels (Level AA).
    pub struct WcagHeadingsDescriptive;
    structural_prop!(WcagHeadingsDescriptive, "WcagHeadingsDescriptive");

    /// Labels describe the purpose of the UI control they label.
    ///
    /// Source: WCAG 2.2 SC 2.4.6 — Headings and Labels (Level AA).
    pub struct WcagLabelsDescriptive;
    structural_prop!(WcagLabelsDescriptive, "WcagLabelsDescriptive");

    /// Any keyboard-operable user interface has a visible keyboard focus
    /// indicator.
    ///
    /// Source: WCAG 2.2 SC 2.4.7 — Focus Visible (Level AA).
    pub struct WcagFocusVisibleKeyboard;
    structural_prop!(WcagFocusVisibleKeyboard, "WcagFocusVisibleKeyboard");

    /// Information is available about the user's current location within a
    /// set of pages (breadcrumbs, site map highlight, etc.).
    ///
    /// Source: WCAG 2.2 SC 2.4.8 — Location (Level AAA).
    pub struct WcagLocationInNavigationSet;
    structural_prop!(WcagLocationInNavigationSet, "WcagLocationInNavigationSet");

    /// The purpose of each link can be determined from the link text alone
    /// (without context).
    ///
    /// Source: WCAG 2.2 SC 2.4.9 — Link Purpose (Link Only) (Level AAA).
    pub struct WcagLinkPurposeLinkOnly;
    structural_prop!(WcagLinkPurposeLinkOnly, "WcagLinkPurposeLinkOnly");

    /// Section headings are used to organize content.
    ///
    /// Source: WCAG 2.2 SC 2.4.10 — Section Headings (Level AAA).
    pub struct WcagSectionHeadingsPresent;
    structural_prop!(WcagSectionHeadingsPresent, "WcagSectionHeadingsPresent");

    /// The keyboard focus indicator has an area of at least the perimeter of
    /// the unfocused component × 2 CSS pixels.
    ///
    /// Source: WCAG 2.2 SC 2.4.11 — Focus Appearance (Level AA).
    pub struct WcagFocusAppearanceMinimumArea;
    structural_prop!(
        WcagFocusAppearanceMinimumArea,
        "WcagFocusAppearanceMinimumArea"
    );

    /// The focus indicator meets at least 3:1 contrast ratio between focused
    /// and unfocused states.
    ///
    /// Source: WCAG 2.2 SC 2.4.11 — Focus Appearance (Level AA).
    pub struct WcagFocusAppearanceMinimumContrast;
    structural_prop!(
        WcagFocusAppearanceMinimumContrast,
        "WcagFocusAppearanceMinimumContrast"
    );

    /// The focus indicator encloses the entire focused component and has no
    /// area less than a 2px offset from the component.
    ///
    /// Source: WCAG 2.2 SC 2.4.12 — Focus Appearance (Enhanced) (Level AAA).
    pub struct WcagFocusAppearanceEnhancedArea;
    structural_prop!(
        WcagFocusAppearanceEnhancedArea,
        "WcagFocusAppearanceEnhancedArea"
    );

    /// The enhanced focus indicator meets at least 4.5:1 contrast ratio.
    ///
    /// Source: WCAG 2.2 SC 2.4.12 — Focus Appearance (Enhanced) (Level AAA).
    pub struct WcagFocusAppearanceEnhancedContrast;
    structural_prop!(
        WcagFocusAppearanceEnhancedContrast,
        "WcagFocusAppearanceEnhancedContrast"
    );

    // §2.5 Input Modalities ───────────────────────────────────────────────────

    /// All functionality that uses multi-point or path-based gestures has a
    /// single-pointer alternative without a path gesture.
    ///
    /// Source: WCAG 2.2 SC 2.5.1 — Pointer Gestures (Level A).
    pub struct WcagPointerGesturesSimpleAlternative;
    structural_prop!(
        WcagPointerGesturesSimpleAlternative,
        "WcagPointerGesturesSimpleAlternative"
    );

    /// The down-event of a pointer is not used to execute functionality
    /// (or the action is abortable / reversible).
    ///
    /// Source: WCAG 2.2 SC 2.5.2 — Pointer Cancellation (Level A).
    pub struct WcagPointerCancellationUpEvent;
    structural_prop!(
        WcagPointerCancellationUpEvent,
        "WcagPointerCancellationUpEvent"
    );

    /// Actions triggered on down-event can be aborted by moving the pointer
    /// off the target before releasing.
    ///
    /// Source: WCAG 2.2 SC 2.5.2 — Pointer Cancellation (Level A).
    pub struct WcagPointerCancellationAbortable;
    structural_prop!(
        WcagPointerCancellationAbortable,
        "WcagPointerCancellationAbortable"
    );

    /// Actions triggered on down-event can be reversed after completion.
    ///
    /// Source: WCAG 2.2 SC 2.5.2 — Pointer Cancellation (Level A).
    pub struct WcagPointerCancellationReversible;
    structural_prop!(
        WcagPointerCancellationReversible,
        "WcagPointerCancellationReversible"
    );

    /// The accessible name of a UI component contains the visible label text
    /// (case-insensitive, punctuation-independent).
    ///
    /// Source: WCAG 2.2 SC 2.5.3 — Label in Name (Level A).
    pub struct WcagLabelInNameMatch;
    structural_prop!(WcagLabelInNameMatch, "WcagLabelInNameMatch");

    /// Functionality that can be triggered by device motion (tilt, shake, etc.)
    /// also has a conventional UI control alternative.
    ///
    /// Source: WCAG 2.2 SC 2.5.4 — Motion Actuation (Level A).
    pub struct WcagMotionActuationAlternative;
    structural_prop!(
        WcagMotionActuationAlternative,
        "WcagMotionActuationAlternative"
    );

    /// The device motion trigger can be disabled to prevent accidental
    /// activation.
    ///
    /// Source: WCAG 2.2 SC 2.5.4 — Motion Actuation (Level A).
    pub struct WcagMotionActuationDisableable;
    structural_prop!(
        WcagMotionActuationDisableable,
        "WcagMotionActuationDisableable"
    );

    /// Interactive target is at least 44 × 44 CSS pixels.
    ///
    /// Source: WCAG 2.2 SC 2.5.5 — Target Size (Enhanced) (Level AAA).
    pub struct WcagTargetSizeEnhanced;
    structural_prop!(WcagTargetSizeEnhanced, "WcagTargetSizeEnhanced");

    /// Target width is at least 44 CSS pixels.
    ///
    /// Source: WCAG 2.2 SC 2.5.5 — Target Size (Enhanced) (Level AAA).
    pub struct WcagTargetSizeEnhancedWidth;
    structural_prop!(WcagTargetSizeEnhancedWidth, "WcagTargetSizeEnhancedWidth");

    /// Target height is at least 44 CSS pixels.
    ///
    /// Source: WCAG 2.2 SC 2.5.5 — Target Size (Enhanced) (Level AAA).
    pub struct WcagTargetSizeEnhancedHeight;
    structural_prop!(WcagTargetSizeEnhancedHeight, "WcagTargetSizeEnhancedHeight");

    /// The platform does not restrict input mechanisms available to users
    /// (mouse, keyboard, touch, etc. concurrently).
    ///
    /// Source: WCAG 2.2 SC 2.5.6 — Concurrent Input Mechanisms (Level AAA).
    pub struct WcagConcurrentInputMechanisms;
    structural_prop!(
        WcagConcurrentInputMechanisms,
        "WcagConcurrentInputMechanisms"
    );

    /// Functionality that uses dragging movements has a single-pointer
    /// alternative that does not require dragging.
    ///
    /// Source: WCAG 2.2 SC 2.5.7 — Dragging Movements (Level AA).
    pub struct WcagDraggingAlternative;
    structural_prop!(WcagDraggingAlternative, "WcagDraggingAlternative");

    /// Interactive target is at least 24 × 24 CSS pixels, or has sufficient
    /// spacing so that a 24px offset circle does not intersect neighbors.
    ///
    /// Source: WCAG 2.2 SC 2.5.8 — Target Size (Minimum) (Level AA).
    pub struct WcagTargetSizeMinimum;
    structural_prop!(WcagTargetSizeMinimum, "WcagTargetSizeMinimum");

    /// Target width is at least 24 CSS pixels.
    ///
    /// Source: WCAG 2.2 SC 2.5.8 — Target Size (Minimum) (Level AA).
    pub struct WcagTargetSizeMinimumWidth;
    structural_prop!(WcagTargetSizeMinimumWidth, "WcagTargetSizeMinimumWidth");

    /// Target height is at least 24 CSS pixels.
    ///
    /// Source: WCAG 2.2 SC 2.5.8 — Target Size (Minimum) (Level AA).
    pub struct WcagTargetSizeMinimumHeight;
    structural_prop!(WcagTargetSizeMinimumHeight, "WcagTargetSizeMinimumHeight");

    /// Target spacing compensates for undersized targets so the offset circle
    /// does not intersect adjacent targets.
    ///
    /// Source: WCAG 2.2 SC 2.5.8 — Target Size (Minimum), spacing exception (Level AA).
    pub struct WcagTargetSizeMinimumSpaced;
    structural_prop!(WcagTargetSizeMinimumSpaced, "WcagTargetSizeMinimumSpaced");

    // ── Principle 3: Understandable ──────────────────────────────────────────

    // §3.1 Readable ───────────────────────────────────────────────────────────

    /// The default human language of the page is programmatically determined.
    ///
    /// Source: WCAG 2.2 SC 3.1.1 — Language of Page (Level A).
    pub struct WcagPageLanguageIdentified;
    structural_prop!(WcagPageLanguageIdentified, "WcagPageLanguageIdentified");

    /// Changes in natural language within content are programmatically
    /// identified (e.g., via `lang` attribute on passage).
    ///
    /// Source: WCAG 2.2 SC 3.1.2 — Language of Parts (Level AA).
    pub struct WcagPartLanguageIdentified;
    structural_prop!(WcagPartLanguageIdentified, "WcagPartLanguageIdentified");

    /// A mechanism is available to identify specific definitions of unusual
    /// words or phrases (jargon, idioms).
    ///
    /// Source: WCAG 2.2 SC 3.1.3 — Unusual Words (Level AAA).
    pub struct WcagUnusualWordsDefined;
    structural_prop!(WcagUnusualWordsDefined, "WcagUnusualWordsDefined");

    /// A mechanism for expanding abbreviations is available.
    ///
    /// Source: WCAG 2.2 SC 3.1.4 — Abbreviations (Level AAA).
    pub struct WcagAbbreviationsExpanded;
    structural_prop!(WcagAbbreviationsExpanded, "WcagAbbreviationsExpanded");

    /// When content requires reading ability more advanced than lower secondary
    /// education, supplemental content or a simpler version is available.
    ///
    /// Source: WCAG 2.2 SC 3.1.5 — Reading Level (Level AAA).
    pub struct WcagReadingLevelSupplemented;
    structural_prop!(WcagReadingLevelSupplemented, "WcagReadingLevelSupplemented");

    /// A mechanism is available for pronouncing words where meaning is
    /// ambiguous without pronunciation.
    ///
    /// Source: WCAG 2.2 SC 3.1.6 — Pronunciation (Level AAA).
    pub struct WcagPronunciationAvailable;
    structural_prop!(WcagPronunciationAvailable, "WcagPronunciationAvailable");

    // §3.2 Predictable ────────────────────────────────────────────────────────

    /// Receiving focus on a component does not initiate a context change.
    ///
    /// Source: WCAG 2.2 SC 3.2.1 — On Focus (Level A).
    pub struct WcagFocusNoContextChange;
    structural_prop!(WcagFocusNoContextChange, "WcagFocusNoContextChange");

    /// Changing a setting on a UI component does not automatically cause a
    /// context change unless the user is advised beforehand.
    ///
    /// Source: WCAG 2.2 SC 3.2.2 — On Input (Level A).
    pub struct WcagInputNoContextChange;
    structural_prop!(WcagInputNoContextChange, "WcagInputNoContextChange");

    /// Navigation mechanisms repeated across pages appear in the same relative
    /// order each time.
    ///
    /// Source: WCAG 2.2 SC 3.2.3 — Consistent Navigation (Level AA).
    pub struct WcagNavigationConsistent;
    structural_prop!(WcagNavigationConsistent, "WcagNavigationConsistent");

    /// Components that have the same functionality carry the same
    /// identification (label, name, or alternative) throughout the site.
    ///
    /// Source: WCAG 2.2 SC 3.2.4 — Consistent Identification (Level AA).
    pub struct WcagIdentificationConsistent;
    structural_prop!(WcagIdentificationConsistent, "WcagIdentificationConsistent");

    /// Context changes are initiated only by user request; no automatic
    /// changes are triggered by time or component state.
    ///
    /// Source: WCAG 2.2 SC 3.2.5 — Change on Request (Level AAA).
    pub struct WcagChangesOnRequest;
    structural_prop!(WcagChangesOnRequest, "WcagChangesOnRequest");

    /// Help mechanisms (contact info, help page, chat, etc.) are located
    /// consistently across pages.
    ///
    /// Source: WCAG 2.2 SC 3.2.6 — Consistent Help (Level AA).
    pub struct WcagConsistentHelpLocated;
    structural_prop!(WcagConsistentHelpLocated, "WcagConsistentHelpLocated");

    // §3.3 Input Assistance ───────────────────────────────────────────────────

    /// When an input error is detected, the item in error is identified and
    /// the error is described to the user in text.
    ///
    /// Source: WCAG 2.2 SC 3.3.1 — Error Identification (Level A).
    pub struct WcagErrorIdentificationDescriptive;
    structural_prop!(
        WcagErrorIdentificationDescriptive,
        "WcagErrorIdentificationDescriptive"
    );

    /// Labels or instructions are provided when content requires user input.
    ///
    /// Source: WCAG 2.2 SC 3.3.2 — Labels or Instructions (Level A).
    pub struct WcagLabelsOrInstructionsPresent;
    structural_prop!(
        WcagLabelsOrInstructionsPresent,
        "WcagLabelsOrInstructionsPresent"
    );

    /// Error correction suggestions are provided when an input error is
    /// detected and suggestions are known.
    ///
    /// Source: WCAG 2.2 SC 3.3.3 — Error Suggestion (Level AA).
    pub struct WcagErrorSuggestionProvided;
    structural_prop!(WcagErrorSuggestionProvided, "WcagErrorSuggestionProvided");

    /// Legal, financial, or data-change submissions are reversible, checked,
    /// or confirmed.
    ///
    /// Source: WCAG 2.2 SC 3.3.4 — Error Prevention (Legal, Financial, Data) (Level AA).
    pub struct WcagErrorPreventionLegal;
    structural_prop!(WcagErrorPreventionLegal, "WcagErrorPreventionLegal");

    /// Submissions can be reversed after the fact.
    ///
    /// Source: WCAG 2.2 SC 3.3.4 — Error Prevention (Level AA).
    pub struct WcagErrorPreventionReversible;
    structural_prop!(
        WcagErrorPreventionReversible,
        "WcagErrorPreventionReversible"
    );

    /// Input is checked for errors and the user can correct them before
    /// final submission.
    ///
    /// Source: WCAG 2.2 SC 3.3.4 — Error Prevention (Level AA).
    pub struct WcagErrorPreventionChecked;
    structural_prop!(WcagErrorPreventionChecked, "WcagErrorPreventionChecked");

    /// A confirmation step or mechanism is available before final submission.
    ///
    /// Source: WCAG 2.2 SC 3.3.4 — Error Prevention (Level AA).
    pub struct WcagErrorPreventionConfirmed;
    structural_prop!(WcagErrorPreventionConfirmed, "WcagErrorPreventionConfirmed");

    /// Context-sensitive help is available.
    ///
    /// Source: WCAG 2.2 SC 3.3.5 — Help (Level AAA).
    pub struct WcagContextSensitiveHelp;
    structural_prop!(WcagContextSensitiveHelp, "WcagContextSensitiveHelp");

    /// All submissions are reversible, checked, or confirmed.
    ///
    /// Source: WCAG 2.2 SC 3.3.6 — Error Prevention (All) (Level AAA).
    pub struct WcagErrorPreventionAll;
    structural_prop!(WcagErrorPreventionAll, "WcagErrorPreventionAll");

    /// Information previously entered in the current session is auto-populated
    /// or available for the user to select.
    ///
    /// Source: WCAG 2.2 SC 3.3.7 — Redundant Entry (Level A).
    pub struct WcagRedundantEntryMinimized;
    structural_prop!(WcagRedundantEntryMinimized, "WcagRedundantEntryMinimized");

    /// Authentication does not require a cognitive function test (unless an
    /// alternative, copy-paste, or assistance is provided).
    ///
    /// Source: WCAG 2.2 SC 3.3.8 — Accessible Authentication (Minimum) (Level AA).
    pub struct WcagAccessibleAuthentication;
    structural_prop!(WcagAccessibleAuthentication, "WcagAccessibleAuthentication");

    /// Authentication does not require a cognitive function test with no
    /// exceptions.
    ///
    /// Source: WCAG 2.2 SC 3.3.9 — Accessible Authentication (Enhanced) (Level AAA).
    pub struct WcagAccessibleAuthenticationEnhanced;
    structural_prop!(
        WcagAccessibleAuthenticationEnhanced,
        "WcagAccessibleAuthenticationEnhanced"
    );

    // ── Principle 4: Robust ──────────────────────────────────────────────────

    // §4.1 Compatible ─────────────────────────────────────────────────────────

    /// HTML parsing produces no duplicate IDs or unclosed elements (deprecated
    /// as an independent requirement in WCAG 2.2 but tracked for completeness).
    ///
    /// Source: WCAG 2.2 SC 4.1.1 — Parsing (Level A, deprecated in 2.2).
    pub struct WcagParsingValid;
    structural_prop!(WcagParsingValid, "WcagParsingValid");

    /// All user interface components have an accessible name, role, and
    /// state/value information programmatically exposed.
    ///
    /// Source: WCAG 2.2 SC 4.1.2 — Name, Role, Value (Level A).
    pub struct WcagNameRoleValueProgrammatic;
    structural_prop!(
        WcagNameRoleValueProgrammatic,
        "WcagNameRoleValueProgrammatic"
    );

    /// An accessible name is programmatically present for interactive elements.
    ///
    /// Source: WCAG 2.2 SC 4.1.2 — Name, Role, Value (Level A).
    pub struct WcagNamePresent;
    structural_prop!(WcagNamePresent, "WcagNamePresent");

    /// The role is programmatically determinable for every UI component.
    ///
    /// Source: WCAG 2.2 SC 4.1.2 — Name, Role, Value (Level A).
    pub struct WcagRoleProgrammatic;
    structural_prop!(WcagRoleProgrammatic, "WcagRoleProgrammatic");

    /// State and property information (checked, expanded, selected, etc.) is
    /// programmatically available for UI components.
    ///
    /// Source: WCAG 2.2 SC 4.1.2 — Name, Role, Value (Level A).
    pub struct WcagValueStatesProgrammatic;
    structural_prop!(WcagValueStatesProgrammatic, "WcagValueStatesProgrammatic");

    /// Status messages can be programmatically determined and do not require
    /// focus for assistive technologies to announce them.
    ///
    /// Source: WCAG 2.2 SC 4.1.3 — Status Messages (Level AA).
    pub struct WcagStatusMessagesProgrammatic;
    structural_prop!(
        WcagStatusMessagesProgrammatic,
        "WcagStatusMessagesProgrammatic"
    );

    /// ARIA live region levels (polite, assertive, off) are used appropriately
    /// to match the urgency of status updates.
    ///
    /// Source: WCAG 2.2 SC 4.1.3 — Status Messages (Level AA).
    pub struct WcagStatusMessagesLiveRegion;
    structural_prop!(WcagStatusMessagesLiveRegion, "WcagStatusMessagesLiveRegion");

    // ── Principle-level conformance seams ────────────────────────────────────

    /// Composite: all WCAG Principle 1 (Perceivable) Level AA criteria satisfied.
    ///
    /// Produced by [`WcagPerceivedFactory::build_perceivable`](crate::WcagPerceivedFactory::build_perceivable) when all required
    /// leaf-factory proofs (contrast, labels, structure, etc.) are supplied as
    /// evidence.  Consumed by `LevelAaEvidence::perceived`.
    ///
    /// Source: WCAG 2.2 Principle 1 — Perceivable
    pub struct WcagPerceivedValid;
    structural_prop!(WcagPerceivedValid, "WcagPerceivedValid");

    /// Composite: all WCAG Principle 2 (Operable) Level AA criteria satisfied.
    ///
    /// Produced by [`WcagOperableFactory::build_operable`](crate::WcagOperableFactory::build_operable).
    ///
    /// Source: WCAG 2.2 Principle 2 — Operable
    pub struct WcagOperableValid;
    structural_prop!(WcagOperableValid, "WcagOperableValid");

    /// Composite: all WCAG Principle 3 (Understandable) Level AA criteria satisfied.
    ///
    /// Produced by [`WcagUnderstandableFactory::build_understandable`](crate::WcagUnderstandableFactory::build_understandable).
    ///
    /// Source: WCAG 2.2 Principle 3 — Understandable
    pub struct WcagUnderstandableValid;
    structural_prop!(WcagUnderstandableValid, "WcagUnderstandableValid");

    /// Composite: all WCAG Principle 4 (Robust) criteria satisfied.
    ///
    /// Produced by [`WcagRobustFactory::build_robust`](crate::WcagRobustFactory::build_robust).
    ///
    /// Source: WCAG 2.2 Principle 4 — Robust
    pub struct WcagRobustValid;
    structural_prop!(WcagRobustValid, "WcagRobustValid");

    // ── Aggregate conformance seams ──────────────────────────────────────────

    /// Composite: all WCAG 2.2 Level A Success Criteria are satisfied.
    ///
    /// Combines all Level A propositions.  Use this as a precondition token
    /// for any operation that requires baseline accessibility.
    pub struct WcagLevelAValid;
    structural_prop!(WcagLevelAValid, "WcagLevelAValid");

    /// Composite: all WCAG 2.2 Level AA Success Criteria (A + AA) are satisfied.
    ///
    /// The typical contractual target for publicly-facing applications.
    pub struct WcagLevelAAValid;
    structural_prop!(WcagLevelAAValid, "WcagLevelAAValid");

    /// Composite: all WCAG 2.2 Level AAA Success Criteria (A + AA + AAA) are satisfied.
    pub struct WcagLevelAAAValid;
    structural_prop!(WcagLevelAAAValid, "WcagLevelAAAValid");

    // ── Legacy API back-compat aliases ───────────────────────────────────────
    //
    // These names were part of the original public API surface and are kept
    // so existing call sites continue to compile.  New code should prefer
    // the canonical `Wcag*` names above.

    /// Legacy alias: element has a non-empty accessible label.
    ///
    /// Equivalent to [`WcagNamePresent`] + non-empty label text.
    ///
    /// WCAG 2.2 SC 2.4.6 / 4.1.2.
    pub struct HasLabel;
    structural_prop!(HasLabel, "HasLabel");

    /// Legacy alias: element has a valid ARIA role.
    ///
    /// Equivalent to [`WcagRoleProgrammatic`].
    ///
    /// WCAG 2.2 SC 4.1.2.
    pub struct ValidRole;
    structural_prop!(ValidRole, "ValidRole");

    /// Legacy alias: interactive element meets 44 × 44 CSS px touch target.
    ///
    /// Equivalent to [`WcagTargetSizeEnhanced`].
    ///
    /// WCAG 2.2 SC 2.5.5 (Level AAA).
    pub struct MinTargetSize;
    structural_prop!(MinTargetSize, "MinTargetSize");

    /// Legacy alias: element does not overflow viewport boundaries.
    ///
    /// Equivalent to [`WcagContentReflowable`].
    ///
    /// WCAG 2.2 SC 1.4.10 (Level AA).
    pub struct NoOverflow;
    structural_prop!(NoOverflow, "NoOverflow");

    /// Legacy alias: element is keyboard accessible.
    ///
    /// Equivalent to [`WcagKeyboardOperable`].
    ///
    /// WCAG 2.2 SC 2.1.1 (Level A).
    pub struct KeyboardAccessible;
    structural_prop!(KeyboardAccessible, "KeyboardAccessible");

    /// Legacy alias: composite Level AA check.
    ///
    /// Equivalent to [`WcagLevelAAValid`].
    pub struct AccessibleAA;
    structural_prop!(AccessibleAA, "AccessibleAA");

    /// Legacy alias: color pair meets 4.5:1 contrast.
    ///
    /// Equivalent to [`WcagContrastMinimumNormalText`].
    ///
    /// WCAG 2.2 SC 1.4.3 (Level AA).
    pub struct SufficientContrast;
    structural_prop!(SufficientContrast, "SufficientContrast");

    /// Legacy alias: visible keyboard focus indicator present.
    ///
    /// Equivalent to [`WcagFocusVisibleKeyboard`].
    ///
    /// WCAG 2.2 SC 2.4.7 (Level AA).
    pub struct FocusVisible;
    structural_prop!(FocusVisible, "FocusVisible");

    /// Legacy alias: non-text content has a text alternative.
    ///
    /// Equivalent to [`WcagNonTextContentAltPresent`].
    ///
    /// WCAG 2.2 SC 1.1.1 (Level A).
    pub struct AltTextProvided;
    structural_prop!(AltTextProvided, "AltTextProvided");

    /// Legacy alias: information and structure are programmatically
    /// determinable.
    ///
    /// Equivalent to [`WcagInfoAndRelationshipsProgrammatic`].
    ///
    /// WCAG 2.2 SC 1.3.1 (Level A).
    pub struct StructuredContent;
    structural_prop!(StructuredContent, "StructuredContent");
}

pub use emit_impls::{
    // ── Legacy aliases (API back-compat) ──────────────────────────────────
    AccessibleAA,
    AltTextProvided,
    FocusVisible,
    HasLabel,
    KeyboardAccessible,
    MinTargetSize,
    NoOverflow,
    StructuredContent,
    SufficientContrast,
    ValidRole,
    WcagAbbreviationsExpanded,
    WcagAccessibleAuthentication,
    WcagAccessibleAuthenticationEnhanced,
    WcagAllFunctionalityKeyboard,
    WcagAudioControlAvailable,
    WcagAudioDescriptionOrMediaAlt,
    WcagAudioDescriptionPrerecorded,
    // 1.2 Time-based Media
    WcagAudioOnlyAlternativeProvided,
    WcagAudioOnlyLiveAlternative,
    WcagAutoUpdatePausable,
    // 2.4 Navigable
    WcagBypassBlocksMechanism,
    WcagCaptchaMultipleModalities,
    WcagCaptionsLiveProvided,
    WcagCaptionsSynchronized,
    WcagChangesOnRequest,
    WcagCharacterShortcutsDisableable,
    WcagCharacterShortcutsFocusOnly,
    WcagCharacterShortcutsRemappable,
    // 1.4 Distinguishable
    WcagColorNotSoleConveyor,
    WcagComponentPurposeIdentifiable,
    WcagConcurrentInputMechanisms,
    WcagConsistentHelpLocated,
    WcagContentReflowable,
    WcagContextSensitiveHelp,
    WcagContrastEnhancedLargeText,
    WcagContrastEnhancedNormalText,
    WcagContrastMinimumLargeText,
    WcagContrastMinimumLogotypeExcepted,
    WcagContrastMinimumNormalText,
    WcagDecorativeImageAltEmpty,
    WcagDraggingAlternative,
    // 3.3 Input Assistance
    WcagErrorIdentificationDescriptive,
    WcagErrorPreventionAll,
    WcagErrorPreventionChecked,
    WcagErrorPreventionConfirmed,
    WcagErrorPreventionLegal,
    WcagErrorPreventionReversible,
    WcagErrorSuggestionProvided,
    WcagExtendedAudioDescription,
    WcagFlashAreaBelowThreshold,
    WcagFocusAppearanceEnhancedArea,
    WcagFocusAppearanceEnhancedContrast,
    WcagFocusAppearanceMinimumArea,
    WcagFocusAppearanceMinimumContrast,
    WcagFocusIndicatorContrast,
    // 3.2 Predictable
    WcagFocusNoContextChange,
    WcagFocusOrderLogical,
    WcagFocusVisibleKeyboard,
    WcagFormLabelsProgrammatic,
    WcagHeadingStructureProgrammatic,
    WcagHeadingsDescriptive,
    WcagHoverContentDismissible,
    WcagHoverContentHoverable,
    WcagHoverContentPersistent,
    WcagIdentificationConsistent,
    WcagImagesOfTextAvoided,
    WcagImagesOfTextCustomizable,
    WcagImagesOfTextNoException,
    // 1.3 Adaptable
    WcagInfoAndRelationshipsProgrammatic,
    WcagInputNoContextChange,
    WcagInputPurposeIdentifiable,
    WcagInterruptionsPostponable,
    WcagKeyboardEscapeFromComponent,
    WcagKeyboardNoTimingPath,
    WcagKeyboardNotTrapped,
    // ── Principle 2: Operable ─────────────────────────────────────────────
    // 2.1 Keyboard Accessible
    WcagKeyboardOperable,
    WcagLabelInNameMatch,
    WcagLabelsDescriptive,
    WcagLabelsOrInstructionsPresent,
    WcagLargeTextClassified,
    WcagLetterSpacingAdjustable,
    WcagLevelAAAValid,
    WcagLevelAAValid,
    // ── Aggregate seams ───────────────────────────────────────────────────
    WcagLevelAValid,
    WcagLineHeightAdjustable,
    WcagLinkPurposeFromContext,
    WcagLinkPurposeLinkOnly,
    WcagListStructureProgrammatic,
    WcagLocationInNavigationSet,
    WcagLowBackgroundAudio,
    WcagMeaningfulSequencePreservable,
    WcagMediaAlternativePrerecorded,
    WcagMotionActuationAlternative,
    WcagMotionActuationDisableable,
    WcagMultiplePathsToContent,
    WcagNamePresent,
    WcagNameRoleValueProgrammatic,
    WcagNavigationConsistent,
    WcagNoHorizontalScrollVerticalText,
    WcagNoThreeFlashAbsolute,
    WcagNoTimingRequired,
    WcagNonTextContentAltDescriptive,
    WcagNonTextContentAltNonEmpty,
    // ── Principle 1: Perceivable ──────────────────────────────────────────
    // 1.1 Text Alternatives
    WcagNonTextContentAltPresent,
    WcagNonTextContrastMinimum,
    WcagOperableValid,
    WcagOrientationNotRestricted,
    // ── Principle 3: Understandable ───────────────────────────────────────
    // 3.1 Readable
    WcagPageLanguageIdentified,
    WcagPageTitleDescriptive,
    WcagPageTitled,
    WcagParagraphSpacingAdjustable,
    // ── Principle 4: Robust ───────────────────────────────────────────────
    WcagParsingValid,
    WcagPartLanguageIdentified,
    WcagPauseStopHideAvailable,
    // ── Principle-level conformance seams ────────────────────────────────
    WcagPerceivedValid,
    WcagPointerCancellationAbortable,
    WcagPointerCancellationReversible,
    WcagPointerCancellationUpEvent,
    // 2.5 Input Modalities
    WcagPointerGesturesSimpleAlternative,
    WcagPronunciationAvailable,
    WcagReadingLevelSupplemented,
    WcagReauthWithoutDataLoss,
    WcagReducedMotionRespected,
    WcagRedundantEntryMinimized,
    WcagRobustValid,
    WcagRoleProgrammatic,
    WcagSectionHeadingsPresent,
    WcagSensoryNotExclusive,
    WcagSignLanguagePrerecorded,
    WcagStatusMessagesLiveRegion,
    WcagStatusMessagesProgrammatic,
    WcagTableHeadersProgrammatic,
    WcagTargetSizeEnhanced,
    WcagTargetSizeEnhancedHeight,
    WcagTargetSizeEnhancedWidth,
    WcagTargetSizeMinimum,
    WcagTargetSizeMinimumHeight,
    WcagTargetSizeMinimumSpaced,
    WcagTargetSizeMinimumWidth,
    WcagTextResizable,
    WcagTextSpacingAdjustable,
    // 2.3 Seizures and Physical Reactions
    WcagThreeFlashBelowThreshold,
    WcagTimeoutWarningProvided,
    WcagTimingAdjustTenX,
    // 2.2 Enough Time
    WcagTimingAdjustable,
    WcagTimingExtendWarning,
    WcagTimingTurnOffAvailable,
    WcagUnderstandableValid,
    WcagUnusualWordsDefined,
    WcagValueStatesProgrammatic,
    WcagVideoOnlyAlternativeProvided,
    WcagVisualPresentationCustomizable,
    WcagWordSpacingAdjustable,
};
