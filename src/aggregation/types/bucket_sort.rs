//! Bucket sort aggregation types.

use serde::{Deserialize, Serialize};

use crate::search::Sort;
#[cfg(feature = "graphql")]
use crate::search::SortInput;

use super::GapPolicy;

/// A parent [**pipeline aggregation**] which sorts the buckets of its parent
/// `multi-bucket` aggregation. Zero or more sort fields may be specified
/// together with the corresponding sort order. Each bucket may be sorted based
/// on its `_key`, `_count` or its sub-aggregations. In addition, parameters
/// from and size may be set in order to truncate the result buckets.
///
/// **Note**: The `bucket_sort` aggregation, like all pipeline aggregations, is
/// executed after all other non-pipeline aggregations. This means the sorting
/// only applies to whatever buckets are already returned from the parent
/// aggregation. For example, if the parent aggregation is `terms` and its
/// `size` is set to `10`, the `bucket_sort` will only sort over those `10`
/// returned term buckets.
///
/// [**pipeline aggregation**]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-pipeline.html
#[cfg(feature = "graphql")]
#[async_graphql::InputObject]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Clone, Debug)]
pub struct BucketSortInput {
    #[field(default)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sort: Vec<SortInput>,

    /// Buckets in positions prior to the set value will be truncated.
    // #[serde(default, skip_serializing_if = "Option::is_none")]
    #[field(default_with = "Some(0)")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<i32>,

    /// The number of buckets to return.
    ///
    /// Defaults to all buckets of the parent aggregation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,

    #[field(default_with = "Some(GapPolicy::Skip)")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gap_policy: Option<GapPolicy>,
}

/// A parent [**pipeline aggregation**] which sorts the buckets of its parent
/// `multi-bucket` aggregation. Zero or more sort fields may be specified
/// together with the corresponding sort order. Each bucket may be sorted based
/// on its `_key`, `_count` or its sub-aggregations. In addition, parameters
/// from and size may be set in order to truncate the result buckets.
///
/// **Note**: The `bucket_sort` aggregation, like all pipeline aggregations, is
/// executed after all other non-pipeline aggregations. This means the sorting
/// only applies to whatever buckets are already returned from the parent
/// aggregation. For example, if the parent aggregation is `terms` and its
/// `size` is set to `10`, the `bucket_sort` will only sort over those `10`
/// returned term buckets.
///
/// [**pipeline aggregation**]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-pipeline.html
#[async_graphql::SimpleObject]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct BucketSort {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sort: Vec<Sort>,

    /// Buckets in positions prior to the set value will be truncated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<i32>,

    /// The number of buckets to return.
    ///
    /// Defaults to all buckets of the parent aggregation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gap_policy: Option<GapPolicy>,
}

#[cfg(feature = "graphql")]
impl From<BucketSortInput> for BucketSort {
    #[inline]
    fn from(input: BucketSortInput) -> Self {
        BucketSort {
            sort: input.sort.into_iter().map(Into::into).collect(),
            from: input.from,
            size: input.size,
            gap_policy: input.gap_policy,
        }
    }
}
