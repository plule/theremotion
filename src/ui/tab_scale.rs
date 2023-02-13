use egui::{ScrollArea, Widget};
use staff::scale::ScaleIntervals;

use crate::{
    scales::MoreScales,
    settings::{NamedScale, Settings},
    solfege::MidiNoteF,
};

use super::KeyboardEditMode;

pub struct TabScale<'a> {
    settings: &'a mut Settings,
    lead_chord_notes: &'a [MidiNoteF; 4],
    lead_chord_volumes: &'a [f32; 4],
    current_scale_name: &'a mut String,
}

impl<'a> TabScale<'a> {
    pub fn new(
        settings: &'a mut Settings,
        lead_chord_notes: &'a [MidiNoteF; 4],
        lead_chord_volumes: &'a [f32; 4],
        current_scale_name: &'a mut String,
    ) -> Self {
        Self {
            settings,
            lead_chord_notes,
            lead_chord_volumes,
            current_scale_name,
        }
    }
}

impl Widget for TabScale<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.add(crate::ui::Keyboard::new(
                self.lead_chord_notes,
                self.lead_chord_volumes,
                &mut self.settings.current_preset,
                KeyboardEditMode::Scale,
            ));
            ui.separator();
            ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.add(super::text_edit_singleline_tabtip(
                        self.current_scale_name,
                        self.settings.system.tabtip,
                    ));
                    if ui.button("💾").clicked() {
                        self.settings.scales.push(NamedScale::new(
                            self.current_scale_name.clone(),
                            self.settings.current_preset.scale,
                        ));
                    }
                });

                ui.separator();

                let mut delete = None;
                for (index, scale) in self.settings.scales.iter().enumerate() {
                    ui.horizontal(|ui| {
                        if ui.button("🗑").clicked() {
                            delete = Some(index);
                        }
                        if ui
                            .selectable_value(
                                &mut self.settings.current_preset.scale,
                                scale.scale,
                                scale.name.clone(),
                            )
                            .clicked()
                        {
                            *self.current_scale_name = scale.name.clone();
                        }
                        ui.add_space(ui.available_width());
                    });
                }

                if let Some(delete_index) = delete {
                    self.settings.scales.remove(delete_index);
                }

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
                        if ui
                            .selectable_value(
                                &mut self.settings.current_preset.scale,
                                scale,
                                format!("🎼 {name}"),
                            )
                            .clicked()
                        {
                            *self.current_scale_name = name.to_string();
                        }
                        ui.add_space(ui.available_width());
                    });
                }
            });
        })
        .response
    }
}
