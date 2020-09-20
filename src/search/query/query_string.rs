//! [Query string query](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-query-string-query.html)

use serde::{Deserialize, Serialize};

// NOTE: some fields require `skip_serializing_if` otherwise Elasticsearch
// will return an error if e.g. `null` is used

// TODO: make all i32 types u64 once async-graphql supports it

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
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(async_graphql::InputObject, Serialize, Clone, Debug)]
#[graphql(name = "QueryStringFilterInput")]
pub struct QueryStringQueryInput {
    /// The query to run in the [simple query string syntax](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-simple-query-string-query.html#simple-query-string-syntax).
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub query: String,

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

    /// Default field you wish to search if no field is provided in the query
    /// string.
    ///
    /// Defaults to the `index.query.default_field` index setting, which has a
    /// default value of `*`. The `*` value extracts all fields that are
    /// eligible for term queries and filters the metadata fields. All extracted
    /// fields are then combined to build a query if no `prefix` is specified.
    ///
    /// Searching across all eligible fields does not include [nested
    /// documents]. Use a [`nested` query] to search those documents.
    ///
    /// For mappings with a large number of fields, searching across all
    /// eligible fields could be expensive.
    ///
    /// There is a limit on the number of fields that can be queried at once. It
    /// is defined by the `indices.query.bool.max_clause_count` [search
    /// setting], which defaults to 1024.
    ///
    /// [nested documents]: https://www.elastic.co/guide/en/elasticsearch/reference/current/nested.html
    /// [`nested` query]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-nested-query.html
    /// [search setting]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-settings.html
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_field: Option<String>,

    /// If `true`, the wildcard characters `*` and `?` are allowed as the first
    /// character of the query string. Defaults to `true`.
    #[cfg_attr(feature = "builder", builder(default_code = "true"))]
    #[serde(default = "default_true")]
    #[field(default = true)]
    pub allow_leading_wildcard: bool,

