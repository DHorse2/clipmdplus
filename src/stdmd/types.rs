// stdmd type.rs

use std::any::Any;
use std::any::type_name;
use std::any::TypeId;

// use crate::stdmd::types::UiType;
// use crate::stdmd::types::UiError;
// use crate::stdmd::types::UiResult;
// use crate::stdmd::types::WindowType;
// use crate::stdmd::types::WindowError;
// use crate::stdmd::types::WindowResult;

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

// !------------------------------------------------------------
include!(".\\type_form.rs");
// !------------------------------------------------------------
