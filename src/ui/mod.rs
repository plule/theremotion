mod app;
mod keyboard;
mod tab_effects;
mod tab_mix;
mod tab_play;
mod tab_presets;
mod tab_root_note;
mod tab_scale;
mod tab_settings;
pub use app::*;
use egui::Slider;
pub use keyboard::*;
pub use tab_effects::*;
pub use tab_mix::*;
pub use tab_play::*;
pub use tab_presets::*;
pub use tab_root_note::*;
pub use tab_scale::*;
pub use tab_settings::*;

use crate::controls;

/// Message to update externally the UI
pub enum UiUpdate {
    // Display an error message
    Error(String),
    // Remove the error message
    ErrorReset,
    // Lead instrument volume (0-1)
    LeadVolume(f32),
    // Lead chord notes (floating midi)
    LeadChordNotes([f32; 4]),
    // Lead chord volumes (0-1)
    LeadChordVolumes([f32; 4]),
    // Floating number of chord notes (2.5 is 2 chord notes and the next half volume)
    ChordsNumber(f32),
    // Lead instrument note, without autotune
    RawNote(f32),
    // Filter cutoff and resonance.
    // Filter: -1 to 1, resonance: 0 to 1
    Filter(f32, f32),
    // Amount of autotune
    AutotuneAmount(usize),
    // Visible hands
    HasHands(bool, bool),
    // Pitch position, in semitones relative to the antenna
    PitchXY(f32, f32),
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
