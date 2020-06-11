//! Facilitates [sorting] documents in the database.
//!
//! [sorting]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl.html

use std::default::Default;

use serde::{
    de::{self, Deserializer, Visitor},
    ser::{SerializeMap, Serializer},
    Deserialize, Serialize,
};

/// The [sort order](https://www.elastic.co/guide/en/elasticsearch/reference/current/search-request-sort.html#_sort_order)
#[cfg_attr(feature = "graphql", async_graphql::Enum)]
#[cfg_attr(not(feature = "graphql"), derive(PartialEq, Clone))]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SortOrder {
    /// Sort in ascending order
    Asc,

    /// Sort in descending order
    Desc,
}

/// The [sort mode](https://www.elastic.co/guide/en/elasticsearch/reference/current/search-request-sort.html#_sort_mode_option)
#[cfg_attr(feature = "graphql", async_graphql::Enum)]
#[cfg_attr(not(feature = "graphql"), derive(PartialEq, Clone))]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SortMode {
    /// Pick the lowest value.
    Min,

    /// Pick the highest value.
    Max,

    /// Use the sum of all values as sort value.
    /// Only applicable for number based array fields.
    Sum,

    /// Use the average of all values as sort value.
    /// Only applicable for number based array fields.
    Avg,

    /// Use the median of all values as sort value.
    /// Only applicable for number based array fields.
    Median,
}

/// The options for sorting.
///
/// When querying/searching, you can specify `_score`. For certain types of
/// aggregations you can specify `_key`, `_count` or an aggregation's name.
///
/// **NOTE**: Currently, the same `sort` options used to retrieve a `cursor`
/// **must** be passed in when using that `cursor` (in the `after` argument).
///
/// **NOTE**: the `id` field will always be used as a tie breaker or a default,
/// regardless of any value specified.
#[cfg(feature = "graphql")]
#[async_graphql::InputObject]
#[derive(PartialEq, Clone, Debug)]
pub struct SortInput {
    /// The field to sort by.
    ///
    /// **NOTE**: until [union input types] are supported by GraphQL, it might
    /// be possible to pass in values for `field` that are valid according to
    /// GraphQL but will result in a database error.
    ///
    /// **TODO**: should this be an enum?
    ///
    /// [union input types]: https://github.com/graphql/graphql-spec/blob/master/rfcs/InputUnion.md
    pub field: String,

    /// The order to sort by.
    pub order: Option<SortOrder>,

    /// The mode to sort with.
    pub mode: Option<SortMode>,
}

#[cfg(feature = "graphql")]
impl Default for SortInput {
    /// Returns the "default value" for a `SortInput`, which consists of only
    /// sorting on the tie-breaker field (`id`).
    #[inline]
    fn default() -> Self {
        Self {
            // `id` is the tie-breaker field
            field: "id".to_string(),
            order: None,
            mode: None,
        }
    }
}

#[cfg(feature = "graphql")]
impl Serialize for SortInput {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(1))?;
        // TODO: are there other special fields? should we even do this?
        let field = match self.field.as_str() {
            "score" => "_score",
            "key" => "_key",
            "count" => "_count",
            _ => self.field.as_str(),
        };
        // https://www.elastic.co/guide/en/elasticsearch/reference/current/search-request-body.html#request-body-search-sort
        map.serialize_entry(&field, &InnerSortValue::from(self))?;
        map.end()
    }
}

/// The options for sorting.
///
/// When querying/searching, you can specify `_score`. For certain types of
/// aggregations you can specify `_key`, `_count` or an aggregation's name.
///
/// **NOTE**: Currently, the same `sort` options used to retrieve a `cursor`
/// **must** be passed in when using that `cursor` (in the `after` argument).
///
/// **NOTE**: the `id` field will always be used as a tie breaker or a default,
/// regardless of any value specified.
#[cfg_attr(feature = "graphql", async_graphql::SimpleObject)]
#[derive(PartialEq, Clone, Debug)]
pub struct Sort {
    /// The field to sort by.
    ///
    /// **NOTE**: until [union input types] are supported by GraphQL, it might
    /// be possible to pass in values for `field` that are valid according to
    /// GraphQL but will result in a database error.
    ///
    /// **TODO**: should this be an enum?
    ///
    /// [union input types]: https://github.com/graphql/graphql-spec/blob/master/rfcs/InputUnion.md
    field: String,

    /// The order to sort by.
    order: Option<SortOrder>,

    /// The mode to sort with.
    mode: Option<SortMode>,
}

impl Default for Sort {
    /// Returns the "default value" for a `Sort`, which consists of only
    /// sorting on the tie-breaker field (`id`).
    #[inline]
    fn default() -> Self {
        Self {
            // `id` is the tie-breaker field
            field: "id".to_string(),
            order: None,
            mode: None,
        }
    }
}

#[cfg(feature = "graphql")]
impl From<SortInput> for Sort {
    #[inline]
    fn from(input: SortInput) -> Self {
        Sort {
            field: input.field,
            order: input.order,
            mode: input.mode,
        }
    }
}

// TODO: re-use the serializer from the input type
impl Serialize for Sort {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(1))?;
        // TODO: are there other special fields? should we even do this?
        let field = match self.field.as_str() {
            "score" => "_score",
            "key" => "_key",
            "count" => "_count",
            _ => &self.field,
        };
        // https://www.elastic.co/guide/en/elasticsearch/reference/current/search-request-body.html#request-body-search-sort
        map.serialize_entry(&field, &InnerSortValue::from(self))?;
        map.end()
    }
}

