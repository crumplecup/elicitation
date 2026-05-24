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
        ElicitComplete, ElicitPromptTree, ElicitSpec, PromptTree, Select, SpecCategoryBuilder,
        SpecEntryBuilder, TypeSpec, TypeSpecBuilder, TypeSpecInventoryKey,
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

            impl ElicitPromptTree for $ty {
                fn prompt_tree() -> PromptTree {
                    let labels = <$ty as Select>::labels();
                    let branch_count = labels.len();
                    PromptTree::Select {
                        prompt: $name.to_string(),
                        type_name: $name.to_string(),
                        options: labels,
                        branches: vec![None; branch_count],
                    }
                }
            }

            impl crate::emit_code::ToCodeLiteral for $ty {
                fn to_code_literal(&self) -> crate::proc_macro2::TokenStream {
                    let variant = format!("{self:?}");
                    let variant_tok: crate::proc_macro2::TokenStream =
                        variant.parse().expect("Debug output is valid Rust ident");
                    let type_path: crate::proc_macro2::TokenStream =
                        stringify!($ty).parse().expect("type path is valid tokens");
                    crate::quote::quote! { #type_path::#variant_tok }
                }

                fn type_tokens() -> crate::proc_macro2::TokenStream {
                    stringify!($ty).parse().expect("type path is valid tokens")
                }
            }
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

    // -------------------------------------------------------------------------
    // Macro: impl_accesskit_survey_spec!
    //
    // Derives ElicitSpec for an accesskit Survey struct. ElicitPromptTree and
    // ToCodeLiteral are already implemented in the primitives files; this macro
    // only adds ElicitSpec + inventory::submit!.
    // -------------------------------------------------------------------------

    macro_rules! impl_accesskit_survey_spec {
        (
            type    = $ty:ty,
            name    = $name:literal,
            summary = $summary:literal,
            fields  = [ $( ($field:literal, $desc:literal) ),+ $(,)? ]
        ) => {
            impl ElicitSpec for $ty {
                fn type_spec() -> TypeSpec {
                    let field_entries = vec![
                        $(
                            SpecEntryBuilder::default()
                                .label($field.to_string())
                                .description($desc.to_string())
                                .build()
                                .expect("valid SpecEntry"),
                        )+
                    ];
                    let fields = SpecCategoryBuilder::default()
                        .name("fields".to_string())
                        .entries(field_entries)
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
                                .description("Survey — elicit each field in sequence".to_string())
                                .build()
                                .expect("valid SpecEntry"),
                        ])
                        .build()
                        .expect("valid SpecCategory");
                    TypeSpecBuilder::default()
                        .type_name($name.to_string())
                        .summary($summary.to_string())
                        .categories(vec![fields, source])
                        .build()
                        .expect("valid TypeSpec")
                }
            }

            inventory::submit!(TypeSpecInventoryKey::new(
                $name,
                <$ty as ElicitSpec>::type_spec,
                std::any::TypeId::of::<$ty>
            ));
        };
    }

    impl_accesskit_survey_spec!(
        type    = accesskit::Point,
        name    = "accesskit::Point",
        summary = "A 2-D point with f64 x and y coordinates.",
        fields  = [
            ("x", "Horizontal coordinate"),
            ("y", "Vertical coordinate"),
        ]
    );

    impl_accesskit_survey_spec!(
        type    = accesskit::Vec2,
        name    = "accesskit::Vec2",
        summary = "A 2-D vector (offset) with f64 x and y components.",
        fields  = [
            ("x", "Horizontal component"),
            ("y", "Vertical component"),
        ]
    );

    impl_accesskit_survey_spec!(
        type    = accesskit::Size,
        name    = "accesskit::Size",
        summary = "A 2-D size with f64 width and height.",
        fields  = [
            ("width",  "Horizontal extent"),
            ("height", "Vertical extent"),
        ]
    );

    impl_accesskit_survey_spec!(
        type    = accesskit::Rect,
        name    = "accesskit::Rect",
        summary = "An axis-aligned rectangle defined by (x0, y0) top-left and (x1, y1) bottom-right.",
        fields  = [
            ("x0", "Left edge"),
            ("y0", "Top edge"),
            ("x1", "Right edge"),
            ("y1", "Bottom edge"),
        ]
    );

    impl_accesskit_survey_spec!(
        type    = accesskit::Affine,
        name    = "accesskit::Affine",
        summary = "A 2-D affine transform represented as a 3×3 matrix with 6 coefficients [a,b,c,d,e,f].",
        fields  = [
            ("a", "Scale X"),
            ("b", "Shear Y"),
            ("c", "Shear X"),
            ("d", "Scale Y"),
            ("e", "Translate X"),
            ("f", "Translate Y"),
        ]
    );

    impl_accesskit_survey_spec!(
        type    = accesskit::NodeId,
        name    = "accesskit::NodeId",
        summary = "A unique numeric identifier (u64) for a node in an accessibility tree.",
        fields  = [
            ("0", "The raw u64 node identifier"),
        ]
    );

    impl_accesskit_survey_spec!(
        type    = accesskit::Node,
        name    = "accesskit::Node",
        summary = "An accessibility tree node — elicited by specifying its Role; \
                   all other properties default to None.",
        fields  = [
            ("role", "The semantic role of this node (e.g. Button, TextField, List)"),
        ]
    );

    impl_accesskit_survey_spec!(
        type    = accesskit::TextDecoration,
        name    = "accesskit::TextDecoration",
        summary = "A text decoration (underline, strikethrough, etc.) with style and optional color.",
        fields  = [
            ("style", "Line style: Solid, Dotted, Dashed, Double, or Wavy"),
            ("color", "Optional RGBA color for the decoration line"),
        ]
    );

    impl_accesskit_survey_spec!(
        type    = accesskit::TextPosition,
        name    = "accesskit::TextPosition",
        summary = "A position within a text run: the node ID and the character index.",
        fields  = [
            ("node",            "The node containing the text"),
            ("character_index", "Zero-based character offset within the node's text"),
        ]
    );

    impl_accesskit_survey_spec!(
        type    = accesskit::TextSelection,
        name    = "accesskit::TextSelection",
        summary = "A text selection range: anchor (where selection started) and focus (current end).",
        fields  = [
            ("anchor", "Start position of the selection (TextPosition)"),
            ("focus",  "End / active position of the selection (TextPosition)"),
        ]
    );

    impl_accesskit_survey_spec!(
        type    = accesskit::CustomAction,
        name    = "accesskit::CustomAction",
        summary = "A custom action defined by the accessibility provider: \
                   a numeric ID and a human-readable description.",
        fields  = [
            ("id",          "Provider-defined integer action identifier"),
            ("description", "Human-readable name of the action"),
        ]
    );

    impl_accesskit_survey_spec!(
        type    = accesskit::Tree,
        name    = "accesskit::Tree",
        summary = "Top-level metadata for an accessibility tree: the root node ID and \
                   optional toolkit name / version strings.",
        fields  = [
            ("root",          "NodeId of the tree root"),
            ("toolkit_name",  "Optional name of the UI toolkit (e.g. \"egui\")"),
            ("toolkit_version", "Optional version string of the UI toolkit"),
        ]
    );

    impl_accesskit_survey_spec!(
        type    = accesskit::TreeUpdate,
        name    = "accesskit::TreeUpdate",
        summary = "An atomic update to an accessibility tree: a batch of nodes, \
                   optional tree metadata, tree UUID, and current focus.",
        fields  = [
            ("nodes",   "Vec of (NodeId, Node) pairs to add or update"),
            ("tree",    "Optional Tree metadata (required on first update)"),
            ("tree_id", "UUID identifying the tree (use TreeId::ROOT for the main tree)"),
            ("focus",   "NodeId of the currently focused node"),
        ]
    );

    impl_accesskit_survey_spec!(
        type    = accesskit::ActionRequest,
        name    = "accesskit::ActionRequest",
        summary = "A request to perform an accessibility action on a specific node \
                   in a specific tree, with optional action data payload.",
        fields  = [
            ("action",      "The Action variant to perform"),
            ("target_tree", "UUID of the tree containing the target node"),
            ("target_node", "NodeId of the node to act on"),
            ("data",        "Optional ActionData payload for the action"),
        ]
    );

    // ActionData: non-unit enum — ElicitSpec registered separately via select-style spec.
    impl ElicitSpec for accesskit::ActionData {
        fn type_spec() -> TypeSpec {
            let variants = SpecCategoryBuilder::default()
                .name("variants".to_string())
                .entries(vec![
                    SpecEntryBuilder::default()
                        .label("customAction".to_string())
                        .description(
                            "CustomAction(i32) — provider-defined action by ID".to_string(),
                        )
                        .build()
                        .expect("valid SpecEntry"),
                    SpecEntryBuilder::default()
                        .label("value".to_string())
                        .description("Value(Box<str>) — set a string value".to_string())
                        .build()
                        .expect("valid SpecEntry"),
                    SpecEntryBuilder::default()
                        .label("numericValue".to_string())
                        .description("NumericValue(f64) — set a numeric value".to_string())
                        .build()
                        .expect("valid SpecEntry"),
                    SpecEntryBuilder::default()
                        .label("scrollUnit".to_string())
                        .description("ScrollUnit(ScrollUnit) — scroll by unit".to_string())
                        .build()
                        .expect("valid SpecEntry"),
                    SpecEntryBuilder::default()
                        .label("scrollHint".to_string())
                        .description(
                            "ScrollHint(ScrollHint) — scroll-into-view hint position".to_string(),
                        )
                        .build()
                        .expect("valid SpecEntry"),
                    SpecEntryBuilder::default()
                        .label("scrollToPoint".to_string())
                        .description(
                            "ScrollToPoint(Point) — scroll to absolute position".to_string(),
                        )
                        .build()
                        .expect("valid SpecEntry"),
                    SpecEntryBuilder::default()
                        .label("setScrollOffset".to_string())
                        .description("SetScrollOffset(Point) — set the scroll offset".to_string())
                        .build()
                        .expect("valid SpecEntry"),
                    SpecEntryBuilder::default()
                        .label("setTextSelection".to_string())
                        .description(
                            "SetTextSelection(TextSelection) — set the text selection".to_string(),
                        )
                        .build()
                        .expect("valid SpecEntry"),
                ])
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
                            "Select — choose variant, then elicit its inner value".to_string(),
                        )
                        .build()
                        .expect("valid SpecEntry"),
                ])
                .build()
                .expect("valid SpecCategory");
            TypeSpecBuilder::default()
                .type_name("accesskit::ActionData".to_string())
                .summary(
                    "Action data payload: pick a variant (CustomAction, Value, NumericValue, \
                     ScrollUnit, ScrollHint, ScrollToPoint, SetScrollOffset, SetTextSelection) \
                     then supply the inner value."
                        .to_string(),
                )
                .categories(vec![variants, source])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "accesskit::ActionData",
        <accesskit::ActionData as ElicitSpec>::type_spec,
        std::any::TypeId::of::<accesskit::ActionData>
    ));

    // ElicitComplete for all 15 new accesskit ReadyNow types
    impl ElicitComplete for accesskit::Point {}
    impl ElicitComplete for accesskit::Vec2 {}
    impl ElicitComplete for accesskit::Size {}
    impl ElicitComplete for accesskit::Rect {}
    impl ElicitComplete for accesskit::Affine {}
    impl ElicitComplete for accesskit::NodeId {}
    impl ElicitComplete for accesskit::Node {}
    impl ElicitComplete for accesskit::TextDecoration {}
    impl ElicitComplete for accesskit::TextPosition {}
    impl ElicitComplete for accesskit::TextSelection {}
    impl ElicitComplete for accesskit::CustomAction {}
    impl ElicitComplete for accesskit::Tree {}
    impl ElicitComplete for accesskit::TreeUpdate {}
    impl ElicitComplete for accesskit::ActionRequest {}
    impl ElicitComplete for accesskit::ActionData {}
}
