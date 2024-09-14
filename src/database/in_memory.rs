use super::{DatabaseError, Lobby};
use crate::protocol::IpAddress;
use bcrypt::verify;
use std::collections::HashMap;

static mut DATABASE: Option<HashMap<String, Lobby>> = None;

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

pub fn modify(lobby: Option<Lobby>) -> Result<(), DatabaseError> {
    if lobby.is_none() {
        return Err(DatabaseError::FailedToHashPassword);
    }

    if let Some(db) = unsafe { &mut DATABASE } {
        let lobby = lobby.unwrap();
        let key = lobby.host_ip.to_string();

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
        let key = host_ip.to_string();
        let lobby = db.get(&key).ok_or(DatabaseError::LobbyDoesNotExist)?;

        let is_valid = if let Some(password) = password {
            let password_check = verify(password, &lobby.password)
                .map_err(|_| DatabaseError::FailedToVerifyPassword)?;
            lobby.host_port == port && password_check
        } else {
            lobby.host_port == port
        };

        if !is_valid {
            return Err(DatabaseError::InvalidCredentials);
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
