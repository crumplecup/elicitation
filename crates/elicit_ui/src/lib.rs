//! `elicit_ui` — WCAG-enforced accessible UI system using AccessKit as the IR.
//!
//! Provides a formally verifiable UI construction system using:
//!
//! 1. **AccessKit universal IR** — all UI represented as accessibility trees
//! 2. **Typestate state machine** — `Pending → Verified`
//! 3. **Proof-carrying contracts** — WCAG compliance enforced by construction
//! 4. **Multiple frontends** — bridge to egui, leptos, ratatui from single IR
//!
//! # Architecture
//!
//! ```text
//! WCAG factory traits
//!        ↓
//! AccessKitUiBackend  (monomorphic — one concrete impl)
//!        ↓
//! AccessKit IR  (TreeUpdate — the validated output)
//!        ↓
//! Frontend bridges  (egui / leptos / ratatui — concrete modules)
//! ```
//!
//! # State Machine
//!
//! - `Layout<Pending>` — awaiting verification
//! - `Layout<Verified>` — verified against WCAG Level AA constraints
//!
//! # Propositions (WCAG Compliance)
//!
//! - `HasLabel` — element has non-empty accessible label
//! - `ValidRole` — element has valid ARIA role
//! - `MinTargetSize` — interactive element ≥44x44 (Level AAA touch targets)
//! - `NoOverflow` — element fits within viewport
//! - `KeyboardAccessible` — element keyboard-navigable
//! - `AccessibleAA` — composite (all Level AA criteria)
//! - `SufficientContrast` — color pair meets 4.5:1 ratio (WCAG 1.4.3)
//! - `FocusVisible` — visible focus indicator (WCAG 2.4.11)
//! - `AltTextProvided` — text alternative for non-text content (WCAG 1.1.1)
//! - `StructuredContent` — structure is programmatically determinable (WCAG 1.3.1)
//! - `RenderComplete` — UI tree successfully rendered

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod accesskit_backend;
mod builder;
mod color_contrast;
pub mod constraints;
mod contracts;
mod css_units;
mod errors;
mod layout_engine;
mod spatial;
pub mod traits;
mod types;
mod typestate;
mod ui_types;
mod validators;
mod wcag_types;

