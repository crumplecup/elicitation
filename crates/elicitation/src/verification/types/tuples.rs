//! Tuple contract types demonstrating compositional verification.
//!
//! Tuples compose contract types - if all elements are valid contract types,
//! the tuple is guaranteed valid by construction.

use crate::{ElicitCommunicator, ElicitResult, Elicitation, Prompt};

// Tuple2 - 2-element tuple where both satisfy contracts
/// A 2-element tuple where both elements are contract types.
///
/// **Compositional verification:** If C1 and C2 are valid contracts,
/// Tuple2<C1, C2> is automatically valid by construction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tuple2<C1, C2>(pub C1, pub C2);

impl<C1, C2> Tuple2<C1, C2> {
    /// Create a new Tuple2. Both elements are already validated contract types.
    pub fn new(first: C1, second: C2) -> Self {
        Self(first, second)
    }

    /// Get the first element.
    pub fn first(&self) -> &C1 {
        &self.0
    }

    /// Get the second element.
    pub fn second(&self) -> &C2 {
        &self.1
    }

    /// Unwrap into components.
    pub fn into_inner(self) -> (C1, C2) {
        (self.0, self.1)
    }
}

impl<C1, C2> Prompt for Tuple2<C1, C2>
where
    C1: Elicitation + Send,
    C2: Elicitation + Send,
{
    fn prompt() -> Option<&'static str> {
        Some("Eliciting tuple with 2 elements:")
    }
}

impl<C1, C2> Elicitation for Tuple2<C1, C2>
where
    C1: Elicitation + Send,
    C2: Elicitation + Send,
{
    type Style = <(C1, C2) as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting Tuple2");
        let first = C1::elicit(communicator).await?; // Guaranteed valid by contract!
        let second = C2::elicit(communicator).await?; // Guaranteed valid by contract!
        Ok(Self::new(first, second)) // Composition = proven valid
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }
}

// Tuple3 - 3-element tuple
/// A 3-element tuple where all elements are contract types.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tuple3<C1, C2, C3>(pub C1, pub C2, pub C3);

impl<C1, C2, C3> Tuple3<C1, C2, C3> {
    /// Create a new Tuple3.
    pub fn new(first: C1, second: C2, third: C3) -> Self {
        Self(first, second, third)
    }

    /// Unwrap into components.
    pub fn into_inner(self) -> (C1, C2, C3) {
        (self.0, self.1, self.2)
    }
}

impl<C1, C2, C3> Prompt for Tuple3<C1, C2, C3>
where
    C1: Elicitation + Send,
    C2: Elicitation + Send,
    C3: Elicitation + Send,
{
    fn prompt() -> Option<&'static str> {
        Some("Eliciting tuple with 3 elements:")
    }
}

impl<C1, C2, C3> Elicitation for Tuple3<C1, C2, C3>
where
    C1: Elicitation + Send,
    C2: Elicitation + Send,
    C3: Elicitation + Send,
{
    type Style = <(C1, C2, C3) as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting Tuple3");
        let first = C1::elicit(communicator).await?;
        let second = C2::elicit(communicator).await?;
        let third = C3::elicit(communicator).await?;
        Ok(Self::new(first, second, third))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }
}

// Tuple4 - 4-element tuple
/// A 4-element tuple where all elements are contract types.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tuple4<C1, C2, C3, C4>(pub C1, pub C2, pub C3, pub C4);

impl<C1, C2, C3, C4> Tuple4<C1, C2, C3, C4> {
    /// Create a new Tuple4.
    pub fn new(first: C1, second: C2, third: C3, fourth: C4) -> Self {
        Self(first, second, third, fourth)
    }

    /// Unwrap into components.
    pub fn into_inner(self) -> (C1, C2, C3, C4) {
        (self.0, self.1, self.2, self.3)
    }
}

impl<C1, C2, C3, C4> Prompt for Tuple4<C1, C2, C3, C4>
where
    C1: Elicitation + Send,
    C2: Elicitation + Send,
    C3: Elicitation + Send,
    C4: Elicitation + Send,
{
    fn prompt() -> Option<&'static str> {
        Some("Eliciting tuple with 4 elements:")
    }
}

