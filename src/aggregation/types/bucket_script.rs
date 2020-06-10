//! Bucket script aggregation types.

use serde::{Deserialize, Serialize};

use super::GapPolicy;

/// A parent [*pipeline aggregation*] which executes a [script] which can
/// perform per bucket computations on specified metrics in the parent
/// multi-bucket aggregation. The specified metric must be numeric and the
/// script must return a numeric value.
///
/// [*pipeline aggregation*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-pipeline.html
/// [script]: https://www.elastic.co/guide/en/elasticsearch/reference/current/modules-scripting.html
#[cfg(feature = "graphql")]
#[async_graphql::InputObject]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Clone, Debug)]
pub struct BucketScriptInput {
    /// The script to run for this aggregation.
    pub script: String,

    /// A map of script variables and their associated path to the buckets to
    /// use for the variable (see [`buckets_path` Syntax] for more details)
    ///
    /// [`buckets_path` Syntax]: /// https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-pipeline.html#buckets-path-syntax
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buckets_path: Option<crate::scalars::Map>,

    /// The policy to apply when gaps are found in the data
    #[field(default_with = "Some(GapPolicy::Skip)")]
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap_policy: Option<GapPolicy>,

    /// Format to apply to the output value of this aggregation
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

/// A parent [*pipeline aggregation*] which executes a [script] which can
/// perform per bucket computations on specified metrics in the parent
/// multi-bucket aggregation. The specified metric must be numeric and the
/// script must return a numeric value.
///
/// [*pipeline aggregation*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-pipeline.html
/// [script]: https://www.elastic.co/guide/en/elasticsearch/reference/current/modules-scripting.html
#[cfg_attr(feature = "graphql", async_graphql::SimpleObject)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct BucketScript {
    /// The script to run for this aggregation.
    pub script: String,

    /// A map of script variables and their associated path to the buckets to
    /// use for the variable (see [`buckets_path` Syntax] for more details)
    ///
    /// [`buckets_path` Syntax]: /// https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-pipeline.html#buckets-path-syntax
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buckets_path: Option<crate::scalars::Map>,

    /// The policy to apply when gaps are found in the data
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gap_policy: Option<GapPolicy>,

    /// Format to apply to the output value of this aggregation
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

#[cfg(feature = "graphql")]
impl From<BucketScriptInput> for BucketScript {
    #[inline]
    fn from(input: BucketScriptInput) -> Self {
        BucketScript {
            script: input.script,
            buckets_path: input.buckets_path,
            gap_policy: input.gap_policy,
            format: input.format,
        }
    }
}
