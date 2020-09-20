//! Bucket selector aggregation types.

use serde::{Deserialize, Serialize};

use super::GapPolicy;

/// A parent [*pipeline aggregation*] which executes a [script] which
/// determines whether the current bucket will be retained in the parent
/// multi-bucket aggregation. The specified metric must be numeric and the
/// script must return a boolean value.
///
/// [*pipeline aggregation*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-pipeline.html
/// [script]: https://www.elastic.co/guide/en/elasticsearch/reference/current/modules-scripting.html
#[cfg(feature = "graphql")]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(async_graphql::InputObject, Serialize, Clone, Debug)]
pub struct BucketSelectorInput {
    /// The script to run for this aggregation.
    pub script: String,

    /// A map of script variables and their associated path to the buckets to
    /// use for the variable (see [`buckets_path` Syntax] for more details)
    ///
    /// [`buckets_path` Syntax]: /// https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-pipeline.html#buckets-path-syntax
    pub buckets_path: crate::scalars::Map,

    /// The policy to apply when gaps are found in the data
    #[field(default_with = "Some(GapPolicy::Skip)")]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap_policy: Option<GapPolicy>,
}

/// A parent [*pipeline aggregation*] which executes a [script] which
/// determines whether the current bucket will be retained in the parent
/// multi-bucket aggregation. The specified metric must be numeric and the
/// script must return a boolean value.
///
/// [*pipeline aggregation*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-pipeline.html
/// [script]: https://www.elastic.co/guide/en/elasticsearch/reference/current/modules-scripting.html
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BucketSelector {
    /// The script to run for this aggregation.
    pub script: String,

    /// A map of script variables and their associated path to the buckets to
    /// use for the variable (see [`buckets_path` Syntax] for more details)
    ///
    /// [`buckets_path` Syntax]: /// https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-pipeline.html#buckets-path-syntax
    pub buckets_path: crate::scalars::Map,

    /// The policy to apply when gaps are found in the data
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gap_policy: Option<GapPolicy>,
}

#[cfg(feature = "graphql")]
impl From<BucketSelectorInput> for BucketSelector {
    #[inline]
    fn from(input: BucketSelectorInput) -> Self {
        BucketSelector {
            script: input.script,
            buckets_path: input.buckets_path,
            gap_policy: input.gap_policy,
        }
    }
}
