#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug)]
pub enum ParseError {
    EmptyMessage,
    InvalidType(u8),
    MissingMessagePart,
    InvalidRegion,
    InvalidName,
}

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

mod version0;
pub use version0::{Flags, Region};
