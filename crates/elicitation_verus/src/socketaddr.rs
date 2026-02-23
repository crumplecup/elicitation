use verus_builtin_macros::verus;

verus! {

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    InvalidPort,
    InvalidAddress,
}

// ============================================================================
// SocketAddrV4Bytes - IPv4 socket address
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SocketAddrV4Bytes {
    pub octets: [u8; 4],
    pub port: u16,
    pub validated: bool,
}

impl SocketAddrV4Bytes {
    /// All combinations of IPv4 address and port are valid.
    pub fn new(octets: [u8; 4], port: u16) -> (result: Self)
        ensures
            result.octets == octets,
            result.port == port,
            result.validated == true,
    {
        SocketAddrV4Bytes { octets, port, validated: true }
    }

    pub fn is_privileged_port(&self) -> (result: bool)
        requires self.validated == true,
    {
        self.port < 1024
    }

    pub fn is_well_known_port(&self) -> (result: bool)
        requires self.validated == true,
    {
        self.port < 1024
    }

    pub fn is_registered_port(&self) -> (result: bool)
        requires self.validated == true,
    {
        self.port >= 1024 && self.port <= 49151
    }

    pub fn is_dynamic_port(&self) -> (result: bool)
        requires self.validated == true,
    {
        self.port >= 49152
    }
}

// ============================================================================
// SocketAddrV6Bytes - IPv6 socket address
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SocketAddrV6Bytes {
    pub segments: [u16; 8],
    pub port: u16,
    pub validated: bool,
}

impl SocketAddrV6Bytes {
    /// All combinations of IPv6 address and port are valid.
    pub fn new(segments: [u16; 8], port: u16) -> (result: Self)
        ensures
            result.segments == segments,
            result.port == port,
            result.validated == true,
    {
        SocketAddrV6Bytes { segments, port, validated: true }
    }

    pub fn is_privileged_port(&self) -> (result: bool)
        requires self.validated == true,
    {
        self.port < 1024
    }

    pub fn is_well_known_port(&self) -> (result: bool)
        requires self.validated == true,
    {
        self.port < 1024
    }

    pub fn is_registered_port(&self) -> (result: bool)
        requires self.validated == true,
    {
        self.port >= 1024 && self.port <= 49151
    }

    pub fn is_dynamic_port(&self) -> (result: bool)
        requires self.validated == true,
    {
        self.port >= 49152
    }
}

// ============================================================================
// SocketAddrV4NonZero - IPv4 socket with non-zero port
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SocketAddrV4NonZero {
    pub octets: [u8; 4],
    pub port: u16,
    pub validated: bool,
}

impl SocketAddrV4NonZero {
    /// Parameters:
    /// - is_nonzero_port: port != 0
    pub fn new(octets: [u8; 4], port: u16, is_nonzero_port: bool) -> (result: Result<Self, ValidationError>)
        ensures
            is_nonzero_port ==> (result matches Ok(sa) && sa.octets == octets && sa.port == port && sa.validated == true),
            !is_nonzero_port ==> (result matches Err(ValidationError::InvalidPort)),
    {
        if is_nonzero_port {
            Ok(SocketAddrV4NonZero { octets, port, validated: true })
        } else {
            Err(ValidationError::InvalidPort)
        }
    }
}

// ============================================================================
// SocketAddrV4Privileged - IPv4 socket with privileged port (< 1024)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SocketAddrV4Privileged {
    pub octets: [u8; 4],
    pub port: u16,
    pub validated: bool,
}

impl SocketAddrV4Privileged {
    /// Parameters:
    /// - is_privileged: port < 1024
    pub fn new(octets: [u8; 4], port: u16, is_privileged: bool) -> (result: Result<Self, ValidationError>)
        ensures
            is_privileged ==> (result matches Ok(sa) && sa.octets == octets && sa.port == port && sa.validated == true),
            !is_privileged ==> (result matches Err(ValidationError::InvalidPort)),
    {
        if is_privileged {
            Ok(SocketAddrV4Privileged { octets, port, validated: true })
        } else {
            Err(ValidationError::InvalidPort)
        }
    }
}

} // verus!
