//! [Reverse Nested aggregation](https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-reverse-nested-aggregation.html)

use serde::{Deserialize, Serialize};

/// A special single [*bucketing*] aggregation that enables aggregating on
/// parent docs from [nested] documents. Effectively this aggregation can break
/// out of the nested block structure and link to other nested structures or the
/// root document, which allows nesting other aggregations that aren’t part of
/// the nested object in a nested aggregation.
///
/// The [`ReverseNestedAggregation`] aggregation must be defined inside a
/// [`nested`] aggregation.
///
/// [*bucketing*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html
/// [nested]: https://www.elastic.co/guide/en/elasticsearch/reference/current/nested.html
/// [`nested`]: https://www.elastic.co/guide/en/elasticsearch/reference/current/nested.html
/// [`ReverseNestedAggregation`]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-reverse-nested-aggregation.html
#[cfg(feature = "graphql")]
#[async_graphql::InputObject]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReverseNestedAggregationInput {
    /// Defines to what [nested] object field should be joined back. The default
    /// is empty, which means that it joins back to the root/main document
    /// level. The path cannot contain a reference to a nested object field that
    /// falls outside the nested aggregation’s nested structure a
    /// [`ReverseNestedAggregation`] is in.
    ///
    /// [nested]: https://www.elastic.co/guide/en/elasticsearch/reference/current/nested.html
    /// [`ReverseNestedAggregation`]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-reverse-nested-aggregation.html
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub path: Option<String>,
}

/// A special single [*bucketing*] aggregation that enables aggregating on
/// parent docs from [nested] documents. Effectively this aggregation can break
/// out of the nested block structure and link to other nested structures or the
/// root document, which allows nesting other aggregations that aren’t part of
/// the nested object in a nested aggregation.
///
/// The [`ReverseNestedAggregation`] aggregation must be defined inside a
/// [`nested`] aggregation.
///
/// [*bucketing*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html
/// [nested]: https://www.elastic.co/guide/en/elasticsearch/reference/current/nested.html
/// [`nested`]: https://www.elastic.co/guide/en/elasticsearch/reference/current/nested.html
/// [`ReverseNestedAggregation`]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-reverse-nested-aggregation.html
#[cfg_attr(feature = "graphql", async_graphql::SimpleObject)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReverseNestedAggregation {
    /// Defines to what [nested] object field should be joined back. The default
    /// is empty, which means that it joins back to the root/main document
    /// level. The path cannot contain a reference to a nested object field that
    /// falls outside the nested aggregation’s nested structure a
    /// [`ReverseNestedAggregation`] is in.
    ///
    /// [nested]: https://www.elastic.co/guide/en/elasticsearch/reference/current/nested.html
    /// [`ReverseNestedAggregation`]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-reverse-nested-aggregation.html
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub path: Option<String>,
}

#[cfg(feature = "graphql")]
impl From<ReverseNestedAggregationInput> for ReverseNestedAggregation {
    #[inline]
    fn from(input: ReverseNestedAggregationInput) -> Self {
        ReverseNestedAggregation { path: input.path }
    }
}
