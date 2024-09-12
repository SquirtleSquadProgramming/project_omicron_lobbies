use super::Lobby;
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

pub fn modify(host_ip: IpAddress) -> Result<(), ()> {
    if let Some(db) = unsafe { &mut DATABASE } {
        let key = host_ip.to_string();
        let lobby = db.get(&key).ok_or(())?;
        // modfiy lobby
        let lobby = Lobby { ..lobby.clone() };
        db.insert(key, lobby);
    }
    Ok(())
}

pub fn delete() -> Result<(), ()> {
    todo!()
}
