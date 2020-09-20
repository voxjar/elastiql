//! [Range query](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-range-query.html#query-dsl-range-query)

use std::fmt;

use serde::{
    de::{self, MapAccess, Visitor},
    ser::{SerializeMap, Serializer},
    Deserialize, Serialize,
};

// TODO: should we present shortened or actual names via graphql? e.g. lt or less_than?

#[allow(clippy::missing_docs_in_private_items)]
#[derive(Serialize, Deserialize)]
struct InnerRangeQuery {
    #[serde(rename = "gt", default, skip_serializing_if = "Option::is_none")]
    greater_than: Option<String>,

    #[serde(rename = "gte", default, skip_serializing_if = "Option::is_none")]
    greater_than_or_equal_to: Option<String>,

    #[serde(rename = "lt", default, skip_serializing_if = "Option::is_none")]
    less_than: Option<String>,

    #[serde(rename = "lte", default, skip_serializing_if = "Option::is_none")]
    less_than_or_equal_to: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    boost: Option<f64>,
}

/// A [Range query] returns documents that contain terms within a provided range.
///
/// [Range query]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-range-query.html#query-dsl-range-query
#[cfg(feature = "graphql")]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(async_graphql::InputObject, Clone, Debug)]
#[graphql(name = "RangeFilterInput")]
pub struct RangeQueryInput {
    /// The name of the field to query.
    pub field: String,

    /// Greater than.
    ///
    /// Although this field is a `String`, it will match `numerical` fields; e.g.
    /// `"1.2"` will match fields containing the floating point value `1.2`.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub greater_than: Option<String>,

    /// Greater than or equal to.
    ///
    /// Although this field is a `String`, it will match `numerical` fields; e.g.
    /// `"1.2"` will match fields containing the floating point value `1.2`.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub greater_than_or_equal_to: Option<String>,

    /// Less than.
    ///
    /// Although this field is a `String`, it will match `numerical` fields; e.g.
    /// `"1.2"` will match fields containing the floating point value `1.2`.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub less_than: Option<String>,

    /// Less than or equal to.
    ///
    /// Although this field is a `String`, it will match `numerical` fields; e.g.
    /// `"1.2"` will match fields containing the floating point value `1.2`.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub less_than_or_equal_to: Option<String>,

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
impl Serialize for RangeQueryInput {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(1))?;

        let inner = InnerRangeQuery {
            greater_than: self.greater_than.as_ref().map(|v| v.to_owned()),
            greater_than_or_equal_to: self.greater_than_or_equal_to.as_ref().map(|v| v.to_owned()),
            less_than: self.less_than.as_ref().map(|v| v.to_owned()),
            less_than_or_equal_to: self.less_than_or_equal_to.as_ref().map(|v| v.to_owned()),
            boost: self.boost,
        };

        map.serialize_entry(&self.field, &inner)?;

        map.end()
    }
}

/// A [Range query] returns documents that contain terms within a provided range.
///
/// [Range query]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-range-query.html#query-dsl-range-query
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
#[cfg_attr(feature = "graphql", graphql(name = "RangeFilter"))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Clone, Debug)]
pub struct RangeQuery {
    /// The name of the field to query.
    pub field: String,

    /// Greater than.
    ///
    /// Although this field is a `String`, it will match `numerical` fields; e.g.
    /// `"1.2"` will match fields containing the floating point value `1.2`.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub greater_than: Option<String>,

    /// Greater than or equal to.
    ///
    /// Although this field is a `String`, it will match `numerical` fields; e.g.
    /// `"1.2"` will match fields containing the floating point value `1.2`.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub greater_than_or_equal_to: Option<String>,

    /// Less than.
    ///
    /// Although this field is a `String`, it will match `numerical` fields; e.g.
    /// `"1.2"` will match fields containing the floating point value `1.2`.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub less_than: Option<String>,

    /// Less than or equal to.
    ///
    /// Although this field is a `String`, it will match `numerical` fields; e.g.
    /// `"1.2"` will match fields containing the floating point value `1.2`.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub less_than_or_equal_to: Option<String>,

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
impl From<RangeQueryInput> for RangeQuery {
    #[inline]
    fn from(input: RangeQueryInput) -> RangeQuery {
        RangeQuery {
            field: input.field,
            greater_than: input.greater_than,
            greater_than_or_equal_to: input.greater_than_or_equal_to,
            less_than: input.less_than,
            less_than_or_equal_to: input.less_than_or_equal_to,
            boost: input.boost,
        }
    }
}

// TODO: re-use the serializer from the input type
impl Serialize for RangeQuery {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(1))?;

        let inner = InnerRangeQuery {
            greater_than: self.greater_than.as_ref().map(|v| v.to_owned()),
            greater_than_or_equal_to: self.greater_than_or_equal_to.as_ref().map(|v| v.to_owned()),
            less_than: self.less_than.as_ref().map(|v| v.to_owned()),
            less_than_or_equal_to: self.less_than_or_equal_to.as_ref().map(|v| v.to_owned()),
            boost: self.boost,
        };

