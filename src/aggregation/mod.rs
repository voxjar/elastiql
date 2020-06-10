//! Request, response and other types used when [aggregating] documents.
//!
//! [aggregating]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations.html

pub use self::{request::*, response::*};

mod request;
mod response;
mod serialization_deserialization;
pub mod types;
