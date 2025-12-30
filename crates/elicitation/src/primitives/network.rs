//! Network type implementations for IP addresses and socket addresses.

use crate::{ElicitError, ElicitErrorKind, ElicitResult, Elicitation, Prompt};
use rmcp::service::{Peer, RoleClient};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

// IpAddr (enum: V4 | V6)
impl Prompt for IpAddr {
    fn prompt() -> Option<&'static str> {
        Some("Please enter an IP address (IPv4 or IPv6):")
    }
}

impl Elicitation for IpAddr {
    #[tracing::instrument(skip(client))]
    async fn elicit(
        client: &Peer<RoleClient>,
    ) -> ElicitResult<Self> {
        tracing::debug!("Eliciting IpAddr");

        let ip_str = String::elicit(client).await?;

        match ip_str.parse::<IpAddr>() {
            Ok(addr) => {
                tracing::debug!(addr = %addr, "Valid IP address");
                Ok(addr)
            }
            Err(e) => {
                tracing::warn!(error = ?e, input = %ip_str, "Invalid IP address format");
                Err(ElicitError::new(ElicitErrorKind::InvalidFormat {
                    expected: "valid IP address (IPv4 or IPv6)".to_string(),
                    received: ip_str,
                }))
            }
        }
    }
}

// Ipv4Addr
impl Prompt for Ipv4Addr {
    fn prompt() -> Option<&'static str> {
        Some("Please enter an IPv4 address (e.g., 192.168.1.1):")
    }
}

impl Elicitation for Ipv4Addr {
    #[tracing::instrument(skip(client))]
    async fn elicit(
        client: &Peer<RoleClient>,
    ) -> ElicitResult<Self> {
        tracing::debug!("Eliciting Ipv4Addr");

        let ip_str = String::elicit(client).await?;

        match ip_str.parse::<Ipv4Addr>() {
            Ok(addr) => {
                tracing::debug!(addr = %addr, "Valid IPv4 address");
                Ok(addr)
            }
            Err(e) => {
                tracing::warn!(error = ?e, input = %ip_str, "Invalid IPv4 address format");
                Err(ElicitError::new(ElicitErrorKind::InvalidFormat {
                    expected: "valid IPv4 address (e.g., 192.168.1.1)".to_string(),
                    received: ip_str,
                }))
            }
        }
    }
}

// Ipv6Addr
impl Prompt for Ipv6Addr {
    fn prompt() -> Option<&'static str> {
        Some("Please enter an IPv6 address (e.g., 2001:db8::1):")
    }
}

impl Elicitation for Ipv6Addr {
    #[tracing::instrument(skip(client))]
    async fn elicit(
        client: &Peer<RoleClient>,
    ) -> ElicitResult<Self> {
        tracing::debug!("Eliciting Ipv6Addr");

        let ip_str = String::elicit(client).await?;

        match ip_str.parse::<Ipv6Addr>() {
            Ok(addr) => {
                tracing::debug!(addr = %addr, "Valid IPv6 address");
                Ok(addr)
            }
            Err(e) => {
                tracing::warn!(error = ?e, input = %ip_str, "Invalid IPv6 address format");
                Err(ElicitError::new(ElicitErrorKind::InvalidFormat {
                    expected: "valid IPv6 address (e.g., 2001:db8::1)".to_string(),
                    received: ip_str,
                }))
            }
        }
    }
}

// SocketAddr (IpAddr + port)
impl Prompt for SocketAddr {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a socket address (e.g., 127.0.0.1:8080 or [::1]:8080):")
    }
}

impl Elicitation for SocketAddr {
    #[tracing::instrument(skip(client))]
    async fn elicit(
        client: &Peer<RoleClient>,
    ) -> ElicitResult<Self> {
        tracing::debug!("Eliciting SocketAddr");

        let addr_str = String::elicit(client).await?;

        match addr_str.parse::<SocketAddr>() {
            Ok(addr) => {
                tracing::debug!(addr = %addr, "Valid socket address");
                Ok(addr)
            }
            Err(e) => {
                tracing::warn!(error = ?e, input = %addr_str, "Invalid socket address format");
                Err(ElicitError::new(ElicitErrorKind::InvalidFormat {
                    expected: "valid socket address (e.g., 127.0.0.1:8080)".to_string(),
                    received: addr_str,
                }))
            }
        }
    }
}

// SocketAddrV4
impl Prompt for SocketAddrV4 {
    fn prompt() -> Option<&'static str> {
        Some("Please enter an IPv4 socket address (e.g., 192.168.1.1:8080):")
    }
}

impl Elicitation for SocketAddrV4 {
    #[tracing::instrument(skip(client))]
    async fn elicit(
        client: &Peer<RoleClient>,
    ) -> ElicitResult<Self> {
        tracing::debug!("Eliciting SocketAddrV4");

        let addr_str = String::elicit(client).await?;

        match addr_str.parse::<SocketAddrV4>() {
            Ok(addr) => {
                tracing::debug!(addr = %addr, "Valid IPv4 socket address");
                Ok(addr)
            }
            Err(e) => {
                tracing::warn!(error = ?e, input = %addr_str, "Invalid IPv4 socket address format");
                Err(ElicitError::new(ElicitErrorKind::InvalidFormat {
                    expected: "valid IPv4 socket address (e.g., 192.168.1.1:8080)".to_string(),
                    received: addr_str,
                }))
            }
        }
    }
}

// SocketAddrV6
impl Prompt for SocketAddrV6 {
    fn prompt() -> Option<&'static str> {
        Some("Please enter an IPv6 socket address (e.g., [2001:db8::1]:8080):")
    }
}

impl Elicitation for SocketAddrV6 {
    #[tracing::instrument(skip(client))]
    async fn elicit(
        client: &Peer<RoleClient>,
    ) -> ElicitResult<Self> {
        tracing::debug!("Eliciting SocketAddrV6");

        let addr_str = String::elicit(client).await?;

        match addr_str.parse::<SocketAddrV6>() {
            Ok(addr) => {
                tracing::debug!(addr = %addr, "Valid IPv6 socket address");
                Ok(addr)
            }
            Err(e) => {
                tracing::warn!(error = ?e, input = %addr_str, "Invalid IPv6 socket address format");
                Err(ElicitError::new(ElicitErrorKind::InvalidFormat {
                    expected: "valid IPv6 socket address (e.g., [2001:db8::1]:8080)".to_string(),
                    received: addr_str,
                }))
            }
        }
    }
}
