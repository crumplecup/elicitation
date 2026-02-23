use verus_builtin_macros::verus;

verus! {

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    NotPrivate,
    NotPublic,
    NotIpV4,
    NotIpV6,
    NotLoopback,
    ParseFailed,
}

// IP address types - abstract std::net parsing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IpPrivate {
    pub validated: bool,
}

impl IpPrivate {
    pub fn new(parses: bool, is_private: bool) -> (result: Result<Self, ValidationError>)
        ensures
            (!parses) ==> (result matches Err(ValidationError::ParseFailed)),
            (parses && is_private) ==> (result matches Ok(ip) && ip.validated == true),
            (parses && !is_private) ==> (result matches Err(ValidationError::NotPrivate)),
    {
        if !parses {
            Err(ValidationError::ParseFailed)
        } else if is_private {
            Ok(IpPrivate { validated: true })
        } else {
            Err(ValidationError::NotPrivate)
        }
    }

    pub fn is_validated(&self) -> (result: bool)
        requires self.validated == true,
        ensures result == true,
    {
        self.validated
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IpPublic {
    pub validated: bool,
}

impl IpPublic {
    pub fn new(parses: bool, is_public: bool) -> (result: Result<Self, ValidationError>)
        ensures
            (!parses) ==> (result matches Err(ValidationError::ParseFailed)),
            (parses && is_public) ==> (result matches Ok(ip) && ip.validated == true),
            (parses && !is_public) ==> (result matches Err(ValidationError::NotPublic)),
    {
        if !parses {
            Err(ValidationError::ParseFailed)
        } else if is_public {
            Ok(IpPublic { validated: true })
        } else {
            Err(ValidationError::NotPublic)
        }
    }

    pub fn is_validated(&self) -> (result: bool)
        requires self.validated == true,
        ensures result == true,
    {
        self.validated
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IpV4 {
    pub validated: bool,
}

impl IpV4 {
    pub fn new(parses: bool) -> (result: Result<Self, ValidationError>)
        ensures
            parses ==> (result matches Ok(ip) && ip.validated == true),
            !parses ==> (result matches Err(ValidationError::NotIpV4)),
    {
        if parses {
            Ok(IpV4 { validated: true })
        } else {
            Err(ValidationError::NotIpV4)
        }
    }

    pub fn is_validated(&self) -> (result: bool)
        requires self.validated == true,
        ensures result == true,
    {
        self.validated
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IpV6 {
    pub validated: bool,
}

impl IpV6 {
    pub fn new(parses: bool) -> (result: Result<Self, ValidationError>)
        ensures
            parses ==> (result matches Ok(ip) && ip.validated == true),
            !parses ==> (result matches Err(ValidationError::NotIpV6)),
    {
        if parses {
            Ok(IpV6 { validated: true })
        } else {
            Err(ValidationError::NotIpV6)
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