    /// If `true`, the query attempts to analyze wildcard terms in the query
    /// string. Defaults to `false`.
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default)]
    #[field(default)]
    pub analyze_wildcard: bool,

    /// [Analyzer] used to convert text in the query string into tokens.
    /// Defaults to the [index-time analyzer] mapped for the default_field. If
    /// no analyzer is mapped, the index’s default analyzer is used.
    ///
    /// [Analyzer]: https://www.elastic.co/guide/en/elasticsearch/reference/current/analysis.html
    /// [index-time analyzer]: https://www.elastic.co/guide/en/elasticsearch/reference/current/specify-analyzer.html#specify-index-time-analyzer
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<String>,

    /// [Analyzer] used to convert quoted text in the query string into tokens.
    /// Defaults to the [`search_quote_analyzer`] mapped for the
    /// `default_field`.
    ///
    /// [Analyzer]: https://www.elastic.co/guide/en/elasticsearch/reference/current/analysis.html
    /// [`search_quote_analyzer`]: https://www.elastic.co/guide/en/elasticsearch/reference/current/analyzer.html#search-quote-analyzer
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote_analyzer: Option<String>,

    /// If `true`, [match phrase] queries are automatically created for
    /// multi-term synonyms. Defaults to `true`. See [Synonyms and the
    /// `query_string` query] for an example.
    ///
    /// [match phrase]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-match-query-phrase.html
    /// [Synonyms and the `query_string` query]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-query-string-query.html#query-string-synonyms
    #[cfg_attr(feature = "builder", builder(default_code = "true"))]
    #[serde(default = "default_true")]
    #[field(default = true)]
    pub auto_generate_synonyms_phrase_query: bool,

    /// Floating point number used to decrease or increase the [relevance
    /// scores] of the query. Defaults to `1.0`.
    ///
    /// Boost values are relative to the default value of `1.0`. A boost value
    /// between `0` and `1.0` decreases the relevance score. A value greater
    /// than `1.0` increases the relevance score.
    ///
    /// [relevance scores]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html#relevance-scores
    #[cfg_attr(feature = "builder", builder(default_code = "1.0"))]
    #[serde(default = "default_one_f32")]
    // TODO: report bug upstream, cannot be `#[field(default = 1.0f32)]
    #[field(default_with = "default_one_f32()")]
    pub boost: f32,

    /// Default boolean logic used to interpret text in the query string if no
    /// operators are specified. Defaults to `OR`.
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default)]
    #[field(default)]
    pub default_operator: QueryStringBooleanOperator,

    /// If `true`, enable position increments in queries constructed from a
    /// `query_string` search. Defaults to `true`.
    #[cfg_attr(feature = "builder", builder(default_code = "true"))]
    #[serde(default = "default_true")]
    #[field(default = true)]
    pub enable_position_increments: bool,

    /// Maximum edit distance allowed for matching. See [Fuzziness] for valid
    /// values and more information.
    ///
    /// [Fuzziness]: https://www.elastic.co/guide/en/elasticsearch/reference/current/common-options.html#fuzziness
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fuzziness: Option<String>,

    /// Maximum number of terms to which the query expands for fuzzy matching.
    /// Defaults to `50`.
    #[cfg_attr(feature = "builder", builder(default_code = "50"))]
    #[serde(default = "default_fifty_i32")]
    #[field(default = 50)]
    pub fuzzy_max_expansions: i32,

    /// Number of beginning characters left unchanged for fuzzy matching.
    /// Defaults to `0`.
    #[cfg_attr(feature = "builder", builder(default_code = "0"))]
    #[serde(default = "default_zero_i32")]
    #[field(default = 0)]
    pub fuzzy_prefix_length: i32,

    /// If `true`, edits for fuzzy matching include transpositions of two
    /// adjacent characters (`ab` → `ba`). Defaults to `true`.
    #[cfg_attr(feature = "builder", builder(default_code = "true"))]
    #[serde(default = "default_true")]
    #[field(default = true)]
    pub fuzzy_transpositions: bool,

    /// If `true`, format-based errors, such as providing a text value for a
    /// [numeric] field, are ignored. Defaults to `false`.
    ///
    /// [numeric]: https://www.elastic.co/guide/en/elasticsearch/reference/current/number.html
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default)]
    #[field(default)]
    pub lenient: bool,

    /// Maximum number of [automaton states] required for the query. Default is
    /// `10000`.
    ///
    /// Elasticsearch uses [Apache Lucene] internally to parse regular
    /// expressions. Lucene converts each regular expression to a finite
    /// automaton containing a number of determinized states.
    ///
    /// You can use this parameter to prevent that conversion from
    /// unintentionally consuming too many resources. You may need to increase
    /// this limit to run complex regular expressions.
    ///
    /// **Note**: If the requested value is above the maximum allowed value, it
    /// will be rejected by the server.
    ///
    /// [automaton states]:
    /// https://en.wikipedia.org/wiki/Deterministic_finite_automaton [Apache
    /// Lucene]: https://lucene.apache.org/core/
    #[cfg_attr(feature = "builder", builder(default_code = "10_000"))]
    #[serde(default = "default_ten_thousand_i32")]
    #[field(default = 10_000)]
    pub max_determinized_states: i32,

    /// Minimum number of clauses that must match for a document to be returned.
    /// See the [`minimum_should_match` parameter] for valid values and more
    /// information. See [How `minimum_should_match` works] for an example.
    ///
    /// [`minimum_should_match` parameter]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-minimum-should-match.html
    /// [How `minimum_should_match` works]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-query-string-query.html#query-string-min-should-match
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_should_match: Option<String>,

    /// Maximum number of positions allowed between matching tokens for phrases. Defaults to `0`. If `0`, exact phrase matches are required. Transposed terms have a slop of `2`.
    #[cfg_attr(feature = "builder", builder(default_code = "0"))]
    #[serde(default = "default_zero_i32")]
    #[field(default = 0)]
    pub phrase_slop: i32,

    /// Suffix appended to quoted text in the query string.
    ///
    /// You can use this suffix to use a different analysis method for exact
    /// matches. See [Mixing exact search with stemming](https://www.elastic.co/guide/en/elasticsearch/reference/current/mixing-exact-search-with-stemming.html).
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote_field_suffix: Option<String>,

    /// Method used to rewrite the query. For valid values and more information,
    /// see the [`rewrite` parameter].
    ///
    /// [`rewrite` parameter]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-multi-term-rewrite.html
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rewrite: Option<String>,

    /// [Coordinated Universal Time (UTC) offset] or [IANA] time zone used to
    /// convert `date` values in the query string to UTC.
    ///
    /// Valid values are [ISO 8601] UTC offsets, such as `+01:00` or -`08:00`,
    /// and IANA time zone IDs, such as `America/Los_Angeles`.
    ///
    /// > **Note**
    /// >
    /// > The `time_zone` parameter does **not** affect the [date math] value of
    /// > `now`. `now` is always the current system time in UTC. However, the
    /// > `time_zone` parameter does convert dates calculated using `now` and
    /// > [date math rounding]. For example, the `time_zone` parameter will
    /// > convert a value of `now/d`.
    ///
    /// [Coordinated Universal Time (UTC) offset]: https://en.wikipedia.org/wiki/List_of_UTC_time_offsets
    /// [IANA]: https://en.wikipedia.org/wiki/List_of_tz_database_time_zones
    /// [ISO 8601]: https://en.wikipedia.org/wiki/ISO_8601
    /// [date math]: https://www.elastic.co/guide/en/elasticsearch/reference/current/common-options.html#date-math
    /// [date math rounding]: https://www.elastic.co/guide/en/elasticsearch/reference/current/common-options.html#date-math
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

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
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
#[cfg_attr(feature = "graphql", graphql(name = "QueryStringFilter"))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct QueryStringQuery {
    /// The query to run in the [simple query string syntax](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-simple-query-string-query.html#simple-query-string-syntax).
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub query: String,

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

    /// Default field you wish to search if no field is provided in the query
    /// string.
    ///
    /// Defaults to the `index.query.default_field` index setting, which has a
    /// default value of `*`. The `*` value extracts all fields that are
    /// eligible for term queries and filters the metadata fields. All extracted
    /// fields are then combined to build a query if no `prefix` is specified.
    ///
    /// Searching across all eligible fields does not include [nested
    /// documents]. Use a [`nested` query] to search those documents.
    ///
    /// For mappings with a large number of fields, searching across all
    /// eligible fields could be expensive.
    ///
    /// There is a limit on the number of fields that can be queried at once. It
    /// is defined by the `indices.query.bool.max_clause_count` [search
    /// setting], which defaults to 1024.
    ///
    /// [nested documents]: https://www.elastic.co/guide/en/elasticsearch/reference/current/nested.html
    /// [`nested` query]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-nested-query.html
    /// [search setting]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-settings.html
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_field: Option<String>,

    /// If `true`, the wildcard characters `*` and `?` are allowed as the first
    /// character of the query string. Defaults to `true`.
    #[cfg_attr(feature = "builder", builder(default_code = "true"))]
    #[serde(default = "default_true")]
    pub allow_leading_wildcard: bool,

    /// If `true`, the query attempts to analyze wildcard terms in the query
    /// string. Defaults to `false`.
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default)]
    pub analyze_wildcard: bool,

    /// [Analyzer] used to convert text in the query string into tokens.
    /// Defaults to the [index-time analyzer] mapped for the default_field. If
    /// no analyzer is mapped, the index’s default analyzer is used.
    ///
    /// [Analyzer]: https://www.elastic.co/guide/en/elasticsearch/reference/current/analysis.html
    /// [index-time analyzer]: https://www.elastic.co/guide/en/elasticsearch/reference/current/specify-analyzer.html#specify-index-time-analyzer
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<String>,

    /// [Analyzer] used to convert quoted text in the query string into tokens.
    /// Defaults to the [`search_quote_analyzer`] mapped for the
    /// `default_field`.
    ///
    /// [Analyzer]: https://www.elastic.co/guide/en/elasticsearch/reference/current/analysis.html
    /// [`search_quote_analyzer`]: https://www.elastic.co/guide/en/elasticsearch/reference/current/analyzer.html#search-quote-analyzer
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote_analyzer: Option<String>,

    /// If `true`, [match phrase] queries are automatically created for
    /// multi-term synonyms. Defaults to `true`. See [Synonyms and the
    /// `query_string` query] for an example.
    ///
    /// [match phrase]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-match-query-phrase.html
    /// [Synonyms and the `query_string` query]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-query-string-query.html#query-string-synonyms
    #[cfg_attr(feature = "builder", builder(default_code = "true"))]
    #[serde(default = "default_true")]
    pub auto_generate_synonyms_phrase_query: bool,

    /// Floating point number used to decrease or increase the [relevance
    /// scores] of the query. Defaults to `1.0`.
    ///
    /// Boost values are relative to the default value of `1.0`. A boost value
    /// between `0` and `1.0` decreases the relevance score. A value greater
    /// than `1.0` increases the relevance score.
    ///
    /// [relevance scores]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html#relevance-scores
    #[cfg_attr(feature = "builder", builder(default_code = "1.0"))]
    #[serde(default = "default_one_f32")]
    pub boost: f32,

    /// Default boolean logic used to interpret text in the query string if no
    /// operators are specified. Defaults to `OR`.
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default)]
    pub default_operator: QueryStringBooleanOperator,

    /// If `true`, enable position increments in queries constructed from a
    /// `query_string` search. Defaults to `true`.
    #[cfg_attr(feature = "builder", builder(default_code = "true"))]
    #[serde(default = "default_true")]
    pub enable_position_increments: bool,

    /// Maximum edit distance allowed for matching. See [Fuzziness] for valid
    /// values and more information.
    ///
    /// [Fuzziness]: https://www.elastic.co/guide/en/elasticsearch/reference/current/common-options.html#fuzziness
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fuzziness: Option<String>,

    /// Maximum number of terms to which the query expands for fuzzy matching.
    /// Defaults to `50`.
    #[cfg_attr(feature = "builder", builder(default_code = "50"))]
    #[serde(default = "default_fifty_i32")]
    pub fuzzy_max_expansions: i32,

    /// Number of beginning characters left unchanged for fuzzy matching.
    /// Defaults to `0`.
    #[cfg_attr(feature = "builder", builder(default_code = "0"))]
    #[serde(default = "default_zero_i32")]
    pub fuzzy_prefix_length: i32,

    /// If `true`, edits for fuzzy matching include transpositions of two
    /// adjacent characters (`ab` → `ba`). Defaults to `true`.
    #[cfg_attr(feature = "builder", builder(default_code = "true"))]
    #[serde(default = "default_true")]
    pub fuzzy_transpositions: bool,

    /// If `true`, format-based errors, such as providing a text value for a
    /// [numeric] field, are ignored. Defaults to `false`.
    ///
    /// [numeric]: https://www.elastic.co/guide/en/elasticsearch/reference/current/number.html
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default)]
    pub lenient: bool,

    /// Maximum number of [automaton states] required for the query. Default is
    /// `10000`.
    ///
    /// Elasticsearch uses [Apache Lucene] internally to parse regular
    /// expressions. Lucene converts each regular expression to a finite
    /// automaton containing a number of determinized states.
    ///
    /// You can use this parameter to prevent that conversion from
    /// unintentionally consuming too many resources. You may need to increase
    /// this limit to run complex regular expressions.
    ///
    /// **Note**: If the requested value is above the maximum allowed value, it
    /// will be rejected by the server.
    ///
    /// [automaton states]:
    /// https://en.wikipedia.org/wiki/Deterministic_finite_automaton [Apache
    /// Lucene]: https://lucene.apache.org/core/
    #[cfg_attr(feature = "builder", builder(default_code = "10_000"))]
    #[serde(default = "default_ten_thousand_i32")]
    pub max_determinized_states: i32,

    /// Minimum number of clauses that must match for a document to be returned.
    /// See the [`minimum_should_match` parameter] for valid values and more
    /// information. See [How `minimum_should_match` works] for an example.
    ///
    /// [`minimum_should_match` parameter]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-minimum-should-match.html
    /// [How `minimum_should_match` works]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-query-string-query.html#query-string-min-should-match
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_should_match: Option<String>,

    /// Maximum number of positions allowed between matching tokens for phrases. Defaults to `0`. If `0`, exact phrase matches are required. Transposed terms have a slop of `2`.
    #[cfg_attr(feature = "builder", builder(default_code = "0"))]
    #[serde(default = "default_zero_i32")]
    pub phrase_slop: i32,

    /// Suffix appended to quoted text in the query string.
    ///
    /// You can use this suffix to use a different analysis method for exact
    /// matches. See [Mixing exact search with stemming](https://www.elastic.co/guide/en/elasticsearch/reference/current/mixing-exact-search-with-stemming.html).
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote_field_suffix: Option<String>,

    /// Method used to rewrite the query. For valid values and more information,
    /// see the [`rewrite` parameter].
    ///
    /// [`rewrite` parameter]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-multi-term-rewrite.html
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rewrite: Option<String>,

    /// [Coordinated Universal Time (UTC) offset] or [IANA] time zone used to
    /// convert `date` values in the query string to UTC.
    ///
    /// Valid values are [ISO 8601] UTC offsets, such as `+01:00` or -`08:00`,
    /// and IANA time zone IDs, such as `America/Los_Angeles`.
    ///
    /// > **Note**
    /// >
    /// > The `time_zone` parameter does **not** affect the [date math] value of
    /// > `now`. `now` is always the current system time in UTC. However, the
    /// > `time_zone` parameter does convert dates calculated using `now` and
    /// > [date math rounding]. For example, the `time_zone` parameter will
    /// > convert a value of `now/d`.
    ///
    /// [Coordinated Universal Time (UTC) offset]: https://en.wikipedia.org/wiki/List_of_UTC_time_offsets
    /// [IANA]: https://en.wikipedia.org/wiki/List_of_tz_database_time_zones
    /// [ISO 8601]: https://en.wikipedia.org/wiki/ISO_8601
    /// [date math]: https://www.elastic.co/guide/en/elasticsearch/reference/current/common-options.html#date-math
    /// [date math rounding]: https://www.elastic.co/guide/en/elasticsearch/reference/current/common-options.html#date-math
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

