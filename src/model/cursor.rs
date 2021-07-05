use crate::request::Query;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A structure included in some API responses containing the parameters to get the next set of results.
///
/// The values of the cursor should be provided to next request of the same endpoint to get the next set of results.
///
/// If there are no more results available, a cursor with a value of `None` is returned.
///
/// Note that sort option should also be specified for it to work.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct Cursor {
    cursor: Value,
}

impl Cursor {
    /// Create a cursor
    #[inline]
    pub fn new(cursor: Value) -> Self {
        Self { cursor }
    }

    pub(crate) fn push_to_query(&self, query: &mut Query) {
        if let Value::Object(ref map) = self.cursor {
            for (key, value) in map {
                let key = format!("cursor[{}]", key);

                match value {
                    Value::Bool(v) => query.push(key.as_str(), v),
                    Value::Number(v) => query.push(key.as_str(), v),
                    Value::String(v) => query.push(key.as_str(), v),
                    Value::Null | Value::Array(_) | Value::Object(_) => {
                        unreachable!("cursor fields expected to be a string, number, or boolean")
                    }
                }
            }
        } else {
            unreachable!("cursor is expected to be a map");
        }
    }
}
