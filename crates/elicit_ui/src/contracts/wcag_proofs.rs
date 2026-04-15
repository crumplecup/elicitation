//! `ProvableFrom` bindings: credential type → WCAG proposition.
//!
//! Each `impl` in this module declares the type-level relationship
//! "credential C proves proposition P".  Only code that can construct C
//! (i.e., only the factory methods in `accesskit_backend.rs`, which are
//! `pub(crate)`) can mint the corresponding `Established<P>` proof token.
//!
//! # Organisation
//!
//! Impls are grouped in the same order as the factory traits:
//! contrast → label → focus → keyboard → timing → target → structure →
//! media → language → error → layout/navigation → section aggregates.

use elicitation::contracts::ProvableFrom;

use crate::proof_credentials::{
    AccessibleNameVerified, AudioDescriptionVerified, CaptionsVerified,
    EnhancedLargeTextContrastVerified, EnhancedNormalTextContrastVerified, ErrorIdentifiedVerified,
    ErrorPreventionVerified, ErrorSuggestionVerified, FocusActivated,
    FocusAppearanceEnhancedVerified, FocusAppearanceMinimumVerified, FocusOrderSet,
    FocusVisibleVerified, FormLabelVerified, HeadingCreated, KeyboardEscapeVerified,
    KeyboardOperableVerified, LabelInNameVerified, LabelsAndInstructionsVerified,
    LargeTextContrastVerified, LayoutContainerCreated, ListCreated, NonTextContrastVerified,
    NormalTextContrastVerified, PageLanguageVerified, PartLanguageVerified,
    PointerCancellationVerified, PointerGestureAlternativeVerified, RemappableShortcutVerified,
    ResizableTextCreated, ShortcutRegistered, SkipLinkAdded, TableCreated,
    TargetSizeEnhancedVerified, TargetSizeMinimumVerified, TimingAdjustableVerified,
};
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

impl ProvableFrom<NormalTextContrastVerified> for WcagContrastMinimumNormalText {}
impl ProvableFrom<LargeTextContrastVerified> for WcagContrastMinimumLargeText {}
impl ProvableFrom<EnhancedNormalTextContrastVerified> for WcagContrastEnhancedNormalText {}
impl ProvableFrom<EnhancedLargeTextContrastVerified> for WcagContrastEnhancedLargeText {}
impl ProvableFrom<NonTextContrastVerified> for WcagNonTextContrastMinimum {}

// ── Label ─────────────────────────────────────────────────────────────────────

impl ProvableFrom<AccessibleNameVerified> for WcagNamePresent {}
impl ProvableFrom<FormLabelVerified> for WcagFormLabelsProgrammatic {}
impl ProvableFrom<LabelInNameVerified> for WcagLabelInNameMatch {}

// ── Focus ─────────────────────────────────────────────────────────────────────

impl ProvableFrom<FocusVisibleVerified> for WcagFocusVisibleKeyboard {}
impl ProvableFrom<FocusAppearanceMinimumVerified> for WcagFocusAppearanceMinimumArea {}
impl ProvableFrom<FocusAppearanceEnhancedVerified> for WcagFocusAppearanceEnhancedArea {}

// ── Keyboard ──────────────────────────────────────────────────────────────────

impl ProvableFrom<KeyboardOperableVerified> for WcagKeyboardOperable {}
impl ProvableFrom<KeyboardEscapeVerified> for WcagKeyboardNotTrapped {}
impl ProvableFrom<RemappableShortcutVerified> for WcagCharacterShortcutsRemappable {}

// ── Timing ────────────────────────────────────────────────────────────────────

impl ProvableFrom<TimingAdjustableVerified> for WcagTimingAdjustable {}

// ── Target size ───────────────────────────────────────────────────────────────

impl ProvableFrom<TargetSizeMinimumVerified> for WcagTargetSizeMinimum {}
impl ProvableFrom<TargetSizeEnhancedVerified> for WcagTargetSizeEnhanced {}
impl ProvableFrom<PointerGestureAlternativeVerified> for WcagPointerGesturesSimpleAlternative {}
impl ProvableFrom<PointerCancellationVerified> for WcagPointerCancellationUpEvent {}

// ── Structure ─────────────────────────────────────────────────────────────────

impl ProvableFrom<HeadingCreated> for WcagHeadingStructureProgrammatic {}
impl ProvableFrom<ListCreated> for WcagListStructureProgrammatic {}
impl ProvableFrom<TableCreated> for WcagTableHeadersProgrammatic {}
impl ProvableFrom<ResizableTextCreated> for WcagTextResizable {}

// ── Media ─────────────────────────────────────────────────────────────────────

impl ProvableFrom<CaptionsVerified> for WcagCaptionsSynchronized {}
impl ProvableFrom<AudioDescriptionVerified> for WcagAudioDescriptionPrerecorded {}

// ── Language ──────────────────────────────────────────────────────────────────

impl ProvableFrom<PageLanguageVerified> for WcagPageLanguageIdentified {}
impl ProvableFrom<PartLanguageVerified> for WcagPartLanguageIdentified {}

// ── Error ─────────────────────────────────────────────────────────────────────

impl ProvableFrom<ErrorIdentifiedVerified> for WcagErrorIdentificationDescriptive {}
impl ProvableFrom<LabelsAndInstructionsVerified> for WcagLabelsOrInstructionsPresent {}
impl ProvableFrom<ErrorSuggestionVerified> for WcagErrorSuggestionProvided {}
impl ProvableFrom<ErrorPreventionVerified> for WcagErrorPreventionLegal {}

// ── Layout ────────────────────────────────────────────────────────────────────

/// A created layout container proves that its children are contained within
/// bounds that prevent viewport overflow (WCAG 1.4.10 Reflow).
impl ProvableFrom<LayoutContainerCreated> for NoOverflow {}

// ── Navigation ────────────────────────────────────────────────────────────────

/// An established focus order proves keyboard accessibility (WCAG 2.4.3).
impl ProvableFrom<FocusOrderSet> for KeyboardAccessible {}

/// A focused widget proves the focus indicator is visible (WCAG 2.4.7).
impl ProvableFrom<FocusActivated> for FocusVisible {}

/// A registered shortcut with an accessible label proves keyboard
/// accessibility (WCAG 2.1.4).
impl ProvableFrom<ShortcutRegistered> for KeyboardAccessible {}

/// A skip-navigation link proves keyboard accessibility (WCAG 2.4.1).
impl ProvableFrom<SkipLinkAdded> for KeyboardAccessible {}

// ── Section aggregates (evidence bundles are the credentials) ─────────────────

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
