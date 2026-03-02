//! [`ElicitSpec`](crate::ElicitSpec) implementations for collection contract types.

use crate::verification::types::{
    BTreeMapNonEmpty, BTreeSetNonEmpty, HashMapNonEmpty, HashSetNonEmpty, LinkedListNonEmpty,
    OptionSome, ResultOk, VecDequeNonEmpty, VecNonEmpty,
};
use crate::{
    ElicitSpec, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec, TypeSpecBuilder,
    TypeSpecInventoryKey,
};

macro_rules! impl_nonempty_spec {
    (
        type    = $ty:ty,
        name    = $name:literal,
        item    = $item:literal,
        summary = $summary:literal $(,)?
    ) => {
        impl ElicitSpec for $ty {
            fn type_spec() -> TypeSpec {
                let requires = SpecCategoryBuilder::default()
                    .name("requires".to_string())
                    .entries(vec![
                        SpecEntryBuilder::default()
                            .label("non_empty".to_string())
                            .description(
                                concat!($item, " must contain at least one element.").to_string(),
                            )
                            .expression(Some(concat!("!", $item, ".is_empty()").to_string()))
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
            <$ty as ElicitSpec>::type_spec,
            std::any::TypeId::of::<$ty>
        ));
    };
}

// Marker types for generic collections (inventory needs concrete types)
// These are the ElicitSpec implementations for generic types using i32 as the
// concrete monomorphization for inventory registration.

// ── Non-empty collections ─────────────────────────────────────────────────────

impl_nonempty_spec!(
    type    = VecNonEmpty<i32>,
    name    = "VecNonEmpty",
    item    = "vec",
    summary = "A Vec guaranteed to contain at least one element.",
);

impl_nonempty_spec!(
    type    = HashMapNonEmpty<String, i32>,
    name    = "HashMapNonEmpty",
    item    = "map",
    summary = "A HashMap guaranteed to contain at least one key-value pair.",
);

impl_nonempty_spec!(
    type    = BTreeMapNonEmpty<String, i32>,
    name    = "BTreeMapNonEmpty",
    item    = "map",
    summary = "A BTreeMap guaranteed to contain at least one key-value pair.",
);

impl_nonempty_spec!(
    type    = HashSetNonEmpty<i32>,
    name    = "HashSetNonEmpty",
    item    = "set",
    summary = "A HashSet guaranteed to contain at least one element.",
);

impl_nonempty_spec!(
    type    = BTreeSetNonEmpty<i32>,
    name    = "BTreeSetNonEmpty",
    item    = "set",
    summary = "A BTreeSet guaranteed to contain at least one element.",
);

impl_nonempty_spec!(
    type    = VecDequeNonEmpty<i32>,
    name    = "VecDequeNonEmpty",
    item    = "deque",
    summary = "A VecDeque guaranteed to contain at least one element.",
);

impl_nonempty_spec!(
    type    = LinkedListNonEmpty<i32>,
    name    = "LinkedListNonEmpty",
    item    = "list",
    summary = "A LinkedList guaranteed to contain at least one element.",
);

// ── Option and Result contract types ─────────────────────────────────────────

impl ElicitSpec for OptionSome<i32> {
    fn type_spec() -> TypeSpec {
        let requires = SpecCategoryBuilder::default()
            .name("requires".to_string())
            .entries(vec![
                SpecEntryBuilder::default()
                    .label("is_some".to_string())
                    .description("Option must be Some, not None.".to_string())
                    .expression(Some("opt.is_some()".to_string()))
                    .build()
                    .expect("valid SpecEntry"),
            ])
            .build()
            .expect("valid SpecCategory");
        TypeSpecBuilder::default()
            .type_name("OptionSome".to_string())
            .summary("An Option<T> guaranteed to be Some(T), never None.".to_string())
            .categories(vec![requires])
            .build()
            .expect("valid TypeSpec")
    }
}

inventory::submit!(TypeSpecInventoryKey::new(
    "OptionSome",
    OptionSome::<i32>::type_spec,
    std::any::TypeId::of::<OptionSome<i32>>
));

impl ElicitSpec for ResultOk<i32> {
    fn type_spec() -> TypeSpec {
        let requires = SpecCategoryBuilder::default()
            .name("requires".to_string())
            .entries(vec![
                SpecEntryBuilder::default()
                    .label("is_ok".to_string())
                    .description("Result must be Ok, not Err.".to_string())
                    .expression(Some("result.is_ok()".to_string()))
                    .build()
                    .expect("valid SpecEntry"),
            ])
            .build()
            .expect("valid SpecCategory");
        TypeSpecBuilder::default()
            .type_name("ResultOk".to_string())
            .summary("A Result<T, E> guaranteed to be Ok(T), never Err(E).".to_string())
            .categories(vec![requires])
            .build()
            .expect("valid TypeSpec")
    }
}

inventory::submit!(TypeSpecInventoryKey::new(
    "ResultOk",
    ResultOk::<i32>::type_spec,
    std::any::TypeId::of::<ResultOk<i32>>
));
