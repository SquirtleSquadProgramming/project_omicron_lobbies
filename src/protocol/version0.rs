use super::{IpAddress, ParseError};
use crate::{
    database::{self, Lobby},
    ConvertError, Errors,
};

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
    Flags(Flags) = 0,
    IpAddr(IpAddress) = 1,
    Port(u16) = 2,
    Region(Region) = 3,
    MaxCount(u8) = 4,
    LName(String) = 5,
    LPass(String) = 6,
    Players(u8) = 7,
}

#[derive(Default, Clone)]
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
#[derive(Clone)]
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

pub fn parse_message(message: &[u8], ip_address: IpAddress) -> Result<(), Errors> {
    let m_type: u8 = message.get(0).ok_or(ParseError::EmptyMessage).convert()? >> 4;

    let typ: Types = m_type.into();

    match typ {
        Types::None => Err(ParseError::InvalidType(m_type)).convert(),
        Types::Create => {
            let lobby = parse_create_lobby(&message[1..], ip_address).convert()?;
            database::create(lobby).convert()
        }
        Types::Modify => parse_modify_lobby(&message[1..], ip_address).convert(),
        Types::Destroy => parse_destroy_lobby(&message[1..], ip_address).convert(),
    }
}

fn parse_create_lobby(message: &[u8], ip_address: IpAddress) -> Result<Option<Lobby>, ParseError> {
    let mut msg = message.iter();
    let flags: Flags = msg
        .next()
        .ok_or(ParseError::MissingMessagePart)?
        .to_owned()
        .into();
    let ip = IpAddress::from_message(&mut msg, flags.is_ipv6)?;
    if ip != ip_address {
        return Err(ParseError::MismatchedIP);
    }
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

    Ok(Lobby::new(
        flags,
        region,
        ip,
        port,
        max_players,
        lobby_name,
        lobby_password,
    ))
}

fn parse_modify_lobby(message: &[u8], ip_address: IpAddress) -> Result<(), ParseError> {
    todo!()
}

fn parse_destroy_lobby(message: &[u8], ip_address: IpAddress) -> Result<(), ParseError> {
    todo!()
}
