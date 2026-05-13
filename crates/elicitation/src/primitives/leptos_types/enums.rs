//! Leptos mode and HTML tag enums.
//!
//! Available with the `leptos-types` feature.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Leptos rendering mode.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    JsonSchema,
    strum::EnumIter,
    derive_more::Display,
)]
pub enum LeptosMode {
    /// Client-side rendering only.
    #[display("csr")]
    Csr,
    /// Server-side rendering with hydration.
    #[display("ssr")]
    Ssr,
    /// Hydration mode (SSR + client hydration).
    #[display("hydrate")]
    Hydrate,
    /// Islands architecture.
    #[display("islands")]
    Islands,
}

/// Common HTML5 element tags.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    JsonSchema,
    strum::EnumIter,
    derive_more::Display,
)]
pub enum LeptosHtmlTag {
    /// Block container.
    #[display("div")]
    Div,
    /// Inline container.
    #[display("span")]
    Span,
    /// Paragraph.
    #[display("p")]
    P,
    /// Anchor/link.
    #[display("a")]
    A,
    /// Button element.
    #[display("button")]
    Button,
    /// Input element.
    #[display("input")]
    Input,
    /// Form element.
    #[display("form")]
    Form,
    /// Heading level 1.
    #[display("h1")]
    H1,
    /// Heading level 2.
    #[display("h2")]
    H2,
    /// Heading level 3.
    #[display("h3")]
    H3,
    /// Unordered list.
    #[display("ul")]
    Ul,
    /// Ordered list.
    #[display("ol")]
    Ol,
    /// List item.
    #[display("li")]
    Li,
    /// Image element.
    #[display("img")]
    Img,
    /// Navigation.
    #[display("nav")]
    Nav,
    /// Main content.
    #[display("main")]
    Main,
    /// Section.
    #[display("section")]
    Section,
    /// Article.
    #[display("article")]
    Article,
    /// Page header.
    #[display("header")]
    Header,
    /// Page footer.
    #[display("footer")]
    Footer,
    /// Aside/sidebar.
    #[display("aside")]
    Aside,
    /// Table.
    #[display("table")]
    Table,
    /// Table row.
    #[display("tr")]
    Tr,
    /// Table data cell.
    #[display("td")]
    Td,
    /// Table header cell.
    #[display("th")]
    Th,
    /// Select dropdown.
    #[display("select")]
    Select,
    /// Option element (named Option_ to avoid keyword clash).
    #[display("option")]
    Option_,
    /// Text area.
    #[display("textarea")]
    Textarea,
    /// Label.
    #[display("label")]
    Label,
    /// Bold/strong.
    #[display("strong")]
    Strong,
    /// Italic/emphasis.
    #[display("em")]
    Em,
    /// Inline code.
    #[display("code")]
    Code,
    /// Preformatted text.
    #[display("pre")]
    Pre,
}
