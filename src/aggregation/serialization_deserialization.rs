//! Facilitates [serialization/deserialization] for all [`Aggregation`] types.
//!
//! [ser/de]: https://docs.rs/serde/latest/serde/

use std::{collections::HashMap, fmt};

use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::ser::{SerializeMap, Serializer};
use serde::{Deserialize, Serialize};

#[cfg(feature = "graphql")]
use super::request::RequestInput as AggregationInput;
use super::{request::Request as Aggregation, response::Ty, types::*, ComputedResult, Response};
use crate::search::query::CompoundQuery;

#[cfg(feature = "graphql")]
impl Serialize for AggregationInput {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(&self.name, &SubAggregation::from(self.to_owned()))?;
        map.end()
    }
}

// TODO: auto generate this with a proc-macro?
#[allow(clippy::missing_docs_in_private_items)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub(super) struct SubAggregation {
    // Metric aggregations
    #[serde(default, skip_serializing_if = "Option::is_none")]
    avg: Option<InnerAggregation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    weighted_avg: Option<WeightedAverageAggregation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cardinality: Option<InnerAggregation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    max: Option<InnerAggregation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    min: Option<InnerAggregation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    median_absolute_deviation: Option<InnerAggregation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    percentiles: Option<InnerAggregation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    percentile_ranks: Option<InnerAggregation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    stats: Option<InnerAggregation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    extended_stats: Option<InnerAggregation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    sum: Option<InnerAggregation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    value_count: Option<InnerAggregation>,

    // Bucketing aggregations
    #[serde(default, rename = "filter", skip_serializing_if = "Option::is_none")]
    filters: Option<CompoundQuery>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    terms: Option<TermsAggregation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    range: Option<RangeAggregation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    date_range: Option<DateRangeAggregation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    date_histogram: Option<DateHistogramAggregation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    auto_date_histogram: Option<AutoDateHistogramAggregation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    histogram: Option<HistogramAggregation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    variable_width_histogram: Option<VariableWidthHistogram>,

    // Pipeline aggregations
    #[serde(default, skip_serializing_if = "Option::is_none")]
    bucket_script: Option<BucketScript>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    bucket_selector: Option<BucketSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    bucket_sort: Option<BucketSort>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    nested: Option<NestedAggregation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    reverse_nested: Option<ReverseNestedAggregation>,

    #[serde(default, rename = "meta", skip_serializing_if = "Option::is_none")]
    metadata: Option<crate::scalars::Map>,

    #[serde(
        default,
        rename = "aggs",
        skip_serializing_if = "Option::is_none",
        with = "serde_sub_aggregations"
    )]
    aggregations: Option<Vec<Aggregation>>,
}

// TODO: auto generate this with a proc-macro?
#[cfg(feature = "graphql")]
impl From<AggregationInput> for SubAggregation {
    #[inline]
    fn from(aggregation: AggregationInput) -> SubAggregation {
        SubAggregation {
            avg: aggregation.avg.map(Into::into),
            weighted_avg: aggregation.weighted_avg.map(Into::into),
            cardinality: aggregation.cardinality.map(Into::into),
            max: aggregation.max.map(Into::into),
            min: aggregation.min.map(Into::into),
            median_absolute_deviation: aggregation.median_absolute_deviation.map(Into::into),
            percentiles: aggregation.percentiles.map(Into::into),
            percentile_ranks: aggregation.percentile_ranks.map(Into::into),
            stats: aggregation.stats.map(Into::into),
            extended_stats: aggregation.extended_stats.map(Into::into),
            sum: aggregation.sum.map(Into::into),
            value_count: aggregation.value_count.map(Into::into),
            filters: aggregation.filters.map(Into::into),
            terms: aggregation.terms.map(Into::into),
            range: aggregation.range.map(Into::into),
            date_range: aggregation.date_range.map(Into::into),
            date_histogram: aggregation.date_histogram.map(Into::into),
            auto_date_histogram: aggregation.auto_date_histogram.map(Into::into),
            histogram: aggregation.histogram.map(Into::into),
            variable_width_histogram: aggregation.variable_width_histogram.map(Into::into),
            bucket_script: aggregation.bucket_script.map(Into::into),
            bucket_selector: aggregation.bucket_selector.map(Into::into),
            bucket_sort: aggregation.bucket_sort.map(Into::into),
            reverse_nested: aggregation.reverse_nested.map(Into::into),
            nested: aggregation.nested.map(Into::into),
            metadata: aggregation.metadata,
            aggregations: aggregation
                .aggregations
                .map(|aggs| aggs.into_iter().map(Into::into).collect()),
        }
    }
}

