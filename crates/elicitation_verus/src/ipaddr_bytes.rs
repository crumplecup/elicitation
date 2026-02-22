//! Verus proofs for IP address byte validation types.
//!
//! Validates IPv4 and IPv6 addresses with private/public classification.
//! Simplified stubs for compositional verification.

use verus_builtin::*;
use verus_builtin_macros::*;
use vstd::prelude::*;

verus! {

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    InvalidOctets,
    InvalidClassification,
}

// ============================================================================
// Ipv4Bytes - 4-byte IPv4 address
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ipv4Bytes {
    pub octets: [u8; 4],
    pub validated: bool,
}

impl Ipv4Bytes {
    /// All octet combinations are valid IPv4 addresses.
    pub fn new(octets: [u8; 4]) -> (result: Self)
        ensures
            result.octets == octets,
            result.validated == true,
    {
        Ipv4Bytes { octets, validated: true }
    }

    pub fn is_private(&self) -> (result: bool)
        requires self.validated == true,
    {
        // RFC 1918: 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16
        let first = self.octets[0];
        let second = self.octets[1];
        
        first == 10 ||
        (first == 172 && second >= 16 && second <= 31) ||
        (first == 192 && second == 168)
    }

    pub fn is_loopback(&self) -> (result: bool)
        requires self.validated == true,
    {
        self.octets[0] == 127
    }
}

// ============================================================================
// Ipv4Private - Private IPv4 address (RFC 1918)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ipv4Private {
    pub octets: [u8; 4],
    pub validated: bool,
}

impl Ipv4Private {
    /// Parameters:
    /// - is_private: IP is in RFC 1918 private range
    pub fn new(octets: [u8; 4], is_private: bool) -> (result: Result<Self, ValidationError>)
        ensures
            is_private ==> (result matches Ok(ip) && ip.octets == octets && ip.validated == true),
            !is_private ==> (result matches Err(ValidationError::InvalidClassification)),
    {
        if is_private {
            Ok(Ipv4Private { octets, validated: true })
        } else {
            Err(ValidationError::InvalidClassification)
        }
    }
}

// ============================================================================
// Ipv4Public - Public IPv4 address
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ipv4Public {
    pub octets: [u8; 4],
    pub validated: bool,
}

impl Ipv4Public {
    /// Parameters:
    /// - is_public: IP is NOT in RFC 1918 private range
    pub fn new(octets: [u8; 4], is_public: bool) -> (result: Result<Self, ValidationError>)
        ensures
            is_public ==> (result matches Ok(ip) && ip.octets == octets && ip.validated == true),
            !is_public ==> (result matches Err(ValidationError::InvalidClassification)),
    {
        if is_public {
            Ok(Ipv4Public { octets, validated: true })
        } else {
            Err(ValidationError::InvalidClassification)
        }
    }
}

// ============================================================================
// Ipv6Bytes - 16-byte IPv6 address
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ipv6Bytes {
    pub segments: [u16; 8],
    pub validated: bool,
}

impl Ipv6Bytes {
    /// All segment combinations are valid IPv6 addresses.
    pub fn new(segments: [u16; 8]) -> (result: Self)
        ensures
            result.segments == segments,
            result.validated == true,
    {
        Ipv6Bytes { segments, validated: true }
    }

    pub fn is_private(&self) -> (result: bool)
        requires self.validated == true,
    {
        // RFC 4193: fc00::/7 (unique local)
        let first = self.segments[0];
        first >= 0xfc00 && first <= 0xfdff
    }

    pub fn is_loopback(&self) -> (result: bool)
        requires self.validated == true,
    {
        // ::1
        self.segments[0] == 0 &&
        self.segments[1] == 0 &&
        self.segments[2] == 0 &&
        self.segments[3] == 0 &&
        self.segments[4] == 0 &&
        self.segments[5] == 0 &&
        self.segments[6] == 0 &&
        self.segments[7] == 1
    }
}

// ============================================================================
// Ipv6Private - Private IPv6 address (RFC 4193)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ipv6Private {
    pub segments: [u16; 8],
    pub validated: bool,
}

impl Ipv6Private {
    /// Parameters:
    /// - is_private: IP is in RFC 4193 private range (fc00::/7)
    pub fn new(segments: [u16; 8], is_private: bool) -> (result: Result<Self, ValidationError>)
        ensures
            is_private ==> (result matches Ok(ip) && ip.segments == segments && ip.validated == true),
            !is_private ==> (result matches Err(ValidationError::InvalidClassification)),
    {
        if is_private {
            Ok(Ipv6Private { segments, validated: true })
        } else {
            Err(ValidationError::InvalidClassification)
        }
    }
}

// ============================================================================
// Ipv6Public - Public IPv6 address
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ipv6Public {
    pub segments: [u16; 8],
    pub validated: bool,
}

impl Ipv6Public {
    /// Parameters:
    /// - is_public: IP is NOT in RFC 4193 private range
    pub fn new(segments: [u16; 8], is_public: bool) -> (result: Result<Self, ValidationError>)
        ensures
            is_public ==> (result matches Ok(ip) && ip.segments == segments && ip.validated == true),
            !is_public ==> (result matches Err(ValidationError::InvalidClassification)),
    {
        if is_public {
            Ok(Ipv6Public { segments, validated: true })
        } else {
            Err(ValidationError::InvalidClassification)
        }
    }
}

} // verus!
