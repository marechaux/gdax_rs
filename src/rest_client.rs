use hyper;
use serde::de;
use serde_json;
use hyper::{Body, Client, Method, Request, Uri};
use hyper::header::{ContentLength, UserAgent};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Handle;
use futures::{Future, Stream};

use url::Route;
use error::RestError;

const PUBLIC_API: &str = "https://api.gdax.com";
const SANDBOX_API: &str = "https://api-public.sandbox.gdax.com";
const USER_AGENT: &str = concat!("gdax_rs/", env!("CARGO_PKG_VERSION"));

pub struct RESTClient {
    api_url: String,
    client: Client<HttpsConnector<HttpConnector>, Body>,
}

// TODO: remove all unwrap and handle error (error chain??)
impl RESTClient {
    /// Create a new `RESTClient` object with a specified API URL, for most cases, you should use
    /// `RESTClient::default` or `RESTClient::staging` to connect to GDAX
    pub fn new(api_url: &str, handle: &Handle) -> Result<RESTClient, RestError> {
        let connector = HttpsConnector::new(4, handle)
            .map_err(|e| RestError::HttpsConnectorError(e.to_string()))?;
        let client = Client::configure().connector(connector).build(handle);
        Ok(RESTClient {
            api_url: String::from(api_url),
            client,
        })
    }

    /// Returns the default APIConnector (connected to the staging API)
    pub fn default(handle: &Handle) -> RESTClient {
        RESTClient::new(PUBLIC_API, handle).unwrap()
    }

    /// Returns the sandbox APIConnector (connected to the sandbox API)
    pub fn sandbox(handle: &Handle) -> RESTClient {
        RESTClient::new(SANDBOX_API, handle).unwrap()
    }

    /// This method send a request to GDAX API and return the result as a `Future`
    pub fn send_request<T: 'static + de::DeserializeOwned>(
        &mut self,
        request: &EndPointRequest<T>,
    ) -> Box<Future<Item = T, Error = hyper::Error> + 'static> {
        let request = request.create_request();

        // create the full request uri
        // TODO: remove unwrap
        let uri: Uri = format!("{}{}", self.api_url, request.route.to_string())
            .parse()
            .unwrap();

        // create request
        let mut req = Request::new(request.http_method.clone(), uri);
        req.headers_mut()
            .set(ContentLength(request.body.len() as u64));
        req.set_body(request.body.clone());

        // set the user agent (required by the API)
        req.headers_mut().set(UserAgent::new(USER_AGENT));

        let work = self.client
            .request(req)
            .and_then(|res| res.body().concat2())
            .and_then(|body| Ok(serde_json::from_slice(&body).unwrap()));

        Box::new(work)
    }
}

#[derive(PartialEq, Debug)]
pub struct RestRequest {
    pub http_method: Method,
    pub route: Route,
    pub body: String,
}

/// A struct that implement the trait `EndPointRequest` is capable of creating generate a
/// request and parse the result.
pub trait EndPointRequest<T: de::DeserializeOwned> {
    fn create_request(&self) -> RestRequest;
}

// TODO: test error handling!
#[cfg(test)]
mod tests {
    use tokio_core::reactor::Core;

    use mockito::{mock, SERVER_URL};
    use hyper::Method;

    use super::{EndPointRequest, RESTClient, RestRequest, Route};

    struct FakeRequestHandler;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct FakeAnswerType {
        value: i64, // this value could be used to test
    }

    impl EndPointRequest<FakeAnswerType> for FakeRequestHandler {
        fn create_request(&self) -> RestRequest {
            RestRequest {
                http_method: Method::Get,
                route: Route::new().add_segment(&"test"),
                body: String::from(""),
            }
        }
    }

    #[test]
    fn test_fake_request() {
        let _m = mock("GET", "/test").with_body("{\"value\": 1}").create();
        let mut core = Core::new().unwrap();
        let handle = core.handle();

        let mut test_client = RESTClient::new(SERVER_URL, &handle).unwrap();
        let request = FakeRequestHandler {};

        let future = test_client.send_request(&request);

        let value = core.run(future).unwrap();

        assert_eq!(value.value, 1);
    }
}
