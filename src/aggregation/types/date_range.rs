//! Date range aggregation types.

use serde::{Deserialize, Serialize};

/// A range ([*bucketing*]) aggregation that is dedicated for date values. The
/// main difference between this aggregation and the normal [`range`]
/// aggregation is that the `from` and `to` values can be expressed in [Date
/// Math] expressions, and it is also possible to specify a date `format` by
/// which the from and to response fields will be returned.
///
/// [*bucketing*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html
/// [`range`]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-range-query.html
/// [Date Math]: https://www.elastic.co/guide/en/elasticsearch/reference/current/common-options.html#date-math
#[cfg(feature = "graphql")]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(async_graphql::InputObject, Serialize, Clone, Debug)]
pub struct DateRangeAggregationInput {
    /// The field to perform the aggregation over.
    pub field: String,

    /// Converts from another time zone to UTC.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,

    /// How the returned date should be [formatted].
    ///
    /// [formatted]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-daterange-aggregation.html#date-format-pattern
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    /// The `missing` parameter defines how documents that are missing a value
    /// should be treated. By default they will be ignored but it is also
    /// possible to treat them as if they had a value.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub missing: Option<String>,

    /// The ranges to use for the aggregation.
    #[graphql(default)]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default)]
    pub ranges: Vec<DateRangeInput>,
}

/// A range ([*bucketing*]) aggregation that is dedicated for date values. The
/// main difference between this aggregation and the normal [`range`]
/// aggregation is that the `from` and `to` values can be expressed in [Date
/// Math] expressions, and it is also possible to specify a date `format` by
/// which the from and to response fields will be returned.
///
/// [*bucketing*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html
/// [`range`]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-range-query.html
/// [Date Math]: https://www.elastic.co/guide/en/elasticsearch/reference/current/common-options.html#date-math
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DateRangeAggregation {
    /// The field to perform the aggregation over.
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub field: String,

    /// Converts from another time zone to UTC.ƒ
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,

    /// How the returned date should be [formatted].
    ///
    /// [formatted]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-daterange-aggregation.html#date-format-pattern
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    /// The `missing` parameter defines how documents that are missing a value
    /// should be treated. By default they will be ignored but it is also
    /// possible to treat them as if they had a value.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub missing: Option<String>,

    /// The ranges to use for the aggregation.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default)]
    pub ranges: Vec<DateRange>,
}

#[cfg(feature = "graphql")]
impl From<DateRangeAggregationInput> for DateRangeAggregation {
    #[inline]
    fn from(input: DateRangeAggregationInput) -> Self {
        DateRangeAggregation {
            field: input.field,
            time_zone: input.time_zone,
            format: input.format,
            missing: input.missing,
            ranges: input.ranges.into_iter().map(Into::into).collect(),
        }
    }
}

/// A range/span of dates.
#[cfg(feature = "graphql")]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(async_graphql::InputObject, Serialize, Clone, Debug)]
pub struct DateRangeInput {
    /// The date to return results *from*; supports [Date Math] expressions.
    ///
    /// [Date Math]: https://www.elastic.co/guide/en/elasticsearch/reference/current/common-options.html#date-math
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,

    /// The date to return results up *to*; supports [Date Math] expressions.
    ///
    /// [Date Math]: https://www.elastic.co/guide/en/elasticsearch/reference/current/common-options.html#date-math
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

/// A range/span of dates.
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DateRange {
    /// The date to return results *from*; supports [Date Math] expressions.
    ///
    /// [Date Math]: https://www.elastic.co/guide/en/elasticsearch/reference/current/common-options.html#date-math
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,

    /// The date to return results up *to*; supports [Date Math] expressions.
    ///
    /// [Date Math]: https://www.elastic.co/guide/en/elasticsearch/reference/current/common-options.html#date-math
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

impl DateRange {
    /// Constructs a new `DateRange`.
    #[allow(dead_code)]
    #[inline]
    pub fn new<T>(from: Option<T>, to: Option<T>) -> Self
    where
        T: Into<String>,
    {
        DateRange {
            from: from.map(Into::into),
            to: to.map(Into::into),
        }
    }
}

#[cfg(feature = "graphql")]
impl From<DateRangeInput> for DateRange {
    #[inline]
    fn from(input: DateRangeInput) -> Self {
        DateRange {
            from: input.from,
            to: input.to,
        }
    }
}
