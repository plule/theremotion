use egui::Widget;

use crate::settings::{Handedness, Settings};

/// Content of the settings tab
pub struct TabSettings<'a> {
    settings: &'a mut Settings,
}

impl<'a> TabSettings<'a> {
    /// Creates a new [`TabSettings`].
    pub fn new(settings: &'a mut Settings) -> Self {
        Self { settings }
    }
}

impl<'a> Widget for TabSettings<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.toggle_value(&mut self.settings.system.fullscreen, "Fullscreen")
                .on_hover_text("Start Theremotion in fullscreen. Requires restart.");
            ui.toggle_value(
                &mut self.settings.system.high_priority_process,
                "High Priority Process",
            )
            .on_hover_text(
                "Start Theremotion with an elevated process priority. Requires restart.",
            );
            ui.toggle_value(
                &mut self.settings.system.tabtip,
                "Use Tabtip for text input",
            )
            .on_hover_text("Enable if the visual keyboard is not opening when using a touchscreen");
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.settings.system.handedness,
                    Handedness::RightHanded,
                    "Right handed",
                )
                .on_hover_text(
                    "The left hand controls the volume and the right hand controls the pitch.",
                );
                ui.selectable_value(
                    &mut self.settings.system.handedness,
                    Handedness::LeftHanded,
                    "Left handed",
                )
                .on_hover_text(
                    "The left hand controls the pitch and the right hand controls the volume.",
                );
            });
        })
        .response
    }
}
