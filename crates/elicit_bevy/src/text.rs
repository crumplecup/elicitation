//! Text layout wrappers.
//!
//! Covers [`JustifyText`] (wraps `bevy::text::Justify`), [`LineBreak`],
//! [`TextFont`], [`FontSmoothing`], [`TextColor`], [`TextLayout`],
//! [`TextSpan`], [`FontWeight`], and [`TextBounds`].

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;
use std::sync::Arc;

// ── JustifyText ───────────────────────────────────────────────────────────────
//
// In Bevy 0.18 the type is `bevy::text::Justify`, aliased as `JustifyText`.

elicit_newtype!(bevy::text::Justify, as JustifyText);
elicit_newtype_traits!(JustifyText, bevy::text::Justify, [eq]);

impl From<JustifyText> for bevy::text::Justify {
    fn from(v: JustifyText) -> Self {
        *v.0
    }
}

impl serde::Serialize for JustifyText {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for JustifyText {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        bevy::text::Justify::deserialize(deserializer).map(|v| JustifyText(Arc::new(v)))
    }
}

#[reflect_methods]
impl JustifyText {
    /// Returns the variant name as a static string.
    #[tracing::instrument(skip(self))]
    pub fn as_str(&self) -> &'static str {
        match *self.0 {
            bevy::text::Justify::Left => "Left",
            bevy::text::Justify::Center => "Center",
            bevy::text::Justify::Right => "Right",
            bevy::text::Justify::Justified => "Justified",
        }
    }

    /// Returns `true` if this is [`Justify::Left`].
    #[tracing::instrument(skip(self))]
    pub fn is_left(&self) -> bool {
        matches!(*self.0, bevy::text::Justify::Left)
    }

    /// Returns `true` if this is [`Justify::Center`].
    #[tracing::instrument(skip(self))]
    pub fn is_center(&self) -> bool {
        matches!(*self.0, bevy::text::Justify::Center)
    }

    /// Returns `true` if this is [`Justify::Right`].
    #[tracing::instrument(skip(self))]
    pub fn is_right(&self) -> bool {
        matches!(*self.0, bevy::text::Justify::Right)
    }

    /// Returns `true` if this is [`Justify::Justified`].
    #[tracing::instrument(skip(self))]
    pub fn is_justified(&self) -> bool {
        matches!(*self.0, bevy::text::Justify::Justified)
    }
}

mod emit_impls_justify {
    use super::JustifyText;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for JustifyText {
        fn to_code_literal(&self) -> TokenStream {
            let variant = quote::format_ident!("{}", self.as_str());
            quote::quote! {
                ::elicit_bevy::JustifyText::from(::bevy::text::Justify::#variant)
            }
        }
    }
}

impl elicitation::ElicitComplete for JustifyText {}

// ── LineBreak ─────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::text::LineBreak, as LineBreak);
elicit_newtype_traits!(LineBreak, bevy::text::LineBreak, [eq]);

impl From<LineBreak> for bevy::text::LineBreak {
    fn from(v: LineBreak) -> Self {
        *v.0
    }
}

impl serde::Serialize for LineBreak {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for LineBreak {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        let s = String::deserialize(deserializer)?;
        let inner = match s.as_str() {
            "WordBoundary" => bevy::text::LineBreak::WordBoundary,
            "AnyCharacter" => bevy::text::LineBreak::AnyCharacter,
            "WordOrCharacter" => bevy::text::LineBreak::WordOrCharacter,
            "NoWrap" => bevy::text::LineBreak::NoWrap,
            _ => {
                return Err(D::Error::unknown_variant(
                    &s,
                    &["WordBoundary", "AnyCharacter", "WordOrCharacter", "NoWrap"],
                ));
            }
        };
        Ok(LineBreak(Arc::new(inner)))
    }
}

#[reflect_methods]
impl LineBreak {
    /// Returns the variant name as a static string.
    #[tracing::instrument(skip(self))]
    pub fn as_str(&self) -> &'static str {
        match *self.0 {
            bevy::text::LineBreak::WordBoundary => "WordBoundary",
            bevy::text::LineBreak::AnyCharacter => "AnyCharacter",
            bevy::text::LineBreak::WordOrCharacter => "WordOrCharacter",
            bevy::text::LineBreak::NoWrap => "NoWrap",
        }
    }

