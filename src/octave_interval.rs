use std::ops::Add;

use serde::{Deserialize, Serialize};
use staff::{midi::Octave, Interval};

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct OctaveInterval {
    octaves: u8,
}

impl OctaveInterval {
    pub fn new(octaves: u8) -> Self {
        Self { octaves }
    }

    pub fn from_octaves(from: Octave, to: Octave) -> Self {
        Self::new((to.into_i8() - from.into_i8()) as u8)
    }

    pub fn octaves(&self) -> u8 {
        self.octaves
    }

    pub fn semitones(&self) -> u8 {
        self.octaves * 12
    }
}

impl From<OctaveInterval> for Interval {
    fn from(value: OctaveInterval) -> Self {
        Interval::new(value.semitones())
    }
}

impl Add for OctaveInterval {
    type Output = Self;

    fn add(self, rhs: OctaveInterval) -> Self::Output {
        Self::new(self.octaves + rhs.octaves)
    }
}

impl Add<Interval> for OctaveInterval {
    type Output = Interval;

    fn add(self, rhs: Interval) -> Self::Output {
        rhs + self.into()
    }
}
