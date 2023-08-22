// clipMdPlusMacro lib.rs
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]

//! ClipMdPlus Trait Library
//!
//! The trait crate clipmdplus_library will have some of it's traits removed.
//! It exists to create a #[derive(clipboard_thingy, todo clarify...)]
//! I'm learning this feature of RUST, beyond that many of these traits are useful.
//! For example, using macro_rules! would be less restrictive.
//! 
//! use super::super::Name;
//! use super::super::Named;
//! use super::super::VariantName;
//! use clipmdplus_library::VariantName;
//!

// pub extern crate document_features;
// use document_features::document_features;
// #![doc = document_features::document_features!()]
// #[doc = document_features::document_features!()]

// extern crate proc_macro;
// use lazy_static::lazy_static;
// use proc_macro::TokenStream;
// use proc_macro2::TokenStream as TokenStream2;
// use quote::quote;
// use std::collections::HashMap;
// use syn::*;
// use syn::{self, parse_quote, Arm, Data};
// use syn::{Attribute, DataEnum, DeriveInput, Expr, ExprLit, ExprParen, Fields, Lit, parse2, parse_macro_input, Variant}; // path, tokens, Token, 
// use std::fmt;
// use std::io::prelude::*;
// use std::net;
// ! derive_name_trait ------------------------------------------------------------
// #[macro_use]
// todo don't export everthing everywhere.

// pub mod pub_derive_name_trait;
// pub use self::pub_derive_name_trait::*;
// // include!(".\\pub_derive_name_trait.rs");
pub fn imafunc() -> String {
    "string".to_string().clone()
}
pub fn imafunc2<'dumm>(mut i: String) -> String {
    // i = AsMut::as_mut(&mut "string");
    // i = &mut String::from("string").as_str().to_owned();
    // i = &mut "string".to_owned().to_string();
    i.push_str("oops");
    if i == "Help" {
        // i = &mut "string".clone();
        i = String::from("string".to_string());
    }
    i
}