    /// Returns `true` if this is [`LineBreak::WordBoundary`].
    #[tracing::instrument(skip(self))]
    pub fn is_word(&self) -> bool {
        matches!(*self.0, bevy::text::LineBreak::WordBoundary)
    }

    /// Returns `true` if this is [`LineBreak::AnyCharacter`].
    #[tracing::instrument(skip(self))]
    pub fn is_char(&self) -> bool {
        matches!(*self.0, bevy::text::LineBreak::AnyCharacter)
    }

    /// Returns `true` if this is [`LineBreak::NoWrap`].
    #[tracing::instrument(skip(self))]
    pub fn is_no_wrap(&self) -> bool {
        matches!(*self.0, bevy::text::LineBreak::NoWrap)
    }
}

mod emit_impls_linebreak {
    use super::LineBreak;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for LineBreak {
        fn to_code_literal(&self) -> TokenStream {
            let variant = quote::format_ident!("{}", self.as_str());
            quote::quote! {
                ::elicit_bevy::LineBreak::from(::bevy::text::LineBreak::#variant)
            }
        }
    }
}

impl elicitation::ElicitComplete for LineBreak {}

// ── FontSmoothing ─────────────────────────────────────────────────────────────

elicit_newtype!(bevy::text::FontSmoothing, as FontSmoothing);
elicit_newtype_traits!(FontSmoothing, bevy::text::FontSmoothing, [eq]);

impl From<FontSmoothing> for bevy::text::FontSmoothing {
    fn from(v: FontSmoothing) -> Self {
        *v.0
    }
}

impl serde::Serialize for FontSmoothing {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for FontSmoothing {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        let s = String::deserialize(deserializer)?;
        let inner = match s.as_str() {
            "None" => bevy::text::FontSmoothing::None,
            "AntiAliased" => bevy::text::FontSmoothing::AntiAliased,
            _ => {
                return Err(D::Error::unknown_variant(&s, &["None", "AntiAliased"]));
            }
        };
        Ok(FontSmoothing(Arc::new(inner)))
    }
}

#[reflect_methods]
impl FontSmoothing {
    /// Returns the variant name as a static string.
    #[tracing::instrument(skip(self))]
    pub fn as_str(&self) -> &'static str {
        match *self.0 {
            bevy::text::FontSmoothing::None => "None",
            bevy::text::FontSmoothing::AntiAliased => "AntiAliased",
        }
    }

    /// Returns `true` if this is [`FontSmoothing::AntiAliased`].
    #[tracing::instrument(skip(self))]
    pub fn is_antialiased(&self) -> bool {
        matches!(*self.0, bevy::text::FontSmoothing::AntiAliased)
    }

    /// Returns `true` if this is [`FontSmoothing::None`] (no antialiasing).
    #[tracing::instrument(skip(self))]
    pub fn is_none(&self) -> bool {
        matches!(*self.0, bevy::text::FontSmoothing::None)
    }
}

