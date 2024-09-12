use protocol::IpAddress;

mod database;
mod protocol;

fn main() {
    println!("Hello, world!");
    let test4 = IpAddress::IpV4([192, 168, 1, 100]);
    let test6 = IpAddress::IpV6([
        0x2405, 0xda40, 0x1179, 0x8400, 0x20ec, 0xc082, 0x2cf6, 0xc8f4,
    ]);

    println!("IpV4: {test4}");
    println!("IpV6: {test6}");
}
