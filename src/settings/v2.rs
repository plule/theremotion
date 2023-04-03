use super::v1;
use serde::{Deserialize, Serialize};
use staff::{
    midi::{MidiNote, Octave},
    scale::ScaleIntervals,
    Pitch,
};

/// Application settings
#[derive(Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(deny_unknown_fields)]
pub struct Settings {
    /// Current sound settings
    #[serde(default)]
    pub current_preset: Preset,

    /// Saved presets
    #[serde(default)]
    pub presets: Vec<Preset>,

    /// Saved scales
    #[serde(default)]
    pub scales: Vec<v1::NamedScale>,

    /// System settings
    #[serde(default)]
    pub system: v1::System,
}

/// Drone settings
#[derive(Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(deny_unknown_fields, default)]
pub struct DroneSettings {
    /// List of notes of the drone
    pub intervals: [Option<u8>; 4],
    /// Detune amount (in midi note) between the notes
    pub detune: f32,
}

/// Sound preset
#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Preset {
    /// Name of the preset
    pub name: String,

    /// Octave of the root note
    pub lead_octave: Octave,

    /// Octave of the guitar sound
    pub guitar_octave: Octave,

    /// Octave of the drone
    pub drone_octave: Octave,

    /// Pitch of the root note
    pub pitch: Pitch,

    /// Scale of the autotune
    pub scale: ScaleIntervals,

    /// Current drone
    #[serde(default)]
    pub drone: DroneSettings,

    /// Volume settings
    #[serde(default)]
    pub mix: v1::MixSettings,

    /// Effects settings
    #[serde(default)]
    pub fx: v1::FxSettings,
}

impl Default for Preset {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            lead_octave: Octave::THREE,
            guitar_octave: Octave::THREE,
            drone_octave: Octave::THREE,
            pitch: Pitch::C,
            scale: ScaleIntervals::major(),
            drone: Default::default(),
            mix: Default::default(),
            fx: Default::default(),
        }
    }
}

impl From<v1::Preset> for Preset {
    fn from(value: v1::Preset) -> Self {
        let root_note = MidiNote::new(value.pitch, value.octave);
        let drone_intervals = value
            .drone
            .notes
            .map(|note| note.map(|note| (note - root_note).semitones()));
        Self {
            name: value.name,
            lead_octave: value.octave,
            guitar_octave: value.guitar_octave,
            drone_octave: value.octave,
            pitch: value.pitch,
            scale: value.scale,
            drone: DroneSettings {
                intervals: drone_intervals,
                detune: value.drone.detune,
            },
            mix: value.mix,
            fx: value.fx,
        }
    }
}

impl From<v1::Settings> for Settings {
    fn from(value: v1::Settings) -> Self {
        Self {
            current_preset: value.current_preset.into(),
            presets: value.presets.into_iter().map(|p| p.into()).collect(),
            scales: value.scales,
            system: value.system,
        }
    }
}