mod emit_impls_smoothing {
    use super::FontSmoothing;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for FontSmoothing {
        fn to_code_literal(&self) -> TokenStream {
            let variant = quote::format_ident!("{}", self.as_str());
            quote::quote! {
                ::elicit_bevy::FontSmoothing::from(::bevy::text::FontSmoothing::#variant)
            }
        }
    }
}

impl elicitation::ElicitComplete for FontSmoothing {}

// ── TextFont ──────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::text::TextFont, as TextFont);
elicit_newtype_traits!(TextFont, bevy::text::TextFont, []);

impl From<TextFont> for bevy::text::TextFont {
    fn from(v: TextFont) -> Self {
        Arc::try_unwrap(v.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

impl serde::Serialize for TextFont {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("font_size", &self.0.font_size)?;
        map.serialize_entry(
            "font_smoothing",
            &FontSmoothing::from(self.0.font_smoothing),
        )?;
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for TextFont {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = TextFont;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "a TextFont JSON object")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<TextFont, A::Error> {
                let mut font_size: Option<f32> = None;
                let mut font_smoothing: Option<FontSmoothing> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "font_size" => font_size = Some(map.next_value()?),
                        "font_smoothing" => font_smoothing = Some(map.next_value()?),
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                let mut t = bevy::text::TextFont::default();
                if let Some(s) = font_size {
                    t.font_size = s;
                }
                if let Some(sm) = font_smoothing {
                    t.font_smoothing = *sm.0;
                }
                Ok(TextFont(Arc::new(t)))
            }
        }
        deserializer.deserialize_map(V)
    }
}

#[reflect_methods]
impl TextFont {
    /// Returns the font size in logical pixels.
    #[tracing::instrument(skip(self))]
    pub fn font_size(&self) -> f32 {
        self.0.font_size
    }

    /// Returns the font smoothing mode.
    #[tracing::instrument(skip(self))]
    pub fn font_smoothing(&self) -> FontSmoothing {
        FontSmoothing::from(self.0.font_smoothing)
    }

    /// Returns a copy with the given font size.
    #[tracing::instrument(skip(self))]
    pub fn with_font_size(&self, size: f32) -> TextFont {
        let mut t = (*self.0).clone();
        t.font_size = size;
        TextFont::from(t)
    }

    /// Creates a new [`TextFont`] with the given font size (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn new_text_font(&self, font_size: f32) -> TextFont {
        let mut t = bevy::text::TextFont::default();
        t.font_size = font_size;
        TextFont::from(t)
    }
}

mod emit_impls_text_font {
    use super::TextFont;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for TextFont {
        fn to_code_literal(&self) -> TokenStream {
            let size = self.0.font_size;
            quote::quote! {
                ::elicit_bevy::TextFont::from({
                    let mut t = ::bevy::text::TextFont::default();
                    t.font_size = #size;
                    t
                })
            }
        }
    }
}

impl elicitation::ElicitComplete for TextFont {}

// ── TextColor ─────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::text::TextColor, as TextColor);
elicit_newtype_traits!(TextColor, bevy::text::TextColor, [eq]);

impl From<TextColor> for bevy::text::TextColor {
    fn from(v: TextColor) -> Self {
        *v.0
    }
}

impl serde::Serialize for TextColor {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.0.0.serialize(s)
    }
}

impl<'de> serde::Deserialize<'de> for TextColor {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::color::Color::deserialize(d).map(|c| TextColor(Arc::new(bevy::text::TextColor(c))))
    }
}

#[reflect_methods]
impl TextColor {
    /// Returns the wrapped color as a [`crate::Color`] wrapper.
    #[tracing::instrument(skip(self))]
    pub fn get_color(&self) -> crate::Color {
        crate::Color::from(self.0.0)
    }
}

