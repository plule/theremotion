mod app;
mod keyboard;
mod tab_effects;
mod tab_instructions;
mod tab_mix;
mod tab_play;
mod tab_presets;
mod tab_root_note;
mod tab_scale;
pub use app::*;
use egui::Slider;
pub use keyboard::*;
pub use tab_effects::*;
pub use tab_instructions::*;
pub use tab_mix::*;
pub use tab_play::*;
pub use tab_presets::*;
pub use tab_root_note::*;
pub use tab_scale::*;

use crate::controls;

/// Message to update externally the UI
pub enum UiUpdate {
    Error(String),
    ErrorReset,
    LeadVolume(f32),
    LeadChordNotes([f32; 4]),
    LeadChordVolumes([f32; 4]),
    RawNote(f32),
    Filter(f32, f32),
    AutotuneAmount(usize),
    HasHands((bool, bool)),
}

trait FromControl<'a> {
    fn from_control(control: &controls::Control, value: &'a mut f32) -> Self;
}

impl<'a> FromControl<'a> for Slider<'a> {
    fn from_control(control: &controls::Control, value: &'a mut f32) -> Slider<'a> {
        Slider::new(value, control.input.range.clone())
            .step_by(control.input.step.into())
            .vertical()
            .show_value(false)
    }
}

trait NamedGroup {
    fn named_group(&mut self, name: &str, add_contents: impl FnOnce(&mut egui::Ui));
}

impl NamedGroup for egui::Ui {
    fn named_group(&mut self, name: &str, add_contents: impl FnOnce(&mut egui::Ui)) {
        self.group(|ui| {
            ui.vertical(|ui| {
                ui.label(name);
                ui.horizontal(add_contents);
            });
        });
    }
}
