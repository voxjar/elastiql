//! Wrapper/[newtype structs] scalars used throughout.
//!
//! [newtype structs]: https://doc.rust-lang.org/1.0.0/style/features/types/newtype.html

pub use self::{json::*, number_or_string::*};

mod json;
mod number_or_string;
