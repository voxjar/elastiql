#![allow(clippy::missing_docs_in_private_items)]

//! Facilitates [filtering] documents in the database.
//!
//! [filtering]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl.html

use std::default::Default;

use serde::{Deserialize, Serialize};

pub use self::types::*;

mod types;

// TODO: make this file smaller!
// TODO: improve filter UX regarding builders/into

/// [Compound queries] wrap other compound or leaf queries, either to combine
/// their results and scores, to change their behavior, or to switch from query
/// to filter context.
///
/// [Compound queries]: https://www.elastic.co/guide/en/elasticsearch/reference/current/compound-queries.html

#[async_graphql::InputObject]
#[derive(Serialize, Default, Clone, Debug)]
pub struct CompoundFilterInput {
    /// The default query for combining multiple leaf or compound query clauses,
    /// as must, should, must_not, or filter clauses. The must and should
    /// clauses have their scores combined — the more matching clauses, the
    /// better — while the must_not and filter clauses are executed in filter
    /// context.
    #[field(name = "bool")]
    #[serde(default, rename = "bool", skip_serializing_if = "Option::is_none")]
    boolean: Option<BooleanFilterInput>,
}

impl CompoundFilterInput {
    /// Returns `true` if this `CompoundFilterInput` is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.boolean
            .as_ref()
            .map_or_else(|| true, |filter| filter.is_empty())
    }

    /// Appends a `filter` on to the current list of filters.
    #[inline]
    pub fn push(&mut self, filter: impl Into<FilterInput>) {
        if let Some(ref mut boolean) = self.boolean {
            boolean.push(filter)
        } else {
            self.boolean = Some(BooleanFilterInput {
                must: vec![],
                filter: vec![filter.into()],
                should: vec![],
                must_not: vec![],
                minimum_should_match: None,
                boost: None,
            })
        }
    }
}

impl From<Option<CompoundFilterInput>> for CompoundFilterInput {
    #[inline]
    fn from(filter: Option<CompoundFilterInput>) -> CompoundFilterInput {
        filter.unwrap_or_default()
    }
}

impl<T: Into<BooleanFilterInput>> From<T> for CompoundFilterInput {
    #[inline]
    fn from(filter: T) -> CompoundFilterInput {
        CompoundFilterInput {
            boolean: Some(filter.into()),
        }
    }
}

/// [Compound queries] wrap other compound or leaf queries, either to combine
/// their results and scores, to change their behavior, or to switch from query
/// to filter context.
///
/// [Compound queries]: https://www.elastic.co/guide/en/elasticsearch/reference/current/compound-queries.html
#[async_graphql::SimpleObject]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct CompoundFilter {
    /// The default query for combining multiple leaf or compound query clauses,
    /// as must, should, must_not, or filter clauses. The must and should
    /// clauses have their scores combined — the more matching clauses, the
    /// better — while the must_not and filter clauses are executed in filter
    /// context.
    // #[field(name = "bool")]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, rename = "bool", skip_serializing_if = "Option::is_none")]
    boolean: Option<BooleanFilter>,
}

#[cfg(feature = "builder")]
impl CompoundFilter {
    /// Returns `true` if this `CompoundFilter` is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.boolean
            .as_ref()
            .map_or_else(|| true, |filter| filter.is_empty())
    }
}

impl<T: Into<CompoundFilterInput>> From<T> for CompoundFilter {
    #[inline]
    fn from(input: T) -> CompoundFilter {
        CompoundFilter {
            boolean: Some(input.into().boolean.unwrap_or_default().into()),
        }
    }
}

/// A [query] that matches documents matching boolean combinations of other
/// queries. It is built using one or more boolean clauses, each clause with a
/// typed occurrence.
///
/// [query]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-bool-query.html

#[async_graphql::InputObject]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Default, Clone, Debug)]
pub struct BooleanFilterInput {
    /// The clause (query) must appear in matching documents and will
    /// contribute to the score of this query.
    #[field(default)]
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    must: Vec<FilterInput>,

    /// The clause (query) must appear in matching documents. However unlike
    /// must, the score of the query will be ignored. Filter clauses are executed
    /// in [filter context], meaning that scoring is ignored and clauses are
    /// considered for caching.
    ///
    /// [filter context]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html
    #[field(default)]
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    filter: Vec<FilterInput>,

    /// The clause (query) should appear in the matching document.
    #[field(default)]
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    should: Vec<FilterInput>,

