use std::{cmp::Ordering, collections::HashSet, ops::RangeInclusive};

use itertools::Itertools;
use staff::{midi::MidiNote, scale::ScaleIntervals, Pitch};

use crate::StepIter;

use super::MidiNoteF;

/// Floating 2 by 2 window in a scale.
/// Useful for many blurry algorithms in theremotion
pub struct ScaleWindows {
    windows: Vec<(MidiNoteF, MidiNoteF)>,
}

impl ScaleWindows {
    /// Initialize from an ordered note window
    pub fn new(windows: Vec<(MidiNoteF, MidiNoteF)>) -> Self {
        Self { windows }
    }

    /// Initialize from a list of midi notes
    pub fn from_notes(notes: Vec<MidiNote>) -> Self {
        Self::new(
            notes
                .into_iter()
                .map(MidiNoteF::from)
                .tuple_windows()
                .collect(),
        )
    }

    /// Find the position of the two neighbours surrounding the given note.
    /// Returns None if the note is not in the scale
    fn neighbours_index(&self, note: MidiNoteF) -> Option<usize> {
        let scale: &[(MidiNoteF, MidiNoteF)] = &self.windows;
        let result = scale.binary_search_by(|(n1, n2)| {
            if note < *n1 {
                return Ordering::Greater;
            }
            if note >= *n2 {
                return Ordering::Less;
            }
            Ordering::Equal
        });
        result.ok()
    }

    /// Find the closest note in the scale of a given note
    pub fn closest_in_scale(&self, note: MidiNoteF) -> MidiNoteF {
        let scale: &[(MidiNoteF, MidiNoteF)] = &self.windows;
        if scale.is_empty() {
            return note.round();
        }
        let neighbours = self.neighbours_index(note);
        if let Some(neighbours) = neighbours {
            let neighbours = scale[neighbours];
            if (neighbours.0 - note).abs() < (neighbours.1 - note).abs() {
                return neighbours.0;
            }
            return neighbours.1;
        }
        if note < scale[0].0 {
            return scale[0].0;
        }
        scale.last().unwrap().1
    }

    /// Given a note and a degree, output the corresponding note, with the capability of
    /// sliding the input
    fn autodegree(&self, note: MidiNoteF, degree: isize) -> Option<MidiNoteF> {
        let scale: &[(MidiNoteF, MidiNoteF)] = &self.windows;
        // Find the two closest neighbours belonging to the scale
        let neighbours_index = self.neighbours_index(note)?;

        // Get the corresponding chord notes based on the given degree
        let (chord1, chord2) = scale.get((neighbours_index as isize + degree) as usize)?;

        // Find the distance between the note and the two scale neighbours
        let (neighbour1, neighbour2) = scale.get(neighbours_index)?;
        let dist1 = (note - *neighbour1).semitones();
        let dist2 = (*neighbour2 - note).semitones();
        let weight = dist1 + dist2;

        // Inverted weighted result (0 is the best match)
        // Todo: missing some operations here?
        Some(MidiNoteF(
            (chord1.note() * dist2 + chord2.note() * dist1) / weight,
        ))
    }

    /// Given a note and a list of degree, output a chord in the scale.
    /// The input note can slide between notes, creating a sliding chord.
    pub fn autochord<const N: usize>(
        &self,
        note: MidiNoteF,
        degrees: &[isize; N],
    ) -> [Option<MidiNoteF>; N] {
        degrees.map(|degree| self.autodegree(note, degree))
    }

    /// Configurable autotune of an input note
    pub fn autotune(&self, value: MidiNoteF, amount: usize) -> MidiNoteF {
        let scale: &[(MidiNoteF, MidiNoteF)] = &self.windows;
        if let Some((start, end)) = scale
            .iter()
            .find(|(start, end)| (start..=end).contains(&&value))
        {
            let mut value = value;
            for _ in 0..amount {
                let smooth = smoothstep(start.note(), end.note(), value.note());
                value = *start + (*end - *start) * smooth;
            }
            return value;
        }
        value
    }
}

/// Build the list of notes in a given scale and root note, restricted to a note interval
pub fn build_scale_notes(
    pitch: Pitch,
    scale: ScaleIntervals,
    restricted_to: RangeInclusive<MidiNote>,
) -> Vec<MidiNote> {
    let pitches: HashSet<u8> = scale
        .map(|interval| (pitch + interval).into_byte())
        .collect();
    restricted_to
        .step_iter()
        .filter(|note| pitches.contains(&note.pitch().into_byte()))
        .collect()
}

/// Smooth step function loosely "sticking" the value to 0 or 1
/// Assumes that value is between 0 and 1
/// <https://en.wikipedia.org/wiki/Smoothstep>
fn smoothstep(a: f32, b: f32, x: f32) -> f32 {
    let x = (x - a) / (b - a);
    x * x * (3.0 - 2.0 * x)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use staff::midi;

    use super::*;

    #[rstest]
    #[case(midi!(B, 1), None)]
    #[case(midi!(CSharp, 2), Some(0))]
    #[case(midi!(DSharp, 2), Some(1))]
    #[case(midi!(E, 2), None)]
    fn neighbours_test(#[case] note: MidiNote, #[case] expected: Option<usize>) {
        let scale = ScaleWindows::from_notes(vec![midi!(C, 2), midi!(D, 2), midi!(E, 2)]);
        assert_eq!(expected, scale.neighbours_index(note.into()));
    }

    #[rstest]
    #[case(5.0, 10.0)]
    #[case(10.1, 10.0)]
    #[case(11.1, 11.0)]
    #[case(25.0, 20.0)]
    fn closest_in_scale_test(#[case] note: f32, #[case] expected: f32) {
        let scale = ScaleWindows::new(vec![
            (MidiNoteF(10.0), MidiNoteF(11.0)),
            (MidiNoteF(11.0), MidiNoteF(15.0)),
            (MidiNoteF(15.0), MidiNoteF(20.0)),
        ]);
        assert_eq!(MidiNoteF(expected), scale.closest_in_scale(MidiNoteF(note)));
    }

    #[rstest]
    // Major tierce
    #[case(midi!(C, 2), midi!(C, 2), ScaleIntervals::major(), 2, midi!(E, 2).into())]
    // Minor tierce
    #[case(midi!(D, 2), midi!(C, 2), ScaleIntervals::major(), 2, midi!(F, 2).into())]
    // Not in the scale, chord is not an exact note (sliding)
    #[case(midi!(CSharp, 2), midi!(C, 2), ScaleIntervals::major(), 2, MidiNoteF(40.5))]
    fn auto_chord_test(
        #[case] note: MidiNote,
        #[case] root_note: MidiNote,
        #[case] scale: ScaleIntervals,
        #[case] degree: isize,
        #[case] expected: MidiNoteF,
    ) {
        let notes = build_scale_notes(
            root_note.pitch(),
            scale,
            MidiNote::from_byte(0)..=MidiNote::from_byte(127),
        );
        let scale = ScaleWindows::from_notes(notes);
        assert_eq!(Some(expected), scale.autodegree(note.into(), degree));
    }
}
