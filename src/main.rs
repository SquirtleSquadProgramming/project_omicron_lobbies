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

pub trait Serialise {
    fn serialise(self) -> Vec<u8>;
}

impl Serialise for String {
    fn serialise(self) -> Vec<u8> {
        let mut output = Vec::new();
        output.push(self.len() as u8);
        output.extend(self.bytes().collect::<Vec<u8>>());
        output
    }
}

impl Serialise for u16 {
    fn serialise(self) -> Vec<u8> {
        vec![(self >> 8) as u8, (self & 0xFF) as u8]
    }
}

impl<T: Serialise + Copy> Serialise for Vec<T> {
    fn serialise(self) -> Vec<u8> {
        let mut temp = Vec::new();
        self.iter().for_each(|el| temp.extend(el.serialise()));
        let mut output = Vec::new();
        output.extend((temp.len() as u16).serialise());
        output.extend(temp);
        output
    }
}

const RECV_TIME_OUT: u64 = 5; // in seconds
const IP_ADDRESS: &'static str = "192.168.1.100:5475";

fn main() {
    database::init();
    let listener = match TcpListener::bind(IP_ADDRESS) {
        Ok(listener) => listener,
        Err(err) => {
            etprintln!("Failed to bind the tcp server to {IP_ADDRESS}: {err:?}");
            return;
        }
    };

    etprintln!("Connected on {IP_ADDRESS}");

    for stream in listener.incoming() {
        etprintln!("Connection incoming.");
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(err) => etprintln!("Connection failed: {err:?}"),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let client_address: SocketAddr = match stream.peer_addr() {
        Ok(addr) => addr,
        Err(err) => {
            etprintln!("Failed to get client address: {err:?}");
            return;
        }
    };

    etprintln!("Recieved client ip: {client_address}");

    let message = if let Some(msg) = get_message(&mut stream, client_address) {
        msg
    } else {
        return;
    };

    etprintln!("Recieved message: {message:?}");

    let parse_result = protocol::parse_message(message.as_slice(), client_address.into());
    let parse_output = match parse_result {
        Err(err) => {
            etprintln!("Bad message: {err:?}");
            return write_response(&mut stream, client_address, err as u8);
        }
        Ok(po) => po,
    };

    let mut response_body: Vec<u8> = Vec::new();

    let database_result = match parse_output {
        protocol::ParseOutput::Create(lobby) => database::create(lobby),
        protocol::ParseOutput::Modify(lobby) => database::modify(lobby),
        protocol::ParseOutput::Destroy((host_ip, port, password)) => {
            database::delete(host_ip, port, password)
        }
        protocol::ParseOutput::Get(get_request) => {
            let page_result = database::get(get_request);
            match page_result {
                Ok(page) => {
                    response_body = page.serialise();
                    Ok(())
                }
                Err(err) => Err(err),
            }
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

    if !response_body.is_empty() {
        let length = response_body.len() as u16;
        response_body.insert(0, (length & 0xFF) as u8);
        response_body.insert(0, (length >> 8) as u8);
        if let Err(err) = stream.write_all(&response_body) {
            etprintln!("Failed to write response_body: {err:?}");
        } else {
            etprintln!("Sent page to client.");
        }
    }
}

fn get_message(stream: &mut TcpStream, client_address: SocketAddr) -> Option<Vec<u8>> {
    let mut thread_stream = match stream.try_clone() {
        Ok(ts) => ts,
        Err(err) => {
            etprintln!("Failed to duplicate stream: {err:?}");
            return None;
        }
    };
    let recv_thread = thread::spawn(move || {
        let mut length: [u8; 1] = [0];
        if let Err(err) = thread_stream.read_exact(&mut length) {
            etprintln!("Connection interupted: Failed to get message length. Error: {err:?}");
            return None;
        };

        let mut message = vec![0; length[0] as usize];
        if let Err(err) = BufReader::new(&mut thread_stream).read_exact(&mut message) {
            etprintln!("Connection interupted: Failed to get message body. Error: {err:?}");
            return None;
        }

        Some(message)
    });

    let start = Instant::now();
    loop {
        if recv_thread.is_finished() {
            let message = match recv_thread.join() {
                Ok(msg) => msg,
                Err(err) => {
                    etprintln!("Failed to join thread: {err:?}");
                    return None;
                }
            };
            break message;
        }
        if start.elapsed() > Duration::from_secs(RECV_TIME_OUT) {
            etprintln!("Connection timed out. Ending connection.");
            write_response(stream, client_address, 101);
            match stream.shutdown(std::net::Shutdown::Both) {
                Err(err) => etprintln!("Failed to shutdown connection: {err:?}"),
                Ok(()) => (),
            }
            return None;
        }

        std::thread::sleep(Duration::from_millis(100));
    }
}

fn write_response(stream: &mut TcpStream, client_address: SocketAddr, response: u8) {
    if let Err(err) = stream.write(&[response]) {
        etprintln!("Failed to write to stream. Client: {client_address} / Code: {response}. Error: {err:?}");
    } else {
        etprintln!("Sent response {response} to client {client_address}.");
    }
}
