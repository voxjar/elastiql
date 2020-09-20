//! [Terms query](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-terms-query.html#query-dsl-terms-query)

use std::fmt;

use serde::de::{self, MapAccess, Visitor};
use serde::ser::{SerializeMap, Serializer};
use serde::Serialize;

/// A [Terms query] returns documents that contain one or more **exact** terms
/// in a provided field.
///
/// [Terms query]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-terms-query.html#query-dsl-terms-query
#[cfg(feature = "graphql")]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(async_graphql::InputObject, Clone, Debug)]
#[graphql(name = "TermsFilterInput")]
pub struct TermsQueryInput {
    /// The name of the field to query.
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub field: String,

    /// A list of terms you wish to find in the provided field. To return a
    /// document, one or more terms must exactly match a field value, including
    /// whitespace and capitalization.
    ///
    /// Although this field is a `String`, it will match `numerical` fields; e.g.
    /// `["1.2"]` will match fields containing the floating point value `1.2`.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub values: Vec<String>,

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
impl TermsQueryInput {
    /// Constructs a new `TermsQueryInput`.
    #[inline]
    pub fn new<T: Into<String>>(
        field: impl Into<String>,
        // TODO: why can't this just be `impl Into<Vec<String>>`?
        values: impl IntoIterator<Item = T>,
    ) -> Self {
        TermsQueryInput {
            field: field.into(),
            values: values.into_iter().map(Into::into).collect::<Vec<String>>(),
            boost: None,
        }
    }
}

#[cfg(feature = "graphql")]
impl Serialize for TermsQueryInput {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry(&self.field, &self.values)?;
        if let Some(boost) = &self.boost {
            map.serialize_entry("boost", &boost)?;
        }
        map.end()
    }
}

/// A [Terms query] returns documents that contain one or more **exact** terms
/// in a provided field.
///
/// [Terms query]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-terms-query.html#query-dsl-terms-query
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
#[cfg_attr(feature = "graphql", graphql(name = "TermsFilter"))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Clone, Debug)]
pub struct TermsQuery {
    /// The name of the field to query.
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub field: String,

    /// A list of terms you wish to find in the provided field. To return a
    /// document, one or more terms must exactly match a field value, including
    /// whitespace and capitalization.
    ///
    /// Although this field is a `String`, it will match `numerical` fields; e.g.
    /// `["1.2"]` will match fields containing the floating point value `1.2`.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub values: Vec<String>,

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

impl TermsQuery {
    /// Constructs a new `TermsQuery`.
    #[inline]
    pub fn new<T: Into<String>>(
        field: impl Into<String>,
        // TODO: why can't this just be `impl Into<Vec<String>>`?
        values: impl IntoIterator<Item = T>,
    ) -> Self {
        TermsQuery {
            field: field.into(),
            values: values.into_iter().map(Into::into).collect::<Vec<String>>(),
            boost: None,
        }
    }
}

#[cfg(feature = "graphql")]
impl From<TermsQueryInput> for TermsQuery {
    #[inline]
    fn from(input: TermsQueryInput) -> TermsQuery {
        TermsQuery {
            field: input.field,
            values: input.values,
            boost: input.boost,
        }
    }
}

// TODO: re-use the serializer from the input type
impl Serialize for TermsQuery {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry(&self.field, &self.values)?;
        if let Some(boost) = &self.boost {
            map.serialize_entry("boost", &boost)?;
        }
        map.end()
    }
}

/// Visits a `TermsQuery` during deserialization.
struct TermsQueryVisitor;

impl<'de> serde::Deserialize<'de> for TermsQuery {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<TermsQuery, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(TermsQueryVisitor)
    }
}

impl<'de> Visitor<'de> for TermsQueryVisitor {
    type Value = TermsQuery;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a `TermsQuery`")
    }

    fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let boost_field = "boost".to_string();

        let mut field: Option<String> = None;
        let mut values: Option<Vec<String>> = None;
        let mut boost: Option<f64> = None;
        while let Some(key) = access.next_key::<String>()? {
            if key == boost_field {
                if boost.is_some() {
                    return Err(de::Error::duplicate_field("boost"));
                }

                boost = Some(access.next_value::<f64>()?);
            } else {
                if field.is_some() {
                    return Err(de::Error::duplicate_field("field"));
                }

                field = Some(key);
                values = Some(access.next_value::<Vec<String>>()?);
            }
        }

        let field = field.ok_or_else(|| de::Error::missing_field("field"))?;
        let values = values.ok_or_else(|| de::Error::missing_field("values"))?;

        Ok(TermsQuery {
            field,
            values,
            boost,
        })
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
                    assert_eq!(serde_json::from_value::<TermsQuery>($j).unwrap(), $f);
                }
            }
        };
    }

    test_case!(
        simple:
        TermsQuery {
            field: "userProfile".to_string(),
            values: vec!["Kimchy".to_string(), "elasticsearch".to_string()],
            boost: None,
        },
        json!({ "userProfile": ["Kimchy", "elasticsearch"] })
    );

    test_case!(
        with_boost:
        TermsQuery {
            field: "user".to_string(),
            values: vec!["Kimchy".to_string(), "elasticsearch".to_string()],
            boost: Some(1.1),
        },
        json!({ "user": ["Kimchy", "elasticsearch"], "boost": 1.1 })
    );

    test_case!(
        without_boost:
        TermsQuery {
            field: "user".to_string(),
            values: vec!["Kimchy".to_string(), "elasticsearch".to_string()],
            boost: None,
        },
        json!({ "user": ["Kimchy", "elasticsearch"] })
    );

    #[test]
    fn deserialize_invalid_boost_is_err() {
        let j = r#"{ "user": { "value": "Kimchy", "boost": "nan" } }"#;
        assert!(serde_json::from_str::<TermsQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "value": "Kimchy", "boost": "asdf" } }"#;
        assert!(serde_json::from_str::<TermsQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "value": "Kimchy", "boost": "1.x" } }"#;
        assert!(serde_json::from_str::<TermsQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "value": "Kimchy", "boost": "x1" } }"#;
        assert!(serde_json::from_str::<TermsQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "value": "Kimchy", "boost": 2.0, "boost": "x1" } }"#;
        assert!(serde_json::from_str::<TermsQuery>(j).is_err(), "{}", &j);
    }

    #[test]
    fn deserialize_missing_values_is_err() {
        // TODO: should we support this Elasticsearch schema?
        let j = r#"{ "user": "missing" }"#;
        assert!(serde_json::from_str::<TermsQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": null }"#;
        assert!(serde_json::from_str::<TermsQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user" }"#;
        assert!(serde_json::from_str::<TermsQuery>(j).is_err(), "{}", &j);
    }

    #[test]
    fn deserialize_invalid_values_is_err() {
        let j = r#"{ "user": { "value": 1.1 } }"#;
        assert!(serde_json::from_str::<TermsQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "value": 1 } }"#;
        assert!(serde_json::from_str::<TermsQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "value": 999 } }"#;
        assert!(serde_json::from_str::<TermsQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "values": [null] } }"#;
        assert!(serde_json::from_str::<TermsQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "values": [1.1] } }"#;
        assert!(serde_json::from_str::<TermsQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "values": [1] } }"#;
        assert!(serde_json::from_str::<TermsQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "values": 999 } }"#;
        assert!(serde_json::from_str::<TermsQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "values": null } }"#;
        assert!(serde_json::from_str::<TermsQuery>(j).is_err(), "{}", &j);
    }
}
