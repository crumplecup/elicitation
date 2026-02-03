//! MAC address byte-level validation foundation.
//!
//! This module provides validated MAC address byte sequences (EUI-48).
//! It forms the foundation for MAC address contract types.

use super::ValidationError;

// ============================================================================
// MAC Address Structure (EUI-48 / IEEE 802)
// ============================================================================
//
// 6 bytes (48 bits): XX:XX:XX:XX:XX:XX
//
// Byte 0, bit 0 (LSB): Unicast/Multicast
//   - 0 = Unicast (individual address)
//   - 1 = Multicast (group address)
//
// Byte 0, bit 1: Universal/Local
//   - 0 = Universal (globally unique, assigned by IEEE)
//   - 1 = Local (locally administered)
//
// Examples:
//   - 00:1A:2B:3C:4D:5E (unicast, universal)
//   - 01:00:5E:00:00:01 (multicast, universal - IPv4 multicast)
//   - 02:1A:2B:3C:4D:5E (unicast, local)
//   - FF:FF:FF:FF:FF:FF (broadcast - special multicast)

// ============================================================================
// Core MAC Address Type
// ============================================================================

/// A validated MAC address (6 bytes, EUI-48).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MacAddr {
    octets: [u8; 6],
}

impl MacAddr {
    /// Create a new MacAddr from octets.
    ///
    /// Always succeeds since all octet combinations are valid MAC addresses.
    pub fn new(octets: [u8; 6]) -> Self {
        Self { octets }
    }

    /// Get the octets.
    pub fn octets(&self) -> [u8; 6] {
        self.octets
    }

    /// Check if this is a unicast address (bit 0 of byte 0 is 0).
    pub fn is_unicast(&self) -> bool {
        (self.octets[0] & 0x01) == 0
    }

    /// Check if this is a multicast address (bit 0 of byte 0 is 1).
    pub fn is_multicast(&self) -> bool {
        (self.octets[0] & 0x01) == 1
    }

    /// Check if this is a universal address (bit 1 of byte 0 is 0).
    pub fn is_universal(&self) -> bool {
        (self.octets[0] & 0x02) == 0
    }

    /// Check if this is a locally administered address (bit 1 of byte 0 is 1).
    pub fn is_local(&self) -> bool {
        (self.octets[0] & 0x02) == 2
    }

    /// Check if this is the broadcast address (FF:FF:FF:FF:FF:FF).
    pub fn is_broadcast(&self) -> bool {
        self.octets == [0xFF; 6]
    }

    /// Check if this is a null address (00:00:00:00:00:00).
    pub fn is_null(&self) -> bool {
        self.octets == [0x00; 6]
    }
}

// ============================================================================
// Validation Functions
// ============================================================================

/// Check if MAC address is unicast (bit 0 of byte 0 is 0).
pub fn is_unicast(octets: &[u8; 6]) -> bool {
    (octets[0] & 0x01) == 0
}

/// Check if MAC address is multicast (bit 0 of byte 0 is 1).
pub fn is_multicast(octets: &[u8; 6]) -> bool {
    (octets[0] & 0x01) == 1
}

/// Check if MAC address is universal (bit 1 of byte 0 is 0).
pub fn is_universal(octets: &[u8; 6]) -> bool {
    (octets[0] & 0x02) == 0
}

/// Check if MAC address is locally administered (bit 1 of byte 0 is 1).
pub fn is_local(octets: &[u8; 6]) -> bool {
    (octets[0] & 0x02) == 2
}

// ============================================================================
// Contract Types
// ============================================================================

/// A MAC address guaranteed to be unicast.
///
/// Unicast addresses are individual device addresses (bit 0 of byte 0 is 0).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MacUnicast(MacAddr);

impl MacUnicast {
    /// Create a new MacUnicast, validating it's a unicast address.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::NotUnicastMac` if multicast.
    pub fn new(octets: [u8; 6]) -> Result<Self, ValidationError> {
        let mac = MacAddr::new(octets);
        if mac.is_unicast() {
            Ok(Self(mac))
        } else {
            Err(ValidationError::NotUnicastMac)
        }
    }

    /// Get the underlying MacAddr.
    pub fn get(&self) -> &MacAddr {
        &self.0
    }

    /// Get the octets.
    pub fn octets(&self) -> [u8; 6] {
        self.0.octets()
    }

    /// Unwrap into the underlying MacAddr.
    pub fn into_inner(self) -> MacAddr {
        self.0
    }
}

/// A MAC address guaranteed to be multicast.
///
/// Multicast addresses are group addresses (bit 0 of byte 0 is 1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MacMulticast(MacAddr);

impl MacMulticast {
    /// Create a new MacMulticast, validating it's a multicast address.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::NotMulticastMac` if unicast.
    pub fn new(octets: [u8; 6]) -> Result<Self, ValidationError> {
        let mac = MacAddr::new(octets);
        if mac.is_multicast() {
            Ok(Self(mac))
        } else {
            Err(ValidationError::NotMulticastMac)
        }
    }

    /// Get the underlying MacAddr.
    pub fn get(&self) -> &MacAddr {
        &self.0
    }

    /// Get the octets.
    pub fn octets(&self) -> [u8; 6] {
        self.0.octets()
    }

    /// Unwrap into the underlying MacAddr.
    pub fn into_inner(self) -> MacAddr {
        self.0
    }
}

