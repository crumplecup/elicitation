//! UUID contract types.
//!
//! Available with the `uuid` feature.

#[cfg(feature = "uuid")]
use super::ValidationError;
#[cfg(feature = "uuid")]
use crate::{ElicitCommunicator, ElicitResult, Elicitation, Prompt};
#[cfg(feature = "uuid")]
use anodized::spec;
#[cfg(all(feature = "uuid", not(kani)))]
use elicitation_derive::instrumented_impl;
#[cfg(feature = "uuid")]
use uuid::Uuid;

// UuidV4 - Only Version 4 (Random) UUIDs
/// A UUID that is guaranteed to be Version 4 (random).
///
/// Version 4 UUIDs are randomly generated and are the most common type.
///
/// Available with the `uuid` feature.
#[cfg(feature = "uuid")]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[cfg_attr(feature = "uuid", schemars(description = "A UUID v4 value"))]
pub struct UuidV4(Uuid);

#[cfg(feature = "uuid")]
#[cfg_attr(not(kani), instrumented_impl)]
impl UuidV4 {
    /// Create a new UuidV4, validating it's Version 4.
    #[spec(requires: [uuid.get_version_num() == 4])]
    #[spec(requires: [uuid != Uuid::nil()])]
    pub fn new(uuid: Uuid) -> Result<Self, ValidationError> {
        let version = uuid.get_version_num() as u8;
        if version == 4 {
            Ok(Self(uuid))
        } else {
            Err(ValidationError::WrongUuidVersion {
                expected: 4,
                got: version,
            })
        }
    }

    /// Get the inner UUID.
    pub fn get(&self) -> Uuid {
        self.0
    }

    /// Unwrap into the inner UUID.
    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

#[cfg(feature = "uuid")]
#[cfg_attr(not(kani), instrumented_impl)]
impl Prompt for UuidV4 {
    fn prompt() -> Option<&'static str> {
        Some("Please provide a Version 4 (random) UUID:")
    }
}

#[cfg(feature = "uuid")]
#[cfg_attr(not(kani), instrumented_impl)]
impl Elicitation for UuidV4 {
    type Style = <Uuid as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting UuidV4");
        loop {
            let uuid = Uuid::elicit(communicator).await?;
            match Self::new(uuid) {
                Ok(valid) => {
                    tracing::debug!(uuid = %valid.0, "Valid V4 UUID");
                    return Ok(valid);
                }
                Err(e) => {
                    tracing::warn!(error = %e, "UUID not V4, re-prompting");
                    continue;
                }
            }
        }
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_uuid_v4()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_type_stub("UuidV4")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_type_stub("UuidV4")
    }
}

// UuidNonNil - Non-nil UUIDs
/// A UUID that is guaranteed to be non-nil.
///
/// Nil UUIDs are all zeros (00000000-0000-0000-0000-000000000000).
///
/// Available with the `uuid` feature.
#[cfg(feature = "uuid")]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[cfg_attr(feature = "uuid", schemars(description = "A non-nil UUID value"))]
pub struct UuidNonNil(Uuid);

#[cfg(feature = "uuid")]
#[cfg_attr(not(kani), instrumented_impl)]
impl UuidNonNil {
    /// Create a new UuidNonNil, validating it's not nil.
    pub fn new(uuid: Uuid) -> Result<Self, ValidationError> {
        if !uuid.is_nil() {
            Ok(Self(uuid))
        } else {
            Err(ValidationError::NilUuid)
        }
    }

    /// Get the inner UUID.
    pub fn get(&self) -> Uuid {
        self.0
    }

    /// Unwrap into the inner UUID.
    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

#[cfg(feature = "uuid")]
#[cfg_attr(not(kani), instrumented_impl)]
impl Prompt for UuidNonNil {
    fn prompt() -> Option<&'static str> {
        Some("Please provide a non-nil UUID (not all zeros):")
    }
}

