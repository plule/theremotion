use music_note::{midi::MidiNote, Pitch, Scale};

/// Application settings
#[derive(Clone)]
pub struct Settings {
    /// How much note sticks to just notes
    /// 0 means no autotune.
    pub autotune_strength: usize,

    /// Root note of the autotune
    pub root_note: Pitch,

    /// Scale of the autotune
    pub scale: ScaleType,
}

impl Settings {
    pub fn octave_notes(&self) -> Vec<Pitch> {
        match self.scale {
            ScaleType::Chromatic => (0..=11).map(Pitch::from_byte).collect(),
            ScaleType::Major => Scale::major(self.root_note).collect(),
            ScaleType::Minor => Scale::harmonic_minor(self.root_note).collect(),
            ScaleType::Blues => Scale::blues(self.root_note).collect(),
        }
    }

    /// List all the existing notes of the current
    pub fn scale_notes(&self) -> Vec<MidiNote> {
        let scale = self.octave_notes();
        (00..=127)
            .map(MidiNote::from_byte)
            .filter(|n| scale.contains(&n.pitch()))
            .collect()
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            autotune_strength: 2,
            root_note: Pitch::C,
            scale: ScaleType::Chromatic,
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum ScaleType {
    Chromatic,
    Major,
    Minor,
    Blues,
}
