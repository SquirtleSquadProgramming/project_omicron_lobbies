use super::{DatabaseError, Lobby};
use crate::protocol::{FieldType, IpAddress};
use bcrypt::DEFAULT_COST;
use std::collections::HashMap;

static mut DATABASE: Option<HashMap<String, Lobby>> = None;

pub fn init() {
    let db = unsafe { &mut DATABASE };
    *db = Some(HashMap::new());
    println!("Initialised database.");
}

pub fn create(lobby: Option<Lobby>) -> Result<(), DatabaseError> {
    if lobby.is_none() {
        return Err(DatabaseError::FailedToHashPassword);
    }
    if let Some(db) = unsafe { &mut DATABASE } {
        let lobby = lobby.unwrap();
        let key = lobby.host_ip.to_string();
        if db.contains_key(&key) {
            return Err(DatabaseError::LobbyAlreadyExists);
        }

        db.insert(key, lobby);
        Ok(())
    } else {
        Err(DatabaseError::NotInitialised)
    }
}

pub fn modify(host_ip: IpAddress, modify_lobby: Vec<FieldType>) -> Result<(), DatabaseError> {
    if let Some(db) = unsafe { &mut DATABASE } {
        let mut key = host_ip.to_string();
        let mut lobby = db
            .get(&key)
            .ok_or(DatabaseError::LobbyDoesNotExist)?
            .clone();

        for change in modify_lobby.iter() {
            match change {
                FieldType::Flags(value) => lobby.flags = value.clone(),
                FieldType::IpAddr(value) => {
                    db.remove(&key);
                    lobby.host_ip = value.clone();
                    key = value.to_string();
                }
                FieldType::Port(value) => lobby.host_port = value.clone(),
                FieldType::Region(value) => lobby.region = value.clone(),
                FieldType::MaxCount(value) => lobby.max_players = value.clone(),
                FieldType::LName(value) => lobby.lobby_name = value.clone(),
                FieldType::LPass(value) => {
                    let password = bcrypt::hash(value.clone(), DEFAULT_COST)
                        .ok()
                        .ok_or(DatabaseError::FailedToHashPassword)?;
                    lobby.password = password;
                }
                FieldType::Players(value) => lobby.current_players = value.clone(),
            }
        }

        db.insert(key, lobby);
        Ok(())
    } else {
        Err(DatabaseError::NotInitialised)
    }
}

pub fn delete(host_ip: IpAddress) -> Result<(), DatabaseError> {
    if let Some(db) = unsafe { &mut DATABASE } {
        let key = host_ip.to_string();

        if db.remove(&key).is_some() {
            Ok(())
        } else {
            Err(DatabaseError::LobbyDoesNotExist)
        }
    } else {
        Err(DatabaseError::NotInitialised)
    }
}
