//! [Regexp query](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-regexp-query.html#query-dsl-regexp-query)

use std::{collections::HashMap, fmt};

use serde::{
    de::{self, MapAccess, Visitor},
    ser::{Serialize, SerializeMap, Serializer},
};

/// A [Regexp query] returns documents that contain terms matching a
/// [regular expression].
///
/// A [regular expression] is a way to match patterns in data using placeholder
/// characters, called operators. For a list of operators supported by the
/// regexp query, see [Regular expression syntax].
///
/// [Regexp query]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-regexp-query.html#query-dsl-regexp-query
/// [regular expression]: https://en.wikipedia.org/wiki/Regular_expression
/// [Regular expression syntax]: https://www.elastic.co/guide/en/elasticsearch/reference/current/regexp-syntax.html
#[cfg(feature = "graphql")]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(async_graphql::InputObject, Clone, Debug)]
#[graphql(name = "RegexpFilterInput")]
pub struct RegexpQueryInput {
    /// The name of the field to query.
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub field: String,

    /// Regular expression for terms you wish to find in the provided `field`.
    /// For a list of supported operators, see [Regular expression syntax].
    ///
    /// [Regular expression syntax]: https://www.elastic.co/guide/en/elasticsearch/reference/current/regexp-syntax.html#regexp-syntax
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub value: String,

    /// Enables optional operators for the regular expression. For valid values
    /// and more information, see [Regular expression syntax].
    ///
    /// To enable multiple operators, use a `|` separator. For example, a flags
    /// value of `COMPLEMENT|INTERVAL` enables the `COMPLEMENT` and `INTERVAL`
    /// operators.
    ///
    /// [Regular expression syntax]: https://www.elastic.co/guide/en/elasticsearch/reference/current/regexp-syntax.html#regexp-optional-operators
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub flags: Option<String>,
}

#[cfg(feature = "graphql")]
impl RegexpQueryInput {
    /// Constructs a new `RegexpQueryInput`.
    #[inline]
    pub fn new(
        field: impl Into<String>,
        value: impl Into<String>,
        flags: Option<impl Into<String>>,
    ) -> RegexpQueryInput {
        RegexpQueryInput {
            field: field.into(),
            value: value.into(),
            flags: flags.map(Into::into),
        }
    }
}

#[cfg(feature = "graphql")]
impl Serialize for RegexpQueryInput {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(1))?;

        let mut values: HashMap<&str, &str> = HashMap::new();

        values.insert("value", &self.value);

        if let Some(flags) = &self.flags {
            values.insert("flags", flags);
        }

        map.serialize_entry(&self.field, &values)?;

        map.end()
    }
}

/// A [Regexp query] returns documents that contain terms matching a
/// [regular expression].
///
/// A [regular expression] is a way to match patterns in data using placeholder
/// characters, called operators. For a list of operators supported by the
/// regexp query, see [Regular expression syntax].
///
/// [Regexp query]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-regexp-query.html#query-dsl-regexp-query
/// [regular expression]: https://en.wikipedia.org/wiki/Regular_expression
/// [Regular expression syntax]: https://www.elastic.co/guide/en/elasticsearch/reference/current/regexp-syntax.html
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
#[cfg_attr(feature = "graphql", graphql(name = "RegexpFilter"))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Clone, Debug)]
pub struct RegexpQuery {
    /// The name of the field to query.
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub field: String,

    /// Regular expression for terms you wish to find in the provided `field`.
    /// For a list of supported operators, see [Regular expression syntax].
    ///
    /// [Regular expression syntax]: https://www.elastic.co/guide/en/elasticsearch/reference/current/regexp-syntax.html#regexp-syntax
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub value: String,

    /// Enables optional operators for the regular expression. For valid values
    /// and more information, see [Regular expression syntax].
    ///
    /// To enable multiple operators, use a `|` separator. For example, a flags
    /// value of `COMPLEMENT|INTERVAL` enables the `COMPLEMENT` and `INTERVAL`
    /// operators.
    ///
    /// [Regular expression syntax]: https://www.elastic.co/guide/en/elasticsearch/reference/current/regexp-syntax.html#regexp-optional-operators
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub flags: Option<String>,
}

