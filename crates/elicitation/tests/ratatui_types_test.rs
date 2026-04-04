//! Tests for ratatui Phase 2 — Elicitation, Select, ElicitIntrospect, ElicitSpec,
//! ElicitComplete implementations for ratatui types.
//!
//! Covers: Alignment, Direction, BorderType, Color, Borders (via BordersSelect),
//! ScrollbarOrientation (via ScrollbarOrientationSelect), RatatuiStyle,
//! RatatuiPadding, RatatuiMargin.

use elicitation::{
    AlignmentSelect, AlignmentStyle, BorderTypeSelect, BorderTypeStyle, BordersSelect, ColorSelect,
    ColorStyle, ElicitComplete, ElicitIntrospect, ElicitSpec, Elicitation, ElicitationPattern,
    RatatuiDirectionSelect, RatatuiDirectionStyle, RatatuiMargin, RatatuiMarginStyle,
    RatatuiPadding, RatatuiPaddingStyle, RatatuiStyle, RatatuiStyleStyle,
    ScrollbarOrientationSelect,
};

/// Compile-time proof that T satisfies all ElicitComplete bounds.
fn assert_elicit_complete<T: ElicitComplete>() {}

/// Assert all three proof methods return non-empty token streams.
#[track_caller]
fn assert_proofs_non_empty<T: Elicitation>(label: &str) {
    assert!(!T::kani_proof().is_empty(), "{label}: kani_proof is empty");
    assert!(
        !T::verus_proof().is_empty(),
        "{label}: verus_proof is empty"
    );
    assert!(
        !T::creusot_proof().is_empty(),
        "{label}: creusot_proof is empty"
    );
}

// ── Select trait: labels / from_label round-trips ─────────────────────────────

mod select_round_trips {
    use elicitation::Select;
    use ratatui::layout::{Alignment, Direction};
    use ratatui::style::Color;
    use ratatui::widgets::{BorderType, Borders, ScrollbarOrientation};

    #[test]
    fn alignment_labels_match_options() {
        let labels = Alignment::labels();
        let options = Alignment::options();
        assert_eq!(labels.len(), options.len());
        for (opt, label) in options.iter().zip(labels.iter()) {
            let round = Alignment::from_label(label).unwrap();
            assert_eq!(*opt, round);
        }
    }

    #[test]
    fn direction_labels_match_options() {
        let labels = Direction::labels();
        let options = Direction::options();
        assert_eq!(labels.len(), options.len());
        for (opt, label) in options.iter().zip(labels.iter()) {
            let round = Direction::from_label(label).unwrap();
            assert_eq!(*opt, round);
        }
    }

    #[test]
    fn border_type_labels_match_options() {
        let labels = BorderType::labels();
        let options = BorderType::options();
        assert_eq!(labels.len(), options.len());
        for (opt, label) in options.iter().zip(labels.iter()) {
            let round = BorderType::from_label(label).unwrap();
            assert_eq!(*opt, round);
        }
    }

    #[test]
    fn color_labels_match_options() {
        let labels = Color::labels();
        let options = Color::options();
        assert_eq!(labels.len(), options.len());
        for (opt, label) in options.iter().zip(labels.iter()) {
            let round = Color::from_label(label).unwrap();
            assert_eq!(*opt, round);
        }
    }

    #[test]
    fn borders_labels_match_options() {
        let labels = Borders::labels();
        let options = Borders::options();
        assert_eq!(labels.len(), options.len());
        for (opt, label) in options.iter().zip(labels.iter()) {
            let round = Borders::from_label(label).unwrap();
            assert_eq!(*opt, round);
        }
    }

    #[test]
    fn scrollbar_orientation_labels_match_options() {
        let labels = ScrollbarOrientation::labels();
        let options = ScrollbarOrientation::options();
        assert_eq!(labels.len(), options.len());
        for (opt, label) in options.iter().zip(labels.iter()) {
            let round = ScrollbarOrientation::from_label(label).unwrap();
            assert_eq!(*opt, round);
        }
    }

