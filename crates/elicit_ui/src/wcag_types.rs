//! Descriptor, construct, and evidence types for WCAG factory traits.
//!
//! # Three categories
//!
//! - **Descriptors** — raw input data supplied by the caller.  No contracts
//!   assumed; the factory validates and either returns a construct or an error.
//! - **Validated constructs** — output of a successful factory call.  Can only
//!   be created through the corresponding factory; their existence is the proof.
//! - **Evidence bundles** — structured collections of `Established<P>` proof
//!   tokens required as preconditions by section factories.  The type system
//!   prevents calling a section factory without supplying all required evidence.

use elicitation::Established;

use crate::{
    SrgbColor, WcagAccessibleAuthentication, WcagBypassBlocksMechanism, WcagColorNotSoleConveyor,
    WcagConsistentHelpLocated, WcagContrastMinimumNormalText, WcagErrorIdentificationDescriptive,
    WcagFocusAppearanceMinimumArea, WcagFocusIndicatorContrast, WcagFocusOrderLogical,
    WcagFocusVisibleKeyboard, WcagIdentificationConsistent, WcagInfoAndRelationshipsProgrammatic,
    WcagKeyboardNotTrapped, WcagKeyboardOperable, WcagLabelsOrInstructionsPresent,
    WcagLinkPurposeFromContext, WcagNamePresent, WcagNameRoleValueProgrammatic,
    WcagNavigationConsistent, WcagNonTextContrastMinimum, WcagOperableValid,
    WcagPageLanguageIdentified, WcagPageTitled, WcagPerceivedValid, WcagPointerCancellationUpEvent,
    WcagRedundantEntryMinimized, WcagRobustValid, WcagStatusMessagesProgrammatic,
    WcagTargetSizeMinimum, WcagTextResizable, WcagUnderstandableValid, WidgetId,
};

// ── Descriptors (raw input data) ──────────────────────────────────────────────

/// Raw input for contrast factory methods.
///
/// The factory validates the ratio and returns a [`ContrastPair`] proof construct
/// or an error if the ratio falls below the required threshold.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContrastDescriptor {
    /// Foreground (text or component) colour.
    pub foreground: SrgbColor,
    /// Background colour.
    pub background: SrgbColor,
}

/// Raw input for label factory methods.
///
/// `name` is a plain `String`; the factory rejects empty strings.
/// The factory creates a new AccessKit node with the given `role`.
#[derive(Debug, Clone, PartialEq)]
pub struct LabelDescriptor {
    /// Proposed accessible name text.
    pub name: String,
    /// AccessKit role string (e.g., `"button"`, `"link"`, `"text-input"`, `"image"`).
    pub role: String,
    /// Optional ID of a separate labelling element (ARIA `labelledby`).
    pub labelled_by: Option<WidgetId>,
}

/// Raw input for focus appearance factory methods.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FocusDescriptor {
    /// Widget whose focus indicator is being validated.
    pub widget: WidgetId,
    /// Area of the focus indicator in CSS pixels².
    ///
    /// WCAG 2.4.11 minimum: perimeter of the component × 2 px.
    pub indicator_area_px: f64,
    /// Contrast ratio between the indicator colour and its adjacent colours.
    ///
    /// WCAG 2.4.11 minimum: 3:1.
    pub indicator_contrast: f64,
}

/// Raw input for keyboard accessibility factory methods.
#[derive(Debug, Clone, PartialEq)]
pub struct KeyboardDescriptor {
    /// Widget to include in the keyboard navigation order.
    pub widget: WidgetId,
    /// Position in the tab order (`0` = natural DOM order).
    pub tab_index: i32,
    /// Single-character keyboard shortcut, if any.
    pub shortcut: Option<char>,
}

/// Raw input for timing factory methods.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimingDescriptor {
    /// Widget (or page context) with a time limit.
    pub element: WidgetId,
    /// Maximum time allowed in seconds (`None` = no limit).
    pub max_seconds: Option<u64>,
    /// Whether the user can pause the time limit.
    pub can_pause: bool,
    /// Whether the user can extend the time limit.
    pub can_extend: bool,
    /// Whether the user can turn off the time limit entirely.
    pub can_turn_off: bool,
}

/// Raw input for pointer target factory methods.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TargetDescriptor {
    /// Widget whose pointer target area is being validated.
    pub widget: WidgetId,
    /// Rendered width in CSS pixels.
    pub width_px: f64,
    /// Rendered height in CSS pixels.
    pub height_px: f64,
    /// Minimum spacing to adjacent targets in CSS pixels.
    ///
    /// A target that does not meet the 24×24 px minimum size requirement
    /// still passes WCAG 2.5.8 if this spacing is sufficient.
    pub adjacent_spacing_px: f64,
}

