//! `accesskit::Role` elicitation.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use accesskit::Role;

impl Prompt for Role {
    fn prompt() -> Option<&'static str> {
        Some("Choose the accessibility role for this node:")
    }
}

impl Select for Role {
    fn options() -> Vec<Self> {
        vec![
            Role::Unknown,
            Role::TextRun,
            Role::Cell,
            Role::Label,
            Role::Image,
            Role::Link,
            Role::Row,
            Role::ListItem,
            Role::ListMarker,
            Role::TreeItem,
            Role::ListBoxOption,
            Role::MenuItem,
            Role::MenuListOption,
            Role::Paragraph,
            Role::GenericContainer,
            Role::CheckBox,
            Role::RadioButton,
            Role::TextInput,
            Role::Button,
            Role::DefaultButton,
            Role::Pane,
            Role::RowHeader,
            Role::ColumnHeader,
            Role::RowGroup,
            Role::List,
            Role::Table,
            Role::LayoutTableCell,
            Role::LayoutTableRow,
            Role::LayoutTable,
            Role::Switch,
            Role::Menu,
            Role::MultilineTextInput,
            Role::SearchInput,
            Role::DateInput,
            Role::DateTimeInput,
            Role::WeekInput,
            Role::MonthInput,
            Role::TimeInput,
            Role::EmailInput,
            Role::NumberInput,
            Role::PasswordInput,
            Role::PhoneNumberInput,
            Role::UrlInput,
            Role::Abbr,
            Role::Alert,
            Role::AlertDialog,
            Role::Application,
            Role::Article,
            Role::Audio,
            Role::Banner,
            Role::Blockquote,
            Role::Canvas,
            Role::Caption,
            Role::Caret,
            Role::Code,
            Role::ColorWell,
            Role::ComboBox,
            Role::EditableComboBox,
            Role::Complementary,
            Role::Comment,
            Role::ContentDeletion,
            Role::ContentInsertion,
            Role::ContentInfo,
            Role::Definition,
            Role::DescriptionList,
            Role::Details,
            Role::Dialog,
            Role::DisclosureTriangle,
            Role::Document,
            Role::EmbeddedObject,
            Role::Emphasis,
            Role::Feed,
            Role::FigureCaption,
            Role::Figure,
            Role::Footer,
            Role::Form,
            Role::Grid,
            Role::GridCell,
            Role::Group,
            Role::Header,
            Role::Heading,
            Role::Iframe,
            Role::IframePresentational,
            Role::ImeCandidate,
            Role::Keyboard,
            Role::Legend,
            Role::LineBreak,
            Role::ListBox,
            Role::Log,
            Role::Main,
            Role::Mark,
            Role::Marquee,
            Role::Math,
            Role::MenuBar,
            Role::MenuItemCheckBox,
            Role::MenuItemRadio,
            Role::MenuListPopup,
            Role::Meter,
            Role::Navigation,
            Role::Note,
            Role::PluginObject,
            Role::ProgressIndicator,
            Role::RadioGroup,
            Role::Region,
            Role::RootWebArea,
            Role::Ruby,
            Role::RubyAnnotation,
            Role::ScrollBar,
            Role::ScrollView,
            Role::Search,
            Role::Section,
            Role::SectionFooter,
            Role::SectionHeader,
            Role::Slider,
            Role::SpinButton,
            Role::Splitter,
            Role::Status,
            Role::Strong,
            Role::Suggestion,
            Role::SvgRoot,
            Role::Tab,
            Role::TabList,
            Role::TabPanel,
            Role::Term,
            Role::Time,
            Role::Timer,
            Role::TitleBar,
            Role::Toolbar,
            Role::Tooltip,
            Role::Tree,
            Role::TreeGrid,
            Role::Video,
            Role::WebView,
            Role::Window,
            Role::PdfActionableHighlight,
            Role::PdfRoot,
            Role::GraphicsDocument,
            Role::GraphicsObject,
            Role::GraphicsSymbol,
            Role::DocAbstract,
            Role::DocAcknowledgements,
            Role::DocAfterword,
            Role::DocAppendix,
            Role::DocBackLink,
            Role::DocBiblioEntry,
            Role::DocBibliography,
            Role::DocBiblioRef,
            Role::DocChapter,
            Role::DocColophon,
            Role::DocConclusion,
            Role::DocCover,
            Role::DocCredit,
            Role::DocCredits,
            Role::DocDedication,
            Role::DocEndnote,
            Role::DocEndnotes,
            Role::DocEpigraph,
            Role::DocEpilogue,
            Role::DocErrata,
            Role::DocExample,
            Role::DocFootnote,
            Role::DocForeword,
            Role::DocGlossary,
            Role::DocGlossRef,
            Role::DocIndex,
            Role::DocIntroduction,
            Role::DocNoteRef,
            Role::DocNotice,
            Role::DocPageBreak,
            Role::DocPageFooter,
            Role::DocPageHeader,
            Role::DocPageList,
            Role::DocPart,
            Role::DocPreface,
            Role::DocPrologue,
            Role::DocPullquote,
            Role::DocQna,
            Role::DocSubtitle,
            Role::DocTip,
            Role::DocToc,
            Role::ListGrid,
            Role::Terminal,
        ]
    }

    fn labels() -> Vec<String> {
        Self::options()
            .iter()
            .map(|v| {
                serde_json::to_string(v)
                    .unwrap()
                    .trim_matches('"')
                    .to_string()
            })
            .collect()
    }

    fn from_label(label: &str) -> Option<Self> {
        serde_json::from_str(&format!("\"{}\"", label)).ok()
    }
}

crate::default_style!(accesskit::Role => RoleStyle);

impl Elicitation for Role {
    type Style = RoleStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting accesskit::Role");
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose accessibility role:"),
            &Self::labels(),
        );
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;
        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid accesskit::Role: {label}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("accesskit::Role", "unknown")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("accesskit::Role", "unknown")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("accesskit::Role", "unknown")
    }
}

impl ElicitIntrospect for Role {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "accesskit::Role",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels()
                    .into_iter()
                    .map(|label| VariantMetadata {
                        label,
                        fields: vec![],
                    })
                    .collect(),
            },
        }
    }
}