mod emit_impls_text_color {
    use super::TextColor;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for TextColor {
        fn to_code_literal(&self) -> TokenStream {
            let color = crate::Color::from(self.0.0);
            let color_tokens = color.to_code_literal();
            quote::quote! {
                ::elicit_bevy::TextColor::from(#color_tokens)
            }
        }
    }
}

impl elicitation::ElicitComplete for TextColor {}

// ── TextLayout ────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::text::TextLayout, as TextLayout);
elicit_newtype_traits!(TextLayout, bevy::text::TextLayout, []);

impl From<TextLayout> for bevy::text::TextLayout {
    fn from(v: TextLayout) -> Self {
        *v.0
    }
}

impl serde::Serialize for TextLayout {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(2))?;
        let justify = match self.0.justify {
            bevy::text::Justify::Left => "Left",
            bevy::text::Justify::Center => "Center",
            bevy::text::Justify::Right => "Right",
            bevy::text::Justify::Justified => "Justified",
        };
        let linebreak = match self.0.linebreak {
            bevy::text::LineBreak::WordBoundary => "WordBoundary",
            bevy::text::LineBreak::AnyCharacter => "AnyCharacter",
            bevy::text::LineBreak::WordOrCharacter => "WordOrCharacter",
            bevy::text::LineBreak::NoWrap => "NoWrap",
        };
        map.serialize_entry("justify", justify)?;
        map.serialize_entry("linebreak", linebreak)?;
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for TextLayout {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = TextLayout;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    r#"an object with "justify" and "linebreak" string fields"#
                )
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<TextLayout, A::Error> {
                let mut justify: Option<String> = None;
                let mut linebreak: Option<String> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "justify" => justify = Some(map.next_value()?),
                        "linebreak" => linebreak = Some(map.next_value()?),
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                let j = match justify.as_deref().unwrap_or("Left") {
                    "Center" => bevy::text::Justify::Center,
                    "Right" => bevy::text::Justify::Right,
                    "Justified" => bevy::text::Justify::Justified,
                    _ => bevy::text::Justify::Left,
                };
                let lb = match linebreak.as_deref().unwrap_or("WordBoundary") {
                    "AnyCharacter" => bevy::text::LineBreak::AnyCharacter,
                    "WordOrCharacter" => bevy::text::LineBreak::WordOrCharacter,
                    "NoWrap" => bevy::text::LineBreak::NoWrap,
                    _ => bevy::text::LineBreak::WordBoundary,
                };
                Ok(TextLayout(Arc::new(bevy::text::TextLayout {
                    justify: j,
                    linebreak: lb,
                })))
            }
        }
        deserializer.deserialize_map(V)
    }
}

#[reflect_methods]
impl TextLayout {
    /// Returns the justify variant name as a static string.
    #[tracing::instrument(skip(self))]
    pub fn justify_str(&self) -> &'static str {
        match self.0.justify {
            bevy::text::Justify::Left => "Left",
            bevy::text::Justify::Center => "Center",
            bevy::text::Justify::Right => "Right",
            bevy::text::Justify::Justified => "Justified",
        }
    }

    /// Returns the linebreak variant name as a static string.
    #[tracing::instrument(skip(self))]
    pub fn linebreak_str(&self) -> &'static str {
        match self.0.linebreak {
            bevy::text::LineBreak::WordBoundary => "WordBoundary",
            bevy::text::LineBreak::AnyCharacter => "AnyCharacter",
            bevy::text::LineBreak::WordOrCharacter => "WordOrCharacter",
            bevy::text::LineBreak::NoWrap => "NoWrap",
        }
    }
}

mod emit_impls_text_layout {
    use super::TextLayout;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for TextLayout {
        fn to_code_literal(&self) -> TokenStream {
            let justify = quote::format_ident!("{}", self.justify_str());
            let linebreak = quote::format_ident!("{}", self.linebreak_str());
            quote::quote! {
                ::elicit_bevy::TextLayout::from(::bevy::text::TextLayout {
                    justify: ::bevy::text::Justify::#justify,
                    linebreak: ::bevy::text::LineBreak::#linebreak,
                })
            }
        }
    }
}

impl elicitation::ElicitComplete for TextLayout {}

// ── TextSpan ──────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::text::TextSpan, as TextSpan);
elicit_newtype_traits!(TextSpan, bevy::text::TextSpan, []);

impl From<TextSpan> for bevy::text::TextSpan {
    fn from(v: TextSpan) -> Self {
        bevy::text::TextSpan(v.0.0.clone())
    }
}