impl<C1, C2, C3, C4> Elicitation for Tuple4<C1, C2, C3, C4>
where
    C1: Elicitation + Send,
    C2: Elicitation + Send,
    C3: Elicitation + Send,
    C4: Elicitation + Send,
{
    type Style = <(C1, C2, C3, C4) as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting Tuple4");
        let first = C1::elicit(communicator).await?;
        let second = C2::elicit(communicator).await?;
        let third = C3::elicit(communicator).await?;
        let fourth = C4::elicit(communicator).await?;
        Ok(Self::new(first, second, third, fourth))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::verification::types::{BoolTrue, I8Positive, StringNonEmpty};

    #[test]
    fn test_tuple2_new() {
        let s: StringNonEmpty = StringNonEmpty::new("test".to_string()).unwrap();
        let t = Tuple2::new(I8Positive::new(5).unwrap(), s);
        assert_eq!(t.first().get(), 5);
        assert_eq!(t.second().get(), "test");
    }

    #[test]
    fn test_tuple2_into_inner() {
        let s: StringNonEmpty = StringNonEmpty::new("test".to_string()).unwrap();
        let t = Tuple2::new(I8Positive::new(5).unwrap(), s);
        let (first, second) = t.into_inner();
        assert_eq!(first.get(), 5);
        assert_eq!(second.get(), "test");
    }

    #[test]
    fn test_tuple3_new() {
        let s: StringNonEmpty = StringNonEmpty::new("test".to_string()).unwrap();
        let t = Tuple3::new(I8Positive::new(5).unwrap(), s, BoolTrue::new(true).unwrap());
        let (first, second, third) = t.into_inner();
        assert_eq!(first.get(), 5);
        assert_eq!(second.get(), "test");
        assert!(third.get());
    }

    #[test]
    fn test_tuple4_new() {
        let t = Tuple4::new(
            I8Positive::new(1).unwrap(),
            I8Positive::new(2).unwrap(),
            I8Positive::new(3).unwrap(),
            I8Positive::new(4).unwrap(),
        );
        let (a, b, c, d) = t.into_inner();
        assert_eq!(a.get(), 1);
        assert_eq!(b.get(), 2);
        assert_eq!(c.get(), 3);
        assert_eq!(d.get(), 4);
    }
}

// ── Serde + JsonSchema impls ──────────────────────────────────────────────────

impl<C1: serde::Serialize, C2: serde::Serialize> serde::Serialize for Tuple2<C1, C2> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (&self.0, &self.1).serialize(s)
    }
}

impl<'de, C1: serde::Deserialize<'de>, C2: serde::Deserialize<'de>> serde::Deserialize<'de>
    for Tuple2<C1, C2>
{
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let (c1, c2) = <(C1, C2)>::deserialize(d)?;
        Ok(Self(c1, c2))
    }
}

impl<C1: schemars::JsonSchema, C2: schemars::JsonSchema> schemars::JsonSchema for Tuple2<C1, C2> {
    fn schema_name() -> ::std::borrow::Cow<'static, str> {
        format!("Tuple2<{},{}>", C1::schema_name(), C2::schema_name()).into()
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let c1_val = serde_json::to_value(generator.subschema_for::<C1>()).expect("schema");
        let c2_val = serde_json::to_value(generator.subschema_for::<C2>()).expect("schema");
        let mut map = serde_json::Map::new();
        map.insert("type".to_string(), "array".into());
        map.insert("minItems".to_string(), 2u64.into());
        map.insert("maxItems".to_string(), 2u64.into());
        map.insert(
            "prefixItems".to_string(),
            serde_json::Value::Array(vec![c1_val, c2_val]),
        );
        schemars::Schema::from(map)
    }
}

impl<C1: serde::Serialize, C2: serde::Serialize, C3: serde::Serialize> serde::Serialize
    for Tuple3<C1, C2, C3>
{
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (&self.0, &self.1, &self.2).serialize(s)
    }
}

impl<'de, C1: serde::Deserialize<'de>, C2: serde::Deserialize<'de>, C3: serde::Deserialize<'de>>
    serde::Deserialize<'de> for Tuple3<C1, C2, C3>
{
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let (c1, c2, c3) = <(C1, C2, C3)>::deserialize(d)?;
        Ok(Self(c1, c2, c3))
    }
}

impl<C1: schemars::JsonSchema, C2: schemars::JsonSchema, C3: schemars::JsonSchema>
    schemars::JsonSchema for Tuple3<C1, C2, C3>
{
    fn schema_name() -> ::std::borrow::Cow<'static, str> {
        format!(
            "Tuple3<{},{},{}>",
            C1::schema_name(),
            C2::schema_name(),
            C3::schema_name()
        )
        .into()
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let c1_val = serde_json::to_value(generator.subschema_for::<C1>()).expect("schema");
        let c2_val = serde_json::to_value(generator.subschema_for::<C2>()).expect("schema");
        let c3_val = serde_json::to_value(generator.subschema_for::<C3>()).expect("schema");
        let mut map = serde_json::Map::new();
        map.insert("type".to_string(), "array".into());
        map.insert("minItems".to_string(), 3u64.into());
        map.insert("maxItems".to_string(), 3u64.into());
        map.insert(
            "prefixItems".to_string(),
            serde_json::Value::Array(vec![c1_val, c2_val, c3_val]),
        );
        schemars::Schema::from(map)
    }
}

impl<C1, C2, C3, C4> serde::Serialize for Tuple4<C1, C2, C3, C4>
where
    C1: serde::Serialize,
    C2: serde::Serialize,
    C3: serde::Serialize,
    C4: serde::Serialize,
{
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (&self.0, &self.1, &self.2, &self.3).serialize(s)
    }
}

