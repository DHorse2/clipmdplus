
// !------------------------------------------------------------
/// UiType indicates the crate used to generates the UI form.
/// Currently only EguiNative is implemented.
#[derive(Clone, Debug, PartialEq)]
pub enum UiType {
    /// Uses the Egui Native OS rendering crate.
    EguiNative,
    /// Uses the Egui Web interface.
    EguiWeb,
    /// This is just an example. Not used.
    Tauri,
    /// A number of UI crates are being evaluated.
    ManyMore,
    /// A user interface is not a requirement.
    /// The clipboard can be run without a UI.
    /// Or alternatively run without a form but with context menus.
    /// As such it acts similarly to a service when in this mode.
    None
}
/// UiError is the custom error used.
/// It indicates the source of the error within the UI form.
/// Currently only EguiNative is implemented.
#[derive(Clone, Debug, PartialEq)]
pub enum UiError {
    /// The window had a WindowError.
    Window(WindowError),
    /// Egui EFrame had an error.
    Eframe,
    /// Egui had an error.
    Egui,
    /// The error came from the Web interface.
    /// It eventually could be the form page or API.
    Web,
    /// not used.
    None
}
/// There are multiple forms (ui components) planned.
/// Currently only Modal is implemented.
#[derive(Clone, Debug, PartialEq)]
pub enum WindowType {
    /// Modal windows can be sized and/or maximised.
    Modal(String),
    /// Context (right click) UI objects.
    ContextMenu(String),
}
/// Window Error is the custom error used.
/// It indicates the source of the error within the UI form.
/// Currently only EguiNative is implemented.
#[derive(Clone, Debug, PartialEq)]
pub enum WindowError {
    /// An egui error.
    EguiError(String),
    /// Error comes from EFrame (for egui)
    EframeRunNative(String),
    /// Unknown origin of error.
    Unknown(String),
}
/// UiResult using a String and UiError.
pub type UiResult = std::result::Result<String, UiError>;
/// WindowResult using a String and WindowError.
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
