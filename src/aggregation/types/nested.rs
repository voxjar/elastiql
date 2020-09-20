//! [Nested aggregation](https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-nested-aggregation.html)

use serde::{Deserialize, Serialize};

/// A special single [*bucketing*] aggregation that enables aggregating [nested]
/// documents.
///
/// [*bucketing*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html
/// [nested]: https://www.elastic.co/guide/en/elasticsearch/reference/current/nested.html
#[cfg(feature = "graphql")]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(async_graphql::InputObject, Serialize, Clone, Debug)]
pub struct NestedAggregationInput {
    /// The nested path to search.
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub path: String,
}

/// A special single [*bucketing*] aggregation that enables aggregating [nested]
/// documents.
///
/// [*bucketing*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html
/// [nested]: https://www.elastic.co/guide/en/elasticsearch/reference/current/nested.html
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NestedAggregation {
    /// The nested path to search.
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub path: String,
}

#[cfg(feature = "graphql")]
impl From<NestedAggregationInput> for NestedAggregation {
    #[inline]
    fn from(input: NestedAggregationInput) -> Self {
        NestedAggregation { path: input.path }
    }
}
