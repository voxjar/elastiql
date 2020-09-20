#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::all)]

//! [Elasticsearch] query language.
//!
//! [Elasticsearch]: https://www.elastic.co/guide/en/elasticsearch/reference/current/index.html

pub mod aggregation;
pub mod bulk;
pub mod scalars;
pub mod search;
