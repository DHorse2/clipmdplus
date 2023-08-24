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
/// Types of forms and UI types.
pub mod type_form; // romove?
pub use self::type_form::*;
/// The knows format types of entities.
pub mod type_format; // romove?
pub use self::type_format::*;

/// A range object. (todo student)
pub mod range; // romove?
pub use self::range::*;

/// A sequence object (todo student)
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
// error: `std::any::type_name` is not yet stable as a const fn
//   --> clipmdplus\src\stdmd\types\mod.rs:48:5
//    |
// 48 |     type_name::<T>()
//    |     ^^^^^^^^^^^^^^^^
// 48 const fn type_name_of_val<T: ?Sized>(_: &T) -> &'static str {
/// The type name of T as a static str.
pub fn type_name_of_val<T: ?Sized>(_: &T) -> &'static str {
    type_name::<T>()
}
// Any
/// The type name done another way. (todo student)
pub fn type_name_of<T>(_: &T) -> &'static str 
where
    T: Any,
    T: ?Sized
{
    // let ty_name = format!("{}", std::any::type_name::<T>()).as_str();
    let ty_name = std::any::type_name::<T>().clone();
    // let result = ty_name.clone();
    ty_name
}
/// Given a TypeId it returns its name as a static str.
pub fn type_name_from_id<T: ?Sized + Any, I>(i: &TypeId) -> &'static str 
where
    I: ?Sized,
{
    // r#"Does not reflect the RUST type system use of a unique id for each type.  
    // It's not available as a readable string"#;
    // format!("{}", TypeId::of::<I>()).as_str()
    // TypeId::of::<I>().to_string()
    // TypeId::type_name::<T>
    // i.type_name::<I>()
    // &TypeId::of::<i<T>>().type_name()
    let b: &'static str = i.type_id().type_name().clone(); // .to_string().as_str();
    b
}
/// Returns the TypeId of T.
pub fn type_of_val<T: ?Sized + 'static>(_: &T) -> TypeId {
    TypeId::of::<T>().clone()
    // t.type_id()
}
//
/// Compare the types of two Sized objects. 
pub fn type_is_equal<T: ?Sized  + 'static, U: ?Sized  + 'static>(_: &T, _: &U) -> bool {
    // if let TypeId::of::<T>() = TypeId::of::<U>() {
    //     return true
    // }
    // false
    // let result = TypeId::of::<T>() == TypeId::of::<U>();
    // result
    // if TypeId::of::<T>() == TypeId::of::<U>() {
    //     return true
    // }
    // false
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
