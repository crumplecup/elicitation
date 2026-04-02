//! `Regex` — elicitation-enabled wrapper around `regex::Regex`.

use std::sync::Arc;

use elicitation_derive::reflect_methods;
use schemars::{JsonSchema, SchemaGenerator, json_schema};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tracing::instrument;

/// Elicitation-enabled wrapper around `regex::Regex`.
///
/// Serializes to/from the pattern string (e.g. `"^hello\\s+world$"`).
#[derive(Debug, Clone)]
pub struct Regex(pub Arc<regex::Regex>);

impl JsonSchema for Regex {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Regex".into()
    }

    fn json_schema(_gen: &mut SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "type": "string",
            "description": "A regular expression pattern string (e.g. \"^hello\\\\s+world$\")"
        })
    }
}

impl Serialize for Regex {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.0.as_str())
    }
}

impl<'de> Deserialize<'de> for Regex {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let pattern = String::deserialize(d)?;
        regex::Regex::new(&pattern)
            .map(|r| Arc::new(r).into())
            .map_err(serde::de::Error::custom)
    }
}

impl std::ops::Deref for Regex {
    type Target = regex::Regex;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::convert::AsRef<regex::Regex> for Regex {
    fn as_ref(&self) -> &regex::Regex {
        &self.0
    }
}

impl From<regex::Regex> for Regex {
    fn from(inner: regex::Regex) -> Self {
        Self(Arc::new(inner))
    }
}

impl From<Arc<regex::Regex>> for Regex {
    fn from(arc: Arc<regex::Regex>) -> Self {
        Self(arc)
    }
}

#[reflect_methods]
impl Regex {
    /// Returns the pattern string of this regex.
    #[instrument(skip(self))]
    pub fn as_str(&self) -> String {
        self.0.as_str().to_string()
    }

    /// Returns the number of capture groups (excluding the implicit whole-match group).
    #[instrument(skip(self))]
    pub fn captures_len(&self) -> usize {
        self.0.captures_len().saturating_sub(1)
    }

    /// Returns `true` if the pattern matches anywhere in `text`.
    #[instrument(skip(self))]
    pub fn is_match(&self, text: String) -> bool {
        self.0.is_match(&text)
    }

    /// Returns the text of the first match in `text`, or `None` if no match.
    #[instrument(skip(self))]
    pub fn find(&self, text: String) -> Option<String> {
        self.0.find(&text).map(|m| m.as_str().to_string())
    }

    /// Returns all non-overlapping matches in `text`.
    #[instrument(skip(self))]
    pub fn find_all(&self, text: String) -> Vec<String> {
        self.0
            .find_iter(&text)
            .map(|m| m.as_str().to_string())
            .collect()
    }

    /// Replaces all non-overlapping matches in `text` with `replacement`.
    ///
    /// Use `$1`, `$2`, … in `replacement` to refer to capture groups.
    #[instrument(skip(self))]
    pub fn replace_all(&self, text: String, replacement: String) -> String {
        self.0.replace_all(&text, replacement.as_str()).into_owned()
    }
}

impl Regex {
    /// Compile a pattern string into a [`Regex`].
    ///
    /// Returns `None` if the pattern is invalid.
    pub fn new(pattern: &str) -> Option<Self> {
        regex::Regex::new(pattern).ok().map(Into::into)
    }
}

// ── Elicitation framework traits ─────────────────────────────────────────────

impl elicitation::Prompt for Regex {
    fn prompt() -> Option<&'static str> {
        Some("Enter a regular expression pattern (e.g. ^[a-z]+$):")
    }
}

impl elicitation::Elicitation for Regex {
    type Style = ();

    async fn elicit<C: elicitation::ElicitCommunicator>(
        communicator: &C,
    ) -> elicitation::ElicitResult<Self> {
        let response = communicator
            .send_prompt("Enter a regular expression pattern (e.g. ^[a-z]+$):")
            .await?;
        let inner = regex::Regex::new(&response)
            .map_err(|e| elicitation::ElicitError::new(elicitation::ElicitErrorKind::ParseError(
                format!("Invalid regex pattern: {e}"),
            )))?;
        Ok(Self(Arc::new(inner)))
    }

    fn kani_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
    fn verus_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
    fn creusot_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
}

impl elicitation::ElicitIntrospect for Regex {
    fn pattern() -> elicitation::ElicitationPattern { elicitation::ElicitationPattern::Primitive }
    fn metadata() -> elicitation::TypeMetadata {
        elicitation::TypeMetadata {
            type_name: "Regex",
            description: <Self as elicitation::Prompt>::prompt(),
            details: elicitation::PatternDetails::Primitive,
        }
    }
}

impl elicitation::ElicitPromptTree for Regex {
    fn prompt_tree() -> elicitation::PromptTree {
        elicitation::PromptTree::Leaf {
            prompt: "Regex".to_string(),
            type_name: "Regex".to_string(),
        }
    }
}

impl elicitation::ElicitSpec for Regex {
    fn type_spec() -> elicitation::TypeSpec {
        elicitation::TypeSpecBuilder::default()
            .type_name("Regex".to_string())
            .summary("A compiled regular expression pattern.".to_string())
            .build()
            .expect("valid TypeSpec")
    }
}

mod emit_impls {
    use super::Regex;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Regex {
        fn to_code_literal(&self) -> TokenStream {
            let s = self.0.as_str().to_string();
            quote::quote! {
                ::elicit_regex::Regex::from(
                    ::regex::Regex::new(#s).expect("valid regex")
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for Regex {}
