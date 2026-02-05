//! Example demonstrating network type elicitation.
//!
//! **Note**: This example requires an MCP client (Claude Desktop or Claude CLI).
//! Run with: `claude "Run the network example"`
//!
//! This example shows how to elicit:
//! - IpAddr - IPv4 or IPv6 addresses
//! - Ipv4Addr - Specific IPv4 addresses
//! - Ipv6Addr - Specific IPv6 addresses  
//! - SocketAddr - IP address + port combinations
//! - SocketAddrV4 - IPv4 socket addresses
//! - SocketAddrV6 - IPv6 socket addresses
use std::sync::Arc;

use elicitation::{ElicitClient, ElicitResult, Elicitation};
use rmcp::ServiceExt;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

#[tokio::main]
async fn main() -> ElicitResult<()> {
    // Initialize tracing for observability
    tracing_subscriber::fmt()
        .with_env_filter("network=debug,elicitation=debug")
        .init();

    tracing::info!("Starting network type elicitation example");

    // Create MCP client with stdio transport

    let service = ().serve(rmcp::transport::stdio()).await.expect("Failed to create MCP client");
    let peer = service.peer();
    let client = ElicitClient::new(Arc::new(peer.clone()));

    // Elicit a generic IP address (can be v4 or v6)
    tracing::info!("=== Eliciting IpAddr ===");
    let ip: IpAddr = IpAddr::elicit(&client).await?;
    tracing::info!(?ip, "Elicited IP address");
    println!("IP address: {}", ip);

    // Elicit a specific IPv4 address
    tracing::info!("=== Eliciting Ipv4Addr ===");
    let ipv4: Ipv4Addr = Ipv4Addr::elicit(&client).await?;
    tracing::info!(?ipv4, "Elicited IPv4 address");
    println!("IPv4 address: {}", ipv4);

    // Elicit a specific IPv6 address
    tracing::info!("=== Eliciting Ipv6Addr ===");
    let ipv6: Ipv6Addr = Ipv6Addr::elicit(&client).await?;
    tracing::info!(?ipv6, "Elicited IPv6 address");
    println!("IPv6 address: {}", ipv6);

    // Elicit a socket address (IP + port)
    tracing::info!("=== Eliciting SocketAddr ===");
    let socket: SocketAddr = SocketAddr::elicit(&client).await?;
    tracing::info!(?socket, "Elicited socket address");
    println!("Socket address: {}", socket);

    // Elicit a specific IPv4 socket address
    tracing::info!("=== Eliciting SocketAddrV4 ===");
    let socket_v4: SocketAddrV4 = SocketAddrV4::elicit(&client).await?;
    tracing::info!(?socket_v4, "Elicited IPv4 socket address");
    println!("IPv4 socket address: {}", socket_v4);

    // Elicit a specific IPv6 socket address
    tracing::info!("=== Eliciting SocketAddrV6 ===");
    let socket_v6: SocketAddrV6 = SocketAddrV6::elicit(&client).await?;
    tracing::info!(?socket_v6, "Elicited IPv6 socket address");
    println!("IPv6 socket address: {}", socket_v6);

    // Optional network addresses
    tracing::info!("=== Eliciting optional IP address ===");
    let optional_ip: Option<IpAddr> = Option::<IpAddr>::elicit(&client).await?;
    tracing::info!(?optional_ip, "Elicited optional IP");
    match optional_ip {
        Some(ip) => println!("Optional IP: {}", ip),
        None => println!("No IP address provided"),
    }

    tracing::info!("Example complete!");
    Ok(())
}
