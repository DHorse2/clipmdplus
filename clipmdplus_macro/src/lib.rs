// clipMdPlusMacro lib.rs
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]

//! ClipMdPlus Macro Library
//!
//! The macro crate clipmdplus_macro will have some of it's macros removed.
//! It exists to create a #[derive(clipboard_thingy, todo clarify...)]
//! I'm learning this feature of RUST, beyond that many of these macros are useful.
//! For example, using macro_rules! would be less restrictive.
//! 
//! use super::super::Name;
//! use super::super::Deserialize_enum;
//! use super::super::Serialize_enum;
//! use super::super::ToString;
//! use super::super::VariantName;
//! use clipmdplus_macro::VariantName;
//!

// #[doc = document_features::document_features!()]
// #![doc = crate::document_features::document_features!()]
#![warn(missing_docs)]

// extern crate document_features;
// use document_features::document_features;
// use crate::document_features::document_features;

// pub extern crate document_features;
// use document_features::document_features;

// #[macro_use]
extern crate proc_macro;
//
// derive_name macros
// use lazy_static::lazy_static;
// // proc_macro
// use proc_macro::TokenStream;
// use proc_macro2::TokenStream as TokenStream2;
// use quote::quote;
// use quote::TokenStreamExt;

// // serde_enum macros
// use std::collections::HashMap;
// // use syn::*;
// use syn::{self, parse_quote, Arm, Data};
// use syn::{Attribute, DataEnum, DeriveInput, Expr, ExprLit, ExprParen, Fields, Lit, parse2, parse_macro_input, Variant}; // path, tokens, Token, 

// ! serde_enum ------------------------------------------------------------
// mod serde_enum;
// #[macro_use]
//  pub mod serde_enum;
//  pub use self::serde_enum::*;
 include!(".\\serde_enum.rs");

// ! derive_name ------------------------------------------------------------
// mod derive_name;
// #[macro_use]
//  pub mod derive_name;
//  pub use self::derive_name::*;
 include!(".\\derive_name.rs");
// ! derive_more ------------------------------------------------------------
extern crate derive_more;
// use derive_more::{add_assign, add, as_mut, as_ref, constructor_derived, deref, deref_mut, display, from, from_str, index, index_mut, into, mul_assign, mul, not, try_into, is_variant};
// use derive_more::*;
//  pub mod derive_more;
//  pub use self::derive_more::*;
// include!(".\\derive_more.rs");

// ! derive_name_trait ------------------------------------------------------------
// extern crate derive_name_trait;
// use derive_name_trait::*;
//  pub mod derive_name_trait;
//  pub use self::derive_name_trait::*;
// include!(".\\derive_name_trait.rs");

// ! msequence ------------------------------------------------------------
// From termion Create a CSI-introduced sequence.
#[allow(unused_macros)]
// #[macro_export]
macro_rules! msequence {
    ($( $l:expr ),*) => { concat!("???", $( $l ),*) }; // "\x1B["
}
// ! ------------------------------------------------------------
/// Derive a non-CSI msequence (static) struct.
/// derive_str_sequence!("Clear the entire screen.", All, "2J");
#[allow(unused_macros)]
// #[macro_export]
macro_rules! derive_static_sequence {
    ($doc:expr, $name:ident, $value:expr) => {
        #[doc = $doc]
        #[derive(Copy, Clone, Debug)]
        pub struct $name;

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut Formatter) -> Result {
                write!(f, $value)
            }
        }

        impl AsRef<[u8]> for $name {
            fn as_ref(&self) -> &'static [u8] { $value.as_bytes() }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &'static str { $value }
        }
    };
}
// !------------------------------------------------------------
/// Derive a constant msequence (static) struct.
#[allow(unused_macros)]
// #[macro_export]
macro_rules! derive_const_sequence {
    ($doc:expr, $name:ident, $value:expr) => {
        #[doc = $doc]
        pub const $name: &str = $value;

        // use std::fmt; 
        // impl fmt::Display for $name {
        //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //         write!(f, $value)
        //     }
        // }

        // impl AsRef<[u8]> for $name {
        //     fn as_ref(&self) -> &'static [u8] { $value.as_bytes() }
        // }

        // impl AsRef<str> for $name {
        //     fn as_ref(&self) -> &'static str { $value }
        // }
    };
}
// !------------------------------------------------------------
/// duplicated mtermion macros:
/// termion's Create a CSI-introduced sequence.
#[allow(unused_macros)]
// #[macro_export]
macro_rules! csi {
    ($( $l:expr ),*) => { concat!("\x1B[", $( $l ),*) };
}
// !------------------------------------------------------------
/// termion's Derive a CSI sequence struct.
/// derive_csi_sequence!("Change the cursor style to blinking block", BlinkingBlock, "\x31 q");
#[allow(unused_macros)]
// #[macro_export]
macro_rules! derive_csi_sequence {
    ($doc:expr, $name:ident, $value:expr) => {
        #[doc = $doc]
        #[derive(Copy, Clone)]
        pub struct $name;

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, csi!($value))
            }
        }

        impl AsRef<[u8]> for $name {
            fn as_ref(&self) -> &'static [u8] { csi!($value).as_bytes() }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &'static str { csi!($value) }
        }
    };
}
// !------------------------------------------------------------
/// derive_fn!("Get user name.", ACCT, "get_current_username()");
#[allow(unused_macros)]
// #[macro_export]
macro_rules! derive_fn {
    ($doc:expr, $name:ident, $value:stmt, $type:ty) => {
        #[doc = $doc]
        pub fn $name() -> $type {
            $value
        }

        // impl fmt::Display for $name {
        //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //         write!(f, format!("fn {}{}{}", $doc, $name, $value))
        //     }
        // }

        //<fn()>? closure?
        // impl AsRef<[u8]> for $name {
        //     fn as_ref(&self) -> &'static [u8] { csi!($value).as_bytes() }
        // }

        // impl AsRef<str> for $name {
        //     fn as_ref(&self) -> &'static str { csi!($value) }
        // }
    };
}
// https://stackoverflow.com/questions/51344951/how-do-you-unwrap-a-result-on-ok-or-return-from-the-function-on-err
//  let res = unwrap_or_return!(callable(&mut param));
// !------------------------------------------------------------
#[allow(unused_macros)]
// #[macro_export]
macro_rules! unwrap_or_return {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => return,
        }
    }
}

