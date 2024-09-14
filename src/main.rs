use database::DatabaseError;
use protocol::{IpAddress, ParseError};

mod database;
mod protocol;

enum Errors {
    Database(DatabaseError),
    Parse(ParseError),
}
trait ConvertError<T> {
    fn convert(self) -> Result<T, Errors>;
}
impl<T> ConvertError<T> for Result<T, DatabaseError> {
    fn convert(self) -> Result<T, Errors> {
        self.map_err(|e| Errors::Database(e))
    }
}
impl<T> ConvertError<T> for Result<T, ParseError> {
    fn convert(self) -> Result<T, Errors> {
        self.map_err(|e| Errors::Parse(e))
    }
}

fn main() {
    database::init();
    println!("Hello, world!");
    let test4 = IpAddress::IpV4([192, 168, 1, 100]);
    let test6 = IpAddress::IpV6([
        0x2405, 0xda40, 0x1179, 0x8400, 0x20ec, 0xc082, 0x2cf6, 0xc8f4,
    ]);

    println!("IpV4: {test4}");
    println!("IpV6: {test6}");
}
