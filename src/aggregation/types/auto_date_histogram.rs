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
#[cfg_attr(feature = "builder", builder(field_defaults(setter(into))))]
pub struct AutoDateHistogramAggregationInput {
    /// The field to perform the aggregation over.
    pub field: String,

    /// Target number of buckets needed; the interval of the buckets is
    /// automatically chosen to best achieve this target. The number of buckets
    /// returned will always be less than or equal to this target number.
    #[graphql(default_with = "Some(10)")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub buckets: Option<u64>,

    /// Specifies the minimum rounding interval that should be used. This can make
    /// the collection process more efficient, as the aggregation will not attempt
    /// to round at any interval lower than `minimum_interval`.
    #[graphql(default_with = "Some(MinimumInterval::Day)")]
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

    /// Indicates that bucketing and rounding should use a different timezone
    /// than the default UTC.
    ///
    /// Accepts either an [ISO 8601] UTC offset (e.g. `+01:00` or `-08:00`) or as
    /// a timezone ID as specified in the [IANA timezone database], such as
    /// `America/Los_Angeles`.
    ///
    /// [ISO 8601]: https://www.iso.org/iso-8601-date-and-time-format.html
    /// [IANA timezone database]: https://www.iana.org/time-zones
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub time_zone: Option<String>,
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
#[cfg_attr(feature = "builder", builder(field_defaults(setter(into))))]
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

    /// Indicates that bucketing and rounding should use a different timezone
    /// than the default UTC.
    ///
    /// Accepts either an [ISO 8601] UTC offset (e.g. `+01:00` or `-08:00`) or as
    /// a timezone ID as specified in the [IANA timezone database], such as
    /// `America/Los_Angeles`.
    ///
    /// [ISO 8601]: https://www.iso.org/iso-8601-date-and-time-format.html
    /// [IANA timezone database]: https://www.iana.org/time-zones
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
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
            time_zone: input.time_zone,
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
