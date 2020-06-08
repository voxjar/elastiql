//! [Search request] types.
//!
//! [Search request]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-request-body.html

use std::{default::Default, result::Result, str, string::ToString};

use serde::{ser, Deserialize, Serialize, Serializer};
use serde_json::{json, Value as JsonValue};

use crate::search::{CompoundFilterInput, Cursor, SortInput};

/// The [request body] for an Elasticsearch search request.
///
/// [request body]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-request-body.html
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Clone, Debug)]
pub struct Request {
    /// Filters the search results.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(
        rename = "query",
        skip_serializing_if = "CompoundFilterInput::is_empty"
    )]
    pub filter: CompoundFilterInput,

    /// Sorts the results.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sort: Vec<SortInput>,

    /// The number of results to return.
    ///
    /// **TODO**: this should be `u32` or some non-negative number...
    /// **TODO**: this should be an `Option` so we don't have to copy Es' max here
    #[cfg_attr(feature = "builder", builder(default = 9_999, setter(into)))]
    #[serde(rename = "size")]
    // TODO: better solution
    #[cfg_attr(not(test), serde(serialize_with = "serialize_by_adding_1"))]
    pub first: i32,

    /// The [opaque cursor] to return results after to facilitate [pagination].
    ///
    /// [pagination]: https://facebook.github.io/relay/graphql/connections.htm#
    /// [opaque cursor]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-request-search-after.html
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(
        rename = "search_after",
        skip_serializing_if = "Cursor::is_empty_or_default"
    )]
    pub after: Cursor,

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
    // TODO: use elasticsearch default of 1_000 for `track_total_hits`?
    /// The lower bound of hits to track
    ///
    /// **TODO**: expose `track_total_hits` via GraphQL?
    #[cfg_attr(feature = "builder", builder(default = 9_999_999, setter(into)))]
    pub track_total_hits: u64,

    // TODO: figure out a way to not use this for queries that don't support it like `count`
    /// The [highlighted] snippets of the part(s) of the field(s) matching the
    /// search query.
    ///
    /// [highlighted]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-request-highlighting.html
    #[cfg_attr(feature = "builder", builder(default = Some(HighlightOptions::default()), setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlight: Option<HighlightOptions>,
}

impl Request {
    /// Get a mutable reference to the [`CompoundFilterInput`].
    #[inline]
    pub fn filter_mut(&mut self) -> &mut CompoundFilterInput {
        &mut self.filter
    }
}

// HACK: used for pagination to determine if there is another page...
#[cfg_attr(test, allow(dead_code))] // TODO: better solution
#[allow(clippy::trivially_copy_pass_by_ref)]
fn serialize_by_adding_1<S>(first: &i32, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_i32(
        first
            .checked_add(1)
            .ok_or("overflow")
            .map_err(ser::Error::custom)?,
    )
}

/// The [options] for highlighting.
///
/// **TODO**: add more options...
/// **TODO**: expose via GraphQL?
///
/// [options]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-request-highlighting.html#highlighting-settings
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
    pub number_of_fragments: u32,

    // TODO: this should be a struct...
    /// The field names and their options to highlight.
    pub fields: JsonValue,
}

impl Default for HighlightOptions {
    #[inline]
    fn default() -> Self {
        HighlightOptions {
            tags_schema: "styled".to_string(),
            number_of_fragments: 999, // HACK
            // TODO(perf): use `LookAheadSelection` instead of always highlighting all fields
            fields: json!({
                "*": {},
                "transcript": { "type": "annotated", "require_field_match": false },
            }),
        }
    }
}
