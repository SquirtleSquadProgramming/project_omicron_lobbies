#![allow(dead_code)]

#[derive(Debug)]
pub enum ParseError {
    EmptyMessage,
    InvalidType(u8),
    MissingMessagePart,
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

mod version0 {
    use super::{IpAddress, ParseError};

    const VERSION: u8 = 0;

    #[repr(u8)]
    pub enum Types {
        None = 0x0,
        Create = 0x1,
        Modify = 0x2,
        Destroy = 0x4,
    }

    impl From<u8> for Types {
        fn from(value: u8) -> Self {
            match value {
                0x1 => Self::Create,
                0x2 => Self::Modify,
                0x4 => Self::Destroy,
                _ => Self::None,
            }
        }
    }

    impl From<Types> for u8 {
        fn from(value: Types) -> Self {
            value as u8
        }
    }

    #[repr(u8)]
    pub enum FieldType {
        Flags = 0,
        IpAddr = 1,
        Port = 2,
        Region = 3,
        MaxCount = 4,
        LName = 5,
        LPass = 6,
        Players = 7,
    }

    #[derive(Default)]
    pub struct Flags {
        is_ipv6: bool,
        is_public: bool,
        has_password: bool,
    }

    impl Into<Flags> for u8 {
        fn into(self) -> Flags {
            let mut flags = Flags::default();
            flags.is_ipv6 = self & 0x1 != 0;
            flags.is_public = self & 0x2 != 0;
            flags.has_password = self & 0x4 != 0;
            flags
        }
    }

    pub fn parse_message(message: &[u8], ip_address: IpAddress) -> Result<(), ParseError> {
        let m_type: u8 = message.get(0).ok_or(ParseError::EmptyMessage)? >> 4;

        let typ: Types = m_type.into();

        match typ {
            Types::None => Err(ParseError::InvalidType(m_type)),
            Types::Create => create_lobby(&message[1..], ip_address),
            Types::Modify => modify_lobby(&message[1..], ip_address),
            Types::Destroy => destroy_lobby(&message[1..], ip_address),
        }
    }

    pub fn create_lobby(message: &[u8], ip_address: IpAddress) -> Result<(), ParseError> {
        let mut msg = message.iter();
        let flags: Flags = (*msg.next().ok_or(ParseError::MissingMessagePart)?).into();
        let ip = IpAddress::from_message(&mut msg, flags.is_ipv6)?;
        let port = {
            let high = *msg.next().ok_or(ParseError::MissingMessagePart)? as u16;
            let low = *msg.next().ok_or(ParseError::MissingMessagePart)? as u16;
            high << 8 + low
        };

        todo!()
    }

    pub fn modify_lobby(message: &[u8], ip_address: IpAddress) -> Result<(), ParseError> {
        todo!()
    }

    pub fn destroy_lobby(message: &[u8], ip_address: IpAddress) -> Result<(), ParseError> {
        todo!()
    }
}
