// Stdmd Hyperlink mod.rs
/// hyperlink_meta HyperlinkMeta.

use std::collections::HashMap;
//
use serde::Deserialize;
// use serde::de::Error;
// use serde::de::{self, Unexpected, Visitor};
// use serde::de::{MapAccess, SeqAccess};
use serde::Serialize;
// use serde::ser::SerializeStruct;
//
// use serde::forward_to_deserialize_any;
// use serde::forward_to_deserialize_any_helper;
//
// use serde_json::{Deserializer, Serializer, Value};

// #region HyperlinkList Fields
/// <summary>
/// Hyperlink meta data including link type,
/// document type and process verb.
    /// hyperlink_data: The actual hyperlink as a string.
    /// hyperlink_type: The document type that the link specifies.
    /// process_name:   Which process (application?) handles the link?
    /// process_startup_info: The additional info needed to launch the process.
/// </summary> 

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperlinkMeta {
    /// The extracted hyperlink.
    pub hyperlink_data: Option<String>,
    /// The link's type.
    pub hyperlink_type: Option<HyperlinkTypeIs>,
    /// The document type pointed to.
    pub document_type: Option<DocumentTypeIs>,
    /// What process handles the link.
    pub process_name: Option<String>,
    /// The startup parameter string required by the process.
    pub process_startup_info: Option<ProcessStartInfo>,
}
impl Default for HyperlinkMeta {
    fn default() -> Self {
        HyperlinkMeta {
            hyperlink_data: None,
            hyperlink_type: Some(HyperlinkTypeIs::default()),
            document_type: Some(DocumentTypeIs::default()),
            process_name: None,
            process_startup_info: None,
        }
    }
}
#[allow(dead_code, unused_imports)] // object creation (pre debug)
impl HyperlinkMeta {
    // Constructors ;-)
    /// Creates a new default empty hyperlink.
    pub fn new() -> Self {
        HyperlinkMeta::default()
    }
    /// Creates a new hyperlink and sets the process name.
    pub fn new_process_name(passed_process_name: &str) -> Self {
        HyperlinkMeta {
            hyperlink_data: None,
            hyperlink_type: Some(HyperlinkTypeIs::default()),
            document_type: Some(DocumentTypeIs::default()),
            process_name: Some(passed_process_name.to_string()),
            process_startup_info: Some(ProcessStartInfo::default()),
        }
    }
    //     public HyperlinkMeta(string PassedProcessName, int iPassedHyperlinkType) {
    /// Creates a new hyperlink and sets the process name, link type.
    pub fn new_hyperlink_type(passed_process_name: &str, passed_hyperlink_type: HyperlinkTypeIs) -> Self {
        HyperlinkMeta {
            hyperlink_data: None,
            hyperlink_type: Some(passed_hyperlink_type),
            document_type: Some(DocumentTypeIs::default()),
            process_name: Some(passed_process_name.to_string()),
            process_startup_info: Some(ProcessStartInfo::default()),
        }
    }
    
    //     public HyperlinkMeta(string PassedProcessName, int iPassedHyperlinkType, int iPassedDocumentType) {
    /// Creates a new hyperlink and sets the process name, link type and document type.
    pub fn new_document_type(passed_process_name: &str, passed_hyperlink_type: HyperlinkTypeIs, passed_document_type: DocumentTypeIs) -> Self {
        HyperlinkMeta {
            hyperlink_data: None,
            hyperlink_type: Some(passed_hyperlink_type),
            document_type: Some(passed_document_type),
            process_name: Some(passed_process_name.to_string()),
            process_startup_info: Some(ProcessStartInfo::default()),
        }
    }
    /// Sets the process name, link type and document type.
    pub fn set_hyperlink_data(&mut self, passed_process_name: String, passed_hyperlink_type: HyperlinkTypeIs, passed_document_type: DocumentTypeIs) {
        self.hyperlink_type = Some(passed_hyperlink_type);
        self.document_type = Some(passed_document_type);
        self.process_name = Some(passed_process_name);
        // if (PassedProcessName.Length == 0) { PassedProcessName = "New Document.txt"; }
        // ProcessStartupInfo = ProcessStartInfo::new(PassedProcessName);
    }
}

// !------------------------------------------------------------
/// This Process (app) Startup Info is generic to any application
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProcessStartInfo {
    // Windows Diagnostics ProcessStartInfo
}
impl Default for ProcessStartInfo {
    fn default() -> Self {
        ProcessStartInfo { } // ! todo
    }
}

// !------------------------------------------------------------
/// HyperlinkList
/// <summary>
/// Dictionary list containing unique hyperlinks.
/// </summary> 
// Dictionary<string, HyperlinkMeta> HyperlinkList = new Dictionary<string, HyperlinkMeta>();
#[derive(Clone, Debug, PartialEq)]
pub struct HyperlinkList { hashmap: HashMap<String, HyperlinkMeta> }
impl Default for HyperlinkList {
    fn default() -> Self {
        HyperlinkList { hashmap: HashMap::new() } // ! todo
    }
}
// impl HyperlinkList {
    // , Serialize, Deserialize
// }
// !------------------------------------------------------------
/// HyperlinkTypeIs
/// <summary>
/// Hyperlink types.
/// </summary> 
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum HyperlinkTypeIs {
    /// A web url link.
    Hyperlink,
    /// A folder link.
    Folder,
    /// A folder link using UNC.
    UncFolder,
    /// A file link using UNC.
    UncFile,
    /// An email address.
    MailTo,
    // #[default] to serialize? sql?
    /// A file link.
    Document,
    /// The null case.
    None
}
impl Default for HyperlinkTypeIs {
    fn default() -> Self { HyperlinkTypeIs::Document }
}
// !------------------------------------------------------------
/// DocumentTypeIs
/// <summary>
/// Document type hyperlink points to.
/// </summary> 
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DocumentTypeIs {
    /// MS Word (r) formatted document.
    WordDocument,
    /// MS Excel (r) document.
    ExcelDocument,
    /// An Open Office document.
    OpenOfficeDocument,
    /// Plain text.
    // #[default] 
    TextDocument,
    /// Text that is a programming language.
    CodeDocument,
    /// A Windows Bat script.
    BatDocument,
    /// A script or partial script.
    ScriptDocument,
    /// None null choice.
    None
}
impl Default for DocumentTypeIs {
    fn default() -> Self { DocumentTypeIs::TextDocument }
}
// #endregion