        map.serialize_entry(&self.field, &inner)?;

        map.end()
    }
}

/// Visits a `RangeQuery` during deserialization.
struct RangeQueryVisitor;

impl<'de> serde::Deserialize<'de> for RangeQuery {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<RangeQuery, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(RangeQueryVisitor)
    }
}

impl<'de> Visitor<'de> for RangeQueryVisitor {
    type Value = RangeQuery;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a `RangeQuery`")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let field = map
            .next_key::<String>()?
            .ok_or_else(|| de::Error::missing_field("field"))?;

        let inner: InnerRangeQuery = map.next_value()?;

        let filter = RangeQuery {
            field,
            greater_than: inner.greater_than,
            greater_than_or_equal_to: inner.greater_than_or_equal_to,
            less_than: inner.less_than,
            less_than_or_equal_to: inner.less_than_or_equal_to,
            boost: inner.boost,
        };

        Ok(filter)
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
                    assert_eq!(serde_json::from_value::<RangeQuery>($j).unwrap(), $f);
                }
            }
        };
    }

    test_case!(
        simple:
        RangeQuery {
            field: "currentAge".to_string(),
            greater_than: None,
            greater_than_or_equal_to: Some("10".to_string()),
            less_than: None,
            less_than_or_equal_to: Some("20".to_string()),
            boost: None,
        },
        json!({ "currentAge": { "gte": "10", "lte": "20" } })
    );

    test_case!(
        with_boost:
        RangeQuery {
            field: "age".to_string(),
            greater_than: None,
            greater_than_or_equal_to: Some("10".to_string()),
            less_than: None,
            less_than_or_equal_to: Some("20".to_string()),
            boost: Some(2.0),
        },
        json!({ "age": { "gte": "10", "lte": "20", "boost": 2.0 } })
    );

    test_case!(
        without_boost:
        RangeQuery {
            field: "age".to_string(),
            greater_than: None,
            greater_than_or_equal_to: Some("10".to_string()),
            less_than: None,
            less_than_or_equal_to: Some("20".to_string()),
            boost: None,
        },
        json!({ "age": { "gte": "10", "lte": "20" } })
    );

    #[test]
    fn deserialize_invalid_boost_is_err() {
        let j = r#"{ "age": { "gte": "10", "lte": "20", "boost": "nan" } }"#;
        assert!(
            serde_json::from_str::<RangeQuery>(j).is_err(),
            "test case: {}",
            &j
        );

        let j = r#"{ "age": { "gte": "10", "lte": "20", "boost": "asdf" } }"#;
        assert!(
            serde_json::from_str::<RangeQuery>(j).is_err(),
            "test case: {}",
            &j
        );

        let j = r#"{ "age": { "gte": "10", "lte": "20", "boost": "1.x" } }"#;
        assert!(
            serde_json::from_str::<RangeQuery>(j).is_err(),
            "test case: {}",
            &j
        );

        let j = r#"{ "age": { "gte": "10", "lte": "20", "boost": "x1" } }"#;
        assert!(
            serde_json::from_str::<RangeQuery>(j).is_err(),
            "test case: {}",
            &j
        );

        let j = r#"{ "age": { "gte": "10", "lte": "20", "boost": "2.0", "boost": "x1" } }"#;
        assert!(
            serde_json::from_str::<RangeQuery>(j).is_err(),
            "test case: {}",
            &j
        );
    }

    #[test]
    fn deserialize_missing_all_ranges_is_err() {
        let j = r#"{ "age": "missing" }"#;
        assert!(
            serde_json::from_str::<RangeQuery>(j).is_err(),
            "test case: {}",
            &j
        );

        let j = r#"{ "age": null }"#;
        assert!(
            serde_json::from_str::<RangeQuery>(j).is_err(),
            "test case: {}",
            &j
        );

        let j = r#"{ "age" }"#;
        assert!(
            serde_json::from_str::<RangeQuery>(j).is_err(),
            "test case: {}",
            &j
        );
    }

    #[test]
    fn deserialize_invalid_ranges_is_err() {
        // TODO: should we support this Elasticsearch schema?
        let j = r#"{ "age": { "gt": 1.1 } }"#;
        assert!(
            serde_json::from_str::<RangeQuery>(j).is_err(),
            "test case: {}",
            &j
        );

        let j = r#"{ "age": { "gte": 1 } }"#;
        assert!(
            serde_json::from_str::<RangeQuery>(j).is_err(),
            "test case: {}",
            &j
        );

        let j = r#"{ "age": { "lt": 999 } }"#;
        assert!(
            serde_json::from_str::<RangeQuery>(j).is_err(),
            "test case: {}",
            &j
        );
    }
}
