//! Per-role proof tokens for the [`crate::UiNodeBridge`] chain of custody.
//
//! Each `XxxNodeValid` type is a structural proposition meaning:
//! "this [`accesskit::Node`] has been validated for the `Xxx` role and
//! is legal to pass to `bridge_xxx`".
//
//! Tokens are issued exclusively by `dispatch_role`, which is only reachable
//! after holding an `Established<WcagVerified>`.

mod emit_impls {
    use crate::contracts::ui::{NodeRoleProof, WcagVerified};
    use elicitation::contracts::{Established, Prop, ProvableFrom};
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    macro_rules! structural_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural: $name — node role validated */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural: $name */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural: $name */ }
                }
            }
            impl NodeRoleProof for $t {}
            impl ProvableFrom<Established<WcagVerified>> for $t {}
        };
    }

    /// Proof that an [`accesskit::Node`] is valid for the `Unknown` role.
    pub struct UnknownNodeValid;
    structural_prop!(UnknownNodeValid, "UnknownNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `GenericContainer` role.
    pub struct GenericContainerNodeValid;
    structural_prop!(GenericContainerNodeValid, "GenericContainerNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Pane` role.
    pub struct PaneNodeValid;
    structural_prop!(PaneNodeValid, "PaneNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Window` role.
    pub struct WindowNodeValid;
    structural_prop!(WindowNodeValid, "WindowNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Document` role.
    pub struct DocumentNodeValid;
    structural_prop!(DocumentNodeValid, "DocumentNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `RootWebArea` role.
    pub struct RootWebAreaNodeValid;
    structural_prop!(RootWebAreaNodeValid, "RootWebAreaNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Application` role.
    pub struct ApplicationNodeValid;
    structural_prop!(ApplicationNodeValid, "ApplicationNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Terminal` role.
    pub struct TerminalNodeValid;
    structural_prop!(TerminalNodeValid, "TerminalNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Button` role.
    pub struct ButtonNodeValid;
    structural_prop!(ButtonNodeValid, "ButtonNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DefaultButton` role.
    pub struct DefaultButtonNodeValid;
    structural_prop!(DefaultButtonNodeValid, "DefaultButtonNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Link` role.
    pub struct LinkNodeValid;
    structural_prop!(LinkNodeValid, "LinkNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `CheckBox` role.
    pub struct CheckBoxNodeValid;
    structural_prop!(CheckBoxNodeValid, "CheckBoxNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `RadioButton` role.
    pub struct RadioButtonNodeValid;
    structural_prop!(RadioButtonNodeValid, "RadioButtonNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Switch` role.
    pub struct SwitchNodeValid;
    structural_prop!(SwitchNodeValid, "SwitchNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `ColorWell` role.
    pub struct ColorWellNodeValid;
    structural_prop!(ColorWellNodeValid, "ColorWellNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DisclosureTriangle` role.
    pub struct DisclosureTriangleNodeValid;
    structural_prop!(DisclosureTriangleNodeValid, "DisclosureTriangleNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `ComboBox` role.
    pub struct ComboBoxNodeValid;
    structural_prop!(ComboBoxNodeValid, "ComboBoxNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `EditableComboBox` role.
    pub struct EditableComboBoxNodeValid;
    structural_prop!(EditableComboBoxNodeValid, "EditableComboBoxNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `ListBox` role.
    pub struct ListBoxNodeValid;
    structural_prop!(ListBoxNodeValid, "ListBoxNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Slider` role.
    pub struct SliderNodeValid;
    structural_prop!(SliderNodeValid, "SliderNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `SpinButton` role.
    pub struct SpinButtonNodeValid;
    structural_prop!(SpinButtonNodeValid, "SpinButtonNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `ProgressIndicator` role.
    pub struct ProgressIndicatorNodeValid;
    structural_prop!(ProgressIndicatorNodeValid, "ProgressIndicatorNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `ScrollBar` role.
    pub struct ScrollBarNodeValid;
    structural_prop!(ScrollBarNodeValid, "ScrollBarNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `ScrollView` role.
    pub struct ScrollViewNodeValid;
    structural_prop!(ScrollViewNodeValid, "ScrollViewNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Splitter` role.
    pub struct SplitterNodeValid;
    structural_prop!(SplitterNodeValid, "SplitterNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `TextInput` role.
    pub struct TextInputNodeValid;
    structural_prop!(TextInputNodeValid, "TextInputNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `MultilineTextInput` role.
    pub struct MultilineTextInputNodeValid;
    structural_prop!(MultilineTextInputNodeValid, "MultilineTextInputNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `SearchInput` role.
    pub struct SearchInputNodeValid;
    structural_prop!(SearchInputNodeValid, "SearchInputNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DateInput` role.
    pub struct DateInputNodeValid;
    structural_prop!(DateInputNodeValid, "DateInputNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DateTimeInput` role.
    pub struct DateTimeInputNodeValid;
    structural_prop!(DateTimeInputNodeValid, "DateTimeInputNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `WeekInput` role.
    pub struct WeekInputNodeValid;
    structural_prop!(WeekInputNodeValid, "WeekInputNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `MonthInput` role.
    pub struct MonthInputNodeValid;
    structural_prop!(MonthInputNodeValid, "MonthInputNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `TimeInput` role.
    pub struct TimeInputNodeValid;
    structural_prop!(TimeInputNodeValid, "TimeInputNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `EmailInput` role.
    pub struct EmailInputNodeValid;
    structural_prop!(EmailInputNodeValid, "EmailInputNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `NumberInput` role.
    pub struct NumberInputNodeValid;
    structural_prop!(NumberInputNodeValid, "NumberInputNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `PasswordInput` role.
    pub struct PasswordInputNodeValid;
    structural_prop!(PasswordInputNodeValid, "PasswordInputNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `PhoneNumberInput` role.
    pub struct PhoneNumberInputNodeValid;
    structural_prop!(PhoneNumberInputNodeValid, "PhoneNumberInputNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `UrlInput` role.
    pub struct UrlInputNodeValid;
    structural_prop!(UrlInputNodeValid, "UrlInputNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `TextRun` role.
    pub struct TextRunNodeValid;
    structural_prop!(TextRunNodeValid, "TextRunNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Paragraph` role.
    pub struct ParagraphNodeValid;
    structural_prop!(ParagraphNodeValid, "ParagraphNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Label` role.
    pub struct LabelNodeValid;
    structural_prop!(LabelNodeValid, "LabelNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Heading` role.
    pub struct HeadingNodeValid;
    structural_prop!(HeadingNodeValid, "HeadingNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `LineBreak` role.
    pub struct LineBreakNodeValid;
    structural_prop!(LineBreakNodeValid, "LineBreakNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Blockquote` role.
    pub struct BlockquoteNodeValid;
    structural_prop!(BlockquoteNodeValid, "BlockquoteNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Code` role.
    pub struct CodeNodeValid;
    structural_prop!(CodeNodeValid, "CodeNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Math` role.
    pub struct MathNodeValid;
    structural_prop!(MathNodeValid, "MathNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Note` role.
    pub struct NoteNodeValid;
    structural_prop!(NoteNodeValid, "NoteNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Term` role.
    pub struct TermNodeValid;
    structural_prop!(TermNodeValid, "TermNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Definition` role.
    pub struct DefinitionNodeValid;
    structural_prop!(DefinitionNodeValid, "DefinitionNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Abbr` role.
    pub struct AbbrNodeValid;
    structural_prop!(AbbrNodeValid, "AbbrNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Emphasis` role.
    pub struct EmphasisNodeValid;
    structural_prop!(EmphasisNodeValid, "EmphasisNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Strong` role.
    pub struct StrongNodeValid;
    structural_prop!(StrongNodeValid, "StrongNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Mark` role.
    pub struct MarkNodeValid;
    structural_prop!(MarkNodeValid, "MarkNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Time` role.
    pub struct TimeNodeValid;
    structural_prop!(TimeNodeValid, "TimeNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Ruby` role.
    pub struct RubyNodeValid;
    structural_prop!(RubyNodeValid, "RubyNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `RubyAnnotation` role.
    pub struct RubyAnnotationNodeValid;
    structural_prop!(RubyAnnotationNodeValid, "RubyAnnotationNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Suggestion` role.
    pub struct SuggestionNodeValid;
    structural_prop!(SuggestionNodeValid, "SuggestionNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Comment` role.
    pub struct CommentNodeValid;
    structural_prop!(CommentNodeValid, "CommentNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `ContentDeletion` role.
    pub struct ContentDeletionNodeValid;
    structural_prop!(ContentDeletionNodeValid, "ContentDeletionNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `ContentInsertion` role.
    pub struct ContentInsertionNodeValid;
    structural_prop!(ContentInsertionNodeValid, "ContentInsertionNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Legend` role.
    pub struct LegendNodeValid;
    structural_prop!(LegendNodeValid, "LegendNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Image` role.
    pub struct ImageNodeValid;
    structural_prop!(ImageNodeValid, "ImageNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Figure` role.
    pub struct FigureNodeValid;
    structural_prop!(FigureNodeValid, "FigureNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `FigureCaption` role.
    pub struct FigureCaptionNodeValid;
    structural_prop!(FigureCaptionNodeValid, "FigureCaptionNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Canvas` role.
    pub struct CanvasNodeValid;
    structural_prop!(CanvasNodeValid, "CanvasNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Video` role.
    pub struct VideoNodeValid;
    structural_prop!(VideoNodeValid, "VideoNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Audio` role.
    pub struct AudioNodeValid;
    structural_prop!(AudioNodeValid, "AudioNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `SvgRoot` role.
    pub struct SvgRootNodeValid;
    structural_prop!(SvgRootNodeValid, "SvgRootNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `EmbeddedObject` role.
    pub struct EmbeddedObjectNodeValid;
    structural_prop!(EmbeddedObjectNodeValid, "EmbeddedObjectNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `PluginObject` role.
    pub struct PluginObjectNodeValid;
    structural_prop!(PluginObjectNodeValid, "PluginObjectNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `WebView` role.
    pub struct WebViewNodeValid;
    structural_prop!(WebViewNodeValid, "WebViewNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Iframe` role.
    pub struct IframeNodeValid;
    structural_prop!(IframeNodeValid, "IframeNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `IframePresentational` role.
    pub struct IframePresentationalNodeValid;
    structural_prop!(
        IframePresentationalNodeValid,
        "IframePresentationalNodeValid"
    );

    /// Proof that an [`accesskit::Node`] is valid for the `Main` role.
    pub struct MainNodeValid;
    structural_prop!(MainNodeValid, "MainNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Navigation` role.
    pub struct NavigationNodeValid;
    structural_prop!(NavigationNodeValid, "NavigationNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Banner` role.
    pub struct BannerNodeValid;
    structural_prop!(BannerNodeValid, "BannerNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `ContentInfo` role.
    pub struct ContentInfoNodeValid;
    structural_prop!(ContentInfoNodeValid, "ContentInfoNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Complementary` role.
    pub struct ComplementaryNodeValid;
    structural_prop!(ComplementaryNodeValid, "ComplementaryNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Form` role.
    pub struct FormNodeValid;
    structural_prop!(FormNodeValid, "FormNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Search` role.
    pub struct SearchNodeValid;
    structural_prop!(SearchNodeValid, "SearchNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Region` role.
    pub struct RegionNodeValid;
    structural_prop!(RegionNodeValid, "RegionNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Section` role.
    pub struct SectionNodeValid;
    structural_prop!(SectionNodeValid, "SectionNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `SectionHeader` role.
    pub struct SectionHeaderNodeValid;
    structural_prop!(SectionHeaderNodeValid, "SectionHeaderNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `SectionFooter` role.
    pub struct SectionFooterNodeValid;
    structural_prop!(SectionFooterNodeValid, "SectionFooterNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Header` role.
    pub struct HeaderNodeValid;
    structural_prop!(HeaderNodeValid, "HeaderNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Footer` role.
    pub struct FooterNodeValid;
    structural_prop!(FooterNodeValid, "FooterNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Article` role.
    pub struct ArticleNodeValid;
    structural_prop!(ArticleNodeValid, "ArticleNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Group` role.
    pub struct GroupNodeValid;
    structural_prop!(GroupNodeValid, "GroupNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Dialog` role.
    pub struct DialogNodeValid;
    structural_prop!(DialogNodeValid, "DialogNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `AlertDialog` role.
    pub struct AlertDialogNodeValid;
    structural_prop!(AlertDialogNodeValid, "AlertDialogNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Details` role.
    pub struct DetailsNodeValid;
    structural_prop!(DetailsNodeValid, "DetailsNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Tooltip` role.
    pub struct TooltipNodeValid;
    structural_prop!(TooltipNodeValid, "TooltipNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Alert` role.
    pub struct AlertNodeValid;
    structural_prop!(AlertNodeValid, "AlertNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Status` role.
    pub struct StatusNodeValid;
    structural_prop!(StatusNodeValid, "StatusNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Log` role.
    pub struct LogNodeValid;
    structural_prop!(LogNodeValid, "LogNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Marquee` role.
    pub struct MarqueeNodeValid;
    structural_prop!(MarqueeNodeValid, "MarqueeNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Timer` role.
    pub struct TimerNodeValid;
    structural_prop!(TimerNodeValid, "TimerNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `List` role.
    pub struct ListNodeValid;
    structural_prop!(ListNodeValid, "ListNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `ListItem` role.
    pub struct ListItemNodeValid;
    structural_prop!(ListItemNodeValid, "ListItemNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `ListMarker` role.
    pub struct ListMarkerNodeValid;
    structural_prop!(ListMarkerNodeValid, "ListMarkerNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DescriptionList` role.
    pub struct DescriptionListNodeValid;
    structural_prop!(DescriptionListNodeValid, "DescriptionListNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Feed` role.
    pub struct FeedNodeValid;
    structural_prop!(FeedNodeValid, "FeedNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `ListBoxOption` role.
    pub struct ListBoxOptionNodeValid;
    structural_prop!(ListBoxOptionNodeValid, "ListBoxOptionNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Table` role.
    pub struct TableNodeValid;
    structural_prop!(TableNodeValid, "TableNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Row` role.
    pub struct RowNodeValid;
    structural_prop!(RowNodeValid, "RowNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Cell` role.
    pub struct CellNodeValid;
    structural_prop!(CellNodeValid, "CellNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Caption` role.
    pub struct CaptionNodeValid;
    structural_prop!(CaptionNodeValid, "CaptionNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `RowGroup` role.
    pub struct RowGroupNodeValid;
    structural_prop!(RowGroupNodeValid, "RowGroupNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `RowHeader` role.
    pub struct RowHeaderNodeValid;
    structural_prop!(RowHeaderNodeValid, "RowHeaderNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `ColumnHeader` role.
    pub struct ColumnHeaderNodeValid;
    structural_prop!(ColumnHeaderNodeValid, "ColumnHeaderNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Grid` role.
    pub struct GridNodeValid;
    structural_prop!(GridNodeValid, "GridNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `GridCell` role.
    pub struct GridCellNodeValid;
    structural_prop!(GridCellNodeValid, "GridCellNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `TreeGrid` role.
    pub struct TreeGridNodeValid;
    structural_prop!(TreeGridNodeValid, "TreeGridNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `ListGrid` role.
    pub struct ListGridNodeValid;
    structural_prop!(ListGridNodeValid, "ListGridNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `LayoutTable` role.
    pub struct LayoutTableNodeValid;
    structural_prop!(LayoutTableNodeValid, "LayoutTableNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `LayoutTableRow` role.
    pub struct LayoutTableRowNodeValid;
    structural_prop!(LayoutTableRowNodeValid, "LayoutTableRowNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `LayoutTableCell` role.
    pub struct LayoutTableCellNodeValid;
    structural_prop!(LayoutTableCellNodeValid, "LayoutTableCellNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Tree` role.
    pub struct TreeNodeValid;
    structural_prop!(TreeNodeValid, "TreeNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `TreeItem` role.
    pub struct TreeItemNodeValid;
    structural_prop!(TreeItemNodeValid, "TreeItemNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Tab` role.
    pub struct TabNodeValid;
    structural_prop!(TabNodeValid, "TabNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `TabList` role.
    pub struct TabListNodeValid;
    structural_prop!(TabListNodeValid, "TabListNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `TabPanel` role.
    pub struct TabPanelNodeValid;
    structural_prop!(TabPanelNodeValid, "TabPanelNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Menu` role.
    pub struct MenuNodeValid;
    structural_prop!(MenuNodeValid, "MenuNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `MenuBar` role.
    pub struct MenuBarNodeValid;
    structural_prop!(MenuBarNodeValid, "MenuBarNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `MenuItem` role.
    pub struct MenuItemNodeValid;
    structural_prop!(MenuItemNodeValid, "MenuItemNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `MenuListOption` role.
    pub struct MenuListOptionNodeValid;
    structural_prop!(MenuListOptionNodeValid, "MenuListOptionNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `MenuListPopup` role.
    pub struct MenuListPopupNodeValid;
    structural_prop!(MenuListPopupNodeValid, "MenuListPopupNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `MenuItemCheckBox` role.
    pub struct MenuItemCheckBoxNodeValid;
    structural_prop!(MenuItemCheckBoxNodeValid, "MenuItemCheckBoxNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `MenuItemRadio` role.
    pub struct MenuItemRadioNodeValid;
    structural_prop!(MenuItemRadioNodeValid, "MenuItemRadioNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Toolbar` role.
    pub struct ToolbarNodeValid;
    structural_prop!(ToolbarNodeValid, "ToolbarNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `TitleBar` role.
    pub struct TitleBarNodeValid;
    structural_prop!(TitleBarNodeValid, "TitleBarNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `RadioGroup` role.
    pub struct RadioGroupNodeValid;
    structural_prop!(RadioGroupNodeValid, "RadioGroupNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Meter` role.
    pub struct MeterNodeValid;
    structural_prop!(MeterNodeValid, "MeterNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Keyboard` role.
    pub struct KeyboardNodeValid;
    structural_prop!(KeyboardNodeValid, "KeyboardNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `Caret` role.
    pub struct CaretNodeValid;
    structural_prop!(CaretNodeValid, "CaretNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `ImeCandidate` role.
    pub struct ImeCandidateNodeValid;
    structural_prop!(ImeCandidateNodeValid, "ImeCandidateNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `PdfRoot` role.
    pub struct PdfRootNodeValid;
    structural_prop!(PdfRootNodeValid, "PdfRootNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `PdfActionableHighlight` role.
    pub struct PdfActionableHighlightNodeValid;
    structural_prop!(
        PdfActionableHighlightNodeValid,
        "PdfActionableHighlightNodeValid"
    );

    /// Proof that an [`accesskit::Node`] is valid for the `GraphicsDocument` role.
    pub struct GraphicsDocumentNodeValid;
    structural_prop!(GraphicsDocumentNodeValid, "GraphicsDocumentNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `GraphicsObject` role.
    pub struct GraphicsObjectNodeValid;
    structural_prop!(GraphicsObjectNodeValid, "GraphicsObjectNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `GraphicsSymbol` role.
    pub struct GraphicsSymbolNodeValid;
    structural_prop!(GraphicsSymbolNodeValid, "GraphicsSymbolNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocAbstract` role.
    pub struct DocAbstractNodeValid;
    structural_prop!(DocAbstractNodeValid, "DocAbstractNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocAcknowledgements` role.
    pub struct DocAcknowledgementsNodeValid;
    structural_prop!(DocAcknowledgementsNodeValid, "DocAcknowledgementsNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocAfterword` role.
    pub struct DocAfterwordNodeValid;
    structural_prop!(DocAfterwordNodeValid, "DocAfterwordNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocAppendix` role.
    pub struct DocAppendixNodeValid;
    structural_prop!(DocAppendixNodeValid, "DocAppendixNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocBackLink` role.
    pub struct DocBackLinkNodeValid;
    structural_prop!(DocBackLinkNodeValid, "DocBackLinkNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocBiblioEntry` role.
    pub struct DocBiblioEntryNodeValid;
    structural_prop!(DocBiblioEntryNodeValid, "DocBiblioEntryNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocBibliography` role.
    pub struct DocBibliographyNodeValid;
    structural_prop!(DocBibliographyNodeValid, "DocBibliographyNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocBiblioRef` role.
    pub struct DocBiblioRefNodeValid;
    structural_prop!(DocBiblioRefNodeValid, "DocBiblioRefNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocChapter` role.
    pub struct DocChapterNodeValid;
    structural_prop!(DocChapterNodeValid, "DocChapterNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocColophon` role.
    pub struct DocColophonNodeValid;
    structural_prop!(DocColophonNodeValid, "DocColophonNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocConclusion` role.
    pub struct DocConclusionNodeValid;
    structural_prop!(DocConclusionNodeValid, "DocConclusionNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocCover` role.
    pub struct DocCoverNodeValid;
    structural_prop!(DocCoverNodeValid, "DocCoverNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocCredit` role.
    pub struct DocCreditNodeValid;
    structural_prop!(DocCreditNodeValid, "DocCreditNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocCredits` role.
    pub struct DocCreditsNodeValid;
    structural_prop!(DocCreditsNodeValid, "DocCreditsNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocDedication` role.
    pub struct DocDedicationNodeValid;
    structural_prop!(DocDedicationNodeValid, "DocDedicationNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocEndnote` role.
    pub struct DocEndnoteNodeValid;
    structural_prop!(DocEndnoteNodeValid, "DocEndnoteNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocEndnotes` role.
    pub struct DocEndnotesNodeValid;
    structural_prop!(DocEndnotesNodeValid, "DocEndnotesNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocEpigraph` role.
    pub struct DocEpigraphNodeValid;
    structural_prop!(DocEpigraphNodeValid, "DocEpigraphNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocEpilogue` role.
    pub struct DocEpilogueNodeValid;
    structural_prop!(DocEpilogueNodeValid, "DocEpilogueNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocErrata` role.
    pub struct DocErrataNodeValid;
    structural_prop!(DocErrataNodeValid, "DocErrataNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocExample` role.
    pub struct DocExampleNodeValid;
    structural_prop!(DocExampleNodeValid, "DocExampleNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocFootnote` role.
    pub struct DocFootnoteNodeValid;
    structural_prop!(DocFootnoteNodeValid, "DocFootnoteNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocForeword` role.
    pub struct DocForewordNodeValid;
    structural_prop!(DocForewordNodeValid, "DocForewordNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocGlossary` role.
    pub struct DocGlossaryNodeValid;
    structural_prop!(DocGlossaryNodeValid, "DocGlossaryNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocGlossRef` role.
    pub struct DocGlossRefNodeValid;
    structural_prop!(DocGlossRefNodeValid, "DocGlossRefNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocIndex` role.
    pub struct DocIndexNodeValid;
    structural_prop!(DocIndexNodeValid, "DocIndexNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocIntroduction` role.
    pub struct DocIntroductionNodeValid;
    structural_prop!(DocIntroductionNodeValid, "DocIntroductionNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocNoteRef` role.
    pub struct DocNoteRefNodeValid;
    structural_prop!(DocNoteRefNodeValid, "DocNoteRefNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocNotice` role.
    pub struct DocNoticeNodeValid;
    structural_prop!(DocNoticeNodeValid, "DocNoticeNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocPageBreak` role.
    pub struct DocPageBreakNodeValid;
    structural_prop!(DocPageBreakNodeValid, "DocPageBreakNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocPageFooter` role.
    pub struct DocPageFooterNodeValid;
    structural_prop!(DocPageFooterNodeValid, "DocPageFooterNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocPageHeader` role.
    pub struct DocPageHeaderNodeValid;
    structural_prop!(DocPageHeaderNodeValid, "DocPageHeaderNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocPageList` role.
    pub struct DocPageListNodeValid;
    structural_prop!(DocPageListNodeValid, "DocPageListNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocPart` role.
    pub struct DocPartNodeValid;
    structural_prop!(DocPartNodeValid, "DocPartNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocPreface` role.
    pub struct DocPrefaceNodeValid;
    structural_prop!(DocPrefaceNodeValid, "DocPrefaceNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocPrologue` role.
    pub struct DocPrologueNodeValid;
    structural_prop!(DocPrologueNodeValid, "DocPrologueNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocPullquote` role.
    pub struct DocPullquoteNodeValid;
    structural_prop!(DocPullquoteNodeValid, "DocPullquoteNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocQna` role.
    pub struct DocQnaNodeValid;
    structural_prop!(DocQnaNodeValid, "DocQnaNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocSubtitle` role.
    pub struct DocSubtitleNodeValid;
    structural_prop!(DocSubtitleNodeValid, "DocSubtitleNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocTip` role.
    pub struct DocTipNodeValid;
    structural_prop!(DocTipNodeValid, "DocTipNodeValid");

    /// Proof that an [`accesskit::Node`] is valid for the `DocToc` role.
    pub struct DocTocNodeValid;
    structural_prop!(DocTocNodeValid, "DocTocNodeValid");
}

/// Declares that `Src` is an ARIA subtype of `Dst`, allowing an
/// `Established<Src>` to mint an `Established<Dst>`.
macro_rules! role_alias {
    ($src:ty => $dst:ty) => {
        impl elicitation::contracts::ProvableFrom<elicitation::contracts::Established<$src>>
            for $dst
        {
        }
    };
}

// Role-alias ProvableFrom impls — ARIA subtype relationships.
// Each line declares: holding proof of Src is sufficient credential to prove Dst.
role_alias!(emit_impls::SearchInputNodeValid => emit_impls::TextInputNodeValid);
role_alias!(emit_impls::DateInputNodeValid => emit_impls::TextInputNodeValid);
role_alias!(emit_impls::DateTimeInputNodeValid => emit_impls::TextInputNodeValid);
role_alias!(emit_impls::WeekInputNodeValid => emit_impls::TextInputNodeValid);
role_alias!(emit_impls::MonthInputNodeValid => emit_impls::TextInputNodeValid);
role_alias!(emit_impls::TimeInputNodeValid => emit_impls::TextInputNodeValid);
role_alias!(emit_impls::EmailInputNodeValid => emit_impls::TextInputNodeValid);
role_alias!(emit_impls::NumberInputNodeValid => emit_impls::TextInputNodeValid);
role_alias!(emit_impls::PasswordInputNodeValid => emit_impls::TextInputNodeValid);
role_alias!(emit_impls::PhoneNumberInputNodeValid => emit_impls::TextInputNodeValid);
role_alias!(emit_impls::UrlInputNodeValid => emit_impls::TextInputNodeValid);
role_alias!(emit_impls::AbbrNodeValid => emit_impls::LabelNodeValid);
role_alias!(emit_impls::EmphasisNodeValid => emit_impls::LabelNodeValid);
role_alias!(emit_impls::StrongNodeValid => emit_impls::LabelNodeValid);
role_alias!(emit_impls::MarkNodeValid => emit_impls::LabelNodeValid);
role_alias!(emit_impls::TimeNodeValid => emit_impls::LabelNodeValid);
role_alias!(emit_impls::RubyNodeValid => emit_impls::LabelNodeValid);
role_alias!(emit_impls::RubyAnnotationNodeValid => emit_impls::LabelNodeValid);
role_alias!(emit_impls::SuggestionNodeValid => emit_impls::ParagraphNodeValid);
role_alias!(emit_impls::CommentNodeValid => emit_impls::ParagraphNodeValid);
role_alias!(emit_impls::ContentDeletionNodeValid => emit_impls::LabelNodeValid);
role_alias!(emit_impls::ContentInsertionNodeValid => emit_impls::LabelNodeValid);
role_alias!(emit_impls::LegendNodeValid => emit_impls::LabelNodeValid);
role_alias!(emit_impls::SvgRootNodeValid => emit_impls::ImageNodeValid);
role_alias!(emit_impls::EmbeddedObjectNodeValid => emit_impls::UnknownNodeValid);
role_alias!(emit_impls::PluginObjectNodeValid => emit_impls::UnknownNodeValid);
role_alias!(emit_impls::WebViewNodeValid => emit_impls::UnknownNodeValid);
role_alias!(emit_impls::IframeNodeValid => emit_impls::UnknownNodeValid);
role_alias!(emit_impls::IframePresentationalNodeValid => emit_impls::GenericContainerNodeValid);
role_alias!(emit_impls::HeaderNodeValid => emit_impls::SectionHeaderNodeValid);
role_alias!(emit_impls::FooterNodeValid => emit_impls::SectionFooterNodeValid);
role_alias!(emit_impls::AlertDialogNodeValid => emit_impls::DialogNodeValid);
role_alias!(emit_impls::LogNodeValid => emit_impls::StatusNodeValid);
role_alias!(emit_impls::MarqueeNodeValid => emit_impls::StatusNodeValid);
role_alias!(emit_impls::ListMarkerNodeValid => emit_impls::LabelNodeValid);
role_alias!(emit_impls::FeedNodeValid => emit_impls::ListNodeValid);
role_alias!(emit_impls::ListBoxOptionNodeValid => emit_impls::ListItemNodeValid);
role_alias!(emit_impls::RowHeaderNodeValid => emit_impls::CellNodeValid);
role_alias!(emit_impls::ColumnHeaderNodeValid => emit_impls::CellNodeValid);
role_alias!(emit_impls::GridNodeValid => emit_impls::TableNodeValid);
role_alias!(emit_impls::GridCellNodeValid => emit_impls::CellNodeValid);
role_alias!(emit_impls::TreeGridNodeValid => emit_impls::TreeNodeValid);
role_alias!(emit_impls::ListGridNodeValid => emit_impls::GridNodeValid);
role_alias!(emit_impls::LayoutTableNodeValid => emit_impls::GenericContainerNodeValid);
role_alias!(emit_impls::LayoutTableRowNodeValid => emit_impls::RowNodeValid);
role_alias!(emit_impls::LayoutTableCellNodeValid => emit_impls::GenericContainerNodeValid);
role_alias!(emit_impls::MenuBarNodeValid => emit_impls::MenuNodeValid);
role_alias!(emit_impls::MenuListOptionNodeValid => emit_impls::MenuItemNodeValid);
role_alias!(emit_impls::MenuListPopupNodeValid => emit_impls::MenuNodeValid);
role_alias!(emit_impls::MenuItemCheckBoxNodeValid => emit_impls::CheckBoxNodeValid);
role_alias!(emit_impls::MenuItemRadioNodeValid => emit_impls::RadioButtonNodeValid);
role_alias!(emit_impls::TitleBarNodeValid => emit_impls::ToolbarNodeValid);
role_alias!(emit_impls::MeterNodeValid => emit_impls::ProgressIndicatorNodeValid);
role_alias!(emit_impls::KeyboardNodeValid => emit_impls::UnknownNodeValid);
role_alias!(emit_impls::CaretNodeValid => emit_impls::UnknownNodeValid);
role_alias!(emit_impls::ImeCandidateNodeValid => emit_impls::UnknownNodeValid);
role_alias!(emit_impls::PdfRootNodeValid => emit_impls::DocumentNodeValid);
role_alias!(emit_impls::PdfActionableHighlightNodeValid => emit_impls::LinkNodeValid);
role_alias!(emit_impls::GraphicsDocumentNodeValid => emit_impls::DocumentNodeValid);
role_alias!(emit_impls::GraphicsObjectNodeValid => emit_impls::GroupNodeValid);
role_alias!(emit_impls::GraphicsSymbolNodeValid => emit_impls::ImageNodeValid);
role_alias!(emit_impls::DocAbstractNodeValid => emit_impls::SectionNodeValid);
role_alias!(emit_impls::DocAcknowledgementsNodeValid => emit_impls::SectionNodeValid);
role_alias!(emit_impls::DocAfterwordNodeValid => emit_impls::SectionNodeValid);
role_alias!(emit_impls::DocAppendixNodeValid => emit_impls::SectionNodeValid);
role_alias!(emit_impls::DocBackLinkNodeValid => emit_impls::LinkNodeValid);
role_alias!(emit_impls::DocBiblioEntryNodeValid => emit_impls::ListItemNodeValid);
role_alias!(emit_impls::DocBibliographyNodeValid => emit_impls::ListNodeValid);
role_alias!(emit_impls::DocBiblioRefNodeValid => emit_impls::LinkNodeValid);
role_alias!(emit_impls::DocChapterNodeValid => emit_impls::SectionNodeValid);
role_alias!(emit_impls::DocColophonNodeValid => emit_impls::SectionNodeValid);
role_alias!(emit_impls::DocConclusionNodeValid => emit_impls::SectionNodeValid);
role_alias!(emit_impls::DocCoverNodeValid => emit_impls::FigureNodeValid);
role_alias!(emit_impls::DocCreditNodeValid => emit_impls::ParagraphNodeValid);
role_alias!(emit_impls::DocCreditsNodeValid => emit_impls::SectionNodeValid);
role_alias!(emit_impls::DocDedicationNodeValid => emit_impls::SectionNodeValid);
role_alias!(emit_impls::DocEndnoteNodeValid => emit_impls::NoteNodeValid);
role_alias!(emit_impls::DocEndnotesNodeValid => emit_impls::ListNodeValid);
role_alias!(emit_impls::DocEpigraphNodeValid => emit_impls::BlockquoteNodeValid);
role_alias!(emit_impls::DocEpilogueNodeValid => emit_impls::SectionNodeValid);
role_alias!(emit_impls::DocErrataNodeValid => emit_impls::SectionNodeValid);
role_alias!(emit_impls::DocExampleNodeValid => emit_impls::SectionNodeValid);
role_alias!(emit_impls::DocFootnoteNodeValid => emit_impls::NoteNodeValid);
role_alias!(emit_impls::DocForewordNodeValid => emit_impls::SectionNodeValid);
role_alias!(emit_impls::DocGlossaryNodeValid => emit_impls::DescriptionListNodeValid);
role_alias!(emit_impls::DocGlossRefNodeValid => emit_impls::LinkNodeValid);
role_alias!(emit_impls::DocIndexNodeValid => emit_impls::SectionNodeValid);
role_alias!(emit_impls::DocIntroductionNodeValid => emit_impls::SectionNodeValid);
role_alias!(emit_impls::DocNoteRefNodeValid => emit_impls::LinkNodeValid);
role_alias!(emit_impls::DocNoticeNodeValid => emit_impls::AlertNodeValid);
role_alias!(emit_impls::DocPageBreakNodeValid => emit_impls::LineBreakNodeValid);
role_alias!(emit_impls::DocPageFooterNodeValid => emit_impls::SectionFooterNodeValid);
role_alias!(emit_impls::DocPageHeaderNodeValid => emit_impls::SectionHeaderNodeValid);
role_alias!(emit_impls::DocPageListNodeValid => emit_impls::ListNodeValid);
role_alias!(emit_impls::DocPartNodeValid => emit_impls::SectionNodeValid);
role_alias!(emit_impls::DocPrefaceNodeValid => emit_impls::SectionNodeValid);
role_alias!(emit_impls::DocPrologueNodeValid => emit_impls::SectionNodeValid);
role_alias!(emit_impls::DocPullquoteNodeValid => emit_impls::BlockquoteNodeValid);
role_alias!(emit_impls::DocQnaNodeValid => emit_impls::GroupNodeValid);
role_alias!(emit_impls::DocSubtitleNodeValid => emit_impls::HeadingNodeValid);
role_alias!(emit_impls::DocTipNodeValid => emit_impls::NoteNodeValid);
role_alias!(emit_impls::DocTocNodeValid => emit_impls::NavigationNodeValid);
role_alias!(emit_impls::DefaultButtonNodeValid => emit_impls::ButtonNodeValid);
role_alias!(emit_impls::EditableComboBoxNodeValid => emit_impls::ComboBoxNodeValid);
role_alias!(emit_impls::ScrollBarNodeValid => emit_impls::ProgressIndicatorNodeValid);
role_alias!(emit_impls::MultilineTextInputNodeValid => emit_impls::TextInputNodeValid);
role_alias!(emit_impls::NumberInputNodeValid => emit_impls::SpinButtonNodeValid);

pub use emit_impls::{
    AbbrNodeValid, AlertDialogNodeValid, AlertNodeValid, ApplicationNodeValid, ArticleNodeValid,
    AudioNodeValid, BannerNodeValid, BlockquoteNodeValid, ButtonNodeValid, CanvasNodeValid,
    CaptionNodeValid, CaretNodeValid, CellNodeValid, CheckBoxNodeValid, CodeNodeValid,
    ColorWellNodeValid, ColumnHeaderNodeValid, ComboBoxNodeValid, CommentNodeValid,
    ComplementaryNodeValid, ContentDeletionNodeValid, ContentInfoNodeValid,
    ContentInsertionNodeValid, DateInputNodeValid, DateTimeInputNodeValid, DefaultButtonNodeValid,
    DefinitionNodeValid, DescriptionListNodeValid, DetailsNodeValid, DialogNodeValid,
    DisclosureTriangleNodeValid, DocAbstractNodeValid, DocAcknowledgementsNodeValid,
    DocAfterwordNodeValid, DocAppendixNodeValid, DocBackLinkNodeValid, DocBiblioEntryNodeValid,
    DocBiblioRefNodeValid, DocBibliographyNodeValid, DocChapterNodeValid, DocColophonNodeValid,
    DocConclusionNodeValid, DocCoverNodeValid, DocCreditNodeValid, DocCreditsNodeValid,
    DocDedicationNodeValid, DocEndnoteNodeValid, DocEndnotesNodeValid, DocEpigraphNodeValid,
    DocEpilogueNodeValid, DocErrataNodeValid, DocExampleNodeValid, DocFootnoteNodeValid,
    DocForewordNodeValid, DocGlossRefNodeValid, DocGlossaryNodeValid, DocIndexNodeValid,
    DocIntroductionNodeValid, DocNoteRefNodeValid, DocNoticeNodeValid, DocPageBreakNodeValid,
    DocPageFooterNodeValid, DocPageHeaderNodeValid, DocPageListNodeValid, DocPartNodeValid,
    DocPrefaceNodeValid, DocPrologueNodeValid, DocPullquoteNodeValid, DocQnaNodeValid,
    DocSubtitleNodeValid, DocTipNodeValid, DocTocNodeValid, DocumentNodeValid,
    EditableComboBoxNodeValid, EmailInputNodeValid, EmbeddedObjectNodeValid, EmphasisNodeValid,
    FeedNodeValid, FigureCaptionNodeValid, FigureNodeValid, FooterNodeValid, FormNodeValid,
    GenericContainerNodeValid, GraphicsDocumentNodeValid, GraphicsObjectNodeValid,
    GraphicsSymbolNodeValid, GridCellNodeValid, GridNodeValid, GroupNodeValid, HeaderNodeValid,
    HeadingNodeValid, IframeNodeValid, IframePresentationalNodeValid, ImageNodeValid,
    ImeCandidateNodeValid, KeyboardNodeValid, LabelNodeValid, LayoutTableCellNodeValid,
    LayoutTableNodeValid, LayoutTableRowNodeValid, LegendNodeValid, LineBreakNodeValid,
    LinkNodeValid, ListBoxNodeValid, ListBoxOptionNodeValid, ListGridNodeValid, ListItemNodeValid,
    ListMarkerNodeValid, ListNodeValid, LogNodeValid, MainNodeValid, MarkNodeValid,
    MarqueeNodeValid, MathNodeValid, MenuBarNodeValid, MenuItemCheckBoxNodeValid,
    MenuItemNodeValid, MenuItemRadioNodeValid, MenuListOptionNodeValid, MenuListPopupNodeValid,
    MenuNodeValid, MeterNodeValid, MonthInputNodeValid, MultilineTextInputNodeValid,
    NavigationNodeValid, NoteNodeValid, NumberInputNodeValid, PaneNodeValid, ParagraphNodeValid,
    PasswordInputNodeValid, PdfActionableHighlightNodeValid, PdfRootNodeValid,
    PhoneNumberInputNodeValid, PluginObjectNodeValid, ProgressIndicatorNodeValid,
    RadioButtonNodeValid, RadioGroupNodeValid, RegionNodeValid, RootWebAreaNodeValid,
    RowGroupNodeValid, RowHeaderNodeValid, RowNodeValid, RubyAnnotationNodeValid, RubyNodeValid,
    ScrollBarNodeValid, ScrollViewNodeValid, SearchInputNodeValid, SearchNodeValid,
    SectionFooterNodeValid, SectionHeaderNodeValid, SectionNodeValid, SliderNodeValid,
    SpinButtonNodeValid, SplitterNodeValid, StatusNodeValid, StrongNodeValid, SuggestionNodeValid,
    SvgRootNodeValid, SwitchNodeValid, TabListNodeValid, TabNodeValid, TabPanelNodeValid,
    TableNodeValid, TermNodeValid, TerminalNodeValid, TextInputNodeValid, TextRunNodeValid,
    TimeInputNodeValid, TimeNodeValid, TimerNodeValid, TitleBarNodeValid, ToolbarNodeValid,
    TooltipNodeValid, TreeGridNodeValid, TreeItemNodeValid, TreeNodeValid, UnknownNodeValid,
    UrlInputNodeValid, VideoNodeValid, WebViewNodeValid, WeekInputNodeValid, WindowNodeValid,
};