    #[test]
    fn alignment_from_label_unknown_is_none() {
        assert!(Alignment::from_label("Justify").is_none());
    }

    #[test]
    fn borders_from_label_unknown_is_none() {
        assert!(Borders::from_label("Diagonal").is_none());
    }
}

// ── ElicitIntrospect ──────────────────────────────────────────────────────────

mod introspect {
    use super::*;
    use ratatui::layout::{Alignment, Direction};
    use ratatui::style::Color;
    use ratatui::widgets::{BorderType, Borders, ScrollbarOrientation};

    #[test]
    fn alignment_is_select() {
        assert_eq!(Alignment::pattern(), ElicitationPattern::Select);
    }

    #[test]
    fn direction_is_select() {
        assert_eq!(Direction::pattern(), ElicitationPattern::Select);
    }

    #[test]
    fn border_type_is_select() {
        assert_eq!(BorderType::pattern(), ElicitationPattern::Select);
    }

    #[test]
    fn color_is_select() {
        assert_eq!(Color::pattern(), ElicitationPattern::Select);
    }

    #[test]
    fn borders_is_select() {
        assert_eq!(Borders::pattern(), ElicitationPattern::Select);
    }

    #[test]
    fn scrollbar_orientation_is_select() {
        assert_eq!(ScrollbarOrientation::pattern(), ElicitationPattern::Select);
    }

    #[test]
    fn borders_select_forwards_introspect() {
        assert_eq!(BordersSelect::pattern(), ElicitationPattern::Select);
        let meta = BordersSelect::metadata();
        assert_eq!(meta.type_name, "ratatui::widgets::Borders");
    }

    #[test]
    fn scrollbar_orientation_select_forwards_introspect() {
        assert_eq!(
            ScrollbarOrientationSelect::pattern(),
            ElicitationPattern::Select
        );
        let meta = ScrollbarOrientationSelect::metadata();
        assert_eq!(meta.type_name, "ratatui::widgets::ScrollbarOrientation");
    }

    #[test]
    fn ratatui_style_is_survey() {
        assert_eq!(RatatuiStyle::pattern(), ElicitationPattern::Survey);
        let meta = RatatuiStyle::metadata();
        assert_eq!(meta.type_name, "ratatui::style::Style");
    }

    #[test]
    fn ratatui_padding_is_survey() {
        assert_eq!(RatatuiPadding::pattern(), ElicitationPattern::Survey);
        let meta = RatatuiPadding::metadata();
        assert_eq!(meta.type_name, "ratatui::widgets::Padding");
    }

    #[test]
    fn ratatui_margin_is_survey() {
        assert_eq!(RatatuiMargin::pattern(), ElicitationPattern::Survey);
        let meta = RatatuiMargin::metadata();
        assert_eq!(meta.type_name, "ratatui::layout::Margin");
    }
}

// ── ElicitSpec (TypeSpec) ─────────────────────────────────────────────────────

mod specs {
    use super::*;
    use ratatui::layout::{Alignment, Direction};
    use ratatui::style::Color;
    use ratatui::widgets::{BorderType, Borders, ScrollbarOrientation};

    #[test]
    fn alignment_spec_has_variants() {
        let spec = <Alignment as ElicitSpec>::type_spec();
        assert_eq!(spec.type_name(), "ratatui::layout::Alignment");
        let variants = spec
            .categories()
            .iter()
            .find(|c| c.name() == "variants")
            .expect("should have variants category");
        assert_eq!(variants.entries().len(), 3);
    }

    #[test]
    fn direction_spec_has_variants() {
        let spec = <Direction as ElicitSpec>::type_spec();
        assert_eq!(spec.type_name(), "ratatui::layout::Direction");
        let variants = spec
            .categories()
            .iter()
            .find(|c| c.name() == "variants")
            .expect("should have variants category");
        assert_eq!(variants.entries().len(), 2);
    }

