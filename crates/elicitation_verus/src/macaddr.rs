use verus_builtin_macros::verus;

verus! {

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    InvalidBytes,
    InvalidClassification,
}

// ============================================================================
// MacAddr - 6-byte MAC address (IEEE 802)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MacAddr {
    pub bytes: [u8; 6],
    pub validated: bool,
}

impl MacAddr {
    /// All byte combinations are valid MAC addresses.
    pub fn new(bytes: [u8; 6]) -> (result: Self)
        ensures
            result.bytes == bytes,
            result.validated == true,
    {
        MacAddr { bytes, validated: true }
    }

    pub fn is_multicast(&self) -> (result: bool)
        requires self.validated == true,
    {
        // Multicast bit: LSB of first byte
        (self.bytes[0] & 0x01) == 0x01
    }

    pub fn is_unicast(&self) -> (result: bool)
        requires self.validated == true,
    {
        // Unicast: NOT multicast
        (self.bytes[0] & 0x01) == 0x00
    }

    pub fn is_local(&self) -> (result: bool)
        requires self.validated == true,
    {
        // Locally administered: 2nd LSB of first byte
        (self.bytes[0] & 0x02) == 0x02
    }

    pub fn is_universal(&self) -> (result: bool)
        requires self.validated == true,
    {
        // Universally administered: NOT local
        (self.bytes[0] & 0x02) == 0x00
    }
}

// ============================================================================
// MacAddrMulticast - Multicast MAC address
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MacAddrMulticast {
    pub bytes: [u8; 6],
    pub validated: bool,
}

impl MacAddrMulticast {
    /// Parameters:
    /// - is_multicast: LSB of first byte is 1
    pub fn new(bytes: [u8; 6], is_multicast: bool) -> (result: Result<Self, ValidationError>)
        ensures
            is_multicast ==> (result matches Ok(mac) && mac.bytes == bytes && mac.validated == true),
            !is_multicast ==> (result matches Err(ValidationError::InvalidClassification)),
    {
        if is_multicast {
            Ok(MacAddrMulticast { bytes, validated: true })
        } else {
            Err(ValidationError::InvalidClassification)
        }
    }
}

// ============================================================================
// MacAddrUnicast - Unicast MAC address
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MacAddrUnicast {
    pub bytes: [u8; 6],
    pub validated: bool,
}

impl MacAddrUnicast {
    /// Parameters:
    /// - is_unicast: LSB of first byte is 0
    pub fn new(bytes: [u8; 6], is_unicast: bool) -> (result: Result<Self, ValidationError>)
        ensures
            is_unicast ==> (result matches Ok(mac) && mac.bytes == bytes && mac.validated == true),
            !is_unicast ==> (result matches Err(ValidationError::InvalidClassification)),
    {
        if is_unicast {
            Ok(MacAddrUnicast { bytes, validated: true })
        } else {
            Err(ValidationError::InvalidClassification)
        }
    }
}

// ============================================================================
// MacAddrLocal - Locally administered MAC address
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MacAddrLocal {
    pub bytes: [u8; 6],
    pub validated: bool,
}

impl MacAddrLocal {
    /// Parameters:
    /// - is_local: 2nd LSB of first byte is 1
    pub fn new(bytes: [u8; 6], is_local: bool) -> (result: Result<Self, ValidationError>)
        ensures
            is_local ==> (result matches Ok(mac) && mac.bytes == bytes && mac.validated == true),
            !is_local ==> (result matches Err(ValidationError::InvalidClassification)),
    {
        if is_local {
            Ok(MacAddrLocal { bytes, validated: true })
        } else {
            Err(ValidationError::InvalidClassification)
        }
    }
}

// ============================================================================
// MacAddrUniversal - Universally administered MAC address
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MacAddrUniversal {
    pub bytes: [u8; 6],
    pub validated: bool,
}

impl MacAddrUniversal {
    /// Parameters:
    /// - is_universal: 2nd LSB of first byte is 0
    pub fn new(bytes: [u8; 6], is_universal: bool) -> (result: Result<Self, ValidationError>)
        ensures
            is_universal ==> (result matches Ok(mac) && mac.bytes == bytes && mac.validated == true),
            !is_universal ==> (result matches Err(ValidationError::InvalidClassification)),
    {
        if is_universal {
            Ok(MacAddrUniversal { bytes, validated: true })
        } else {
            Err(ValidationError::InvalidClassification)
        }
    }
}

} // verus!
