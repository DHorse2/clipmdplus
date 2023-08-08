// form_window.rs

#[derive(Clone, Default, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
// #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct FormWindow {}

impl FormWindow {
    // super::Demo for 
    #[allow(dead_code, unused_variables, unused_imports)] // object creation (pre debug)
    fn name(&self) -> &'static str {
        "🗖 Window With Panels"
    }

    #[allow(dead_code, unused_variables, unused_imports)] // object creation (pre debug)
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        // use super::View as _;
        let window = egui::Window::new("Clipboard MD Plus")
            .default_width(300.0)
            .default_height(150.0)
            .vscroll(true)
            .open(open);
        window.show(ctx, |ui| self.ui(ui));
    }
}

impl FormWindow {
    // from super::View for 
    #[allow(dead_code, unused_variables, unused_imports)] // object creation (pre debug)
    fn ui(&mut self, ui: &mut egui::Ui) {
        // Note that the order we add the panels is very important!

        egui::TopBottomPanel::top("top_panel")
            .resizable(true)
            .min_height(32.0)
            .show_inside(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Contents");
                    });
                    // CONTENT
                    // lorem_ipsum(ui);
                    todo!() // define top panel
                });
            });

        egui::SidePanel::left("left_panel")
            .resizable(true)
            .default_width(150.0)
            .width_range(80.0..=200.0)
            .show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Left Panel");
                });
                egui::ScrollArea::vertical().show(ui, |ui| {
                // CONTENT
                // lorem_ipsum(ui);
                todo!() // define Top Left Panel
            });
            });

        egui::SidePanel::right("right_panel")
            .resizable(true)
            .default_width(150.0)
            .width_range(80.0..=200.0)
            .show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Right Panel");
                });
                egui::ScrollArea::vertical().show(ui, |ui| {
                    // CONTENT
                    // lorem_ipsum(ui); // mockup
                    todo!() // define Top Right Panel
                });
            });

        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(true)
            .min_height(100.0)
            .show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Bottom Panel");
                });
                egui::ScrollArea::vertical().show(ui, |ui| {
                    // CONTENT
                    // lorem_ipsum(ui);
                    todo!() // define Bottom Pannels
                });
            });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Central Panel");
            });
            egui::ScrollArea::vertical().show(ui, |ui| {
                // CONTENT
                // lorem_ipsum(ui);
                todo!() // Scroll Areas
            });
        });
    }
}

// fn lorem_ipsum(ui: &mut egui::Ui) {
//     ui.with_layout(
//         egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true),
//         |ui| {
//             ui.label(egui::RichText::new(egui::LOREM_IPSUM_LONG).small().weak());
//             ui.add(egui::Separator::default().grow(8.0));
//             ui.label(egui::RichText::new(egui::LOREM_IPSUM_LONG).small().weak());
//         },
//     );
// }