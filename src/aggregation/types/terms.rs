//! [Terms] aggregation types.
//!
//! [Terms]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-terms-aggregation.html

use serde::{Deserialize, Serialize};

use crate::search::Script;
#[cfg(feature = "graphql")]
use crate::search::ScriptInput;

// TODO: add `order` field: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-terms-aggregation.html#search-aggregations-bucket-terms-aggregation-order

/// A [*multi-bucketing*] value source based aggregation where buckets are
/// dynamically built - one per unique value.
///
/// **Note**: until GraphQL [Union input types] are supported either `field` or
/// `script` *must* be specified but *not* both.
///
/// [Union input types]: https://github.com/graphql/graphql-spec/blob/master/rfcs/InputUnion.md
/// [*multi-bucketing*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html
#[cfg(feature = "graphql")]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(async_graphql::InputObject, Serialize, Clone, Debug)]
pub struct TermsAggregationInput {
    /// The field to perform the aggregation over.
    pub field: Option<String>,

    /// The script to use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script: Option<ScriptInput>,

    /// The [size parameter] can be set to define how many term buckets should
    /// be returned out of the overall terms list. By default, the node
    /// coordinating the search process will request each shard to provide its
    /// own top size term buckets and once all shards respond, it will reduce
    /// the results to the final list that will then be returned to the client.
    /// This means that if the number of unique terms is greater than size, the
    /// returned list is slightly off and not accurate (it could be that the
    /// term counts are slightly off and it could even be that a term that
    /// should have been in the top size buckets was not returned).
    ///
    /// [size parameter]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-terms-aggregation.html#search-aggregations-bucket-terms-aggregation-size
    #[graphql(default_with = "Some(1_000)")]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,

    /// How missing values should be treated.
    ///
    /// By default they will be ignored, but it is also possible to treat them
    /// as if they had the value.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub missing: Option<f64>,
}

/// A [*multi-bucketing*] value source based aggregation where buckets are
/// dynamically built - one per unique value.
///
/// [*multi-bucketing*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TermsAggregation {
    /// The field to perform the aggregation over.
    pub field: Option<String>,

    /// The script to use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script: Option<Script>,

    /// The [size parameter] can be set to define how many term buckets should
    /// be returned out of the overall terms list. By default, the node
    /// coordinating the search process will request each shard to provide its
    /// own top size term buckets and once all shards respond, it will reduce
    /// the results to the final list that will then be returned to the client.
    /// This means that if the number of unique terms is greater than size, the
    /// returned list is slightly off and not accurate (it could be that the
    /// term counts are slightly off and it could even be that a term that
    /// should have been in the top size buckets was not returned).
    ///
    /// [size parameter]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-terms-aggregation.html#search-aggregations-bucket-terms-aggregation-size
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,

    /// How missing values should be treated.
    ///
    /// By default they will be ignored, but it is also possible to treat them
    /// as if they had the value.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub missing: Option<f64>,
}

#[cfg(feature = "graphql")]
impl From<TermsAggregationInput> for TermsAggregation {
    #[inline]
    fn from(aggregation: TermsAggregationInput) -> Self {
        TermsAggregation {
            field: aggregation.field,
            script: aggregation.script.map(Into::into),
            size: aggregation.size,
            missing: aggregation.missing,
        }
    }
}

#[cfg(test)]
impl<T: Into<String>> From<T> for TermsAggregation {
    #[inline]
    fn from(field: T) -> TermsAggregation {
        TermsAggregation {
            field: Some(field.into()),
            size: None,
            script: None,
            missing: None,
        }
    }
}
