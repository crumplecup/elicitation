//! Integration tests for `AccessKitUiBackend` WCAG factory methods.

use elicit_ui::{
    AccessKitUiBackend, ContrastDescriptor, ErrorDescriptor, FocusDescriptor, KeyboardDescriptor,
    LabelDescriptor, LanguageDescriptor, MediaDescriptor, SrgbColor, StructureDescriptor,
    TargetDescriptor, TextSizeDescriptor, TextSpacingDescriptor, TimingDescriptor, UiInspector,
    UiLayoutManager, UiNavigationManager, Viewport, WcagContrastFactory, WcagElementMeta,
    WcagErrorFactory, WcagFocusFactory, WcagKeyboardFactory, WcagLabelFactory, WcagLanguageFactory,
    WcagMediaFactory, WcagPageMeta, WcagStructureFactory, WcagTargetFactory, WcagTimingFactory,
    WidgetId,
};

fn black() -> SrgbColor {
    SrgbColor {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    }
}

fn white() -> SrgbColor {
    SrgbColor {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    }
}

fn light_gray() -> SrgbColor {
    SrgbColor {
        r: 0.9,
        g: 0.9,
        b: 0.9,
    }
}

// ── WcagContrastFactory ───────────────────────────────────────────────────────

#[test]
fn contrast_minimum_passes_high_contrast() {
    let b = AccessKitUiBackend::new();
    let desc = ContrastDescriptor {
        foreground: black(),
        background: white(),
        widget: None,
    };
    let result = b.build_contrast_minimum(desc);
    assert!(result.is_ok(), "black on white should satisfy 4.5:1");
    let (pair, _proof) = result.unwrap();
    assert!(pair.ratio >= 4.5);
}

#[test]
fn contrast_minimum_rejects_low_contrast() {
    let b = AccessKitUiBackend::new();
    let desc = ContrastDescriptor {
        foreground: light_gray(),
        background: white(),
        widget: None,
    };
    let result = b.build_contrast_minimum(desc);
    assert!(result.is_err(), "light gray on white fails 4.5:1");
}

#[test]
fn contrast_minimum_large_passes_at_three_to_one() {
    let b = AccessKitUiBackend::new();
    // Dark gray on white comfortably beats 3:1.
    let dark_gray = SrgbColor {
        r: 0.4,
        g: 0.4,
        b: 0.4,
    };
    let desc = ContrastDescriptor {
        foreground: dark_gray,
        background: white(),
        widget: None,
    };
    let result = b.build_contrast_minimum_large(desc);
    assert!(
        result.is_ok(),
        "dark gray on white should satisfy 3:1 large text"
    );
}

#[test]
fn contrast_non_text_passes_at_three_to_one() {
    let b = AccessKitUiBackend::new();
    let dark = SrgbColor {
        r: 0.3,
        g: 0.3,
        b: 0.3,
    };
    let desc = ContrastDescriptor {
        foreground: dark,
        background: white(),
        widget: None,
    };
    assert!(b.build_non_text_contrast(desc).is_ok());
}

#[test]
fn contrast_enhanced_requires_seven_to_one() {
    let b = AccessKitUiBackend::new();
    // Mid-gray (0.5) on white is around 3.95:1 — passes 3:1 but not 7:1.
    let mid = SrgbColor {
        r: 0.5,
        g: 0.5,
        b: 0.5,
    };
    let desc = ContrastDescriptor {
        foreground: mid,
        background: white(),
        widget: None,
    };
    assert!(
        b.build_contrast_enhanced(desc).is_err(),
        "mid-gray on white fails 7:1"
    );
    let desc2 = ContrastDescriptor {
        foreground: black(),
        background: white(),
        widget: None,
    };
    assert!(
        b.build_contrast_enhanced(desc2).is_ok(),
        "black on white satisfies 7:1"
    );
}

// ── WcagContrastFactory — classify_large_text (SC 1.4.3) ─────────────────────

#[test]
fn large_text_passes_at_18pt_normal() {
    let b = AccessKitUiBackend::new();
    let desc = TextSizeDescriptor {
        font_size_pt: 18.0,
        bold: false,
        widget: None,
    };
    assert!(
        b.classify_large_text(desc).is_ok(),
        "18 pt normal is large text"
    );
}

