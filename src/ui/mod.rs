mod app;
mod keyboard;
mod tab_instructions;
mod tab_mix;
mod tab_play;
mod tab_root_edit;
mod tab_scale_edit;
mod tab_settings;
pub use app::*;
use egui::Slider;
pub use keyboard::*;
pub use tab_instructions::*;
pub use tab_mix::*;
pub use tab_play::*;
pub use tab_root_edit::*;
pub use tab_scale_edit::*;
pub use tab_settings::*;

use crate::controls;

trait FromControl<'a> {
    fn from_control(control: &controls::Control, value: &'a mut f32) -> Self;
}

impl<'a> FromControl<'a> for Slider<'a> {
    fn from_control(control: &controls::Control, value: &'a mut f32) -> Slider<'a> {
        Slider::new(value, control.input.range.clone()).step_by(control.input.step.into())
    }
}
