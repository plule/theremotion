use std::ops::{Add, Sub};

use derive_more::{Add, From, Into, Mul};
use serde::{Deserialize, Serialize};
use staff::{
    midi::{MidiNote, Octave},
    Interval,
};

/// Floating midi note, from 0 to 127
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Into)]
pub struct MidiNoteF(pub f32);

/// Interval between to floating notes
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Into, Add, Mul)]
pub struct IntervalF(pub f32);

/// Volume from 0 to 1
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, From, Into)]
pub struct Volume(pub f32);

/// Octave difference
#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Debug)]
#[serde(transparent)]
pub struct OctaveInterval(pub u8);

impl Add<IntervalF> for MidiNoteF {
    type Output = Self;

    fn add(self, rhs: IntervalF) -> Self::Output {
        Self(self.note() + rhs.semitones())
    }
}

impl Sub<IntervalF> for MidiNoteF {
    type Output = Self;

    fn sub(self, rhs: IntervalF) -> Self::Output {
        Self(self.note() - rhs.semitones())
    }
}

impl Sub for MidiNoteF {
    type Output = IntervalF;

    fn sub(self, rhs: Self) -> Self::Output {
        IntervalF(self.note() - rhs.note())
    }
}

impl IntervalF {
    /// Returns the semitones of this [`IntervalF`].
    pub fn semitones(&self) -> f32 {
        self.0
    }
    /// Absolute interval.
    pub fn abs(&self) -> Self {
        Self(self.semitones().abs())
    }
    /// Clamp this interval between a minimal and a maximal value
    pub fn clamp(&self, min: IntervalF, max: IntervalF) -> Self {
        Self(self.semitones().clamp(min.semitones(), max.semitones()))
    }
}

impl From<Interval> for IntervalF {
    fn from(value: Interval) -> Self {
        Self(value.semitones() as f32)
    }
}

impl Sub for IntervalF {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.semitones() - rhs.semitones())
    }
}

impl Volume {
    pub fn clamped(&self) -> Self {
        Self(self.0.min(1.0).max(0.0))
    }
}

impl OctaveInterval {
    /// Initialize from two octaves
    pub fn from_octaves(from: Octave, to: Octave) -> Self {
        Self((to.into_i8() - from.into_i8()) as u8)
    }

    /// Returns the octaves of this [`OctaveInterval`].
    pub fn octaves(&self) -> u8 {
        self.0
    }

    /// Returns the semitones of this [`OctaveInterval`].
    pub fn semitones(&self) -> u8 {
        self.0 * 12
    }
}

impl From<OctaveInterval> for Interval {
    fn from(value: OctaveInterval) -> Self {
        Interval::new(value.semitones())
    }
}

impl MidiNoteF {
    /// Returns the note of this [`MidiNoteF`].
    pub fn note(&self) -> f32 {
        self.0
    }

    /// Closest rounded value.
    pub fn round(&self) -> Self {
        Self(self.note().round())
    }

    /// Clamp this note between two notes
    pub fn clamp(&self, min: MidiNoteF, max: MidiNoteF) -> Self {
        assert!(min <= max);
        Self(self.note().clamp(min.note(), max.note()))
    }
}

impl From<MidiNote> for MidiNoteF {
    fn from(value: MidiNote) -> Self {
        Self(value.into_byte() as f32)
    }
}
