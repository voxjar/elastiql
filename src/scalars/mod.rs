//! Wrapper/[newtype structs] [scalars] used in many different database documents.
//!
//! [newtype structs]: https://doc.rust-lang.org/1.0.0/style/features/types/newtype.html
//! [scalars]: https://graphql.org/learn/schema/#scalar-types

pub use self::json::*;

mod json;
