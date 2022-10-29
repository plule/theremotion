use egui::Slider;

use crate::settings::Settings;

pub fn tab_mix(ui: &mut egui::Ui, settings: &mut Settings) {
    ui.horizontal(|ui| {
        ui.style_mut().spacing.slider_width = 300.0;
        ui.style_mut().spacing.button_padding = egui::vec2(32.0, 32.0);
        ui.style_mut().spacing.interact_size = egui::vec2(64.0, 64.0);
        let space = 40.0;
        ui.group(|ui| {
            ui.add_space(space);
            mix_slider(ui, "Lead", &mut settings.lead_volume);
            ui.add_space(space);
            mix_slider(ui, "Guitar", &mut settings.guitar_volume);
            ui.add_space(space);
            mix_slider(ui, "Drone", &mut settings.drone_volume);
            ui.add_space(space);
        });
        ui.group(|ui| {
            ui.add_space(space);
            mix_slider(ui, "Master", &mut settings.master_volume);
            ui.add_space(space);
        });
    });
}

fn mix_slider(ui: &mut egui::Ui, name: &str, value: &mut f32) {
    ui.vertical(|ui| {
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
    });
}
