//! Auto-interval Date Histogram Aggregation types.

use serde::{Deserialize, Serialize};

/// A [*multi-bucket*] aggregation similar to the [Date histogram aggregation]
/// except instead of providing an interval to use as the width of each bucket,
/// a target number of buckets is provided indicating the number of buckets
/// needed and the interval of the buckets is automatically chosen to best
/// achieve that target. The number of buckets returned will always be less than
/// or equal to this target number.
///
/// [Date histogram aggregation]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-datehistogram-aggregation.html
/// [*multi-bucket*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html
#[cfg(feature = "graphql")]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(async_graphql::InputObject, Serialize, Clone, Debug)]
pub struct AutoDateHistogramAggregationInput {
    /// The field to perform the aggregation over.
    pub field: String,

    /// Target number of buckets needed; the interval of the buckets is
    /// automatically chosen to best achieve this target. The number of buckets
    /// returned will always be less than or equal to this target number.
    #[field(default_with = "Some(10)")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub buckets: Option<u64>,

    /// Specifies the minimum rounding interval that should be used. This can make
    /// the collection process more efficient, as the aggregation will not attempt
    /// to round at any interval lower than `minimum_interval`.
    #[field(default_with = "Some(MinimumInterval::Day)")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub minimum_interval: Option<MinimumInterval>,

    /// How the returned date should be [formatted].
    ///
    /// [formatted]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-daterange-aggregation.html#date-format-pattern
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub format: Option<String>,

    /// Defines how documents that are missing a value should be treated. By
    /// default they will be ignored but it is also possible to treat them as if
    /// they had a value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub missing: Option<String>,
}

/// A [*multi-bucket*] aggregation similar to the [Date histogram aggregation]
/// except instead of providing an interval to use as the width of each bucket,
/// a target number of buckets is provided indicating the number of buckets
/// needed and the interval of the buckets is automatically chosen to best
/// achieve that target. The number of buckets returned will always be less than
/// or equal to this target number.
///
/// [Date histogram aggregation]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-datehistogram-aggregation.html
/// [*multi-bucket*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AutoDateHistogramAggregation {
    /// The field to perform the aggregation over.
    pub field: String,

    /// Target number of buckets needed; the interval of the buckets is
    /// automatically chosen to best achieve this target. The number of buckets
    /// returned will always be less than or equal to this target number.
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buckets: Option<u64>,

    /// Specifies the minimum rounding interval that should be used. This can make
    /// the collection process more efficient, as the aggregation will not attempt
    /// to round at any interval lower than `minimum_interval`.
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_interval: Option<MinimumInterval>,

    /// How the returned date should be [formatted].
    ///
    /// [formatted]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-daterange-aggregation.html#date-format-pattern
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    /// Defines how documents that are missing a value should be treated. By
    /// default they will be ignored but it is also possible to treat them as if
    /// they had a value.
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub missing: Option<String>,
}

#[cfg(feature = "graphql")]
impl From<AutoDateHistogramAggregationInput> for AutoDateHistogramAggregation {
    #[inline]
    fn from(input: AutoDateHistogramAggregationInput) -> Self {
        AutoDateHistogramAggregation {
            field: input.field,
            buckets: input.buckets,
            minimum_interval: input.minimum_interval,
            format: input.format,
            missing: input.missing,
        }
    }
}

/// Specifies the minimum rounding interval that should be used. This can make
/// the collection process more efficient, as the aggregation will not attempt
/// to round at any interval lower than `minimum_interval`.
#[allow(missing_docs)]
#[cfg_attr(all(test, not(feature = "graphql")), derive(PartialEq))]
#[cfg_attr(feature = "graphql", derive(async_graphql::Enum, Eq, PartialEq, Copy))]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum MinimumInterval {
    Second,
    Minute,
    Hour,
    Day,
    Month,
    Year,
}
