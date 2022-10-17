use std::ops::RangeInclusive;

use staff::{midi::MidiNote, scale::ScaleIntervals, Interval};

/// Find the two scale neighbours of a floating note in a scale.
/// Return the index of the first neighbour, or none if the note is not strictly in the scale.
/// Assumes that the scale is ordered
fn neighbours(note: f32, scale: &Vec<MidiNote>) -> Option<usize> {
    if scale.is_empty() {
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

/// Find the neighbour in the scale
pub fn closest_in_scale(note: f32, scale: &[MidiNote]) -> usize {
    let mut distance = std::f32::INFINITY;
    let mut index = 0;

    for (scale_index, scale_note) in scale.iter().enumerate() {
        let scale_note = scale_note.into_byte() as f32;
        let scale_distance = (scale_note - note).abs();
        if scale_distance < distance {
            distance = scale_distance;
            index = scale_index;
        }
    }

    index
}

/// From a floating note, a scale and a degree, get the approximate floating note
///
/// Ex, in C major, C degree 2 will E. Note in between the scale will give note in between
/// the intervals
pub fn auto_chord(note: f32, scale: &Vec<MidiNote>, degree: isize) -> Option<f32> {
    // Find the two closest neighbours belonging to the scale
    let neighbours_index = neighbours(note, scale)?;

    // Get the corresponding chord notes based on the given degree
    let chord1_note = scale.get((neighbours_index as isize + degree) as usize)?;
    let chord1 = chord1_note.into_byte() as f32;
    let chord2_note = scale.get((neighbours_index as isize + 1 + degree) as usize)?;
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

/// Smooth step function loosely "sticking" the value to 0 or 1
/// Assumes that value is between 0 and 1
/// https://en.wikipedia.org/wiki/Smoothstep
fn smoothstep(interval: &RangeInclusive<f32>, x: f32) -> f32 {
    let x = (x - interval.start()) / (interval.end() - interval.start());
    x * x * (3.0 - 2.0 * x)
}

/// Roughly tune a floating note into a given scale
pub fn autotune(value: f32, amount: usize, scale: Vec<MidiNote>) -> f32 {
    let scale: Vec<_> = scale
        .windows(2)
        .map(|w| (w[0].into_byte() as f32)..=(w[1].into_byte() as f32))
        .collect();

    if let Some(interval) = scale.iter().find(|interval| interval.contains(&value)) {
        let mut value = value;

        for _ in 0..amount {
            let smooth = smoothstep(interval, value);
            value = interval.start() + smooth * (interval.end() - interval.start());
        }
        return value;
    }
    value
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
        #[case] degree: isize,
        #[case] expected: f32,
    ) {
        let scale = build_scale(root_note, scale);
        assert_eq!(
            Some(expected),
            auto_chord(note.into_byte().into(), &scale, degree)
        );
    }
}
