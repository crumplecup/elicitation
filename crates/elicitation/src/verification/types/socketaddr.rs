//! Socket address byte-level validation foundation.
//!
//! This module provides validated socket address byte sequences (IP + port).
//! It forms the foundation for socket address contract types.

use super::ValidationError;
use super::ipaddr_bytes::{Ipv4Bytes, Ipv6Bytes};

// ============================================================================
// Port Ranges (IANA)
// ============================================================================
//
// Well-Known Ports: 0-1023
//   - Assigned by IANA for standard services
//   - Examples: 80 (HTTP), 443 (HTTPS), 22 (SSH)
//
// Registered Ports: 1024-49151
//   - Registered with IANA for specific applications
//   - Examples: 3000 (dev servers), 5432 (PostgreSQL)
//
// Dynamic/Private Ports: 49152-65535
//   - Available for dynamic allocation, ephemeral connections
//
// Special:
//   - Port 0: Invalid for binding (used for "any available port")

// ============================================================================
// Port Validation
// ============================================================================

/// Check if port is well-known (0-1023).
pub fn is_well_known_port(port: u16) -> bool {
    port <= 1023
}

/// Check if port is registered (1024-49151).
pub fn is_registered_port(port: u16) -> bool {
    (1024..=49151).contains(&port)
}

/// Check if port is dynamic/private (49152-65535).
pub fn is_dynamic_port(port: u16) -> bool {
    port >= 49152
}

/// Check if port is privileged (< 1024, requires special permissions).
pub fn is_privileged_port(port: u16) -> bool {
    port < 1024
}

/// Check if port is non-zero.
pub fn is_nonzero_port(port: u16) -> bool {
    port != 0
}

// ============================================================================
// SocketAddrV4
// ============================================================================

/// A validated IPv4 socket address (IPv4 + port).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SocketAddrV4Bytes {
    ip: Ipv4Bytes,
    port: u16,
}

impl SocketAddrV4Bytes {
    /// Create a new SocketAddrV4Bytes.
    ///
    /// Always succeeds since all combinations are valid.
    pub fn new(ip: Ipv4Bytes, port: u16) -> Self {
        Self { ip, port }
    }

    /// Create from raw octets and port.
    pub fn from_octets(ip_octets: [u8; 4], port: u16) -> Self {
        Self::new(Ipv4Bytes::new(ip_octets), port)
    }

    /// Get the IP address.
    pub fn ip(&self) -> &Ipv4Bytes {
        &self.ip
    }

    /// Get the port.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Decompose into IP and port.
    pub fn into_parts(self) -> (Ipv4Bytes, u16) {
        (self.ip, self.port)
    }
}

// ============================================================================
// SocketAddrV6
// ============================================================================

/// A validated IPv6 socket address (IPv6 + port).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SocketAddrV6Bytes {
    ip: Ipv6Bytes,
    port: u16,
}

impl SocketAddrV6Bytes {
    /// Create a new SocketAddrV6Bytes.
    ///
    /// Always succeeds since all combinations are valid.
    pub fn new(ip: Ipv6Bytes, port: u16) -> Self {
        Self { ip, port }
    }

    /// Create from raw octets and port.
    pub fn from_octets(ip_octets: [u8; 16], port: u16) -> Self {
        Self::new(Ipv6Bytes::new(ip_octets), port)
    }

    /// Get the IP address.
    pub fn ip(&self) -> &Ipv6Bytes {
        &self.ip
    }

    /// Get the port.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Decompose into IP and port.
    pub fn into_parts(self) -> (Ipv6Bytes, u16) {
        (self.ip, self.port)
    }
}

// ============================================================================
// Contract Types: Non-Zero Port
// ============================================================================

/// An IPv4 socket address with non-zero port.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SocketAddrV4NonZero(SocketAddrV4Bytes);

impl SocketAddrV4NonZero {
    /// Create a new SocketAddrV4NonZero, validating port is non-zero.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::PortIsZero` if port is 0.
    pub fn new(ip: Ipv4Bytes, port: u16) -> Result<Self, ValidationError> {
        if port == 0 {
            return Err(ValidationError::PortIsZero);
        }
        Ok(Self(SocketAddrV4Bytes::new(ip, port)))
    }

    /// Get the underlying SocketAddrV4Bytes.
    pub fn get(&self) -> &SocketAddrV4Bytes {
        &self.0
    }