impl<'de> Deserialize<'de> for Sort {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Sort, D::Error>
    where
        D: Deserializer<'de>,
    {
        /// Visits a `Sort` during deserialization.
        struct SortVisitor;

        impl<'de> Visitor<'de> for SortVisitor {
            type Value = Sort;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a `Sort`")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let field = map
                    .next_key::<String>()?
                    .ok_or_else(|| de::Error::missing_field("field"))?;

                let inner: InnerSortValue = map.next_value()?;

                Ok(Sort {
                    field,
                    order: inner.order,
                    mode: inner.mode,
                })
            }
        }

        deserializer.deserialize_map(SortVisitor)
    }
}

/// Sorting criteria in a format suitable for Elasticsearch.
#[derive(Serialize, Deserialize)]
struct InnerSortValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<SortOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<SortMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unmapped_type: Option<String>,
}

#[cfg(feature = "graphql")]
impl From<&SortInput> for InnerSortValue {
    #[inline]
    fn from(sort: &SortInput) -> Self {
        // TODO: make it so we don't have to clone- maybe borrow data in InnerSortValue?
        InnerSortValue {
            order: sort.order,
            mode: sort.mode,
            // HACK: in case the field is one we don't have an index mapping for
            //       see: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-request-body.html#_ignoring_unmapped_fields
            unmapped_type: if sort.field.starts_with('_') {
                None
            } else {
                Some("keyword".to_string())
            },
        }
    }
}

// TODO: re-use from
impl From<&Sort> for InnerSortValue {
    #[inline]
    fn from(sort: &Sort) -> Self {
        // TODO: make it so we don't have to clone- maybe borrow data in InnerSortValue?
        InnerSortValue {
            order: sort.order.clone(),
            mode: sort.mode.clone(),
            // HACK: in case the field is one we don't have an index mapping for
            //       see: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-request-body.html#_ignoring_unmapped_fields
            unmapped_type: if sort.field.starts_with('_') {
                None
            } else {
                Some("keyword".to_string())
            },
        }
    }
}

// TODO: refactor to use macro
#[cfg(test)]
#[allow(clippy::restriction)]
mod tests {
    use super::*;

    use serde_json::json;

    #[test]
    fn can_serialize_default() {
        let f = Sort::default();
        let j = json!({ "id": { "unmapped_type": "keyword" } });
        assert_eq!(serde_json::to_value(&f).unwrap(), j, "{}", &j);
    }

    #[test]
    fn can_serialize_with_order() {
        let sort = vec![
            Sort {
                field: "id".to_string(),
                mode: None,
                order: Some(SortOrder::Asc),
            },
            Sort {
                field: "id".to_string(),
                mode: None,
                order: Some(SortOrder::Desc),
            },
        ];

        let j = json!([
            { "id": { "order": "asc", "unmapped_type": "keyword" } },
            { "id": { "order": "desc", "unmapped_type": "keyword" } },
        ]);

        assert_eq!(serde_json::to_value(&sort).unwrap(), j, "{}", &j);
    }

    #[test]
    fn can_serialize_with_mode() {
        let sorts: Vec<Sort> = vec![
            SortMode::Min,
            SortMode::Max,
            SortMode::Sum,
            SortMode::Avg,
            SortMode::Median,
        ]
        .into_iter()
        .map(|m| Sort {
            field: "id".to_string(),
            mode: Some(m),
            order: None,
        })
        .collect();

        let j = json!([
            { "id": { "mode": "min", "unmapped_type": "keyword" } },
            { "id": { "mode": "max", "unmapped_type": "keyword" } },
            { "id": { "mode": "sum", "unmapped_type": "keyword" } },
            { "id": { "mode": "avg", "unmapped_type": "keyword" } },
            { "id": { "mode": "median", "unmapped_type": "keyword" } },
        ]);

        assert_eq!(serde_json::to_value(&sorts).unwrap(), j, "{}", &j);
    }

    #[test]
    fn can_serialize_with_everything() {
        let sort = Sort {
            field: "id".to_string(),
            mode: Some(SortMode::Max),
            order: Some(SortOrder::Desc),
        };
        let j = json!({ "id": { "mode": "max", "order": "desc", "unmapped_type": "keyword" } });
        assert_eq!(serde_json::to_value(&sort).unwrap(), j, "{}", &j);
    }

    #[test]
    fn can_serialize_with_special_field() {
        let sort = Sort {
            field: "_score".to_string(),
            mode: None,
            order: None,
        };
        let j = json!({ "_score": { } });
        assert_eq!(serde_json::to_value(&sort).unwrap(), j, "{}", &j);

        let sort = Sort {
            field: "_key".to_string(),
            mode: Some(SortMode::Avg),
            order: None,
        };
        let j = json!({ "_key": { "mode": "avg" } });
        assert_eq!(serde_json::to_value(&sort).unwrap(), j, "{}", &j);

        let sort = Sort {
            field: "_count".to_string(),
            mode: None,
            order: None,
        };
        let j = json!({ "_count": { } });
        assert_eq!(serde_json::to_value(&sort).unwrap(), j, "{}", &j);
    }

    #[test]
    fn can_deserialize_with_everything() {
        let j = json!({ "id": { "mode": "max", "order": "desc", "unmapped_type": "keyword" } });
        let actual: Sort = serde_json::from_value(j).unwrap();

        let expected = Sort {
            field: "id".to_string(),
            mode: Some(SortMode::Max),
            order: Some(SortOrder::Desc),
        };

        assert_eq!(actual, expected, "{:#?}", &actual);
    }
}
