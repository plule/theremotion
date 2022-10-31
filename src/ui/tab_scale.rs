use egui::Widget;
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
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().button_padding.x = 10.0;
                ui.spacing_mut().button_padding.y = 10.0;
                ui.selectable_value(&mut preset.scale, ScaleIntervals::all(), "🎼 Chromatic");
                ui.selectable_value(&mut preset.scale, ScaleIntervals::major(), "🎼 Major");
                ui.selectable_value(
                    &mut preset.scale,
                    ScaleIntervals::melodic_minor(),
                    "🎼 Melodic Minor",
                );
                ui.selectable_value(
                    &mut preset.scale,
                    ScaleIntervals::harmonic_minor(),
                    "🎼 Harmonic Minor",
                );
                ui.selectable_value(
                    &mut preset.scale,
                    ScaleIntervals::natural_minor(),
                    "🎼 Natural Minor",
                );
                ui.selectable_value(&mut preset.scale, ScaleIntervals::dorian(), "🎼 Dorian");
                ui.selectable_value(&mut preset.scale, ScaleIntervals::blues(), "🎼 Blues");
                ui.selectable_value(&mut preset.scale, ScaleIntervals::freygish(), "🎼 Freygish");
                ui.selectable_value(
                    &mut preset.scale,
                    ScaleIntervals::altered_dorian(),
                    "🎼 Altered Dorian",
                );
            });
        })
        .response
    }
}