    #[test]
    fn border_type_spec_has_variants() {
        let spec = <BorderType as ElicitSpec>::type_spec();
        let variants = spec
            .categories()
            .iter()
            .find(|c| c.name() == "variants")
            .expect("should have variants category");
        assert_eq!(variants.entries().len(), 4);
    }

    #[test]
    fn color_spec_has_many_variants() {
        let spec = <Color as ElicitSpec>::type_spec();
        let variants = spec
            .categories()
            .iter()
            .find(|c| c.name() == "variants")
            .expect("should have variants category");
        // 17 named + Reset + RGB + Indexed sentinels = 20
        assert!(variants.entries().len() >= 17);
    }

    #[test]
    fn borders_spec_has_variants() {
        let spec = <Borders as ElicitSpec>::type_spec();
        let variants = spec
            .categories()
            .iter()
            .find(|c| c.name() == "variants")
            .expect("should have variants category");
        assert!(variants.entries().len() >= 6);
    }

    #[test]
    fn scrollbar_orientation_spec_has_variants() {
        let spec = <ScrollbarOrientation as ElicitSpec>::type_spec();
        let variants = spec
            .categories()
            .iter()
            .find(|c| c.name() == "variants")
            .expect("should have variants category");
        assert_eq!(variants.entries().len(), 4);
    }

    #[test]
    fn borders_select_spec_delegates() {
        let raw = <Borders as ElicitSpec>::type_spec();
        let wrapper = <BordersSelect as ElicitSpec>::type_spec();
        assert_eq!(raw.type_name(), wrapper.type_name());
    }

    #[test]
    fn scrollbar_select_spec_delegates() {
        let raw = <ScrollbarOrientation as ElicitSpec>::type_spec();
        let wrapper = <ScrollbarOrientationSelect as ElicitSpec>::type_spec();
        assert_eq!(raw.type_name(), wrapper.type_name());
    }

    #[test]
    fn ratatui_style_spec_has_fields() {
        let spec = <RatatuiStyle as ElicitSpec>::type_spec();
        assert_eq!(spec.type_name(), "ratatui::style::Style");
        let fields = spec
            .categories()
            .iter()
            .find(|c| c.name() == "fields")
            .expect("should have fields category");
        assert_eq!(fields.entries().len(), 5);
    }

    #[test]
    fn ratatui_padding_spec_has_fields() {
        let spec = <RatatuiPadding as ElicitSpec>::type_spec();
        assert_eq!(spec.type_name(), "ratatui::widgets::Padding");
        let fields = spec
            .categories()
            .iter()
            .find(|c| c.name() == "fields")
            .expect("should have fields category");
        assert_eq!(fields.entries().len(), 4);
    }

    #[test]
    fn ratatui_margin_spec_has_fields() {
        let spec = <RatatuiMargin as ElicitSpec>::type_spec();
        assert_eq!(spec.type_name(), "ratatui::layout::Margin");
        let fields = spec
            .categories()
            .iter()
            .find(|c| c.name() == "fields")
            .expect("should have fields category");
        assert_eq!(fields.entries().len(), 2);
    }

    #[test]
    fn all_specs_have_source_category() {
        for spec in [
            <Alignment as ElicitSpec>::type_spec(),
            <Direction as ElicitSpec>::type_spec(),
            <BorderType as ElicitSpec>::type_spec(),
            <Color as ElicitSpec>::type_spec(),
            <Borders as ElicitSpec>::type_spec(),
            <ScrollbarOrientation as ElicitSpec>::type_spec(),
            <RatatuiStyle as ElicitSpec>::type_spec(),
            <RatatuiPadding as ElicitSpec>::type_spec(),
            <RatatuiMargin as ElicitSpec>::type_spec(),
        ] {
            let source = spec
                .categories()
                .iter()
                .find(|c| c.name() == "source")
                .unwrap_or_else(|| panic!("{} missing source category", spec.type_name()));
            let crate_entry = source
                .entries()
                .iter()
                .find(|e| e.label() == "crate")
                .unwrap_or_else(|| panic!("{} missing crate entry in source", spec.type_name()));
            assert!(
                crate_entry.description().contains("ratatui"),
                "{} source should mention ratatui",
                spec.type_name()
            );
        }
    }
}

