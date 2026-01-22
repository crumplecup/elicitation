//! IP address byte-level validation foundation.
//!
//! This module provides validated IP address byte sequences (IPv4 and IPv6).
//! It forms the foundation for IP address contract types.

use super::ValidationError;

// ============================================================================
// IPv4 Byte Structure
// ============================================================================
//
// 4 bytes (32 bits): A.B.C.D
//
// Private Ranges (RFC 1918):
//   - 10.0.0.0/8        (10.0.0.0 - 10.255.255.255)
//   - 172.16.0.0/12     (172.16.0.0 - 172.31.255.255)
//   - 192.168.0.0/16    (192.168.0.0 - 192.168.255.255)
//
// Special Ranges:
//   - 127.0.0.0/8       (Loopback)
//   - 224.0.0.0/4       (Multicast: 224-239.x.x.x)
//   - 0.0.0.0           (Unspecified)
//   - 255.255.255.255   (Broadcast)

// ============================================================================
// IPv6 Byte Structure (RFC 4291)
// ============================================================================
//
// 16 bytes (128 bits): 8 segments of 2 bytes each
//
// Private Range (RFC 4193):
//   - fc00::/7          (Unique local: fc00-fdff)
//
// Special Addresses:
//   - ::1               (Loopback)
//   - ::                (Unspecified)
//   - ff00::/8          (Multicast: ff00-ffff)

// ============================================================================
// IPv4 Validation
// ============================================================================

/// A validated IPv4 address (4 bytes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ipv4Bytes {
    octets: [u8; 4],
}

impl Ipv4Bytes {
    /// Create a new Ipv4Bytes from octets.
    ///
    /// Always succeeds since all octet combinations are valid IPv4.
    pub fn new(octets: [u8; 4]) -> Self {
        Self { octets }
    }

    /// Get the octets.
    pub fn octets(&self) -> [u8; 4] {
        self.octets
    }

    /// Check if this is a private address (RFC 1918).
    pub fn is_private(&self) -> bool {
        is_ipv4_private(&self.octets)
    }

    /// Check if this is a public address (not private, not special).
    pub fn is_public(&self) -> bool {
        !self.is_private() && !self.is_loopback() && !self.is_unspecified() && !self.is_broadcast()
    }

    /// Check if this is a loopback address (127.0.0.0/8).
    pub fn is_loopback(&self) -> bool {
        self.octets[0] == 127
    }

    /// Check if this is multicast (224.0.0.0/4).
    pub fn is_multicast(&self) -> bool {
        self.octets[0] >= 224 && self.octets[0] <= 239
    }

    /// Check if this is the unspecified address (0.0.0.0).
    pub fn is_unspecified(&self) -> bool {
        self.octets == [0, 0, 0, 0]
    }

    /// Check if this is the broadcast address (255.255.255.255).
    pub fn is_broadcast(&self) -> bool {
        self.octets == [255, 255, 255, 255]
    }
}

/// Check if IPv4 octets represent a private address (RFC 1918).
pub fn is_ipv4_private(octets: &[u8; 4]) -> bool {
    match octets[0] {
        10 => true,                                      // 10.0.0.0/8
        172 => octets[1] >= 16 && octets[1] <= 31,      // 172.16.0.0/12
        192 => octets[1] == 168,                         // 192.168.0.0/16
        _ => false,
    }
}

// ============================================================================
// IPv4 Contract Types
// ============================================================================

/// An IPv4 address guaranteed to be private (RFC 1918).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ipv4Private(Ipv4Bytes);

impl Ipv4Private {
    /// Create a new Ipv4Private, validating it's a private address.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::NotPrivateIp` if not private.
    pub fn new(octets: [u8; 4]) -> Result<Self, ValidationError> {
        let ipv4 = Ipv4Bytes::new(octets);
        if ipv4.is_private() {
            Ok(Self(ipv4))
        } else {
            Err(ValidationError::NotPrivateIp(format!(
                "{}.{}.{}.{}",
                octets[0], octets[1], octets[2], octets[3]
            )))
        }
    }

    /// Get the underlying Ipv4Bytes.
    pub fn get(&self) -> &Ipv4Bytes {
        &self.0
    }

    /// Get the octets.
    pub fn octets(&self) -> [u8; 4] {
        self.0.octets()
    }

