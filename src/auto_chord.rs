use staff::{midi::MidiNote, scale::ScaleIntervals, Interval};

/// Find the two scale neighbours of a floating note in a scale.
/// Return the index of the first neighbour, or none if the note is not strictly in the scale.
/// Assumes that the scale is ordered
fn neighbours(note: f32, scale: &Vec<MidiNote>) -> Option<usize> {
    if scale.len() == 0 {
        return None;
    }

    if note < scale[0].into_byte().into() {
        return None;
    }

    scale.iter().enumerate().find_map(|(index, scale_note)| {
        if scale_note.into_byte() as f32 > note {
            Some(index - 1)
        } else {
            None
        }
    })
}

pub fn auto_chord(note: f32, scale: &Vec<MidiNote>, degree: usize) -> Option<f32> {
    // Find the two closest neighbours belonging to the scale
    let neighbours_index = neighbours(note, &scale)?;

    // Get the corresponding chord notes based on the given degree
    let chord1_note = scale.get(neighbours_index + degree)?;
    let chord1 = chord1_note.into_byte() as f32;
    let chord2_note = scale.get(neighbours_index + 1 + degree)?;
    let chord2 = chord2_note.into_byte() as f32;

    // Find the distance between the note and the two scale neighbours
    let dist1 = note - scale[neighbours_index].into_byte() as f32;
    let dist2 = scale[neighbours_index + 1].into_byte() as f32 - note;
    let weight = dist1 + dist2;

    // Inverted weighted result (0 is the best match)
    Some((chord1 * dist2 + chord2 * dist1) / weight)
}

pub fn build_scale(root_note: MidiNote, scale: ScaleIntervals) -> Vec<MidiNote> {
    (0..=127)
        .map(MidiNote::from_byte)
        .filter(|note| {
            let interval = Interval::new((*note - root_note).semitones() % 12);
            scale.contains(interval)
        })
        .collect()
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
        let scale = vec![midi!(C, 2), midi!(D, 2), midi!(E, 2)];
        assert_eq!(expected, neighbours(note.into_byte().into(), &scale));
    }

    #[rstest]
    // Major tierce
    #[case(midi!(C, 2), midi!(C, 2), ScaleIntervals::major(), 2, midi!(E, 2).into_byte() as f32)]
    // Minor tierce
    #[case(midi!(D, 2), midi!(C, 2), ScaleIntervals::major(), 2, midi!(F, 2).into_byte() as f32)]
    // Not in the scale, chord is not an exact note (sliding)
    #[case(midi!(CSharp, 2), midi!(C, 2), ScaleIntervals::major(), 2, 40.5)]
    fn auto_chord_test(
        #[case] note: MidiNote,
        #[case] root_note: MidiNote,
        #[case] scale: ScaleIntervals,
        #[case] degree: usize,
        #[case] expected: f32,
    ) {
        let scale = build_scale(root_note, scale);
        assert_eq!(
            Some(expected),
            auto_chord(note.into_byte().into(), &scale, degree)
        );
    }
}
