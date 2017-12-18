extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate itertools;
extern crate mockito;
extern crate tokio_core;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

pub mod products;
mod url;

use std::str::FromStr;
use std::fmt::Display;
use serde::de::{Deserialize, Deserializer};
use hyper::{Client, Method, Request};
use hyper::header::{ContentLength, UserAgent};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;
use futures::{Future, Stream};

use url::Route;

pub struct RESTClient {
    api_url: String,
    core: Core,
    client: Client<HttpsConnector<HttpConnector>, hyper::Body>,
}

impl RESTClient {
    pub fn new(api_url: &str) -> RESTClient {
        let core = Core::new().unwrap();
        let handle = core.handle();
        let connector = HttpsConnector::new(4, &handle).unwrap();
        let client = Client::configure().connector(connector).build(&handle);

        RESTClient {
            api_url: String::from(api_url),
            core,
            client,
        }
    }

    // TODO: Remove the https part from the url
    /// Returns the default APIConnector (connected to the staging API)
    pub fn default() -> RESTClient {
        RESTClient::new("https://api.gdax.com")
    }

    /// Returns the sandbox APIConnector (connected to the staging API)
    pub fn sandbox() -> RESTClient {
        RESTClient::new("https://api-public.sandbox.gdax.com")
    }

    fn send_http_request(&mut self, request: &EndPointRequest) -> String {
        // create the full request uri
        let uri = format!("{}{}", self.api_url, request.route.to_string())
            .parse()
            .unwrap();

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

        String::from_utf8(self.core.run(work).unwrap().to_vec()).unwrap()
    }

    pub fn request<T>(&mut self, request_handler: &EndPointRequestHandler<T>) -> T {
        request_handler.deserialize(self.send_http_request(&request_handler.create_request()))
    }
}

#[derive(PartialEq, Debug)]
pub struct EndPointRequest {
    http_method: Method,
    route: Route,
    body: String,
}

pub trait EndPointRequestHandler<T> {
    fn create_request(&self) -> EndPointRequest;
    // TODO : ref or not?
    // TODO : Handle error
    fn deserialize(&self, http_body: String) -> T;
}

/// Gdax return the floats values as strings, we need ti use the `FromStr` trait to
/// deserialize the string.
///
/// Taken from <https://stackoverflow.com/documentation/rust/1170/serde#t=201708271607008933769/>
fn deserialize_from_str<'de, S, D>(deserializer: D) -> Result<S, D::Error>
where
    S: FromStr,
    S::Err: Display,
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    S::from_str(&s).map_err(serde::de::Error::custom)
}

#[cfg(test)]
mod tests {
    use mockito::{mock, SERVER_URL};
    use hyper::Method;

    use EndPointRequestHandler;
    use EndPointRequest;
    use RESTClient;
    use url::Route;

    struct FakeRequestHandler;

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

        fn deserialize(&self, http_body: String) -> FakeAnswerType {
            FakeAnswerType {
                value: http_body.parse().unwrap(),
            }
        }
    }

    #[test]
    fn test_fake_request() {
        let _m = mock("GET", "/test").with_body("1").create();

        let mut test_client = RESTClient::new(SERVER_URL);
        let request = FakeRequestHandler {};

        assert_eq!(test_client.request(&request).value, 1);
    }
}
