use tokio::io::Error as IOError;

#[derive(Debug)]
pub enum Error {
    IOError,
    ConfigParsingError,
    AuthError,
}
