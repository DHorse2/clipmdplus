#![warn(missing_docs)]
#![allow(unused_macros, unused_imports)]
#![cfg_attr(
    feature = "document-features",
    // #[doc(hidden)]
    cfg_attr(doc, doc = ::document_features::document_features!())
    // #![doc = document_features::document_features!()]
)]
//! doc for ClipMdPlus main
//! Normal crate documentation goes here.
//! 
//! todo Design note. Move utility modules and macros to lib.
//! todo Features could be used to optimize compile time like they did.
//! Features are: OS (required?), History, Clipboard (no hist), Window, ProcMacros.
//! At lower detail level WindowTypes (ie egui, web), Specific macros and
//! crates could all be easily excluded in a clear (to the dev) manner.
//! It's a discussion topic for later releases.
//! 
//! The next Project Phase Task is connect to Windows (known) 
//! and then Linux (new to me) clipboards.
//!
//! As to documentation standards, again follow the derive_more crate. 
//! Sample to follow from the derive_more crate:
//!------------------------------------------------------------
//! ## Feature flags
//! 
//! # `clipmdplus`
//!
//! [![Build Status](https://github.com/DHorse2/clipmdplus/project_admin/something.svg)](https://github.com/DHorse2/clipmdplus/project_admin)
//! [![Latest Version](https://img.shields.io/crates/v/clipmdplus.svg)](https://crates.io/crates/clipmdplus)
//! [![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://dhorse2.github.io/clipmdplus/clipmdplus/)
//! [![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/JelteF/clipmdplus/master/LICENSE)
//! [![Rust 1.36+](https://img.shields.io/badge/rustc-1.36+-lightgray.svg)](https://blog.rust-lang.org/2019/07/04/Rust-1.36.0.html)
//!
//! This library tries to remove these annoyances boilerplate code.
//! Once it's working properly the following macro with be created:
//! Clipmd!("name")
//!
//! ## Example code
//!
//! By using this library the following code just works:
//!
//! ```rust
//! extern crate clipmdplus;
//! use clipmdplus::{ClipboardMeta};
//!
//! ! clipboard_meta: ClipboardMeta = ClipboardMeta::new();
//! Clipmd!("Named")
//! clipboard_meta: ClipboardMeta = Clipmd!("Named");
//! 
//!
//! assert!(MyInt(11) == MyInt(5) + 6.into());
//! assert!((5, 6) == Point2D { x: 5, y: 6 }.into());
//! assert!(MyEnum::Int(15) == (MyEnum::Int(8) + 7.into()).unwrap());
//! assert!(MyEnum::Int(15).to_string() == "int: 15");
//! assert!(MyEnum::Uint(42).to_string() == "42");
//! assert!(MyEnum::Nothing.to_string() == "nothing");
//! ```
//!
//! ## The derivable traits
//!
//! Below are all the traits that you can derive using this library.
//! Some trait derivations are so similar that the further documentation will only show a single one
//! of them.
//! You can recognize these by the "-like" suffix in their name.
//! The trait name before that will be the only one that is used throughout the further
//! documentation.
//!
//! It is important to understand what code gets generated when using one of the
//! derives from this crate.
//! That is why the links below explain what code gets generated for a trait for
//! each group from before.
//!
//! You can use the [`cargo-expand`] utility to see the exact code that is generated
//! for your specific type.
//! This will show you your code with all macros and derives expanded.
//!
//! **NOTE**: You still have to derive each trait separately. So `#[derive(Mul)]` doesn't
//! automatically derive `Div` as well. To derive both you should do `#[derive(Mul, Div)]`
//!
//! ### Conversion traits
//!
//! These are traits that are used to convert automatically between types.
//!
//! 1. [`From`]
//! 2. [`Into`]
//! 3. [`FromStr`]
//! 4. [`TryInto`]
//! 5. [`IntoIterator`]
//! 6. [`AsRef`]
//! 7. [`AsMut`]
//!
//! ### Formatting traits
//!
//! These traits are used for converting a struct to a string in different ways.
//!
//! 1. [`Display`-like], contains `Display`, `Binary`, `Octal`, `LowerHex`,
//!    `UpperHex`, `LowerExp`, `UpperExp`, `Pointer`
//!
//! ### Error-handling traits
//! These traits are used to define error-types.
//!
//! 1. [`Error`]
//!
//! ### Operators
//!
//! These are traits that can be used for operator overloading.
//!
//! 1. [`Index`]
//! 2. [`Deref`]
//! 3. [`Not`-like], contains `Not` and `Neg`
//! 4. [`Add`-like], contains `Add`, `Sub`, `BitAnd`, `BitOr`, `BitXor`
//! 5. [`Mul`-like], contains `Mul`, `Div`, `Rem`, `Shr` and `Shl`
//! 3. [`Sum`-like], contains `Sum` and `Product`
//! 6. [`IndexMut`]
//! 7. [`DerefMut`]
//! 8. [`AddAssign`-like], contains `AddAssign`, `SubAssign`, `BitAndAssign`,
//!    `BitOrAssign` and `BitXorAssign`
//! 9. [`MulAssign`-like], contains `MulAssign`, `DivAssign`, `RemAssign`,
//!    `ShrAssign` and `ShlAssign`
//!
//! ### Static methods
//!
//! These don't derive traits, but derive static methods instead.
//!
//! 1. [`Constructor`], this derives a `new` method that can be used as a constructor.
//!    This is very basic if you need more customization for your constructor, check
//!    out the [`derive-new`] crate.
//! 2. [`IsVariant`], for each variant `foo` of an enum type, derives a `is_foo` method.
//! 3. [`Unwrap`], for each variant `foo` of an enum type, derives an `unwrap_foo` method.
//!
//! ## Generated code
//!
//! ## Installation
//!
//! This library requires Rust 1.36 or higher and it supports `no_std` out of the box.
//! Then add the following to `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! clipmdplus = "0.99.0"
//! # You can specifiy the types of derives that you need for less time spent
//! # compiling. For the full list of features see this crate its Cargo.toml.
//! default-features = false
//! features = ["from", "add", "iterator"]
//! ```
//!
//! And this to the top of your Rust file for Rust 2018:
//!
//! ```rust
//! extern crate clipmdplus;
//! // use the derives that you want in the file
//! use clipmdplus::{Add, Display, From};
//! ```
//! If you're still using Rust 2015 you should add this instead:
//! ```rust
//! extern crate core;
//! #[macro_use]
//! extern crate clipmdplus;
//! # fn main(){}
//! ```
//!
//! [`cargo-expand`]: https://github.com/dtolnay/cargo-expand
//! [`derive-new`]: https://github.com/nrc/derive-new
//!
//! [`From`]: https://dhorse2.github.io/clipmdplus/clipmdplus/from.html
//! [`Into`]: https://dhorse2.github.io/clipmdplus/clipmdplus/into.html
//! [`FromStr`]: https://dhorse2.github.io/clipmdplus/clipmdplus/from_str.html
//! [`TryInto`]: https://dhorse2.github.io/clipmdplus/clipmdplus/try_into.html
//! [`IntoIterator`]: https://dhorse2.github.io/clipmdplus/clipmdplus/into_iterator.html
//! [`AsRef`]: https://dhorse2.github.io/clipmdplus/clipmdplus/as_ref.html
//! [`AsMut`]: https://dhorse2.github.io/clipmdplus/clipmdplus/as_mut.html
//!
//! [`Display`-like]: https://dhorse2.github.io/clipmdplus/clipmdplus/display.html
//!
//! [`Error`]: https://dhorse2.github.io/clipmdplus/clipmdplus/error.html
//!
//! [`Index`]: https://dhorse2.github.io/clipmdplus/clipmdplus/index_op.html
//! [`Deref`]: https://dhorse2.github.io/clipmdplus/clipmdplus/deref.html
//! [`Not`-like]: https://dhorse2.github.io/clipmdplus/clipmdplus/not.html
//! [`Add`-like]: https://dhorse2.github.io/clipmdplus/clipmdplus/add.html
//! [`Mul`-like]: https://dhorse2.github.io/clipmdplus/clipmdplus/mul.html
//! [`Sum`-like]: https://dhorse2.github.io/clipmdplus/clipmdplus/sum.html
//! [`IndexMut`]: https://dhorse2.github.io/clipmdplus/clipmdplus/index_mut.html
//! [`DerefMut`]: https://dhorse2.github.io/clipmdplus/clipmdplus/deref_mut.html
//! [`AddAssign`-like]: https://dhorse2.github.io/clipmdplus/clipmdplus/add_assign.html
//! [`MulAssign`-like]: https://dhorse2.github.io/clipmdplus/clipmdplus/mul_assign.html
//!
//! [`Constructor`]: https://dhorse2.github.io/clipmdplus/clipmdplus/constructor.html
//! [`IsVariant`]: https://dhorse2.github.io/clipmdplus/clipmdplus/is_variant.html
//! [`Unwrap`]: https://dhorse2.github.io/clipmdplus/clipmdplus/unwrap.html