impl serde::Serialize for TextSpan {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.0.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for TextSpan {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Ok(TextSpan(Arc::new(bevy::text::TextSpan(s))))
    }
}

#[reflect_methods]
impl TextSpan {
    /// Returns the inner string value.
    #[tracing::instrument(skip(self))]
    pub fn as_str(&self) -> &str {
        &self.0.0
    }
}

mod emit_impls_text_span {
    use super::TextSpan;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for TextSpan {
        fn to_code_literal(&self) -> TokenStream {
            let s = &self.0.0;
            quote::quote! {
                ::elicit_bevy::TextSpan::from(::bevy::text::TextSpan(#s.to_string()))
            }
        }
    }
}

impl elicitation::ElicitComplete for TextSpan {}

// ── FontWeight ────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::text::FontWeight, as FontWeight);
elicit_newtype_traits!(FontWeight, bevy::text::FontWeight, [cmp]);

impl From<FontWeight> for bevy::text::FontWeight {
    fn from(v: FontWeight) -> Self {
        *v.0
    }
}

impl serde::Serialize for FontWeight {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.0.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for FontWeight {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let v = u16::deserialize(deserializer)?;
        Ok(FontWeight(Arc::new(bevy::text::FontWeight(v))))
    }
}

#[reflect_methods]
impl FontWeight {
    /// Returns the numeric weight value (100–900).
    #[tracing::instrument(skip(self))]
    pub fn value(&self) -> u16 {
        self.0.0
    }
}

mod emit_impls_font_weight {
    use super::FontWeight;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for FontWeight {
        fn to_code_literal(&self) -> TokenStream {
            let v = self.0.0;
            quote::quote! {
                ::elicit_bevy::FontWeight::from(::bevy::text::FontWeight(#v))
            }
        }
    }
}

impl elicitation::ElicitComplete for FontWeight {}

// ── unit_elicitation macro ────────────────────────────────────────────────────

macro_rules! unit_elicitation {
    ($name:ident, $inner_path:path) => {
        impl elicitation::Prompt for $name {
            fn prompt() -> Option<&'static str> {
                None
            }
        }
        impl elicitation::Elicitation for $name {
            type Style = ();
            async fn elicit<C: elicitation::ElicitCommunicator>(
                _communicator: &C,
            ) -> elicitation::ElicitResult<Self> {
                Ok(Self)
            }
            fn kani_proof() -> elicitation::proc_macro2::TokenStream {
                elicitation::verification::proof_helpers::kani_trusted_opaque(stringify!($name))
            }
            fn verus_proof() -> elicitation::proc_macro2::TokenStream {
                elicitation::verification::proof_helpers::verus_trusted_opaque(stringify!($name))
            }
            fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
                elicitation::verification::proof_helpers::creusot_trusted_opaque(stringify!($name))
            }
        }
        impl elicitation::ElicitIntrospect for $name {
            fn pattern() -> elicitation::ElicitationPattern {
                elicitation::ElicitationPattern::Primitive
            }
            fn metadata() -> elicitation::TypeMetadata {
                elicitation::TypeMetadata {
                    type_name: stringify!($name),
                    description: None,
                    details: elicitation::PatternDetails::Primitive,
                }
            }
        }
        impl elicitation::ElicitPromptTree for $name {
            fn prompt_tree() -> elicitation::PromptTree {
                elicitation::PromptTree::Leaf {
                    prompt: stringify!($name).to_string(),
                    type_name: stringify!($name).to_string(),
                }
            }
        }
        impl elicitation::ElicitSpec for $name {
            fn type_spec() -> elicitation::TypeSpec {
                elicitation::TypeSpecBuilder::default()
                    .type_name(stringify!($name).to_string())
                    .summary(
                        concat!(
                            "Marker component shadow for `",
                            stringify!($inner_path),
                            "`."
                        )
                        .to_string(),
                    )
                    .build()
                    .expect("valid TypeSpec")
            }
        }
        impl elicitation::ElicitComplete for $name {}
    };
}

// ── shadow_elicitation macro ──────────────────────────────────────────────────

macro_rules! shadow_elicitation {
    ($name:ident) => {
        impl elicitation::Prompt for $name {
            fn prompt() -> Option<&'static str> {
                None
            }
        }
        impl elicitation::Elicitation for $name {
            type Style = ();
            async fn elicit<C: elicitation::ElicitCommunicator>(
                communicator: &C,
            ) -> elicitation::ElicitResult<Self> {
                let response = communicator
                    .send_prompt(concat!("Enter value for ", stringify!($name)))
                    .await?;
                serde_json::from_str(&response)
                    .or_else(|_| serde_json::from_str::<Self>(&format!("\"{}\"", response)))
                    .map_err(|e| {
                        elicitation::ElicitError::new(elicitation::ElicitErrorKind::ParseError(
                            format!("Invalid {}: {}", stringify!($name), e),
                        ))
                    })
            }
            fn kani_proof() -> elicitation::proc_macro2::TokenStream {
                elicitation::verification::proof_helpers::kani_trusted_opaque(stringify!($name))
            }
            fn verus_proof() -> elicitation::proc_macro2::TokenStream {
                elicitation::verification::proof_helpers::verus_trusted_opaque(stringify!($name))
            }
            fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
                elicitation::verification::proof_helpers::creusot_trusted_opaque(stringify!($name))
            }
        }
        impl elicitation::ElicitIntrospect for $name {
            fn pattern() -> elicitation::ElicitationPattern {
                elicitation::ElicitationPattern::Primitive
            }
            fn metadata() -> elicitation::TypeMetadata {
                elicitation::TypeMetadata {
                    type_name: stringify!($name),
                    description: None,
                    details: elicitation::PatternDetails::Primitive,
                }
            }
        }
        impl elicitation::ElicitPromptTree for $name {
            fn prompt_tree() -> elicitation::PromptTree {
                elicitation::PromptTree::Leaf {
                    prompt: stringify!($name).to_string(),
                    type_name: stringify!($name).to_string(),
                }
            }
        }
        impl elicitation::ElicitSpec for $name {
            fn type_spec() -> elicitation::TypeSpec {
                elicitation::TypeSpecBuilder::default()
                    .type_name(stringify!($name).to_string())
                    .summary(concat!("Shadow type for `", stringify!($name), "`.").to_string())
                    .build()
                    .expect("valid TypeSpec")
            }
        }
        impl elicitation::ElicitComplete for $name {}
    };
}

