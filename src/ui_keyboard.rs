use egui::{Color32, Widget};
use music_note::{midi::MidiNote, Pitch};

/// Display a keyboard with a floating point note
pub struct Keyboard {
    /// Currently played midi note
    pub note: f32,
}

impl Keyboard {
    pub fn new(note: f32) -> Self {
        Self { note }
    }

    fn draw_key(&self, ui: &mut egui::Ui, key_dimension: &egui::Vec2, note: &MidiNote) {
        let note_byte = note.into_byte();

        let note_float = note_byte as f32;
        let note_distance = (note_float - self.note).abs().clamp(0.0, 1.0);

        let color = Color32::from_gray(((1.0 - note_distance) * 255.0) as u8);
        let (rect, mut response) = ui.allocate_exact_size(*key_dimension, egui::Sense::click());
        if response.clicked() {
            response.mark_changed();
        }
        let visuals = ui.style().interact_selectable(&response, true);
        if ui.is_rect_visible(rect) {
            // All coordinates are in absolute screen coordinates so we use `rect` to place the elements.
            let rect = rect.expand(visuals.expansion);
            let radius = 0.1 * rect.height();
            ui.painter().rect(rect, radius, color, visuals.bg_stroke);
        }
    }
}

impl Widget for Keyboard {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let key_dimension = egui::vec2(
            ui.spacing().interact_size.x / 4.0,
            ui.spacing().interact_size.y,
        );
        let keyboard_size = egui::vec2(
            (key_dimension.x + 1.0) * 70.0,
            (key_dimension.y + 1.0) * 2.0,
        );
        ui.spacing_mut().item_spacing.x = 1.0;
        ui.allocate_ui(keyboard_size, |ui| {
            // Black keys
            ui.horizontal(|ui| {
                // Half key offset
                ui.allocate_exact_size(
                    egui::vec2(key_dimension.x / 2.0, key_dimension.y),
                    egui::Sense::focusable_noninteractive(),
                );

                for byte in 0..=127 {
                    let note = MidiNote::from_byte(byte);
                    match note.pitch() {
                        Pitch::B | Pitch::E => {
                            // Space between keys
                            ui.allocate_exact_size(
                                key_dimension,
                                egui::Sense::focusable_noninteractive(),
                            );
                        }
                        Pitch::CSharp
                        | Pitch::DSharp
                        | Pitch::FSharp
                        | Pitch::GSharp
                        | Pitch::ASharp => {
                            self.draw_key(ui, &key_dimension, &note);
                        }
                        _ => {}
                    }
                }
            });

            // White keys
            ui.horizontal(|ui| {
                for byte in 0..=127 {
                    let note = MidiNote::from_byte(byte);
                    match note.pitch() {
                        Pitch::C
                        | Pitch::D
                        | Pitch::E
                        | Pitch::F
                        | Pitch::G
                        | Pitch::A
                        | Pitch::B => {
                            self.draw_key(ui, &key_dimension, &note);
                        }
                        _ => {}
                    }
                }
            });
        })
        .response
    }
}
