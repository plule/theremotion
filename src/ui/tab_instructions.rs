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
            ui.label("👋 Move your hands on the horizontal plane to adapt the timbre.");
            ui.label("👌 Pinch with your left hand to stick on a scale.");
            ui.label("🎸 Pinch with your right hand, and rotate it to play guitar.");
            ui.label("✌ ☝ Retract your pinky and ring fingers of your left hand, then play with the other fingers to play scales");
        }).response
    }
}
