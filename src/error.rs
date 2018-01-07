use std::io;
use std::error::Error;
use std::fmt;
use std::string::FromUtf8Error;

use hyper;

#[derive(Debug)]
pub enum RestError {
    /// The tokio core failed to instantiate
    CoreError(io::Error),
    // cannot reexport the native_tls error until RFC #1977 is implemented
    /// The Https Connector has failed to instantiate
    HttpsConnectorError(String),
    /// The http response from GDAX cannot be parse correctly
    ParseError(ParseError),
    // TODO: Is it possible if the coverage is good?
    /// Uri creation Error
    UriError(String),
    /// The https request has failed
    RequestError(String),
    /// This error can happen when the http body response is converted to string
    FromUtf8Error(FromUtf8Error),
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
        match *self {
            RestError::CoreError(_) => "Error while instantiating the tokio core",
            RestError::HttpsConnectorError(_) => {
                "Error while instantiating hyper_tls::HTTPSConnector"
            }
            RestError::ParseError(_) => "Cannot parse GDAX response",
            RestError::UriError(_) => "Error while creating the uri",
            RestError::RequestError(_) => "Error while sending the https request to GDAX",
            RestError::FromUtf8Error(_) => "Error while converting GDAX http response to UFT8",
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl fmt::Display for RestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RestError::CoreError(ref io_error) => {
                write!(f, "{} : {}", self.description(), io_error.to_string())
            }
            RestError::ParseError(ref parse_error) => write!(
                f,
                "{} ({}) : {}",
                self.description(),
                parse_error.content,
                parse_error.error_message
            ),
            RestError::UriError(ref error_string)
            | RestError::HttpsConnectorError(ref error_string)
            | RestError::RequestError(ref error_string) => {
                write!(f, "{} : {}", self.description(), error_string)
            }
            RestError::FromUtf8Error(ref utf8_error) => {
                write!(f, "{} : {}", self.description(), utf8_error)
            }
        }
    }
}

impl From<hyper::error::UriError> for RestError {
    fn from(uri_error: hyper::error::UriError) -> RestError {
        RestError::UriError(uri_error.to_string())
    }
}

impl From<hyper::error::Error> for RestError {
    fn from(error: hyper::error::Error) -> RestError {
        RestError::RequestError(error.to_string())
    }
}

impl From<FromUtf8Error> for RestError {
    fn from(utf8_error: FromUtf8Error) -> RestError {
        RestError::FromUtf8Error(utf8_error)
    }
}

impl From<io::Error> for RestError {
    fn from(io_error: io::Error) -> RestError {
        RestError::CoreError(io_error)
    }
}
