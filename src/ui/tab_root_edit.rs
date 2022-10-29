use egui::{RichText, Widget};
use staff::Pitch;

use crate::{controls, settings::Settings};

use super::KeyboardEditMode;

pub struct TabRootEdit<'a> {
    controls: &'a mut controls::Controls,
    settings: &'a mut Settings,
}

impl<'a> TabRootEdit<'a> {
    pub fn new(controls: &'a mut controls::Controls, settings: &'a mut Settings) -> Self {
        Self { controls, settings }
    }
}

impl Widget for TabRootEdit<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.add(crate::ui::Keyboard::new(
                self.controls.lead.iter().collect(),
                self.settings,
                KeyboardEditMode::RootNote,
            ));
            ui.separator();
            octave_selector(ui, "Lead Octave", &mut self.settings.octave);
            octave_selector(ui, "Guitar Octave", &mut self.settings.guitar_octave);
            ui.separator();
            ui.vertical_centered_justified(|ui| {
                ui.label(RichText::new("Note").size(30.0));
            });
            ui.horizontal_wrapped(|ui| {
                for pitch in 0..=11 {
                    let pitch = Pitch::from_byte(pitch);
                    ui.selectable_value(
                        &mut self.settings.pitch,
                        pitch,
                        RichText::new(format!("  {}  ", pitch)).size(40.0),
                    );
                }
            });
        })
        .response
    }
}

fn octave_selector(ui: &mut egui::Ui, name: &str, octave_value: &mut i8) {
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new(name).size(40.0));
        for octave in 0..=4 {
            ui.selectable_value(
                octave_value,
                octave,
                RichText::new(format!("  {}  ", octave)).size(40.0),
            );
        }
    });
}
