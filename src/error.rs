use std::io;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum RestError {
    /// The tokio core failed to instantiate
    CoreError(io::Error),
    // cannot reexport the native_tls error until RFC #1977 is implemented
    /// The Https Connector has failed to instantiate
    HttpsConnectorError(String),
    /// The http response from GDAX cannot be parse correctly
    ParseError(ParseError),
    /// Not implemented Error : TODO
    NotImplemented,
}

#[derive(Debug)]
pub struct ParseError {
    content: String,
    error_message: String,
}

impl ParseError {
    pub fn new(content: String, error_message: String) -> ParseError {
        ParseError {
            content,
            error_message,
        }
    }
}

impl Error for RestError {
    fn description(&self) -> &str {
        unimplemented!()
    }

    fn cause(&self) -> Option<&Error> {
        unimplemented!()
    }
}

impl fmt::Display for RestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}
