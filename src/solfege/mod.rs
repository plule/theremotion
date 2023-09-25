mod interval;
mod midi_note;
mod octave_interval;
mod scale_windows;
mod scales;
use std::ops::{Add, Sub};

pub use interval::*;
pub use midi_note::*;
pub use octave_interval::*;
pub use scale_windows::*;
pub use scales::*;

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
