//! [Exists query](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-exists-query.html#query-dsl-exists-query)

use serde::{Deserialize, Serialize};

/// An [Exists query] returns documents that contain a non-null or empty
/// (e.g. `[]`) value for a field.
///
/// [Exists query]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-exists-query.html#query-dsl-exists-query
#[async_graphql::InputObject]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Clone, Debug)]
pub struct ExistsFilterInput {
    /// The name of the field to query.
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub field: String,
}

/// An [Exists query] returns documents that contain a non-null or empty
/// (e.g. `[]`) value for a field.
///
/// [Exists query]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-exists-query.html#query-dsl-exists-query
#[async_graphql::SimpleObject]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExistsFilter {
    /// The name of the field to query.
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub field: String,
}

impl From<ExistsFilterInput> for ExistsFilter {
    #[inline]
    fn from(input: ExistsFilterInput) -> ExistsFilter {
        ExistsFilter { field: input.field }
    }
}
