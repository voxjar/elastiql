//! [Search request] types.
//!
//! [Search request]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-request-body.html

use serde::{Deserialize, Serialize};

#[cfg(feature = "graphql")]
use crate::search::{query::CompoundQueryInput, SortInput};
use crate::{
    scalars::NumberOrString,
    search::{query::CompoundQuery, Sort},
};

/// The [request body] for an Elasticsearch search request.
///
/// [request body]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-request-body.html
#[cfg(feature = "graphql")]
#[async_graphql::InputObject]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Clone, Debug)]
pub struct RequestInput {
    /// The query to perform in this search request.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "CompoundQueryInput::is_empty")]
    pub query: CompoundQueryInput,

    /// Sorts the results.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sort: Vec<SortInput>,

    /// The number of results to return.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,

    /// The live cursor from which to search after to fascilitate [pagination].
    ///
    /// [pagination]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-request-body.html#request-body-search-search-after
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(rename = "search_after", skip_serializing_if = "Vec::is_empty")]
    pub after: Vec<NumberOrString>,

    /// Whether or not to include the document version in the search results.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub version: bool,

    /// Whether or not to include the [sequence number & primary term] in the
    /// search results.
    ///
    /// [sequence number & primary term]: https://www.elastic.co/guide/en/elasticsearch/reference/current/optimistic-concurrency-control.html
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub seq_no_primary_term: bool,

    // TODO: could also be a bool...
    /// The lower bound for the number of hits to track
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_total_hits: Option<u64>,

    // TODO: figure out a way to not use this for queries that don't support it like `count`
    /// The [highlighted] snippets of the part(s) of the field(s) matching the
    /// search query.
    ///
    /// [highlighted]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-request-highlighting.html
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlight: Option<HighlightOptionsInput>,
}

#[cfg(feature = "graphql")]
impl RequestInput {
    /// Get a mutable reference to the [`CompoundQueryInput`].
    #[inline]
    pub fn query_mut(&mut self) -> &mut CompoundQueryInput {
        &mut self.query
    }
}

/// The [request body] for an Elasticsearch search request.
///
/// [request body]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-request-body.html
#[cfg_attr(feature = "graphql", async_graphql::SimpleObject)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Clone, Debug)]
pub struct Request {
    /// The query to perform in this search request.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "CompoundQuery::is_empty")]
    pub query: CompoundQuery,

    /// Sorts the results.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sort: Vec<Sort>,

    /// The number of results to return.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,

    /// The live cursor from which to search after to fascilitate [pagination].
    ///
    /// [pagination]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-request-body.html#request-body-search-search-after
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(rename = "search_after", skip_serializing_if = "Vec::is_empty")]
    pub after: Vec<NumberOrString>,

    /// Whether or not to include the document version in the search results.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub version: bool,

    /// Whether or not to include the [sequence number & primary term] in the
    /// search results.
    ///
    /// [sequence number & primary term]: https://www.elastic.co/guide/en/elasticsearch/reference/current/optimistic-concurrency-control.html
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub seq_no_primary_term: bool,

    // TODO: could also be a bool...
    /// The lower bound for the number of hits to track
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_total_hits: Option<u64>,

    // TODO: figure out a way to not use this for queries that don't support it like `count`
    /// The [highlighted] snippets of the part(s) of the field(s) matching the
    /// search query.
    ///
    /// [highlighted]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-request-highlighting.html
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlight: Option<HighlightOptions>,
}

impl Request {
    /// Get a mutable reference to the [`CompoundQuery`].
    #[inline]
    pub fn query_mut(&mut self) -> &mut CompoundQuery {
        &mut self.query
    }
}

/// The [options] for highlighting.
///
/// **TODO**: add more options...
///
/// [options]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-request-highlighting.html#highlighting-settings
#[cfg(feature = "graphql")]
#[async_graphql::InputObject]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct HighlightOptionsInput {
    // TODO: should be an enum?
    /// Set to [`styled`] to use the built-in tag schema.
    ///
    /// [`styled`]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-request-highlighting.html
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub tags_schema: String,

    /// The maximum number of fragments to return.
    pub number_of_fragments: i32,

    /// The field names and their options to highlight.
    pub fields: crate::scalars::Map,
}

/// The [options] for highlighting.
///
/// **TODO**: add more options...
///
/// [options]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-request-highlighting.html#highlighting-settings
#[cfg_attr(feature = "graphql", async_graphql::SimpleObject)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct HighlightOptions {
    // TODO: should be an enum?
    /// Set to [`styled`] to use the built-in tag schema.
    ///
    /// [`styled`]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-request-highlighting.html
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub tags_schema: String,

    /// The maximum number of fragments to return.
    pub number_of_fragments: i32,

    /// The field names and their options to highlight.
    pub fields: crate::scalars::Map,
}
