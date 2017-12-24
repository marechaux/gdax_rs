use hyper::Method;
use serde_json;

use rest_client::{deserialize_from_str, EndPointRequest, EndPointRequestHandler};
use url::Route;

pub struct Get24hrStats {
    product_id: String,
}

impl Get24hrStats {
    pub fn new(product_id: String) -> Get24hrStats {
        Get24hrStats { product_id }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Stats {
    #[serde(deserialize_with = "deserialize_from_str")] open: f64,
    #[serde(deserialize_with = "deserialize_from_str")] high: f64,
    #[serde(deserialize_with = "deserialize_from_str")] volume: f64,
}

impl EndPointRequestHandler<Stats> for Get24hrStats {
    fn create_request(&self) -> EndPointRequest {
        EndPointRequest {
            http_method: Method::Get,
            route: Route::new()
                .add_segment(&"products")
                .add_segment(&self.product_id)
                .add_segment(&"stats"),
            body: String::new(),
        }
    }

    fn deserialize(&self, http_body: String) -> Stats {
        serde_json::from_str(&http_body).unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::{EndPointRequest, EndPointRequestHandler, Get24hrStats, Method, Route, Stats};

    #[test]
    fn test_create_request() {
        let result = Get24hrStats::new(String::from("BTC-USD")).create_request();

        let expected = EndPointRequest {
            http_method: Method::Get,
            route: Route::new()
                .add_segment(&"products")
                .add_segment(&"BTC-USD")
                .add_segment(&"stats"),
            body: String::new(),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_deserialize() {
        let result = Get24hrStats::new(String::from("BTC-USD")).deserialize(String::from(
            "{
    \"open\": \"34.19000000\",
    \"high\": \"95.70000000\",
    \"low\": \"7.06000000\",
    \"volume\": \"2.41000000\"
}",
        ));
        let expected = Stats {
            open: 34.19,
            high: 95.7,
            volume: 2.41,
        };

        assert_eq!(result, expected);
    }
}
