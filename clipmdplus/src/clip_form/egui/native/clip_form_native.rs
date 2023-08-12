// Clip_form equi native clip_form.native.rs
/// ClipForm using the Egui Native UI crate
#[derive(Default)]
pub struct ClipFormEgui {}

impl ClipFormEgui {
    /// Create a new egui window using eframe
    pub fn run() -> WindowResult  {
        let native_options = eframe::NativeOptions::default();
        let eframe_result = eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(ClipFormEgui::new(cc))));
        match eframe_result {
            Ok(()) => Ok("eframe app run_native ok".to_string()),
            Err(why) => {
                let message: String = format!("unable to open app window: {why}");
                Err(WindowError::EframeRunNative(message))
            },
            // _ => Err(WindowError::Unknown("unable error opening app window".to_string())),
        }
        // Ok("???")
    }
    /// Create a new egui window from a CreationContext using eframe
    #[allow(dead_code, unused_variables, unused_imports)] // object creation (pre debug)
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for ClipFormEgui {
    /// Update the egui eframe window
    #[allow(dead_code, unused_variables, unused_imports)] // object creation (pre debug)
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
           ui.heading("Hello World!");
       });
   }
}