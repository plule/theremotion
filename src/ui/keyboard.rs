use egui::{Color32, Response, Widget};
use staff::{
    midi::{MidiNote, Octave},
    Interval, Pitch,
};

use crate::{controls::NoteControl, settings::Preset};

/// Display a keyboard with a floating point note
pub struct Keyboard<'a> {
    /// Currently played midi note
    pub notes: Vec<&'a NoteControl>,

    /// Settings preset
    pub preset: &'a mut Preset,

    /// Edit mode
    pub edit_mode: KeyboardEditMode,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum KeyboardEditMode {
    #[default]
    None,
    Drone,
    RootNote,
    Scale,
}

impl<'a> Keyboard<'a> {
    pub fn new(
        notes: Vec<&'a NoteControl>,
        settings: &'a mut Preset,
        edit_mode: KeyboardEditMode,
    ) -> Self {
        Self {
            notes,
            preset: settings,
            edit_mode,
        }
    }

    fn draw_key(&self, ui: &mut egui::Ui, key_dimension: &egui::Vec2, note: &MidiNote) -> Response {
        let scale = self.preset.restricted_scale();
        let note_byte = note.into_byte();

        let note_float = note_byte as f32;
        let mut red = 0;

        for played_note in &self.notes {
            let note_distance = (note_float - played_note.note.value).abs().clamp(0.0, 1.0);
            red = red.max(((1.0 - note_distance) * 255.0 * played_note.volume.value) as u8);
        }
        let green = if self
            .preset
            .drone
            .notes
            .iter()
            .any(|drone_note| drone_note.iter().any(|drone_note| note == drone_note))
        {
            255
        } else {
            0
        };
        let blue = if scale.contains(note) { 255 } else { 0 };

        let color = Color32::from_rgb(red, green, blue);
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

        response
    }

    fn add_key(&mut self, ui: &mut egui::Ui, key_dimension: egui::Vec2, note: MidiNote) {
        let response = self.draw_key(ui, &key_dimension, &note);

        if response.clicked() {
            match self.edit_mode {
                KeyboardEditMode::None => {}
                KeyboardEditMode::Drone => {
                    // Toggle the selected note
                    let drone_notes = &mut self.preset.drone.notes;
                    if let Some(existing_drone) = drone_notes
                        .iter_mut()
                        .find(|n| n.iter().any(|n| *n == note))
                    {
                        *existing_drone = None;
                    } else if let Some(empty_slot) = drone_notes.iter_mut().find(|n| n.is_none()) {
                        *empty_slot = Some(note);
                    }
                }
                KeyboardEditMode::RootNote => {
                    self.preset.pitch = note.pitch();
                }
                KeyboardEditMode::Scale => {
                    let interval = note - self.preset.root_note();
                    let interval = Interval::new(interval.semitones() % 12);
                    if self.preset.scale.contains(interval) {
                        self.preset.scale.remove(interval);
                    } else {
                        self.preset.scale.push(interval);
                    }
                }
            }
        }
    }
}

impl<'a> Widget for Keyboard<'a> {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        let start_octave = Octave::new_unchecked(self.preset.octave.clamp(-1, 8));
        let displayed_octave_count = self.preset.octave_range + 1;

        let start_note = MidiNote::new(Pitch::C, start_octave).into_byte();
        let note_range = start_note..=start_note + 12 * displayed_octave_count;

        let key_dimension = egui::vec2(22.0, 40.0);
        let keyboard_size = egui::vec2(
            (key_dimension.x + 1.0) * displayed_octave_count as f32 * 7.0,
            (key_dimension.y + 1.0) * 2.0,
        );
        ui.spacing_mut().item_spacing.x = 1.0;
        ui.spacing_mut().item_spacing.y = 1.0;
        ui.spacing_mut().interact_size.x = 1.0;
        ui.spacing_mut().interact_size.y = 1.0;
        ui.allocate_ui(keyboard_size, |ui| {
            // Black keys
            ui.horizontal(|ui| {
                // Half key offset
                ui.allocate_exact_size(
                    egui::vec2(key_dimension.x / 2.0, key_dimension.y),
                    egui::Sense::focusable_noninteractive(),
                );

                for byte in note_range.clone() {
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
                            self.add_key(ui, key_dimension, note);
                        }
                        _ => {}
                    }
                }
            });

            // White keys
            ui.horizontal(|ui| {
                for byte in note_range.clone() {
                    let note = MidiNote::from_byte(byte);
                    match note.pitch() {
                        Pitch::C
                        | Pitch::D
                        | Pitch::E
                        | Pitch::F
                        | Pitch::G
                        | Pitch::A
                        | Pitch::B => {
                            self.add_key(ui, key_dimension, note);
                        }
                        _ => {}
                    }
                }
            });
        })
        .response
    }
}

impl<'a> Keyboard<'a> {}
