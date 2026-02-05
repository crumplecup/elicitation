//! Default style enums for types with no custom styles.

/// Generate a default-only style enum for a type.
///
/// For types that don't have custom style variants, this macro generates
/// a simple enum with only a `Default` variant.
#[macro_export]
macro_rules! default_style {
    ($type_name:ty => $style_name:ident) => {
        /// Default-only style enum.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
        pub enum $style_name {
            /// Default presentation style.
            #[default]
            Default,
        }

        impl $crate::Prompt for $style_name {
            fn prompt() -> Option<&'static str> {
                None // No style selection needed - only one option
            }
        }

        impl $crate::Elicitation for $style_name {
            type Style = $style_name; // Self-reference

            async fn elicit(_client: &$crate::ElicitClient) -> $crate::ElicitResult<Self> {
                Ok(Self::Default) // Always default
            }
        }
    };
}
