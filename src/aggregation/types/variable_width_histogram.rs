//! [Variable width histogram] aggregation types.
//!
//! [Variable width histogram]: https://www.elastic.co/guide/en/elasticsearch/reference/latest/search-aggregations-bucket-variablewidthhistogram-aggregation.html

use serde::{Deserialize, Serialize};

/// [Variable width histogram] is a [*multi-bucket*] aggregation similar to
/// [histogram]. However, the width of each bucket is not specified. Rather, a
/// target number of buckets is provided and bucket intervals are dynamically
/// determined based on the document distribution. This is done using a simple
/// one-pass document clustering algorithm that aims to obtain low distances
/// between bucket centroids. Unlike other multi-bucket aggregations, the
/// intervals will not necessarily have a uniform width.
///
/// **Note**: until GraphQL [Union input types] are supported, either
/// `calendarInterval` or `fixedInterval` *must* be specified but *not* both.
///
/// [Variable width histogram]: https://www.elastic.co/guide/en/elasticsearch/reference/latest/search-aggregations-bucket-variablewidthhistogram-aggregation.html
/// [Union input types]: https://github.com/graphql/graphql-spec/blob/master/rfcs/InputUnion.md
/// [histogram]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-histogram-aggregation.html
/// [*multi-bucket*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html
#[cfg(feature = "graphql")]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "builder", builder(field_defaults(setter(into))))]
#[derive(async_graphql::InputObject, Serialize, Clone, Debug)]
pub struct VariableWidthHistogramInput {
    /// The field to perform the aggregation over.
    pub field: String,

    /// The target number of buckets.
    pub buckets: u64,
}

/// [Variable width histogram] is a [*multi-bucket*] aggregation similar to
/// [histogram]. However, the width of each bucket is not specified. Rather, a
/// target number of buckets is provided and bucket intervals are dynamically
/// determined based on the document distribution. This is done using a simple
/// one-pass document clustering algorithm that aims to obtain low distances
/// between bucket centroids. Unlike other multi-bucket aggregations, the
/// intervals will not necessarily have a uniform width.
///
/// **Note**: until GraphQL [Union input types] are supported, either
/// `calendarInterval` or `fixedInterval` *must* be specified but *not* both.
///
/// [Variable width histogram]: https://www.elastic.co/guide/en/elasticsearch/reference/latest/search-aggregations-bucket-variablewidthhistogram-aggregation.html
/// [Union input types]: https://github.com/graphql/graphql-spec/blob/master/rfcs/InputUnion.md
/// [histogram]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-histogram-aggregation.html
/// [*multi-bucket*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "builder", builder(field_defaults(setter(into))))]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VariableWidthHistogram {
    /// The field to perform the aggregation over.
    #[cfg_attr(feature = "builder", builder(!default))]
    pub field: String,

    /// The target number of buckets.
    pub buckets: u64,
}

#[cfg(feature = "graphql")]
impl From<VariableWidthHistogramInput> for VariableWidthHistogram {
    #[inline]
    fn from(input: VariableWidthHistogramInput) -> Self {
        Self {
            field: input.field,
            buckets: input.buckets,
        }
    }
}