// ── Composite struct conversions ──────────────────────────────────────────────

mod conversions {
    use super::*;
    use ratatui::layout::Margin;
    use ratatui::style::{Modifier, Style};
    use ratatui::widgets::Padding;

    #[test]
    fn ratatui_style_round_trip() {
        let style = Style::default()
            .fg(ratatui::style::Color::Red)
            .bg(ratatui::style::Color::Blue)
            .add_modifier(Modifier::BOLD | Modifier::ITALIC);
        let wrapper = RatatuiStyle::from(style);
        assert_eq!(wrapper.fg.as_deref(), Some("Red"));
        assert_eq!(wrapper.bg.as_deref(), Some("Blue"));
        assert!(wrapper.bold);
        assert!(wrapper.italic);
        assert!(!wrapper.underlined);

        let back: Style = wrapper.try_into().expect("valid style");
        assert_eq!(back.fg, Some(ratatui::style::Color::Red));
        assert_eq!(back.bg, Some(ratatui::style::Color::Blue));
        assert!(back.add_modifier.contains(Modifier::BOLD));
        assert!(back.add_modifier.contains(Modifier::ITALIC));
    }

    #[test]
    fn ratatui_style_default_round_trip() {
        let wrapper = RatatuiStyle::from(Style::default());
        assert!(wrapper.fg.is_none());
        assert!(wrapper.bg.is_none());
        assert!(!wrapper.bold);
        let back: Style = wrapper.try_into().expect("valid style");
        assert_eq!(back, Style::default());
    }

    #[test]
    fn ratatui_padding_round_trip() {
        let pad = Padding::new(1, 2, 3, 4);
        let wrapper = RatatuiPadding::from(pad);
        assert_eq!(wrapper.left, 1);
        assert_eq!(wrapper.right, 2);
        assert_eq!(wrapper.top, 3);
        assert_eq!(wrapper.bottom, 4);
        let back: Padding = wrapper.into();
        assert_eq!(back.left, 1);
        assert_eq!(back.right, 2);
        assert_eq!(back.top, 3);
        assert_eq!(back.bottom, 4);
    }

    #[test]
    fn ratatui_margin_round_trip() {
        let margin = Margin::new(5, 10);
        let wrapper = RatatuiMargin::from(margin);
        assert_eq!(wrapper.horizontal, 5);
        assert_eq!(wrapper.vertical, 10);
        let back: Margin = wrapper.into();
        assert_eq!(back.horizontal, 5);
        assert_eq!(back.vertical, 10);
    }
}

// ── Serde round-trips for wrappers ────────────────────────────────────────────

mod serde_round_trips {
    use super::*;

    #[test]
    fn borders_select_serde_round_trip() {
        use ratatui::widgets::Borders;
        let select = BordersSelect::from(Borders::ALL);
        let json = serde_json::to_string(&select).expect("serialize");
        let back: BordersSelect = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(select, back);
    }

    #[test]
    fn ratatui_style_serde_round_trip() {
        let style = RatatuiStyle {
            fg: Some("Red".to_string()),
            bg: None,
            bold: true,
            italic: false,
            underlined: true,
        };
        let json = serde_json::to_string(&style).expect("serialize");
        let back: RatatuiStyle = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(style, back);
    }

    #[test]
    fn ratatui_padding_serde_round_trip() {
        let pad = RatatuiPadding {
            left: 1,
            right: 2,
            top: 3,
            bottom: 4,
        };
        let json = serde_json::to_string(&pad).expect("serialize");
        let back: RatatuiPadding = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(pad, back);
    }

