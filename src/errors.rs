use tokio::io::Error as IOError;

pub enum Error {
    IOError(IOError),
    ConfigParsingError,
}
