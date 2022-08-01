use std::ops::RangeInclusive;

use music_note::{
    midi::{MidiNote, Octave},
    Interval, Pitch, Scale,
};

/// Application settings
#[derive(Clone)]
pub struct Settings {
    /// How much note sticks to just notes
    /// 0 means no autotune.
    pub autotune_strength: usize,

    /// Root note of the keyboard
    pub root_note: MidiNote,

    /// Number of octave to display
    pub octave_range: u8,

    /// Scale of the autotune
    pub scale: ScaleType,
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

    pub fn octave_notes(&self) -> Vec<Pitch> {
        match &self.scale {
            ScaleType::Chromatic => (0..=11).map(Pitch::from_byte).collect(),
            ScaleType::Major => Scale::major(self.root_note.pitch()).collect(),
            ScaleType::NaturalMinor => Scale::natural_minor(self.root_note.pitch()).collect(),
            ScaleType::MelodicMinor => Scale::melodic_minor(self.root_note.pitch()).collect(),
            ScaleType::HarmonicMinor => Scale::harmonic_minor(self.root_note.pitch()).collect(),
            ScaleType::Blues => Scale::blues(self.root_note.pitch()).collect(),
            ScaleType::Custom(pitches) => pitches.clone(),
        }
    }

    /// List all the existing notes of the current
    pub fn scale_notes(&self) -> Vec<MidiNote> {
        let scale = self.octave_notes();

        self.note_range()
            .map(MidiNote::from_byte)
            .filter(|n| scale.contains(&n.pitch()))
            .collect()
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            autotune_strength: 0,
            root_note: MidiNote::new(Pitch::C, Octave::ONE),
            octave_range: 3,
            scale: ScaleType::Chromatic,
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum ScaleType {
    Chromatic,
    Major,
    NaturalMinor,
    HarmonicMinor,
    MelodicMinor,
    Blues,
    Custom(Vec<Pitch>),
}
