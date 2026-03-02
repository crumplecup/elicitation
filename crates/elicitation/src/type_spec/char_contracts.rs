//! [`ElicitSpec`](crate::ElicitSpec) implementations for char contract types.

use crate::verification::types::{CharAlphabetic, CharAlphanumeric, CharNumeric};
use crate::{
    ElicitSpec, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec, TypeSpecBuilder,
    TypeSpecInventoryKey,
};

macro_rules! impl_char_contract_spec {
    (
        type    = $ty:ty,
        name    = $name:literal,
        summary = $summary:literal,
        requires = [($req_label:literal, $req_desc:literal, $req_expr:literal)] $(,)?
    ) => {
        impl ElicitSpec for $ty {
            fn type_spec() -> TypeSpec {
                let requires = SpecCategoryBuilder::default()
                    .name("requires".to_string())
                    .entries(vec![
                        SpecEntryBuilder::default()
                            .label($req_label.to_string())
                            .description($req_desc.to_string())
                            .expression(Some($req_expr.to_string()))
                            .build()
                            .expect("valid SpecEntry"),
                    ])
                    .build()
                    .expect("valid SpecCategory");
                TypeSpecBuilder::default()
                    .type_name($name.to_string())
                    .summary($summary.to_string())
                    .categories(vec![requires])
                    .build()
                    .expect("valid TypeSpec")
            }
        }

        inventory::submit!(TypeSpecInventoryKey::new(
            $name,
            <$ty as ElicitSpec>::type_spec
        ));
    };
}

impl_char_contract_spec!(
    type    = CharAlphabetic,
    name    = "CharAlphabetic",
    summary = "A Unicode alphabetic character (char::is_alphabetic()).",
    requires = [("alphabetic", "Character must satisfy char::is_alphabetic().", "value.is_alphabetic()")],
);

impl_char_contract_spec!(
    type    = CharNumeric,
    name    = "CharNumeric",
    summary = "A Unicode numeric character (char::is_numeric()).",
    requires = [("numeric", "Character must satisfy char::is_numeric().", "value.is_numeric()")],
);

impl_char_contract_spec!(
    type    = CharAlphanumeric,
    name    = "CharAlphanumeric",
    summary = "A Unicode alphanumeric character (char::is_alphanumeric()).",
    requires = [("alphanumeric", "Character must satisfy char::is_alphanumeric().", "value.is_alphanumeric()")],
);
