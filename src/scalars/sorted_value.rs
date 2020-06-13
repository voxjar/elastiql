//! A scalar that represents a number or a string.

use serde::{Deserialize, Serialize};

/// An int, float or a string value.
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(untagged)]
pub enum SortedValue {
    Null,
    Int(i32),
    Float(f64),
    String(String),
}

impl From<i32> for SortedValue {
    fn from(val: i32) -> Self {
        SortedValue::Int(val)
    }
}

impl From<f64> for SortedValue {
    fn from(val: f64) -> Self {
        SortedValue::Float(val)
    }
}

impl From<String> for SortedValue {
    fn from(val: String) -> Self {
        SortedValue::String(val)
    }
}

#[cfg(feature = "graphql")]
#[cfg(feature = "graphql")]
#[async_graphql::Scalar]
impl async_graphql::ScalarType for SortedValue {
    #[inline]
    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        match value {
            async_graphql::Value::Null => Ok(SortedValue::Null),
            async_graphql::Value::Int(val) => Ok(SortedValue::Int(val)),
            async_graphql::Value::Float(val) => Ok(SortedValue::Float(val)),
            async_graphql::Value::String(val) => Ok(SortedValue::String(val)),
            async_graphql::Value::Object(_)
            | async_graphql::Value::Variable(_)
            | async_graphql::Value::Boolean(_)
            | async_graphql::Value::Enum(_)
            | async_graphql::Value::List(_)
            | async_graphql::Value::Upload(_) => {
                Err(async_graphql::InputValueError::ExpectedType(value))
            }
        }
    }

    #[inline]
    fn to_value(&self) -> async_graphql::Value {
        match self {
            SortedValue::Null => async_graphql::Value::Null,
            SortedValue::Int(val) => async_graphql::Value::Int(*val),
            SortedValue::Float(val) => async_graphql::Value::Float(*val),
            SortedValue::String(val) => async_graphql::Value::String(val.clone()),
        }
    }
}
