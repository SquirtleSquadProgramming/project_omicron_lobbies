#![allow(dead_code)]

use crate::{
    protocol::{Flags, IpAddress, Region},
    Serialise,
};
use bcrypt::{hash, DEFAULT_COST};
pub use in_memory::{create, dbg_database, delete, get, init, modify};

#[repr(u8)]
#[derive(Debug)]
pub enum DatabaseError {
    NotInitialised = 50,
    LobbyAlreadyExists = 51,
    LobbyDoesNotExist = 52,
    FailedToHashPassword = 53,
    FailedToVerifyPassword = 54,
    InvalidCredentials = 55,
    InvalidFilter = 56,
}

pub const PAGE_SIZE: u8 = 15;

pub struct Page {
    lobbies: Vec<Lobby>,
    page_number: u8,
    total_pages: u8,
}

impl Page {
    pub fn new(lobbies: Vec<Lobby>, page_number: u8, total_pages: u8) -> Self {
        Page {
            lobbies,
            page_number,
            total_pages,
        }
    }
}

impl Serialise for Page {
    fn serialise(self) -> Vec<u8> {
        let mut output = self
            .lobbies
            .iter()
            .map(|l| l)
            .collect::<Vec<_>>()
            .serialise();
        output.push(self.page_number);
        output.push(self.total_pages);
        output
    }
}

#[derive(Clone, Debug)]
pub struct Lobby {
    pub flags: Flags,
    pub region: Region,
    pub host_ip: IpAddress,
    pub host_port: u16,
    pub max_players: u8,
    pub lobby_name: String,
    pub password: String, // bcrypted!
    pub current_players: u8,
}

impl Serialise for &Lobby {
    fn serialise(self) -> Vec<u8> {
        let mut output = self.flags.clone().serialise();
        output.extend(self.region.clone().serialise());
        output.extend(self.host_ip.serialise());
        output.extend(self.host_port.serialise());
        output.push(self.max_players);
        output.extend(self.lobby_name.clone().serialise());
        output.push(self.current_players);
        output.insert(0, output.len() as u8);
        output
    }
}

impl PartialEq for Lobby {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags
            && self.region == other.region
            && self.host_ip == other.host_ip
            && self.host_port == other.host_port
            && self.max_players == other.max_players
            && self.lobby_name == other.lobby_name
            && self.current_players == other.current_players
    }
}

impl Lobby {
    pub fn new(
        flags: Flags,
        region: Region,
        host_ip: IpAddress,
        host_port: u16,
        max_players: u8,
        lobby_name: String,
        password: String,
    ) -> Option<Self> {
        let password = hash(password, DEFAULT_COST).ok()?;
        Some(Self {
            flags,
            region,
            host_ip,
            host_port,
            max_players,
            lobby_name,
            password,
            current_players: 1,
        })
    }

    pub fn set_player_count(&mut self, count: u8) {
        self.current_players = count;
    }
}

mod in_memory;
