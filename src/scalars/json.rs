//! [JSON] wrapper/[newtype struct].
//!
//! [JSON]: https://tools.ietf.org/html/rfc8259#section-3
//! [newtype struct]: https://doc.rust-lang.org/1.0.0/style/features/types/newtype.html
//! [scalar]: https://graphql.org/learn/schema/#scalar-types

use std::borrow::Borrow;
#[cfg(feature = "graphql")]
use std::convert::{TryFrom, TryInto};
use std::default::Default;
use std::hash::Hash;
use std::str::FromStr;
use std::string::String;

use serde::{Deserialize, Serialize};
use serde_json::map::{
    Entry as JsonMapEntry, Iter as JsonMapIter, IterMut as JsonMapIterMut, Keys as JsonMapKeys,
    Values as JsonMapValues, ValuesMut as JsonMapValuesMut,
};
use serde_json::{Map as JsonMap, Value as JsonValue};

#[cfg(feature = "graphql")]
type GraphQlObject = std::collections::BTreeMap<async_graphql::Name, async_graphql::Value>;

/// A [JSON] object (`key` => `value` map).
///
/// [JSON]: https://tools.ietf.org/html/rfc8259#section-3
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "graphql", derive(async_graphql::Description))]
pub struct Map(JsonMap<String, JsonValue>);

// documentation taken from `serde_json::Map`
impl Map {
    /// Makes a new empty Map.
    #[inline]
    pub fn new() -> Self {
        Map(JsonMap::<String, JsonValue>::new())
    }

    /// Makes a new empty Map with the given initial capacity.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Map(JsonMap::<String, JsonValue>::with_capacity(capacity))
    }

    /// Clears the map, removing all values.
    #[inline]
    pub fn clear(&mut self) {
        self.0.clear()
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but the ordering
    /// on the borrowed form *must* match the ordering on the key type.
    #[inline]
    pub fn get<Q>(&self, key: &Q) -> Option<&JsonValue>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash,
    {
        self.0.get(key)
    }

    /// Returns true if the map contains a value for the specified key.
    ///
    /// The key may be any borrowed form of the map's key type, but the ordering
    /// on the borrowed form *must* match the ordering on the key type.
    #[inline]
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash,
    {
        self.0.contains_key(key)
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but the ordering
    /// on the borrowed form *must* match the ordering on the key type.
    #[inline]
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut JsonValue>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash,
    {
        self.0.get_mut(key)
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, `None` is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned.
    #[inline]
    pub fn insert(&mut self, k: String, v: JsonValue) -> Option<JsonValue> {
        self.0.insert(k, v)
    }

    /// Removes a key from the map, returning the value at the key if the key
    /// was previously in the map.
    ///
    /// The key may be any borrowed form of the map's key type, but the ordering
    /// on the borrowed form *must* match the ordering on the key type.
    #[inline]
    pub fn remove<Q>(&mut self, key: &Q) -> Option<JsonValue>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash,
    {
        self.0.remove(key)
    }

    /// Removes a key from the map, returning the stored key and value if the
    /// key was previously in the map.
    ///
    /// The key may be any borrowed form of the map's key type, but the ordering
    /// on the borrowed form *must* match the ordering on the key type.
    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(String, JsonValue)>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash,
    {
        self.0.remove_entry(key)
    }

    /// Moves all elements from other into Self, leaving other empty.
    #[inline]
    pub fn append(&mut self, other: &mut Self) {
        self.0.append(&mut other.0)
    }

    /// Gets the given key's corresponding entry in the map for in-place
    /// manipulation.
    pub fn entry<S>(&mut self, key: S) -> JsonMapEntry
    where
        S: Into<String>,
    {
        self.0.entry(key)
    }

    /// Returns the number of elements in the map.
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the map contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Gets an iterator over the entries of the map.
    #[inline]
    pub fn iter(&self) -> JsonMapIter {
        self.0.iter()
    }

    /// Gets a mutable iterator over the entries of the map.
    #[inline]
    pub fn iter_mut(&mut self) -> JsonMapIterMut {
        self.0.iter_mut()
    }

    /// Gets an iterator over the keys of the map.
    #[inline]
    pub fn keys(&self) -> JsonMapKeys {
        self.0.keys()
    }

    /// Gets an iterator over the values of the map.
    #[inline]
    pub fn values(&self) -> JsonMapValues {
        self.0.values()
    }

    /// Gets an iterator over mutable values of the map.
    #[inline]
    pub fn values_mut(&mut self) -> JsonMapValuesMut {
        self.0.values_mut()
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

impl From<JsonMap<String, JsonValue>> for Map {
    #[inline]
    fn from(value: JsonMap<String, JsonValue>) -> Self {
        Map(value)
    }
}

#[cfg(feature = "graphql")]
impl TryFrom<GraphQlObject> for Map {
    type Error = serde_json::Error;

    #[inline]
    fn try_from(value: GraphQlObject) -> Result<Self, Self::Error> {
        let result = value
            .into_iter()
            .map(|(k, v)| Ok((k.to_string(), v.try_into()?)))
            .collect::<Result<JsonMap<String, JsonValue>, _>>()?;

        Ok(Map(result))
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
#[async_graphql::Scalar(use_type_description)]
impl async_graphql::ScalarType for Map {
    #[inline]
    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        match value {
            async_graphql::Value::Null => Ok(Map::default()),
            async_graphql::Value::String(val) => Ok(val.parse::<Map>()?),
            async_graphql::Value::Object(val) => Ok(Map::try_from(val)?),
            async_graphql::Value::Number(_)
            | async_graphql::Value::Boolean(_)
            | async_graphql::Value::Enum(_)
            | async_graphql::Value::List(_) => {
                Err(async_graphql::InputValueError::expected_type(value))
            }
        }
    }

    #[inline]
    fn to_value(&self) -> async_graphql::Value {
        // TODO: is there a better way to support custom raw JSON objects?
        let val = &self.0;
        let val: GraphQlObject = val
            .into_iter()
            .map(|(k, v)| {
                let key = async_graphql::Name::new(k);
                let value = v.to_owned().try_into().expect(
                    "invalid JSON encountered when converting a `Map` to a `graphql::Value`",
                );
                (key, value)
            })
            .collect();

        async_graphql::Value::Object(val)
    }
}