pub use accesskit_backend::AccessKitUiBackend;
pub use builder::LayoutBuilder;
pub use color_contrast::{
    ContrastEnhanced, ContrastMinimum, NonTextContrast, SrgbColor, TextSize, contrast_ratio,
};
pub use constraints::{
    BreakpointOutcome, BreakpointReport, BreakpointResult, BreakpointTier, Constraint,
    ConstraintContext, ConstraintSet, ConstraintSetBuilder, ConstraintVerification, GridAlignment,
    HasLabelConstraint, KeyboardAccessibleConstraint, MinReadableSize, MinSpacing,
    MinTouchTargetConstraint, NoOverflowConstraint, Reflow320, ResizeText200, SpecReference,
    TerminalAccessible, TerminalBreakpoint, TerminalBreakpointSet, TerminalNoOverflow, TextSpacing,
    ValidRoleConstraint, Violation, WcagLevel,
};
pub use contracts::{
    // ── Legacy aliases (API back-compat) ──────────────────────────────────
    AccessibleAA,
    AltTextProvided,
    FocusVisible,
    HasLabel,
    // ── UI pipeline ───────────────────────────────────────────────────────
    IrSourced,
    KeyboardAccessible,
    MinTargetSize,
    NoOverflow,
    RenderComplete,
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
    WcagAudioOnlyAlternativeProvided,
    WcagAudioOnlyLiveAlternative,
    WcagAutoUpdatePausable,
    WcagBypassBlocksMechanism,
    WcagCaptchaMultipleModalities,
    WcagCaptionsLiveProvided,
    WcagCaptionsSynchronized,
    WcagChangesOnRequest,
    WcagCharacterShortcutsDisableable,
    WcagCharacterShortcutsFocusOnly,
    WcagCharacterShortcutsRemappable,
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
    WcagInfoAndRelationshipsProgrammatic,
    WcagInputNoContextChange,
    WcagInputPurposeIdentifiable,
    WcagInterruptionsPostponable,
    WcagKeyboardEscapeFromComponent,
    WcagKeyboardNoTimingPath,
    WcagKeyboardNotTrapped,
    // ── Principle 2: Operable ─────────────────────────────────────────────
    WcagKeyboardOperable,
    WcagLabelInNameMatch,
    WcagLabelsDescriptive,
    WcagLabelsOrInstructionsPresent,
    WcagLetterSpacingAdjustable,
    // ── Aggregate seams ───────────────────────────────────────────────────
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
    WcagNonTextContentAltPresent,
    WcagNonTextContrastMinimum,
    WcagOperableValid,
    WcagOrientationNotRestricted,
    // ── Principle 3: Understandable ───────────────────────────────────────
    WcagPageLanguageIdentified,
    WcagPageTitleDescriptive,
    WcagPageTitled,
    WcagParagraphSpacingAdjustable,
    // ── Principle 4: Robust ───────────────────────────────────────────────
    WcagParsingValid,
    WcagPartLanguageIdentified,
    WcagPauseStopHideAvailable,
    // ── Principle-level conformance seams ────────────────────────────────────
    WcagPerceivedValid,
    WcagPointerCancellationAbortable,
    WcagPointerCancellationReversible,
    WcagPointerCancellationUpEvent,
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
    WcagThreeFlashBelowThreshold,
    WcagTimeoutWarningProvided,
    WcagTimingAdjustTenX,
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
pub use css_units::{Breakpoint, BreakpointSet, CssLength, CssParseError, is_zoom_invariant};
pub use elicit_accesskit::ColorTheme;
pub use errors::{
    UiError, UiErrorKind, UiResult, VerificationError, VerificationErrorKind, VerificationReport,
};
pub use layout_engine::LayoutEngineError;
#[cfg(feature = "layout-engine")]
pub use layout_engine::{LayoutMode, TaffyBridge};
pub use spatial::{BoundingBox, LayoutContext};
pub use traits::{
    UiBackend, UiEventBridge, UiEventDispatcher, UiInspector, UiLayoutManager, UiNavigationManager,
    UiNodeBridge, UiRenderBackend, UiRenderer, UiTreeRenderer, WcagBackend, WcagContrastFactory,
    WcagElementMeta, WcagErrorFactory, WcagFocusFactory, WcagKeyboardFactory, WcagLabelFactory,
    WcagLanguageFactory, WcagMediaFactory, WcagOperableFactory, WcagPageMeta, WcagPerceivedFactory,
    WcagRobustFactory, WcagStructureFactory, WcagTargetFactory, WcagTimingFactory,
    WcagUnderstandableFactory,
};
pub use types::{ElementId, Label, RenderStats, Size, Viewport};
pub use typestate::{ConstraintProfile, Layout, Pending, Verified};
pub use ui_types::{
    ContainerId, ContrastViolation, VerifiedTree, WidgetA11y, WidgetId, WidgetInfo,
};
pub use wcag_types::{
    CaptionedMedia, ContrastDescriptor, ContrastPair, ErrorDescriptor, ErrorField, FocusDescriptor,
    FocusIndicator, KeyboardDescriptor, KeyboardPath, LabelDescriptor, LabeledElement,
    LanguageDescriptor, LanguagePage, LevelAaEvidence, MediaDescriptor, OperableEvidence,
    OperableInterface, PerceivedEvidence, PerceivedSection, PointerTarget, RobustEvidence,
    RobustWidget, StructureDescriptor, StructuredElement, TargetDescriptor, TimedElement,
    TimingDescriptor, UnderstandableEvidence, UnderstandableInterface,
};