#[test]
fn large_text_passes_at_14pt_bold() {
    let b = AccessKitUiBackend::new();
    let desc = TextSizeDescriptor {
        font_size_pt: 14.0,
        bold: true,
        widget: None,
    };
    assert!(
        b.classify_large_text(desc).is_ok(),
        "14 pt bold is large text"
    );
}

#[test]
fn large_text_passes_above_18pt() {
    let b = AccessKitUiBackend::new();
    let desc = TextSizeDescriptor {
        font_size_pt: 24.0,
        bold: false,
        widget: None,
    };
    assert!(
        b.classify_large_text(desc).is_ok(),
        "24 pt normal is large text"
    );
}

#[test]
fn large_text_rejects_12pt_normal() {
    let b = AccessKitUiBackend::new();
    let desc = TextSizeDescriptor {
        font_size_pt: 12.0,
        bold: false,
        widget: None,
    };
    assert!(
        b.classify_large_text(desc).is_err(),
        "12 pt normal is not large text"
    );
}

#[test]
fn large_text_rejects_13pt_bold() {
    let b = AccessKitUiBackend::new();
    let desc = TextSizeDescriptor {
        font_size_pt: 13.0,
        bold: true,
        widget: None,
    };
    assert!(
        b.classify_large_text(desc).is_err(),
        "13 pt bold does not reach 14 pt threshold"
    );
}

#[test]
fn large_text_rejects_17pt_normal() {
    let b = AccessKitUiBackend::new();
    // 17.9 pt is below the 18 pt threshold.
    let desc = TextSizeDescriptor {
        font_size_pt: 17.9,
        bold: false,
        widget: None,
    };
    assert!(
        b.classify_large_text(desc).is_err(),
        "17.9 pt normal does not meet 18 pt threshold"
    );
}

// ── WcagContrastFactory — nodeId traceability ────────────────────────────────

#[test]
fn contrast_proof_stored_on_widget_when_widget_set() {
    let b = AccessKitUiBackend::new();
    let btn = b
        .build_labeled_element(LabelDescriptor {
            name: "Submit".into(),
            role: "button".into(),
            labelled_by: None,
        })
        .unwrap()
        .0;
    let desc = ContrastDescriptor {
        foreground: black(),
        background: white(),
        widget: Some(btn.id),
    };
    b.build_contrast_minimum(desc).unwrap();
    let tree = b.to_verified_tree(Viewport::new(800, 600));
    let node_id = accesskit::NodeId(btn.id.0);
    let proofs = tree.node_proofs().get(&node_id).unwrap();
    assert!(
        proofs.contrast_normal.is_some(),
        "contrast proof should be auto-stored on the widget"
    );
}

#[test]
fn large_text_proof_stored_on_widget_when_widget_set() {
    let b = AccessKitUiBackend::new();
    let btn = b
        .build_labeled_element(LabelDescriptor {
            name: "Heading".into(),
            role: "heading".into(),
            labelled_by: None,
        })
        .unwrap()
        .0;
    let desc = TextSizeDescriptor {
        font_size_pt: 18.0,
        bold: false,
        widget: Some(btn.id),
    };
    b.classify_large_text(desc).unwrap();
    let tree = b.to_verified_tree(Viewport::new(800, 600));
    let node_id = accesskit::NodeId(btn.id.0);
    let proofs = tree.node_proofs().get(&node_id).unwrap();
    assert!(
        proofs.large_text.is_some(),
        "large_text proof should be auto-stored on the widget"
    );
}

// ── WcagStructureFactory — build_text_spacing (SC 1.4.12) ────────────────────

fn compliant_spacing(font_size_pt: f32) -> TextSpacingDescriptor {
    // Exactly at the SC 1.4.12 minimums.
    TextSpacingDescriptor {
        font_size_pt,
        line_height_pt: Some(1.5 * font_size_pt),
        letter_spacing_pt: Some(0.12 * font_size_pt),
        word_spacing_pt: Some(0.16 * font_size_pt),
        paragraph_spacing_pt: Some(2.0 * font_size_pt),
    }
}

#[test]
fn text_spacing_passes_at_minimum_thresholds() {
    let b = AccessKitUiBackend::new();
    let (spaced, _proof) = b.build_text_spacing(compliant_spacing(16.0)).unwrap();
    let _ = spaced.id;
}

