use egui::{Slider, Widget};

use crate::{controls, settings::Settings};

use super::FromControl;

pub struct TabSettings<'a> {
    controls: &'a mut controls::Controls,
    settings: &'a mut Settings,
}

impl<'a> TabSettings<'a> {
    pub fn new(controls: &'a mut controls::Controls, settings: &'a mut Settings) -> Self {
        Self { controls, settings }
    }
}

impl Widget for TabSettings<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let Self { controls, settings } = self;
        ui.vertical(|ui| {
            ui.group(|ui| {
                ui.label("Echo");
                ui.add(
                    Slider::from_control(&controls.echo_mix, &mut settings.echo_mix).text("Mix"),
                );
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
        })
        .response
    }
}
