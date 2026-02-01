//! Network contract types.

use super::ValidationError;
use crate::{ElicitClient, ElicitResult, Elicitation, Prompt};
use elicitation_macros::instrumented_impl;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

// IpPrivate - Private IP addresses
/// An IP address that is guaranteed to be private (RFC 1918, RFC 4193).
///
/// Private IPv4: 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16
/// Private IPv6: fc00::/7
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IpPrivate(IpAddr);

#[instrumented_impl]
impl IpPrivate {
    /// Create a new IpPrivate, validating it's a private address.
    pub fn new(ip: IpAddr) -> Result<Self, ValidationError> {
        // Check based on IP version
        let is_private = match ip {
            IpAddr::V4(v4) => v4.is_private(),
            IpAddr::V6(v6) => {
                // IPv6 unique local addresses (fc00::/7)
                let segments = v6.segments();
                (segments[0] & 0xfe00) == 0xfc00
            }
        };

        if is_private {
            Ok(Self(ip))
        } else {
            Err(ValidationError::NotPrivateIp)
        }
    }

    /// Get the inner IP address.
    pub fn get(&self) -> IpAddr {
        self.0
    }

    /// Unwrap into the inner IP address.
    pub fn into_inner(self) -> IpAddr {
        self.0
    }
}

#[instrumented_impl]
impl Prompt for IpPrivate {
    fn prompt() -> Option<&'static str> {
        Some("Please provide a private IP address (RFC 1918 or RFC 4193):")
    }
}

#[instrumented_impl]
impl Elicitation for IpPrivate {
    type Style = <IpAddr as Elicitation>::Style;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting IpPrivate");
        loop {
            let ip = IpAddr::elicit(client).await?;
            match Self::new(ip) {
                Ok(valid) => {
                    tracing::debug!(ip = %valid.0, "Valid private IP");
                    return Ok(valid);
                }
                Err(e) => {
                    tracing::warn!(error = %e, "IP not private, re-prompting");
                    continue;
                }
            }
        }
    }
}

// IpPublic - Public IP addresses
/// An IP address that is guaranteed to be public (not private).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IpPublic(IpAddr);

#[instrumented_impl]
impl IpPublic {
    /// Create a new IpPublic, validating it's not a private address.
    pub fn new(ip: IpAddr) -> Result<Self, ValidationError> {
        // Use IpPrivate validation logic, but invert
        match IpPrivate::new(ip) {
            Err(_) => Ok(Self(ip)),
            Ok(_) => Err(ValidationError::NotPublicIp),
        }
    }

    /// Get the inner IP address.
    pub fn get(&self) -> IpAddr {
        self.0
    }

    /// Unwrap into the inner IP address.
    pub fn into_inner(self) -> IpAddr {
        self.0
    }
}

#[instrumented_impl]
impl Prompt for IpPublic {
    fn prompt() -> Option<&'static str> {
        Some("Please provide a public IP address (not RFC 1918 or RFC 4193):")
    }
}

#[instrumented_impl]
impl Elicitation for IpPublic {
    type Style = <IpAddr as Elicitation>::Style;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting IpPublic");
        loop {
            let ip = IpAddr::elicit(client).await?;
            match Self::new(ip) {
                Ok(valid) => {
                    tracing::debug!(ip = %valid.0, "Valid public IP");
                    return Ok(valid);
                }
                Err(e) => {
                    tracing::warn!(error = %e, "IP not public, re-prompting");
                    continue;
                }
            }
        }
    }
}

// IpV4 - IPv4 addresses from IpAddr
/// An IP address that is guaranteed to be IPv4.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IpV4(Ipv4Addr);

#[instrumented_impl]
impl IpV4 {
    /// Create a new IpV4, validating it's an IPv4 address.
    pub fn new(ip: IpAddr) -> Result<Self, ValidationError> {
        match ip {
            IpAddr::V4(v4) => Ok(Self(v4)),
            IpAddr::V6(v6) => Err(ValidationError::WrongIpVersion {
                expected: "IPv4".to_string(),
                got: format!("IPv6 ({})", v6),
            }),
        }
    }

