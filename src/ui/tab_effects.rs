use egui::{Slider, Widget};

use crate::{controls, settings::Preset};

use super::{FromControl, NamedGroup};

pub struct TabEffects<'a> {
    controls: &'a mut controls::Controls,
    preset: &'a mut Preset,
}

impl<'a> TabEffects<'a> {
    pub fn new(controls: &'a mut controls::Controls, preset: &'a mut Preset) -> Self {
        Self { controls, preset }
    }
}

impl Widget for TabEffects<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let Self { controls, preset } = self;
        ui.horizontal(|ui| {
            ui.named_group("Echo", |ui| {
                ui.add(
                    Slider::from_control(&controls.echo_mix, &mut preset.fx.echo.mix)
                        .text("Amount"),
                );
                ui.add(
                    Slider::from_control(&controls.echo_duration, &mut preset.fx.echo.duration)
                        .logarithmic(true)
                        .step_by(0.0)
                        .text("Duration"),
                );
                ui.add(
                    Slider::from_control(&controls.echo_feedback, &mut preset.fx.echo.feedback)
                        .text("Feedback"),
                );
            });

            ui.named_group("Reverb", |ui| {
                ui.add(
                    Slider::from_control(&controls.reverb_mix, &mut preset.fx.reverb.mix)
                        .text("Amount"),
                );
                ui.add(
                    Slider::from_control(&controls.reverb_time, &mut preset.fx.reverb.time)
                        .text("Time"),
                );
                ui.add(
                    Slider::from_control(&controls.reverb_damp, &mut preset.fx.reverb.damp)
                        .text("Damp"),
                );
                ui.add(
                    Slider::from_control(&controls.reverb_size, &mut preset.fx.reverb.size)
                        .text("Size"),
                );
            });
        })
        .response
    }
}
