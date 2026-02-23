use verus_builtin_macros::verus;

verus! {

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    NotObject,
    NotArray,
    IsNull,
}

// ============================================================================
// ValueObject - JSON object
// ============================================================================

/// Contract type for JSON values that are objects.
///
/// Abstracts serde_json::Value checking - assume serde_json works correctly,
/// verify our wrapper logic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValueObject {
    pub validated: bool,
}

impl ValueObject {
    /// Creates a ValueObject given type check result.
    ///
    /// Parameters abstract the JSON type check:
    /// - is_object: result of value.is_object()
    pub fn new(is_object: bool) -> (result: Result<Self, ValidationError>)
        ensures
            is_object ==> (result matches Ok(v) && v.validated == true),
            !is_object ==> (result matches Err(ValidationError::NotObject)),
    {
        if is_object {
            Ok(ValueObject { validated: true })
        } else {
            Err(ValidationError::NotObject)
        }
    }

    pub fn is_validated(&self) -> (result: bool)
        requires self.validated == true,
        ensures result == true,
    {
        self.validated
    }
}

// ============================================================================
// ValueArray - JSON array
// ============================================================================

/// Contract type for JSON values that are arrays.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValueArray {
    pub validated: bool,
}

impl ValueArray {
    /// Creates a ValueArray given type check result.
    ///
    /// Parameters:
    /// - is_array: result of value.is_array()
    pub fn new(is_array: bool) -> (result: Result<Self, ValidationError>)
        ensures
            is_array ==> (result matches Ok(v) && v.validated == true),
            !is_array ==> (result matches Err(ValidationError::NotArray)),
    {
        if is_array {
            Ok(ValueArray { validated: true })
        } else {
            Err(ValidationError::NotArray)
        }
    }

    pub fn is_validated(&self) -> (result: bool)
        requires self.validated == true,
        ensures result == true,
    {
        self.validated
    }
}

// ============================================================================
// ValueNonNull - Non-null JSON value
// ============================================================================

/// Contract type for JSON values that are not null.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValueNonNull {
    pub validated: bool,
}

impl ValueNonNull {
    /// Creates a ValueNonNull given null check result.
    ///
    /// Parameters:
    /// - is_null: result of value.is_null()
    pub fn new(is_null: bool) -> (result: Result<Self, ValidationError>)
        ensures
            !is_null ==> (result matches Ok(v) && v.validated == true),
            is_null ==> (result matches Err(ValidationError::IsNull)),
    {
        if !is_null {
            Ok(ValueNonNull { validated: true })
        } else {
            Err(ValidationError::IsNull)
        }
    }

    pub fn is_validated(&self) -> (result: bool)
        requires self.validated == true,
        ensures result == true,
    {
        self.validated
    }
}

} // verus!
