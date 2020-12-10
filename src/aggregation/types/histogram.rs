//! [Histogram] aggregation types.
//!
//! [Histogram]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-histogram-aggregation.html

use serde::{Deserialize, Serialize};

/// The [histogram] aggregation is a [*multi-bucket*] aggregation that can be
/// applied on numeric values or numeric range values extracted from the
/// documents. It dynamically builds fixed size (a.k.a. interval) buckets over
/// the values. For example, if the documents have a field that holds a price
/// (numeric), we can configure this aggregation to dynamically build buckets
/// with interval 5 (in case of price it may represent $5). When the aggregation
/// executes, the price field of every document will be evaluated and will be
/// rounded down to its closest bucket - for example, if the price is 32 and the
/// bucket size is 5 then the rounding will yield 30 and thus the document will
/// "fall" into the bucket that is associated with the key 30. To make this more
/// formal, here is the rounding function that is used:
///
/// ```not_rust
/// bucket_key = Math.floor((value - offset) / interval) * interval + offset
///```
///
/// For range values, a document can fall into multiple buckets. The first
/// bucket is computed from the lower bound of the range in the same way as a
/// bucket for a single value is computed. The final bucket is computed in the
/// same way from the upper bound of the range, and the range is counted in all
/// buckets in between and including those two.
///
/// **Note**: until GraphQL [Union input types] are supported, either
/// `calendarInterval` or `fixedInterval` *must* be specified but *not* both.
///
/// [Union input types]: https://github.com/graphql/graphql-spec/blob/master/rfcs/InputUnion.md
/// [histogram]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-histogram-aggregation.html
/// [*multi-bucket*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html
#[cfg(feature = "graphql")]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "builder", builder(field_defaults(default, setter(into))))]
#[derive(async_graphql::InputObject, Serialize, Clone, Debug)]
pub struct HistogramAggregationInput {
    /// The field to perform the aggregation over.
    #[cfg_attr(feature = "builder", builder(!default))]
    pub field: String,

    /// The [histogram] aggregation will sum the counts of each `interval`
    /// computed based on the values.
    ///
    /// [histogram]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-histogram-aggregation.html
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<f64>,

    /// By default the bucket keys start with `0 and then continue in even
    /// spaced steps of `interval`, e.g. if the interval is `10`, the first
    /// three buckets (assuming there is data inside them) will be `[0, 10)`,
    /// `[10, 20)`, `[20, 30)`. The bucket boundaries can be shifted by using
    /// the offset option.
    ///
    /// This can be best illustrated with an example. If there are `10`
    /// documents with values ranging from `5` to `14`, using interval `10` will
    /// result in two buckets with `5` documents each. If an additional offset
    /// `5` is used, there will be only one single bucket `[5, 15)` containing
    /// all the `10` documents.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<f64>,

    /// Defines how documents that are missing a value should be treated. By
    /// default they will be ignored but it is also possible to treat them as if
    /// they had a value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub missing: Option<f64>,

    /// By default the response will fill gaps in the histogram with empty
    /// buckets. To fill the gaps with a different amount, use `min_doc_count`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_doc_count: Option<u64>,

    /// With `extended_bounds` setting, you can "force" the histogram
    /// aggregation to start building buckets on a specific min value and also
    /// keep on building buckets up to a max value (even if there are no
    /// documents anymore). Using `extended_bounds` only makes sense when
    /// min_doc_count is 0 (the empty buckets will never be returned if
    /// min_doc_count is greater than 0).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extended_bounds: Option<HistogramBoundsInput>,

    /// The `hard_bounds` option is a counterpart of extended_bounds and can
    /// limit the range of buckets in the histogram. It is particularly useful
    /// in the case of open data ranges that can result in a very large number
    /// of buckets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hard_bounds: Option<HistogramBoundsInput>,
}

