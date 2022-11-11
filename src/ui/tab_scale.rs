use egui::{ScrollArea, Widget};
use staff::scale::ScaleIntervals;

use crate::{controls, scales::MoreScales, settings::Preset};

use super::KeyboardEditMode;

pub struct TabScale<'a> {
    controls: &'a mut controls::Controls,
    preset: &'a mut Preset,
}

impl<'a> TabScale<'a> {
    pub fn new(controls: &'a mut controls::Controls, preset: &'a mut Preset) -> Self {
        Self { controls, preset }
    }
}

impl Widget for TabScale<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let Self { controls, preset } = self;
        ui.vertical(|ui| {
            ui.add(crate::ui::Keyboard::new(
                controls.lead.iter().collect(),
                preset,
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
                        ui.selectable_value(&mut preset.scale, scale, format!("🎼 {}", name));
                        ui.add_space(ui.available_width());
                    });
                }
            });
        })
        .response
    }
}
