//! A scalar that represents a number or a string.

use serde::{Deserialize, Serialize};

/// An int, float or a string value.
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(untagged)]
pub enum SortedValue {
    Null,
    Int(u64),
    Float(f64),
    String(String),
}

impl From<u8> for SortedValue {
    #[inline]
    fn from(val: u8) -> Self {
        SortedValue::Int(val as u64)
    }
}

impl From<u32> for SortedValue {
    #[inline]
    fn from(val: u32) -> Self {
        SortedValue::Int(val as u64)
    }
}

impl From<u64> for SortedValue {
    #[inline]
    fn from(val: u64) -> Self {
        SortedValue::Int(val)
    }
}

impl From<f64> for SortedValue {
    #[inline]
    fn from(val: f64) -> Self {
        SortedValue::Float(val)
    }
}

impl From<String> for SortedValue {
    #[inline]
    fn from(val: String) -> Self {
        SortedValue::String(val)
    }
}

#[cfg(feature = "graphql")]
#[async_graphql::Scalar]
impl async_graphql::ScalarType for SortedValue {
    #[inline]
    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        match value {
            async_graphql::Value::Null => Ok(SortedValue::Null),
            async_graphql::Value::Number(ref val) => {
                if let Some(v) = val.as_u64() {
                    Ok(v.into())
                } else if let Some(v) = val.as_f64() {
                    if v < 0.0 {
                        Err(async_graphql::InputValueError::expected_type(value))
                    } else {
                        Ok(v.into())
                    }
                } else {
                    Err(async_graphql::InputValueError::expected_type(value))
                }
            }
            async_graphql::Value::String(val) => Ok(SortedValue::String(val)),
            async_graphql::Value::Object(_)
            | async_graphql::Value::Boolean(_)
            | async_graphql::Value::Enum(_)
            | async_graphql::Value::List(_) => {
                Err(async_graphql::InputValueError::expected_type(value))
            }
        }
    }

    #[inline]
    fn to_value(&self) -> async_graphql::Value {
        match *self {
            SortedValue::Null => async_graphql::Value::Null,
            SortedValue::Int(val) => async_graphql::Value::Number(val.into()),
            SortedValue::Float(val) => {
                let val = async_graphql::Number::from_f64(val).unwrap_or_else(|| {
                    // `NaN` and `infinite` values are not valid JSON
                    panic!(
                        "invalid JSON float value: `{}` encountered when \
                         converting a `ScoreValue` to a `graphql::Value`",
                        val
                    )
                });
                async_graphql::Value::Number(val)
            }
            SortedValue::String(ref val) => async_graphql::Value::String(val.clone()),
        }
    }
}

#[cfg(test)]
#[cfg(feature = "graphql")]
mod tests {
    use super::*;

    use async_graphql::{ScalarType, Value as GraphQLValue};
    use serde_json::Number as JsonNumber;

    #[test]
    fn can_parse_null() {
        let val = SortedValue::parse(GraphQLValue::Null).unwrap();
        assert_eq!(val, SortedValue::Null);
    }

    #[test]
    fn can_parse_string() {
        let x: String = "x".to_string();
        let val = SortedValue::parse(GraphQLValue::String(x.clone())).unwrap();
        assert_eq!(val, SortedValue::String(x));
    }

    #[test]
    fn can_parse_u8() {
        let x: u8 = 101;
        let val = SortedValue::parse(GraphQLValue::Number(x.into())).unwrap();
        assert_eq!(val, SortedValue::from(x));
    }

    #[test]
    fn can_parse_u32() {
        let x: u32 = 101;
        let val = SortedValue::parse(GraphQLValue::Number(x.into())).unwrap();
        assert_eq!(val, SortedValue::from(x));
    }

    #[test]
    fn can_parse_u64() {
        let x: u64 = 101;
        let val = SortedValue::parse(GraphQLValue::Number(x.into())).unwrap();
        assert_eq!(val, SortedValue::Int(x));
    }

    // TODO: implement `Eq` to assert we received the correct error
    #[test]
    fn parse_negative_is_err() {
        let json_number = JsonNumber::from_f64(-0.00000000000001).unwrap();
        let result = SortedValue::parse(GraphQLValue::Number(json_number));
        // use assert instead of `#[should_panic]` so we keep the output pretty with `--no-capture`
        assert!(result.is_err());

        let x: i64 = -101;
        let result = SortedValue::parse(GraphQLValue::Number(x.into()));
        // use assert instead of `#[should_panic]` so we keep the output pretty with `--no-capture`
        assert!(result.is_err());
    }

    #[test]
    fn can_parse_f64() {
        let x: f64 = 101.0;
        let json_number = JsonNumber::from_f64(x).unwrap();
        let val = SortedValue::parse(GraphQLValue::Number(json_number)).unwrap();
        assert_eq!(val, SortedValue::Float(x));
    }
}