#[cfg(feature = "uuid")]
#[cfg_attr(not(kani), instrumented_impl)]
impl Elicitation for UuidNonNil {
    type Style = <Uuid as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting UuidNonNil");
        loop {
            let uuid = Uuid::elicit(communicator).await?;
            match Self::new(uuid) {
                Ok(valid) => {
                    tracing::debug!(uuid = %valid.0, "Valid non-nil UUID");
                    return Ok(valid);
                }
                Err(e) => {
                    tracing::warn!(error = %e, "UUID is nil, re-prompting");
                    continue;
                }
            }
        }
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_uuid_non_nil()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_type_stub("UuidNonNil")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_type_stub("UuidNonNil")
    }
}

#[cfg(all(test, feature = "uuid"))]
mod tests {
    use super::*;

    #[test]
    fn test_uuid_v4_new_valid() {
        let uuid = Uuid::new_v4();
        let result = UuidV4::new(uuid);
        assert!(result.is_ok());
    }

    #[test]
    fn test_uuid_v4_new_wrong_version() {
        // Create a V1 UUID
        let uuid = Uuid::parse_str("550e8400-e29b-11d4-a716-446655440000").unwrap();
        assert_eq!(uuid.get_version_num(), 1);
        let result = UuidV4::new(uuid);
        assert!(result.is_err());
    }

    #[test]
    fn test_uuid_v4_get() {
        let uuid = Uuid::new_v4();
        let v4 = UuidV4::new(uuid).unwrap();
        assert_eq!(v4.get(), uuid);
    }

    #[test]
    fn test_uuid_v4_into_inner() {
        let uuid = Uuid::new_v4();
        let v4 = UuidV4::new(uuid).unwrap();
        assert_eq!(v4.into_inner(), uuid);
    }

    #[test]
    fn test_uuid_non_nil_new_valid() {
        let uuid = Uuid::new_v4();
        let result = UuidNonNil::new(uuid);
        assert!(result.is_ok());
    }

    #[test]
    fn test_uuid_non_nil_new_nil() {
        let uuid = Uuid::nil();
        let result = UuidNonNil::new(uuid);
        assert!(result.is_err());
    }

    #[test]
    fn test_uuid_non_nil_get() {
        let uuid = Uuid::new_v4();
        let non_nil = UuidNonNil::new(uuid).unwrap();
        assert_eq!(non_nil.get(), uuid);
    }

    #[test]
    fn test_uuid_non_nil_into_inner() {
        let uuid = Uuid::new_v4();
        let non_nil = UuidNonNil::new(uuid).unwrap();
        assert_eq!(non_nil.into_inner(), uuid);
    }
}

// ── ElicitIntrospect impls ────────────────────────────────────────────────────

#[cfg(feature = "uuid")]
macro_rules! impl_primitive_introspect_uuid {
    ($($ty:ty => $name:literal),+ $(,)?) => {
        $(
            impl crate::ElicitIntrospect for $ty {
                fn pattern() -> crate::ElicitationPattern {
                    crate::ElicitationPattern::Primitive
                }
                fn metadata() -> crate::TypeMetadata {
                    crate::TypeMetadata {
                        type_name: $name,
                        description: <$ty as crate::Prompt>::prompt(),
                        details: crate::PatternDetails::Primitive,
                    }
                }
            }
        )+
    };
}

#[cfg(feature = "uuid")]
impl_primitive_introspect_uuid!(
    UuidV4     => "UuidV4",
    UuidNonNil => "UuidNonNil",
);

// ── ToCodeLiteral impls ───────────────────────────────────────────────────────

#[cfg(feature = "uuid")]
mod emit_impls {
    use super::*;
    use crate::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for UuidV4 {
        fn to_code_literal(&self) -> TokenStream {
            let s = self.get().to_string();
            quote::quote! {
                elicitation::UuidV4::new(
                    ::uuid::Uuid::parse_str(#s).expect("valid uuid")
                ).expect("valid UuidV4")
            }
        }
    }

    impl ToCodeLiteral for UuidNonNil {
        fn to_code_literal(&self) -> TokenStream {
            let s = self.get().to_string();
            quote::quote! {
                elicitation::UuidNonNil::new(
                    ::uuid::Uuid::parse_str(#s).expect("valid uuid")
                ).expect("valid UuidNonNil")
            }
        }
    }
}
