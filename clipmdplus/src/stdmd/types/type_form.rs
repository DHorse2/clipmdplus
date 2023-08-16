
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
// no good. unit variants only
// #[macro_use] extern crate custom_derive;
// #[macro_use] extern crate enum_derive;
// custom_derive! {
//     #[derive(Debug, PartialEq, Eq,
//         IterVariants(CandyVariants), IterVariantName(CandyVariantName))]
//     pub enum Candy { Musk, FruitRock, BoPeeps, LemonSherbert }
// }
