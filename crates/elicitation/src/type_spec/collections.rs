//! [`ElicitSpec`](crate::ElicitSpec) implementations for `Vec`, standard
//! collections, `Option`, and `Result`.

use crate::{
    ElicitSpec, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec, TypeSpecBuilder,
    TypeSpecInventoryKey,
};

// ── Vec<T> ───────────────────────────────────────────────────────────────────

/// Marker type for `Vec<T>` spec registration (generic types need a concrete stand-in).
pub struct VecSpec;

impl ElicitSpec for VecSpec {
    fn type_spec() -> TypeSpec {
        let ensures = SpecCategoryBuilder::default()
            .name("ensures".to_string())
            .entries(vec![
                SpecEntryBuilder::default()
                    .label("contiguous".to_string())
                    .description("Elements are stored contiguously in heap memory.".to_string())
                    .build()
                    .expect("valid entry"),
            ])
            .build()
            .expect("valid ensures");

        let fields = SpecCategoryBuilder::default()
            .name("fields".to_string())
            .entries(vec![
                SpecEntryBuilder::default()
                    .label("len".to_string())
                    .description("Number of elements currently in the vector.".to_string())
                    .build()
                    .expect("valid entry"),
                SpecEntryBuilder::default()
                    .label("capacity".to_string())
                    .description(
                        "Number of elements the vector can hold without reallocating.".to_string(),
                    )
                    .build()
                    .expect("valid entry"),
            ])
            .build()
            .expect("valid fields");

        TypeSpecBuilder::default()
            .type_name("Vec<T>".to_string())
            .summary("A heap-allocated, growable array of elements of type T.".to_string())
            .categories(vec![ensures, fields])
            .build()
            .expect("valid TypeSpec")
    }
}

inventory::submit!(TypeSpecInventoryKey::new(
    "Vec<T>",
    VecSpec::type_spec,
    std::any::TypeId::of::<VecSpec>
));

// ── HashMap<K, V> ────────────────────────────────────────────────────────────

/// Spec registration marker for `HashMap<K, V>`.
pub struct HashMapSpec;

impl ElicitSpec for HashMapSpec {
    fn type_spec() -> TypeSpec {
        let requires = SpecCategoryBuilder::default()
            .name("requires".to_string())
            .entries(vec![
                SpecEntryBuilder::default()
                    .label("key_hash_eq".to_string())
                    .description("Key type K must implement Hash + Eq.".to_string())
                    .expression(Some("K: Hash + Eq".to_string()))
                    .build()
                    .expect("valid entry"),
            ])
            .build()
            .expect("valid requires");

        TypeSpecBuilder::default()
            .type_name("HashMap<K, V>".to_string())
            .summary("A hash table mapping keys of type K to values of type V. Keys must implement Hash + Eq.".to_string())
            .categories(vec![requires])
            .build().expect("valid TypeSpec")
    }
}

inventory::submit!(TypeSpecInventoryKey::new(
    "HashMap<K, V>",
    HashMapSpec::type_spec,
    std::any::TypeId::of::<HashMapSpec>
));

// ── HashSet<T> ───────────────────────────────────────────────────────────────

/// Spec registration marker for `HashSet<T>`.
pub struct HashSetSpec;

impl ElicitSpec for HashSetSpec {
    fn type_spec() -> TypeSpec {
        let requires = SpecCategoryBuilder::default()
            .name("requires".to_string())
            .entries(vec![
                SpecEntryBuilder::default()
                    .label("elem_hash_eq".to_string())
                    .description("Element type T must implement Hash + Eq.".to_string())
                    .expression(Some("T: Hash + Eq".to_string()))
                    .build()
                    .expect("valid entry"),
                SpecEntryBuilder::default()
                    .label("unique".to_string())
                    .description(
                        "All elements are unique — duplicates are silently ignored on insert."
                            .to_string(),
                    )
                    .build()
                    .expect("valid entry"),
            ])
            .build()
            .expect("valid requires");

        TypeSpecBuilder::default()
            .type_name("HashSet<T>".to_string())
            .summary(
                "A hash set of unique elements of type T. T must implement Hash + Eq.".to_string(),
            )
            .categories(vec![requires])
            .build()
            .expect("valid TypeSpec")
    }
}

inventory::submit!(TypeSpecInventoryKey::new(
    "HashSet<T>",
    HashSetSpec::type_spec,
    std::any::TypeId::of::<HashSetSpec>
));

// ── Option<T> ────────────────────────────────────────────────────────────────

/// Spec registration marker for `Option<T>`.
pub struct OptionSpec;

impl ElicitSpec for OptionSpec {
    fn type_spec() -> TypeSpec {
        let values = SpecCategoryBuilder::default()
            .name("values".to_string())
            .entries(vec![
                SpecEntryBuilder::default()
                    .label("some".to_string())
                    .description("Some(value) — contains a value of type T.".to_string())
                    .expression(Some("Option::Some(T)".to_string()))
                    .build()
                    .expect("valid entry"),
                SpecEntryBuilder::default()
                    .label("none".to_string())
                    .description(
                        "None — absence of a value. Agents should supply `null` or omit the field."
                            .to_string(),
                    )
                    .expression(Some("Option::None".to_string()))
                    .build()
                    .expect("valid entry"),
            ])
            .build()
            .expect("valid values");

        TypeSpecBuilder::default()
            .type_name("Option<T>".to_string())
            .summary("An optional value: either Some(T) containing a value, or None representing absence.".to_string())
            .categories(vec![values])
            .build().expect("valid TypeSpec")
    }
}

inventory::submit!(TypeSpecInventoryKey::new(
    "Option<T>",
    OptionSpec::type_spec,
    std::any::TypeId::of::<OptionSpec>
));

// ── Result<T, E> ─────────────────────────────────────────────────────────────

/// Spec registration marker for `Result<T, E>`.
pub struct ResultSpec;

impl ElicitSpec for ResultSpec {
    fn type_spec() -> TypeSpec {
        let values = SpecCategoryBuilder::default()
            .name("values".to_string())
            .entries(vec![
                SpecEntryBuilder::default()
                    .label("ok".to_string())
                    .description(
                        "Ok(value) — operation succeeded, contains result of type T.".to_string(),
                    )
                    .expression(Some("Result::Ok(T)".to_string()))
                    .build()
                    .expect("valid entry"),
                SpecEntryBuilder::default()
                    .label("err".to_string())
                    .description(
                        "Err(error) — operation failed, contains error of type E.".to_string(),
                    )
                    .expression(Some("Result::Err(E)".to_string()))
                    .build()
                    .expect("valid entry"),
            ])
            .build()
            .expect("valid values");

        TypeSpecBuilder::default()
            .type_name("Result<T, E>".to_string())
            .summary("A result type representing success (Ok(T)) or failure (Err(E)).".to_string())
            .categories(vec![values])
            .build()
            .expect("valid TypeSpec")
    }
}

inventory::submit!(TypeSpecInventoryKey::new(
    "Result<T, E>",
    ResultSpec::type_spec,
    std::any::TypeId::of::<ResultSpec>
));
