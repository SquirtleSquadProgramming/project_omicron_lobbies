#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug)]
pub enum ParseError {
    EmptyMessage,
    InvalidType(u8),
    MissingMessagePart,
    InvalidRegion,
    InvalidName,
    MismatchedIP,
}

#[derive(PartialEq, Eq, Clone)]
pub enum IpAddress {
    IpV4([u8; 4]),
    IpV6([u16; 8]),
}

impl IpAddress {
    fn from_message(msg: &mut std::slice::Iter<u8>, is_ipv6: bool) -> Result<Self, ParseError> {
        if is_ipv6 {
            let mut parts: [u16; 8] = [0; 8];

            for i in 0..8 {
                let part1 = *msg.next().ok_or(ParseError::MissingMessagePart)? as u16;
                let part2 = *msg.next().ok_or(ParseError::MissingMessagePart)? as u16;
                parts[i] = part1 << 8 + part2;
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
            IpAddress::IpV4(ip) => write!(f, "4/{}", ip.map(|n| n.to_string()).join(".")),
            IpAddress::IpV6(ip) => write!(f, "6/{}", ip.map(|n| format!("{n:x}")).join(":")),
        }
    }
}

mod version0;
use std::fmt::Display;

pub use version0::{Flags, Region};
