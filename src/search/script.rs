//! Types for searching/aggregating using [scripts]
//!
//! [scripts]: https://www.elastic.co/guide/en/elasticsearch/reference/current/modules-scripting.html

use std::default::Default;

use serde::{Deserialize, Serialize};

/// Available sandboxed scripting [languages].
///
/// [languages]: https://www.elastic.co/guide/en/elasticsearch/reference/current/modules-scripting.html
#[cfg_attr(feature = "graphql", async_graphql::Enum)]
#[cfg_attr(not(feature = "graphql"), derive(Clone))]
#[derive(Serialize, Deserialize, Debug)]
pub enum ScriptLanguage {
    /// [Lucene expressions language] compile a Javascript expression to
    /// bytecode. They are designed for high-performance custom ranking and
    /// sorting functions.
    ///
    /// ## Example
    ///
    /// ```notrust
    /// (doc['silence_duration'] / doc['duration']) * 100.0
    /// ```
    ///
    /// [Lucene expressions language]: https://www.elastic.co/guide/en/elasticsearch/reference/current/modules-scripting-expression.html
    Expressions,

    /// General purpose [scripting] language similar to Java.
    ///
    /// Refer to the [Painless API reference] for more information.
    ///
    /// ## Examples
    ///
    /// ```notrust
    /// (doc.getOrDefault​('silence_duration', 0.0) / doc['duration']) * 100.0
    /// ```
    ///
    /// ```notrust
    /// int total = 0;
    /// for (int i = 0; i < doc['goals'].length; ++i) {
    ///     total += doc['goals'][i];
    /// }
    /// return total;
    /// ```
    ///
    /// [scripting]: https://www.elastic.co/guide/en/elasticsearch/painless/current/painless-walkthrough.html
    /// [Painless API reference]: https://www.elastic.co/guide/en/elasticsearch/painless/current/painless-api-reference.html
    Painless,
}

impl Default for ScriptLanguage {
    #[inline]
    fn default() -> Self {
        ScriptLanguage::Expressions
    }
}

/// Evaluates custom expressions/[scripts].
///
/// [scripts]: https://www.elastic.co/guide/en/elasticsearch/reference/current/modules-scripting.html
#[cfg(feature = "graphql")]
#[async_graphql::InputObject]
#[derive(Serialize, Clone, Debug)]
pub struct ScriptInput {
    source: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    params: Option<crate::scalars::Map>,
    // #[field(default]
    // #[serde(default, skip_serializing_if = "Option::is_none")]
    // lang: Option<ScriptLanguage>,
}

/// Evaluates custom expressions/[scripts].
///
/// [scripts]: https://www.elastic.co/guide/en/elasticsearch/reference/current/modules-scripting.html
#[cfg_attr(feature = "graphql", async_graphql::SimpleObject)]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Script {
    source: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    params: Option<crate::scalars::Map>,
}

#[cfg(feature = "graphql")]
impl From<ScriptInput> for Script {
    #[inline]
    fn from(script: ScriptInput) -> Self {
        Script {
            source: script.source,
            params: script.params,
        }
    }
}