impl<'de, C1, C2, C3, C4> serde::Deserialize<'de> for Tuple4<C1, C2, C3, C4>
where
    C1: serde::Deserialize<'de>,
    C2: serde::Deserialize<'de>,
    C3: serde::Deserialize<'de>,
    C4: serde::Deserialize<'de>,
{
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let (c1, c2, c3, c4) = <(C1, C2, C3, C4)>::deserialize(d)?;
        Ok(Self(c1, c2, c3, c4))
    }
}

impl<C1, C2, C3, C4> schemars::JsonSchema for Tuple4<C1, C2, C3, C4>
where
    C1: schemars::JsonSchema,
    C2: schemars::JsonSchema,
    C3: schemars::JsonSchema,
    C4: schemars::JsonSchema,
{
    fn schema_name() -> ::std::borrow::Cow<'static, str> {
        format!(
            "Tuple4<{},{},{},{}>",
            C1::schema_name(),
            C2::schema_name(),
            C3::schema_name(),
            C4::schema_name()
        )
        .into()
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let c1_val = serde_json::to_value(generator.subschema_for::<C1>()).expect("schema");
        let c2_val = serde_json::to_value(generator.subschema_for::<C2>()).expect("schema");
        let c3_val = serde_json::to_value(generator.subschema_for::<C3>()).expect("schema");
        let c4_val = serde_json::to_value(generator.subschema_for::<C4>()).expect("schema");
        let mut map = serde_json::Map::new();
        map.insert("type".to_string(), "array".into());
        map.insert("minItems".to_string(), 4u64.into());
        map.insert("maxItems".to_string(), 4u64.into());
        map.insert(
            "prefixItems".to_string(),
            serde_json::Value::Array(vec![c1_val, c2_val, c3_val, c4_val]),
        );
        schemars::Schema::from(map)
    }
}

// ── ElicitIntrospect impls ────────────────────────────────────────────────────

impl<C1: crate::Elicitation + Send, C2: crate::Elicitation + Send> crate::ElicitIntrospect
    for Tuple2<C1, C2>
{
    fn pattern() -> crate::ElicitationPattern {
        crate::ElicitationPattern::Primitive
    }
    fn metadata() -> crate::TypeMetadata {
        crate::TypeMetadata {
            type_name: "Tuple2",
            description: <Self as crate::Prompt>::prompt(),
            details: crate::PatternDetails::Primitive,
        }
    }
}

impl<C1: crate::Elicitation + Send, C2: crate::Elicitation + Send, C3: crate::Elicitation + Send>
    crate::ElicitIntrospect for Tuple3<C1, C2, C3>
{
    fn pattern() -> crate::ElicitationPattern {
        crate::ElicitationPattern::Primitive
    }
    fn metadata() -> crate::TypeMetadata {
        crate::TypeMetadata {
            type_name: "Tuple3",
            description: <Self as crate::Prompt>::prompt(),
            details: crate::PatternDetails::Primitive,
        }
    }
}

impl<C1, C2, C3, C4> crate::ElicitIntrospect for Tuple4<C1, C2, C3, C4>
where
    C1: crate::Elicitation + Send,
    C2: crate::Elicitation + Send,
    C3: crate::Elicitation + Send,
    C4: crate::Elicitation + Send,
{
    fn pattern() -> crate::ElicitationPattern {
        crate::ElicitationPattern::Primitive
    }
    fn metadata() -> crate::TypeMetadata {
        crate::TypeMetadata {
            type_name: "Tuple4",
            description: <Self as crate::Prompt>::prompt(),
            details: crate::PatternDetails::Primitive,
        }
    }
}

// ── ToCodeLiteral impls ───────────────────────────────────────────────────────

mod emit_impls {
    use super::*;
    use crate::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl<C1: ToCodeLiteral, C2: ToCodeLiteral> ToCodeLiteral for Tuple2<C1, C2> {
        fn to_code_literal(&self) -> TokenStream {
            let first = self.0.to_code_literal();
            let second = self.1.to_code_literal();
            quote::quote! { elicitation::Tuple2::new(#first, #second) }
        }
    }

    impl<C1: ToCodeLiteral, C2: ToCodeLiteral, C3: ToCodeLiteral> ToCodeLiteral for Tuple3<C1, C2, C3> {
        fn to_code_literal(&self) -> TokenStream {
            let first = self.0.to_code_literal();
            let second = self.1.to_code_literal();
            let third = self.2.to_code_literal();
            quote::quote! { elicitation::Tuple3::new(#first, #second, #third) }
        }
    }

    impl<C1: ToCodeLiteral, C2: ToCodeLiteral, C3: ToCodeLiteral, C4: ToCodeLiteral> ToCodeLiteral
        for Tuple4<C1, C2, C3, C4>
    {
        fn to_code_literal(&self) -> TokenStream {
            let first = self.0.to_code_literal();
            let second = self.1.to_code_literal();
            let third = self.2.to_code_literal();
            let fourth = self.3.to_code_literal();
            quote::quote! { elicitation::Tuple4::new(#first, #second, #third, #fourth) }
        }
    }
}
