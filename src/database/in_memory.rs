use super::{DatabaseError, Lobby};
use crate::protocol::IpAddress;
use bcrypt::verify;
use std::collections::HashMap;

static mut DATABASE: Option<HashMap<String, Lobby>> = None;

fn make_key(ip: IpAddress, port: u16) -> String {
    format!("{ip}:{port}")
}

pub fn dbg_database() {
    dbg!(unsafe { &DATABASE });
}

pub fn init() {
    let db = unsafe { &mut DATABASE };
    *db = Some(HashMap::new());
}

pub fn create(lobby: Option<Lobby>) -> Result<(), DatabaseError> {
    if lobby.is_none() {
        return Err(DatabaseError::FailedToHashPassword);
    }

    if let Some(db) = unsafe { &mut DATABASE } {
        let lobby = lobby.unwrap();
        let key = make_key(lobby.host_ip, lobby.host_port);

        if db.contains_key(&key) {
            return Err(DatabaseError::LobbyAlreadyExists);
        }

        db.insert(key, lobby);
        Ok(())
    } else {
        Err(DatabaseError::NotInitialised)
    }
}

pub fn modify(lobby: Option<Lobby>) -> Result<(), DatabaseError> {
    if lobby.is_none() {
        return Err(DatabaseError::FailedToHashPassword);
    }

    if let Some(db) = unsafe { &mut DATABASE } {
        let lobby = lobby.unwrap();
        let key = make_key(lobby.host_ip, lobby.host_port);

        if !db.contains_key(&key) {
            return Err(DatabaseError::LobbyDoesNotExist);
        }

        db.insert(key, lobby);
        Ok(())
    } else {
        Err(DatabaseError::NotInitialised)
    }
}

pub fn delete(
    host_ip: IpAddress,
    port: u16,
    password: Option<String>,
) -> Result<(), DatabaseError> {
    if let Some(db) = unsafe { &mut DATABASE } {
        let key = make_key(host_ip, port);
        let lobby = db.get(&key).ok_or(DatabaseError::LobbyDoesNotExist)?;

        if let Some(password) = password {
            let password_check = verify(password, &lobby.password)
                .map_err(|_| DatabaseError::FailedToVerifyPassword)?;
            if !password_check {
                return Err(DatabaseError::InvalidCredentials);
            }
        }

        if db.remove(&key).is_some() {
            Ok(())
        } else {
            Err(DatabaseError::LobbyDoesNotExist)
        }
    } else {
        Err(DatabaseError::NotInitialised)
    }
}
