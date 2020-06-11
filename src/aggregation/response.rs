//! Response types used when [aggregating] documents.
//!
//! [aggregating]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations.html

use serde::Deserialize;

pub(crate) use super::serialization_deserialization::*;

// TODO: rename?
// TODO: add more fields
/// The response from performing an aggregation.
#[cfg_attr(
    feature = "graphql",
    async_graphql::SimpleObject(name = "AggregationResponse")
)]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Deserialize, Clone, Debug)]
#[serde(from = "ElasticAggregationResponse")]
pub struct Response {
    /// The aggregations.
    pub aggregations: Vec<ComputedResult>,
}

/// An individual result from performing an aggregation/calculation.
#[cfg_attr(
    feature = "graphql",
    async_graphql::SimpleObject(name = "AggregationResult")
)]
#[derive(Deserialize, Clone, Debug)]
pub struct ComputedResult {
    /// The parent of this aggregation (if any).
    pub parent: Option<String>,

    /// The name assigned to this aggregation.
    pub name: String,

    // TODO: rename to `ty` once https://github.com/async-graphql/async-graphql/issues/164
    /// The `type` of this aggregation.
    pub type_: Ty,

    /// The fields this aggregation computed over.
    pub fields: Vec<String>,

    /// The actual values/result of performing this aggregation.
    pub values: Vec<f64>,

    /// The user-supplied metadata attached to this aggregation.
    pub metadata: Option<crate::scalars::Map>,
}

// TODO: generate this with proc-macro from Aggregation struct
/// The type of aggregation.
#[cfg_attr(feature = "graphql", async_graphql::Enum(name = "AggregationType"))]
#[cfg_attr(not(feature = "graphql"), derive(Clone))]
#[derive(Deserialize, Debug)]
pub enum Ty {
    /// metric
    Avg,
    /// metric
    WeightedAvg,
    /// metric
    Cardinality,
    /// metric
    Max,
    /// metric
    Min,
    /// metric
    MedianAbsoluteDeviation,
    /// metric
    Percentiles,
    /// metric
    PercentileRanks,
    /// metric
    Stats,
    /// metric
    ExtendedStats,
    /// metric
    Sum,
    /// metric
    ValueCount,

    /// bucket
    Filter,
    /// bucket
    Filters,
    /// bucket
    Terms,
    // TODO: is this `sterms`?
    /// bucket
    SignificantTerms,
    /// bucket
    Range,
    /// bucket
    DateRange,
    /// bucket
    DateHistogram,
    /// bucket
    AutoDateHistogram,
    /// bucket
    Nested,
    /// bucket
    ReverseNested,

    /// pipeline
    BucketScript,
    /// pipeline
    BucketSelector,
    /// pipeline
    BucketSort,

    /// Any Unknown type
    Unknown,
}

// TODO: generate this with proc-macro from Aggregation struct
impl From<&str> for Ty {
    #[inline]
    fn from(value: &str) -> Self {
        use self::Ty::*;
        match value.to_lowercase().as_str() {
            "avg" => Avg,
            "weighted_avg" => WeightedAvg,
            "cardinality" => Cardinality,
            "max" => Max,
            "min" => Min,
            "median_absolute_deviation" => MedianAbsoluteDeviation,
            "percentiles" => Percentiles,
            "percentile_ranks" => PercentileRanks,
            "stats" => Stats,
            "extended_stats" => ExtendedStats,
            "sum" => Sum,
            "value_count" => ValueCount,
            "filter" => Filter,
            "filters" => Filters,
            "terms" => Terms,
            "sterms" => SignificantTerms,
            "range" => Range,
            "date_range" => DateRange,
            "date_histogram" => DateHistogram,
            "auto_date_histogram" => AutoDateHistogram,
            "bucket_script" => BucketScript,
            "bucket_selector" => BucketSelector,
            "bucket_sort" => BucketSort,
            "nested" => Nested,
            "reverse_nested" => ReverseNested,
            _ => Unknown,
        }
    }
}
