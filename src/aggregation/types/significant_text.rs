//! [Significant text] aggregation types.
//!
//! [Significant text]: https://www.elastic.co/guide/en/elasticsearch/reference/7.x/search-aggregations-bucket-significanttext-aggregation.html

use serde::{Deserialize, Serialize};

/// An aggregation that returns interesting or unusual occurrences of free-text
/// terms in a set.
///
/// See the official documentation for [significant text] for more information.
///
/// [significant text]: https://www.elastic.co/guide/en/elasticsearch/reference/7.x/search-aggregations-bucket-significanttext-aggregation.html
#[cfg(feature = "graphql")]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "builder", builder(field_defaults(setter(into))))]
#[derive(async_graphql::InputObject, Serialize, Clone, Debug)]
pub struct SignificantTextAggregationInput {
    /// The field to perform the aggregation over.
    pub field: String,

    /// The number of term buckets that should be returned out of the overall
    /// terms list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub size: Option<u64>,

    #[allow(missing_docs)]
    #[serde(default)]
    #[graphql(default)]
    #[cfg_attr(feature = "builder", builder(default))]
    pub filter_duplicate_text: bool,
}

/// An aggregation that returns interesting or unusual occurrences of free-text
/// terms in a set.
///
/// See the official documentation for [significant text] for more information.
///
/// [significant text]: https://www.elastic.co/guide/en/elasticsearch/reference/7.x/search-aggregations-bucket-significanttext-aggregation.html
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "builder", builder(field_defaults(setter(into))))]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SignificantTextAggregation {
    /// The field to perform the aggregation over.
    pub field: String,

    /// The number of term buckets that should be returned out of the overall
    /// terms list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub size: Option<u64>,

    #[allow(missing_docs)]
    #[serde(default)]
    #[cfg_attr(feature = "builder", builder(default))]
    pub filter_duplicate_text: bool,
}

#[cfg(feature = "graphql")]
impl From<SignificantTextAggregationInput> for SignificantTextAggregation {
    #[inline]
    fn from(input: SignificantTextAggregationInput) -> Self {
        Self {
            field: input.field,
            size: input.size,
            filter_duplicate_text: input.filter_duplicate_text,
        }
    }
}
