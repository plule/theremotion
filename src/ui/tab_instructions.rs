use egui::Widget;

use crate::{controls, settings::Settings};

use super::KeyboardEditMode;

pub struct TabInstructions<'a> {
    controls: &'a mut controls::Controls,
    settings: &'a mut Settings,
}

impl<'a> TabInstructions<'a> {
    pub fn new(controls: &'a mut controls::Controls, settings: &'a mut Settings) -> Self {
        Self { controls, settings }
    }
}

impl Widget for TabInstructions<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let Self { controls, settings } = self;
        ui.vertical(|ui| {
            ui.add(crate::ui::Keyboard::new(
                controls.lead.iter().collect(),
                settings,
                KeyboardEditMode::None,
            ));
            ui.separator();
            ui.label("👐 Theremotion is a synthesizer controlled by your hands.");
            ui.label("👉 Move up and down your right hand to control the volume.");
            ui.label("👈 Move up and down your left hand to control the pitch.");
            ui.label("👋 Move your hands on the horizontal plane to adapt the timbre.");
            ui.label("👌 Pinch with your left hand to stick on a scale.");
            ui.label("🎸 Pinch with your right hand, and rotate it to play guitar.");
            ui.label("✌ ☝ Retract your pinky and ring fingers of your left hand, then play with the other fingers to play scales");
        }).response
    }
}
