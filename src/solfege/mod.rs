mod interval;
mod midi_note;
use std::ops::{Add, Sub};

pub use interval::*;
pub use midi_note::*;

impl Add<IntervalF> for MidiNoteF {
    type Output = Self;

    fn add(self, rhs: IntervalF) -> Self::Output {
        Self::new(self.note() + rhs.semitones())
    }
}

impl Sub<IntervalF> for MidiNoteF {
    type Output = Self;

    fn sub(self, rhs: IntervalF) -> Self::Output {
        Self::new(self.note() - rhs.semitones())
    }
}

impl Sub for MidiNoteF {
    type Output = IntervalF;

    fn sub(self, rhs: Self) -> Self::Output {
        IntervalF::new(self.note() - rhs.note())
    }
}