    /// Unwrap into the underlying Ipv4Bytes.
    pub fn into_inner(self) -> Ipv4Bytes {
        self.0
    }
}

/// An IPv4 address guaranteed to be public (not private, not special).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ipv4Public(Ipv4Bytes);

impl Ipv4Public {
    /// Create a new Ipv4Public, validating it's a public address.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::NotPublicIp` if not public.
    pub fn new(octets: [u8; 4]) -> Result<Self, ValidationError> {
        let ipv4 = Ipv4Bytes::new(octets);
        if ipv4.is_public() {
            Ok(Self(ipv4))
        } else {
            Err(ValidationError::NotPublicIp(format!(
                "{}.{}.{}.{}",
                octets[0], octets[1], octets[2], octets[3]
            )))
        }
    }

    /// Get the underlying Ipv4Bytes.
    pub fn get(&self) -> &Ipv4Bytes {
        &self.0
    }

    /// Get the octets.
    pub fn octets(&self) -> [u8; 4] {
        self.0.octets()
    }

    /// Unwrap into the underlying Ipv4Bytes.
    pub fn into_inner(self) -> Ipv4Bytes {
        self.0
    }
}

// ============================================================================
// IPv6 Validation
// ============================================================================

/// A validated IPv6 address (16 bytes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ipv6Bytes {
    octets: [u8; 16],
}

impl Ipv6Bytes {
    /// Create a new Ipv6Bytes from octets.
    ///
    /// Always succeeds since all byte combinations are valid IPv6.
    pub fn new(octets: [u8; 16]) -> Self {
        Self { octets }
    }

    /// Get the octets.
    pub fn octets(&self) -> [u8; 16] {
        self.octets
    }

    /// Get the segments (8 × u16 big-endian).
    pub fn segments(&self) -> [u16; 8] {
        let mut segments = [0u16; 8];
        for i in 0..8 {
            segments[i] = u16::from_be_bytes([self.octets[i * 2], self.octets[i * 2 + 1]]);
        }
        segments
    }

    /// Check if this is a private address (fc00::/7 - unique local).
    pub fn is_private(&self) -> bool {
        is_ipv6_private(&self.octets)
    }

    /// Check if this is a public address (not private, not special).
    pub fn is_public(&self) -> bool {
        !self.is_private() && !self.is_loopback() && !self.is_unspecified() && !self.is_multicast()
    }

    /// Check if this is the loopback address (::1).
    pub fn is_loopback(&self) -> bool {
        self.octets[..15] == [0; 15] && self.octets[15] == 1
    }

    /// Check if this is multicast (ff00::/8).
    pub fn is_multicast(&self) -> bool {
        self.octets[0] == 0xff
    }

    /// Check if this is the unspecified address (::).
    pub fn is_unspecified(&self) -> bool {
        self.octets == [0; 16]
    }
}

/// Check if IPv6 octets represent a private address (fc00::/7).
pub fn is_ipv6_private(octets: &[u8; 16]) -> bool {
    // fc00::/7 means first 7 bits are 1111110
    // This is byte 0 & 0xfe == 0xfc
    (octets[0] & 0xfe) == 0xfc
}

// ============================================================================
// IPv6 Contract Types
// ============================================================================

/// An IPv6 address guaranteed to be private (fc00::/7).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ipv6Private(Ipv6Bytes);

impl Ipv6Private {
    /// Create a new Ipv6Private, validating it's a private address.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::NotPrivateIp` if not private.
    pub fn new(octets: [u8; 16]) -> Result<Self, ValidationError> {
        let ipv6 = Ipv6Bytes::new(octets);
        if ipv6.is_private() {
            Ok(Self(ipv6))
        } else {
            Err(ValidationError::NotPrivateIp(
                "IPv6 address not in fc00::/7".to_string()
            ))
        }
    }

    /// Get the underlying Ipv6Bytes.
    pub fn get(&self) -> &Ipv6Bytes {
        &self.0
    }

    /// Get the octets.
    pub fn octets(&self) -> [u8; 16] {
        self.0.octets()
    }

    /// Unwrap into the underlying Ipv6Bytes.
    pub fn into_inner(self) -> Ipv6Bytes {
        self.0
    }
}

/// An IPv6 address guaranteed to be public (not private, not special).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ipv6Public(Ipv6Bytes);