    /// The clause (query) must not appear in the matching documents. Clauses
    /// are executed in [filter context] meaning that scoring is ignored and
    /// clauses are considered for caching. Because scoring is ignored, a score
    /// of 0 for all documents is returned.
    ///
    /// [filter context]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html
    #[field(default)]
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    must_not: Vec<FilterInput>,

    /// [Controls] how many optional (`should`) parameters must match.
    ///
    /// | Example       | Description                                                                                                                                                                                   |
    /// |---------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
    /// | `3`           | *Fixed value* regardless of the number of optional clauses                                                                                                                                    |
    /// | `-2`          | Total number of optional clauses, *minus* this number should be mandatory                                                                                                                     |
    /// | `75%`         | *Percent* of the total number of optional clauses are *necessary*. The number computed from the percentage is rounded down and used as the minimum.                                           |
    /// | `-25%`        | *Percent* of the total number of optional clauses can be *missing*. The number computed from the percentage is rounded down, before being subtracted from the total to determine the minimum. |
    /// | `3<90%`       | *e.g.*: if there are 1 to 3 clauses they are all required, but for 4 or more clauses only 90% are required.                                                                                   |
    /// | `2<-25% 9<-3` | *e.g.*: if there are 1 or 2 clauses both are required, if there are 3-9 clauses all but 25% are required, and if there are more than 9 clauses, all but three are required.                   |
    ///
    /// [Controls]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-minimum-should-match.html
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    minimum_should_match: Option<String>,

    /// Floating point number used to decrease or increase the
    /// [relevance scores] of a query. (Defaults to `1.0`.)
    ///
    /// You can use the boost parameter to adjust relevance scores for searches
    /// containing two or more queries.
    ///
    /// Boost values are relative to the default value of `1.0`. A boost value
    /// between `0` and `1.0` decreases the relevance score. A value greater
    /// than  `1.0` increases the relevance score.
    ///
    /// [relevance scores]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html#relevance-scores
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    boost: Option<f64>,
}

impl BooleanFilterInput {
    /// Returns `true` if this `BooleanFilterInput` is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.must.is_empty()
            && self.filter.is_empty()
            && self.should.is_empty()
            && self.must_not.is_empty()
    }

    /// Appends a `filter` to the current list of filters.
    #[inline]
    pub fn push(&mut self, filter: impl Into<FilterInput>) {
        // TODO: should we always default to `filter` context?
        self.filter.push(filter.into())
    }
}

impl<T: Into<FilterInput>> From<T> for BooleanFilterInput {
    #[inline]
    fn from(filter: T) -> BooleanFilterInput {
        BooleanFilterInput {
            must: vec![],
            filter: vec![filter.into()],
            should: vec![],
            must_not: vec![],
            minimum_should_match: None,
            boost: None,
        }
    }
}

/// A [query] that matches documents matching boolean combinations of other
/// queries. It is built using one or more boolean clauses, each clause with a
/// typed occurrence.
///
/// [query]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-bool-query.html
#[async_graphql::SimpleObject]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct BooleanFilter {
    /// The clause (query) **must** appear in matching documents and *will
    /// contribute to the score* of this query.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    must: Vec<Filter>,

    /// The clause (query) **must** appear in matching documents. However unlike
    /// `must`, the *score of the query will be ignored*. Filter clauses are
    /// executed in [filter context], meaning that scoring is ignored and
    /// clauses are considered for caching.
    ///
    /// [filter context]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    filter: Vec<Filter>,

    /// The clause (query) **should** appear in the matching document.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    should: Vec<Filter>,

    /// The clause (query) **must not** appear in the matching documents. Clauses
    /// are executed in [filter context] meaning that *scoring is ignored* and
    /// clauses are considered for caching. Because scoring is ignored, a score
    /// of 0 for all documents is returned.
    ///
    /// [filter context]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    must_not: Vec<Filter>,

    /// [Controls] how many optional (`should`) parameters must match.
    ///
    /// | Example       | Description                                                                                                                                                                                   |
    /// |---------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
    /// | `3`           | *Fixed value* regardless of the number of optional clauses                                                                                                                                    |
    /// | `-2`          | Total number of optional clauses, *minus* this number should be mandatory                                                                                                                     |
    /// | `75%`         | *Percent* of the total number of optional clauses are *necessary*. The number computed from the percentage is rounded down and used as the minimum.                                           |
    /// | `-25%`        | *Percent* of the total number of optional clauses can be *missing*. The number computed from the percentage is rounded down, before being subtracted from the total to determine the minimum. |
    /// | `3<90%`       | *e.g.*: if there are 1 to 3 clauses they are all required, but for 4 or more clauses only 90% are required.                                                                                   |
    /// | `2<-25% 9<-3` | *e.g.*: if there are 1 or 2 clauses both are required, if there are 3-9 clauses all but 25% are required, and if there are more than 9 clauses, all but three are required.                   |
    ///
    /// [Controls]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-minimum-should-match.html
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    minimum_should_match: Option<String>,

    /// Floating point number used to decrease or increase the
    /// [relevance scores] of a query. (Defaults to `1.0`.)
    ///
    /// You can use the boost parameter to adjust relevance scores for searches
    /// containing two or more queries.
    ///
    /// Boost values are relative to the default value of `1.0`. A boost value
    /// between `0` and `1.0` decreases the relevance score. A value greater
    /// than  `1.0` increases the relevance score.
    ///
    /// [relevance scores]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html#relevance-scores
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    boost: Option<f64>,
}

