use egui::{Slider, Widget};

use crate::settings::Settings;

pub struct TabMix<'a> {
    settings: &'a mut Settings,
}

impl<'a> TabMix<'a> {
    pub fn new(settings: &'a mut Settings) -> Self {
        Self { settings }
    }
}

impl Widget for TabMix<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let Self { settings } = self;
        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label("Instruments");
                    ui.horizontal(|ui| {
                        mix_slider(ui, "Lead", &mut settings.lead_volume);
                        mix_slider(ui, "Guitar", &mut settings.guitar_volume);
                        mix_slider(ui, "Drone", &mut settings.drone_volume);
                    });
                })
            });
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label(""); // align
                    ui.horizontal(|ui| {
                        mix_slider(ui, "Master", &mut settings.master_volume);
                    });
                });
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
            .text(format!("{} {}", icon, name)),
    );
}
