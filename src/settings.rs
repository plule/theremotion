use std::ops::RangeInclusive;

use staff::{
    midi::{MidiNote, Octave},
    scale::ScaleIntervals,
    Interval, Pitch,
};

/// Application settings
#[derive(Clone)]
pub struct Settings {
    /// Root note of the keyboard
    pub root_note: MidiNote,

    /// Number of octave to display
    pub octave_range: u8,

    /// Scale of the autotune
    pub scale: ScaleIntervals,

    /// Current drone
    pub drone: Option<MidiNote>,
}

impl Settings {
    pub fn note_range(&self) -> RangeInclusive<u8> {
        self.root_note.into_byte()
            ..=(self.root_note + Interval::new(self.octave_range * 12)).into_byte()
    }

    pub fn note_range_f(&self) -> RangeInclusive<f32> {
        let range = self.note_range();
        (*range.start() as f32)..=(*range.end() as f32)
    }

    /// List all the existing notes of the current
    pub fn scale_notes(&self) -> Vec<MidiNote> {
        self.note_range()
            .map(MidiNote::from_byte)
            .filter(|note| {
                let interval = Interval::new((*note - self.root_note).semitones() % 12);
                self.scale.contains(interval)
            })
            .collect()
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            root_note: MidiNote::new(Pitch::C, Octave::THREE),
            octave_range: 3,
            scale: ScaleIntervals::all(),
            drone: None,
        }
    }
}