/// A MAC address guaranteed to be universal (IEEE assigned).
///
/// Universal addresses have globally unique OUIs (bit 1 of byte 0 is 0).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MacUniversal(MacAddr);

impl MacUniversal {
    /// Create a new MacUniversal, validating it's a universal address.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::NotUniversalMac` if locally administered.
    pub fn new(octets: [u8; 6]) -> Result<Self, ValidationError> {
        let mac = MacAddr::new(octets);
        if mac.is_universal() {
            Ok(Self(mac))
        } else {
            Err(ValidationError::NotUniversalMac)
        }
    }

    /// Get the underlying MacAddr.
    pub fn get(&self) -> &MacAddr {
        &self.0
    }

    /// Get the octets.
    pub fn octets(&self) -> [u8; 6] {
        self.0.octets()
    }

    /// Unwrap into the underlying MacAddr.
    pub fn into_inner(self) -> MacAddr {
        self.0
    }
}

/// A MAC address guaranteed to be locally administered.
///
/// Locally administered addresses are user-configurable (bit 1 of byte 0 is 1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MacLocal(MacAddr);

impl MacLocal {
    /// Create a new MacLocal, validating it's a locally administered address.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::NotLocalMac` if universal.
    pub fn new(octets: [u8; 6]) -> Result<Self, ValidationError> {
        let mac = MacAddr::new(octets);
        if mac.is_local() {
            Ok(Self(mac))
        } else {
            Err(ValidationError::NotLocalMac)
        }
    }

    /// Get the underlying MacAddr.
    pub fn get(&self) -> &MacAddr {
        &self.0
    }

    /// Get the octets.
    pub fn octets(&self) -> [u8; 6] {
        self.0.octets()
    }

    /// Unwrap into the underlying MacAddr.
    pub fn into_inner(self) -> MacAddr {
        self.0
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Format MAC address as XX:XX:XX:XX:XX:XX.
// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unicast_detection() {
        // Even first byte = unicast
        let unicast = [0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E];
        assert!(is_unicast(&unicast));
        assert!(!is_multicast(&unicast));

        let mac = MacAddr::new(unicast);
        assert!(mac.is_unicast());
        assert!(!mac.is_multicast());

        let unicast_type = MacUnicast::new(unicast);
        assert!(unicast_type.is_ok());
    }

    #[test]
    fn test_multicast_detection() {
        // Odd first byte = multicast
        let multicast = [0x01, 0x00, 0x5E, 0x00, 0x00, 0x01];
        assert!(is_multicast(&multicast));
        assert!(!is_unicast(&multicast));

        let mac = MacAddr::new(multicast);
        assert!(mac.is_multicast());
        assert!(!mac.is_unicast());

        let multicast_type = MacMulticast::new(multicast);
        assert!(multicast_type.is_ok());
    }

    #[test]
    fn test_broadcast() {
        let broadcast = [0xFF; 6];
        let mac = MacAddr::new(broadcast);
        assert!(mac.is_broadcast());
        assert!(mac.is_multicast()); // Broadcast is a special multicast
    }

    #[test]
    fn test_null_address() {
        let null = [0x00; 6];
        let mac = MacAddr::new(null);
        assert!(mac.is_null());
        assert!(mac.is_unicast()); // Null is unicast
    }

    #[test]
    fn test_universal_detection() {
        // Bit 1 clear = universal
        let universal = [0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E];
        assert!(is_universal(&universal));
        assert!(!is_local(&universal));

        let mac = MacAddr::new(universal);
        assert!(mac.is_universal());
        assert!(!mac.is_local());

        let universal_type = MacUniversal::new(universal);
        assert!(universal_type.is_ok());
    }

    #[test]
    fn test_local_detection() {
        // Bit 1 set = local
        let local = [0x02, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E];
        assert!(is_local(&local));
        assert!(!is_universal(&local));

        let mac = MacAddr::new(local);
        assert!(mac.is_local());
        assert!(!mac.is_universal());

        let local_type = MacLocal::new(local);
        assert!(local_type.is_ok());
    }

    #[test]
    fn test_unicast_rejects_multicast() {
        let multicast = [0x01, 0x00, 0x5E, 0x00, 0x00, 0x01];
        let result = MacUnicast::new(multicast);
        assert!(result.is_err());
    }

    #[test]
    fn test_multicast_rejects_unicast() {
        let unicast = [0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E];
        let result = MacMulticast::new(unicast);
        assert!(result.is_err());
    }

    #[test]
    fn test_combined_bits() {
        // Both bits set: local multicast (0x03)
        let local_multicast = [0x03, 0x00, 0x00, 0x00, 0x00, 0x01];
        let mac = MacAddr::new(local_multicast);
        assert!(mac.is_multicast());
        assert!(mac.is_local());

        // Only bit 1 set: local unicast (0x02)
        let local_unicast = [0x02, 0x00, 0x00, 0x00, 0x00, 0x01];
        let mac = MacAddr::new(local_unicast);
        assert!(mac.is_unicast());
        assert!(mac.is_local());

        // Only bit 0 set: universal multicast (0x01)
        let universal_multicast = [0x01, 0x00, 0x5E, 0x00, 0x00, 0x01];
        let mac = MacAddr::new(universal_multicast);
        assert!(mac.is_multicast());
        assert!(mac.is_universal());
    }
}
