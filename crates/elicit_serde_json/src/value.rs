//! `JsonValue` — elicitation-enabled wrapper around `serde_json::Value`.

use elicitation::elicit_newtype;
use elicitation_derive::reflect_methods;

use crate::JsonNumber;

elicit_newtype!(serde_json::Value, as JsonValue, serde);

#[reflect_methods]
impl JsonValue {
    /// Returns true if this value is `null`.
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    /// Returns true if this value is a boolean.
    pub fn is_boolean(&self) -> bool {
        self.0.is_boolean()
    }

    /// Returns true if this value is a number.
    pub fn is_number(&self) -> bool {
        self.0.is_number()
    }

    /// Returns true if this value is a string.
    pub fn is_string(&self) -> bool {
        self.0.is_string()
    }

    /// Returns true if this value is an array.
    pub fn is_array(&self) -> bool {
        self.0.is_array()
    }

    /// Returns true if this value is an object.
    pub fn is_object(&self) -> bool {
        self.0.is_object()
    }

    /// Returns the boolean if this value is a `bool`, or `None`.
    pub fn as_bool(&self) -> Option<bool> {
        self.0.as_bool()
    }

    /// Returns the string slice if this value is a `String`, or `None`.
    pub fn as_str(&self) -> Option<String> {
        self.0.as_str().map(ToOwned::to_owned)
    }

    /// Returns the integer if this value is a number representable as `i64`, or `None`.
    pub fn as_i64(&self) -> Option<i64> {
        self.0.as_i64()
    }

    /// Returns the integer if this value is a number representable as `u64`, or `None`.
    pub fn as_u64(&self) -> Option<u64> {
        self.0.as_u64()
    }

    /// Returns the float if this value is a number, or `None`.
    pub fn as_f64(&self) -> Option<f64> {
        self.0.as_f64()
    }

    /// Returns the number of elements if this is an array or object, or `None`.
    pub fn len(&self) -> Option<usize> {
        match &*self.0 {
            serde_json::Value::Array(a) => Some(a.len()),
            serde_json::Value::Object(o) => Some(o.len()),
            _ => None,
        }
    }

    /// Returns true if this is an empty array or empty object.
    pub fn is_empty(&self) -> bool {
        match &*self.0 {
            serde_json::Value::Array(a) => a.is_empty(),
            serde_json::Value::Object(o) => o.is_empty(),
            _ => false,
        }
    }

    /// Returns the JSON type name: `"null"`, `"bool"`, `"number"`, `"string"`, `"array"`, or `"object"`.
    pub fn type_name(&self) -> &'static str {
        match &*self.0 {
            serde_json::Value::Null => "null",
            serde_json::Value::Bool(_) => "bool",
            serde_json::Value::Number(_) => "number",
            serde_json::Value::String(_) => "string",
            serde_json::Value::Array(_) => "array",
            serde_json::Value::Object(_) => "object",
        }
    }

    /// Looks up a value by an RFC 6901 JSON Pointer (e.g. `"/foo/0/bar"`).
    /// Returns `None` if the pointer does not match.
    pub fn pointer(&self, ptr: String) -> Option<JsonValue> {
        self.0
            .pointer(&ptr)
            .cloned()
            .map(|v| std::sync::Arc::new(v).into())
    }

    /// Returns the number if this value is a `Number`, or `None`.
    pub fn as_number(&self) -> Option<JsonNumber> {
        self.0
            .as_number()
            .cloned()
            .map(|n| std::sync::Arc::new(n).into())
    }

    /// Serialize this value to a compact JSON string.
    pub fn to_json_string(&self) -> String {
        self.0.to_string()
    }

    /// Serialize this value to a pretty-printed JSON string.
    pub fn to_json_string_pretty(&self) -> String {
        serde_json::to_string_pretty(&*self.0).unwrap_or_else(|e| e.to_string())
    }
}
