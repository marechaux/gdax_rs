use serde::de;
use serde_json;
use hyper::{Body, Client, Method, Request, Uri};
use hyper::header::{ContentLength, UserAgent};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;
use futures::{Future, Stream};

use url::Route;
use error::RestError;
use error::ParseError;

const PUBLIC_API: &str = "https://api.gdax.com";
const SANDBOX_API: &str = "https://api-public.sandbox.gdax.com";
const USER_AGENT: &str = concat!("gdax_rs/", env!("CARGO_PKG_VERSION"));

pub struct RESTClient {
    api_url: String,
    core: Core,
    client: Client<HttpsConnector<HttpConnector>, Body>,
}

impl RESTClient {
    /// Create a new `RESTClient` object with a specified API URL, for most cases, you should use
    /// `RESTClient::default` or `RESTClient::staging` to connect to GDAX
    pub fn new(api_url: &str) -> Result<RESTClient, RestError> {
        let core = Core::new()?;
        let handle = core.handle();
        let connector = HttpsConnector::new(4, &handle)
            .map_err(|e| RestError::HttpsConnectorError(e.to_string()))?;
        let client = Client::configure().connector(connector).build(&handle);
        Ok(RESTClient {
            api_url: String::from(api_url),
            core,
            client,
        })
    }

    /// Returns the default APIConnector (connected to the staging API)
    pub fn default() -> RESTClient {
        RESTClient::new(PUBLIC_API).unwrap()
    }

    /// Returns the sandbox APIConnector (connected to the sandbox API)
    pub fn sandbox() -> RESTClient {
        RESTClient::new(SANDBOX_API).unwrap()
    }

    fn send_http_request(&mut self, request: &EndPointRequest) -> Result<String, RestError> {
        // create the full request uri
        let uri: Uri = format!("{}{}", self.api_url, request.route.to_string()).parse()?;

        // create request
        let mut req = Request::new(request.http_method.clone(), uri);
        req.headers_mut()
            .set(ContentLength(request.body.len() as u64));
        req.set_body(request.body.clone());

        // set the user agent (required by the API)
        req.headers_mut().set(UserAgent::new(USER_AGENT));

        let work = self.client
            .request(req)
            .and_then(|res| res.body().concat2());

        let request_result = self.core.run(work)?;
        let result = String::from_utf8(request_result.to_vec())?;

        Ok(result)
    }

    /// This method send a request to GDAX API and return the result as an struct `T`
    pub fn request<T: de::DeserializeOwned>(
        &mut self,
        request_handler: &EndPointRequestHandler<T>,
    ) -> Result<T, RestError> {
        let http_result = self.send_http_request(&request_handler.create_request())?;
        request_handler.deserialize(&http_result)
    }
}

// TODO Should this be public?
#[derive(PartialEq, Debug)]
pub struct EndPointRequest {
    pub http_method: Method,
    pub route: Route,
    pub body: String,
}

// TODO : Should it be public?
/// A struct that implement the trait `EndPointRequestHandler` is capable of creating generate a
/// request and parse the result.
pub trait EndPointRequestHandler<T: de::DeserializeOwned> {
    fn create_request(&self) -> EndPointRequest;
    fn deserialize(&self, http_body: &str) -> Result<T, RestError> {
        serde_json::from_str(http_body).map_err(|e| {
            RestError::ParseError(ParseError::new(String::from(http_body), e.to_string()))
        })
    }
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