/// Raw input for structure factory methods.
///
/// The factory creates a new AccessKit node with the given `role` and `label`.
#[derive(Debug, Clone, PartialEq)]
pub struct StructureDescriptor {
    /// Accessible name / visible text for the structure element.
    pub label: String,
    /// AccessKit role name (e.g., `"heading"`, `"list"`, `"table"`).
    pub role: String,
    /// Heading level 1–6 when `role` is `"heading"`.
    pub heading_level: Option<u8>,
}

/// Raw input for media factory methods.
///
/// The factory creates a new AccessKit media node with the given label.
#[derive(Debug, Clone, PartialEq)]
pub struct MediaDescriptor {
    /// Accessible label / alt text for the media element.
    pub label: String,
    /// Whether synchronised captions are present (WCAG 1.2.2).
    pub has_captions: bool,
    /// Whether an audio description track is present (WCAG 1.2.5).
    pub has_audio_description: bool,
    /// Whether a text transcript is available (WCAG 1.2.1).
    pub has_transcript: bool,
}

/// Raw input for language factory methods.
#[derive(Debug, Clone, PartialEq)]
pub struct LanguageDescriptor {
    /// BCP-47 language tag for the page (`html lang` attribute).
    pub page_lang: String,
    /// BCP-47 language tag for a specific element, if different from the page.
    pub element_lang: Option<String>,
}

/// Raw input for error field factory methods.
#[derive(Debug, Clone, PartialEq)]
pub struct ErrorDescriptor {
    /// Form widget that has an associated error.
    pub widget: WidgetId,
    /// Human-readable error description (WCAG 3.3.1).
    pub error_text: Option<String>,
    /// Suggested correction (WCAG 3.3.3).
    pub suggestion: Option<String>,
}

// ── Validated constructs (output of successful factory calls) ─────────────────

/// A colour pair that has been proven to meet a WCAG contrast threshold.
///
/// Only constructible through a [`WcagContrastFactory`](crate::WcagContrastFactory) method; its existence
/// constitutes evidence of the associated contrast proposition.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContrastPair {
    /// Foreground colour.
    pub foreground: SrgbColor,
    /// Background colour.
    pub background: SrgbColor,
    /// Measured WCAG 2.1 contrast ratio (1.0–21.0).
    pub ratio: f64,
}

/// A widget that has been proven to have an accessible name.
///
/// Only constructible through a [`WcagLabelFactory`](crate::WcagLabelFactory) method.
#[derive(Debug, Clone, PartialEq)]
pub struct LabeledElement {
    /// Widget identifier.
    pub id: WidgetId,
    /// Accessible name text.
    pub name: String,
}

/// A focus indicator that has been proven to meet the WCAG 2.4.11 appearance thresholds.
///
/// Only constructible through a [`WcagFocusFactory`](crate::WcagFocusFactory) method.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FocusIndicator {
    /// Widget identifier.
    pub widget: WidgetId,
    /// Indicator area in CSS pixels².
    pub area_px: f64,
    /// Indicator contrast ratio.
    pub contrast: f64,
}

/// A widget that has been proven to be reachable via keyboard navigation.
///
/// Only constructible through a [`WcagKeyboardFactory`](crate::WcagKeyboardFactory) method.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyboardPath {
    /// Widget identifier.
    pub widget: WidgetId,
    /// Position in the tab order.
    pub tab_index: i32,
}

/// A timed element whose time controls have been proven to meet WCAG 2.2.
///
/// Only constructible through a [`WcagTimingFactory`](crate::WcagTimingFactory) method.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimedElement {
    /// Widget identifier.
    pub widget: WidgetId,
    /// Maximum time in seconds (`None` if no limit).
    pub max_seconds: Option<u64>,
}

/// A pointer target that has been proven to meet the WCAG 2.5.8 minimum size.
///
/// Only constructible through a [`WcagTargetFactory`](crate::WcagTargetFactory) method.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointerTarget {
    /// Widget identifier.
    pub id: WidgetId,
    /// Validated target width in CSS pixels.
    pub width_px: f64,
    /// Validated target height in CSS pixels.
    pub height_px: f64,
}

/// A widget whose semantic structure has been proven to be programmatically determinable.
///
/// Only constructible through a [`WcagStructureFactory`](crate::WcagStructureFactory) method.
#[derive(Debug, Clone, PartialEq)]
pub struct StructuredElement {
    /// Widget identifier.
    pub id: WidgetId,
    /// AccessKit role name.
    pub role: String,
}