// ── TextBounds ────────────────────────────────────────────────────────────────

/// Shadow for [`bevy::text::TextBounds`].
///
/// Add to a `Text2d` entity to limit its layout bounding box. Use `None` for
/// unconstrained (default behavior).
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct TextBounds {
    /// Maximum width in logical pixels (`None` = unbounded).
    pub width: Option<f32>,
    /// Maximum height in logical pixels (`None` = unbounded).
    pub height: Option<f32>,
}

impl From<TextBounds> for bevy::text::TextBounds {
    fn from(v: TextBounds) -> Self {
        Self {
            width: v.width,
            height: v.height,
        }
    }
}

mod emit_text_bounds {
    use super::TextBounds;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for TextBounds {
        fn to_code_literal(&self) -> TokenStream {
            let width = match self.width {
                None => quote::quote! { None },
                Some(w) => quote::quote! { Some(#w) },
            };
            let height = match self.height {
                None => quote::quote! { None },
                Some(h) => quote::quote! { Some(#h) },
            };
            quote::quote! { ::bevy::text::TextBounds { width: #width, height: #height } }
        }
    }
}

shadow_elicitation!(TextBounds);

// ── FontHinting ───────────────────────────────────────────────────────────────

/// Shadow of [`bevy::text::FontHinting`].
///
/// Font hinting strategy. `Disabled` (default) uses subpixel coordinates;
/// `Enabled` snaps glyphs to integral X coordinates during layout.
#[derive(
    Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub enum FontHinting {
    /// Glyphs use subpixel coordinates (default).
    #[default]
    Disabled,
    /// Glyphs are snapped to integral X coordinates during layout.
    Enabled,
}

impl From<FontHinting> for bevy::text::FontHinting {
    fn from(v: FontHinting) -> Self {
        match v {
            FontHinting::Disabled => bevy::text::FontHinting::Disabled,
            FontHinting::Enabled => bevy::text::FontHinting::Enabled,
        }
    }
}

mod emit_impls_font_hinting {
    use super::FontHinting;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for FontHinting {
        fn to_code_literal(&self) -> TokenStream {
            match self {
                FontHinting::Disabled => quote::quote! { ::bevy::text::FontHinting::Disabled },
                FontHinting::Enabled => quote::quote! { ::bevy::text::FontHinting::Enabled },
            }
        }
    }
}

shadow_elicitation!(FontHinting);

// ── Strikethrough ─────────────────────────────────────────────────────────────

/// Shadow of [`bevy::text::Strikethrough`].
///
/// Marker component: text entities with this component are drawn with a strikethrough line.
#[derive(
    Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct Strikethrough;

impl From<Strikethrough> for bevy::text::Strikethrough {
    fn from(_: Strikethrough) -> Self {
        bevy::text::Strikethrough
    }
}

mod emit_impls_strikethrough {
    use super::Strikethrough;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Strikethrough {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::bevy::text::Strikethrough }
        }
    }
}

unit_elicitation!(Strikethrough, bevy::text::Strikethrough);

// ── StrikethroughColor ────────────────────────────────────────────────────────

/// Shadow of [`bevy::text::StrikethroughColor`].
///
/// Color for the text's strikethrough line. If absent the entity's `TextColor` is used.
/// Serialized/deserialized as the inner [`Color`].
elicit_newtype!(bevy::text::StrikethroughColor, as StrikethroughColor);
elicit_newtype_traits!(StrikethroughColor, bevy::text::StrikethroughColor, [eq]);

impl From<StrikethroughColor> for bevy::text::StrikethroughColor {
    fn from(v: StrikethroughColor) -> Self {
        *v.0
    }
}

impl serde::Serialize for StrikethroughColor {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.0.0.serialize(s)
    }
}

impl<'de> serde::Deserialize<'de> for StrikethroughColor {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::color::Color::deserialize(d)
            .map(|c| StrikethroughColor(Arc::new(bevy::text::StrikethroughColor(c))))
    }
}

mod emit_impls_strikethrough_color {
    use super::StrikethroughColor;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for StrikethroughColor {
        fn to_code_literal(&self) -> TokenStream {
            let color = crate::Color::from(self.0.0);
            let color_tokens = color.to_code_literal();
            quote::quote! { ::elicit_bevy::StrikethroughColor::from(#color_tokens) }
        }
    }
}

impl elicitation::ElicitComplete for StrikethroughColor {}

// ── Underline ─────────────────────────────────────────────────────────────────

/// Shadow of [`bevy::text::Underline`].
///
/// Marker component: text entities with this component are drawn with an underline.
#[derive(
    Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct Underline;

impl From<Underline> for bevy::text::Underline {
    fn from(_: Underline) -> Self {
        bevy::text::Underline
    }
}

mod emit_impls_underline {
    use super::Underline;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Underline {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::bevy::text::Underline }
        }
    }
}

unit_elicitation!(Underline, bevy::text::Underline);

// ── UnderlineColor ────────────────────────────────────────────────────────────

/// Shadow of [`bevy::text::UnderlineColor`].
///
/// Color for the text's underline. If absent the entity's `TextColor` is used.
/// Serialized/deserialized as the inner [`Color`].
elicit_newtype!(bevy::text::UnderlineColor, as UnderlineColor);
elicit_newtype_traits!(UnderlineColor, bevy::text::UnderlineColor, [eq]);

impl From<UnderlineColor> for bevy::text::UnderlineColor {
    fn from(v: UnderlineColor) -> Self {
        *v.0
    }
}

impl serde::Serialize for UnderlineColor {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.0.0.serialize(s)
    }
}

impl<'de> serde::Deserialize<'de> for UnderlineColor {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::color::Color::deserialize(d)
            .map(|c| UnderlineColor(Arc::new(bevy::text::UnderlineColor(c))))
    }
}