// TODO: auto generate this with a proc-macro?
impl From<Aggregation> for SubAggregation {
    #[inline]
    fn from(aggregation: Aggregation) -> SubAggregation {
        SubAggregation {
            avg: aggregation.avg.map(Into::into),
            weighted_avg: aggregation.weighted_avg.map(Into::into),
            cardinality: aggregation.cardinality.map(Into::into),
            max: aggregation.max.map(Into::into),
            min: aggregation.min.map(Into::into),
            median_absolute_deviation: aggregation.median_absolute_deviation.map(Into::into),
            percentiles: aggregation.percentiles.map(Into::into),
            percentile_ranks: aggregation.percentile_ranks.map(Into::into),
            stats: aggregation.stats.map(Into::into),
            extended_stats: aggregation.extended_stats.map(Into::into),
            sum: aggregation.sum.map(Into::into),
            value_count: aggregation.value_count.map(Into::into),
            filters: aggregation.filters.map(Into::into),
            terms: aggregation.terms.map(Into::into),
            range: aggregation.range.map(Into::into),
            date_range: aggregation.date_range.map(Into::into),
            date_histogram: aggregation.date_histogram.map(Into::into),
            auto_date_histogram: aggregation.auto_date_histogram.map(Into::into),
            histogram: aggregation.histogram.map(Into::into),
            variable_width_histogram: aggregation.variable_width_histogram.map(Into::into),
            bucket_script: aggregation.bucket_script.map(Into::into),
            bucket_selector: aggregation.bucket_selector.map(Into::into),
            bucket_sort: aggregation.bucket_sort.map(Into::into),
            reverse_nested: aggregation.reverse_nested.map(Into::into),
            nested: aggregation.nested.map(Into::into),
            metadata: aggregation.metadata,
            aggregations: aggregation
                .aggregations
                .map(|aggs| aggs.into_iter().map(Into::into).collect()),
        }
    }
}

impl Aggregation {
    #[allow(clippy::missing_docs_in_private_items)]
    pub(super) fn from_sub_aggregation(name: String, aggregation: SubAggregation) -> Aggregation {
        Aggregation {
            name,
            avg: aggregation.avg.map(Into::into),
            weighted_avg: aggregation.weighted_avg.map(Into::into),
            cardinality: aggregation.cardinality.map(Into::into),
            max: aggregation.max.map(Into::into),
            min: aggregation.min.map(Into::into),
            median_absolute_deviation: aggregation.median_absolute_deviation.map(Into::into),
            percentiles: aggregation.percentiles.map(Into::into),
            percentile_ranks: aggregation.percentile_ranks.map(Into::into),
            stats: aggregation.stats.map(Into::into),
            extended_stats: aggregation.extended_stats.map(Into::into),
            sum: aggregation.sum.map(Into::into),
            value_count: aggregation.value_count.map(Into::into),
            filters: aggregation.filters.map(Into::into),
            terms: aggregation.terms.map(Into::into),
            range: aggregation.range.map(Into::into),
            date_range: aggregation.date_range.map(Into::into),
            date_histogram: aggregation.date_histogram.map(Into::into),
            auto_date_histogram: aggregation.auto_date_histogram.map(Into::into),
            histogram: aggregation.histogram.map(Into::into),
            variable_width_histogram: aggregation.variable_width_histogram.map(Into::into),
            bucket_script: aggregation.bucket_script.map(Into::into),
            bucket_selector: aggregation.bucket_selector.map(Into::into),
            bucket_sort: aggregation.bucket_sort.map(Into::into),
            reverse_nested: aggregation.reverse_nested.map(Into::into),
            nested: aggregation.nested.map(Into::into),
            metadata: aggregation.metadata,
            aggregations: aggregation
                .aggregations
                .map(|aggs| aggs.into_iter().map(Into::into).collect()),
        }
    }
}

// TODO: re-use the serializer from the input type
impl Serialize for Aggregation {
    #[inline]
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(&self.name, &SubAggregation::from(self.to_owned()))?;
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for Aggregation {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Aggregation, D::Error>
    where
        D: Deserializer<'de>,
    {
        /// Visits an `Aggregation` during deserialization.
        struct AggregationVisitor;

        impl<'de> Visitor<'de> for AggregationVisitor {
            type Value = Aggregation;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an `Aggregation`")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                // NOTE: this only handles the first top-level agg found

                let name = map
                    .next_key::<String>()?
                    .ok_or_else(|| de::Error::missing_field("name"))?;

                let agg: SubAggregation = map.next_value()?;

                Ok(Aggregation::from_sub_aggregation(name, agg))
            }
        }

        deserializer.deserialize_map(AggregationVisitor)
    }
}

/// The raw JSON response to performing an aggregation from Elasticsearch.
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ElasticAggregationResponse {
    #[serde(default, alias = "aggs")]
    aggregations: HashMap<String, ElasticAggregationResult>,
}

