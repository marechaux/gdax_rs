use hyper::Method;
use serde_json;

use ::{EndPointRequestHandler, EndPointRequest};
use ::deserialize_from_str;

#[derive(Copy, Clone)]
pub enum Level {
    Level1 = 1,
    Level2 = 2,
    Level3 = 3, // TODO: Handle level 3
}

// TODO Make field private and create a constructor
pub struct GetProductOrderBook {
    pub product_id: String,
    pub level: Level,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct OrderBook<T> {
    pub sequence: i64,
    pub bids: Vec<T>,
    pub asks: Vec<T>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct PriceLevel {
    #[serde(deserialize_with = "deserialize_from_str")]
    pub price: f64,
    #[serde(deserialize_with = "deserialize_from_str")]
    pub size: f64,
    pub num_order: i64,
}

impl EndPointRequestHandler<OrderBook<PriceLevel>> for GetProductOrderBook {
    fn create_request(&self) -> EndPointRequest {
        EndPointRequest {
            http_method: Method::Get,
            // TODO use a url lib.... (for get att)
            route: format!("/products/{}/book?level={}", self.product_id, self.level as i32),
            body: String::new(),
        }
    }

    fn deserialize(&self, http_body: String) -> OrderBook<PriceLevel> {
        println!("{}", &http_body);
        serde_json::from_str(&http_body).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use hyper::Method;

    use super::{
        OrderBook,
        Level,
        PriceLevel,
        GetProductOrderBook,
    };
    use EndPointRequestHandler;
    use EndPointRequest;

    #[test]
    fn test_create_request() {
        let request_handler = GetProductOrderBook {
            product_id: String::from("BTC-USD"),
            level: Level::Level2,
        };
        let result = request_handler.create_request();
        let expected = EndPointRequest {
            http_method: Method::Get,
            route: String::from("/products/BTC-USD/book?level=2"),
            body: String::new(),
        };
        assert_eq!(result, expected);

    }

    #[test]
    fn test_deserialize() {
        let request_handler = GetProductOrderBook {
            product_id: String::from("BTC-USD"),
            level: Level::Level2,
        };
        let result = request_handler.deserialize(String::from("
{
    \"sequence\": 3,
    \"bids\": [
        [\"16839.45\",\"0.47037038\",2],
        [\"16835.39\",\"0.00075522\",2]
    ],
    \"asks\": [
        [\"16913.21\",\"4.85\",1],
        [\"16918.01\",\"0.70301839\",11],
        [\"16918.02\",\"9.88197274\",24]
    ]
}
        "));
        let expected = OrderBook {
            sequence: 3,
            bids: vec![
                PriceLevel {
                    price: 16839.45,
                    size: 0.47037038,
                    num_order: 2,
                },
                PriceLevel {
                    price: 16835.39,
                    size: 0.00075522,
                    num_order: 2,
                },
            ],
            asks: vec![
                PriceLevel {
                    price: 16913.21,
                    size: 4.85,
                    num_order: 1,
                },
                PriceLevel {
                    price: 16918.01,
                    size: 0.70301839,
                    num_order: 11,
                },
                PriceLevel {
                    price: 16918.02,
                    size: 9.88197274,
                    num_order: 24,
                },

            ],
        };
        assert_eq!(result, expected)
    }

}
