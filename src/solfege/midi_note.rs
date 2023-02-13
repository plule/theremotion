use staff::midi::MidiNote;

/// Floating midi note, from 0 to 127
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MidiNoteF(f32);

impl MidiNoteF {
    /// Creates a new [`MidiNoteF`].
    pub fn new(note: f32) -> Self {
        Self(note)
    }
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

impl From<MidiNoteF> for f32 {
    fn from(value: MidiNoteF) -> Self {
        value.note()
    }
}

impl From<MidiNoteF> for f64 {
    fn from(value: MidiNoteF) -> Self {
        value.note().into()
    }
}
