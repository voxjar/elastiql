# elastiql

[![Latest Version]][crates.io] [![Docs]][docs.rs] [![CI]][github-actions]

[CI]: https://github.com/voxjar/elastiql/workflows/CI/badge.svg
[github-actions]: https://github.com/voxjar/elastiql/actions
[latest version]: https://img.shields.io/crates/v/elastiql.svg
[crates.io]: https://crates.io/crates/elastiql
[docs]: https://docs.rs/elastiql/badge.svg
[docs.rs]: https://docs.rs/elastiql

> An opinionated [Elasticsearch] query language for [Rust].

This library pairs nicely with the official Elasticsearch [crate] and the
[async-graphql] crate. It is meant to be a more batteries included approach to
working with [Elasticsearch] in [Rust] than the official [crate] (see:
[elastic/elasticsearch-rs#75]).

### project status

This project has not yet reached `v1.0.0`. As such, you can expect some amount
of breaking changes. It has also not (yet) been published to _crates.io_, but
most likely will in the future.

We have defined types (optionally with [`builder`] methods) for most
Elasticsearch [aggregations], [query DSL] and other miscellaneous
request/response types.

### usage

This crate allows for more idiomatic request/response handling, e.g.:

```rust
let query = Request::builder()
    .query(TermQuery::new("id", "test_user_id"))
    .size(1)
    .version(true)
    .highlight(HighlightOptions::default())
    .build();
```

For more examples, see the [examples](examples) directory.

### differences between the Elasticsearch REST API

Because this project is expected to mainly be used via [GraphQL] some
concessions had to be made to balance idiomatic [Rust], [Elasticsearch] and
[GraphQL]. For example, the structure of aggregation responses is not true to
[Elasticsearch] and is vastly simplified.

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in elastiql by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>

[`builder`]: https://crates.io/crates/typed-builder
[aggregations]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations.html
[async-graphql]: https://crates.io/crates/async-graphql
[crate]: https://crates.io/crates/elasticsearch
[elasticsearch]: https://www.elastic.co/guide/en/elasticsearch/reference/current/index.html
[graphql]: https://graphql.org/
[query dsl]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl.html
[rust]: https://www.rust-lang.org/
[elastic/elasticsearch-rs#75]: https://github.com/elastic/elasticsearch-rs/issues/75
