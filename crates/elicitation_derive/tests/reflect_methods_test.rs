//! Integration tests for the #[reflect_methods] macro.
//!
//! This test verifies that parameter structs are generated correctly
//! from method signatures.

use elicitation::elicit_newtype;
use elicitation_derive::reflect_methods;

// Test with a simple newtype wrapper
elicit_newtype!(String, as StringClient);

// Test method reflection - generates parameter structs
#[reflect_methods]
impl StringClient {
    pub fn append(&self, suffix: &str) -> String {
        format!("{}{}", self.0, suffix)
    }

    pub fn repeat(&self, times: usize) -> String {
        self.0.repeat(times)
    }

    // Method with no parameters - no param struct generated
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[test]
fn test_param_struct_generation() {
    // Verify that AppendParams was generated
    let _params = AppendParams {
        suffix: "world".to_string(),
    };

    // Verify that RepeatParams was generated
    let _params = RepeatParams { times: 3 };

    // No LenParams should be generated (no parameters)
}

#[test]
fn test_original_methods_still_work() {
    // Use the newtype constructor directly
    let client = StringClient("hello".to_string());

    // Original methods should still work
    assert_eq!(client.len(), 5);
    assert_eq!(client.append(" world"), "hello world");
    assert_eq!(client.repeat(2), "hellohello");
}

// Test with multiple parameter types
elicit_newtype!(Vec<u8>, as ByteClient);

#[reflect_methods]
impl ByteClient {
    pub fn extend_with(&self, data: &[u8], count: usize) -> Vec<u8> {
        let mut result = self.0.clone();
        for _ in 0..count {
            result.extend_from_slice(data);
        }
        result
    }
}

#[test]
fn test_slice_conversion() {
    // Verify that &[u8] was converted to Vec<u8> in param struct
    let params = ExtendWithParams {
        data: vec![1, 2, 3],
        count: 2,
    };

    assert_eq!(params.data, vec![1, 2, 3]);
    assert_eq!(params.count, 2);
}

#[test]
fn test_method_with_slice_param() {
    let client = ByteClient(vec![0]);
    let result = client.extend_with(&[1, 2], 2);
    assert_eq!(result, vec![0, 1, 2, 1, 2]);
}

// Test that MCP tool wrapper methods are generated
#[test]
fn test_wrapper_methods_generated() {
    use rmcp::handler::server::wrapper::{Json, Parameters};

    let client = StringClient("hello".to_string());

    // Test append_tool wrapper
    let params = Parameters(AppendParams {
        suffix: " world".to_string(),
    });
    let result = client.append_tool(params);
    assert!(result.is_ok());
    let Json(value) = result.unwrap();
    assert_eq!(value, "hello world");

    // Test repeat_tool wrapper
    let params = Parameters(RepeatParams { times: 3 });
    let result = client.repeat_tool(params);
    assert!(result.is_ok());
    let Json(value) = result.unwrap();
    assert_eq!(value, "hellohellohello");

    // Test len_tool wrapper (no params)
    let result = client.len_tool();
    assert!(result.is_ok());
    let Json(value) = result.unwrap();
    assert_eq!(value, 5);
}

#[test]
fn test_wrapper_with_slice_param() {
    use rmcp::handler::server::wrapper::{Json, Parameters};

    let client = ByteClient(vec![0]);

    let params = Parameters(ExtendWithParams {
        data: vec![1, 2],
        count: 2,
    });

    let result = client.extend_with_tool(params);
    assert!(result.is_ok());
    let Json(value) = result.unwrap();
    assert_eq!(value, vec![0, 1, 2, 1, 2]);
}
