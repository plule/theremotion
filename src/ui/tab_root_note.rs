use egui::{RichText, Widget};
use staff::Pitch;

use crate::settings::Preset;

use super::KeyboardEditMode;

pub struct TabRootNote<'a> {
    preset: &'a mut Preset,
    lead_chord_notes: &'a [f32; 4],
    lead_chord_volumes: &'a [f32; 4],
}

impl<'a> TabRootNote<'a> {
    pub fn new(
        preset: &'a mut Preset,
        lead_chord_notes: &'a [f32; 4],
        lead_chord_volumes: &'a [f32; 4],
    ) -> Self {
        Self {
            preset,
            lead_chord_notes,
            lead_chord_volumes,
        }
    }
}

impl Widget for TabRootNote<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.add(crate::ui::Keyboard::new(
                self.lead_chord_notes,
                self.lead_chord_volumes,
                self.preset,
                KeyboardEditMode::RootNote,
            ));
            ui.separator();
            octave_selector(ui, "Lead Octave", &mut self.preset.octave);
            octave_selector(ui, "Guitar Octave", &mut self.preset.guitar_octave);
            ui.separator();
            ui.vertical_centered_justified(|ui| {
                ui.label(RichText::new("Note").size(30.0));
            });
            ui.horizontal_wrapped(|ui| {
                for pitch in 0..=11 {
                    let pitch = Pitch::from_byte(pitch);
                    ui.selectable_value(
                        &mut self.preset.pitch,
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
