//! [Match query](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-match-query.html#query-dsl-match-query)

use std::{collections::HashMap, fmt};

use serde::de::{self, MapAccess, Visitor};
use serde::ser::{Serialize, SerializeMap, Serializer};

// TODO: add additional options
/// A [Match query] returns documents that match a provided text, number, date
/// or boolean value. The provided text is analyzed before matching.
///
/// The [Match query] is the standard query for performing a full-text search,
/// including options for fuzzy matching.
///
/// [Match query]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-match-query.html#query-dsl-match-query
#[cfg(feature = "graphql")]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(async_graphql::InputObject, Clone, Debug)]
#[graphql(name = "MatchFilterInput")]
pub struct MatchQueryInput {
    /// The name of the field to query.
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub field: String,

    /// Text, number, boolean value or date you wish to find in the provided
    /// `field`.
    ///
    /// The [match query] analyzes any provided text before performing a search.
    /// This means the match query can search text fields for analyzed tokens
    /// rather than an exact term.
    ///
    /// [match query]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-match-query.html#query-dsl-match-query
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub query: String,
}

#[cfg(feature = "graphql")]
impl MatchQueryInput {
    /// Constructs a new `MatchQueryInput`.
    #[inline]
    pub fn new(field: impl Into<String>, query: impl Into<String>) -> MatchQueryInput {
        MatchQueryInput {
            field: field.into(),
            query: query.into(),
        }
    }
}

#[cfg(feature = "graphql")]
impl From<MatchQuery> for MatchQueryInput {
    #[inline]
    fn from(query: MatchQuery) -> Self {
        Self {
            field: query.field,
            query: query.query,
        }
    }
}

#[cfg(feature = "graphql")]
impl Serialize for MatchQueryInput {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(1))?;

        let mut values: HashMap<&str, &str> = HashMap::new();
        values.insert("query", &self.query);

        map.serialize_entry(&self.field, &values)?;

        map.end()
    }
}

// TODO: add additional options
/// A [Match query] returns documents that match a provided text, number, date
/// or boolean value. The provided text is analyzed before matching.
///
/// The [Match query] is the standard query for performing a full-text search,
/// including options for fuzzy matching.
///
/// [Match query]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-match-query.html#query-dsl-match-query
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
#[cfg_attr(feature = "graphql", graphql(name = "MatchFilter"))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Clone, Debug)]
pub struct MatchQuery {
    /// The name of the field to query.
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub field: String,

    /// Text, number, boolean value or date you wish to find in the provided
    /// `field`.
    ///
    /// The [match query] analyzes any provided text before performing a search.
    /// This means the match query can search text fields for analyzed tokens
    /// rather than an exact term.
    ///
    /// [match query]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-match-query.html#query-dsl-match-query
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub query: String,
}

impl MatchQuery {
    /// Constructs a new `MatchQuery`.
    #[inline]
    pub fn new(field: impl Into<String>, query: impl Into<String>) -> MatchQuery {
        MatchQuery {
            field: field.into(),
            query: query.into(),
        }
    }
}

#[cfg(feature = "graphql")]
impl From<MatchQueryInput> for MatchQuery {
    #[inline]
    fn from(input: MatchQueryInput) -> MatchQuery {
        MatchQuery {
            field: input.field,
            query: input.query,
        }
    }
}

// TODO: re-use the serializer from the input type
impl Serialize for MatchQuery {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(1))?;

        let mut values: HashMap<&str, &str> = HashMap::new();
        values.insert("query", &self.query);

        map.serialize_entry(&self.field, &values)?;

        map.end()
    }
}

/// Visits a `MatchQuery` during deserialization.
struct MatchQueryVisitor;

impl<'de> serde::Deserialize<'de> for MatchQuery {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<MatchQuery, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(MatchQueryVisitor)
    }
}

impl<'de> Visitor<'de> for MatchQueryVisitor {
    type Value = MatchQuery;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a `MatchQuery`")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let field = map
            .next_key::<String>()?
            .ok_or_else(|| de::Error::missing_field("field"))?;

        let values: HashMap<String, String> = map.next_value()?;

        let query = values
            .get("query")
            .ok_or_else(|| de::Error::missing_field("query"))?
            .to_string();

        Ok(MatchQuery { field, query })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;

    macro_rules! test_case {
        ($name:ident : $f:expr, $j:expr) => {
            mod $name {
                use super::*;

                #[test]
                fn can_serialize() {
                    assert_eq!(serde_json::to_value(&$f).unwrap(), $j);
                }

                #[test]
                fn can_deserialize() {
                    assert_eq!(serde_json::from_value::<MatchQuery>($j).unwrap(), $f);
                }
            }
        };
    }

    test_case!(
        simple:
        MatchQuery { field: "testMessage".to_string(), query: "this is a test".to_string() },
        json!({ "testMessage": { "query": "this is a test" } })
    );

    #[test]
    fn missing_query_is_err() {
        // TODO: should we support this Elasticsearch schema?
        let j = r#"{ "message": "missing" }"#;
        assert!(serde_json::from_str::<MatchQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "message": null }"#;
        assert!(serde_json::from_str::<MatchQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "message" }"#;
        assert!(serde_json::from_str::<MatchQuery>(j).is_err(), "{}", &j);
    }

    #[test]
    fn invalid_query_is_err() {
        // TODO: should we support this Elasticsearch schema?

        let j = r#"{ "message": { "query": 1.1 } }"#;
        assert!(serde_json::from_str::<MatchQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "message": { "query": 1 } }"#;
        assert!(serde_json::from_str::<MatchQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "message": { "query": 999 } }"#;
        assert!(serde_json::from_str::<MatchQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "message": { "query": null } }"#;
        assert!(serde_json::from_str::<MatchQuery>(j).is_err(), "{}", &j);
    }
}
