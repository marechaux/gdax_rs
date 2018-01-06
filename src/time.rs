use hyper::Method;
use serde_json;
use chrono::{DateTime, Utc};

use rest_client::{EndPointRequest, EndPointRequestHandler};
use url::Route;
use error::RestError;

#[derive(Default)]
pub struct GetTime;

impl GetTime {
    pub fn new() -> Self {
        GetTime::default()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Time {
    iso: DateTime<Utc>,
    epoch: f64,
}

impl EndPointRequestHandler<Time> for GetTime {
    fn create_request(&self) -> EndPointRequest {
        EndPointRequest {
            http_method: Method::Get,
            route: Route::new().add_segment(&"time"),
            body: String::new(),
        }
    }

    fn deserialize(&self, http_body: String) -> Result<Time, RestError> {
        serde_json::from_str(&http_body).or(Err(RestError::NotImplemented))
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};
    use hyper::Method;

    use super::{EndPointRequest, EndPointRequestHandler, GetTime, Route, Time};

    #[test]
    fn test_create_request() {
        let result = GetTime::new().create_request();

        let expected = EndPointRequest {
            http_method: Method::Get,
            route: Route::new().add_segment(&"time"),
            body: String::new(),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_deserialize() {
        let result = GetTime::new()
            .deserialize(String::from(
                "{
    \"iso\": \"2015-01-07T23:47:25.201Z\",
    \"epoch\": 1420674445.201
}",
            ))
            .unwrap();
        let expected = Time {
            iso: Utc.ymd(2015, 1, 7).and_hms_micro(23, 47, 25, 201_000),
            epoch: 1420674445.201,
        };

        assert_eq!(result, expected);
    }
}
