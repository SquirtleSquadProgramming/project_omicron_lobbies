#[derive(Debug)]
#[repr(u8)]
pub enum ParseError {
    EmptyMessage = 40,
    InvalidType = 41,
    MissingMessagePart = 42,
    InvalidRegion = 43,
    InvalidName = 44,
    MismatchedIP = 45,
    OutOfDate = 46,
    InvalidFilter = 47,
    // 50+ is reserved currently
}

#[derive(Debug, PartialEq)]
pub enum ParseOutput {
    Create(Option<Lobby>),
    Modify(Option<Lobby>),
    Destroy((IpAddress, u16, Option<String>)),
    Get(GetRequest),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum IpAddress {
    IpV4([u8; 4]),
    IpV6([u16; 8]),
}
impl Default for IpAddress {
    fn default() -> Self {
        Self::IpV4([0; 4])
    }
}

impl IpAddress {
    fn from_message(msg: &mut std::slice::Iter<u8>, is_ipv6: bool) -> Result<Self, ParseError> {
        if is_ipv6 {
            let mut parts: [u16; 8] = [0; 8];

            for i in 0..8 {
                let part1 = *msg.next().ok_or(ParseError::MissingMessagePart)? as u16;
                let part2 = *msg.next().ok_or(ParseError::MissingMessagePart)? as u16;
                parts[i] = (part1 << 8) + part2;
            }

            Ok(IpAddress::IpV6(parts))
        } else {
            let mut parts: [u8; 4] = [0; 4];

            for i in 0..4 {
                parts[i] = *msg.next().ok_or(ParseError::MissingMessagePart)?;
            }

            Ok(IpAddress::IpV4(parts))
        }
    }
}

impl Display for IpAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IpAddress::IpV4(ip) => write!(f, "{}/4", ip.map(|n| n.to_string()).join(".")),
            IpAddress::IpV6(ip) => write!(f, "{}/6", ip.map(|n| format!("{n:x}")).join(":")),
        }
    }
}

impl Into<IpAddress> for std::net::SocketAddr {
    fn into(self) -> IpAddress {
        fn to_u16(high: u8, low: u8) -> u16 {
            ((high as u16) << 8) | (low as u16)
        }

        match self {
            std::net::SocketAddr::V4(addr4) => IpAddress::IpV4(addr4.ip().octets()),
            std::net::SocketAddr::V6(addr6) => {
                let octets = addr6.ip().octets();
                IpAddress::IpV6([
                    to_u16(octets[0], octets[1]),
                    to_u16(octets[2], octets[3]),
                    to_u16(octets[4], octets[5]),
                    to_u16(octets[6], octets[7]),
                    to_u16(octets[8], octets[9]),
                    to_u16(octets[10], octets[11]),
                    to_u16(octets[12], octets[13]),
                    to_u16(octets[14], octets[15]),
                ])
            }
        }
    }
}

#[cfg(test)]
mod parse_tests;
mod version0;

use crate::database::Lobby;
use std::fmt::Display;
pub use version0::{parse_message, Filter, Flags, GetRequest, Region};
