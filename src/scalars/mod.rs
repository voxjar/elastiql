//! Primitive data types used by Elasticsearch.

use std::collections::HashMap;

use serde_json::Value as JsonValue;

pub use self::sorted_value::*;

mod sorted_value;

// TODO: remove `Map` type alias; better way to conditionally compile?

/// A JSON Object
#[cfg(feature = "graphql")]
pub type Map = HashMap<String, async_graphql::Json<JsonValue>>;

/// A JSON Object
#[cfg(not(feature = "graphql"))]
pub type Map = HashMap<String, JsonValue>;
