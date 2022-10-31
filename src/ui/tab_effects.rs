use egui::{Slider, Widget};

use crate::{controls, settings::Settings};

use super::{FromControl, NamedGroup};

pub struct TabEffects<'a> {
    controls: &'a mut controls::Controls,
    settings: &'a mut Settings,
}

impl<'a> TabEffects<'a> {
    pub fn new(controls: &'a mut controls::Controls, settings: &'a mut Settings) -> Self {
        Self { controls, settings }
    }
}

impl Widget for TabEffects<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let Self { controls, settings } = self;
        ui.horizontal(|ui| {
            ui.named_group("Echo", |ui| {
                ui.add(
                    Slider::from_control(&controls.echo_mix, &mut settings.echo_mix).text("Amount"),
                );
                ui.add(
                    Slider::from_control(&controls.echo_duration, &mut settings.echo_duration)
                        .logarithmic(true)
                        .step_by(0.0)
                        .text("Duration"),
                );
                ui.add(
                    Slider::from_control(&controls.echo_feedback, &mut settings.echo_feedback)
                        .text("Feedback"),
                );
            });

            ui.named_group("Reverb", |ui| {
                ui.add(
                    Slider::from_control(&controls.reverb_mix, &mut settings.reverb_mix)
                        .text("Amount"),
                );
                ui.add(
                    Slider::from_control(&controls.reverb_time, &mut settings.reverb_time)
                        .text("Time"),
                );
                ui.add(
                    Slider::from_control(&controls.reverb_damp, &mut settings.reverb_damp)
                        .text("Damp"),
                );
                ui.add(
                    Slider::from_control(&controls.reverb_size, &mut settings.reverb_size)
                        .text("Size"),
                );
            });
        })
        .response
    }
}