mod emit_impls_underline_color {
    use super::UnderlineColor;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for UnderlineColor {
        fn to_code_literal(&self) -> TokenStream {
            let color = crate::Color::from(self.0.0);
            let color_tokens = color.to_code_literal();
            quote::quote! { ::elicit_bevy::UnderlineColor::from(#color_tokens) }
        }
    }
}

impl elicitation::ElicitComplete for UnderlineColor {}

// ── TextBackgroundColor ───────────────────────────────────────────────────────

/// Shadow of [`bevy::text::TextBackgroundColor`].
///
/// Background color behind a text section. Defaults to black.
/// Serialized/deserialized as the inner [`Color`].
elicit_newtype!(bevy::text::TextBackgroundColor, as TextBackgroundColor);
elicit_newtype_traits!(TextBackgroundColor, bevy::text::TextBackgroundColor, [eq]);

impl From<TextBackgroundColor> for bevy::text::TextBackgroundColor {
    fn from(v: TextBackgroundColor) -> Self {
        *v.0
    }
}

impl serde::Serialize for TextBackgroundColor {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.0.0.serialize(s)
    }
}

impl<'de> serde::Deserialize<'de> for TextBackgroundColor {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::color::Color::deserialize(d)
            .map(|c| TextBackgroundColor(Arc::new(bevy::text::TextBackgroundColor(c))))
    }
}

