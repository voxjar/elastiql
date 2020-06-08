//! [Nested query](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-nested-query.html)

use serde::{Deserialize, Serialize};

use super::super::{CompoundFilter, CompoundFilterInput};

/// A [Nested query] wraps another query to search [nested] fields.
///
/// The [nested query] searches nested field objects as if they were indexed as
/// separate documents. If an object matches the search, the nested query
/// returns the root parent document.
///
/// [Nested query]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-nested-query.html
/// [nested]: https://www.elastic.co/guide/en/elasticsearch/reference/current/nested.html

#[async_graphql::InputObject]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Clone, Debug)]
pub struct NestedFilterInput {
    /// Path to the nested object to search.
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub path: String,

    /// Query to run on nested objects in the path. If an object
    /// matches the search, the nested query returns the root parent document.
    ///
    /// You can search nested fields using dot notation that includes the
    /// complete path, such as `obj1.name`.
    ///
    /// Multi-level nesting is automatically supported, and detected, resulting
    /// in an inner nested query to automatically match the relevant nesting
    /// level, rather than root, if it exists within another nested query.
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub query: CompoundFilterInput,

    /// Indicates whether to ignore an unmapped path and not return any
    /// documents instead of an error.
    #[field(default)]
    #[cfg_attr(feature = "builder", builder(default))]
    pub ignore_unmapped: bool,
}

impl NestedFilterInput {
    /// Constructs a new `NestedFilterInput`.
    #[inline]
    pub fn new(path: impl Into<String>, query: impl Into<CompoundFilterInput>) -> Self {
        NestedFilterInput {
            path: path.into(),
            query: query.into(),
            ignore_unmapped: true,
        }
    }
}

/// A [Nested query] wraps another query to search [nested] fields.
///
/// The [nested query] searches nested field objects as if they were indexed as
/// separate documents. If an object matches the search, the nested query
/// returns the root parent document.
///
/// [Nested query]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-nested-query.html
/// [nested]: https://www.elastic.co/guide/en/elasticsearch/reference/current/nested.html
#[async_graphql::SimpleObject]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NestedFilter {
    /// Path to the nested object to search.
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub path: String,

    /// Query to run on nested objects in the path. If an object
    /// matches the search, the nested query returns the root parent document.
    ///
    /// You can search nested fields using dot notation that includes the
    /// complete path, such as `obj1.name`.
    ///
    /// Multi-level nesting is automatically supported, and detected, resulting
    /// in an inner nested query to automatically match the relevant nesting
    /// level, rather than root, if it exists within another nested query.
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub query: CompoundFilter,

    /// Indicates whether to ignore an unmapped path and not return any
    /// documents instead of an error.
    #[cfg_attr(feature = "builder", builder(default))]
    pub ignore_unmapped: bool,
}

impl From<NestedFilterInput> for NestedFilter {
    #[inline]
    fn from(input: NestedFilterInput) -> NestedFilter {
        NestedFilter {
            path: input.path,
            query: input.query.into(),
            ignore_unmapped: input.ignore_unmapped,
        }
    }
}
