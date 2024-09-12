use super::{IpAddress, ParseError};

const VERSION: u8 = 0;
const MAX_LOBBY_NAME_SIZE: usize = 32;
const MAX_LOBBY_PASS_SIZE: usize = 32;

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

#[repr(u8)]
pub enum Region {
    Africa,
    Asia,
    Europe,
    NorthAmerica,
    SouthAmerica,
    Oceania,
}

impl TryInto<Region> for u8 {
    type Error = ParseError;

    fn try_into(self) -> Result<Region, Self::Error> {
        let region = match self {
            0 => Region::Africa,
            1 => Region::Asia,
            2 => Region::Europe,
            3 => Region::NorthAmerica,
            4 => Region::SouthAmerica,
            5 => Region::Oceania,
            _ => Err(ParseError::InvalidRegion)?,
        };

        Ok(region)
    }
}

fn deserialise_string(
    message: &mut std::slice::Iter<u8>,
    max_length: usize,
) -> Result<String, ParseError> {
    let length = *message.next().ok_or(ParseError::MissingMessagePart)? as usize;
    if length > max_length {
        return Err(ParseError::InvalidName);
    }

    let mut lobby_name = String::with_capacity(length);

    for _ in 0..length {
        let ch = *message.next().ok_or(ParseError::MissingMessagePart)? as char;
        lobby_name.push(ch);
    }

    Ok(lobby_name)
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

fn create_lobby(message: &[u8], ip_address: IpAddress) -> Result<(), ParseError> {
    let mut msg = message.iter();
    let flags: Flags = msg
        .next()
        .ok_or(ParseError::MissingMessagePart)?
        .to_owned()
        .into();
    let ip = IpAddress::from_message(&mut msg, flags.is_ipv6)?;
    let port = {
        let high = *msg.next().ok_or(ParseError::MissingMessagePart)? as u16;
        let low = *msg.next().ok_or(ParseError::MissingMessagePart)? as u16;
        high << 8 + low
    };
    let region: Region = msg
        .next()
        .ok_or(ParseError::MissingMessagePart)?
        .to_owned()
        .try_into()?;
    let max_players: u8 = *msg.next().ok_or(ParseError::MissingMessagePart)?;
    let lobby_name: String = deserialise_string(&mut msg, MAX_LOBBY_NAME_SIZE)?;
    let lobby_password: String = deserialise_string(&mut msg, MAX_LOBBY_PASS_SIZE)?;

    todo!()
}

fn modify_lobby(message: &[u8], ip_address: IpAddress) -> Result<(), ParseError> {
    todo!()
}

fn destroy_lobby(message: &[u8], ip_address: IpAddress) -> Result<(), ParseError> {
    todo!()
}
