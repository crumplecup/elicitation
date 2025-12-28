//! Interaction paradigm traits for elicitation patterns.

use crate::Prompt;

/// Select one value from a finite set (dropdown/radio button pattern).
///
/// This trait represents the conversational equivalent of a dropdown menu or
/// radio button group. It is the natural elicitation mode for enums and
/// categorical fields.
///
/// # Example
///
/// ```rust,no_run
/// use elicitation::{Select, Prompt};
///
/// #[derive(Debug, Clone, Copy)]
/// enum Color {
///     Red,
///     Green,
///     Blue,
/// }
///
/// impl Prompt for Color {
///     fn prompt() -> Option<&'static str> {
///         Some("Choose a color:")
///     }
/// }
///
/// impl Select for Color {
///     fn options() -> &'static [Self] {
///         &[Color::Red, Color::Green, Color::Blue]
///     }
///
///     fn labels() -> &'static [&'static str] {
///         &["Red", "Green", "Blue"]
///     }
///
///     fn from_label(label: &str) -> Option<Self> {
///         match label {
///             "Red" => Some(Color::Red),
///             "Green" => Some(Color::Green),
///             "Blue" => Some(Color::Blue),
///             _ => None,
///         }
///     }
/// }
/// ```
pub trait Select: Prompt + Sized {
    /// All valid options for this selection.
    ///
    /// Returns a static slice of all possible values. The MCP tool will
    /// ensure the user selects one of these options.
    fn options() -> &'static [Self];

    /// Human-readable labels for each option.
    ///
    /// Returns labels corresponding to each value in `options()`. These
    /// are presented to the user and used to parse their selection.
    fn labels() -> &'static [&'static str];

    /// Parse a label back into the type.
    ///
    /// Given a label string (from user selection), returns the corresponding
    /// value, or `None` if the label is invalid.
    ///
    /// # Arguments
    ///
    /// * `label` - The label to parse
    ///
    /// # Returns
    ///
    /// `Some(Self)` if the label is valid, `None` otherwise.
    fn from_label(label: &str) -> Option<Self>;
}

/// Binary confirmation (yes/no, true/false pattern).
///
/// This trait represents a yes/no confirmation dialog. It is the natural
/// elicitation mode for boolean fields and confirmation prompts.
///
/// # Example
///
/// ```rust
/// use elicitation::Affirm;
///
/// // bool implements Affirm by default
/// fn requires_affirm<T: Affirm>() {}
/// requires_affirm::<bool>();
/// ```
pub trait Affirm: Prompt {}

/// Multi-field structured elicitation (form/wizard pattern).
///
/// This trait represents a multi-step form or wizard. It is the natural
/// elicitation mode for structs and configuration objects.
///
/// The derive macro generates implementations of this trait for structs,
/// creating a state machine that elicits each field in sequence.
pub trait Survey: Prompt {
    /// Field metadata for survey construction.
    ///
    /// Returns information about each field in the struct, used to drive
    /// the elicitation state machine.
    fn fields() -> &'static [FieldInfo];
}

/// Metadata for a single survey field.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FieldInfo {
    /// Field name in the struct.
    pub name: &'static str,
    /// Optional custom prompt for this field.
    pub prompt: Option<&'static str>,
    /// Type name for dispatching elicitation.
    pub type_name: &'static str,
}

/// Permission-granting interaction with policy choices.
///
/// This trait represents a permission dialog with multiple policy options.
/// It is used for authorization decisions and policy selection.
///
/// **Note**: Implementation of this paradigm is planned for v0.2.0.
pub trait Authorize: Prompt + Sized {
    /// Available permission policies.
    ///
    /// Returns a static slice of all available policy options.
    fn policies() -> &'static [Self];
}
