use hyper::Method;
use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};

use crate::serde_util::deserialize_from_str;
use crate::rest_client::{EndPointRequest, RestRequest};
use crate::url::Route;



/// This struct represents the `Get Product Ticker` end point.
/// <https://docs.gdax.com/#get-product-ticker>
pub struct GetProductTicker {
    product_id: String,
}

impl GetProductTicker {
    pub fn new(product_id: String) -> GetProductTicker {
        GetProductTicker { product_id }
    }
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Ticker {
    trade_id: i64,
    #[serde(deserialize_with = "deserialize_from_str")]
    price: f64,
    #[serde(deserialize_with = "deserialize_from_str")]
    size: f64,
    #[serde(deserialize_with = "deserialize_from_str")]
    bid: f64,
    #[serde(deserialize_with = "deserialize_from_str")]
    ask: f64,
    #[serde(deserialize_with = "deserialize_from_str")]
    volume: f64,
    time: DateTime<Utc>,
}

impl EndPointRequest<Ticker> for GetProductTicker {
    fn create_request(&self) -> RestRequest {
        RestRequest {
            http_method: Method::GET,
            route: Route::new()
                .add_segment(&"products")
                .add_segment(&self.product_id)
                .add_segment(&"ticker"),
            body: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};
    use hyper::Method;
    use serde_json;

    use super::{EndPointRequest, GetProductTicker, RestRequest, Route, Ticker};

    #[test]
    fn test_create_request() {
        let result = GetProductTicker::new(String::from("BTC-USD")).create_request();
        let expected = RestRequest {
            http_method: Method::GET,
            route: Route::new()
                .add_segment(&"products")
                .add_segment(&"BTC-USD")
                .add_segment(&"ticker"),
            body: String::new(),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_deserialize() {
        let result: Ticker = serde_json::from_str(
            "\
{
  \"trade_id\": 4729088,
  \"price\": \"333.99\",
  \"size\": \"0.193\",
  \"bid\": \"333.98\",
  \"ask\": \"333.99\",
  \"volume\": \"5957.11914015\",
  \"time\": \"2015-11-14T20:46:03.511254Z\"
}
            ",
        ).unwrap();

        let expected = Ticker {
            trade_id: 4729088,
            price: 333.99,
            size: 0.193,
            bid: 333.98,
            ask: 333.99,
            volume: 5957.11914015,
            time: Utc.ymd(2015, 11, 14).and_hms_micro(20, 46, 3, 511_254),
        };

        assert_eq!(result, expected);
    }

}