    /// Get the inner IPv4 address.
    pub fn get(&self) -> Ipv4Addr {
        self.0
    }

    /// Unwrap into the inner IPv4 address.
    pub fn into_inner(self) -> Ipv4Addr {
        self.0
    }
}

#[instrumented_impl]
impl Prompt for IpV4 {
    fn prompt() -> Option<&'static str> {
        Some("Please provide an IPv4 address:")
    }
}

#[instrumented_impl]
impl Elicitation for IpV4 {
    type Style = <IpAddr as Elicitation>::Style;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting IpV4");
        loop {
            let ip = IpAddr::elicit(client).await?;
            match Self::new(ip) {
                Ok(valid) => {
                    tracing::debug!(ip = %valid.0, "Valid IPv4");
                    return Ok(valid);
                }
                Err(e) => {
                    tracing::warn!(error = %e, "Not IPv4, re-prompting");
                    continue;
                }
            }
        }
    }
}

// IpV6 - IPv6 addresses from IpAddr
/// An IP address that is guaranteed to be IPv6.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IpV6(Ipv6Addr);

#[instrumented_impl]
impl IpV6 {
    /// Create a new IpV6, validating it's an IPv6 address.
    pub fn new(ip: IpAddr) -> Result<Self, ValidationError> {
        match ip {
            IpAddr::V6(v6) => Ok(Self(v6)),
            IpAddr::V4(v4) => Err(ValidationError::WrongIpVersion {
                expected: "IPv6".to_string(),
                got: format!("IPv4 ({})", v4),
            }),
        }
    }

    /// Get the inner IPv6 address.
    pub fn get(&self) -> Ipv6Addr {
        self.0
    }

    /// Unwrap into the inner IPv6 address.
    pub fn into_inner(self) -> Ipv6Addr {
        self.0
    }
}

#[instrumented_impl]
impl Prompt for IpV6 {
    fn prompt() -> Option<&'static str> {
        Some("Please provide an IPv6 address:")
    }
}

#[instrumented_impl]
impl Elicitation for IpV6 {
    type Style = <IpAddr as Elicitation>::Style;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting IpV6");
        loop {
            let ip = IpAddr::elicit(client).await?;
            match Self::new(ip) {
                Ok(valid) => {
                    tracing::debug!(ip = %valid.0, "Valid IPv6");
                    return Ok(valid);
                }
                Err(e) => {
                    tracing::warn!(error = %e, "Not IPv6, re-prompting");
                    continue;
                }
            }
        }
    }
}

// Ipv4Loopback - IPv4 loopback addresses (127.0.0.0/8)
/// An IPv4 address that is guaranteed to be loopback (127.0.0.0/8).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ipv4Loopback(Ipv4Addr);

#[instrumented_impl]
impl Ipv4Loopback {
    /// Create a new Ipv4Loopback, validating it's a loopback address.
    pub fn new(ip: Ipv4Addr) -> Result<Self, ValidationError> {
        if ip.is_loopback() {
            Ok(Self(ip))
        } else {
            Err(ValidationError::NotLoopback(ip.to_string()))
        }
    }

    /// Get the inner IPv4 address.
    pub fn get(&self) -> Ipv4Addr {
        self.0
    }

    /// Unwrap into the inner IPv4 address.
    pub fn into_inner(self) -> Ipv4Addr {
        self.0
    }
}

#[instrumented_impl]
impl Prompt for Ipv4Loopback {
    fn prompt() -> Option<&'static str> {
        Some("Please provide an IPv4 loopback address (127.0.0.0/8):")
    }
}

#[instrumented_impl]
impl Elicitation for Ipv4Loopback {
    type Style = <Ipv4Addr as Elicitation>::Style;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting Ipv4Loopback");
        loop {
            let ip = Ipv4Addr::elicit(client).await?;
            match Self::new(ip) {
                Ok(valid) => {
                    tracing::debug!(ip = %valid.0, "Valid IPv4 loopback");
                    return Ok(valid);
                }
                Err(e) => {
                    tracing::warn!(error = %e, "Not loopback, re-prompting");
                    continue;
                }
            }
        }
    }
}

