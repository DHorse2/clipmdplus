// stdmd type mod.rs
// #[macro_use]

use std::any::Any;
use std::any::type_name;
use std::any::TypeId;

// use crate::stdmd::types::types_format::Format;
// use crate::stdmd::types::types_format::FormatType;
// use crate::stdmd::types::types_format::FormatList;
// use crate::stdmd::types::types_form::UiType;
// use crate::stdmd::types::types_form::UiError;
// use crate::stdmd::types::types_form::UiResult;
// use crate::stdmd::types::types_form::WindowType;
// use crate::stdmd::types::types_form::WindowError;
// use crate::stdmd::types::types_form::WindowResult;
// use crate::stdmd::types::types_form::Range
// use crate::stdmd::types::types_form::Sequence
// use crate::stdmd::types::types_form::
// !------------------------------------------------------------
pub mod type_form; // romove?
pub use self::type_form::*;
pub mod type_format; // romove?
pub use self::type_format::*;
pub mod range; // romove?
pub use self::range::*;
pub mod sequence; // romove?
pub use self::sequence::*;
// #[macro_use]
// pub mod serde_enum;
// pub use self::serde_enum::*;
// !------------------------------------------------------------
// !------------------------------------------------------------
// (function pointers)
// fn(_) -> _ 

// (traits)
// Fn(t) -> U

// FnMut(t) -> U

// FnOnce(t) -> U

// Metadata
// shim unstable nightly feature
// ?shadowing this?
const fn type_name_of_val<T: ?Sized>(_: &T) -> &'static str {
    type_name::<T>()
}
// Any
const fn type_name_of<T>(_: &T) -> &'static str 
where
    T: Any,
    T: ?Sized
{
    format!("{}", std::any::type_name::<T>()).as_str()
}

const fn type_name_from_id<T: ?Sized + Any, I>(i: &TypeId) -> &'static str 
where
    I: ?Sized,
{
    r#"Does not reflect the RUST type system use of a unique id for each type.  
    It's not available as a readable string"#
    // format!("{}", TypeId::of::<I>()).as_str()
    // TypeId::of::<I>().to_string()
    // TypeId::type_name::<T>
    // i.type_name::<I>()
    // &TypeId::of::<i<T>>()
}

const fn type_of_val<T: ?Sized>(_: &T) -> &'static TypeId {
    &TypeId::of::<T>()
    // t.type_id()
}
//
// Compare
const fn type_is_equal<T: ?Sized, U: ?Sized>(_: &T, _: &U) -> bool {
    TypeId::of::<T>() == TypeId::of::<U>()
}
// note on function pointers.
// local's type becomes a function pointer:
    // let mut local: fn(_) -> = |closure|;
    // local = |new closure|
// however this doesn't work with closures that capture their environment.

// NamedAny Trait
trait NamedAny: Any {
    fn type_name(&self) -> &'static str;
}

impl<T: Any> NamedAny for T {
    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}