/// A media element proven to have synchronised captions.
///
/// Only constructible through a [`WcagMediaFactory`](crate::WcagMediaFactory) method.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CaptionedMedia {
    /// Widget identifier.
    pub id: WidgetId,
}

/// A page proven to have a programmatically identified language.
///
/// Only constructible through a [`WcagLanguageFactory`](crate::WcagLanguageFactory) method.
#[derive(Debug, Clone, PartialEq)]
pub struct LanguagePage {
    /// BCP-47 language tag.
    pub lang: String,
}

/// A form field proven to have a descriptive error message.
///
/// Only constructible through a [`WcagErrorFactory`](crate::WcagErrorFactory) method.
#[derive(Debug, Clone, PartialEq)]
pub struct ErrorField {
    /// Widget identifier.
    pub id: WidgetId,
    /// Descriptive error text.
    pub description: String,
}

/// A UI surface proven to satisfy all WCAG Principle 1 (Perceivable) Level AA criteria.
///
/// Only constructible through [`WcagPerceivedFactory::build_perceivable`](crate::WcagPerceivedFactory::build_perceivable).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PerceivedSection {
    /// Number of elements included in the validation sweep.
    pub validated_count: usize,
}

/// A UI surface proven to satisfy all WCAG Principle 2 (Operable) Level AA criteria.
///
/// Only constructible through [`WcagOperableFactory::build_operable`](crate::WcagOperableFactory::build_operable).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OperableInterface {
    /// Number of interactive elements included in the validation sweep.
    pub validated_count: usize,
}

/// A UI surface proven to satisfy all WCAG Principle 3 (Understandable) Level AA criteria.
///
/// Only constructible through [`WcagUnderstandableFactory::build_understandable`](crate::WcagUnderstandableFactory::build_understandable).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UnderstandableInterface {
    /// Number of elements included in the validation sweep.
    pub validated_count: usize,
}

/// A UI surface proven to satisfy all WCAG Principle 4 (Robust) criteria.
///
/// Only constructible through [`WcagRobustFactory::build_robust`](crate::WcagRobustFactory::build_robust).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RobustWidget {
    /// Number of widgets included in the validation sweep.
    pub validated_count: usize,
}

// ── Evidence bundles (preconditions for section factories) ────────────────────
//
// Each bundle wraps the `Established<P>` proof tokens that must have been
// obtained from leaf factories *before* calling the corresponding section
// factory.  The type system makes it impossible to call a section factory
// without supplying complete evidence: you cannot construct the bundle without
// all required proofs.

/// Evidence required to prove WCAG Principle 1 (Perceivable) Level AA.
///
/// Collect by running the relevant leaf factories for every element in the
/// UI surface, then pass the bundle to
/// [`WcagPerceivedFactory::build_perceivable`](crate::WcagPerceivedFactory::build_perceivable).
#[derive(Debug, Clone, Copy)]
pub struct PerceivedEvidence {
    /// Normal text meets WCAG 1.4.3 contrast (4.5:1).
    pub contrast_normal: Established<WcagContrastMinimumNormalText>,
    /// Non-text controls meet WCAG 1.4.11 contrast (3:1).
    pub non_text_contrast: Established<WcagNonTextContrastMinimum>,
    /// Focus indicator meets WCAG 1.4.11 / 2.4.11 contrast.
    pub focus_contrast: Established<WcagFocusIndicatorContrast>,
    /// Colour is not the sole means of conveying information (WCAG 1.4.1).
    pub color_not_sole: Established<WcagColorNotSoleConveyor>,
    /// Text can be resized up to 200 % without loss of content (WCAG 1.4.4).
    pub text_resizable: Established<WcagTextResizable>,
    /// Information and relationships are programmatically determinable (WCAG 1.3.1).
    pub structure: Established<WcagInfoAndRelationshipsProgrammatic>,
    /// All non-text content has an accessible name (WCAG 1.1.1 / 4.1.2).
    pub name_present: Established<WcagNamePresent>,
}

