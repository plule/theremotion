use std::ops::RangeInclusive;

use serde::{Deserialize, Serialize};
use staff::{
    midi::{MidiNote, Octave},
    scale::ScaleIntervals,
    Interval, Pitch,
};

/// Application settings
#[derive(Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Settings {
    /// Current sound settings
    #[serde(default)]
    pub current_preset: Preset,

    /// Saved presets
    #[serde(default)]
    pub presets: Vec<Preset>,
}

/// Sound preset
#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Preset {
    /// Name of the preset
    pub name: String,

    /// Octave of the root note
    pub octave: i8,

    /// Octave of the guitar sound
    pub guitar_octave: i8,

    /// Pitch of the root note
    pub pitch: Pitch,

    /// Number of octave to display
    pub octave_range: u8,

    /// Scale of the autotune
    pub scale: ScaleIntervals,

    /// Current drone
    #[serde(default)]
    pub drone: DroneSettings,

    #[serde(default)]
    pub mix: MixSettings,

    #[serde(default)]
    pub fx: FxSettings,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct MixSettings {
    pub master: f32,
    pub lead: f32,
    pub guitar: f32,
    pub drone: f32,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FxSettings {
    #[serde(default)]
    pub echo: EchoSettings,
    #[serde(default)]
    pub reverb: ReverbSettings,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct EchoSettings {
    pub mix: f32,
    pub duration: f32,
    pub feedback: f32,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct ReverbSettings {
    pub mix: f32,
    pub time: f32,
    pub damp: f32,
    pub size: f32,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct DroneSettings {
    pub notes: [Option<MidiNote>; 4],
    pub detune: f32,
}

impl Default for DroneSettings {
    fn default() -> Self {
        Self {
            notes: Default::default(),
            detune: 0.1,
        }
    }
}

impl Settings {
    pub fn try_read() -> Option<Self> {
        let f = std::fs::File::open("settings.yaml").ok()?;
        let settings: Settings = serde_yaml::from_reader(f).ok()?;
        Some(settings)
    }

    pub fn read() -> Self {
        Self::try_read().unwrap_or_default()
    }

    pub fn save(&self) {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open("settings.yaml")
            .ok()
            .unwrap();
        serde_yaml::to_writer(f, &self).unwrap();
    }

    pub fn can_save_current_preset(&self) -> bool {
        !self.presets.contains(&self.current_preset)
    }

    pub fn save_current_preset(&mut self) {
        self.presets.push(self.current_preset.clone());
    }

    pub fn delete_preset(&mut self, name: &String) {
        self.presets.retain_mut(|preset| preset.name != *name);
    }
}

impl Preset {
    pub fn root_note(&self) -> MidiNote {
        MidiNote::new(self.pitch, Octave::new_unchecked(self.octave.clamp(-1, 8)))
    }

    pub fn note_range(&self) -> RangeInclusive<u8> {
        self.root_note().into_byte()
            ..=(self.root_note() + Interval::new(self.octave_range * 12)).into_byte()
    }

    pub fn note_range_f(&self) -> RangeInclusive<f32> {
        let range = self.note_range();
        (*range.start() as f32)..=(*range.end() as f32)
    }

    /// List all the existing notes of the current
    pub fn scale_notes(&self) -> Vec<MidiNote> {
        self.note_range()
            .map(MidiNote::from_byte)
            .filter(|note| {
                let interval = Interval::new((*note - self.root_note()).semitones() % 12);
                self.scale.contains(interval)
            })
            .collect()
    }
}

impl Default for Preset {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            octave: 3,
            guitar_octave: 3,
            pitch: Pitch::C,
            octave_range: 3,
            scale: ScaleIntervals::all(),
            drone: Default::default(),
            mix: Default::default(),
            fx: Default::default(),
        }
    }
}

impl Default for MixSettings {
    fn default() -> Self {
        Self {
            master: 1.0,
            lead: 1.0,
            guitar: 1.0,
            drone: 0.4,
        }
    }
}

impl Default for EchoSettings {
    fn default() -> Self {
        Self {
            mix: 1.0,
            duration: 0.3,
            feedback: 0.3,
        }
    }
}

impl Default for ReverbSettings {
    fn default() -> Self {
        Self {
            mix: 0.11,
            time: 3.5,
            damp: 0.88,
            size: 5.0,
        }
    }
}
