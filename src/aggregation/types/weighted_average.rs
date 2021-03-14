//! Weighted average aggregation types.

use serde::{Deserialize, Serialize};

use super::InnerAggregation;
#[cfg(feature = "graphql")]
use super::InnerAggregationInput;

/// A `single-value` [*metrics*] aggregation that computes the weighted average
/// of numeric values that are extracted from the aggregated documents. These
/// values can be extracted either from specific numeric fields in the
/// documents.
///
/// When calculating a regular average, each datapoint has an equal "weight"...
/// it contributes equally to the final value. Weighted averages, on the other
/// hand, weight each datapoint differently. The amount that each datapoint
/// contributes to the final value is extracted from the document, or provided
/// by a script.
///
/// As a formula, a weighted average is the `∑(value * weight) / ∑(weight)`
///
/// A regular average can be thought of as a weighted average where every value
/// has an implicit weight of `1`.
///
/// [*metrics*]:  https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics.html
#[cfg(feature = "graphql")]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(async_graphql::InputObject, Serialize, Clone, Debug)]
#[cfg_attr(feature = "builder", builder(field_defaults(setter(into))))]
pub struct WeightedAverageAggregationInput {
    /// The configuration for the field or script that provides the values
    pub value: InnerAggregationInput,

    /// The configuration for the field or script that provides the weights
    pub weight: InnerAggregationInput,

    /// The numeric response formatter
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    /// A hint about the values for pure scripts or unmapped fields
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,
}

/// A `single-value` [*metrics*] aggregation that computes the weighted average
/// of numeric values that are extracted from the aggregated documents. These
/// values can be extracted either from specific numeric fields in the
/// documents.
///
/// When calculating a regular average, each datapoint has an equal "weight"...
/// it contributes equally to the final value. Weighted averages, on the other
/// hand, weight each datapoint differently. The amount that each datapoint
/// contributes to the final value is extracted from the document, or provided
/// by a script.
///
/// As a formula, a weighted average is the `∑(value * weight) / ∑(weight)`
///
/// A regular average can be thought of as a weighted average where every value
/// has an implicit weight of `1`.
///
/// [*metrics*]:  https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics.html
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "builder", builder(field_defaults(setter(into))))]
pub struct WeightedAverageAggregation {
    /// The configuration for the field or script that provides the values
    pub value: InnerAggregation,

    /// The configuration for the field or script that provides the weights
    pub weight: InnerAggregation,

    /// The numeric response formatter
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    /// A hint about the values for pure scripts or unmapped fields
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,
}

#[cfg(feature = "graphql")]
impl From<WeightedAverageAggregationInput> for WeightedAverageAggregation {
    #[inline]
    fn from(aggregation: WeightedAverageAggregationInput) -> Self {
        WeightedAverageAggregation {
            value: aggregation.value.into(),
            weight: aggregation.weight.into(),
            format: aggregation.format,
            value_type: aggregation.value_type,
        }
    }
}
