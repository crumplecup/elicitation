//! [`ElicitSpec`](crate::ElicitSpec) implementations for accesskit type elicitation.
//!
//! Available with the `accesskit` feature.
//!
//! Complements the [`ElicitIntrospect`](crate::ElicitIntrospect) impls in
//! `primitives/accesskit_types/` — those describe *structure* (pattern, variants),
//! these describe *contracts and usage* browsable by agents via `describe_type`.

#[cfg(feature = "accesskit")]
mod accesskit_impls {
    use crate::{
        ElicitComplete, ElicitSpec, Select, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec,
        TypeSpecBuilder, TypeSpecInventoryKey,
    };

    // -------------------------------------------------------------------------
    // Macro: impl_accesskit_select_spec!
    //
    // Derives ElicitSpec for an accesskit Select enum using Select::labels()
    // at runtime (avoids enumerating all variants twice in source). The "source"
    // category is hardcoded to "accesskit v0.24".
    // -------------------------------------------------------------------------

    macro_rules! impl_accesskit_select_spec {
        (
            type    = $ty:ty,
            name    = $name:literal,
            summary = $summary:literal
        ) => {
            impl ElicitSpec for $ty {
                fn type_spec() -> TypeSpec {
                    let variants = SpecCategoryBuilder::default()
                        .name("variants".to_string())
                        .entries(
                            <$ty as Select>::labels()
                                .into_iter()
                                .map(|label| {
                                    SpecEntryBuilder::default()
                                        .label(label.clone())
                                        .description(label)
                                        .build()
                                        .expect("valid SpecEntry")
                                })
                                .collect(),
                        )
                        .build()
                        .expect("valid SpecCategory");
                    let source = SpecCategoryBuilder::default()
                        .name("source".to_string())
                        .entries(vec![
                            SpecEntryBuilder::default()
                                .label("crate".to_string())
                                .description(
                                    "accesskit v0.24 — cross-platform accessibility tree library"
                                        .to_string(),
                                )
                                .build()
                                .expect("valid SpecEntry"),
                            SpecEntryBuilder::default()
                                .label("pattern".to_string())
                                .description(
                                    "Select — choose one variant from the list".to_string(),
                                )
                                .build()
                                .expect("valid SpecEntry"),
                        ])
                        .build()
                        .expect("valid SpecCategory");
                    TypeSpecBuilder::default()
                        .type_name($name.to_string())
                        .summary($summary.to_string())
                        .categories(vec![variants, source])
                        .build()
                        .expect("valid TypeSpec")
                }
            }

            inventory::submit!(TypeSpecInventoryKey::new(
                $name,
                <$ty as ElicitSpec>::type_spec,
                std::any::TypeId::of::<$ty>
            ));

            impl ElicitComplete for $ty {}
        };
    }

    impl_accesskit_select_spec!(
        type    = accesskit::Role,
        name    = "accesskit::Role",
        summary = "The accessibility role of a node in the tree — describes its semantic \
                   purpose to assistive technologies (screen readers, automation tools)."
    );

    impl_accesskit_select_spec!(
        type    = accesskit::Action,
        name    = "accesskit::Action",
        summary = "An action that can be performed on an accessibility node, \
                   such as Click, Focus, SetValue, or ScrollIntoView."
    );

    impl_accesskit_select_spec!(
        type    = accesskit::Invalid,
        name    = "accesskit::Invalid",
        summary = "Indicates whether and how the current value of a node is invalid \
                   (False, True, Grammar, or Spelling)."
    );

    impl_accesskit_select_spec!(
        type    = accesskit::Toggled,
        name    = "accesskit::Toggled",
        summary = "The toggled state of a checkbox, switch, or similar control \
                   (False, True, or Mixed for indeterminate)."
    );

    impl_accesskit_select_spec!(
        type    = accesskit::Orientation,
        name    = "accesskit::Orientation",
        summary = "Whether a scrollbar, slider, or separator is oriented \
                   Horizontally or Vertically."
    );

    impl_accesskit_select_spec!(
        type    = accesskit::TextDirection,
        name    = "accesskit::TextDirection",
        summary = "The base text direction for a node: LeftToRight, RightToLeft, \
                   TopToBottom, or BottomToTop."
    );

    impl_accesskit_select_spec!(
        type    = accesskit::SortDirection,
        name    = "accesskit::SortDirection",
        summary = "The sort direction of a column header: Ascending, Descending, or Other."
    );

    impl_accesskit_select_spec!(
        type    = accesskit::AriaCurrent,
        name    = "accesskit::AriaCurrent",
        summary = "The aria-current attribute value indicating the current item within \
                   a set (False, True, Page, Step, Location, Date, Time)."
    );

    impl_accesskit_select_spec!(
        type    = accesskit::AutoComplete,
        name    = "accesskit::AutoComplete",
        summary = "How the autocomplete list for an input field is presented: \
                   Inline, List, or Both."
    );

    impl_accesskit_select_spec!(
        type    = accesskit::Live,
        name    = "accesskit::Live",
        summary = "The ARIA live region politeness: Off, Polite (waits for idle), \
                   or Assertive (interrupts immediately)."
    );

    impl_accesskit_select_spec!(
        type    = accesskit::HasPopup,
        name    = "accesskit::HasPopup",
        summary = "The type of popup a node can trigger: False (none), True (unspecified), \
                   Menu, ListBox, Tree, Grid, or Dialog."
    );

    impl_accesskit_select_spec!(
        type    = accesskit::ListStyle,
        name    = "accesskit::ListStyle",
        summary = "The visual marker style for list items: Circle, Disc, Image, \
                   Numeric, Square, or DefinitionList."
    );

    impl_accesskit_select_spec!(
        type    = accesskit::TextAlign,
        name    = "accesskit::TextAlign",
        summary = "Horizontal text alignment within a node: Left, Right, Center, or Justify."
    );

    impl_accesskit_select_spec!(
        type    = accesskit::VerticalOffset,
        name    = "accesskit::VerticalOffset",
        summary = "Vertical text offset for typographic effects: Subscript or Superscript."
    );

    impl_accesskit_select_spec!(
        type    = accesskit::TextDecorationStyle,
        name    = "accesskit::TextDecorationStyle",
        summary = "The line style used for text decorations (underline, strikethrough, etc.): \
                   Solid, Dotted, Dashed, Double, or Wavy."
    );

    impl_accesskit_select_spec!(
        type    = accesskit::ScrollUnit,
        name    = "accesskit::ScrollUnit",
        summary = "The unit for scroll actions: Page, Line, or Document."
    );

    impl_accesskit_select_spec!(
        type    = accesskit::ScrollHint,
        name    = "accesskit::ScrollHint",
        summary = "The preferred position of a node after ScrollIntoView: TopLeft, BottomRight, \
                   TopEdge, BottomEdge, LeftEdge, or RightEdge."
    );
}
