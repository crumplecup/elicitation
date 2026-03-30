//! Newtype wrappers for all accesskit enum types.

use elicitation_derive::reflect_methods;

use crate::accesskit_copy_enum;

accesskit_copy_enum!(Role, accesskit::Role);
accesskit_copy_enum!(Action, accesskit::Action);
accesskit_copy_enum!(Orientation, accesskit::Orientation);
accesskit_copy_enum!(TextDirection, accesskit::TextDirection);
accesskit_copy_enum!(Invalid, accesskit::Invalid);
accesskit_copy_enum!(Toggled, accesskit::Toggled);
accesskit_copy_enum!(SortDirection, accesskit::SortDirection);
accesskit_copy_enum!(AriaCurrent, accesskit::AriaCurrent);
accesskit_copy_enum!(AutoComplete, accesskit::AutoComplete);
accesskit_copy_enum!(Live, accesskit::Live);
accesskit_copy_enum!(HasPopup, accesskit::HasPopup);
accesskit_copy_enum!(ListStyle, accesskit::ListStyle);
accesskit_copy_enum!(TextAlign, accesskit::TextAlign);
accesskit_copy_enum!(VerticalOffset, accesskit::VerticalOffset);
accesskit_copy_enum!(TextDecorationStyle, accesskit::TextDecorationStyle);
accesskit_copy_enum!(ScrollUnit, accesskit::ScrollUnit);
accesskit_copy_enum!(ScrollHint, accesskit::ScrollHint);

#[reflect_methods]
impl Role {
    /// Returns `true` if this role represents an interactive form control.
    #[tracing::instrument(skip(self))]
    pub fn is_form_control(&self) -> bool {
        matches!(
            self.0,
            accesskit::Role::Button
                | accesskit::Role::DefaultButton
                | accesskit::Role::CheckBox
                | accesskit::Role::RadioButton
                | accesskit::Role::Switch
                | accesskit::Role::TextInput
                | accesskit::Role::MultilineTextInput
                | accesskit::Role::SearchInput
                | accesskit::Role::DateInput
                | accesskit::Role::DateTimeInput
                | accesskit::Role::WeekInput
                | accesskit::Role::MonthInput
                | accesskit::Role::TimeInput
                | accesskit::Role::EmailInput
                | accesskit::Role::NumberInput
                | accesskit::Role::PasswordInput
                | accesskit::Role::PhoneNumberInput
                | accesskit::Role::UrlInput
                | accesskit::Role::Slider
                | accesskit::Role::SpinButton
                | accesskit::Role::ComboBox
                | accesskit::Role::EditableComboBox
        )
    }

    /// Returns `true` if this role represents a text input of any kind.
    #[tracing::instrument(skip(self))]
    pub fn is_text_input(&self) -> bool {
        matches!(
            self.0,
            accesskit::Role::TextInput
                | accesskit::Role::MultilineTextInput
                | accesskit::Role::SearchInput
                | accesskit::Role::DateInput
                | accesskit::Role::DateTimeInput
                | accesskit::Role::WeekInput
                | accesskit::Role::MonthInput
                | accesskit::Role::TimeInput
                | accesskit::Role::EmailInput
                | accesskit::Role::NumberInput
                | accesskit::Role::PasswordInput
                | accesskit::Role::PhoneNumberInput
                | accesskit::Role::UrlInput
        )
    }

    /// Returns `true` if this role is a container/structural element.
    #[tracing::instrument(skip(self))]
    pub fn is_container(&self) -> bool {
        matches!(
            self.0,
            accesskit::Role::Group
                | accesskit::Role::Pane
                | accesskit::Role::Section
                | accesskit::Role::Article
                | accesskit::Role::Navigation
                | accesskit::Role::Banner
                | accesskit::Role::Complementary
                | accesskit::Role::ContentInfo
                | accesskit::Role::Main
                | accesskit::Role::Region
                | accesskit::Role::Form
                | accesskit::Role::Dialog
                | accesskit::Role::AlertDialog
                | accesskit::Role::Application
                | accesskit::Role::Document
                | accesskit::Role::Feed
                | accesskit::Role::Figure
                | accesskit::Role::Grid
                | accesskit::Role::List
                | accesskit::Role::ListBox
                | accesskit::Role::Menu
                | accesskit::Role::MenuBar
                | accesskit::Role::MenuListPopup
                | accesskit::Role::TabList
                | accesskit::Role::TabPanel
                | accesskit::Role::Table
                | accesskit::Role::Tree
                | accesskit::Role::TreeGrid
        )
    }

