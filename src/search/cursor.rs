//! [GraphQL scalar] for a `Cursor` for paginating database results.
//!
//! [GraphQL scalar]: https://graphql.org/learn/schema/#scalar-types

use std::{
    convert::TryFrom,
    default::Default,
    str::{self, FromStr},
};

use serde::{
    de::{Deserialize, Deserializer},
    Serialize,
};
use serde_json::Value as JsonValue;

// TODO:
// TODO: figure out better way, also this should only `String` | `Number`
pub(crate) type InnerCursor = Vec<JsonValue>;

/// An [opaque cursor] (obtained from the `cursor` field on an `Edge`) to return
/// results after. This facilitates results pagination according to the
/// [Relay Cursor Connections Specification].
///
/// **Note**: it is expected that this will be used in conjunction with the same
/// `sort` argument that created this `Cursor`.
///
/// [opaque cursor]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-request-search-after.html
/// [Relay Cursor Connections Specification]: https://facebook.github.io/relay/graphql/connections.htm
#[derive(Serialize, Default, PartialEq, Clone, Debug)]
pub struct Cursor(InnerCursor);

impl Cursor {
    /// Encodes this `Cursor` to a [base64] [JSON] string.
    ///
    /// [base64]: https://tools.ietf.org/html/rfc4648#section-5
    /// [JSON]: https://tools.ietf.org/html/rfc7159
    #[inline]
    pub fn encode(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // NB: must use inner data, otherwise an endless fn call cycle happens
        Ok(base64::encode(&serde_json::to_string(&self.0)?))
    }

    /// Returns true if the `Cursor` contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns true if the `Cursor` contains no elements.
    pub(crate) fn is_empty_or_default(&self) -> bool {
        self.is_empty() || *self == Default::default()
    }
}

impl From<InnerCursor> for Cursor {
    #[inline]
    fn from(sort: InnerCursor) -> Self {
        Cursor(sort)
    }
}

impl FromStr for Cursor {
    type Err = Box<dyn std::error::Error + Send + Sync>;

    /// Parses a `Cursor` from a string.
    #[inline]
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_slice(&base64::decode(value)?)?)
    }
}

impl<T: Into<Cursor>> From<Option<T>> for Cursor {
    /// Performs the conversion.
    #[inline]
    fn from(cursor: Option<T>) -> Self {
        cursor.map_or_else(Default::default, Into::<Self>::into)
    }
}

// TODO: impl<T: Into<String>> TryFrom<T> for Cursor
impl TryFrom<String> for Cursor {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    /// Performs the conversion.
    #[inline]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

// TODO: figure out associated Error type to do:
//       `impl<T: TryInto<Cursor>> TryFrom<Option<T>> for Cursor`
impl TryFrom<Option<String>> for Cursor {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    #[inline]
    fn try_from(value: Option<String>) -> Result<Self, Self::Error> {
        value.map_or_else(|| Ok(Default::default()), |v| v.parse())
    }
}

impl<'de> Deserialize<'de> for Cursor {
    // Deserializes a `Cursor` from a base64 encoded JSON string.
    #[inline]
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let inner_cursor: InnerCursor = Deserialize::deserialize(deserializer)?;
        Ok(Cursor::from(inner_cursor))
    }
}

#[async_graphql::Scalar]
impl async_graphql::ScalarType for Cursor {
    #[inline]
    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        match value {
            async_graphql::Value::String(val) => Ok(Cursor::try_from(val)?),
            async_graphql::Value::Null
            | async_graphql::Value::Variable(..)
            | async_graphql::Value::Int(..)
            | async_graphql::Value::Float(..)
            | async_graphql::Value::Boolean(..)
            | async_graphql::Value::Enum(..)
            | async_graphql::Value::List(..)
            | async_graphql::Value::Object(..)
            | async_graphql::Value::Upload(..) => {
                Err(async_graphql::InputValueError::ExpectedType(value))
            }
        }
    }

    #[inline]
    fn to_value(&self) -> async_graphql::Value {
        // TODO: `Cursor::to_value` should not be unwrapped ?
        #[allow(clippy::expect_used)]
        async_graphql::Value::String(self.encode().expect("error encoding Cursor to base64"))
    }
}