    #[test]
    fn ratatui_margin_serde_round_trip() {
        let margin = RatatuiMargin {
            horizontal: 5,
            vertical: 10,
        };
        let json = serde_json::to_string(&margin).expect("serialize");
        let back: RatatuiMargin = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(margin, back);
    }
}

// ── Style type associations ───────────────────────────────────────────────────

mod style_types {
    use super::*;

    #[test]
    fn alignment_style_is_unit() {
        assert_eq!(AlignmentStyle::default(), AlignmentStyle::Default);
    }

    #[test]
    fn border_type_style_is_unit() {
        assert_eq!(BorderTypeStyle::default(), BorderTypeStyle::Default);
    }

    #[test]
    fn color_style_is_unit() {
        assert_eq!(ColorStyle::default(), ColorStyle::Default);
    }

    #[test]
    fn direction_style_is_unit() {
        assert_eq!(
            RatatuiDirectionStyle::default(),
            RatatuiDirectionStyle::Default
        );
    }

    #[test]
    fn ratatui_style_style_is_unit() {
        assert_eq!(RatatuiStyleStyle::default(), RatatuiStyleStyle::Default);
    }

    #[test]
    fn ratatui_padding_style_is_unit() {
        assert_eq!(RatatuiPaddingStyle::default(), RatatuiPaddingStyle::Default);
    }

    #[test]
    fn ratatui_margin_style_is_unit() {
        assert_eq!(RatatuiMarginStyle::default(), RatatuiMarginStyle::Default);
    }
}

// ── ElicitComplete (compiler-verified bound satisfaction) ──────────────────────

mod elicit_complete {
    use super::*;

    #[test]
    fn all_ratatui_wrappers_are_elicit_complete() {
        // Select wrappers
        assert_elicit_complete::<AlignmentSelect>();
        assert_elicit_complete::<RatatuiDirectionSelect>();
        assert_elicit_complete::<BorderTypeSelect>();
        assert_elicit_complete::<ColorSelect>();
        assert_elicit_complete::<BordersSelect>();
        assert_elicit_complete::<ScrollbarOrientationSelect>();

        // Composite wrappers
        assert_elicit_complete::<RatatuiStyle>();
        assert_elicit_complete::<RatatuiPadding>();
        assert_elicit_complete::<RatatuiMargin>();
    }
}

// ── Proof coverage (non-empty token streams) ──────────────────────────────────

mod proof_coverage {
    use super::*;
    use ratatui::layout::{Alignment, Direction};
    use ratatui::style::Color;
    use ratatui::widgets::{BorderType, Borders, ScrollbarOrientation};

    #[test]
    fn raw_type_proofs_non_empty() {
        assert_proofs_non_empty::<Alignment>("Alignment");
        assert_proofs_non_empty::<Direction>("Direction");
        assert_proofs_non_empty::<BorderType>("BorderType");
        assert_proofs_non_empty::<Color>("Color");
        assert_proofs_non_empty::<Borders>("Borders");
        assert_proofs_non_empty::<ScrollbarOrientation>("ScrollbarOrientation");
    }

    #[test]
    fn wrapper_proofs_non_empty() {
        assert_proofs_non_empty::<AlignmentSelect>("AlignmentSelect");
        assert_proofs_non_empty::<RatatuiDirectionSelect>("RatatuiDirectionSelect");
        assert_proofs_non_empty::<BorderTypeSelect>("BorderTypeSelect");
        assert_proofs_non_empty::<ColorSelect>("ColorSelect");
        assert_proofs_non_empty::<BordersSelect>("BordersSelect");
        assert_proofs_non_empty::<ScrollbarOrientationSelect>("ScrollbarOrientationSelect");
    }

    #[test]
    fn composite_proofs_non_empty() {
        assert_proofs_non_empty::<RatatuiStyle>("RatatuiStyle");
        assert_proofs_non_empty::<RatatuiPadding>("RatatuiPadding");
        assert_proofs_non_empty::<RatatuiMargin>("RatatuiMargin");
    }
}
