//! Builder wrapper for validated WKB byte buffers.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, FieldInfo, PatternDetails, Prompt, TypeMetadata, WkbDimension,
    WkbEndianness, WkbGeometryType,
};

/// Builder type for validated WKB bytes.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct WkbBytes {
    /// The raw validated WKB bytes.
    pub bytes: Vec<u8>,
}

impl WkbBytes {
    /// Creates validated WKB bytes from an owned byte vector.
    pub fn new(bytes: Vec<u8>) -> Result<Self, String> {
        wkb::reader::read_wkb(&bytes).map_err(|error| error.to_string())?;
        Ok(Self { bytes })
    }

    /// Decodes and validates WKB bytes from a hexadecimal string.
    pub fn from_hex(hex: &str) -> Result<Self, String> {
        let bytes = hex::decode(hex).map_err(|error| error.to_string())?;
        Self::new(bytes)
    }

    /// Returns the bytes as an uppercase-free hexadecimal string.
    pub fn hex_string(&self) -> String {
        hex::encode(&self.bytes)
    }

    fn parsed(&self) -> wkb::reader::Wkb<'_> {
        wkb::reader::read_wkb(&self.bytes).expect("WkbBytes is validated on construction")
    }

    /// Returns the byte order encoded at the start of the WKB payload.
    pub fn endianness(&self) -> WkbEndianness {
        wkb::Endianness::try_from(self.bytes[0])
            .expect("validated WKB starts with a valid endianness byte")
            .into()
    }

    /// Returns the parsed coordinate dimension.
    pub fn dimension(&self) -> WkbDimension {
        self.parsed().dimension().into()
    }

    /// Returns the parsed geometry type.
    pub fn geometry_type(&self) -> Result<WkbGeometryType, String> {
        self.parsed().geometry_type().try_into()
    }
}

crate::default_style!(WkbBytes => WkbBytesStyle);

impl Prompt for WkbBytes {
    fn prompt() -> Option<&'static str> {
        Some("Enter a WKB geometry as a hex string:")
    }
}

impl Elicitation for WkbBytes {
    type Style = WkbBytesStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let raw = String::elicit(communicator).await?;
        Self::from_hex(raw.trim()).map_err(|error| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid WKB hex string: {error}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        Vec::<u8>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        Vec::<u8>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        Vec::<u8>::creusot_proof()
    }
}

impl ElicitIntrospect for WkbBytes {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "WkbBytes",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "bytes",
                    type_name: "Vec<u8>",
                    prompt: Some("Hex-encoded WKB payload:"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for WkbBytes {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "WkbBytes".to_string(),
            fields: vec![("bytes".to_string(), Box::new(String::prompt_tree()))],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for WkbBytes {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let bytes = &self.bytes;
        quote::quote! {
            ::elicitation::WkbBytes::new(vec![#(#bytes),*]).expect("valid WKB bytes")
        }
    }
}
