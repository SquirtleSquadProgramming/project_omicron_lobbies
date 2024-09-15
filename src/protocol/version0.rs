use super::{IpAddress, ParseError, ParseOutput};
use crate::database::Lobby;

const VERSION: u8 = 0;
const MAX_LOBBY_NAME_SIZE: usize = 32;
const MAX_LOBBY_PASS_SIZE: usize = 32;

type IterU8<'a> = std::slice::Iter<'a, u8>;

#[repr(u8)]
pub enum Types {
    None = 0x0,
    Create = 0x1,
    Modify = 0x2,
    Destroy = 0x4,
    Get = 0x8,
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

#[derive(Default, PartialEq, Debug, Clone)]
pub struct Flags {
    is_ipv6: bool,
    is_public: bool,
    has_password: bool,
}

#[cfg(test)]
impl Flags {
    pub fn new(is_ipv6: bool, is_public: bool, has_password: bool) -> Self {
        Self {
            is_ipv6,
            is_public,
            has_password,
        }
    }
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
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub enum Region {
    #[default]
    Africa = 1,
    Asia = 2,
    Europe = 4,
    NorthAmerica = 8,
    SouthAmerica = 16,
    Oceania = 32,
}

impl TryInto<Region> for u8 {
    type Error = ParseError;

    fn try_into(self) -> Result<Region, Self::Error> {
        let region = match self {
            1 => Region::Africa,
            2 => Region::Asia,
            4 => Region::Europe,
            8 => Region::NorthAmerica,
            16 => Region::SouthAmerica,
            32 => Region::Oceania,
            _ => Err(ParseError::InvalidRegion)?,
        };

        Ok(region)
    }
}

impl Region {
    pub fn get_regions(value: u8) -> Vec<Region> {
        let mut output = Vec::new();

        if value & 1 == 1 {
            output.push(Region::Africa);
        }
        if value & 2 == 2 {
            output.push(Region::Asia);
        }
        if value & 4 == 4 {
            output.push(Region::Europe);
        }
        if value & 8 == 8 {
            output.push(Region::NorthAmerica);
        }
        if value & 16 == 16 {
            output.push(Region::SouthAmerica);
        }
        if value & 32 == 32 {
            output.push(Region::Oceania);
        }

        if output.is_empty() {
            output = vec![
                Region::Africa,
                Region::Asia,
                Region::Europe,
                Region::NorthAmerica,
                Region::SouthAmerica,
                Region::Oceania,
            ];
        }

        output
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum GetRequest {
    Standard((Filter, Vec<Region>)),
    Search(String),
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
pub enum Filter {
    NameAscending = 0,
    NameDescending = 1,
    PlayerCountAscending = 2,
    PlayerCountDescending = 3,
    Search = 255,
}

impl TryInto<Filter> for u8 {
    type Error = ParseError;

    fn try_into(self) -> Result<Filter, Self::Error> {
        let filter = match self {
            0 => Filter::NameAscending,
            1 => Filter::NameDescending,
            2 => Filter::PlayerCountAscending,
            3 => Filter::PlayerCountDescending,
            255 => Filter::Search,
            _ => Err(ParseError::InvalidFilter)?,
        };

        Ok(filter)
    }
}

fn deserialise_string(
    message: &mut IterU8,
    max_length: usize,
) -> Result<Option<String>, ParseError> {
    let length = match message.next() {
        Some(length) => *length as usize,
        None => return Ok(None),
    };

    if length > max_length {
        return Err(ParseError::InvalidName);
    }

    let mut lobby_name = String::with_capacity(length);

    for _ in 0..length {
        let ch = *message.next().ok_or(ParseError::MissingMessagePart)? as char;
        lobby_name.push(ch);
    }

    Ok(Some(lobby_name))
}

pub fn parse_message(message: &[u8], ip_address: IpAddress) -> Result<ParseOutput, ParseError> {
    let m_type: u8 = *message.get(0).ok_or(ParseError::EmptyMessage)?;

    let version: u8 = m_type & 0xF;
    if version != VERSION {
        return Err(ParseError::OutOfDate);
    }
    let typ: Types = (m_type >> 4).into();
    let mut msg = message[1..].iter();

    match typ {
        Types::None => Err(ParseError::InvalidType),
        Types::Create => {
            parse_create_lobby(&mut msg, ip_address).map(|lobby| ParseOutput::Create(lobby))
        }
        Types::Modify => {
            parse_modify_lobby(&mut msg, ip_address).map(|lobby| ParseOutput::Modify(lobby))
        }
        Types::Destroy => {
            parse_destroy_lobby(&mut msg, ip_address).map(|del_info| ParseOutput::Destroy(del_info))
        }
        Types::Get => parse_get(&mut msg, ip_address).map(|get| ParseOutput::Get(get)),
    }
}

fn parse_create_lobby(
    message: &mut IterU8,
    ip_address: IpAddress,
) -> Result<Option<Lobby>, ParseError> {
    let flags: Flags = message
        .next()
        .ok_or(ParseError::MissingMessagePart)?
        .to_owned()
        .into();

    let ip = IpAddress::from_message(message, flags.is_ipv6)?;
    if ip != ip_address {
        return Err(ParseError::MismatchedIP);
    }

    let port = {
        let high = *message.next().ok_or(ParseError::MissingMessagePart)? as u16;
        let low = *message.next().ok_or(ParseError::MissingMessagePart)? as u16;
        (high << 8) + low
    };

    let region: Region = message
        .next()
        .ok_or(ParseError::MissingMessagePart)?
        .to_owned()
        .try_into()?;

    let max_players: u8 = *message.next().ok_or(ParseError::MissingMessagePart)?;
    let lobby_name: String =
        deserialise_string(message, MAX_LOBBY_NAME_SIZE)?.ok_or(ParseError::MissingMessagePart)?;
    let lobby_password: String =
        deserialise_string(message, MAX_LOBBY_PASS_SIZE)?.ok_or(ParseError::MissingMessagePart)?;

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

fn parse_modify_lobby(
    message: &mut IterU8,
    ip_address: IpAddress,
) -> Result<Option<Lobby>, ParseError> {
    if let Some(mut lobby) = parse_create_lobby(message, ip_address)? {
        lobby.set_player_count(*message.next().ok_or(ParseError::MissingMessagePart)?);
        Ok(Some(lobby))
    } else {
        Ok(None)
    }
}

fn parse_destroy_lobby(
    message: &mut IterU8,
    ip_address: IpAddress,
) -> Result<(IpAddress, u16, Option<String>), ParseError> {
    let is_ipv6 = message.next().ok_or(ParseError::MissingMessagePart)? == &1;
    let ip = IpAddress::from_message(message, is_ipv6)?;

    if ip != ip_address {
        return Err(ParseError::MismatchedIP);
    }

    let port = {
        let high = *message.next().ok_or(ParseError::MissingMessagePart)? as u16;
        let low = *message.next().ok_or(ParseError::MissingMessagePart)? as u16;
        (high << 8) + low
    };

    let password = deserialise_string(message, MAX_LOBBY_PASS_SIZE)?;

    Ok((ip, port, password))
}

fn parse_get(message: &mut IterU8, ip_address: IpAddress) -> Result<GetRequest, ParseError> {
    todo!()
}
