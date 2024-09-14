use super::*;

#[cfg(test)]
fn basic_lobby_message(typ: u8) -> Vec<u8> {
    let type_version = (typ << 4) + 0x0; // CREATE | V0
    let flags = 0b100; // Has password
    let ip_address = [192, 168, 1, 111];
    let (port_high, port_low) = {
        let port = 25565;
        ((port >> 8) as u8, (port & 0xFF) as u8)
    };
    let region = 5; // Oceania
    let max_players = 10;
    let (lobby_name_size, lobby_name_bytes) = {
        let lobby_name = String::from("Test Lobby!");
        (
            lobby_name.len() as u8,
            lobby_name.bytes().collect::<Vec<_>>(),
        )
    };
    let (pass_size, pass_bytes) = {
        let password = String::from("password123");
        (password.len() as u8, password.bytes().collect::<Vec<_>>())
    };

    let mut message = Vec::new();
    message.push(type_version);
    message.push(flags);
    message.extend(ip_address);
    message.push(port_high);
    message.push(port_low);
    message.push(region);
    message.push(max_players);
    message.push(lobby_name_size);
    message.extend(lobby_name_bytes);
    message.push(pass_size);
    message.extend(pass_bytes);
    message
}

#[test]
fn create() {
    let message = basic_lobby_message(0b1);

    let expected_lobby = Lobby::new(
        Flags::new(false, false, true),
        Region::Oceania,
        IpAddress::IpV4([192, 168, 1, 111]),
        25565,
        10,
        String::from("Test Lobby!"),
        String::from("password123"),
    )
    .unwrap();

    let parsed = parse_message(message.as_slice(), IpAddress::IpV4([192, 168, 1, 111]));
    match parsed.unwrap() {
        ParseOutput::Create(lobby) => {
            assert!(lobby.is_some());
            let lobby = lobby.unwrap();
            assert_eq!(expected_lobby, lobby);
        }
        _ => panic!("Incorrect protocol type."),
    }
}

#[test]
fn modify() {
    let mut message = basic_lobby_message(0b10);
    message.push(25);

    let mut expected_lobby = Lobby::new(
        Flags::new(false, false, true),
        Region::Oceania,
        IpAddress::IpV4([192, 168, 1, 111]),
        25565,
        10,
        String::from("Test Lobby!"),
        String::from("password123"),
    )
    .unwrap();

    expected_lobby.set_player_count(25);

    let parsed = parse_message(message.as_slice(), IpAddress::IpV4([192, 168, 1, 111]));
    match parsed.unwrap() {
        ParseOutput::Modify(lobby) => {
            assert!(lobby.is_some());
            let lobby = lobby.unwrap();
            assert_eq!(expected_lobby, lobby);
        }
        _ => panic!("Incorrect protocol type."),
    }
}

#[test]
fn destory() {
    let type_version = (0b100 << 4) + 0x0; // CREATE | V0
    let ip_address = [192, 168, 1, 111];
    let port = {
        let port = 25565;
        [(port >> 8) as u8, (port & 0xFF) as u8]
    };
    let mut message1 = Vec::new();
    message1.push(type_version);
    message1.push(false as u8);
    message1.extend(ip_address);
    message1.extend(port);
    let mut message2 = message1.clone();
    let (pass_size, pass_bytes) = {
        let password = String::from("password123");
        (password.len() as u8, password.bytes().collect::<Vec<_>>())
    };
    message2.push(pass_size);
    message2.extend(pass_bytes);

    let expected1 = (IpAddress::IpV4(ip_address), 25565, None);
    let expected2 = (
        IpAddress::IpV4(ip_address),
        25565,
        Some(String::from("password123")),
    );

    let parsed = parse_message(message1.as_slice(), IpAddress::IpV4(ip_address));
    assert_eq!(parsed.unwrap(), ParseOutput::Destroy(expected1));

    let parsed = parse_message(message2.as_slice(), IpAddress::IpV4(ip_address));
    assert_eq!(parsed.unwrap(), ParseOutput::Destroy(expected2));
}
