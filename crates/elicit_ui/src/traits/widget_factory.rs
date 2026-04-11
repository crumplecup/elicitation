//! Create typed UI widgets with WCAG proof tokens.

use elicitation::Established;

use crate::{
    AltTextProvided, HasLabel, KeyboardAccessible, MinTargetSize, StructuredContent, UiResult,
    ValidRole, WidgetId,
};

/// Create typed UI widgets with WCAG proof tokens.
///
/// Every factory method returns proof tokens as part of its return type.
/// A backend *cannot* implement this trait without producing the tokens,
/// enforcing WCAG at the contract level.
pub trait UiWidgetFactory: Send + Sync {
    /// Create a button with accessible label and minimum target size.
    fn create_button(
        &self,
        label: &str,
        width: u32,
        height: u32,
    ) -> UiResult<(
        WidgetId,
        Established<HasLabel>,
        Established<MinTargetSize>,
        Established<KeyboardAccessible>,
    )>;

    /// Create a plain text label.
    fn create_label(
        &self,
        text: &str,
        role_hint: &str,
    ) -> UiResult<(WidgetId, Established<HasLabel>, Established<ValidRole>)>;

    /// Create a text input field.
    fn create_input(
        &self,
        label: &str,
        input_type: &str,
    ) -> UiResult<(
        WidgetId,
        Established<HasLabel>,
        Established<KeyboardAccessible>,
    )>;

    /// Create an image with alt text.
    fn create_image(
        &self,
        alt_text: &str,
        src: &str,
    ) -> UiResult<(
        WidgetId,
        Established<HasLabel>,
        Established<AltTextProvided>,
    )>;

    /// Create a heading with appropriate level (1–6).
    fn create_heading(
        &self,
        text: &str,
        level: u8,
    ) -> UiResult<(
        WidgetId,
        Established<HasLabel>,
        Established<ValidRole>,
        Established<StructuredContent>,
    )>;

    /// Create a hyperlink.
    fn create_link(
        &self,
        text: &str,
        href: &str,
    ) -> UiResult<(
        WidgetId,
        Established<HasLabel>,
        Established<KeyboardAccessible>,
    )>;

    /// Create a data table with caption.
    fn create_table(
        &self,
        caption: &str,
        headers: Vec<String>,
    ) -> UiResult<(
        WidgetId,
        Established<HasLabel>,
        Established<StructuredContent>,
    )>;

    /// Create a checkbox.
    fn create_checkbox(
        &self,
        label: &str,
        checked: bool,
    ) -> UiResult<(
        WidgetId,
        Established<HasLabel>,
        Established<KeyboardAccessible>,
    )>;

    /// Create a select/dropdown.
    fn create_select(
        &self,
        label: &str,
        options: Vec<String>,
    ) -> UiResult<(
        WidgetId,
        Established<HasLabel>,
        Established<KeyboardAccessible>,
    )>;
}
