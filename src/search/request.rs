//! [Search request] types.
//!
//! [Search request]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-request-body.html

use serde::{Deserialize, Serialize};
use serde_json::json;

#[cfg(feature = "graphql")]
use crate::search::{query::CompoundQueryInput, SortInput};
use crate::{
    scalars::SortedValue,
    search::{query::CompoundQuery, Sort},
};

/// The [request body] for an Elasticsearch search request.
///
/// [request body]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-request-body.html
#[cfg(feature = "graphql")]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(async_graphql::InputObject, Serialize, Clone, Debug)]
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
    pub size: Option<u64>,

    /// The maximum number of documents to collect for each shard, upon reaching
    /// which the query execution will terminate early.
    ///
    /// Defaults to `0`, which does not terminate query execution early.
    #[graphql(default)]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub terminate_after: u64,

    /// The live cursor from which to search after to fascilitate [pagination].
    ///
    /// [pagination]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-request-body.html#request-body-search-search-after
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(rename = "search_after", skip_serializing_if = "Vec::is_empty")]
    pub after: Vec<SortedValue>,

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
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
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

    /// The maximum number of documents to collect for each shard, upon reaching
    /// which the query execution will terminate early.
    ///
    /// Defaults to `0`, which does not terminate query execution early.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub terminate_after: u64,

    /// The live cursor from which to search after to fascilitate [pagination].
    ///
    /// [pagination]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-request-body.html#request-body-search-search-after
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(rename = "search_after", skip_serializing_if = "Vec::is_empty")]
    pub after: Vec<SortedValue>,

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
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(async_graphql::InputObject, Serialize, Deserialize, Clone, Debug)]
pub struct HighlightOptionsInput {
    /// The field names and their options to highlight.
    pub fields: crate::scalars::Map,

    /// The highligher type to use.
    #[graphql(name = "type", default)]
    #[serde(rename = "type")]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub ty: HighlighterType,

    /// The maximum number of fragments to return.
    #[graphql(default_with = "5")]
    #[cfg_attr(feature = "builder", builder(default = 5, setter(into)))]
    pub number_of_fragments: u64,

    /// The size of the highlighted fragment in characters.
    #[graphql(default_with = "100")]
    #[cfg_attr(feature = "builder", builder(default = 100, setter(into)))]
    pub fragment_size: u32,

    /// How far to scan for boundary characters.
    #[graphql(default_with = "20")]
    #[cfg_attr(feature = "builder", builder(default = 20, setter(into)))]
    pub boundary_max_scan: u32,

    // TODO: should be an enum?
    /// Set to [`styled`] to use the built-in tag schema.
    ///
    /// [`styled`]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-request-highlighting.html
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub tags_schema: Option<String>,

    /// Use in conjunction with `post_tags` to define the HTML tags to use for
    /// the highlighted text. By default, highlighted text is wrapped in `<em>`
    /// and `</em>` tags.
    #[graphql(default)]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Vec::is_empty")] // es errors without this
    pub pre_tags: Vec<String>,

    /// Use in conjunction with `pre_tags` to define the HTML tags to use for
    /// the highlighted text. By default, highlighted text is wrapped in `<em>`
    /// and `</em>` tags.
    #[graphql(default)]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Vec::is_empty")] // es errors without this
    pub post_tags: Vec<String>,

    /// By default, only fields that contains a query match are highlighted. Set
    /// `require_field_match` to `false` to highlight all fields.
    #[graphql(default = true)]
    #[cfg_attr(feature = "builder", builder(default = true, setter(into)))]
    pub require_field_match: bool,
}

/// The [options] for highlighting.
///
/// **TODO**: add more options...
///
/// [options]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-request-highlighting.html#highlighting-settings
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HighlightOptions {
    /// The field names and their options to highlight.
    pub fields: crate::scalars::Map,

    /// The highligher type to use.
    #[serde(rename = "type")]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub ty: HighlighterType,

    /// The maximum number of fragments to return.
    #[cfg_attr(feature = "builder", builder(default = 5, setter(into)))]
    pub number_of_fragments: u64,

    /// The size of the highlighted fragment in characters.
    #[cfg_attr(feature = "builder", builder(default = 100, setter(into)))]
    pub fragment_size: u32,

    /// How far to scan for boundary characters.
    #[cfg_attr(feature = "builder", builder(default = 20, setter(into)))]
    pub boundary_max_scan: u32,

    // TODO: should be an enum?
    /// Set to [`styled`] to use the built-in tag schema.
    ///
    /// [`styled`]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-request-highlighting.html
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub tags_schema: Option<String>,

    /// Use in conjunction with `post_tags` to define the HTML tags to use for
    /// the highlighted text. By default, highlighted text is wrapped in `<em>`
    /// and `</em>` tags.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Vec::is_empty")] // es errors without this
    pub pre_tags: Vec<String>,

    /// Use in conjunction with `pre_tags` to define the HTML tags to use for
    /// the highlighted text. By default, highlighted text is wrapped in `<em>`
    /// and `</em>` tags.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Vec::is_empty")] // es errors without this
    pub post_tags: Vec<String>,

    /// By default, only fields that contains a query match are highlighted. Set
    /// `require_field_match` to `false` to highlight all fields.
    #[cfg_attr(feature = "builder", builder(default = true, setter(into)))]
    pub require_field_match: bool,
}

impl Default for HighlightOptions {
    #[inline]
    fn default() -> Self {
        HighlightOptions {
            fields: json!({ "*": {} }).into(),
            ty: HighlighterType::default(),
            number_of_fragments: 5,
            fragment_size: 100,
            boundary_max_scan: 20,
            tags_schema: Some("styled".to_string()),
            pre_tags: vec![],
            post_tags: vec![],
            require_field_match: true,
        }
    }
}

/// The different supported highlighter types/algorithm.
#[cfg_attr(feature = "graphql", derive(async_graphql::Enum, Eq, PartialEq, Copy))]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum HighlighterType {
    /// The `unified` highlighter uses the Lucene Unified Highlighter. This
    /// highlighter breaks the text into sentences and uses the BM25 algorithm
    /// to score individual sentences as if they were documents in the corpus.
    /// It also supports accurate phrase and multi-term (fuzzy, prefix, regex)
    /// highlighting. This is the default highlighter.
    Unified,

    /// The `plain` highlighter uses the standard Lucene highlighter. It
    /// attempts to reflect the query matching logic in terms of understanding
    /// word importance and any word positioning criteria in phrase queries.
    Plain,

    /// The `fvh` highlighter uses the Lucene Fast Vector highlighter. This
    /// highlighter can be used on fields with term_vector set to
    /// `with_positions_offsets` in the mapping. The fast vector highlighter:
    ///
    /// - Can be customized with a `boundary_scanner`.
    /// - Requires setting term_vector to with_positions_offsets which increases
    ///   the size of the index
    /// - Can combine matches from multiple fields into one result. See
    ///   `matched_fields`
    /// - Can assign different weights to matches at different positions
    ///   allowing for things like phrase matches being sorted above term
    ///   matches when highlighting a Boosting Query that boosts phrase matches
    ///   over term matches
    Fvh,
}

impl Default for HighlighterType {
    #[inline]
    fn default() -> Self {
        Self::Unified
    }
}
