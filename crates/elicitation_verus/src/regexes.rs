use verus_builtin_macros::verus;

verus! {

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    RegexCompileFailed,
    NotCaseInsensitive,
}

// ============================================================================
// RegexValid - valid regex pattern
// ============================================================================

/// Contract type for valid regex patterns.
///
/// Abstracts Regex::new() compilation - assume regex crate works correctly,
/// verify our wrapper logic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegexValid {
    pub validated: bool,
}

impl RegexValid {
    /// Creates a RegexValid given compilation result.
    ///
    /// Parameters abstract the regex compilation:
    /// - compiles: result of Regex::new(pattern).is_ok()
    pub fn new(compiles: bool) -> (result: Result<Self, ValidationError>)
        ensures
            compiles ==> (result matches Ok(r) && r.validated == true),
            !compiles ==> (result matches Err(ValidationError::RegexCompileFailed)),
    {
        if compiles {
            Ok(RegexValid { validated: true })
        } else {
            Err(ValidationError::RegexCompileFailed)
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
// RegexCaseInsensitive - case-insensitive regex
// ============================================================================

/// Contract type for case-insensitive regex patterns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegexCaseInsensitive {
    pub validated: bool,
}

impl RegexCaseInsensitive {
    /// Creates a RegexCaseInsensitive given compilation result.
    ///
    /// Parameters:
    /// - compiles: result of RegexBuilder::new(pattern).case_insensitive(true).build().is_ok()
    pub fn new(compiles: bool) -> (result: Result<Self, ValidationError>)
        ensures
            compiles ==> (result matches Ok(r) && r.validated == true),
            !compiles ==> (result matches Err(ValidationError::RegexCompileFailed)),
    {
        if compiles {
            Ok(RegexCaseInsensitive { validated: true })
        } else {
            Err(ValidationError::RegexCompileFailed)
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
