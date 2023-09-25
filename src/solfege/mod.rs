mod interval;
mod midi_note;
mod octave_interval;
mod scale_windows;
mod scales;
use std::ops::{Add, Sub};

use derive_more::{Add, Into, Mul, Sub};
pub use interval::*;
pub use midi_note::*;
pub use octave_interval::*;
pub use scale_windows::*;
pub use scales::*;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Add, Sub, Mul, Into,
)]
pub struct Volume(pub f32);

impl Volume {
    pub fn clamped(&self) -> Self {
        Self(self.0.min(1.0).max(0.0))
    }
}

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
