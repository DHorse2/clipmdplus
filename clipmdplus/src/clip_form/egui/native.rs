// clip_form egui native.rs
/// Egui Native implimentation
/// Defined:
///     clip_form_native
///     clip_form_window_native
///     window_options_native.rs
/// Status: Opens with a "hello world"
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

use crate::stdmd;
// use stdmd::UiType;
// use stdmd::UiError;
// use stdmd::UiResult;
// use stdmd::WindowType;
use stdmd::WindowError;
use stdmd::WindowResult;

// use clip_form::egui::native::FormWindow;
// use clip_form::egui::native::WindowOptions;
//
// !------------------------------------------------------------
include!(".\\native\\clip_form_native.rs");
// !------------------------------------------------------------
include!(".\\native\\clip_form_window_native.rs");
// ! ------------------------------------------------------------
include!(".\\native\\window_options_native.rs");
// ! ------------------------------------------------------------