/// Boolean logic operator used to interpret/combine words in the query string.
#[cfg_attr(all(test, not(feature = "graphql")), derive(PartialEq))]
#[cfg_attr(feature = "graphql", derive(async_graphql::Enum, Eq, PartialEq, Copy))]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QueryStringBooleanOperator {
    /// For example, a query string of `capital of Hungary` is interpreted as
    /// `capital OR of OR Hungary`.
    Or,

    /// For example, a query string of `capital of Hungary` is interpreted as
    /// `capital AND of AND Hungary`.
    And,
}

impl Default for QueryStringBooleanOperator {
    fn default() -> Self {
        QueryStringBooleanOperator::Or
    }
}

#[cfg(feature = "graphql")]
impl From<QueryStringQueryInput> for QueryStringQuery {
    #[inline]
    fn from(input: QueryStringQueryInput) -> QueryStringQuery {
        QueryStringQuery {
            query: input.query,
            fields: input.fields,
            default_field: input.default_field,
            allow_leading_wildcard: input.allow_leading_wildcard,
            analyze_wildcard: input.analyze_wildcard,
            analyzer: input.analyzer,
            quote_analyzer: input.quote_analyzer,
            auto_generate_synonyms_phrase_query: input.auto_generate_synonyms_phrase_query,
            boost: input.boost,
            default_operator: input.default_operator,
            enable_position_increments: input.enable_position_increments,
            fuzziness: input.fuzziness,
            fuzzy_max_expansions: input.fuzzy_max_expansions,
            fuzzy_prefix_length: input.fuzzy_prefix_length,
            fuzzy_transpositions: input.fuzzy_transpositions,
            lenient: input.lenient,
            max_determinized_states: input.max_determinized_states,
            minimum_should_match: input.minimum_should_match,
            phrase_slop: input.phrase_slop,
            quote_field_suffix: input.quote_field_suffix,
            rewrite: input.rewrite,
            time_zone: input.time_zone,
        }
    }
}

fn default_true() -> bool {
    true
}

fn default_zero_i32() -> i32 {
    0
}

fn default_fifty_i32() -> i32 {
    50
}

fn default_one_f32() -> f32 {
    1.0
}

fn default_ten_thousand_i32() -> i32 {
    10_000
}