impl From<ElasticAggregationResponse> for Response {
    // TODO: make this recursive instead/cleanup this function...
    /// Converts aggregation results from Elasticsearch to a trace like format
    /// suitable for plotting libraries.
    #[inline]
    fn from(response: ElasticAggregationResponse) -> Self {
        let aggs = response.aggregations;

        // (parent, name) => AggregationResult
        let mut results: HashMap<(Option<&String>, String), ComputedResult> = HashMap::new();

        let mut pending_aggs: Vec<(Option<&String>, _)> = vec![(None, &aggs)];
        while let Some(curr) = pending_aggs.pop() {
            let (parent, aggs) = curr;

            for (ty_and_name, curr_agg) in aggs.iter() {
                let (ty, name) = split_ty_and_name(ty_and_name);

                let mut handle_leaf_agg = |agg: &ElasticAggregationResult| {
                    if let Some(value) = agg.value_or_doc_count() {
                        if !agg.should_skip() {
                            #[allow(clippy::clone_on_copy)] // necessary for TypedBuilder
                            let result =
                                results
                                    .entry((parent, name.to_string()))
                                    .or_insert_with(|| ComputedResult {
                                        parent: parent.map(|p| p.to_owned()),
                                        name: name.to_string(),
                                        type_: ty.clone(),
                                        fields: vec![],
                                        values: vec![],
                                        metadata: agg.metadata.to_owned(),
                                    });

                            if let Some(key) = agg.parent_key.as_ref().or_else(|| agg.key.as_ref())
                            {
                                result.fields.push(key.to_owned());
                            }

                            // TODO: should we only push this if there is a `key`?
                            result.values.push(value);
                        }
                    }
                };

                handle_leaf_agg(curr_agg);

                pending_aggs.push((None, &curr_agg.aggregations));

                for bucket_agg in curr_agg.buckets.iter() {
                    if bucket_agg.aggregations.is_empty() {
                        handle_leaf_agg(bucket_agg);
                    } else {
                        pending_aggs.push((curr_agg.parent_key.as_ref(), &bucket_agg.aggregations));
                    }
                }
            }
        }

        Response {
            aggregations: results.into_iter().map(|(_, agg)| agg).collect(),
        }
    }
}

#[doc(hidden)]
#[allow(clippy::indexing_slicing)]
fn split_ty_and_name(ty_and_name: &str) -> (Ty, String) {
    let parts: Vec<&str> = ty_and_name.split('#').collect();

    if parts.len() < 2 {
        // the split will always have at least 1 item
        (Ty::Unknown, parts[0].to_owned())
    } else {
        (parts[0].into(), parts[1..].join(""))
    }
}

// TODO; this should be an enum
#[derive(Serialize, Default, Debug)]
struct ElasticAggregationResult {
    parent_key: Option<String>,
    key: Option<String>,
    doc_count: Option<u64>,
    value: Option<f64>,
    buckets: Vec<ElasticAggregationResult>,
    metadata: Option<crate::scalars::Map>,
    aggregations: HashMap<String, ElasticAggregationResult>,
}

impl ElasticAggregationResult {
    fn value_or_doc_count(&self) -> Option<f64> {
        self.value.or_else(|| {
            // we only want to use the doc_count for leaf nodes
            // TODO: should we add: `&& (self.parent_key.is_some() || self.key.is_some())`
            if self.buckets.is_empty() {
                #[allow(clippy::as_conversions)]
                self.doc_count.map(|v| v as f64)
            } else {
                None
            }
        })
    }

    fn should_skip(&self) -> bool {
        let mut skip = false;
        if let Some(meta) = self.metadata.as_ref() {
            if let Some(s) = meta.get("_skip") {
                if let Some(b) = s.as_bool() {
                    skip = b;
                }
            }
        }
        skip
    }
}

// TODO: replace with default implementation from Serde using an enum for ElasticAggregationResult
impl<'de> serde::Deserialize<'de> for ElasticAggregationResult {
    fn deserialize<D>(deserializer: D) -> Result<ElasticAggregationResult, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[doc(hidden)]
        /// Visits a `Agg` during deserialization.
        struct ElasticAggregationResultVisitor;

        impl<'de> Visitor<'de> for ElasticAggregationResultVisitor {
            type Value = ElasticAggregationResult;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an `ElasticAggregationResult`")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                // Make our own value so we don't need to depend on `serde_json::Value`
                #[derive(Deserialize)]
                #[serde(untagged)]
                enum Value {
                    Null,
                    Bool(bool),
                    Int(u64),
                    Float(f64),
                    String(String),
                    Array(Vec<Value>),
                    Object(HashMap<String, Value>),
                }

