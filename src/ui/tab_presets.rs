use egui::{ScrollArea, Widget};

use crate::settings::{Preset, Settings};

/// Content of the presets tab
pub struct TabPresets<'a> {
    settings: &'a mut Settings,
}

impl<'a> TabPresets<'a> {
    /// Creates a new [`TabPresets`].
    pub fn new(settings: &'a mut Settings) -> Self {
        Self { settings }
    }
}

impl Widget for TabPresets<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.add(super::text_edit_singleline_touchscreen(
                    &mut self.settings.current_preset.name,
                    self.settings.system.force_touchscreen,
                ));
                if ui.button("💾").clicked() {
                    self.settings
                        .presets
                        .push(self.settings.current_preset.clone());
                }
            });

            ui.separator();
            ScrollArea::vertical().show(ui, |ui| {
                let mut delete = None;
                for (index, preset) in self.settings.presets.iter().enumerate() {
                    ui.horizontal(|ui| {
                        if ui.button("🗑").clicked() {
                            delete = Some(index);
                        }
                        ui.selectable_value(
                            &mut self.settings.current_preset,
                            preset.clone(),
                            preset.name.clone(),
                        );
                        ui.add_space(ui.available_width());
                    });
                }

                if let Some(delete_index) = delete {
                    self.settings.presets.remove(delete_index);
                }

                for preset in Preset::system_presets() {
                    ui.horizontal(|ui| {
                        ui.selectable_value(
                            &mut self.settings.current_preset,
                            preset.clone(),
                            preset.name.clone(),
                        );
                        ui.add_space(ui.available_width());
                    });
                }
            });
        })
        .response
    }
}
