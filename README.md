# elastiql

> Opinionated [Elasticsearch] query language for [Rust].

This library pairs nicely with the official Elasticsearch [crate] and the
[async-graphql] crate. It is meant to be a more batteries included approach to
working with [Elasticsearch] in [Rust], than the official [crate] (see:
https://github.com/elastic/elasticsearch-rs/issues/75).

Because this project is expected to mainly be used via [GraphQL] some
concessions had to be made to balance idiomatic [Rust], [Elasticsearch] and
[GraphQL]. For example, the structure of aggregation responses is not true to
[Elasticsearch] and is vastly simplified.

## project status

This project has not yet reached `v1.0.0`. As such, you can expect breaking
changes. That being said, we expect most of the breaking changes to be related
to naming and the import/module structure...

We have defined types (optionally with [`builder`] methods) for most
Elasticsearch [aggregations], [query DSL] and other miscellaneous
request/response types.

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