                let mut result = ElasticAggregationResult::default();

                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "key" => {
                            if let Value::String(val) = map.next_value()? {
                                result.key = Some(val);
                            }
                        }
                        "key_as_string" => result.key = Some(map.next_value()?),
                        "value" => result.value = Some(map.next_value()?),
                        "buckets" => result.buckets = map.next_value()?,
                        "doc_count" => result.doc_count = Some(map.next_value()?),
                        "doc_count_error_upper_bound" | "sum_other_doc_count" | "interval" => {
                            // Must throw the next value away, otherwise the parser will fail
                            let _: Value = map.next_value()?;
                        }
                        "meta" | "metadata" => result.metadata = Some(map.next_value()?),
                        _ => match map.next_value::<ElasticAggregationResult>() {
                            Ok(val) => {
                                // TODO(perf): make this a str?
                                result.aggregations.insert(k.to_string(), val);
                            }
                            Err(_err) => {
                                // TODO: should we error if this happens?
                            }
                        },
                    }
                }

                let key = &result.key;
                result.aggregations = result
                    .aggregations
                    .into_iter()
                    .map(|(name, mut agg)| {
                        agg.parent_key = key.clone();
                        (name, agg)
                    })
                    .collect();

                Ok(result)
            }
        }

        deserializer.deserialize_map(ElasticAggregationResultVisitor)
    }
}

pub(super) mod serde_sub_aggregations {
    //! ser/de implementation for `SubAggregations`.
    use std::collections::HashMap;

    use serde::{ser::SerializeMap, Deserialize, Deserializer, Serializer};

    use super::{Aggregation, SubAggregation};

    /// Serializes the data to a format expected by Elasticsearch, with the
    /// field name as a key.
    #[inline]
    pub(crate) fn serialize<S>(aggs: &Option<Vec<Aggregation>>, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(aggs) = aggs {
            let mut map = ser.serialize_map(Some(aggs.len()))?;

            for agg in aggs.iter() {
                map.serialize_entry(agg.name.as_str(), &SubAggregation::from(agg.to_owned()))?;
            }

            map.end()
        } else {
            ser.serialize_none()
        }
    }

