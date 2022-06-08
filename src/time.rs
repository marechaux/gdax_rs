//! This module contains all `EndPointRequest` and there response type of GDAX API doc under
//! "Market Data/Time" section (<https://docs.gdax.com/#time>)

use hyper::Method;
use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};

use crate::rest_client::{EndPointRequest, RestRequest};
use crate::url::Route;

#[derive(Default)]
pub struct GetTime;

impl GetTime {
    pub fn new() -> GetTime {
        GetTime::default()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Time {
    iso: DateTime<Utc>,
    epoch: f64,
}

impl EndPointRequest<Time> for GetTime {
    fn create_request(&self) -> RestRequest {
        RestRequest {
            http_method: Method::GET,
            route: Route::new().add_segment(&"time"),
            body: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};
    use hyper::Method;
    use serde_json;

    use super::{EndPointRequest, GetTime, RestRequest, Route, Time};

    #[test]
    fn test_create_request() {
        let result = GetTime::new().create_request();

        let expected = RestRequest {
            http_method: Method::GET,
            route: Route::new().add_segment(&"time"),
            body: String::new(),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_deserialize() {
        let result: Time = serde_json::from_str(
            "{
    \"iso\": \"2015-01-07T23:47:25.201Z\",
    \"epoch\": 1420674445.201
}",
        ).unwrap();
        let expected = Time {
            iso: Utc.ymd(2015, 1, 7).and_hms_micro(23, 47, 25, 201_000),
            epoch: 1420674445.201,
        };

        assert_eq!(result, expected);
    }
}