    /// Get the IP address.
    pub fn ip(&self) -> &Ipv4Bytes {
        self.0.ip()
    }

    /// Get the port (guaranteed non-zero).
    pub fn port(&self) -> u16 {
        self.0.port()
    }

    /// Unwrap into the underlying SocketAddrV4Bytes.
    pub fn into_inner(self) -> SocketAddrV4Bytes {
        self.0
    }
}

/// An IPv6 socket address with non-zero port.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SocketAddrV6NonZero(SocketAddrV6Bytes);

impl SocketAddrV6NonZero {
    /// Create a new SocketAddrV6NonZero, validating port is non-zero.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::PortIsZero` if port is 0.
    pub fn new(ip: Ipv6Bytes, port: u16) -> Result<Self, ValidationError> {
        if port == 0 {
            return Err(ValidationError::PortIsZero);
        }
        Ok(Self(SocketAddrV6Bytes::new(ip, port)))
    }

    /// Get the underlying SocketAddrV6Bytes.
    pub fn get(&self) -> &SocketAddrV6Bytes {
        &self.0
    }

    /// Get the IP address.
    pub fn ip(&self) -> &Ipv6Bytes {
        self.0.ip()
    }

    /// Get the port (guaranteed non-zero).
    pub fn port(&self) -> u16 {
        self.0.port()
    }

    /// Unwrap into the underlying SocketAddrV6Bytes.
    pub fn into_inner(self) -> SocketAddrV6Bytes {
        self.0
    }
}

// ============================================================================
// Contract Types: Privileged Ports
// ============================================================================

/// An IPv4 socket address with privileged port (< 1024).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SocketAddrV4Privileged(SocketAddrV4Bytes);

impl SocketAddrV4Privileged {
    /// Create a new SocketAddrV4Privileged, validating port is privileged.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::PortNotPrivileged` if port >= 1024.
    pub fn new(ip: Ipv4Bytes, port: u16) -> Result<Self, ValidationError> {
        if port >= 1024 {
            return Err(ValidationError::PortNotPrivileged(port));
        }
        Ok(Self(SocketAddrV4Bytes::new(ip, port)))
    }

    /// Get the underlying SocketAddrV4Bytes.
    pub fn get(&self) -> &SocketAddrV4Bytes {
        &self.0
    }

    /// Get the port (guaranteed < 1024).
    pub fn port(&self) -> u16 {
        self.0.port()
    }

    /// Unwrap into the underlying SocketAddrV4Bytes.
    pub fn into_inner(self) -> SocketAddrV4Bytes {
        self.0
    }
}

/// An IPv6 socket address with privileged port (< 1024).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SocketAddrV6Privileged(SocketAddrV6Bytes);

impl SocketAddrV6Privileged {
    /// Create a new SocketAddrV6Privileged, validating port is privileged.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::PortNotPrivileged` if port >= 1024.
    pub fn new(ip: Ipv6Bytes, port: u16) -> Result<Self, ValidationError> {
        if port >= 1024 {
            return Err(ValidationError::PortNotPrivileged(port));
        }
        Ok(Self(SocketAddrV6Bytes::new(ip, port)))
    }

    /// Get the underlying SocketAddrV6Bytes.
    pub fn get(&self) -> &SocketAddrV6Bytes {
        &self.0
    }

    /// Get the port (guaranteed < 1024).
    pub fn port(&self) -> u16 {
        self.0.port()
    }

    /// Unwrap into the underlying SocketAddrV6Bytes.
    pub fn into_inner(self) -> SocketAddrV6Bytes {
        self.0
    }
}

// ============================================================================
// Contract Types: Unprivileged Ports
// ============================================================================

/// An IPv4 socket address with unprivileged port (>= 1024).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SocketAddrV4Unprivileged(SocketAddrV4Bytes);

impl SocketAddrV4Unprivileged {
    /// Create a new SocketAddrV4Unprivileged, validating port is unprivileged.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::PortIsPrivileged` if port < 1024.
    pub fn new(ip: Ipv4Bytes, port: u16) -> Result<Self, ValidationError> {
        if port < 1024 {
            return Err(ValidationError::PortIsPrivileged(port));
        }
        Ok(Self(SocketAddrV4Bytes::new(ip, port)))
    }

    /// Get the underlying SocketAddrV4Bytes.
    pub fn get(&self) -> &SocketAddrV4Bytes {
        &self.0
    }

