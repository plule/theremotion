use egui::Widget;

pub struct TabInstructions {}

impl TabInstructions {
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget for TabInstructions {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.label("👐 Theremotion is a synthesizer controlled by your hands.");
            ui.label("👉 Move up and down your right hand to control the volume.");
            ui.label("👈 Move up and down your left hand to control the pitch.");
            ui.label("👋 Move your right hand on the horizontal plane to control the filter.");
            ui.label("👋 Advance left hand on the horizontal plane to play a chord.");
            ui.label("👌 Pinch with your left hand to stick on a scale.");
            ui.label("🎸 Pinch with your right hand, and rotate it to play guitar.");
        })
        .response
    }
}
