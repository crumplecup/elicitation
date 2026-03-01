//! Integration tests for generic method support in #[reflect_methods] proc macro.

use elicitation::elicit_newtype;
use elicitation_derive::reflect_methods;

// Create newtype wrapper
elicit_newtype!(Vec<String>, as StringList);

// Simplified test with just one generic method
#[reflect_methods]
impl StringList {
    pub fn contains<T>(&self, item: &T) -> bool
    where
        T: elicitation::Elicitation + schemars::JsonSchema + PartialEq<String>,
    {
        self.0.iter().any(|s| item == s)
    }
}

#[test]
fn test_generic_method_compiles() {
    // Just verify that the code compiles
    // The #[reflect_methods] macro should have generated:
    // 1. ContainsParams<T> struct
    // 2. contains_tool<T>(...) method
}

#[test]
fn test_generic_method_delegation() {
    let list = StringList::from(vec!["hello".to_string(), "world".to_string()]);

    // Test contains - delegates to inner Vec
    assert!(list.contains(&"hello".to_string()));
    assert!(!list.contains(&"goodbye".to_string()));
}

// The generated code should look like:
//
// #[derive(Debug, Clone, Elicit, JsonSchema)]
// pub struct ContainsParams<T>
// where
//     T: Elicitation + JsonSchema + PartialEq<String>,
// {
//     pub item: T,
// }
//
// impl StringList {
//     #[tool(description = "contains operation")]
//     pub fn contains_tool<T>(
//         &self,
//         params: Parameters<ContainsParams<T>>,
//     ) -> Result<Json<bool>, ErrorData>
//     where
//         T: Elicitation + JsonSchema + PartialEq<String>,
//     {
//         let item = &params.0.item;
//         self.contains::<T>(item)
//             .map(Json)
//             .map_err(|e| ErrorData::internal_error(e.to_string(), None))
//     }
// }
