use egui::{Color32, Response, Widget};
use staff::{midi::MidiNote, Interval, Pitch};

use crate::{controls::NoteControl, settings::Settings};

/// Display a keyboard with a floating point note
pub struct Keyboard<'a> {
    /// Currently played midi note
    pub notes: Vec<&'a NoteControl>,

    /// Settings
    pub settings: &'a mut Settings,

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
        settings: &'a mut Settings,
        edit_mode: KeyboardEditMode,
    ) -> Self {
        Self {
            notes,
            settings,
            edit_mode,
        }
    }

    fn draw_key(&self, ui: &mut egui::Ui, key_dimension: &egui::Vec2, note: &MidiNote) -> Response {
        let scale = self.settings.scale_notes();
        let note_byte = note.into_byte();

        let note_float = note_byte as f32;
        let mut red = 0;

        for played_note in &self.notes {
            let note_distance = (note_float - played_note.note.value).abs().clamp(0.0, 1.0);
            red = red.max(((1.0 - note_distance) * 255.0 * played_note.volume.value) as u8);
        }
        let green = if matches!(self.settings.drone, Some(drone) if drone == *note) {
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
                    self.settings.drone = if let Some(drone) = self.settings.drone {
                        if drone == note {
                            None
                        } else {
                            Some(note)
                        }
                    } else {
                        Some(note)
                    }
                }
                KeyboardEditMode::RootNote => {
                    self.settings.root_note =
                        MidiNote::new(note.pitch(), self.settings.root_note.octave());
                }
                KeyboardEditMode::Scale => {
                    let interval = note - self.settings.root_note;
                    let interval = Interval::new(interval.semitones() % 12);
                    if self.settings.scale.contains(interval) {
                        self.settings.scale.remove(interval);
                    } else {
                        self.settings.scale.push(interval);
                    }
                }
            }
        }
    }
}

impl<'a> Widget for Keyboard<'a> {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        let start_octave = self.settings.root_note.octave();
        let displayed_octave_count = self.settings.octave_range + 1;

        let start_note = MidiNote::new(Pitch::C, start_octave).into_byte();
        let note_range = start_note..=start_note + 12 * displayed_octave_count;

        let key_dimension = egui::vec2(22.0, 40.0);
        let keyboard_size = egui::vec2(
            (key_dimension.x + 1.0) * displayed_octave_count as f32 * 7.0,
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
