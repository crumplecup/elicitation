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

            async fn elicit<C: $crate::ElicitCommunicator>(
                _communicator: &C,
            ) -> $crate::ElicitResult<Self> {
                Ok(Self::Default) // Always default
            }

            #[cfg(feature = "proofs")]
            fn kani_proof() -> proc_macro2::TokenStream {
                $crate::verification::proof_helpers::kani_single_variant_enum(stringify!(
                    $style_name
                ))
            }

            #[cfg(feature = "proofs")]
            fn verus_proof() -> proc_macro2::TokenStream {
                $crate::verification::proof_helpers::verus_single_variant_enum(stringify!(
                    $style_name
                ))
            }

            #[cfg(feature = "proofs")]
            fn creusot_proof() -> proc_macro2::TokenStream {
                $crate::verification::proof_helpers::creusot_single_variant_enum(stringify!(
                    $style_name
                ))
            }
        }

        /// Default styles use the default prompt formatting.
        impl $crate::style::ElicitationStyle for $style_name {}

        #[cfg(feature = "prompt-tree")]
        impl $crate::ElicitPromptTree for $style_name {
            fn prompt_tree() -> $crate::PromptTree {
                $crate::PromptTree::Leaf {
                    prompt: <$style_name as $crate::Prompt>::prompt()
                        .unwrap_or(stringify!($style_name))
                        .to_string(),
                    type_name: stringify!($style_name).to_string(),
                }
            }
        }
    };
}
