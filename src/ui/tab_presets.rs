use std::os::windows::process::CommandExt;

use egui::{ScrollArea, Widget};

use crate::settings::{Preset, Settings};

pub struct TabPresets<'a> {
    settings: &'a mut Settings,
    tabtip: bool,
}

impl<'a> TabPresets<'a> {
    pub fn new(settings: &'a mut Settings, tabtip: bool) -> Self {
        Self { settings, tabtip }
    }
}

impl Widget for TabPresets<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                if ui
                    .text_edit_singleline(&mut self.settings.current_preset.name)
                    .clicked()
                    && self.tabtip
                {
                    // hack
                    // windows touchscreen keyboard does not show up by default
                    // running tabtip.exe opens it (on the nuc only)
                    // running tabtip directly trigger a privilege error, but through cmd.exe it works.
                    std::process::Command::new("cmd.exe")
                        .arg("/C")
                        .arg(r"C:\Program Files\Common Files\microsoft shared\ink\TabTip.exe")
                        .creation_flags(0x00000008) // DETACHED_PROCESS
                        .spawn()
                        .unwrap();
                }
                ui.add_enabled_ui(self.settings.can_save_current_preset(), |ui| {
                    if ui.button("💾").clicked() {
                        self.settings.save_current_preset();
                    }
                });
            });

            ui.separator();
            ScrollArea::vertical().show(ui, |ui| {
                ui.selectable_value(
                    &mut self.settings.current_preset,
                    Preset::default(),
                    "Default",
                );
                let mut delete = None;
                for preset in &self.settings.presets {
                    ui.horizontal(|ui| {
                        if ui.button("🗑").clicked() {
                            delete = Some(preset.name.clone());
                        }
                        ui.selectable_value(
                            &mut self.settings.current_preset,
                            preset.clone(),
                            preset.name.clone(),
                        );
                        ui.add_space(ui.available_width());
                    });
                }

                if let Some(delete) = delete {
                    self.settings.delete_preset(&delete);
                }
            });
        })
        .response
    }
}