    /// Deserializes the data from a format expected by Elasticsearch, with the
    /// field name as a key.
    #[inline]
    #[cfg(not(test))]
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<Aggregation>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(
            Option::deserialize(deserializer)?.map(|agg: HashMap<String, SubAggregation>| {
                agg.into_iter()
                    .map(|(name, sub_agg)| Aggregation::from_sub_aggregation(name, sub_agg))
                    .collect()
            }),
        )
    }

    // HACK: this is so we don't have to manually derive PartialEq and potentially forget to add fields
    #[doc(hidden)]
    #[cfg(test)]
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<Aggregation>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(
            Option::deserialize(deserializer)?.map(|agg: HashMap<String, SubAggregation>| {
                let mut aggs: Vec<Aggregation> = agg
                    .into_iter()
                    .map(|(name, sub_agg)| Aggregation::from_sub_aggregation(name, sub_agg))
                    .collect();

                #[allow(clippy::expect_used)]
                aggs.sort_by(|a, b| a.name.partial_cmp(&b.name).expect("invalid ordering"));

                aggs
            }),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;

    use crate::search::query::TermsQuery;

    /// Simple smoke test. This also makes it so editors pick up this test mod as runnable.
    #[test]
    fn simple() {
        let input = json!({ "aggregations": { "agg": { "value_count": { "field": "id" } } } });
        let _: Aggregation = serde_json::from_value(input).unwrap();

        let result = json!({ "aggregations": { "agg": { "value": 123_456_789 } } });
        let _: Response = serde_json::from_value(result).unwrap();
    }

    mod aggregation_input {
        use super::*;

        /// Simple smoke test. This also makes it so editors pick up this test mod as runnable.
        #[test]
        fn simple() {
            let input = json!({ "aggs": { "AVG_AGG": { "avg": { "field": "duration" } } } });
            let _: Aggregation = serde_json::from_value(input).unwrap();
        }

        macro_rules! test_case {
            ($name:ident : $agg:expr, $json_value:expr) => {
                mod $name {
                    use super::*;

                    #[test]
                    fn can_serialize() {
                        let actual = serde_json::to_value($agg).unwrap();
                        let expected = $json_value;
                        let json_pretty_print = |d| serde_json::to_string_pretty(&d).unwrap();
                        assert_eq!(
                            &actual,
                            &expected,
                            "\nactual:\n{}\nexpected:\n{}",
                            json_pretty_print(&actual),
                            json_pretty_print(&expected)
                        );
                    }

                    #[test]
                    fn can_deserialize() {
                        let expected = $agg;
                        let actual: Aggregation = serde_json::from_value($json_value).unwrap();
                        assert_eq!(
                            actual, expected,
                            "\nactual:\n{:#?}\nexpected:\n{:#?}",
                            &actual, &expected
                        );
                    }
                }
            };
        }

        test_case!(
            simple_with_metadata:
            Aggregation::builder()
                .name("hasMetadata")
                .terms(Some("id".into()))
                .metadata(Some(json!({"test": true}).into()))
                .build(),
            json!({
                "hasMetadata": {
                    "terms": { "field": "id" },
                    "meta": { "test": true },
                },
            })
        );

        test_case!(
            simple_with_nest:
            Aggregation::builder()
                .name("SPECIFIC_AGENTS")
                .filters(Some(TermsQuery::new("agents", vec!["123", "456", "789"]).into()))
                .aggregations(vec![Aggregation::builder()
                    .name("PER_AGENT")
                    .terms(Some("agents".into()))
                    .aggregations(vec![Aggregation::builder()
                        .name("PER_TYPE")
                        .terms(Some("metadata.type".into()))
                        .aggregations(vec![
                            Aggregation::builder()
                                .name("AVG_OF_DURATION")
                                .avg(Some("duration".into()))
                                .build(),
                            Aggregation::builder()
                                .name("AVG_OF_SILENCE_DURATION")
                                .avg(Some("silenceDuration".into()))
                                .build(),
                            Aggregation::builder()
                                .name("COUNT_OF_CALLS")
                                .value_count(Some("id".into()))
                                .build(),
                        ])
                        .build()])
                    .build()])
                .build(),
            json!({
                "SPECIFIC_AGENTS": {
                    "filter": {
                        "bool": {
                            "filter": [{ "terms": { "agents": ["123", "456", "789"] } }]
                        }
                    },
                    "aggs": {
                        "PER_AGENT": {
                            "terms": { "field": "agents" },
                            "aggs": {
                                "PER_TYPE": {
                                    "terms": { "field": "metadata.type" },
                                    "aggs": {
                                        "AVG_OF_DURATION": { "avg": { "field": "duration" } },
                                        "AVG_OF_SILENCE_DURATION": {
                                            "avg": { "field": "silenceDuration" }
                                        },
                                        "COUNT_OF_CALLS": { "value_count": { "field": "id" } },
                                    }
                                }
                            }
                        }
                    }
                }
            })
        );

        test_case!(
            date_range_with_nest:
            Aggregation::builder()
                .name("TIMESTAMP_DATE_RANGE")
                .date_range(
                    DateRangeAggregation::builder()
                        .field("timestamp")
                        .format(Some("yyyy-MM-dd'T'HH:mm:ssX".into()))
                        .missing(Some("1970-01-01T00:00:00Z".into()))
                        .ranges(vec![DateRange::new(Some("now-10M/M"), Some("now-1d/d"))])
                        .build()
                )
                .aggregations(vec![
                    Aggregation::builder()
                        .name("ID_VALUE_COUNT")
                        .value_count(Some("id".into()))
                        .build(),
                ])
                .build(),
            json!({
                "TIMESTAMP_DATE_RANGE": {
                    "aggs": {
                        "ID_VALUE_COUNT": { "value_count": { "field": "id" } }
                    },
                    "date_range": {
                        "field": "timestamp",
                        "format": "yyyy-MM-dd'T'HH:mm:ssX",
                        "missing": "1970-01-01T00:00:00Z",
                        "ranges": [{ "from": "now-10M/M", "to": "now-1d/d" }]
                    }
                },
            })
        );
    }

    mod aggregation_results {
        use super::*;

        use std::cmp::Ordering;

        use crate::aggregation::Response;

        /// Simple smoke test. This also makes it so editors pick up this test mod as runnable.
        #[test]
        fn simple() {
            let result = json!({ "aggregations": { "AVG_DURATION": { "value": 353_964.312 } } });
            let _: Response = serde_json::from_value(result).unwrap();
        }

        // TODO: use real `AggregationTypes` instead of `Unknown`

        macro_rules! test_case {
            ($name:ident : $agg:expr, $json_value:expr) => {
                mod $name {
                    use super::*;

                    #[test]
                    fn can_deserialize() {
                        let mut expected = $agg;

                        let mut actual: Response = serde_json::from_value($json_value).unwrap();

                        actual.aggregations.sort();
                        expected.aggregations.sort();

                        assert_eq!(
                            actual, expected,
                            "\nactual:\n{:#?}\nexpected:\n{:#?}",
                            &actual, &expected
                        );
                    }
                }
            };
        }

        test_case!(
            simple_with_metadata:
            Response {
                aggregations: vec![ComputedResult {
                    parent: None,
                    name: "AVG_DURATION".to_string(),
                    fields: vec![],
                    values: vec![3.0, 4.0],
                    metadata: Some(json!({"test": true}).into()),
                    type_: Ty::Avg,
                }],
            },
            json!({
                "aggregations": {
                    "avg#AVG_DURATION": { "value": 353_964.312_5 },
                    "metadata": { "test": true }
                }
            })
        );

        test_case!(
            simple_with_skip:
            Response {
                aggregations: vec![ComputedResult {
                    parent: None,
                    name: "PERCENT_DEAD_AIR".to_string(),
                    fields: vec!["dallin".to_string(), "will".to_string()],
                    values: vec![0.009, 0.017],
                    metadata: None,
                    type_: Ty::Unknown,
                }],
            },
            json!({
                "aggregations": {
                    "PER_AGENT": {
                        "doc_count_error_upper_bound": 0,
                        "sum_other_doc_count": 0,
                        "buckets": [
                            {
                                "key": "dallin",
                                "doc_count": 7,
                                "sum#SUM_DURATION": {
                                    "metadata": { "_skip": true },
                                    "value": 3_237_014.0,
                                },
                                "sum#SUM_SILENCE_DURATION": {
                                    "metadata": { "_skip": true },
                                    "value": 31074.0
                                },
                                "PERCENT_DEAD_AIR": {
                                    "value": 0.009,
                                }
                            },
                            {
                                "key": "will",
                                "doc_count": 7,
                                "sum#SUM_DURATION": {
                                    "metadata": { "_skip": true },
                                    "value": 2_426_214.0
                                },
                                "sum#SUM_SILENCE_DURATION": {
                                    "metadata": { "_skip": true },
                                    "value": 41074.0,
                                },
                                "PERCENT_DEAD_AIR": {
                                    "value": 0.017,
                                }
                            }
                        ]
                    }
                }
            })
        );

        test_case!(
            simple_without_nest:
            Response {
                aggregations: vec![ComputedResult {
                    parent: None,
                    name: "AVG_DURATION".to_string(),
                    fields: vec![],
                    values: vec![3.0, 4.0],
                    metadata: None,
                    type_: Ty::Avg,
                }],
            },
            json!({ "aggregations": { "avg#AVG_DURATION": { "value": 353_964.312_5 } } })
        );

        test_case!(
            simple_with_nest:
            Response {
                aggregations: vec![
                    ComputedResult {
                        parent: None,
                        name: "AVG_DURATION".to_string(),
                        fields: vec!["dallin".to_string(), "will".to_string()],
                        values: vec![462_430.123, 346_602.0],
                        metadata: None,
                        type_: Ty::Avg,
                    },
                ]
            },
            json!({
                "took": 2,
                "timed_out": false,
                "hits": {
                    "total": { "value": 20, "relation": "eq" },
                    "max_score": null,
                    "hits": []
                },
                "aggregations": {
                    "PER_AGENT": {
                        "doc_count_error_upper_bound": 0,
                        "sum_other_doc_count": 0,
                        "buckets": [
                            {
                                "key": "dallin",
                                "doc_count": 7,
                                "avg#AVG_DURATION": { "value": 462_430.123 }
                            },
                            {
                                "key": "will",
                                "doc_count": 7,
                                "avg#AVG_DURATION": { "value": 346_602 }
                            }
                        ]
                    }
                }
            })
        );

        test_case!(
            complex_with_nest:
            Response {
                aggregations: vec![
                    ComputedResult {
                        parent: Some("sales".to_string()),
                        name: "COUNT_OF_CALLS".to_string(),
                        fields: vec!["dallin".to_string(), "will".to_string()],
                        values: vec![3.0, 4.0],
                        metadata: None,
                        type_: Ty::ValueCount,
                    },
                    ComputedResult {
                        parent: Some("sales".to_string()),
                        name: "SUM_OF_DURATION".to_string(),
                        fields: vec!["dallin".to_string(), "will".to_string()],
                        values: vec![2997.0, 2196.0],
                        metadata: None,
                        type_: Ty::Sum,
                    },
                    ComputedResult {
                        parent: Some("sales".to_string()),
                        name: "AVG_OF_DURATION".to_string(),
                        fields: vec!["dallin".to_string(), "will".to_string()],
                        values: vec![999.0, 549.0],
                        metadata: None,
                        type_: Ty::Avg,
                    },
                    ComputedResult {
                        parent: Some("(missing)".to_string()),
                        name: "COUNT_OF_CALLS".to_string(),
                        fields: vec!["dallin".to_string(), "will".to_string()],
                        values: vec![4.0, 3.0],
                        metadata: None,
                        type_: Ty::ValueCount,
                    },
                    ComputedResult {
                        parent: Some("(missing)".to_string()),
                        name: "SUM_OF_DURATION".to_string(),
                        fields: vec!["dallin".to_string(), "will".to_string()],
                        values: vec![3_234_017.0, 2_424_018.0],
                        metadata: None,
                        type_: Ty::Sum,
                    },
                    ComputedResult {
                        parent: Some("(missing)".to_string()),
                        name: "AVG_OF_DURATION".to_string(),
                        fields: vec!["dallin".to_string(), "will".to_string()],
                        values: vec![808_504.25, 808_006.0],
                        metadata: None,
                        type_: Ty::Avg,
                    },
                ],
            },
            json!({
                "took": 16,
                "timed_out": false,
                "_shards": { "total": 2, "successful": 2, "skipped": 0, "failed": 0 },
                "hits": {
                    "total": { "value": 14, "relation": "eq" },
                    "max_score": null,
                    "hits": []
                },
                "aggregations": {
                    "ANY_CALL": {
                        "doc_count": 14,
                        "meta": { "_skip": true },
                        "PER_TYPE": {
                            "doc_count_error_upper_bound": 0,
                            "sum_other_doc_count": 0,
                            "buckets": [
                                {
                                    "key": "(missing)",
                                    "doc_count": 7,
                                    "PER_AGENT": {
                                        "doc_count_error_upper_bound": 0,
                                        "sum_other_doc_count": 0,
                                        "buckets": [
                                            {
                                                "key": "dallin",
                                                "doc_count": 4,
                                                "avg#AVG_OF_DURATION": { "value": 808_504.25 },
                                                "value_count#COUNT_OF_CALLS": { "value": 4 },
                                                "sum#SUM_OF_DURATION": { "value": 3_234_017 }
                                            },
                                            {
                                                "key": "will",
                                                "doc_count": 3,
                                                "avg#AVG_OF_DURATION": { "value": 808_006 },
                                                "value_count#COUNT_OF_CALLS": { "value": 3 },
                                                "sum#SUM_OF_DURATION": { "value": 2_424_018 }
                                            }
                                        ]
                                    }
                                },
                                {
                                    "key": "sales",
                                    "doc_count": 7,
                                    "PER_AGENT": {
                                        "doc_count_error_upper_bound": 0,
                                        "sum_other_doc_count": 0,
                                        "buckets": [
                                            {
                                                "key": "will",
                                                "doc_count": 4,
                                                "avg#AVG_OF_DURATION": { "value": 549 },
                                                "value_count#COUNT_OF_CALLS": { "value": 4 },
                                                "sum#SUM_OF_DURATION": { "value": 2196 }
                                            },
                                            {
                                                "key": "dallin",
                                                "doc_count": 3,
                                                "avg#AVG_OF_DURATION": { "value": 999 },
                                                "value_count#COUNT_OF_CALLS": { "value": 3 },
                                                "sum#SUM_OF_DURATION": { "value": 2997 }
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                }
            })
        );

        test_case!(
            date_range_with_nest:
            Response {
                aggregations: vec![
                    ComputedResult {
                        parent: None,
                        name: "ID_VALUE_COUNT".to_string(),
                        fields: vec![
                            "*-2018-12-01T00:00:00Z".to_string(),
                            "2018-12-01T00:00:00Z-*".to_string(),
                        ],
                        values: vec![0.0, 30.0],
                        metadata: None,
                        type_: Ty::ValueCount,
                    },
                ]
            },
            json!({
                "aggregations": {
                    "date_range#TIMESTAMP_DATE_RANGE": {
                        "buckets": [
                            {
                                "key": "*-2018-12-01T00:00:00Z",
                                "to": 1_543_622_400_000.0,
                                "to_as_string": "2018-12-01T00:00:00Z",
                                "doc_count": 0,
                                "value_count#ID_VALUE_COUNT": { "value": 0 }
                            },
                            {
                                "key": "2018-12-01T00:00:00Z-*",
                                "from": 1_543_622_400_000.0,
                                "from_as_string": "2018-12-01T00:00:00Z",
                                "doc_count": 30,
                                "value_count#ID_VALUE_COUNT": { "value": 30 }
                            }
                        ]
                    }
                }
            })
        );

        test_case!(
            date_histogram:
            Response {
                aggregations: vec![
                    ComputedResult {
                        parent: None,
                        name: "TIMESTAMP_HISTOGRAM".to_string(),
                        fields: vec![
                            "2020-01-01T00:00:00.000Z".to_string(),
                            "2020-01-02T00:00:00.000Z".to_string(),
                            "2020-01-03T00:00:00.000Z".to_string(),
                            "2020-01-04T00:00:00.000Z".to_string(),
                            "2020-01-05T00:00:00.000Z".to_string(),
                        ],
                        values: vec![1.0, 2.0, 1.0, 1.0, 2.0],
                        metadata: None,
                        type_: Ty::DateHistogram,
                    },
                ]
            },
            json!({
                "aggregations": {
                    "filter#PER_COMPANY": {
                        "doc_count": 7,
                        "meta": { "_skip": true },
                        "date_histogram#TIMESTAMP_HISTOGRAM": {
                            "buckets": [
                                {
                                    "doc_count": 1,
                                    "key": 1_577_836_800_000_u64,
                                    "key_as_string": "2020-01-01T00:00:00.000Z"
                                },
                                {
                                    "doc_count": 2,
                                    "key": 1_577_923_200_000_u64,
                                    "key_as_string": "2020-01-02T00:00:00.000Z"
                                },
                                {
                                    "doc_count": 1,
                                    "key": 1_578_009_600_000_u64,
                                    "key_as_string": "2020-01-03T00:00:00.000Z"
                                },
                                {
                                    "doc_count": 1,
                                    "key": 1_578_096_000_000_u64,
                                    "key_as_string": "2020-01-04T00:00:00.000Z"
                                },
                                {
                                    "doc_count": 2,
                                    "key": 1_578_182_400_000_u64,
                                    "key_as_string": "2020-01-05T00:00:00.000Z"
                                }
                            ]
                        },
                    }
                },
            })
        );

        test_case!(
            auto_date_histogram:
            Response {
                aggregations: vec![
                    ComputedResult {
                        parent: None,
                        name: "TIMESTAMP_AUTO_DATE_HISTOGRAM".to_string(),
                        fields: vec![
                            "2020-01-01T00:00:00.000Z".to_string(),
                            "2020-01-01T12:00:00.000Z".to_string(),
                            "2020-01-02T00:00:00.000Z".to_string(),
                            "2020-01-02T12:00:00.000Z".to_string(),
                            "2020-01-03T00:00:00.000Z".to_string(),
                            "2020-01-03T12:00:00.000Z".to_string(),
                            "2020-01-04T00:00:00.000Z".to_string(),
                            "2020-01-04T12:00:00.000Z".to_string(),
                            "2020-01-05T00:00:00.000Z".to_string(),
                        ],
                        values: vec![1.0, 0.0, 2.0, 0.0, 1.0, 0.0, 1.0, 0.0, 2.0],
                        metadata: None,
                        type_: Ty::AutoDateHistogram,
                    },
                ]
            },
            json!({
                "aggregations": {
                    "filter#PER_COMPANY": {
                        "doc_count": 7,
                        "meta": { "_skip": true },
                        "auto_date_histogram#TIMESTAMP_AUTO_DATE_HISTOGRAM": {
                            "buckets": [
                                {
                                    "doc_count": 1,
                                    "key": 1_577_836_800_000_u64,
                                    "key_as_string": "2020-01-01T00:00:00.000Z"
                                },
                                {
                                    "doc_count": 0,
                                    "key": 1_577_880_000_000_u64,
                                    "key_as_string": "2020-01-01T12:00:00.000Z"
                                },
                                {
                                    "doc_count": 2,
                                    "key": 1_577_923_200_000_u64,
                                    "key_as_string": "2020-01-02T00:00:00.000Z"
                                },
                                {
                                    "doc_count": 0,
                                    "key": 1_577_966_400_000_u64,
                                    "key_as_string": "2020-01-02T12:00:00.000Z"
                                },
                                {
                                    "doc_count": 1,
                                    "key": 1_578_009_600_000_u64,
                                    "key_as_string": "2020-01-03T00:00:00.000Z"
                                },
                                {
                                    "doc_count": 0,
                                    "key": 1_578_052_800_000_u64,
                                    "key_as_string": "2020-01-03T12:00:00.000Z"
                                },
                                {
                                    "doc_count": 1,
                                    "key": 1_578_096_000_000_u64,
                                    "key_as_string": "2020-01-04T00:00:00.000Z"
                                },
                                {
                                    "doc_count": 0,
                                    "key": 1_578_139_200_000_u64,
                                    "key_as_string": "2020-01-04T12:00:00.000Z"
                                },
                                {
                                    "doc_count": 2,
                                    "key": 1_578_182_400_000_u64,
                                    "key_as_string": "2020-01-05T00:00:00.000Z"
                                }
                            ],
                            "interval": "12h"
                        }
                    }
                },
            })
        );

        test_case!(
            bucketing_agg_as_leaf_node:
            Response {
                aggregations: vec![
                    ComputedResult {
                        parent: None,
                        name: "PER_AGENT".to_string(),
                        fields: vec!["Denmark".to_string()],
                        values: vec![1.0],
                        metadata: None,
                        type_: Ty::Unknown,
                    },
                ]
            },
            json!({
                "aggregations": {
                    "filter#PER_COMPANY": {
                        "doc_count": 41,
                        "meta": { "_skip": true },
                        "PER_AGENT": {
                            "doc_count_error_upper_bound": 0,
                            "sum_other_doc_count": 0,
                            "buckets": [
                                {
                                    "key": "Denmark",
                                    "doc_count": 1
                                }
                            ]
                        }
                    }
                }
            })
        );

        // make it so order of arrays does not matter
        impl Ord for ComputedResult {
            fn cmp(&self, other: &Self) -> Ordering {
                self.parent
                    .cmp(&other.parent)
                    .then(self.name.cmp(&other.name))
                    .then_with(|| {
                        let mut self_fields = self.fields.clone();
                        self_fields.sort();

                        let mut other_fields = other.fields.clone();
                        other_fields.sort();

                        self_fields.cmp(&other_fields)
                    })
            }
        }

        impl PartialOrd for ComputedResult {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        // make it so order of arrays does not matter
        impl PartialEq for ComputedResult {
            fn eq(&self, other: &Self) -> bool {
                let mut self_fields = self.fields.clone();
                self_fields.sort();

                let mut other_fields = other.fields.clone();
                other_fields.sort();

                self.parent == other.parent
                    && self.name == other.name
                    && self_fields == other_fields
            }
        }

        impl Eq for ComputedResult {}
    }
}