impl Ipv6Public {
    /// Create a new Ipv6Public, validating it's a public address.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::NotPublicIp` if not public.
    pub fn new(octets: [u8; 16]) -> Result<Self, ValidationError> {
        let ipv6 = Ipv6Bytes::new(octets);
        if ipv6.is_public() {
            Ok(Self(ipv6))
        } else {
            Err(ValidationError::NotPublicIp(
                "IPv6 address is not public".to_string()
            ))
        }
    }

    /// Get the underlying Ipv6Bytes.
    pub fn get(&self) -> &Ipv6Bytes {
        &self.0
    }

    /// Get the octets.
    pub fn octets(&self) -> [u8; 16] {
        self.0.octets()
    }

    /// Unwrap into the underlying Ipv6Bytes.
    pub fn into_inner(self) -> Ipv6Bytes {
        self.0
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // IPv4 Tests
    #[test]
    fn test_ipv4_private_10() {
        let octets = [10, 0, 0, 1];
        assert!(is_ipv4_private(&octets));
        
        let private = Ipv4Private::new(octets);
        assert!(private.is_ok());
    }

    #[test]
    fn test_ipv4_private_172() {
        // 172.16.0.0 - 172.31.255.255
        assert!(is_ipv4_private(&[172, 16, 0, 0]));
        assert!(is_ipv4_private(&[172, 31, 255, 255]));
        assert!(!is_ipv4_private(&[172, 15, 0, 0]));
        assert!(!is_ipv4_private(&[172, 32, 0, 0]));
    }

    #[test]
    fn test_ipv4_private_192() {
        assert!(is_ipv4_private(&[192, 168, 0, 1]));
        assert!(!is_ipv4_private(&[192, 167, 0, 1]));
        assert!(!is_ipv4_private(&[192, 169, 0, 1]));
    }

    #[test]
    fn test_ipv4_public() {
        let octets = [8, 8, 8, 8]; // Google DNS
        let ipv4 = Ipv4Bytes::new(octets);
        assert!(ipv4.is_public());
        
        let public = Ipv4Public::new(octets);
        assert!(public.is_ok());
    }

    #[test]
    fn test_ipv4_loopback() {
        let octets = [127, 0, 0, 1];
        let ipv4 = Ipv4Bytes::new(octets);
        assert!(ipv4.is_loopback());
        assert!(!ipv4.is_public());
    }

    #[test]
    fn test_ipv4_multicast() {
        let octets = [224, 0, 0, 1];
        let ipv4 = Ipv4Bytes::new(octets);
        assert!(ipv4.is_multicast());
    }

    // IPv6 Tests
    #[test]
    fn test_ipv6_private() {
        // fc00::/7 - first byte in range fc-fd
        let mut octets = [0u8; 16];
        octets[0] = 0xfc;
        assert!(is_ipv6_private(&octets));
        
        octets[0] = 0xfd;
        assert!(is_ipv6_private(&octets));
        
        let private = Ipv6Private::new(octets);
        assert!(private.is_ok());
    }

    #[test]
    fn test_ipv6_not_private() {
        let mut octets = [0u8; 16];
        octets[0] = 0xfe; // fe00::/7 is not private
        assert!(!is_ipv6_private(&octets));
    }

    #[test]
    fn test_ipv6_loopback() {
        let mut octets = [0u8; 16];
        octets[15] = 1; // ::1
        let ipv6 = Ipv6Bytes::new(octets);
        assert!(ipv6.is_loopback());
        assert!(!ipv6.is_public());
    }

    #[test]
    fn test_ipv6_multicast() {
        let mut octets = [0u8; 16];
        octets[0] = 0xff; // ff00::/8
        let ipv6 = Ipv6Bytes::new(octets);
        assert!(ipv6.is_multicast());
    }

    #[test]
    fn test_ipv6_public() {
        // 2001:4860:4860::8888 (Google DNS)
        let octets = [0x20, 0x01, 0x48, 0x60, 0x48, 0x60, 0, 0, 0, 0, 0, 0, 0, 0, 0x88, 0x88];
        let ipv6 = Ipv6Bytes::new(octets);
        assert!(ipv6.is_public());
        
        let public = Ipv6Public::new(octets);
        assert!(public.is_ok());
    }
}
