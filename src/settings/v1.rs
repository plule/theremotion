use serde::{Deserialize, Serialize};
use staff::{
    midi::{MidiNote, Octave},
    scale::ScaleIntervals,
    Pitch,
};

use crate::solfege::OctaveInterval;

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
    pub scales: Vec<NamedScale>,

    /// System settings
    #[serde(default)]
    pub system: System,
}

/// Sound preset
#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Preset {
    /// Name of the preset
    pub name: String,

    /// Octave of the root note
    pub octave: Octave,

    /// Octave of the guitar sound
    pub guitar_octave: Octave,

    /// Pitch of the root note
    pub pitch: Pitch,

    /// Number of octave to display
    pub octave_range: OctaveInterval,

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

impl Default for Preset {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            octave: Octave::THREE,
            guitar_octave: Octave::THREE,
            pitch: Pitch::C,
            octave_range: OctaveInterval::new(3),
            scale: ScaleIntervals::major(),
            drone: Default::default(),
            mix: Default::default(),
            fx: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct MixSettings {
    #[serde(default)]
    pub master: f32,
    #[serde(default)]
    pub lead: f32,
    #[serde(default)]
    pub guitar: f32,
    #[serde(default)]
    pub drone: f32,
}

impl Default for MixSettings {
    fn default() -> Self {
        Self {
            master: 1.0,
            lead: 1.0,
            guitar: 1.0,
            drone: 0.14,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(deny_unknown_fields)]
pub struct FxSettings {
    #[serde(default)]
    pub echo: EchoSettings,
    #[serde(default)]
    pub reverb: ReverbSettings,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct EchoSettings {
    #[serde(default)]
    pub mix: f32,
    #[serde(default)]
    pub duration: f32,
    #[serde(default)]
    pub feedback: f32,
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

#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ReverbSettings {
    #[serde(default)]
    pub mix: f32,
    #[serde(default)]
    pub time: f32,
    #[serde(default)]
    pub damp: f32,
    #[serde(default)]
    pub size: f32,
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

#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
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

#[derive(Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct System {
    /// Start theremotion in full screen
    #[serde(default)]
    pub fullscreen: bool,

    /// Run tabtip.exe on text input
    #[serde(default)]
    pub tabtip: bool,

    /// Bump up the process priority
    #[serde(default)]
    pub high_priority_process: bool,

    #[serde(default)]
    pub handedness: Handedness,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub enum Handedness {
    RightHanded,
    LeftHanded,
}

impl Default for Handedness {
    fn default() -> Self {
        Self::RightHanded
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(deny_unknown_fields, default)]
pub struct NamedScale {
    pub name: String,
    pub scale: ScaleIntervals,
}

impl NamedScale {
    pub fn new(name: String, scale: ScaleIntervals) -> Self {
        Self { name, scale }
    }
}