#[cfg(feature = "builder")]
impl BooleanFilter {
    /// Returns `true` if this `BooleanFilter` is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.must.is_empty()
            && self.filter.is_empty()
            && self.should.is_empty()
            && self.must_not.is_empty()
    }
}

impl From<BooleanFilterInput> for BooleanFilter {
    #[inline]
    fn from(input: BooleanFilterInput) -> BooleanFilter {
        // TODO: why isn't the blanket impl in the std library auto impl these?
        BooleanFilter {
            must: input.must.into_iter().map(Into::into).collect(),
            filter: input.filter.into_iter().map(Into::into).collect(),
            should: input.should.into_iter().map(Into::into).collect(),
            must_not: input.must_not.into_iter().map(Into::into).collect(),
            minimum_should_match: input.minimum_should_match.map(Into::into),
            boost: input.boost.map(Into::into),
        }
    }
}

impl<T: Into<Filter>> From<T> for BooleanFilter {
    #[inline]
    fn from(filter: T) -> BooleanFilter {
        BooleanFilter {
            must: vec![],
            filter: vec![filter.into()],
            should: vec![],
            must_not: vec![],
            minimum_should_match: None,
            boost: None,
        }
    }
}

/// A single query to perform for this search request.
///
/// **Note**: If a filter over a list of objects does not return the
/// expected results, try a `NestedFilterInput`.
///
/// **Note**: Specifying more than one field will result in an error.
///
/// **TODO**: Change this type once [union input types] are supported by GraphQL
/// to only allow specifying a single field.
///
/// [union input types]: https://github.com/graphql/graphql-spec/blob/master/rfcs/InputUnion.md
#[async_graphql::InputObject]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Clone, Debug)]
pub struct FilterInput {
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    exists: Option<ExistsFilterInput>,

    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    term: Option<TermFilterInput>,

    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    terms: Option<TermsFilterInput>,

    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    range: Option<RangeFilterInput>,

    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    regexp: Option<RegexpFilterInput>,

    #[field(name = "match")]
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<MatchFilterInput>,

    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    simple_query_string: Option<SimpleQueryStringFilterInput>,

    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    query_string: Option<QueryStringFilterInput>,

    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    nested: Option<NestedFilterInput>,

    /// A nested bool query.
    #[field(name = "bool")]
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    #[serde(rename = "bool", default, skip_serializing_if = "Option::is_none")]
    boolean: Option<BooleanFilterInput>,
}

impl From<ExistsFilterInput> for FilterInput {
    #[inline]
    fn from(filter: ExistsFilterInput) -> FilterInput {
        FilterInput {
            exists: Some(filter),
            term: None,
            terms: None,
            range: None,
            regexp: None,
            match_: None,
            simple_query_string: None,
            query_string: None,
            nested: None,
            boolean: None,
        }
    }
}

impl From<TermFilterInput> for FilterInput {
    #[inline]
    fn from(filter: TermFilterInput) -> FilterInput {
        FilterInput {
            exists: None,
            term: Some(filter),
            terms: None,
            range: None,
            regexp: None,
            match_: None,
            simple_query_string: None,
            query_string: None,
            nested: None,
            boolean: None,
        }
    }
}

