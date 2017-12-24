use chrono::{DateTime, Utc};
use serde_json;
use hyper::Method;

use rest_client::{EndPointRequest, EndPointRequestHandler};
use url::Route;

pub struct GetHistoricRates {
    product_id: String,
    // TODO: put as Option (as it is not required by the API)
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    granularity: i64,
}

impl GetHistoricRates {
    pub fn new(
        product_id: String,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        granularity: i64,
    ) -> GetHistoricRates {
        GetHistoricRates {
            product_id,
            start,
            end,
            granularity,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Candle {
    time: i64, // TODO: change it to Datetime??
    low: f64,
    high: f64,
    open: f64,
    close: f64,
    volume: f64,
}

impl EndPointRequestHandler<Vec<Candle>> for GetHistoricRates {
    fn create_request(&self) -> EndPointRequest {
        EndPointRequest {
            http_method: Method::Get,
            route: Route::new()
                .add_segment(&"products")
                .add_segment(&self.product_id)
                .add_segment(&"candles")
                .add_attribute_value(&"start", &self.start.naive_utc())
                .add_attribute_value(&"end", &self.end.naive_utc())
                .add_attribute_value(&"granularity", &self.granularity),
            body: String::new(),
        }
    }

    fn deserialize(&self, http_body: String) -> Vec<Candle> {
        println!("body : \"{}\"", &http_body);
        serde_json::from_str(&http_body).unwrap()
    }
}

#[cfg(test)]
mod test {
    use hyper::Method;
    use chrono::{TimeZone, Utc};

    use super::{Candle, EndPointRequest, EndPointRequestHandler, GetHistoricRates, Route};

    #[test]
    fn test_create_request() {
        let result = GetHistoricRates::new(
            String::from("BTC-USD"),
            Utc.ymd(2014, 11, 07).and_hms_micro(22, 19, 28, 578_544),
            Utc.ymd(2014, 11, 07).and_hms_micro(22, 20, 28, 1),
            1,
        ).create_request();

        let expected = EndPointRequest {
            http_method: Method::Get,
            route: Route::new()
                .add_segment(&"products")
                .add_segment(&"BTC-USD")
                .add_segment(&"candles")
                .add_attribute_value(&"start", &"2014-11-07 22:19:28.578544")
                .add_attribute_value(&"end", &"2014-11-07 22:20:28.000001")
                .add_attribute_value(&"granularity", &1),
            body: String::new(),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_deserialize() {
        let result = GetHistoricRates::new(
            String::from("BTC-USD"),
            Utc.ymd(2014, 11, 07).and_hms_micro(22, 19, 28, 578_544),
            Utc.ymd(2014, 11, 07).and_hms_micro(22, 20, 28, 1),
            1,
        ).deserialize(String::from(
            "[
    [ 1415398768, 0.32, 4.2, 0.35, 4.2, 12.3 ],
    [ 1415398769, 0.33, 4.3, 0.36, 4.2, 12.3 ]
]",
        ));
        let expected = vec![
            Candle {
                time: 1415398768,
                low: 0.32,
                high: 4.2,
                open: 0.35,
                close: 4.2,
                volume: 12.3,
            },
            Candle {
                time: 1415398769,
                low: 0.33,
                high: 4.3,
                open: 0.36,
                close: 4.2,
                volume: 12.3,
            },
        ];

        assert_eq!(result, expected);
    }
}
