use std::ops::RangeInclusive;

use staff::midi::MidiNote;

/// Poor's man Step implementation
pub trait StepIter<T> {
    type ItemIterator: Iterator<Item = T>;
    fn step_iter(&self) -> Self::ItemIterator;
}

impl StepIter<MidiNote> for RangeInclusive<MidiNote> {
    type ItemIterator = Box<dyn Iterator<Item = MidiNote>>;

    fn step_iter(&self) -> Self::ItemIterator {
        Box::new(
            (self.start().into_byte()..=self.end().into_byte())
                .into_iter()
                .map(MidiNote::from_byte),
        )
    }
}
