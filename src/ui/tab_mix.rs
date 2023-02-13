use egui::{Slider, Widget};

use crate::settings::Preset;

use super::NamedGroup;

/// Content of the mixing tab
pub struct TabMix<'a> {
    preset: &'a mut Preset,
}

impl<'a> TabMix<'a> {
    /// Creates a new [`TabMix`].
    pub fn new(preset: &'a mut Preset) -> Self {
        Self { preset }
    }
}

impl Widget for TabMix<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let Self { preset } = self;
        ui.horizontal(|ui| {
            ui.named_group("Instruments", |ui| {
                mix_slider(ui, "Lead", &mut preset.mix.lead);
                mix_slider(ui, "Guitar", &mut preset.mix.guitar);
                mix_slider(ui, "Drone", &mut preset.mix.drone);
            });
            ui.named_group("", |ui| {
                mix_slider(ui, "Master", &mut preset.mix.master);
            });
        })
        .response
    }
}

fn mix_slider(ui: &mut egui::Ui, name: &str, value: &mut f32) {
    let icon = match &value {
        value if **value <= 0.0 => "🔇",
        value if **value <= 0.33 => "🔈",
        value if **value <= 0.66 => "🔉",
        _ => "🔊",
    };
    ui.add(
        Slider::new(value, 0.0..=1.0)
            .vertical()
            .min_decimals(2)
            .max_decimals(2)
            .step_by(0.01)
            .show_value(false)
            .text(format!("{icon} {name}")),
    );
}
