use std::io;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum RestError {
    /// The tokio core failed to instantiate
    CoreError(io::Error),
    /// The Https Connector has failed to instantiate
    // cannot reexport the native_tls error until RFC #1977 is implemented
    HttpsConnectorError(String),
    /// Not implemented Error : TODO
    NotImplemented,
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
