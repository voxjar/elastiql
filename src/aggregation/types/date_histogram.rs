//! Date histogram aggregation types.

use serde::{Deserialize, Serialize};

/// This [*multi-bucket*] aggregation is similar to the normal [histogram], but it
/// can only be used with date or date range values.
///
/// **Note**: until GraphQL [Union input types] are supported, either
/// `calendarInterval` or `fixedInterval` *must* be specified but *not* both.
///
/// [Union input types]: https://github.com/graphql/graphql-spec/blob/master/rfcs/InputUnion.md
/// [histogram]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-histogram-aggregation.html
/// [*multi-bucket*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html
#[cfg(feature = "graphql")]
#[async_graphql::InputObject]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Clone, Debug)]
pub struct DateHistogramAggregationInput {
    /// The field to perform the aggregation over.
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub field: String,

    /// Calendar-aware intervals understand that daylight savings changes the
    /// length of specific days, months have different amounts of days, and leap
    /// seconds can be tacked onto a particular year.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calendar_interval: Option<CalendarInterval>,

    /// In contrast to calendar-aware intervals, fixed intervals are a fixed
    /// number of SI units and never deviate, regardless of where they fall on
    /// the calendar. One second is always composed of `1000ms`. This allows
    /// fixed intervals to be specified in any multiple of the supported units.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_interval: Option<String>,

    /// Indicates that bucketing and rounding should use a different timezone
    /// than the default UTC.
    ///
    /// Accepts either an [ISO 8601] UTC offset (e.g. `+01:00` or `-08:00`) or as
    /// a timezone ID as specified in the [IANA timezone database], such as
    /// `America/Los_Angeles`.
    ///
    /// [ISO 8601]: https://www.iso.org/iso-8601-date-and-time-format.html
    /// [IANA timezone database]: https://www.iana.org/time-zones
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,

    /// Changes the start value of each bucket by the specified positive (`+`)
    /// or negative offset (`-`) duration, such as `1h` for an hour, or `1d` for
    /// a day. See [Time units] for more possible time duration options.
    ///
    /// **Note**: The start `offset` of each bucket is calculated after
    /// `timeZone` adjustments have been made.
    ///
    /// [Time units]: https://www.elastic.co/guide/en/elasticsearch/reference/current/common-options.html#time-units
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,

    /// How the returned date should be [formatted].
    ///
    /// [formatted]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-daterange-aggregation.html#date-format-pattern
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    /// Defines how documents that are missing a value should be treated. By
    /// default they will be ignored but it is also possible to treat them as if
    /// they had a value.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub missing: Option<String>,
}

/// This [*multi-bucket*] aggregation is similar to the normal [histogram], but it
/// can only be used with date or date range values.
///
/// **Note**: until GraphQL [Union input types] are supported, either
/// `calendarInterval` or `fixedInterval` *must* be specified but *not* both.
///
/// [Union input types]: https://github.com/graphql/graphql-spec/blob/master/rfcs/InputUnion.md
/// [histogram]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-histogram-aggregation.html
/// [*multi-bucket*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html
#[cfg_attr(feature = "graphql", async_graphql::SimpleObject)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DateHistogramAggregation {
    /// The field to perform the aggregation over.
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub field: String,

    /// Calendar-aware intervals understand that daylight savings changes the
    /// length of specific days, months have different amounts of days, and leap
    /// seconds can be tacked onto a particular year.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calendar_interval: Option<CalendarInterval>,

    /// In contrast to calendar-aware intervals, fixed intervals are a fixed
    /// number of SI units and never deviate, regardless of where they fall on
    /// the calendar. One second is always composed of `1000ms`. This allows
    /// fixed intervals to be specified in any multiple of the supported units.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_interval: Option<String>,

    /// Indicates that bucketing and rounding should use a different timezone
    /// than the default UTC.
    ///
    /// Accepts either an [ISO 8601] UTC offset (e.g. `+01:00` or `-08:00`) or as
    /// a timezone ID as specified in the [IANA timezone database], such as
    /// `America/Los_Angeles`.
    ///
    /// [ISO 8601]: https://www.iso.org/iso-8601-date-and-time-format.html
    /// [IANA timezone database]: https://www.iana.org/time-zones
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,

    /// Changes the start value of each bucket by the specified positive (`+`)
    /// or negative offset (`-`) duration, such as `1h` for an hour, or `1d` for
    /// a day. See [Time units] for more possible time duration options.
    ///
    /// **Note**: The start `offset` of each bucket is calculated after
    /// `timeZone` adjustments have been made.
    ///
    /// [Time units]: https://www.elastic.co/guide/en/elasticsearch/reference/current/common-options.html#time-units
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,

    /// How the returned date should be [formatted].
    ///
    /// [formatted]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-daterange-aggregation.html#date-format-pattern
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    /// Defines how documents that are missing a value should be treated. By
    /// default they will be ignored but it is also possible to treat them as if
    /// they had a value.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub missing: Option<String>,
}

#[cfg(feature = "graphql")]
impl From<DateHistogramAggregationInput> for DateHistogramAggregation {
    #[inline]
    fn from(input: DateHistogramAggregationInput) -> Self {
        DateHistogramAggregation {
            field: input.field,
            calendar_interval: input.calendar_interval,
            fixed_interval: input.fixed_interval,
            time_zone: input.time_zone,
            offset: input.offset,
            format: input.format,
            missing: input.missing,
        }
    }
}

#[cfg_attr(feature = "graphql", async_graphql::Enum)]
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CalendarInterval {
    /// One *minute* is the interval between `00` seconds of the first minute
    /// and `00` seconds of the following minute in the specified timezone,
    /// compensating for any intervening leap seconds, so that the number of
    /// minutes and seconds past the hour is the same at the start and end.
    Minute,

    /// One *hour* is the interval between `00:00` minutes of the first hour and
    /// `00:00` minutes of the following hour in the specified timezone,
    /// compensating for any intervening leap seconds, so that the number of
    /// minutes and seconds past the hour is the same at the start and end.
    Hour,

    /// One *day* is the interval between the start of the day, at the earliest
    /// possible time, which is usually `00:00:00` (midnight), and the start of
    /// of the following day in the specified timezone, compensating for any
    /// intervening time changes.
    Day,

    /// One *week* is the interval between the start
    /// `day_of_week:hour:minute:second` and the same day of the week and time
    /// of the following week in the specified timezone.
    Week,

    /// One *month* is the interval between the start day of the month and time
    /// of day and the same day of the month and time of the following month in
    /// the specified timezone, so that the day of the month and time of day are
    /// the same at the start and end.
    Month,

    /// One *quarter* is the interval between the start day of the month and
    /// time of day and the same day of the month and time of day three months
    /// later, so that the day of the month and time of day are the same at the
    /// start and end.
    Quarter,

    /// One *year* is the interval between the start day of the month and time
    /// of day and the same day of the month and time of day the following year
    /// in the specified timezone, so that the date and time are the same at the
    /// start and end.
    Year,
}
