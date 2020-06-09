//! [Exists query](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-exists-query.html#query-dsl-exists-query)

use serde::{Deserialize, Serialize};

/// An [Exists query] returns documents that contain a non-null or empty
/// (e.g. `[]`) value for a field.
///
/// [Exists query]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-exists-query.html#query-dsl-exists-query
#[cfg(feature = "graphql")]
#[async_graphql::InputObject(name = "ExistsFilterInput")]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Clone, Debug)]
pub struct ExistsQueryInput {
    /// The name of the field to query.
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub field: String,
}

/// An [Exists query] returns documents that contain a non-null or empty
/// (e.g. `[]`) value for a field.
///
/// [Exists query]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-exists-query.html#query-dsl-exists-query
#[async_graphql::SimpleObject(name = "ExistsFilter")]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExistsQuery {
    /// The name of the field to query.
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub field: String,
}

#[cfg(feature = "graphql")]
impl From<ExistsQueryInput> for ExistsQuery {
    #[inline]
    fn from(input: ExistsQueryInput) -> ExistsQuery {
        ExistsQuery { field: input.field }
    }
}
