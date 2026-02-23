use verus_builtin_macros::verus;

verus! {

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    ParseFailed,
    NotAfter,
    NotBefore,
}

// ============================================================================
// DateTimeUtcAfter - DateTime after a threshold
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateTimeUtcAfter {
    pub validated: bool,
}

impl DateTimeUtcAfter {
    /// Parameters:
    /// - parses: DateTime::parse_from_rfc3339(string).is_ok()
    /// - is_after: datetime > threshold
    pub fn new(parses: bool, is_after: bool) -> (result: Result<Self, ValidationError>)
        ensures
            (!parses) ==> (result matches Err(ValidationError::ParseFailed)),
            (parses && is_after) ==> (result matches Ok(dt) && dt.validated == true),
            (parses && !is_after) ==> (result matches Err(ValidationError::NotAfter)),
    {
        if !parses {
            Err(ValidationError::ParseFailed)
        } else if is_after {
            Ok(DateTimeUtcAfter { validated: true })
        } else {
            Err(ValidationError::NotAfter)
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
// DateTimeUtcBefore - DateTime before a threshold
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateTimeUtcBefore {
    pub validated: bool,
}

impl DateTimeUtcBefore {
    /// Parameters:
    /// - parses: DateTime::parse_from_rfc3339(string).is_ok()
    /// - is_before: datetime < threshold
    pub fn new(parses: bool, is_before: bool) -> (result: Result<Self, ValidationError>)
        ensures
            (!parses) ==> (result matches Err(ValidationError::ParseFailed)),
            (parses && is_before) ==> (result matches Ok(dt) && dt.validated == true),
            (parses && !is_before) ==> (result matches Err(ValidationError::NotBefore)),
    {
        if !parses {
            Err(ValidationError::ParseFailed)
        } else if is_before {
            Ok(DateTimeUtcBefore { validated: true })
        } else {
            Err(ValidationError::NotBefore)
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