// todo Review the following doc options:
// #![doc(html_favicon_url = "https://example.com/favicon.ico")]
// #![doc(html_logo_url = "https://example.com/logo.jpg")]
// #![doc(issue_tracker_base_url = "https://github.com/rust-lang/rust/issues/")]
// #![doc(html_no_source)]
// per item doc options
// #[doc(inline)] (no_inline)
// #[doc(hidden)]
// #[doc(alias = "TheAlias")]
// FFI might need this for clarity and search:
// #[doc(alias = "lib_name_do_something")] 
// !------------------------------------------------------------
// #![warn(missing_docs)]
// #![allow(unused_macros, unused_imports)]
// #![cfg_attr(
//     feature = "document-features",
//     // #[doc(hidden)]
//     cfg_attr(doc, doc = ::document_features::document_features!())
//     // #![doc = document_features::document_features!()]
// )]
// !------------------------------------------------------------
// extern crate clipmdplus;
// extern crate clipmdplus_library;
// #[macro_use]
// extern crate clipmdplus_macro;
// extern crate derive_more;
// !------------------------------------------------------------
// #[allow(unused_imports)]
// #[macro_use]
extern crate clipmdplus_macro;
pub use crate::clipmdplus_macro::Name;
pub use crate::clipmdplus_macro::VariantName;
// 
pub use crate::clipmdplus_macro::Deserialize_enum;
pub use crate::clipmdplus_macro::Serialize_enum;

