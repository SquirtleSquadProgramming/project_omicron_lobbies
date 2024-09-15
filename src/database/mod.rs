#![allow(dead_code)]

use crate::protocol::{Flags, IpAddress, Region};
use bcrypt::{hash, DEFAULT_COST};
pub use in_memory::{create, dbg_database, delete, init, modify};

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

pub const PAGE_SIZE: usize = 15;

pub struct Page {
    lobbies: Vec<Lobby>,
    page_number: usize,
    total_pages: usize,
}

impl Page {
    pub fn new(lobbies: Vec<Lobby>, page_number: usize, total_pages: usize) -> Self {
        Page {
            lobbies,
            page_number,
            total_pages,
        }
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
