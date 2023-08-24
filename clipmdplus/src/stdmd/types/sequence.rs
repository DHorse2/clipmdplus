
// stdmd sequence.rs
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
use std::fmt;

use crate::stdmd::types::range::Range;
// temporary
/// A trivial sequence object .
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sequence { 
    /// The current value.
    current: i32,
    /// The sequence range that is selected.
    selected: Option<Range>,
    /// Possible paging handling.
    page: Option<i32>,
 }
 impl Default for Sequence {
    fn default() -> Self {
        Sequence {
            current: i32::default(), // load this
            selected: None,
            page: None,
        }
    }
}
impl fmt::Display for Sequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "Clipboard data {} {} {:?}", self.id_key, self.data1, self)
        write!(f, "Sequence: current {}, selected {:#?}", self.current, self.selected.as_ref())
        // write!(f, "Sequence: current {}, selected ({},{}), page {}", self.current, self.selected.unwrap().start, self.selected.unwrap().end, self.page.unwrap())
    }
}
#[allow(dead_code, unused_imports)] // object creation (pre debug)
impl Sequence { 
    /// <summary>
    /// Open the clipboard database file.
    /// </summary> 
    pub fn new() -> Self
    {
        Self::default()
    }
    /// <summary>
    /// Open the clipboard database file.
    /// </summary> 
    pub fn sequence_get(&self) -> i32
    {
        self.current
    }
    /// <summary>
    /// Open the clipboard database file.
    /// </summary> 
    pub fn sequence_set(&mut self, sequence_number_passed: i32) -> bool
    {
        self.current = sequence_number_passed;
        true
    }
}
// !------------------------------------------------------------