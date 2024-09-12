#![allow(dead_code)]

use bcrypt::{hash, DEFAULT_COST};
pub use in_memory::{create, delete, init, modify};

use crate::protocol::{Flags, IpAddress, Region};

pub struct Lobby {
    flags: Flags,
    region: Region,
    host_ip: IpAddress,
    host_port: u16,
    max_players: u8,
    lobby_name: String,
    password: String, // bcrypted!
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
        })
    }
}

mod in_memory;
