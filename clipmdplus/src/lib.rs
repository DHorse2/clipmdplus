#![warn(missing_docs)]
#![allow(unused_macros, unused_imports)]
#![cfg_attr(
    feature = "document-features",
    // #[doc(hidden)]
    cfg_attr(doc, doc = ::document_features::document_features!())
    // #![doc = document_features::document_features!()]
)]
//! ## ClipMdPlus Library
//! Consists of clip_util and stdmd (extended standard) and derive_name (extended) modules
//! 
//! ------------------------------------------------------------
// #![warn(missing_docs)]
// #![allow(missing_docs)]
// !------------------------------------------------------------
// mod macros;
// pub mod macros;
// pub use self::macros::*;
// !------------------------------------------------------------
#[allow(unused_imports)]
// #[macro_use]
extern crate clipmdplus_macro;
// pub use clipmdplus_macro::*;
use clipmdplus_macro::Deserialize_enum;
use clipmdplus_macro::Name;
use clipmdplus_macro::Serialize_enum;
use clipmdplus_macro::VariantName;
// !------------------------------------------------------------
// #[macro_use]
extern crate clipmdplus_library;
#[allow(unused_imports)]
pub use clipmdplus_library::*;
// pub use clipmdplus_library::imafunc;

// !------------------------------------------------------------
/// todo Note (again) these should be library features.
/// Derive more has many features and is idiomatic.
/// It compliments an abstraction layer.
// #[macro_use]
extern crate derive_more;
#[doc(hidden)]
pub use derive_more::*;

// Design: The clipboard history UI
// uses this history and utils. 
// It should always be running in order to capture clipboard events.
// Ideally clip_util should be a background service independent of
// the history, which can optionally be permanent or when started/running.
/// Clipboard history utilities.
pub mod clip_util;
pub use self::clip_util::*;

// Extended standard library.
// NOT std. This is application level cross-cutting abstractions.
// In our case, clipboards have a rich variety of data types.
// Error types are another example. Traits. Basically ".\Common\".
/// Extended standard library.
pub mod stdmd;
pub use self::stdmd::*;
