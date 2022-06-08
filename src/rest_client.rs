use hyper;
use serde::de;
use serde_json;
use hyper::{Body, Client, Method, Request, Uri};
use hyper::header::{CONTENT_LENGTH, USER_AGENT, HeaderValue};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use futures::{Future, Stream, TryFuture, TryFutureExt};
use serde_derive::{Serialize, Deserialize};

use crate::url::Route;
use crate::error::RestError;

const PUBLIC_API: &str = "https://api.gdax.com";
const SANDBOX_API: &str = "https://api-public.sandbox.gdax.com";
const USER_AGENT_VALUE: &str = concat!("gdax_rs/", env!("CARGO_PKG_VERSION"));

pub struct RESTClient {
    api_url: String,
    client: Client<HttpsConnector<HttpConnector>, Body>,
}

// TODO: remove all unwrap and handle error (error chain??)
impl RESTClient {
    /// Create a new `RESTClient` object with a specified API URL, for most cases, you should use
    /// `RESTClient::default` or `RESTClient::staging` to connect to GDAX
    pub fn new(api_url: &str) -> Result<RESTClient, RestError> {
        let https = HttpsConnector::new();
        let client = Client::builder()
            .build::<_, hyper::Body>(https);
        // let connector = HttpsConnector::new(4, handle)
        //     .map_err(|e| RestError::HttpsConnectorError(e.to_string()))?;
        // let client = Client::configure().connector(connector).build(handle);
        Ok(RESTClient {
            api_url: String::from(api_url),
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

    /// This method send a request to GDAX API and return the result as a `Future`
    pub async fn send_request<T: 'static + de::DeserializeOwned>(
        &mut self,
        request: &dyn EndPointRequest<T>,
    ) -> Result<T, hyper::Error> {
        let request = request.create_request();

        // create the full request uri
        // TODO: remove unwrap
        let uri: Uri = format!("{}{}", self.api_url, request.route.to_string())
            .parse()
            .unwrap();

        // create request
        let mut req = Request::builder()
            .method(request.http_method.clone())
            .uri(uri)
            .header(USER_AGENT, HeaderValue::from_static(USER_AGENT_VALUE))
            // .header(CONTENT_LENGTH, HeaderValue::from_static(&request.body.len().to_string()))
            .body(Body::from(request.body.clone())).unwrap();
            // .and_then(|res| res.body().concat2())
            // .and_then(|body| Ok(serde_json::from_slice(&body).unwrap()));

        
        // req.headers_mut()
        //     .insert(CONTENT_LENGTH, HeaderValue::from_static(&request.body.len().to_string()));
        // req.set_body(request.body.clone());

        // set the user agent (required by the API)
        // req.headers_mut().insert(USER_AGENT, HeaderValue::from_static(USER_AGENT_VALUE));

        let client = Client::new();


        // let work = self.client
        let resp = client.request(req).await?;

        let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;

        Ok(serde_json::from_slice(&body_bytes.to_vec()).unwrap())
            // .map_ok(|res| res.body().concat2())
            // .map_ok(hyper::body::to_bytes)
            // .map_ok(|body| Ok(serde_json::from_slice(&body.body().to_string()).unwrap()));

        // Box::new(work)

        // resp
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
    use tokio;
    use tokio_core::reactor::Core;
    use serde_derive::{Serialize, Deserialize};

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
                http_method: Method::GET,
                route: Route::new().add_segment(&"test"),
                body: String::from(""),
            }
        }
    }

    #[tokio::test]
    async fn test_fake_request() {
        let _m = mock("GET", "/test").with_body("{\"value\": 1}").create();
        let mut core = Core::new().unwrap();
        // let handle = core.handle();

        let mut test_client = RESTClient::new(SERVER_URL).unwrap();
        let request = FakeRequestHandler {};

        let value = test_client.send_request(&request).await.unwrap();

        // let value = core.run(future).unwrap();

        assert_eq!(value.value, 1);
    }
}
