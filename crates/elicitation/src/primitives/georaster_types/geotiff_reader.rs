//! Elicitation support for [`georaster::geotiff::GeoTiffReader`].
//!
//! `GeoTiffReader<R>` is a runtime I/O resource that wraps a GeoTIFF file
//! decoder. It cannot be constructed from user input, so `elicit` returns
//! an error explaining the constraint. All other traits describe the type.
//!
//! Bounds `R: Read + Seek + Send + Sync + 'static` are required by the
//! `Elicitation: 'static` and async-future `Send` constraints.

use std::io::{Read, Seek};

use crate::{
    ElicitCommunicator, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, TypeMetadata,
};
use georaster::geotiff::GeoTiffReader;

crate::default_style!(georaster::geotiff::GeoTiffReader<std::fs::File> => GeoRasterGeoTiffReaderStyle);

impl<R: Read + Seek + Send + Sync + 'static> Prompt for GeoTiffReader<R> {
    fn prompt() -> Option<&'static str> {
        Some("GeoTIFF file reader — a runtime I/O resource, not interactively constructable")
    }
}

impl<R: Read + Seek + Send + Sync + 'static> Elicitation for GeoTiffReader<R> {
    type Style = GeoRasterGeoTiffReaderStyle;

    #[tracing::instrument(skip(_communicator))]
    async fn elicit<C: ElicitCommunicator>(_communicator: &C) -> ElicitResult<Self> {
        Err(ElicitErrorKind::InvalidFormat {
            expected: "a runtime GeoTIFF file reader — open via GeoTiffReader::open".into(),
            received: "GeoTiffReader is an I/O resource and cannot be elicited interactively"
                .into(),
        }
        .into())
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("georaster::geotiff::GeoTiffReader")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque(
            "georaster::geotiff::GeoTiffReader",
        )
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque(
            "georaster::geotiff::GeoTiffReader",
        )
    }
}

impl<R: Read + Seek + Send + Sync + 'static> ElicitIntrospect for GeoTiffReader<R> {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "georaster::geotiff::GeoTiffReader",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

impl<R: Read + Seek + Send + Sync + 'static> crate::ElicitPromptTree for GeoTiffReader<R> {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Leaf {
            prompt:
                "GeoTiffReader is a runtime I/O resource and cannot be constructed interactively."
                    .to_string(),
            type_name: "georaster::geotiff::GeoTiffReader".to_string(),
        }
    }
}

impl<R: Read + Seek + Send + Sync + 'static> crate::emit_code::ToCodeLiteral for GeoTiffReader<R> {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        // GeoTiffReader is an I/O resource with no public constructor — no code literal.
        unimplemented!("GeoTiffReader cannot be converted to a code literal")
    }
}
