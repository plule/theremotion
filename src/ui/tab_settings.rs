use egui::Slider;

use crate::{controls, settings::Settings};

use super::FromControl;

pub fn tab_settings(ui: &mut egui::Ui, controls: &mut controls::Controls, settings: &mut Settings) {
    ui.group(|ui| {
        ui.label("Echo");
        ui.add(Slider::from_control(&controls.echo_mix, &mut settings.echo_mix).text("Mix"));
        ui.add(
            Slider::from_control(&controls.echo_duration, &mut settings.echo_duration)
                .logarithmic(true)
                .text("Duration"),
        );
        ui.add(
            Slider::from_control(&controls.echo_feedback, &mut settings.echo_feedback)
                .text("Feedback"),
        );
    });
}
