//! [Query string query](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-query-string-query.html)

use serde::{Deserialize, Serialize};

// TODO: add additional options
/// [Query string] returns documents based on a provided query string, using a
/// parser with a strict syntax.
///
/// This query uses a [syntax] to parse and split the provided query string
/// based on operators, such as `AND` or `NOT`. The query then [analyzes] each
/// split text independently before returning matching documents.
///
/// [Query string]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-query-string-query.html
/// [syntax]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-query-string-query.html#query-string-syntax
/// [analyzes]: https://www.elastic.co/guide/en/elasticsearch/reference/current/analysis.html

#[cfg(feature = "graphql")]
#[async_graphql::InputObject(name = "QueryStringFilterInput")]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Clone, Debug)]
pub struct QueryStringQueryInput {
    /// The name of the fields to query.
    ///
    /// Defaults to all field that have full text search enabled.
    ///
    /// Accepts wildcard expressions. You also can boost relevance scores for
    /// matches to particular fields using a caret (`^`) notation. See
    /// [Wildcards and per-field boosts in the fields parameter] for examples.
    ///
    /// [Wildcards and per-field boosts in the fields parameter]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-simple-query-string-query.html#simple-query-string-boost
    #[field(default)]
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<String>,

    /// The query to run in the [simple query string syntax](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-simple-query-string-query.html#simple-query-string-syntax).
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub query: String,
}

// TODO: add additional options
/// [Query string] returns documents based on a provided query string, using a
/// parser with a strict syntax.
///
/// This query uses a [syntax] to parse and split the provided query string
/// based on operators, such as `AND` or `NOT`. The query then [analyzes] each
/// split text independently before returning matching documents.
///
/// [Query string]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-query-string-query.html
/// [syntax]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-query-string-query.html#query-string-syntax
/// [analyzes]: https://www.elastic.co/guide/en/elasticsearch/reference/current/analysis.html
#[async_graphql::SimpleObject(name = "QueryStringFilter")]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct QueryStringQuery {
    /// The name of the fields to query.
    ///
    /// Defaults to all field that have full text search enabled.
    ///
    /// Accepts wildcard expressions. You also can boost relevance scores for
    /// matches to particular fields using a caret (`^`) notation. See
    /// [Wildcards and per-field boosts in the fields parameter] for examples.
    ///
    /// [Wildcards and per-field boosts in the fields parameter]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-simple-query-string-query.html#simple-query-string-boost
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<String>,

    /// The query to run in the [simple query string syntax](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-simple-query-string-query.html#simple-query-string-syntax).
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub query: String,
}

#[cfg(feature = "graphql")]
impl From<QueryStringQueryInput> for QueryStringQuery {
    #[inline]
    fn from(input: QueryStringQueryInput) -> QueryStringQuery {
        QueryStringQuery {
            fields: input.fields,
            query: input.query,
        }
    }
}
