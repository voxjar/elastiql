#![allow(clippy::missing_docs_in_private_items)]

//! Inner [aggregation] types that specify the actual computation/aggregation to
//! perform.
//!
//! [aggregation]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations.html

pub use self::{
    auto_date_histogram::*, bucket_script::*, bucket_selector::*, bucket_sort::*,
    date_histogram::*, date_range::*, nested::*, range::*, reverse_nested::*, weighted_average::*,
};
use super::*;
use crate::search::{Script, ScriptInput};

mod auto_date_histogram;
mod bucket_script;
mod bucket_selector;
mod bucket_sort;
mod date_histogram;
mod date_range;
mod nested;
mod range;
mod reverse_nested;
mod weighted_average;

/// A generic input for an aggregation.
///
/// **Note**: until GraphQL [Union input types] are supported either `field` or
/// `script` *must* be specified but *not* both.
///
/// [Union input types]: https://github.com/graphql/graphql-spec/blob/master/rfcs/InputUnion.md
#[async_graphql::InputObject]
#[derive(Serialize, Clone, Debug)]
pub struct InnerAggregationInput {
    pub field: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script: Option<ScriptInput>,

    /// How missing values should be treated.
    ///
    /// By default they will be ignored, but it is also possible to treat them
    /// as if they had the value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub missing: Option<f64>,
}

/// A generic aggregation.
#[async_graphql::SimpleObject]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InnerAggregation {
    pub field: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script: Option<Script>,

    /// How missing values should be treated.
    ///
    /// By default they will be ignored, but it is also possible to treat them
    /// as if they had the value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub missing: Option<f64>,
}

impl From<InnerAggregationInput> for InnerAggregation {
    #[inline]
    fn from(aggregation: InnerAggregationInput) -> Self {
        InnerAggregation {
            field: aggregation.field,
            script: aggregation.script.map(Into::into),
            missing: aggregation.missing,
        }
    }
}

#[cfg(test)]
impl<T: Into<String>> From<T> for InnerAggregation {
    #[inline]
    fn from(field: T) -> InnerAggregation {
        InnerAggregation {
            field: Some(field.into()),
            script: None,
            missing: None,
        }
    }
}

/// The policy to apply when gaps are found in the data.
#[async_graphql::Enum]
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum GapPolicy {
    /// Treats missing data as if the bucket does not exist. It will skip the
    /// bucket and continue calculating using the next available value.
    Skip,

    /// Replaces missing values with a zero (`0`) and pipeline aggregation
    /// computation will proceed as normal.
    InsertZeros,
}