    /// Returns `true` if this role represents a landmark region.
    #[tracing::instrument(skip(self))]
    pub fn is_landmark(&self) -> bool {
        matches!(
            self.0,
            accesskit::Role::Banner
                | accesskit::Role::Complementary
                | accesskit::Role::ContentInfo
                | accesskit::Role::Form
                | accesskit::Role::Main
                | accesskit::Role::Navigation
                | accesskit::Role::Region
                | accesskit::Role::Search
        )
    }

    /// Returns the role name as a string slice.
    #[tracing::instrument(skip(self))]
    pub fn name(&self) -> &'static str {
        match self.0 {
            accesskit::Role::Unknown => "Unknown",
            accesskit::Role::TextRun => "TextRun",
            accesskit::Role::Cell => "Cell",
            accesskit::Role::Label => "Label",
            accesskit::Role::Image => "Image",
            accesskit::Role::Link => "Link",
            accesskit::Role::Row => "Row",
            accesskit::Role::ListItem => "ListItem",
            accesskit::Role::ListMarker => "ListMarker",
            accesskit::Role::TreeItem => "TreeItem",
            accesskit::Role::ListBoxOption => "ListBoxOption",
            accesskit::Role::MenuItem => "MenuItem",
            accesskit::Role::MenuListOption => "MenuListOption",
            accesskit::Role::Paragraph => "Paragraph",
            accesskit::Role::GenericContainer => "GenericContainer",
            accesskit::Role::CheckBox => "CheckBox",
            accesskit::Role::RadioButton => "RadioButton",
            accesskit::Role::TextInput => "TextInput",
            accesskit::Role::Button => "Button",
            accesskit::Role::DefaultButton => "DefaultButton",
            accesskit::Role::Pane => "Pane",
            accesskit::Role::RowHeader => "RowHeader",
            accesskit::Role::ColumnHeader => "ColumnHeader",
            accesskit::Role::RowGroup => "RowGroup",
            accesskit::Role::List => "List",
            accesskit::Role::Table => "Table",
            accesskit::Role::LayoutTableCell => "LayoutTableCell",
            accesskit::Role::LayoutTableRow => "LayoutTableRow",
            accesskit::Role::LayoutTable => "LayoutTable",
            accesskit::Role::Switch => "Switch",
            accesskit::Role::Menu => "Menu",
            accesskit::Role::MultilineTextInput => "MultilineTextInput",
            accesskit::Role::SearchInput => "SearchInput",
            accesskit::Role::DateInput => "DateInput",
            accesskit::Role::DateTimeInput => "DateTimeInput",
            accesskit::Role::WeekInput => "WeekInput",
            accesskit::Role::MonthInput => "MonthInput",
            accesskit::Role::TimeInput => "TimeInput",
            accesskit::Role::EmailInput => "EmailInput",
            accesskit::Role::NumberInput => "NumberInput",
            accesskit::Role::PasswordInput => "PasswordInput",
            accesskit::Role::PhoneNumberInput => "PhoneNumberInput",
            accesskit::Role::UrlInput => "UrlInput",
            accesskit::Role::Abbr => "Abbr",
            accesskit::Role::Alert => "Alert",
            accesskit::Role::AlertDialog => "AlertDialog",
            accesskit::Role::Application => "Application",
            accesskit::Role::Article => "Article",
            accesskit::Role::Audio => "Audio",
            accesskit::Role::Banner => "Banner",
            accesskit::Role::Blockquote => "Blockquote",
            accesskit::Role::Canvas => "Canvas",
            accesskit::Role::Caption => "Caption",
            accesskit::Role::Caret => "Caret",
            accesskit::Role::Code => "Code",
            accesskit::Role::ColorWell => "ColorWell",
            accesskit::Role::ComboBox => "ComboBox",
            accesskit::Role::EditableComboBox => "EditableComboBox",
            accesskit::Role::Complementary => "Complementary",
            accesskit::Role::Comment => "Comment",
            accesskit::Role::ContentDeletion => "ContentDeletion",
            accesskit::Role::ContentInsertion => "ContentInsertion",
            accesskit::Role::ContentInfo => "ContentInfo",
            accesskit::Role::Definition => "Definition",
            accesskit::Role::DescriptionList => "DescriptionList",
            accesskit::Role::Details => "Details",
            accesskit::Role::Dialog => "Dialog",
            accesskit::Role::DisclosureTriangle => "DisclosureTriangle",
            accesskit::Role::Document => "Document",
            accesskit::Role::EmbeddedObject => "EmbeddedObject",
            accesskit::Role::Emphasis => "Emphasis",
            accesskit::Role::Feed => "Feed",
            accesskit::Role::FigureCaption => "FigureCaption",
            accesskit::Role::Figure => "Figure",
            accesskit::Role::Footer => "Footer",
            accesskit::Role::Form => "Form",
            accesskit::Role::Grid => "Grid",
            accesskit::Role::GridCell => "GridCell",
            accesskit::Role::Group => "Group",
            accesskit::Role::Header => "Header",
            accesskit::Role::Heading => "Heading",
            accesskit::Role::Iframe => "Iframe",
            accesskit::Role::IframePresentational => "IframePresentational",
            accesskit::Role::ImeCandidate => "ImeCandidate",
            accesskit::Role::Keyboard => "Keyboard",
            accesskit::Role::Legend => "Legend",
            accesskit::Role::LineBreak => "LineBreak",
            accesskit::Role::ListBox => "ListBox",
            accesskit::Role::Log => "Log",
            accesskit::Role::Main => "Main",
            accesskit::Role::Mark => "Mark",
            accesskit::Role::Marquee => "Marquee",
            accesskit::Role::Math => "Math",
            accesskit::Role::MenuBar => "MenuBar",
            accesskit::Role::MenuItemCheckBox => "MenuItemCheckBox",
            accesskit::Role::MenuItemRadio => "MenuItemRadio",
            accesskit::Role::MenuListPopup => "MenuListPopup",
            accesskit::Role::Meter => "Meter",
            accesskit::Role::Navigation => "Navigation",
            accesskit::Role::Note => "Note",
            accesskit::Role::PluginObject => "PluginObject",
            accesskit::Role::ProgressIndicator => "ProgressIndicator",
            accesskit::Role::RadioGroup => "RadioGroup",
            accesskit::Role::Region => "Region",
            accesskit::Role::RootWebArea => "RootWebArea",
            accesskit::Role::Ruby => "Ruby",
            accesskit::Role::RubyAnnotation => "RubyAnnotation",
            accesskit::Role::ScrollBar => "ScrollBar",
            accesskit::Role::ScrollView => "ScrollView",
            accesskit::Role::Search => "Search",
            accesskit::Role::Section => "Section",
            accesskit::Role::SectionFooter => "SectionFooter",
            accesskit::Role::SectionHeader => "SectionHeader",
            accesskit::Role::Slider => "Slider",
            accesskit::Role::SpinButton => "SpinButton",
            accesskit::Role::Splitter => "Splitter",
            accesskit::Role::Status => "Status",
            accesskit::Role::Strong => "Strong",
            accesskit::Role::Suggestion => "Suggestion",
            accesskit::Role::SvgRoot => "SvgRoot",
            accesskit::Role::Tab => "Tab",
            accesskit::Role::TabList => "TabList",
            accesskit::Role::TabPanel => "TabPanel",
            accesskit::Role::Term => "Term",
            accesskit::Role::Time => "Time",
            accesskit::Role::Timer => "Timer",
            accesskit::Role::TitleBar => "TitleBar",
            accesskit::Role::Toolbar => "Toolbar",
            accesskit::Role::Tooltip => "Tooltip",
            accesskit::Role::Tree => "Tree",
            accesskit::Role::TreeGrid => "TreeGrid",
            accesskit::Role::Video => "Video",
            accesskit::Role::WebView => "WebView",
            accesskit::Role::Window => "Window",
            accesskit::Role::PdfActionableHighlight => "PdfActionableHighlight",
            accesskit::Role::PdfRoot => "PdfRoot",
            accesskit::Role::GraphicsDocument => "GraphicsDocument",
            accesskit::Role::GraphicsObject => "GraphicsObject",
            accesskit::Role::GraphicsSymbol => "GraphicsSymbol",
            accesskit::Role::DocAbstract => "DocAbstract",
            accesskit::Role::DocAcknowledgements => "DocAcknowledgements",
            accesskit::Role::DocAfterword => "DocAfterword",
            accesskit::Role::DocAppendix => "DocAppendix",
            accesskit::Role::DocBackLink => "DocBackLink",
            accesskit::Role::DocBiblioEntry => "DocBiblioEntry",
            accesskit::Role::DocBibliography => "DocBibliography",
            accesskit::Role::DocBiblioRef => "DocBiblioRef",
            accesskit::Role::DocChapter => "DocChapter",
            accesskit::Role::DocColophon => "DocColophon",
            accesskit::Role::DocConclusion => "DocConclusion",
            accesskit::Role::DocCover => "DocCover",
            accesskit::Role::DocCredit => "DocCredit",
            accesskit::Role::DocCredits => "DocCredits",
            accesskit::Role::DocDedication => "DocDedication",
            accesskit::Role::DocEndnote => "DocEndnote",
            accesskit::Role::DocEndnotes => "DocEndnotes",
            accesskit::Role::DocEpigraph => "DocEpigraph",
            accesskit::Role::DocEpilogue => "DocEpilogue",
            accesskit::Role::DocErrata => "DocErrata",
            accesskit::Role::DocExample => "DocExample",
            accesskit::Role::DocFootnote => "DocFootnote",
            accesskit::Role::DocForeword => "DocForeword",
            accesskit::Role::DocGlossary => "DocGlossary",
            accesskit::Role::DocGlossRef => "DocGlossRef",
            accesskit::Role::DocIndex => "DocIndex",
            accesskit::Role::DocIntroduction => "DocIntroduction",
            accesskit::Role::DocNoteRef => "DocNoteRef",
            accesskit::Role::DocNotice => "DocNotice",
            accesskit::Role::DocPageBreak => "DocPageBreak",
            accesskit::Role::DocPageFooter => "DocPageFooter",
            accesskit::Role::DocPageHeader => "DocPageHeader",
            accesskit::Role::DocPageList => "DocPageList",
            accesskit::Role::DocPart => "DocPart",
            accesskit::Role::DocPreface => "DocPreface",
            accesskit::Role::DocPrologue => "DocPrologue",
            accesskit::Role::DocPullquote => "DocPullquote",
            accesskit::Role::DocQna => "DocQna",
            accesskit::Role::DocSubtitle => "DocSubtitle",
            accesskit::Role::DocTip => "DocTip",
            accesskit::Role::DocToc => "DocToc",
            accesskit::Role::ListGrid => "ListGrid",
            accesskit::Role::Terminal => "Terminal",
        }
    }
}

