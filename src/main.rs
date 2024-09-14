use database::DatabaseError;
use protocol::ParseError;

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
}
