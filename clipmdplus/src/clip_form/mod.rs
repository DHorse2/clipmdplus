#![allow(dead_code, unused_imports)] 
/// ClipForm clip_form mod.rs
/// This is the ClipForm main module.
/// It displays the clipboard history data.
/// Actual forms are in crate specific modules
/// the only form module created so far is:
///     egui
/// although I'm been looking at several crates.
// object creation (pre debug)
//
extern crate chrono;
extern crate libc;
use std::fmt; 
use std::str;
//
use crate::UiType;
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
/// The egui crate is used along with eframe to create the window.
pub mod egui;
//
// !------------------------------------------------------------
/// This is the meta data for the ClipForm.
/// It include form settings and clipboard objects.
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
    /// This is the internal next unique clip id.
    pub sequence_number_current: i32,
    /// This is a range of unique clip id's currently selected within the form window.
    pub sequence_number_selected: Range,
    /// Last Clipboard String
    // IDataObject is the OS clipboard object
    clip_data_last: String,

    /// Constants, std and core functionality not implemented.
    /// Program control item. Used to initialize and load the clipboard.
    // StdBaseRunFileConsoleDef st = new StdBaseRunFileConsoleDef();
    prog_control: ProgControl,
    /// The user interface type used by the form (egui currently).
    pub ui_type: UiType,

    /// The meta data for the current clip.
    pub clip_meta: ClipMeta,
    // MemoryStream ClipMetaMemStream = new MemoryStream();
    /// The meta data in string form. Not going to be used?
    pub clip_meta_out: String,

    // File:
    // mFileSql ClipFile;
    /// A vector containing the history of clips.
    pub clip_hist: Vec<ClipMeta>, // List<ClipMeta> 

    // #region HyperlinkList Fields
    /// Hyperlinks extracted from the current clip.
    pub hyperlink_list: HyperlinkList,

    // todo A pointer to the OS shell clipboard handling.
    // tmpPath: Shell32.ShellLinkObject,
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
#[allow(dead_code, unused_imports)] // object creation (pre debug)
impl ClipForm {
    /// Create the form using the run function.
    #[allow(unused_mut, unused_variables, unused_assignments)] // temp
    pub fn run(&mut self, mut ui_type: UiType) -> Result<bool, String> {
        // Form Creation
        // Form Object Creation
        // Form Menu Definition
        // Form Component Definition
        let _ = self.initialize_clip_component(&mut ui_type);

        // ui_type: UiTpye
        self.ui_type = ui_type.clone();
        // eframe::Result<()>
        let eframe_result: Result<bool, _> = Ok(true);
        // let eframe_result: eframe::Result<bool> = Ok(true);
        match ui_type {
            UiType::EguiNative => {
                //  Result<(), eframe::Error>
                self.ui_type = UiType::EguiNative;
                ui_type = UiType::EguiNative.clone();
                // let mut ui_type = UiType::EguiNative;
                let _clip_form_result = ClipFormEgui::run();
                // let mut clip_form = ClipFormEgui::new(cc); // ::egui::FormWindow::default(); // ClipForm::default();
                // let _clip_form_result: Result<(), eframe::Error> = clip_form.run(&mut ui_type);
            },
            UiType::EguiWeb => {
                self.ui_type = UiType::None;
                ui_type = UiType::EguiNative.clone();
                return Err(String::from("undefined UI type"));
                // todo!(); // Web UI and API.
            },
            UiType::None => {
                self.ui_type = UiType::None;
                // allowed.
                // todo!(); // NoUi (maybe background or service)
            },
            _ => {
                return Err(String::from("undefined UI type"));
            },
        }
        // Ok(true)
        eframe_result
    }
    // todo! Complete UI design and component definitions
// !------------------------------------------------------------
    #[allow(unused_mut, unused_variables)] // temp
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

    #[allow(unused_mut, unused_variables)] // temp
    fn file_open(&self) -> Result<bool, String>
    { 
        // ClipFile = new mFileSql();
        Ok(true)
    }

    #[allow(unused_mut, unused_variables)] // temp
    fn clipboard_data_read(&self, select_cmd_passed: &str) -> Result<bool, String>
    {
        Ok(true)
    }

    #[allow(unused_mut, unused_variables)] // temp
    fn clipboard_data_get(&self, use_clipboard_passed: bool) -> Result<bool, String>
    {
        Ok(true)
    }
}
