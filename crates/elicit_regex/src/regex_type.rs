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
