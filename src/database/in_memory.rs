use super::{DatabaseError, Lobby, Page};
use crate::{
    database::PAGE_SIZE,
    protocol::{Filter, GetRequest, IpAddress},
};
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

pub fn get(request: GetRequest) -> Result<Page, DatabaseError> {
    if let Some(db) = unsafe { &mut DATABASE } {
        let response = match request {
            GetRequest::Standard((filter, regions, page_number)) => {
                let mut lobbies = db
                    .iter()
                    .filter(|&(_, lobby)| regions.contains(&lobby.region))
                    .map(|(_, lobby)| lobby)
                    .collect::<Vec<_>>();

                match filter {
                    Filter::NameAscending => lobbies.sort_by(|&left, &right| {
                        left.lobby_name
                            .to_lowercase()
                            .cmp(&right.lobby_name.to_lowercase())
                    }),
                    Filter::NameDescending => lobbies.sort_by(|&left, &right| {
                        right
                            .lobby_name
                            .to_lowercase()
                            .cmp(&left.lobby_name.to_lowercase())
                    }),
                    Filter::PlayerCountAscending => lobbies
                        .sort_by(|&left, &right| left.current_players.cmp(&right.current_players)),
                    Filter::PlayerCountDescending => lobbies
                        .sort_by(|&left, &right| right.current_players.cmp(&left.current_players)),
                    Filter::Search => Err(DatabaseError::InvalidFilter)?,
                }

                let num_lobbies = lobbies.len() as u8;
                let lobbies: Vec<_> = lobbies
                    .iter()
                    .skip((page_number * PAGE_SIZE) as usize)
                    .take(PAGE_SIZE as usize)
                    .map(|&lobby| lobby.clone())
                    .collect();

                Page::new(lobbies, page_number, num_lobbies / PAGE_SIZE)
            }
            GetRequest::Search((name, page_number)) => {
                let lobbies = db.iter().filter(|&(_, lobby)| {
                    lobby
                        .lobby_name
                        .to_lowercase()
                        .contains(&name.to_lowercase())
                });

                let num_lobbies = lobbies.clone().count() as u8;

                let lobbies: Vec<_> = lobbies
                    .skip((page_number * PAGE_SIZE) as usize)
                    .take(PAGE_SIZE as usize)
                    .map(|(_, lobby)| lobby.clone())
                    .collect();

                Page::new(lobbies, page_number, num_lobbies / PAGE_SIZE)
            }
        };

        Ok(response)
    } else {
        Err(DatabaseError::NotInitialised)
    }
}
