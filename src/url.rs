use std::fmt;

use itertools::join;

#[derive(PartialEq, Debug)]
pub struct Route {
    path: Vec<String>,
    query: Vec<AttributeValue>,
}

#[derive(PartialEq, Debug)]
struct AttributeValue {
    attribute: String,
    value: String,
}

impl Route {
    pub fn new() -> Route {
        Route {
            path: Vec::new(),
            query: Vec::new(),
        }
    }

    pub fn add_attribute_value<T, U>(mut self, attribute: &T, value: &U) -> Route
    where
        T: ToString,
        U: ToString,
    {
        self.query.push(AttributeValue {
            attribute: attribute.to_string(),
            value: value.to_string(),
        });
        self
    }

    pub fn add_segment<T: ToString>(mut self, segment: &T) -> Route {
        self.path.push(segment.to_string());
        self
    }
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let path = format!("/{}", join(&self.path, "/"));
        let query = if !self.query.is_empty() {
            format!("?{}", join(&self.query, "&"))
        } else {
            String::new()
        };
        write!(f, "{}{}", &path, &query)
    }
}

impl fmt::Display for AttributeValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}={}", &self.attribute, &self.value)
    }
}

#[cfg(test)]
mod tests {
    use url::Route;

    #[test]
    fn test_simple_route_string() {
        let result = Route::new().add_segment(&String::from("seg")).to_string();

        let expected = String::from("/seg");

        assert_eq!(result, expected);
    }

    #[test]
    fn test_complex_route_string() {
        let result = Route::new()
            .add_segment(&"seg1")
            .add_segment(&String::from("seg2"))
            .add_segment(&String::from("seg3"))
            .add_attribute_value(&String::from("attr1"), &String::from("1"))
            .add_attribute_value(&String::from("attr2"), &String::from("2"))
            .add_attribute_value(&String::from("attr3"), &String::from("3"))
            .to_string();

        let expected = String::from("/seg1/seg2/seg3?attr1=1&attr2=2&attr3=3");

        assert_eq!(result, expected);
    }
}