// Ipv6Loopback - IPv6 loopback address (::1)
/// An IPv6 address that is guaranteed to be loopback (::1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ipv6Loopback(Ipv6Addr);

#[instrumented_impl]
impl Ipv6Loopback {
    /// Create a new Ipv6Loopback, validating it's the loopback address.
    pub fn new(ip: Ipv6Addr) -> Result<Self, ValidationError> {
        if ip.is_loopback() {
            Ok(Self(ip))
        } else {
            Err(ValidationError::NotLoopback(ip.to_string()))
        }
    }

    /// Get the inner IPv6 address.
    pub fn get(&self) -> Ipv6Addr {
        self.0
    }

    /// Unwrap into the inner IPv6 address.
    pub fn into_inner(self) -> Ipv6Addr {
        self.0
    }
}

#[instrumented_impl]
impl Prompt for Ipv6Loopback {
    fn prompt() -> Option<&'static str> {
        Some("Please provide an IPv6 loopback address (::1):")
    }
}

#[instrumented_impl]
impl Elicitation for Ipv6Loopback {
    type Style = <Ipv6Addr as Elicitation>::Style;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting Ipv6Loopback");
        loop {
            let ip = Ipv6Addr::elicit(client).await?;
            match Self::new(ip) {
                Ok(valid) => {
                    tracing::debug!(ip = %valid.0, "Valid IPv6 loopback");
                    return Ok(valid);
                }
                Err(e) => {
                    tracing::warn!(error = %e, "Not loopback, re-prompting");
                    continue;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // IpPrivate tests
    #[test]
    fn test_ip_private_v4_valid() {
        let ip = "192.168.1.1".parse::<IpAddr>().unwrap();
        assert!(IpPrivate::new(ip).is_ok());
    }

    #[test]
    fn test_ip_private_v4_public() {
        let ip = "8.8.8.8".parse::<IpAddr>().unwrap();
        assert!(IpPrivate::new(ip).is_err());
    }

    #[test]
    fn test_ip_private_v6_valid() {
        let ip = "fc00::1".parse::<IpAddr>().unwrap();
        assert!(IpPrivate::new(ip).is_ok());
    }

    // IpPublic tests
    #[test]
    fn test_ip_public_valid() {
        let ip = "8.8.8.8".parse::<IpAddr>().unwrap();
        assert!(IpPublic::new(ip).is_ok());
    }

    #[test]
    fn test_ip_public_private() {
        let ip = "192.168.1.1".parse::<IpAddr>().unwrap();
        assert!(IpPublic::new(ip).is_err());
    }

    // IpV4 tests
    #[test]
    fn test_ip_v4_valid() {
        let ip = "192.168.1.1".parse::<IpAddr>().unwrap();
        let result = IpV4::new(ip);
        assert!(result.is_ok());
    }

    #[test]
    fn test_ip_v4_wrong_version() {
        let ip = "::1".parse::<IpAddr>().unwrap();
        assert!(IpV4::new(ip).is_err());
    }

    // IpV6 tests
    #[test]
    fn test_ip_v6_valid() {
        let ip = "::1".parse::<IpAddr>().unwrap();
        let result = IpV6::new(ip);
        assert!(result.is_ok());
    }

    #[test]
    fn test_ip_v6_wrong_version() {
        let ip = "192.168.1.1".parse::<IpAddr>().unwrap();
        assert!(IpV6::new(ip).is_err());
    }

    // Loopback tests
    #[test]
    fn test_ipv4_loopback_valid() {
        let ip = "127.0.0.1".parse().unwrap();
        assert!(Ipv4Loopback::new(ip).is_ok());
    }

    #[test]
    fn test_ipv4_loopback_not_loopback() {
        let ip = "192.168.1.1".parse().unwrap();
        assert!(Ipv4Loopback::new(ip).is_err());
    }

    #[test]
    fn test_ipv6_loopback_valid() {
        let ip = "::1".parse().unwrap();
        assert!(Ipv6Loopback::new(ip).is_ok());
    }

    #[test]
    fn test_ipv6_loopback_not_loopback() {
        let ip = "2001:db8::1".parse().unwrap();
        assert!(Ipv6Loopback::new(ip).is_err());
    }
}
