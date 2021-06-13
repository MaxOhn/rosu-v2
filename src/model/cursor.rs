use crate::request::Query;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct Cursor {
    cursor: Value,
}

impl Cursor {
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
