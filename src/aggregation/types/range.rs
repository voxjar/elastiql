//! Range aggregation types.

use serde::{Deserialize, Serialize};

use crate::search::Script;

#[cfg(feature = "graphql")]
use crate::search::ScriptInput;

/// A [*multi-bucket*] value source based aggregation that enables the user to
/// define a set of ranges - each representing a bucket. During the aggregation
/// process, the values extracted from each document will be checked against
/// each bucket range and "bucket" the relevant/matching document.
///
/// **Note**: this aggregation *includes* the `from` value and *excludes* the
/// `to` value for each range.
///
/// **Note**: until GraphQL [Union input types] are supported either `field` or
/// `script` *must* be specified but *not* both.
///
/// [Union input types]: https://github.com/graphql/graphql-spec/blob/master/rfcs/InputUnion.md
/// [*multi-bucket*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html
#[cfg(feature = "graphql")]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(async_graphql::InputObject, Serialize, Clone, Debug)]
pub struct RangeAggregationInput {
    /// The field to perform the aggregation over.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub field: Option<String>,

    /// A script used to calculate the field to perform the aggregation over.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script: Option<ScriptInput>,

    /// The ranges to use for the aggregation.
    #[field(default)]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default)]
    pub ranges: Vec<RangeInput>,
}

/// A [*multi-bucket*] value source based aggregation that enables the user to
/// define a set of ranges - each representing a bucket. During the aggregation
/// process, the values extracted from each document will be checked against
/// each bucket range and "bucket" the relevant/matching document.
///
/// **Note**: this aggregation *includes* the `from` value and *excludes* the
/// `to` value for each range.
///
/// **Note**: until GraphQL [Union input types] are supported either `field` or
/// `script` *must* be specified but *not* both.
///
/// [Union input types]: https://github.com/graphql/graphql-spec/blob/master/rfcs/InputUnion.md
/// [*multi-bucket*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RangeAggregation {
    /// The field to perform the aggregation over.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub field: Option<String>,

    /// A script used to calculate the field to perform the aggregation over.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script: Option<Script>,

    /// The ranges to use for the aggregation.
    #[cfg_attr(feature = "builder", builder(default))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default)]
    pub ranges: Vec<Range>,
}

#[cfg(feature = "graphql")]
impl From<RangeAggregationInput> for RangeAggregation {
    #[inline]
    fn from(input: RangeAggregationInput) -> Self {
        RangeAggregation {
            field: input.field,
            script: input.script.map(Into::into),
            ranges: input.ranges.into_iter().map(Into::into).collect(),
        }
    }
}

/// A range/span of data.
#[cfg(feature = "graphql")]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(async_graphql::InputObject, Serialize, Clone, Debug)]
pub struct RangeInput {
    /// The value to return results *from* and including.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    from: Option<f64>,

    /// The value to return results up *to* but *not* including.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    to: Option<f64>,
}

/// A range/span of data.
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Range {
    /// The value to return results *from* and including.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    from: Option<f64>,

    /// The value to return results up *to* but *not* including.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    to: Option<f64>,
}

// TODO: generate this with a proc-macro?
#[cfg(feature = "graphql")]
impl From<RangeInput> for Range {
    #[inline]
    fn from(input: RangeInput) -> Self {
        Range {
            from: input.from,
            to: input.to,
        }
    }
}