impl From<TermsFilterInput> for FilterInput {
    #[inline]
    fn from(filter: TermsFilterInput) -> FilterInput {
        FilterInput {
            exists: None,
            term: None,
            terms: Some(filter),
            range: None,
            regexp: None,
            match_: None,
            simple_query_string: None,
            query_string: None,
            nested: None,
            boolean: None,
        }
    }
}

impl From<RangeFilterInput> for FilterInput {
    #[inline]
    fn from(filter: RangeFilterInput) -> FilterInput {
        FilterInput {
            exists: None,
            term: None,
            terms: None,
            range: Some(filter),
            regexp: None,
            match_: None,
            simple_query_string: None,
            query_string: None,
            nested: None,
            boolean: None,
        }
    }
}

impl From<RegexpFilterInput> for FilterInput {
    #[inline]
    fn from(filter: RegexpFilterInput) -> FilterInput {
        FilterInput {
            exists: None,
            term: None,
            terms: None,
            range: None,
            regexp: Some(filter),
            match_: None,
            simple_query_string: None,
            query_string: None,
            nested: None,
            boolean: None,
        }
    }
}

impl From<MatchFilterInput> for FilterInput {
    #[inline]
    fn from(filter: MatchFilterInput) -> FilterInput {
        FilterInput {
            exists: None,
            term: None,
            terms: None,
            range: None,
            regexp: None,
            match_: Some(filter),
            simple_query_string: None,
            query_string: None,
            nested: None,
            boolean: None,
        }
    }
}

impl From<SimpleQueryStringFilterInput> for FilterInput {
    #[inline]
    fn from(filter: SimpleQueryStringFilterInput) -> FilterInput {
        FilterInput {
            exists: None,
            term: None,
            terms: None,
            range: None,
            regexp: None,
            match_: None,
            simple_query_string: Some(filter),
            query_string: None,
            nested: None,
            boolean: None,
        }
    }
}

impl From<QueryStringFilterInput> for FilterInput {
    #[inline]
    fn from(filter: QueryStringFilterInput) -> FilterInput {
        FilterInput {
            exists: None,
            term: None,
            terms: None,
            range: None,
            regexp: None,
            match_: None,
            simple_query_string: None,
            query_string: Some(filter),
            nested: None,
            boolean: None,
        }
    }
}

impl From<NestedFilterInput> for FilterInput {
    #[inline]
    fn from(filter: NestedFilterInput) -> FilterInput {
        FilterInput {
            exists: None,
            term: None,
            terms: None,
            range: None,
            regexp: None,
            match_: None,
            simple_query_string: None,
            query_string: None,
            nested: Some(filter),
            boolean: None,
        }
    }
}

/// A single query to perform for this search request.
///
/// **Note**: This will *never* have more than *one* defined (and non-null) field.
#[async_graphql::SimpleObject]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Filter {
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    exists: Option<ExistsFilter>,

    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    term: Option<TermFilter>,

    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    terms: Option<TermsFilter>,

    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    range: Option<RangeFilter>,

    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    regexp: Option<RegexpFilter>,

    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<MatchFilter>,

    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    simple_query_string: Option<SimpleQueryStringFilter>,

    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    query_string: Option<QueryStringFilter>,

    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    nested: Option<NestedFilter>,

    /// A nested bool query.
    #[field(name = "bool")]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(rename = "bool", default, skip_serializing_if = "Option::is_none")]
    boolean: Option<BooleanFilter>,
}

impl From<FilterInput> for Filter {
    #[inline]
    fn from(input: FilterInput) -> Filter {
        Filter {
            exists: input.exists.map(Into::into),
            term: input.term.map(Into::into),
            terms: input.terms.map(Into::into),
            range: input.range.map(Into::into),
            regexp: input.regexp.map(Into::into),
            match_: input.match_.map(Into::into),
            simple_query_string: input.simple_query_string.map(Into::into),
            query_string: input.query_string.map(Into::into),
            nested: input.nested.map(Into::into),
            boolean: input.boolean.map(Into::into),
        }
    }
}

/// Describes a field that can be queried and its type.
#[async_graphql::SimpleObject]
#[derive(Debug)]
pub struct FilterField {
    /// The field name.
    pub field: String,

    /// The type
    ///
    /// TODO: make this an enum
    pub ty: String,
}

impl FilterField {
    /// Create a new `FilterField`.
    #[inline]
    pub fn new(field: impl Into<String>, ty: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            ty: ty.into(),
        }
    }
}
