use egui::Widget;

use crate::settings::{Preset, Settings};

pub struct TabPresets<'a> {
    settings: &'a mut Settings,
}

impl<'a> TabPresets<'a> {
    pub fn new(settings: &'a mut Settings) -> Self {
        Self { settings }
    }
}

impl Widget for TabPresets<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.settings.current_preset.name);
                ui.add_enabled_ui(self.settings.can_save_current_preset(), |ui| {
                    if ui.button("💾").clicked() {
                        self.settings.save_current_preset();
                    }
                });
            });

            ui.separator();
            ui.horizontal_wrapped(|ui| {
                ui.selectable_value(
                    &mut self.settings.current_preset,
                    Preset::default(),
                    "Default",
                );
                let mut delete = None;
                for preset in &self.settings.presets {
                    ui.selectable_value(
                        &mut self.settings.current_preset,
                        preset.clone(),
                        preset.name.clone(),
                    );
                    if &self.settings.current_preset == preset && ui.button("🗑").clicked() {
                        delete = Some(preset.name.clone());
                    }
                }

                if let Some(delete) = delete {
                    self.settings.delete_preset(&delete);
                }
            })
        })
        .response
    }
}
