use bcrypt::DEFAULT_COST;

use super::{Lobby, ModifyLobby};
use crate::protocol::IpAddress;
use std::collections::HashMap;

static mut DATABASE: Option<HashMap<String, Lobby>> = None;

pub fn init() {
    let db = unsafe { &mut DATABASE };
    *db = Some(HashMap::new());
    println!("Initialised database.");
}

pub fn create(lobby: Lobby) -> Result<(), ()> {
    if let Some(db) = unsafe { &mut DATABASE } {
        let key = lobby.host_ip.to_string();
        if db.contains_key(&key) {
            return Err(());
        }

        db.insert(key, lobby);
    }
    Ok(())
}

pub fn modify(host_ip: IpAddress, modify_lobby: ModifyLobby) -> Result<(), ()> {
    if let Some(db) = unsafe { &mut DATABASE } {
        let key = host_ip.to_string();
        let mut lobby = db.get(&key).ok_or(())?.clone();

        if let Some(flags) = modify_lobby.flags {
            lobby.flags = flags;
        }

        if let Some(region) = modify_lobby.region {
            lobby.region = region;
        }

        if let Some(host_port) = modify_lobby.host_port {
            lobby.host_port = host_port;
        }

        if let Some(max_players) = modify_lobby.max_players {
            lobby.max_players = max_players;
        }

        if let Some(lobby_name) = modify_lobby.lobby_name {
            lobby.lobby_name = lobby_name;
        }

        if let Some(password) = modify_lobby.password {
            let password = bcrypt::hash(password, DEFAULT_COST).ok().ok_or(())?;
            lobby.password = password;
        }

        let key = if let Some(host_ip) = modify_lobby.host_ip {
            db.remove(&key);
            lobby.host_ip = host_ip.clone();
            host_ip.to_string()
        } else {
            key
        };

        db.insert(key, lobby);
    }
    Ok(())
}

pub fn delete(host_ip: IpAddress) -> Result<(), ()> {
    if let Some(db) = unsafe { &mut DATABASE } {
        let key = host_ip.to_string();

        if db.remove(&key).is_some() {
            return Ok(());
        }

        return Err(());
    } else {
        Err(())
    }
}
