//! Request & response types used when making [bulk] queries.
//!
//! [bulk]: https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-bulk.html

use crate::search::ErrResponse;
use serde::Deserialize;

// TODO: add these upstream https://github.com/elastic/elasticsearch-rs/issues/75
// TODO: add missing fields...

/// The bulk API’s response contains the individual results of each operation in
/// the request, returned in the order submitted. The success or failure of an
/// individual operation does not affect other operations in the request.
#[derive(Deserialize, Debug)]
pub struct Response<T> {
    /// How long, in milliseconds, it took to process the bulk request.
    pub took: u64,

    /// If `true`, one or more of the operations in the bulk request did not
    /// complete successfully.
    pub errors: bool,

    /// The bulk response items.
    #[serde(default = "Vec::new")]
    pub items: Vec<Action<T>>,
}

/// The result of a bulk operation.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Action<T> {
    /// Result from performing a bulk `Create` operation.
    Create(Item<T>),

    /// Result from performing a bulk `Index` operation.
    Index(Item<T>),

    /// Result from performing a bulk `Update` operation.
    Update(Item<T>),

    /// Result from performing a bulk `Delete` operation.
    Delete(Item<T>),
}

/// An individual bulk item.
///
/// *FIXME*: somehow use `std::error::Error` instead of this...
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Item<T> {
    /// An `Ok` item.
    Ok(OkItem<T>),

    /// An `Err` item.
    Err {
        /// The error.
        error: ErrResponse,

        /// The [HTTP status code](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status).
        status: u16,
    },
}

/// An individual item from performing a successful bulk operation.
#[derive(Deserialize, Debug)]
pub struct OkItem<T> {
    status: u16,

    /// The document ID associated with the operation.
    #[serde(rename = "_id")]
    pub id: String,

    /// The index the document belongs to.
    #[serde(rename = "_index")]
    pub index: String,

    /// The document's source (if requested).
    pub get: Option<Get<T>>,
}

/// Encompasses the `source` document.
#[derive(Deserialize, Debug)]
pub struct Get<T> {
    /// The document's source (if requested).
    #[serde(rename = "_source")]
    pub source: Option<T>,
}