impl RegexpQuery {
    /// Constructs a new `RegexpQuery`.
    #[inline]
    pub fn new(
        field: impl Into<String>,
        value: impl Into<String>,
        flags: Option<impl Into<String>>,
    ) -> RegexpQuery {
        RegexpQuery {
            field: field.into(),
            value: value.into(),
            flags: flags.map(Into::into),
        }
    }
}

#[cfg(feature = "graphql")]
impl From<RegexpQueryInput> for RegexpQuery {
    #[inline]
    fn from(input: RegexpQueryInput) -> RegexpQuery {
        RegexpQuery {
            field: input.field,
            value: input.value,
            flags: input.flags,
        }
    }
}

// TODO: re-use the serializer from the input type
impl Serialize for RegexpQuery {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(1))?;

        let mut values: HashMap<&str, &str> = HashMap::new();

        values.insert("value", &self.value);

        // TODO: should we check for invalid flags?
        if let Some(flags) = &self.flags {
            values.insert("flags", flags);
        }

        map.serialize_entry(&self.field, &values)?;

        map.end()
    }
}

/// Visits a `RegexpQuery` during deserialization.
struct RegexpQueryVisitor;

impl<'de> serde::Deserialize<'de> for RegexpQuery {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<RegexpQuery, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(RegexpQueryVisitor)
    }
}

impl<'de> Visitor<'de> for RegexpQueryVisitor {
    type Value = RegexpQuery;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a `RegexpQuery`")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let field = map
            .next_key::<String>()?
            .ok_or_else(|| de::Error::missing_field("field"))?;

        let values: HashMap<String, String> = map.next_value()?;

        let value = values
            .get("value")
            .ok_or_else(|| de::Error::missing_field("value"))?
            .to_string();

        // TODO: should we check for invalid flags?
        let flags = values.get("flags").cloned();

        Ok(RegexpQuery {
            field,
            value,
            flags,
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
                    assert_eq!(serde_json::from_value::<RegexpQuery>($j).unwrap(), $f);
                }
            }
        };
    }

    test_case!(
        simple:
        RegexpQuery {
            field: "userProfile".to_string(),
            value: "k.*y".to_string(),
            flags: None,
        },
        json!({ "userProfile": { "value": "k.*y" } })
    );

    test_case!(
        with_flags:
        RegexpQuery {
            field: "user".to_string(),
            value: "k.*y".to_string(),
            flags: Some("ALL".to_string()),
        },
        json!({ "user": { "value": "k.*y", "flags": "ALL" } })
    );

    test_case!(
        without_flags:
        RegexpQuery {
            field: "user".to_string(),
            value: "k.*y".to_string(),
            flags: None,
        },
        json!({ "user": { "value": "k.*y" } })
    );

    #[test]
    fn deserialize_missing_values_is_err() {
        // TODO: should we support this Elasticsearch schema?
        let j = r#"{ "user": "missing" }"#;
        assert!(serde_json::from_str::<RegexpQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": null }"#;
        assert!(serde_json::from_str::<RegexpQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user" }"#;
        assert!(serde_json::from_str::<RegexpQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "value": null } }"#;
        assert!(serde_json::from_str::<RegexpQuery>(j).is_err(), "{}", &j);
    }

    #[test]
    fn deserialize_invalid_values_is_err() {
        // TODO: should we support this Elasticsearch schema?

        let j = r#"{ "user": { "value": 1.1 } }"#;
        assert!(serde_json::from_str::<RegexpQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "value": 1 } }"#;
        assert!(serde_json::from_str::<RegexpQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "value": 999 } }"#;
        assert!(serde_json::from_str::<RegexpQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "value": null } }"#;
        assert!(serde_json::from_str::<RegexpQuery>(j).is_err(), "{}", &j);
    }

    #[test]
    fn deserialize_invalid_flags_is_err() {
        let j = r#"{ "user": { "flags": 1.1 } }"#;
        assert!(serde_json::from_str::<RegexpQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "flags": 1 } }"#;
        assert!(serde_json::from_str::<RegexpQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "flags": 999 } }"#;
        assert!(serde_json::from_str::<RegexpQuery>(j).is_err(), "{}", &j);

        let j = r#"{ "user": { "flags": null } }"#;
        assert!(serde_json::from_str::<RegexpQuery>(j).is_err(), "{}", &j);
    }
}