/// The [histogram] aggregation is a [*multi-bucket*] aggregation that can be
/// applied on numeric values or numeric range values extracted from the
/// documents. It dynamically builds fixed size (a.k.a. interval) buckets over
/// the values. For example, if the documents have a field that holds a price
/// (numeric), we can configure this aggregation to dynamically build buckets
/// with interval 5 (in case of price it may represent $5). When the aggregation
/// executes, the price field of every document will be evaluated and will be
/// rounded down to its closest bucket - for example, if the price is 32 and the
/// bucket size is 5 then the rounding will yield 30 and thus the document will
/// "fall" into the bucket that is associated with the key 30. To make this more
/// formal, here is the rounding function that is used:
///
/// ```not_rust
/// bucket_key = Math.floor((value - offset) / interval) * interval + offset
///```
///
/// For range values, a document can fall into multiple buckets. The first
/// bucket is computed from the lower bound of the range in the same way as a
/// bucket for a single value is computed. The final bucket is computed in the
/// same way from the upper bound of the range, and the range is counted in all
/// buckets in between and including those two.
///
/// **Note**: until GraphQL [Union input types] are supported, either
/// `calendarInterval` or `fixedInterval` *must* be specified but *not* both.
///
/// [Union input types]: https://github.com/graphql/graphql-spec/blob/master/rfcs/InputUnion.md
/// [histogram]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-histogram-aggregation.html
/// [*multi-bucket*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "builder", builder(field_defaults(default, setter(into))))]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HistogramAggregation {
    /// The field to perform the aggregation over.
    #[cfg_attr(feature = "builder", builder(!default))]
    pub field: String,

    /// The [histogram] aggregation will sum the counts of each `interval`
    /// computed based on the values.
    ///
    /// [histogram]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-histogram-aggregation.html
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<f64>,

    /// By default the bucket keys start with `0 and then continue in even
    /// spaced steps of `interval`, e.g. if the interval is `10`, the first
    /// three buckets (assuming there is data inside them) will be `[0, 10)`,
    /// `[10, 20)`, `[20, 30)`. The bucket boundaries can be shifted by using
    /// the offset option.
    ///
    /// This can be best illustrated with an example. If there are `10`
    /// documents with values ranging from `5` to `14`, using interval `10` will
    /// result in two buckets with `5` documents each. If an additional offset
    /// `5` is used, there will be only one single bucket `[5, 15)` containing
    /// all the `10` documents.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<f64>,

    /// Defines how documents that are missing a value should be treated. By
    /// default they will be ignored but it is also possible to treat them as if
    /// they had a value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub missing: Option<f64>,

    /// By default the response will fill gaps in the histogram with empty
    /// buckets. To fill the gaps with a different amount, use `min_doc_count`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_doc_count: Option<u64>,

    /// With `extended_bounds` setting, you can "force" the histogram
    /// aggregation to start building buckets on a specific min value and also
    /// keep on building buckets up to a max value (even if there are no
    /// documents anymore). Using `extended_bounds` only makes sense when
    /// min_doc_count is 0 (the empty buckets will never be returned if
    /// min_doc_count is greater than 0).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extended_bounds: Option<HistogramBounds>,

    /// The `hard_bounds` option is a counterpart of extended_bounds and can
    /// limit the range of buckets in the histogram. It is particularly useful
    /// in the case of open data ranges that can result in a very large number
    /// of buckets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hard_bounds: Option<HistogramBounds>,
}

#[cfg(feature = "graphql")]
impl From<HistogramAggregationInput> for HistogramAggregation {
    #[inline]
    fn from(input: HistogramAggregationInput) -> Self {
        Self {
            field: input.field,
            interval: input.interval,
            offset: input.offset,
            missing: input.missing,
            min_doc_count: input.min_doc_count,
            extended_bounds: input.extended_bounds.map(Into::into),
            hard_bounds: input.hard_bounds.map(Into::into),
        }
    }
}

/// Bounds for controlling the `Histogram`.
#[cfg(feature = "graphql")]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "builder", builder(field_defaults(setter(into))))]
#[derive(async_graphql::InputObject, Serialize, Clone, Debug)]
#[allow(missing_docs)]
pub struct HistogramBoundsInput {
    pub min: f64,
    pub max: f64,
}

/// Bounds for controlling the `Histogram`.
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "builder", builder(field_defaults(default, setter(into))))]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(missing_docs)]
pub struct HistogramBounds {
    pub min: f64,
    pub max: f64,
}

#[cfg(feature = "graphql")]
impl From<HistogramBoundsInput> for HistogramBounds {
    #[inline]
    fn from(input: HistogramBoundsInput) -> Self {
        Self {
            min: input.min,
            max: input.max,
        }
    }
}
