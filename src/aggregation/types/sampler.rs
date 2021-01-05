//! [Sampler] aggregation types.
//!
//! [Sampler]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-sampler-aggregation.html

use serde::{Deserialize, Serialize};

/// A filtering aggregation used to limit any sub aggregations' processing to a
/// sample of the top-scoring documents.
#[cfg(feature = "graphql")]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "builder", builder(field_defaults(setter(into))))]
#[derive(async_graphql::InputObject, Serialize, Clone, Debug)]
pub struct SamplerAggregationInput {
    #[allow(missing_docs)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub shard_size: Option<u64>,
}

/// A filtering aggregation used to limit any sub aggregations' processing to a
/// sample of the top-scoring documents.
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SamplerAggregation {
    #[allow(missing_docs)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub shard_size: Option<u64>,
}

#[cfg(feature = "graphql")]
impl From<SamplerAggregationInput> for SamplerAggregation {
    #[inline]
    fn from(input: SamplerAggregationInput) -> Self {
        Self {
            shard_size: input.shard_size,
        }
    }
}
