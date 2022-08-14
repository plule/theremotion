use std::ops::RangeInclusive;

use staff::{
    midi::{MidiNote, Octave},
    scale::ScaleIntervals,
    set::Set,
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
    pub scale: ScaleType,

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

    pub fn scale(&self) -> Set<Interval, u16> {
        match &self.scale {
            ScaleType::Chromatic => Set::from_iter((0..=11).map(Interval::new)), // (0..=11).map(Interval::new).collect(),
            ScaleType::Major => ScaleIntervals::major(),
            ScaleType::NaturalMinor => ScaleIntervals::natural_minor(),
            ScaleType::MelodicMinor => ScaleIntervals::melodic_minor(),
            ScaleType::HarmonicMinor => ScaleIntervals::harmonic_minor(),
            ScaleType::Blues => ScaleIntervals::blues(),
            ScaleType::Custom(intervals) => Set::from_iter(intervals.clone().into_iter()),
        }
    }

    /// List all the existing notes of the current
    pub fn scale_notes(&self) -> Vec<MidiNote> {
        let pitches: Vec<Pitch> = self
            .scale()
            .map(|interval| (self.root_note + interval).pitch())
            .collect();

        self.note_range()
            .map(MidiNote::from_byte)
            .filter(|n| pitches.contains(&n.pitch()))
            .collect()
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            root_note: MidiNote::new(Pitch::C, Octave::ONE),
            octave_range: 3,
            scale: ScaleType::Chromatic,
            drone: None,
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
    Custom(Set<Interval, u16>),
}