    /// Get the port (guaranteed >= 1024).
    pub fn port(&self) -> u16 {
        self.0.port()
    }

    /// Unwrap into the underlying SocketAddrV4Bytes.
    pub fn into_inner(self) -> SocketAddrV4Bytes {
        self.0
    }
}

/// An IPv6 socket address with unprivileged port (>= 1024).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SocketAddrV6Unprivileged(SocketAddrV6Bytes);

impl SocketAddrV6Unprivileged {
    /// Create a new SocketAddrV6Unprivileged, validating port is unprivileged.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::PortIsPrivileged` if port < 1024.
    pub fn new(ip: Ipv6Bytes, port: u16) -> Result<Self, ValidationError> {
        if port < 1024 {
            return Err(ValidationError::PortIsPrivileged(port));
        }
        Ok(Self(SocketAddrV6Bytes::new(ip, port)))
    }

    /// Get the underlying SocketAddrV6Bytes.
    pub fn get(&self) -> &SocketAddrV6Bytes {
        &self.0
    }

    /// Get the port (guaranteed >= 1024).
    pub fn port(&self) -> u16 {
        self.0.port()
    }

    /// Unwrap into the underlying SocketAddrV6Bytes.
    pub fn into_inner(self) -> SocketAddrV6Bytes {
        self.0
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_ranges() {
        assert!(is_well_known_port(80));
        assert!(is_well_known_port(443));
        assert!(!is_well_known_port(1024));

        assert!(is_registered_port(3000));
        assert!(is_registered_port(5432));
        assert!(!is_registered_port(1023));
        assert!(!is_registered_port(49152));

        assert!(is_dynamic_port(49152));
        assert!(is_dynamic_port(65535));
        assert!(!is_dynamic_port(49151));
    }

    #[test]
    fn test_privileged_port() {
        assert!(is_privileged_port(0));
        assert!(is_privileged_port(80));
        assert!(is_privileged_port(1023));
        assert!(!is_privileged_port(1024));
        assert!(!is_privileged_port(8080));
    }

    #[test]
    fn test_socketaddrv4_construction() {
        let ip = Ipv4Bytes::new([192, 168, 1, 1]);
        let port = 8080;

        let addr = SocketAddrV4Bytes::new(ip, port);
        assert_eq!(addr.ip().octets(), [192, 168, 1, 1]);
        assert_eq!(addr.port(), 8080);
    }

    #[test]
    fn test_socketaddrv4_nonzero() {
        let ip = Ipv4Bytes::new([192, 168, 1, 1]);

        let zero_result = SocketAddrV4NonZero::new(ip, 0);
        assert!(zero_result.is_err());

        let nonzero_result = SocketAddrV4NonZero::new(ip, 8080);
        assert!(nonzero_result.is_ok());
        assert_eq!(nonzero_result.unwrap().port(), 8080);
    }

    #[test]
    fn test_socketaddrv4_privileged() {
        let ip = Ipv4Bytes::new([192, 168, 1, 1]);

        let privileged = SocketAddrV4Privileged::new(ip, 80);
        assert!(privileged.is_ok());
        assert_eq!(privileged.unwrap().port(), 80);

        let unprivileged = SocketAddrV4Privileged::new(ip, 8080);
        assert!(unprivileged.is_err());
    }

    #[test]
    fn test_socketaddrv4_unprivileged() {
        let ip = Ipv4Bytes::new([192, 168, 1, 1]);

        let unprivileged = SocketAddrV4Unprivileged::new(ip, 8080);
        assert!(unprivileged.is_ok());
        assert_eq!(unprivileged.unwrap().port(), 8080);

        let privileged = SocketAddrV4Unprivileged::new(ip, 80);
        assert!(privileged.is_err());
    }

    #[test]
    fn test_socketaddrv6_construction() {
        let ip = Ipv6Bytes::new([0x20, 0x01, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
        let port = 8080;

        let addr = SocketAddrV6Bytes::new(ip, port);
        assert_eq!(addr.port(), 8080);
    }

    #[test]
    fn test_socketaddrv6_nonzero() {
        let ip = Ipv6Bytes::new([0x20, 0x01, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);

        let zero_result = SocketAddrV6NonZero::new(ip, 0);
        assert!(zero_result.is_err());

        let nonzero_result = SocketAddrV6NonZero::new(ip, 8080);
        assert!(nonzero_result.is_ok());
    }
}
