// hyperlink mod.rs
// hyperlink_meta HyperlinkMeta fix this? (is shadowing an issue?)

use std::collections::HashMap;

use serde::Deserialize;
// use serde::de::Error;
// use serde::de::{self, Unexpected, Visitor};
// use serde::de::{MapAccess, SeqAccess};

use serde::Serialize;
// use serde::ser::SerializeStruct;

// use serde::forward_to_deserialize_any;
// use serde::forward_to_deserialize_any_helper;

// use serde_json::{Deserializer, Serializer, Value};

// #region HyperlinkList Fields
/// <summary>
/// Hyperlink meta data including link type,
/// document type and process verb.
/// </summary> 

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperlinkMeta {
    pub hyperlink_type: Option<HyperlinkTypeIs>,
    pub document_type: Option<DocumentTypeIs>,
    pub process_name: Option<String>,
    pub process_startup_info: Option<ProcessStartInfo>,
}
impl Default for HyperlinkMeta {
    fn default() -> Self {
        HyperlinkMeta {
            hyperlink_type: Some(HyperlinkTypeIs::default()),
            document_type: Some(DocumentTypeIs::default()),
            process_name: None,
            process_startup_info: None,
        }
    }
}
#[allow(dead_code, unused, unused_imports)] // object creation (pre debug)
impl HyperlinkMeta {
    // Constructors ;-)
    pub fn new() -> Self {
        HyperlinkMeta::default()
    }
    pub fn new_process_name(passed_process_name: &str) -> Self {
        HyperlinkMeta {
            hyperlink_type: Some(HyperlinkTypeIs::default()),
            document_type: Some(DocumentTypeIs::default()),
            process_name: Some(passed_process_name.to_string()),
            process_startup_info: Some(ProcessStartInfo::default()),
        }
    }
    //     public HyperlinkMeta(string PassedProcessName, int iPassedHyperlinkType) {
    pub fn new_hyperlink_type(passed_process_name: &str, passed_hyperlink_type: HyperlinkTypeIs) -> Self {
        HyperlinkMeta {
            hyperlink_type: Some(passed_hyperlink_type),
            document_type: Some(DocumentTypeIs::default()),
            process_name: Some(passed_process_name.to_string()),
            process_startup_info: Some(ProcessStartInfo::default()),
        }
    }
    
    //     public HyperlinkMeta(string PassedProcessName, int iPassedHyperlinkType, int iPassedDocumentType) {
    pub fn new_document_type(passed_process_name: &str, passed_hyperlink_type: HyperlinkTypeIs, passed_document_type: DocumentTypeIs) -> Self {
        HyperlinkMeta {
            hyperlink_type: Some(passed_hyperlink_type),
            document_type: Some(passed_document_type),
            process_name: Some(passed_process_name.to_string()),
            process_startup_info: Some(ProcessStartInfo::default()),
        }
    }
        
    pub fn set_hyperlink_data(&mut self, passed_process_name: String, passed_hyperlink_type: HyperlinkTypeIs, passed_document_type: DocumentTypeIs) {
        self.hyperlink_type = Some(passed_hyperlink_type);
        self.document_type = Some(passed_document_type);
        self.process_name = Some(passed_process_name);
        // if (PassedProcessName.Length == 0) { PassedProcessName = "New Document.txt"; }
        // ProcessStartupInfo = ProcessStartInfo::new(PassedProcessName);
    }
}

// !------------------------------------------------------------
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
/// <summary>
/// Dictionary list containing unique hyperlinks.
/// </summary> 
// Dictionary<string, HyperlinkMeta> HyperlinkList = new Dictionary<string, HyperlinkMeta>();
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperlinkList { hashmap: HashMap<String, HyperlinkMeta> }
impl Default for HyperlinkList {
    fn default() -> Self {
        HyperlinkList { hashmap: HashMap::new() } // ! todo
    }
}
// !------------------------------------------------------------
/// <summary>
/// Hyperlink types.
/// </summary> 
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum HyperlinkTypeIs {
    Hyperlink,
    Folder,
    UncFolder,
    UncFile,
    MailTo,
    // #[default] to serialize? sql?
    Document,
    None
}
impl Default for HyperlinkTypeIs {
    fn default() -> Self { HyperlinkTypeIs::Document }
}
// !------------------------------------------------------------
/// <summary>
/// Document type hyperlink points to.
/// </summary> 
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DocumentTypeIs {
    WordDocument,
    ExcelDocument,
    OpenOfficeDocument,
    // #[default] 
    TextDocument,
    CodeDocument,
    BatDocument,
    ScriptDocument,
    None
}
impl Default for DocumentTypeIs {
    fn default() -> Self { DocumentTypeIs::TextDocument }
}
// #endregion
