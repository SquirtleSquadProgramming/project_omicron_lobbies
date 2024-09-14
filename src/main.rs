use std::{
    io::{BufReader, Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    thread,
    time::{Duration, Instant},
};

mod database;
mod protocol;

macro_rules! etprintln {
    () => {
        etprintln!(as "[%Y/%m/%d-%H:%M:%S]")
    };
    (as $f:expr) => {
        eprintln!("{}", chrono::Local::now().format($f))
    };
    (as $f:expr; $($arg:tt)*) => {
        {
            eprint!("{} ", chrono::Local::now().format($f));
            eprintln!($( $arg )*)
        }
    };
    ($($arg:tt)*) => {
        etprintln!(as "[%Y/%m/%d-%H:%M:%S]"; $( $arg )*)
    };
}

const RECV_TIME_OUT: u64 = 5; // in seconds
const IP_ADDRESS: &'static str = "192.168.1.100:5475";

fn main() {
    database::init();
    let listener = TcpListener::bind(IP_ADDRESS).unwrap();
    etprintln!("Connected on {IP_ADDRESS}");

    for stream in listener.incoming() {
        etprintln!("Connection incoming.");
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(_) => etprintln!("Connection failed!"),
        }
    }
}

fn write_response(stream: &mut TcpStream, client_address: SocketAddr, response: u8) {
    if stream.write(&[response]).is_err() {
        etprintln!("Failed to write to stream. Client: {client_address} / Code: {response}");
    }
}

fn handle_connection(mut stream: TcpStream) {
    let client_address: SocketAddr = if let Ok(address) = stream.peer_addr() {
        address
    } else {
        etprintln!("Failed to get client address");
        return;
    };

    etprintln!("Recieved client ip: {client_address}");

    let mut thread_stream = stream.try_clone().expect("Failed to duplicate stream.");
    let recv_thread = thread::spawn(move || {
        let mut length: [u8; 1] = [0];
        if thread_stream.read_exact(&mut length).is_err() {
            etprintln!("Connection interupted: Failed to get message length.");
            return None;
        };

        let mut message = vec![0; length[0] as usize];
        if BufReader::new(&mut thread_stream)
            .read_exact(&mut message)
            .is_err()
        {
            etprintln!("Connection interupted: Failed to get message body.");
            return None;
        }

        Some(message)
    });

    let start = Instant::now();
    let message = loop {
        if recv_thread.is_finished() {
            let message = recv_thread.join().expect("Failed to join thread!");

            if message.is_none() {
                return;
            }

            break message.unwrap();
        }
        if start.elapsed() > Duration::from_secs(RECV_TIME_OUT) {
            etprintln!("Connection timed out.");
            write_response(&mut stream, client_address, 101);
            stream.shutdown(std::net::Shutdown::Both).unwrap();
            return;
        }
    };

    etprintln!("Recieved message: {message:?}");

    let parse_result = protocol::parse_message(message.as_slice(), client_address.into());
    if parse_result.is_err() {
        match parse_result {
            Err(parse_error) => {
                etprintln!("Bad message: {parse_error:?}");
                return write_response(&mut stream, client_address, parse_error as u8);
            }
            Ok(_) => (),
        }
    }
    let parse_output = parse_result.unwrap();
    let database_result = match parse_output {
        protocol::ParseOutput::Create(lobby) => database::create(lobby),
        protocol::ParseOutput::Modify(lobby) => database::modify(lobby),
        protocol::ParseOutput::Destroy((host_ip, port, password)) => {
            database::delete(host_ip, port, password)
        }
    };
    let response: u8 = match database_result {
        Err(err) => {
            etprintln!("Bad message: {err:?}");
            err as u8
        }
        Ok(()) => 10,
    };

    write_response(&mut stream, client_address, response);
    database::dbg_database();
}