#[reflect_methods]
impl Action {
    /// Returns the action name as a string slice.
    #[tracing::instrument(skip(self))]
    pub fn name(&self) -> &'static str {
        match self.0 {
            accesskit::Action::Click => "Click",
            accesskit::Action::Focus => "Focus",
            accesskit::Action::Blur => "Blur",
            accesskit::Action::Collapse => "Collapse",
            accesskit::Action::Expand => "Expand",
            accesskit::Action::CustomAction => "CustomAction",
            accesskit::Action::Decrement => "Decrement",
            accesskit::Action::Increment => "Increment",
            accesskit::Action::HideTooltip => "HideTooltip",
            accesskit::Action::ShowTooltip => "ShowTooltip",
            accesskit::Action::ReplaceSelectedText => "ReplaceSelectedText",
            accesskit::Action::ScrollDown => "ScrollDown",
            accesskit::Action::ScrollLeft => "ScrollLeft",
            accesskit::Action::ScrollRight => "ScrollRight",
            accesskit::Action::ScrollUp => "ScrollUp",
            accesskit::Action::ScrollIntoView => "ScrollIntoView",
            accesskit::Action::ScrollToPoint => "ScrollToPoint",
            accesskit::Action::SetScrollOffset => "SetScrollOffset",
            accesskit::Action::SetTextSelection => "SetTextSelection",
            accesskit::Action::SetSequentialFocusNavigationStartingPoint => {
                "SetSequentialFocusNavigationStartingPoint"
            }
            accesskit::Action::SetValue => "SetValue",
            accesskit::Action::ShowContextMenu => "ShowContextMenu",
        }
    }

    /// Returns `true` if this is an input/value manipulation action.
    #[tracing::instrument(skip(self))]
    pub fn is_value_action(&self) -> bool {
        matches!(
            self.0,
            accesskit::Action::SetValue
                | accesskit::Action::Increment
                | accesskit::Action::Decrement
                | accesskit::Action::ReplaceSelectedText
                | accesskit::Action::SetTextSelection
        )
    }

    /// Returns `true` if this is a focus/navigation action.
    #[tracing::instrument(skip(self))]
    pub fn is_focus_action(&self) -> bool {
        matches!(
            self.0,
            accesskit::Action::Focus
                | accesskit::Action::Blur
                | accesskit::Action::ScrollIntoView
                | accesskit::Action::ScrollToPoint
                | accesskit::Action::SetSequentialFocusNavigationStartingPoint
        )
    }
}
