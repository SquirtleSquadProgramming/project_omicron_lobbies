#![allow(dead_code)]

use crate::protocol::{Flags, IpAddress, Region};
use bcrypt::{hash, DEFAULT_COST};
pub use in_memory::{create, delete, init, modify};

pub enum DatabaseError {
    NotInitialised,
    LobbyAlreadyExists,
    LobbyDoesNotExist,
    FailedToHashPassword,
    FailedToVerifyPassword,
    InvalidCredentials,
}

#[derive(Clone, Debug)]
pub struct Lobby {
    flags: Flags,
    region: Region,
    host_ip: IpAddress,
    host_port: u16,
    max_players: u8,
    lobby_name: String,
    password: String, // bcrypted!
    current_players: u8,
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
