use staff::{scale::ScaleIntervals, Interval};

pub trait MoreScales {
    fn freygish() -> Self;

    fn altered_dorian() -> Self;
}

impl MoreScales for ScaleIntervals {
    /// https://en.wikipedia.org/wiki/Phrygian_dominant_scale
    fn freygish() -> Self {
        ScaleIntervals::from_iter([
            Interval::UNISON,
            Interval::MINOR_SECOND,
            Interval::MAJOR_THIRD,
            Interval::PERFECT_FOURTH,
            Interval::PERFECT_FIFTH,
            Interval::MINOR_SIXTH,
            Interval::MINOR_SEVENTH,
        ])
    }

    /// https://en.wikipedia.org/wiki/Ukrainian_Dorian_scale
    fn altered_dorian() -> Self {
        Self::from_iter([
            Interval::UNISON,
            Interval::MAJOR_SECOND,
            Interval::MINOR_THIRD,
            Interval::TRITONE,
            Interval::PERFECT_FIFTH,
            Interval::MAJOR_SIXTH,
            Interval::MINOR_SEVENTH,
        ])
    }
}
