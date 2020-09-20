use std::collections::HashMap;
#[cfg(feature = "graphql")]
use std::convert::TryFrom;

use serde::Deserialize;

/// The database response for performing a `Search`.
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Response<T> {
    /// An `Ok` response.
    Ok(OkResponse<T>),

    /// An `Err` response.
    Err {
        /// The error.
        error: ErrResponse,

        /// The [HTTP status code](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status).
        status: u16,
    },
}

/// The `Error` details from performing a failed Elasticsearch query.
#[derive(Deserialize, Debug)]
pub struct ErrResponse {
    /// The error type.
    #[serde(rename = "type")]
    pub ty: String,

    /// The reason/message for this error.
    pub reason: String,

    /// The name of the relevant Elasticsearch index.
    pub index: String,

    /// The `UUID` of the relevant Elasticsearch index.
    pub index_uuid: String,

    /// The root cause of this error.
    #[serde(default = "Vec::new")]
    pub root_cause: Vec<ErrResponse>,
}

// #[non_exhaustive]
// #[derive(Deserialize, PartialEq, Debug)]
// #[serde(untagged, rename_all = "snake_case")]
// pub enum ErrType {
//     IndexNotFoundException,
//     // TODO: map other error types
//     Unknown(String),
// }

/// The response for performing a successful `Search`.
#[derive(Deserialize, Debug)]
pub struct OkResponse<T> {
    /// Time it took for the database to process the request.
    pub took: u64,

    /// Whether or not the database request timed out before completing.
    pub timed_out: bool,

    /// The hits matched by the search query.
    pub hits: Hits<T>,
}

/// The hits/matches from performing a Elasticsearch search.
#[derive(Deserialize, Default, Debug)]
pub struct Hits<T> {
    /// The total count of the hits/matches.
    #[serde(default, rename = "total")]
    pub total_count: Count,

    /// The maximum score for any of the hits/matches.
    #[serde(default)]
    pub max_score: Option<f32>,

    // XXX: must be `Vec::new` instead of normal `default`
    /// The search hits.
    #[serde(default = "Vec::new")]
    pub hits: Vec<Hit<T>>,
}

impl<T> Hits<T> {
    /// Gets the first document's source (if any).
    #[inline]
    pub fn first_doc(&self) -> Option<&T> {
        self.hits.get(0).map(|hit| &hit.source)
    }
}

/// An individual Elasticsearch search hit/match.
#[derive(Deserialize, Debug)]
pub struct Hit<T> {
    // TODO: should this be a different type?
    /// The database Id of this `Document`.
    #[serde(rename = "_id")]
    pub id: String,

    /// The name of the database index that this `Document` belongs to.
    ///
    /// **TODO**: should this be a different type?
    #[serde(rename = "_index")]
    pub index: String,

    /// The actual `Document` of this search hit/match.
    #[serde(rename = "_source")]
    pub source: T,

    /// The [version] number of this `Document`.
    ///
    /// [version]: https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-index_.html#index-versioning
    #[serde(rename = "_version")]
    pub version: Option<u64>,

    /// The sequence number number of this `Document`, used for
    /// [optimistic concurrency control].
    ///
    /// [optimistic concurrency control]: https://www.elastic.co/guide/en/elasticsearch/reference/current/optimistic-concurrency-control.html
    #[serde(rename = "_seq_no")]
    pub sequence_number: Option<u64>,

    /// The primary term of this `Document`, used for
    /// [optimistic concurrency control].
    ///
    /// [optimistic concurrency control]: https://www.elastic.co/guide/en/elasticsearch/reference/current/optimistic-concurrency-control.html
    #[serde(rename = "_primary_term")]
    pub primary_term: Option<u64>,

    /// The relevance score for this search hit.
    #[serde(rename = "_score")]
    pub score: Option<f32>,

    // TODO: make this a GraphQL object
    /// The [highlighted] snippets of the part(s) of the field(s) matching the
    /// search query.
    ///
    /// [highlighted]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-request-highlighting.html
    #[serde(default)]
    pub highlight: HashMap<String, Vec<String>>,

    /// The live cursor from which to search after to fascilitate [pagination].
    ///
    /// [pagination]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-request-body.html#request-body-search-search-after
    #[serde(default)]
    pub sort: Vec<serde_json::Value>,
}

/// The type of count.
#[cfg_attr(feature = "graphql", derive(async_graphql::Enum, Eq, PartialEq, Copy))]
#[cfg_attr(feature = "graphql", graphql(name = "SearchCountRelation"))]
#[derive(Deserialize, Clone, Debug)]
pub enum CountRelation {
    /// An exact count.
    #[serde(rename = "eq")]
    EqualTo,

    /// A lower bound estimate count.
    #[serde(rename = "gte")]
    GreaterThanOrEqualTo,
}

impl Default for CountRelation {
    #[inline]
    fn default() -> Self {
        CountRelation::EqualTo
    }
}

/// The total count of the hits/matches.
#[derive(Deserialize, Default, Debug)]
pub struct Count {
    /// The type of count this is.
    pub relation: CountRelation,

    /// The actual count.
    pub value: u64,
}

/// The total count of the hits/matches.
#[cfg(feature = "graphql")]
#[async_graphql::Object(name = "SearchCount")]
impl Count {
    /// The type of count this is.
    async fn relation(&self) -> &CountRelation {
        &self.relation
    }

    /// The actual count.
    ///
    /// **FIXME**: overflow possible; make this a custom scalar type
    async fn value(&self) -> async_graphql::FieldResult<i32> {
        Ok(i32::try_from(self.value)?)
    }
}

// TODO: add tests!