#[test]
fn text_spacing_passes_above_minimum() {
    let b = AccessKitUiBackend::new();
    let desc = TextSpacingDescriptor {
        font_size_pt: 16.0,
        line_height_pt: Some(32.0),
        letter_spacing_pt: Some(4.0),
        word_spacing_pt: Some(6.0),
        paragraph_spacing_pt: Some(40.0),
    };
    assert!(b.build_text_spacing(desc).is_ok());
}

#[test]
fn text_spacing_rejects_line_height_below_threshold() {
    let b = AccessKitUiBackend::new();
    let mut desc = compliant_spacing(16.0);
    desc.line_height_pt = Some(16.0 * 1.4);
    assert!(b.build_text_spacing(desc).is_err());
}

#[test]
fn text_spacing_rejects_letter_spacing_below_threshold() {
    let b = AccessKitUiBackend::new();
    let mut desc = compliant_spacing(16.0);
    desc.letter_spacing_pt = Some(16.0 * 0.11);
    assert!(b.build_text_spacing(desc).is_err());
}

#[test]
fn text_spacing_rejects_word_spacing_below_threshold() {
    let b = AccessKitUiBackend::new();
    let mut desc = compliant_spacing(16.0);
    desc.word_spacing_pt = Some(16.0 * 0.15);
    assert!(b.build_text_spacing(desc).is_err());
}

#[test]
fn text_spacing_rejects_paragraph_spacing_below_threshold() {
    let b = AccessKitUiBackend::new();
    let mut desc = compliant_spacing(16.0);
    desc.paragraph_spacing_pt = Some(16.0 * 1.9);
    assert!(b.build_text_spacing(desc).is_err());
}

#[test]
fn text_spacing_rejects_missing_line_height() {
    let b = AccessKitUiBackend::new();
    let mut desc = compliant_spacing(16.0);
    desc.line_height_pt = None;
    assert!(b.build_text_spacing(desc).is_err());
}

#[test]
fn text_spacing_rejects_missing_letter_spacing() {
    let b = AccessKitUiBackend::new();
    let mut desc = compliant_spacing(16.0);
    desc.letter_spacing_pt = None;
    assert!(b.build_text_spacing(desc).is_err());
}

#[test]
fn text_spacing_rejects_missing_word_spacing() {
    let b = AccessKitUiBackend::new();
    let mut desc = compliant_spacing(16.0);
    desc.word_spacing_pt = None;
    assert!(b.build_text_spacing(desc).is_err());
}

#[test]
fn text_spacing_rejects_missing_paragraph_spacing() {
    let b = AccessKitUiBackend::new();
    let mut desc = compliant_spacing(16.0);
    desc.paragraph_spacing_pt = None;
    assert!(b.build_text_spacing(desc).is_err());
}

#[test]
fn text_spacing_rejects_zero_font_size() {
    let b = AccessKitUiBackend::new();
    let desc = TextSpacingDescriptor {
        font_size_pt: 0.0,
        line_height_pt: Some(0.0),
        letter_spacing_pt: Some(0.0),
        word_spacing_pt: Some(0.0),
        paragraph_spacing_pt: Some(0.0),
    };
    assert!(b.build_text_spacing(desc).is_err());
}

// ── WcagLabelFactory ──────────────────────────────────────────────────────────

#[test]
fn labeled_element_creates_node_with_name() {
    let b = AccessKitUiBackend::new();
    let desc = LabelDescriptor {
        name: "Submit".into(),
        role: "button".into(),
        labelled_by: None,
    };
    let (elem, _proof) = b.build_labeled_element(desc).unwrap();
    assert_eq!(elem.name, "Submit");
    let label = b.element_label(elem.id).unwrap();
    assert_eq!(label.as_deref(), Some("Submit"));
}

#[test]
fn labeled_element_rejects_empty_name() {
    let b = AccessKitUiBackend::new();
    let desc = LabelDescriptor {
        name: "".into(),
        role: "button".into(),
        labelled_by: None,
    };
    assert!(b.build_labeled_element(desc).is_err());
}

