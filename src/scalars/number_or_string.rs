//! A scalar that represents a number or a string.

use serde::{Deserialize, Serialize};

/// An int, float or a string value.
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(untagged)]
pub enum NumberOrString {
    Int(i32),
    Float(f64),
    String(String),
}

impl From<i32> for NumberOrString {
    fn from(val: i32) -> Self {
        NumberOrString::Int(val)
    }
}

impl From<f64> for NumberOrString {
    fn from(val: f64) -> Self {
        NumberOrString::Float(val)
    }
}

impl From<String> for NumberOrString {
    fn from(val: String) -> Self {
        NumberOrString::String(val)
    }
}

#[cfg(feature = "graphql")]
#[cfg(feature = "graphql")]
#[async_graphql::Scalar]
impl async_graphql::ScalarType for NumberOrString {
    #[inline]
    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        match value {
            async_graphql::Value::Int(val) => Ok(NumberOrString::Int(val)),
            async_graphql::Value::Float(val) => Ok(NumberOrString::Float(val)),
            async_graphql::Value::String(val) => Ok(NumberOrString::String(val)),
            async_graphql::Value::Null
            | async_graphql::Value::Object(_)
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
            NumberOrString::Int(val) => async_graphql::Value::Int(*val),
            NumberOrString::Float(val) => async_graphql::Value::Float(*val),
            NumberOrString::String(val) => async_graphql::Value::String(val.clone()),
        }
    }
}
