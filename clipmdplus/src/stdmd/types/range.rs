// stdmd types range.rs
//
// use serde::de::Error;
// use serde::de::{self, Unexpected, Visitor};
// use serde::de::{MapAccess, SeqAccess};
// use serde::ser::SerializeStruct;
use serde::{
    // forward_to_deserialize_any, forward_to_deserialize_any_helper, 
    Deserialize, Serialize,
};
//
// use serde_json::{Deserializer, Serializer, Value};
use std::fmt;
/// A trivial range object.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Range {
    /// The start.
    pub start: i32,
    /// The end.
    pub end: i32,
}
impl Default for Range {
    fn default() -> Self {
        Range {
            start: i32::default(), // load this
            end: i32::default(), // load this
        }
    }
}
impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "Clipboard data {} {} {:?}", self.id_key, self.data1, self)
        write!(f, "Range: start {}, end {}", self.start, self.end)
    }
}