mod emit_impls_text_background_color {
    use super::TextBackgroundColor;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for TextBackgroundColor {
        fn to_code_literal(&self) -> TokenStream {
            let color = crate::Color::from(self.0.0);
            let color_tokens = color.to_code_literal();
            quote::quote! { ::elicit_bevy::TextBackgroundColor::from(#color_tokens) }
        }
    }
}

impl elicitation::ElicitComplete for TextBackgroundColor {}

// ── FontGenerator ─────────────────────────────────────────────────────────────

/// Generator for [`bevy::text::Font`] assets.
///
/// Fonts are loaded from disk via `AssetServer` at runtime. This generator
/// captures the asset path so that generated Bevy code can call
/// `asset_server.load(path)` at the appropriate time.
///
/// # Example
///
/// ```rust,ignore
/// use elicit_bevy::FontGenerator;
/// use elicitation::Generator;
///
/// let gen = FontGenerator::new("fonts/FiraSans-Bold.ttf".to_string());
/// // In generated code: asset_server.load("fonts/FiraSans-Bold.ttf")
/// ```
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation::Elicit,
)]
pub struct FontGenerator {
    /// Asset path relative to the Bevy `assets/` directory.
    pub path: String,
}

impl FontGenerator {
    /// Create a new `FontGenerator` for the given asset path.
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }
}

impl elicitation::Generator for FontGenerator {
    type Target = bevy::text::Font;

    #[tracing::instrument(skip(self))]
    fn generate(&self) -> bevy::text::Font {
        let data = std::fs::read(&self.path)
            .unwrap_or_else(|e| panic!("FontGenerator: failed to read '{}': {e}", self.path));
        bevy::text::Font::try_from_bytes(data)
            .unwrap_or_else(|e| panic!("FontGenerator: invalid font '{}': {e}", self.path))
    }
}