#[test]
fn labeled_form_field_rejects_non_form_role() {
    let b = AccessKitUiBackend::new();
    let desc = LabelDescriptor {
        name: "Name".into(),
        role: "heading".into(),
        labelled_by: None,
    };
    assert!(b.build_labeled_form_field(desc).is_err());
}

#[test]
fn label_in_name_sets_matching_value() {
    let b = AccessKitUiBackend::new();
    let desc = LabelDescriptor {
        name: "Search".into(),
        role: "button".into(),
        labelled_by: None,
    };
    let (elem, _proof) = b.build_label_in_name(desc).unwrap();
    let desc_val = b.element_description(elem.id).unwrap();
    assert_eq!(desc_val.as_deref(), Some("Search"));
}

// ── WcagFocusFactory ──────────────────────────────────────────────────────────

fn make_widget(b: &AccessKitUiBackend) -> WidgetId {
    let desc = LabelDescriptor {
        name: "Widget".into(),
        role: "button".into(),
        labelled_by: None,
    };
    b.build_labeled_element(desc).unwrap().0.id
}

#[test]
fn focus_visible_accepts_sufficient_contrast() {
    let b = AccessKitUiBackend::new();
    let id = make_widget(&b);
    let desc = FocusDescriptor {
        widget: id,
        indicator_area_px: 4.0,
        indicator_contrast: 3.5,
    };
    assert!(b.build_focus_visible(desc).is_ok());
}

#[test]
fn focus_visible_rejects_low_contrast() {
    let b = AccessKitUiBackend::new();
    let id = make_widget(&b);
    let desc = FocusDescriptor {
        widget: id,
        indicator_area_px: 4.0,
        indicator_contrast: 2.0,
    };
    assert!(b.build_focus_visible(desc).is_err());
}

#[test]
fn focus_appearance_minimum_rejects_zero_area() {
    let b = AccessKitUiBackend::new();
    let id = make_widget(&b);
    let desc = FocusDescriptor {
        widget: id,
        indicator_area_px: 0.0,
        indicator_contrast: 3.5,
    };
    assert!(b.build_focus_appearance_minimum(desc).is_err());
}

// ── WcagKeyboardFactory ───────────────────────────────────────────────────────

#[test]
fn keyboard_accessible_adds_to_focus_order() {
    let b = AccessKitUiBackend::new();
    let id = make_widget(&b);
    let desc = KeyboardDescriptor {
        widget: id,
        tab_index: 0,
        shortcut: None,
    };
    assert!(b.build_keyboard_accessible(desc).is_ok());
    let order = b.focus_order().unwrap();
    assert!(order.contains(&id));
}

#[test]
fn keyboard_escape_succeeds_for_known_widget() {
    let b = AccessKitUiBackend::new();
    let id = make_widget(&b);
    let desc = KeyboardDescriptor {
        widget: id,
        tab_index: 0,
        shortcut: None,
    };
    assert!(b.build_keyboard_escape(desc).is_ok());
}

// ── WcagTimingFactory ─────────────────────────────────────────────────────────

#[test]
fn timed_element_no_limit_always_passes() {
    let b = AccessKitUiBackend::new();
    let id = make_widget(&b);
    let desc = TimingDescriptor {
        element: id,
        max_seconds: None,
        can_pause: false,
        can_extend: false,
        can_turn_off: false,
    };
    assert!(b.build_timed_element(desc).is_ok());
}

#[test]
fn timed_element_limited_no_controls_fails() {
    let b = AccessKitUiBackend::new();
    let id = make_widget(&b);
    let desc = TimingDescriptor {
        element: id,
        max_seconds: Some(30),
        can_pause: false,
        can_extend: false,
        can_turn_off: false,
    };
    assert!(
        b.build_timed_element(desc).is_err(),
        "time limit with no controls must fail"
    );
}

#[test]
fn timed_element_limited_with_pause_passes() {
    let b = AccessKitUiBackend::new();
    let id = make_widget(&b);
    let desc = TimingDescriptor {
        element: id,
        max_seconds: Some(30),
        can_pause: true,
        can_extend: false,
        can_turn_off: false,
    };
    assert!(b.build_timed_element(desc).is_ok());
}

// ── WcagTargetFactory ─────────────────────────────────────────────────────────

