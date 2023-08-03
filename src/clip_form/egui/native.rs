// clip_form egui native.rs
//
#[allow(dead_code, unused, unused_imports)] 
    // object creation (pre debug)
    // also seems to be needed for API's
//
// use std::any::{Any, TypeId};
// use std::error::Error;
// use std::fmt;
// use std::marker::PhantomData;
// use std::result::Result;
// use std::sync::Arc;
// use std::time::Instant;
//
// use eframe::egui::*;
// use eframe::*;
// use egui::*;
//
// pub mod form_window_native;
//
use crate::stdmd::types::UiType;
// use crate::stdmd::types::UiError;
// use crate::stdmd::types::UiResult;
// use crate::stdmd::types::WindowType;
use crate::stdmd::types::WindowError;
use crate::stdmd::types::WindowResult;

// use crate::clip_form::egui::native::FormWindow;
// use crate::clip_form::egui::native::WindowOptions;
//
// !------------------------------------------------------------
include!(".\\native\\clip_form_native.rs");
// !------------------------------------------------------------
include!(".\\native\\form_window_native.rs");
// ! ------------------------------------------------------------
include!(".\\native\\window_options.rs");
// ! ------------------------------------------------------------
