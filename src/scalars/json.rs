//! [JSON] wrapper/[newtype struct].
//!
//! [JSON]: https://tools.ietf.org/html/rfc8259#section-3
//! [newtype struct]: https://doc.rust-lang.org/1.0.0/style/features/types/newtype.html
//! [scalar]: https://graphql.org/learn/schema/#scalar-types

#[cfg(feature = "graphql")]
use std::convert::TryFrom;
use std::borrow::Borrow;
use std::default::Default;
use std::hash::Hash;
use std::str::FromStr;
use std::string::String;

use serde::{Deserialize, Serialize};
use serde_json::{Map as JsonMap, Value as JsonValue};

/// A [JSON] object (`key` => `value` map).
///
/// [JSON]: https://tools.ietf.org/html/rfc8259#section-3
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Map(JsonMap<String, JsonValue>);

impl Map {
    /// Returns a reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but the ordering
    /// on the borrowed form must match the ordering on the key type.
    #[inline]
    pub fn get<Q>(&self, key: &Q) -> Option<&JsonValue>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash,
    {
        self.0.get(key)
    }

    /// Returns true if the map contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Default for Map {
    #[inline]
    fn default() -> Self {
        Map(JsonMap::<String, JsonValue>::new())
    }
}

impl From<JsonValue> for Map {
    #[inline]
    fn from(value: JsonValue) -> Self {
        match value.as_object() {
            Some(val) => Map(val.to_owned()),
            None if value.is_null() => Self::default(),
            None => panic!("invalid JSON object: `{}`", &value),
        }
    }
}

#[cfg(feature = "graphql")]
impl From<std::collections::BTreeMap<String, async_graphql::Value>> for Map {
    #[inline]
    fn from(value: std::collections::BTreeMap<String, async_graphql::Value>) -> Self {
        let result = value
            .into_iter()
            .map(|(k, v)| (k, JsonValue::from(v)))
            .collect();

        Map(result)
    }
}

impl FromStr for Map {
    type Err = serde_json::error::Error;

    #[inline]
    fn from_str(val: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(val)
    }
}

#[cfg(feature = "graphql")]
#[async_graphql::Scalar]
impl async_graphql::ScalarType for Map {
    #[inline]
    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        match value {
            async_graphql::Value::Null => Ok(Map::default()),
            async_graphql::Value::String(val) => Ok(val.parse::<Map>()?),
            async_graphql::Value::Object(val) => Ok(Map::try_from(val)?),
            async_graphql::Value::Variable(_)
            | async_graphql::Value::Int(_)
            | async_graphql::Value::Float(_)
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
        serde_json::json!(&self.0).into()
    }
}
