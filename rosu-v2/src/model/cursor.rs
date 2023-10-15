use serde::{ser::SerializeMap, Deserialize, Serializer};
use serde_json::Value;

/// A structure included in some API responses containing the parameters to get the next set of results.
///
/// The values of the cursor should be provided to next request of the same endpoint to get the next set of results.
/// If there are no more results available, a cursor with a value of `None` is returned.
///
/// Note that a sort option should also be specified for it to work.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
// TODO
// #[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
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

    pub(crate) fn serialize_as_query<S: Serializer>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let Value::Object(ref json_map) = self.cursor else {
            unreachable!("cursor is expected to be a map");
        };

        let mut map = serializer.serialize_map(Some(json_map.len()))?;

        for (key, value) in json_map {
            let key = format!("cursor[{key}]");
            map.serialize_entry(&key, value)?;
        }

        map.end()
    }
}
