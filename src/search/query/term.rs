//! [Term query](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-term-query.html#query-dsl-term-query)

use std::fmt;

use serde::{
    de::{self, MapAccess, Visitor},
    ser::{SerializeMap, Serializer},
    Deserialize, Serialize,
};

#[allow(clippy::missing_docs_in_private_items)]
#[derive(Serialize, Deserialize)]
struct InnerTermQuery {
    value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    boost: Option<f64>,
}

/// A [Term query] returns documents that contain an **exact** term in a provided field.
///
/// [Term query]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-term-query.html#query-dsl-term-query
#[cfg(feature = "graphql")]
#[async_graphql::InputObject(name = "TermFilterInput")]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Clone, Debug)]
pub struct TermQueryInput {
    /// The name of the field to query.
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub field: String,

    /// A list of terms you wish to find in the provided field. To return a
    /// document, one or more terms must exactly match a field value, including
    /// whitespace and capitalization.
    ///
    /// Although this field is a `String`, it will match `numerical` fields; e.g.
    /// `"1.2"` will match fields containing the floating point value `1.2`.
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub value: String,

    /// Floating point number used to decrease or increase the
    /// [relevance scores] of a query. (Defaults to `1.0`.)
    ///
    /// You can use the boost parameter to adjust relevance scores for searches
    /// containing two or more queries.
    ///
    /// Boost values are relative to the default value of `1.0`. A boost value
    /// between `0` and `1.0` decreases the relevance score. A value greater
    /// than  `1.0` increases the relevance score.
    ///
    /// [relevance scores]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html#relevance-scores
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub boost: Option<f64>,
}

#[cfg(feature = "graphql")]
impl TermQueryInput {
    /// Create a new `TermQueryInput`.
    #[inline]
    pub fn new(field: impl Into<String>, value: impl Into<String>) -> Self {
        TermQueryInput {
            field: field.into(),
            value: value.into(),
            boost: None,
        }
    }
}

#[cfg(feature = "graphql")]
impl Serialize for TermQueryInput {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(1))?;

        let inner = InnerTermQuery {
            value: self.value.to_owned(),
            boost: self.boost,
        };

        map.serialize_entry(&self.field, &inner)?;

        map.end()
    }
}

/// A [Term query] returns documents that contain an **exact** term in a provided field.
///
/// [Term query]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-term-query.html#query-dsl-term-query
#[cfg_attr(feature = "graphql", async_graphql::SimpleObject(name = "TermFilter"))]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Clone, Debug)]
pub struct TermQuery {
    /// The name of the field to query.
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub field: String,

    /// A list of terms you wish to find in the provided field. To return a
    /// document, one or more terms must exactly match a field value, including
    /// whitespace and capitalization.
    ///
    /// Although this field is a `String`, it will match `numerical` fields; e.g.
    /// `"1.2"` will match fields containing the floating point value `1.2`.
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub value: String,

    /// Floating point number used to decrease or increase the
    /// [relevance scores] of a query. (Defaults to `1.0`.)
    ///
    /// You can use the boost parameter to adjust relevance scores for searches
    /// containing two or more queries.
    ///
    /// Boost values are relative to the default value of `1.0`. A boost value
    /// between `0` and `1.0` decreases the relevance score. A value greater
    /// than  `1.0` increases the relevance score.
    ///
    /// [relevance scores]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html#relevance-scores
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub boost: Option<f64>,
}

#[cfg(feature = "graphql")]
impl From<TermQueryInput> for TermQuery {
    #[inline]
    fn from(input: TermQueryInput) -> TermQuery {
        TermQuery {
            field: input.field,
            value: input.value,
            boost: input.boost,
        }
    }
}

// TODO: re-use the serializer from the input type
impl Serialize for TermQuery {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(1))?;

        let inner = InnerTermQuery {
            value: self.value.to_owned(),
            boost: self.boost,
        };

        map.serialize_entry(&self.field, &inner)?;

        map.end()
    }
}

/// Visits a `TermQuery` during deserialization.
struct TermQueryVisitor;

impl<'de> serde::Deserialize<'de> for TermQuery {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<TermQuery, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(TermQueryVisitor)
    }
}

impl<'de> Visitor<'de> for TermQueryVisitor {
    type Value = TermQuery;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a `TermQuery`")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let field = map
            .next_key::<String>()?
            .ok_or_else(|| de::Error::missing_field("field"))?;

        let inner: InnerTermQuery = map.next_value()?;

        Ok(TermQuery {
            field,
            value: inner.value.to_owned(),
            boost: inner.boost,
        })
    }
}

#[cfg(test)]
#[allow(clippy::restriction)]
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
                    assert_eq!(serde_json::from_value::<TermQuery>($j).unwrap(), $f);
                }
            }
        };
    }

    test_case!(
        simple:
        TermQuery {
            field: "userProfile".to_string(),
            value: "Kimchy".to_string(),
            boost: None,
        },
        json!({ "userProfile": { "value": "Kimchy" } })
    );

    test_case!(
        with_boost:
        TermQuery {
            field: "user".to_string(),
            value: "Kimchy".to_string(),
            boost: Some(1.1),
        },
        json!({ "user": { "value": "Kimchy", "boost": 1.1 } })
    );

    test_case!(
        without_boost:
        TermQuery {
            field: "user".to_string(),
            value: "Kimchy".to_string(),
            boost: None,
        },
        json!({ "user": { "value": "Kimchy" } })
    );

    #[test]
    fn deserialize_invalid_boost_is_err() {
        let j = r#"{ "user": { "value": "Kimchy", "boost": "nan" } }"#;
        assert!(serde_json::from_str::<TermQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "value": "Kimchy", "boost": "asdf" } }"#;
        assert!(serde_json::from_str::<TermQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "value": "Kimchy", "boost": "1.x" } }"#;
        assert!(serde_json::from_str::<TermQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "value": "Kimchy", "boost": "x1" } }"#;
        assert!(serde_json::from_str::<TermQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "value": "Kimchy", "boost": 2.0, "boost": "x1" } }"#;
        assert!(serde_json::from_str::<TermQuery>(j).is_err(), "{}", &j);
    }

    #[test]
    fn deserialize_missing_values_is_err() {
        // TODO: should we support this Elasticsearch schema?
        let j = r#"{ "user": "missing" }"#;
        assert!(serde_json::from_str::<TermQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": null }"#;
        assert!(serde_json::from_str::<TermQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user" }"#;
        assert!(serde_json::from_str::<TermQuery>(j).is_err(), "{}", &j);
    }

    #[test]
    fn deserialize_invalid_values_is_err() {
        let j = r#"{ "user": { "value": null } }"#;
        assert!(serde_json::from_str::<TermQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "value": 1.1 } }"#;
        assert!(serde_json::from_str::<TermQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "value": 1 } }"#;
        assert!(serde_json::from_str::<TermQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "value": 999 } }"#;
        assert!(serde_json::from_str::<TermQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "value": [null] } }"#;
        assert!(serde_json::from_str::<TermQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "value": ["Kimchy"] } }"#;
        assert!(serde_json::from_str::<TermQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "value": ["Kimchy", "elasticsearch"] } }"#;
        assert!(serde_json::from_str::<TermQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "value": [1.1] } }"#;
        assert!(serde_json::from_str::<TermQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "value": [1] } }"#;
        assert!(serde_json::from_str::<TermQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "value": [999] } }"#;
        assert!(serde_json::from_str::<TermQuery>(j).is_err(), "{}", &j);
    }
}
