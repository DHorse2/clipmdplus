// ClipForm clip_form.rs
#[allow(dead_code, unused, unused_imports)] // object creation (pre debug)
//
extern crate chrono;
extern crate libc;
extern crate sys;
//
// use serde::{
//     Deserialize, Serialize,
// };
// use std::env;
// use std::env::consts::*;
// use std::error::Error;
use std::fmt; 
// use std::fs::File;
//
// use std::io::prelude::*;
// use std::io::{
    // Error,
    // ErrorKind,
    // self, 
    // BufReader, 
    // Read,
    // Result, 
    // Write, 
// };
// use std::os::windows::prelude::*;
// use std::path::PathBuf;
// use std::process;
use std::str;
use crate::UiType;
// use std::thread;
//
// (see chrono below)
// use std::time::{ 
//     Duration, 
//     Instant,
//     SystemTime
// };
//
// use crate::clip_form::egui::native:
// use crate::clip_form::egui::native::FormWindow;
// use crate::clip_form::egui::native::WindowOptions;
use crate::clip_form::egui::native::ClipFormEgui;

use crate::clip_util::ClipMeta;
// use crate::clip_util::ClipboardMeta;
use crate::clip_util::ProgControl;
//
// use crate::stdmd::hyperlink::HyperlinkMeta;
use crate::stdmd::hyperlink::HyperlinkList;
use crate::stdmd::range::Range;
// use crate::stdmd::types::UiType;
// use crate::stdmd::types::UiError;
// use crate::stdmd::types::UiResult;
// use crate::stdmd::types::WindowType;
// use crate::stdmd::types::WindowError;
// use crate::stdmd::types::WindowResult;
use crate::stdmd::sequence::Sequence;

pub mod egui;
//
// !------------------------------------------------------------
// #[allow(non_snake_case)]
#[derive(Clone, Debug, PartialEq)]
pub struct ClipForm {
    //     public class ClipFormMain : System.Windows.Forms.Form
    // todo! Create Native option of ClipForm
    // #region Constants
    // #endregion
    // !#region Fields
    // public IDataObject iData;
    // public ScriptEngine sciptEngine = new ScriptEngine();
    // public PropManager propManager = new PropManager();
    // private Func<Int32> localFunc;
    // !#region Program Fields
    show_when_minimized: bool,
    show_from_menu: bool,
    has_file: bool,

    /// Os clipboard chain pointer
    // IntPtr _ClipboardViewerNext;
    // // IDataObject iData;
    pub sequence: Sequence, 
    pub sequence_number_current: i32,
    pub sequence_number_selected: Range,
    // Last Clipboard String
    // IDataObject is the OS clipboard object
    clip_data_last: String,

    // Constants, std and core functionality not implemented
    // StdBaseRunFileConsoleDef st = new StdBaseRunFileConsoleDef();
    prog_control: ProgControl,
    ui_type: UiType,

    pub clip_meta: ClipMeta,
    // MemoryStream ClipMetaMemStream = new MemoryStream();
    pub clip_meta_out: String,

    // File:
    // mFileSql ClipFile;

    pub clip_hist: Vec<ClipMeta>, // List<ClipMeta> 

    // #region HyperlinkList Fields
    pub hyperlink_list: HyperlinkList,

    // tmpPath: Shell32.ShellLinkObject,

    // !#endregion Program Fields

    // !#region Form Fields
    // egui
    
    // !#endregion

    // !#endregion Fields

}

impl fmt::Display for ClipForm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "Clipboard data {} {} {:?}", self.id_key, self.data1, self)
        write!(f, "Clip Form Main {:?}", self)
    }
}
impl Default for ClipForm {
    fn default() -> Self {
        ClipForm {
            show_when_minimized: true,
            show_from_menu: true,
            has_file: false,
            sequence: Sequence::default(), 
            sequence_number_current: i32::default(),
            sequence_number_selected: Range::default(),
            clip_data_last: String::new(),
            prog_control: ProgControl::default(),
            ui_type: UiType::EguiNative,
            clip_meta: ClipMeta::default(),
            clip_meta_out: String::new(),
            clip_hist: Vec::new(), // List<ClipMeta> 
            hyperlink_list: HyperlinkList::default(),
        }
    }
}

// !------------------------------------------------------------
#[allow(dead_code, unused, unused_imports)] // object creation (pre debug)
impl ClipForm {
    // type Error;

    pub fn run(&mut self,  ui_type: &mut UiType) -> Result<bool, String> {
        // Form Creation
        // Form Object Creation
        // Form Menu Definition
        // Form Component Definition
        let _ = self.initialize_clip_component(ui_type);

        // ui_type: UiTpye
        self.ui_type = ui_type.clone();
        // eframe::Result<()>
        let mut eframe_result: eframe::Result<()> = Ok(());
        match ui_type {
            UiType::EguiNative => {
                //  Result<(), eframe::Error>
                let mut ui_type = UiType::EguiNative;
                let _clip_form_result = ClipFormEgui::run();
                // let mut clip_form = ClipFormEgui::new(cc); // ::egui::FormWindow::default(); // ClipForm::default();
                // let _clip_form_result: Result<(), eframe::Error> = clip_form.run(&mut ui_type);
            },
            UiType::EguiWeb => {
                return Err(String::from("undefined UI type"));
                // todo!(); // Web UI and API.
            },
            UiType::None => {
                // allowed.
                // todo!(); // NoUi (maybe background or service)
            },
            _ => {
                return Err(String::from("undefined UI type"));
            },
        }
        Ok(true)
    }
    // todo! Complete UI design and component definitions
// !------------------------------------------------------------
    fn initialize_clip_component(&mut self, ui_type: &mut UiType) -> Result<bool, String>
    {
        // ? closes open files?
        // This array will be used to keep track of changes to the clipboard data
        self.prog_control = ProgControl::default();
        self.clip_meta = ClipMeta::default();
        self.clip_hist = Vec::new();
        //
        self.has_file = self.file_open().unwrap();
        //
        self.sequence = Sequence::default();
        self.sequence_number_current = self.sequence.sequence_get();
        let _ = self.clipboard_data_read("")?;
        let _ = self.clipboard_data_get(true)?;

        Ok(true)
    }

    fn file_open(&self) -> Result<bool, String>
    { 
        // ClipFile = new mFileSql();
        Ok(true)
    }
    fn clipboard_data_read(&self, select_cmd_passed: &str) -> Result<bool, String>
    {
        Ok(true)
    }
    fn clipboard_data_get(&self, use_clipboard_passed: bool) -> Result<bool, String>
    {
        Ok(true)
    }
}
