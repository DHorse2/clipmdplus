
// !------------------------------------------------------------
#[derive(Clone, Debug, PartialEq)]
pub enum UiType {
EguiNative,
EguiWeb,
// Tauri,
None
}
#[derive(Clone, Debug, PartialEq)]
pub enum UiError {
    Window(WindowError),
    Eframe,
    Egui,
    Web,
    None
}
#[derive(Clone, Debug, PartialEq)]
pub enum WindowType {
    Modal(String),
    ContextMenu(String),
}
#[derive(Clone, Debug, PartialEq)]
pub enum WindowError {
    EframeRunNative(String),
    Unknown(String),
}
pub type UiResult = std::result::Result<String, UiError>;
pub type WindowResult = std::result::Result<String, WindowError>;
// !------------------------------------------------------------