/// Evidence required to prove WCAG Principle 2 (Operable) Level AA.
///
/// Collect by running the relevant leaf factories, then pass to
/// [`WcagOperableFactory::build_operable`](crate::WcagOperableFactory::build_operable).
#[derive(Debug, Clone, Copy)]
pub struct OperableEvidence {
    /// All functionality is operable via keyboard (WCAG 2.1.1).
    pub keyboard_operable: Established<WcagKeyboardOperable>,
    /// Keyboard focus is never trapped (WCAG 2.1.2).
    pub no_keyboard_trap: Established<WcagKeyboardNotTrapped>,
    /// Focus order is logical (WCAG 2.4.3).
    pub focus_order: Established<WcagFocusOrderLogical>,
    /// Focus is visually apparent for keyboard users (WCAG 2.4.7).
    pub focus_visible: Established<WcagFocusVisibleKeyboard>,
    /// Focus indicator meets minimum appearance (WCAG 2.4.11).
    pub focus_appearance: Established<WcagFocusAppearanceMinimumArea>,
    /// A mechanism exists to bypass repeated navigation blocks (WCAG 2.4.1).
    pub bypass_blocks: Established<WcagBypassBlocksMechanism>,
    /// Pages have descriptive titles (WCAG 2.4.2).
    pub page_titled: Established<WcagPageTitled>,
    /// Link purpose can be determined from context (WCAG 2.4.4).
    pub link_purpose: Established<WcagLinkPurposeFromContext>,
    /// Pointer events use up-event or provide cancellation (WCAG 2.5.2).
    pub pointer_cancel: Established<WcagPointerCancellationUpEvent>,
    /// Pointer target meets minimum size or spacing (WCAG 2.5.8).
    pub target_minimum: Established<WcagTargetSizeMinimum>,
}

/// Evidence required to prove WCAG Principle 3 (Understandable) Level AA.
///
/// Collect by running the relevant leaf factories, then pass to
/// [`WcagUnderstandableFactory::build_understandable`](crate::WcagUnderstandableFactory::build_understandable).
#[derive(Debug, Clone, Copy)]
pub struct UnderstandableEvidence {
    /// Page language is programmatically identified (WCAG 3.1.1).
    pub page_language: Established<WcagPageLanguageIdentified>,
    /// Navigation is consistent across pages (WCAG 3.2.3).
    pub navigation_consistent: Established<WcagNavigationConsistent>,
    /// Components that repeat are identified consistently (WCAG 3.2.4).
    pub identification_consistent: Established<WcagIdentificationConsistent>,
    /// Errors are identified in text (WCAG 3.3.1).
    pub error_identified: Established<WcagErrorIdentificationDescriptive>,
    /// Labels or instructions accompany all inputs (WCAG 3.3.2).
    pub labels_present: Established<WcagLabelsOrInstructionsPresent>,
    /// Help is located consistently across pages (WCAG 3.2.6).
    pub help_located: Established<WcagConsistentHelpLocated>,
    /// Redundant entry of information is minimised (WCAG 3.3.7).
    pub redundant_entry: Established<WcagRedundantEntryMinimized>,
    /// Authentication does not require cognitive function tests (WCAG 3.3.8).
    pub accessible_auth: Established<WcagAccessibleAuthentication>,
}

/// Evidence required to prove WCAG Principle 4 (Robust).
///
/// Collect by running the relevant leaf factories, then pass to
/// [`WcagRobustFactory::build_robust`](crate::WcagRobustFactory::build_robust).
#[derive(Debug, Clone, Copy)]
pub struct RobustEvidence {
    /// Name, role, and value are programmatically determinable (WCAG 4.1.2).
    pub name_role_value: Established<WcagNameRoleValueProgrammatic>,
    /// Status messages are programmatically determinable (WCAG 4.1.3).
    pub status_messages: Established<WcagStatusMessagesProgrammatic>,
}

/// Evidence required to produce a full [`WcagLevelAAValid`] proof.
///
/// Assemble by running all four section factories, then pass to
/// [`WcagBackend::build_level_aa`](crate::WcagBackend::build_level_aa).
///
/// [`WcagLevelAAValid`]: crate::WcagLevelAAValid
#[derive(Debug, Clone, Copy)]
pub struct LevelAaEvidence {
    /// Principle 1 proof from [`WcagPerceivedFactory::build_perceivable`](crate::WcagPerceivedFactory::build_perceivable).
    pub perceived: Established<WcagPerceivedValid>,
    /// Principle 2 proof from [`WcagOperableFactory::build_operable`](crate::WcagOperableFactory::build_operable).
    pub operable: Established<WcagOperableValid>,
    /// Principle 3 proof from [`WcagUnderstandableFactory::build_understandable`](crate::WcagUnderstandableFactory::build_understandable).
    pub understandable: Established<WcagUnderstandableValid>,
    /// Principle 4 proof from [`WcagRobustFactory::build_robust`](crate::WcagRobustFactory::build_robust).
    pub robust: Established<WcagRobustValid>,
}
