use std::str::FromStr;
use std::fmt::Display;
//use std::io::Error;
//use serde::de::{Deserialize, Deserializer, Error};
use serde::de;
use serde_json;
use hyper::{Body, Client, Method, Request};
use hyper::header::{ContentLength, UserAgent};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;
use futures::{Future, Stream};

use url::Route;
use error::RestError;
use error::ParseError;

pub struct RESTClient {
    api_url: String,
    core: Core,
    client: Client<HttpsConnector<HttpConnector>, Body>,
}

impl RESTClient {
    pub fn new(api_url: &str) -> Result<Self, RestError> {
        let core = Core::new().or_else(|e| Err(RestError::CoreError(e)))?;
        let handle = core.handle();
        let connector = HttpsConnector::new(4, &handle)
            .or_else(|e| Err(RestError::HttpsConnectorError(format!("{}", e))))?;
        let client = Client::configure().connector(connector).build(&handle);
        Ok(RESTClient {
            api_url: String::from(api_url),
            core,
            client,
        })
    }

    // TODO: Remove the https part from the url
    /// Returns the default APIConnector (connected to the staging API)
    pub fn default() -> RESTClient {
        RESTClient::new("https://api.gdax.com").unwrap()
    }

    /// Returns the sandbox APIConnector (connected to the staging API)
    pub fn sandbox() -> RESTClient {
        RESTClient::new("https://api-public.sandbox.gdax.com").unwrap()
    }

    fn send_http_request(&mut self, request: &EndPointRequest) -> Result<String, RestError> {
        // create the full request uri
        let uri = format!("{}{}", self.api_url, request.route.to_string())
            .parse()
            .or(Err(RestError::NotImplemented))?;

        // create request
        let mut req = Request::new(request.http_method.clone(), uri);
        req.headers_mut()
            .set(ContentLength(request.body.len() as u64));
        req.set_body(request.body.clone());

        // set the user agent (required by the API)
        req.headers_mut().set(UserAgent::new("hyper/0.11"));

        let work = self.client
            .request(req)
            .and_then(|res| res.body().concat2());

        let request_result = self.core.run(work).or(Err(RestError::NotImplemented))?;
        let result = String::from_utf8(request_result.to_vec()).or(Err(RestError::NotImplemented))?;

        Ok(result)
    }

    pub fn request<T: de::DeserializeOwned>(
        &mut self,
        request_handler: &EndPointRequestHandler<T>,
    ) -> Result<T, RestError> {
        let http_result = self.send_http_request(&request_handler.create_request())
            .or(Err(RestError::NotImplemented))?;
        request_handler.deserialize(&http_result)
    }
}

// TODO: make a constructor?
#[derive(PartialEq, Debug)]
pub struct EndPointRequest {
    pub http_method: Method,
    pub route: Route,
    pub body: String,
}

/// TODO: doc
pub trait EndPointRequestHandler<T: de::DeserializeOwned> {
    fn create_request(&self) -> EndPointRequest;
    fn deserialize(&self, http_body: &String) -> Result<T, RestError> {
        serde_json::from_str(http_body)
            .map_err(|e| RestError::ParseError(ParseError::new(http_body.clone(), e.to_string())))
    }
}

/// Gdax return the floats values as strings, we need ti use the `FromStr` trait to
/// deserialize the string.
///
/// Taken from <https://stackoverflow.com/documentation/rust/1170/serde#t=201708271607008933769/>
pub fn deserialize_from_str<'de, S, D>(deserializer: D) -> Result<S, D::Error>
where
    S: FromStr,
    S::Err: Display,
    D: de::Deserializer<'de>,
{
    let s: String = de::Deserialize::deserialize(deserializer)?;
    S::from_str(&s).map_err(de::Error::custom)
}

#[cfg(test)]
mod tests {
    use mockito::{mock, SERVER_URL};
    use hyper::Method;

    use super::{EndPointRequest, EndPointRequestHandler, RESTClient, Route};

    struct FakeRequestHandler;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct FakeAnswerType {
        value: i64, // this value could be used to test
    }

    impl EndPointRequestHandler<FakeAnswerType> for FakeRequestHandler {
        fn create_request(&self) -> EndPointRequest {
            EndPointRequest {
                http_method: Method::Get,
                route: Route::new().add_segment(&"test"),
                body: String::from(""),
            }
        }
    }

    #[test]
    fn test_fake_request() {
        let _m = mock("GET", "/test").with_body("{\"value\": 1}").create();

        let mut test_client = RESTClient::new(SERVER_URL).unwrap();
        let request = FakeRequestHandler {};

        assert_eq!(test_client.request(&request).unwrap().value, 1);
    }
}