// #[macro_use]
extern crate clipmdplus_library;
#[allow(unused_imports)]
use clipmdplus_library::*;
// pub use clipmdplus_library::imafunc;

// #[doc(hidden)]
// #[macro_use]
extern crate derive_more;
pub use derive_more::*;

/// Design: A clipboard history should not have a UI,
/// a UI uses the history. It's always running.
/// Ideally the current clip should be independent of
/// the history, which itself should be optional,
/// or just live for the current session.
pub mod clip_util;
pub use self::clip_util::*;

/// The window and form.
/// todo some design notes re the form.
pub mod clip_form;
pub use self::clip_form::*;

/// Application specific standard library.
/// NOT std.
pub mod stdmd;
pub use self::stdmd::*;

/// Function main (bin) for ClipMdPlus.
/// Loads the clipboard history.
/// Opens the form(s).
#[tokio::main]
#[warn(unused_mut)]
pub async fn main() -> Result<(), reqwest::Error> {
    // trace_macros!(true);
    // println!("Hello, world!");
    // trace_macros!(false);
    let mut clip_form = clip_form::ClipForm::default();
    let ui_type = stdmd::types::UiType::EguiNative;
    let _clip_form_result = clip_form.run(ui_type);
    Ok(())
}

// Note on error handling:
// I'm using thiserror but there are other popular crates:
// •	error-chain
// •	failure
// •	quick-error
// •	Anyhow
// •	SNAFU
