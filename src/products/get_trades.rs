// TODO : handle pagination
use chrono::{DateTime, Utc};
use hyper::Method;

use rest_client::{deserialize_from_str, EndPointRequest, EndPointRequestHandler};
use url::Route;

pub struct GetTrades {
    product_id: String,
}

impl GetTrades {
    pub fn new(product_id: String) -> Self {
        GetTrades { product_id }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Side {
    #[serde(rename = "sell")] Sell,
    #[serde(rename = "buy")] Buy,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Trade {
    time: DateTime<Utc>,
    trade_id: i64,
    #[serde(deserialize_with = "deserialize_from_str")] price: f64,
    #[serde(deserialize_with = "deserialize_from_str")] size: f64,
    side: Side,
}

impl EndPointRequestHandler<Vec<Trade>> for GetTrades {
    fn create_request(&self) -> EndPointRequest {
        EndPointRequest {
            http_method: Method::Get,
            route: Route::new()
                .add_segment(&"products")
                .add_segment(&self.product_id)
                .add_segment(&"trades"),
            body: String::new(),
        }
    }

    //    fn deserialize(&self, http_body: String) -> Result<Vec<Trade>, RestError> {
    //        serde_json::from_str(&http_body).or(Err(RestError::NotImplemented))
    //    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use super::{EndPointRequest, EndPointRequestHandler, GetTrades, Method, Route, Side, Trade};

    #[test]
    fn test_create_request() {
        let result = GetTrades::new(String::from("BTC-USD")).create_request();
        let expected = EndPointRequest {
            http_method: Method::Get,
            route: Route::new()
                .add_segment(&"products")
                .add_segment(&"BTC-USD")
                .add_segment(&"trades"),
            body: String::new(),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_deserialize() {
        let result = GetTrades::new(String::from("BTC-USD"))
            .deserialize(&String::from(
                "\
[{
    \"time\": \"2014-11-07T22:19:28.578544Z\",
    \"trade_id\": 74,
    \"price\": \"10.00000000\",
    \"size\": \"0.01000000\",
    \"side\": \"buy\"
}, {
    \"time\": \"2014-11-07T01:08:43.642366Z\",
    \"trade_id\": 73,
    \"price\": \"100.00000000\",
    \"size\": \"0.01000000\",
    \"side\": \"sell\"
}]",
            ))
            .unwrap();
        let expected = vec![
            Trade {
                time: Utc.ymd(2014, 11, 07).and_hms_micro(22, 19, 28, 578_544),
                trade_id: 74,
                price: 10.0,
                size: 0.01,
                side: Side::Buy,
            },
            Trade {
                time: Utc.ymd(2014, 11, 07).and_hms_micro(1, 8, 43, 642_366),
                trade_id: 73,
                price: 100.0,
                size: 0.01,
                side: Side::Sell,
            },
        ];

        assert_eq!(result, expected);
    }
}