#[test]
fn target_minimum_passes_24x24() {
    let b = AccessKitUiBackend::new();
    let id = make_widget(&b);
    let desc = TargetDescriptor {
        widget: id,
        width_px: 24.0,
        height_px: 24.0,
        adjacent_spacing_px: 0.0,
    };
    assert!(b.build_target_minimum(desc).is_ok());
}

#[test]
fn target_minimum_fails_small_no_spacing() {
    let b = AccessKitUiBackend::new();
    let id = make_widget(&b);
    let desc = TargetDescriptor {
        widget: id,
        width_px: 16.0,
        height_px: 16.0,
        adjacent_spacing_px: 0.0,
    };
    assert!(b.build_target_minimum(desc).is_err());
}

#[test]
fn target_minimum_small_with_sufficient_spacing_passes() {
    let b = AccessKitUiBackend::new();
    let id = make_widget(&b);
    // 16×16 with 8 px spacing: gap is 8 px, spacing covers it.
    let desc = TargetDescriptor {
        widget: id,
        width_px: 16.0,
        height_px: 16.0,
        adjacent_spacing_px: 8.0,
    };
    assert!(b.build_target_minimum(desc).is_ok());
}

#[test]
fn target_enhanced_requires_44x44() {
    let b = AccessKitUiBackend::new();
    let id = make_widget(&b);
    let small = TargetDescriptor {
        widget: id,
        width_px: 24.0,
        height_px: 24.0,
        adjacent_spacing_px: 0.0,
    };
    assert!(
        b.build_target_enhanced(small).is_err(),
        "24×24 fails enhanced 44×44"
    );
    let id2 = make_widget(&b);
    let large = TargetDescriptor {
        widget: id2,
        width_px: 44.0,
        height_px: 44.0,
        adjacent_spacing_px: 0.0,
    };
    assert!(b.build_target_enhanced(large).is_ok());
}

// ── WcagStructureFactory ──────────────────────────────────────────────────────

#[test]
fn build_heading_creates_heading_node() {
    let b = AccessKitUiBackend::new();
    let desc = StructureDescriptor {
        label: "Introduction".into(),
        role: "heading".into(),
        heading_level: Some(2),
    };
    let (elem, _proof) = b.build_heading(desc).unwrap();
    assert_eq!(elem.role, "heading");
    let headings = b.page_headings().unwrap();
    assert!(headings.contains(&elem.id));
}

#[test]
fn build_heading_rejects_empty_label() {
    let b = AccessKitUiBackend::new();
    let desc = StructureDescriptor {
        label: "".into(),
        role: "heading".into(),
        heading_level: Some(1),
    };
    assert!(b.build_heading(desc).is_err());
}

#[test]
fn build_table_rejects_empty_caption() {
    let b = AccessKitUiBackend::new();
    let desc = StructureDescriptor {
        label: "".into(),
        role: "table".into(),
        heading_level: None,
    };
    assert!(b.build_table(desc).is_err());
}

// ── WcagMediaFactory ──────────────────────────────────────────────────────────

#[test]
fn captioned_media_requires_captions() {
    let b = AccessKitUiBackend::new();
    let no_captions = MediaDescriptor {
        label: "Demo video".into(),
        has_captions: false,
        has_audio_description: false,
        has_transcript: false,
    };
    assert!(b.build_captioned_media(no_captions).is_err());

    let with_captions = MediaDescriptor {
        label: "Demo video".into(),
        has_captions: true,
        has_audio_description: false,
        has_transcript: false,
    };
    assert!(b.build_captioned_media(with_captions).is_ok());
}

#[test]
fn audio_described_media_requires_description() {
    let b = AccessKitUiBackend::new();
    let no_desc = MediaDescriptor {
        label: "Tutorial".into(),
        has_captions: true,
        has_audio_description: false,
        has_transcript: false,
    };
    assert!(b.build_audio_described_media(no_desc).is_err());
}

// ── WcagLanguageFactory ───────────────────────────────────────────────────────

#[test]
fn language_page_stores_lang_tag() {
    let b = AccessKitUiBackend::new();
    let desc = LanguageDescriptor {
        page_lang: "en".into(),
        element_lang: None,
        widget: None,
    };
    assert!(b.build_language_page(desc).is_ok());
    assert_eq!(b.page_language().unwrap().as_deref(), Some("en"));
}

