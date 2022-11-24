use egui::{ScrollArea, Widget};
use staff::scale::ScaleIntervals;

use crate::{scales::MoreScales, settings::Preset};

use super::KeyboardEditMode;

pub struct TabScale<'a> {
    preset: &'a mut Preset,
    lead_chord_notes: &'a [f32; 4],
    lead_chord_volumes: &'a [f32; 4],
}

impl<'a> TabScale<'a> {
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

impl Widget for TabScale<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.add(crate::ui::Keyboard::new(
                self.lead_chord_notes,
                self.lead_chord_volumes,
                self.preset,
                KeyboardEditMode::Scale,
            ));
            ui.separator();
            ScrollArea::vertical().show(ui, |ui| {
                for (name, scale) in [
                    ("Chromatic", ScaleIntervals::all()),
                    ("Major", ScaleIntervals::major()),
                    ("Melodic Minor", ScaleIntervals::melodic_minor()),
                    ("Harmonic Minor", ScaleIntervals::harmonic_minor()),
                    ("Natural Minor", ScaleIntervals::natural_minor()),
                    ("Dorian", ScaleIntervals::dorian()),
                    ("Blues", ScaleIntervals::blues()),
                    ("Freygish", ScaleIntervals::freygish()),
                    ("Altered Dorian", ScaleIntervals::altered_dorian()),
                ] {
                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut self.preset.scale, scale, format!("🎼 {}", name));
                        ui.add_space(ui.available_width());
                    });
                }
            });
        })
        .response
    }
}
