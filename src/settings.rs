use std::ops::RangeInclusive;

use serde::{Deserialize, Serialize};
use staff::{
    midi::{MidiNote, Octave},
    scale::ScaleIntervals,
    Interval, Pitch,
};

/// Application settings
#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Settings {
    /// Octave of the root note
    pub octave: i8,

    /// Octave of the guitar sound
    pub guitar_octave: i8,

    /// Pitch of the root note
    pub pitch: Pitch,

    /// Number of octave to display
    pub octave_range: u8,

    /// Scale of the autotune
    pub scale: ScaleIntervals,

    /// Current drone
    pub drone: Option<MidiNote>,

    /// fx
    pub echo_mix: f32,
    pub echo_duration: f32,
    pub echo_feedback: f32,

    /// Mix
    pub master_volume: f32,
    pub lead_volume: f32,
    pub guitar_volume: f32,
    pub drone_volume: f32,
}

impl Settings {
    pub fn try_read() -> Option<Self> {
        let f = std::fs::File::open("settings.yaml").ok()?;
        let settings: Settings = serde_yaml::from_reader(f).ok()?;
        Some(settings)
    }

    pub fn read() -> Self {
        Self::try_read().unwrap_or_default()
    }

    pub fn save(&self) {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open("settings.yaml")
            .ok()
            .unwrap();
        serde_yaml::to_writer(f, &self).unwrap();
    }

    pub fn root_note(&self) -> MidiNote {
        MidiNote::new(self.pitch, Octave::new_unchecked(self.octave.clamp(-1, 8)))
    }

    pub fn note_range(&self) -> RangeInclusive<u8> {
        self.root_note().into_byte()
            ..=(self.root_note() + Interval::new(self.octave_range * 12)).into_byte()
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
                let interval = Interval::new((*note - self.root_note()).semitones() % 12);
                self.scale.contains(interval)
            })
            .collect()
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            octave: 3,
            guitar_octave: 3,
            pitch: Pitch::C,
            octave_range: 3,
            scale: ScaleIntervals::all(),
            drone: None,
            echo_mix: 1.0,
            echo_duration: 0.3,
            echo_feedback: 0.3,
            master_volume: 1.0,
            lead_volume: 1.0,
            guitar_volume: 1.0,
            drone_volume: 1.0,
        }
    }
}
