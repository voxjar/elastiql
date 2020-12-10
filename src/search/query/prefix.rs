//! [Prefix query](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-term-query.html#query-dsl-term-query)

use std::fmt;

use serde::de::{self, MapAccess, Visitor};
use serde::ser::{SerializeMap, Serializer};
use serde::{Deserialize, Serialize};

#[allow(clippy::missing_docs_in_private_items)]
#[derive(Serialize, Deserialize)]
struct InnerPrefixQuery {
    value: String,
    #[serde(default)]
    case_insensitive: bool,
}

/// A [Prefix query] returns documents that contain a specific prefix in a
/// provided field.
///
/// [Prefix query]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-prefix-query.html
#[cfg(feature = "graphql")]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(async_graphql::InputObject, Clone, Debug)]
#[graphql(name = "PrefixFilterInput")]
pub struct PrefixQueryInput {
    /// The name of the field to query.
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub field: String,

    /// Beginning characters of terms you wish to find in the provided `field`.
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub value: String,

    /// Allows ASCII case insensitive matching of the value with the indexed
    /// field values when set to true. Default is false which means the case
    /// sensitivity of matching depends on the underlying field’s mapping.
    #[graphql(default)]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub case_insensitive: bool,
}

#[cfg(feature = "graphql")]
impl PrefixQueryInput {
    /// Constructs a new `PrefixQueryInput`.
    #[inline]
    pub fn new(field: impl Into<String>, value: impl Into<String>) -> Self {
        PrefixQueryInput {
            field: field.into(),
            value: value.into(),
            case_insensitive: false,
        }
    }
}

#[cfg(feature = "graphql")]
impl From<PrefixQuery> for PrefixQueryInput {
    #[inline]
    fn from(query: PrefixQuery) -> Self {
        Self {
            field: query.field,
            value: query.value,
            case_insensitive: query.case_insensitive,
        }
    }
}

#[cfg(feature = "graphql")]
impl Serialize for PrefixQueryInput {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(1))?;

        let inner = InnerPrefixQuery {
            value: self.value.to_owned(),
            case_insensitive: self.case_insensitive,
        };

        map.serialize_entry(&self.field, &inner)?;

        map.end()
    }
}

/// A [Prefix query] returns documents that contain a specific prefix in a
/// provided field.
///
/// [Prefix query]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-prefix-query.html
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
#[cfg_attr(feature = "graphql", graphql(name = "PrefixFilter"))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Clone, Debug)]
pub struct PrefixQuery {
    /// The name of the field to query.
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub field: String,

    /// Beginning characters of terms you wish to find in the provided `field`.
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub value: String,

    /// Allows ASCII case insensitive matching of the value with the indexed
    /// field values when set to true. Default is false which means the case
    /// sensitivity of matching depends on the underlying field’s mapping.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub case_insensitive: bool,
}

impl PrefixQuery {
    /// Constructs a new `PrefixQuery`.
    #[inline]
    pub fn new(field: impl Into<String>, value: impl Into<String>) -> Self {
        PrefixQuery {
            field: field.into(),
            value: value.into(),
            case_insensitive: false,
        }
    }
}

#[cfg(feature = "graphql")]
impl From<PrefixQueryInput> for PrefixQuery {
    #[inline]
    fn from(input: PrefixQueryInput) -> PrefixQuery {
        PrefixQuery {
            field: input.field,
            value: input.value,
            case_insensitive: input.case_insensitive,
        }
    }
}

// TODO: re-use the serializer from the input type
impl Serialize for PrefixQuery {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(1))?;

        let inner = InnerPrefixQuery {
            value: self.value.to_owned(),
            case_insensitive: self.case_insensitive,
        };

        map.serialize_entry(&self.field, &inner)?;

        map.end()
    }
}

/// Visits a `PrefixQuery` during deserialization.
struct PrefixQueryVisitor;

impl<'de> serde::Deserialize<'de> for PrefixQuery {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<PrefixQuery, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(PrefixQueryVisitor)
    }
}

impl<'de> Visitor<'de> for PrefixQueryVisitor {
    type Value = PrefixQuery;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a `PrefixQuery`")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let field = map
            .next_key::<String>()?
            .ok_or_else(|| de::Error::missing_field("field"))?;

        let inner: InnerPrefixQuery = map.next_value()?;

        Ok(PrefixQuery {
            field,
            value: inner.value.to_owned(),
            case_insensitive: inner.case_insensitive,
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
                    assert_eq!(serde_json::from_value::<PrefixQuery>($j).unwrap(), $f);
                }
            }
        };
    }

    test_case!(
        simple:
        PrefixQuery {
            field: "userProfile".to_string(),
            value: "Kimchy".to_string(),
            case_insensitive: false,
        },
        json!({ "userProfile": { "value": "Kimchy", "case_insensitive": false } })
    );

    test_case!(
        with_case_insensitive:
        PrefixQuery {
            field: "user".to_string(),
            value: "Kimchy".to_string(),
            case_insensitive: true,
        },
        json!({ "user": { "value": "Kimchy", "case_insensitive": true } })
    );

    test_case!(
        without_case_insensitive:
        PrefixQuery {
            field: "user".to_string(),
            value: "Kimchy".to_string(),
            case_insensitive: false,
        },
        json!({ "user": { "value": "Kimchy", "case_insensitive": false } })
    );

    #[test]
    fn deserialize_invalid_case_insensitive_is_err() {
        let j = r#"{ "user": { "value": "Kimchy", "case_insensitive": "nan" } }"#;
        assert!(serde_json::from_str::<PrefixQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "value": "Kimchy", "case_insensitive": "asdf" } }"#;
        assert!(serde_json::from_str::<PrefixQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "value": "Kimchy", "case_insensitive": "1.x" } }"#;
        assert!(serde_json::from_str::<PrefixQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "value": "Kimchy", "case_insensitive": "x1" } }"#;
        assert!(serde_json::from_str::<PrefixQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "value": "Kimchy", "case_insensitive": 2.0, "case_insensitive": "x1" } }"#;
        assert!(serde_json::from_str::<PrefixQuery>(j).is_err(), "{}", &j);
    }

    #[test]
    fn deserialize_missing_values_is_err() {
        // TODO: should we support this Elasticsearch schema?
        let j = r#"{ "user": "missing" }"#;
        assert!(serde_json::from_str::<PrefixQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": null }"#;
        assert!(serde_json::from_str::<PrefixQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user" }"#;
        assert!(serde_json::from_str::<PrefixQuery>(j).is_err(), "{}", &j);
    }

    #[test]
    fn deserialize_invalid_values_is_err() {
        let j = r#"{ "user": { "value": null } }"#;
        assert!(serde_json::from_str::<PrefixQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "value": 1.1 } }"#;
        assert!(serde_json::from_str::<PrefixQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "value": 1 } }"#;
        assert!(serde_json::from_str::<PrefixQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "value": 999 } }"#;
        assert!(serde_json::from_str::<PrefixQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "value": [null] } }"#;
        assert!(serde_json::from_str::<PrefixQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "value": ["Kimchy"] } }"#;
        assert!(serde_json::from_str::<PrefixQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "value": ["Kimchy", "elasticsearch"] } }"#;
        assert!(serde_json::from_str::<PrefixQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "value": [1.1] } }"#;
        assert!(serde_json::from_str::<PrefixQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "value": [1] } }"#;
        assert!(serde_json::from_str::<PrefixQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "value": [999] } }"#;
        assert!(serde_json::from_str::<PrefixQuery>(j).is_err(), "{}", &j);
    }
}