#[test]
fn language_page_rejects_empty_tag() {
    let b = AccessKitUiBackend::new();
    let desc = LanguageDescriptor {
        page_lang: "".into(),
        element_lang: None,
        widget: None,
    };
    assert!(b.build_language_page(desc).is_err());
}

#[test]
fn language_element_requires_element_lang() {
    let b = AccessKitUiBackend::new();
    let desc = LanguageDescriptor {
        page_lang: "en".into(),
        element_lang: None,
        widget: None,
    };
    assert!(b.build_language_element(desc).is_err());
    let desc2 = LanguageDescriptor {
        page_lang: "en".into(),
        element_lang: Some("fr".into()),
        widget: None,
    };
    assert!(b.build_language_element(desc2).is_ok());
}

// ── WcagErrorFactory ──────────────────────────────────────────────────────────

#[test]
fn identified_error_requires_description() {
    let b = AccessKitUiBackend::new();
    let id = make_widget(&b);
    let no_text = ErrorDescriptor {
        widget: id,
        error_text: None,
        suggestion: None,
    };
    assert!(b.build_identified_error(no_text).is_err());

    let with_text = ErrorDescriptor {
        widget: id,
        error_text: Some("Field is required".into()),
        suggestion: None,
    };
    assert!(b.build_identified_error(with_text).is_ok());
}

#[test]
fn error_suggestion_requires_suggestion_text() {
    let b = AccessKitUiBackend::new();
    let id = make_widget(&b);
    let no_hint = ErrorDescriptor {
        widget: id,
        error_text: None,
        suggestion: None,
    };
    assert!(b.build_error_suggestion(no_hint).is_err());

    let with_hint = ErrorDescriptor {
        widget: id,
        error_text: None,
        suggestion: Some("Enter a valid email".into()),
    };
    assert!(b.build_error_suggestion(with_hint).is_ok());
}

// ── WcagElementMeta ───────────────────────────────────────────────────────────

#[test]
fn element_role_returns_role_string() {
    let b = AccessKitUiBackend::new();
    let id = make_widget(&b);
    let role = b.element_role(id).unwrap();
    assert!(role.is_some());
    let role_str = role.unwrap();
    assert!(role_str.contains("Button"), "unexpected role: {role_str}");
}

#[test]
fn element_has_focus_reports_first_in_order() {
    let b = AccessKitUiBackend::new();
    let id1 = make_widget(&b);
    let id2 = make_widget(&b);
    b.set_focus_order(vec![id1, id2]).unwrap();
    assert!(b.element_has_focus(id1).unwrap());
    assert!(!b.element_has_focus(id2).unwrap());
}

// ── WcagPageMeta ──────────────────────────────────────────────────────────────

#[test]
fn page_title_returns_root_label() {
    let b = AccessKitUiBackend::new();
    let title = b.page_title().unwrap();
    assert_eq!(title.as_deref(), Some("Application"));
}

#[test]
fn page_headings_tracks_headings() {
    let b = AccessKitUiBackend::new();
    assert_eq!(b.page_headings().unwrap().len(), 0);
    let desc = StructureDescriptor {
        label: "Chapter 1".into(),
        role: "heading".into(),
        heading_level: Some(1),
    };
    b.build_heading(desc).unwrap();
    assert_eq!(b.page_headings().unwrap().len(), 1);
}

// ── UiLayoutManager ───────────────────────────────────────────────────────────

#[test]
fn container_stack_returns_proof() {
    let b = AccessKitUiBackend::new();
    let id1 = make_widget(&b);
    let id2 = make_widget(&b);
    let result = b.container_stack("horizontal", vec![id1, id2]);
    assert!(result.is_ok());
}

#[test]
fn widget_count_tracks_additions() {
    let b = AccessKitUiBackend::new();
    let initial = b.widget_count();
    let desc1 = LabelDescriptor {
        name: "A".into(),
        role: "button".into(),
        labelled_by: None,
    };
    let desc2 = LabelDescriptor {
        name: "B".into(),
        role: "button".into(),
        labelled_by: None,
    };
    b.build_labeled_element(desc1).unwrap();
    b.build_labeled_element(desc2).unwrap();
    assert_eq!(b.widget_count(), initial + 2);
}
