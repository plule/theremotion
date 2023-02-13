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

    /// Volume settings
    #[serde(default)]
    pub mix: MixSettings,

    /// Effects settings
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

/// Mix table settings
#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields, default)]
pub struct MixSettings {
    /// Master volume
    pub master: f32,
    /// Lead synthesizer volume
    pub lead: f32,
    /// Guitar volume
    pub guitar: f32,
    /// Drone volume
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

/// Effects settings
#[derive(Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(deny_unknown_fields, default)]
pub struct FxSettings {
    /// Echo settings
    pub echo: EchoSettings,
    /// Reverb settings
    pub reverb: ReverbSettings,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields, default)]
/// Echo settings
pub struct EchoSettings {
    /// Echo amount
    pub mix: f32,
    /// Echo duration (seconds)
    pub duration: f32,
    /// Echo feedback (0-1)
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

/// Reverb settings
#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields, default)]
pub struct ReverbSettings {
    /// Reverb amount
    pub mix: f32,
    /// Reverb time
    pub time: f32,
    /// Reverb damp amount
    pub damp: f32,
    /// Reverb room size
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

/// Drone settings
#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields, default)]
pub struct DroneSettings {
    /// List of notes of the drone
    pub notes: [Option<MidiNote>; 4],
    /// Detune amount (in midi note) between the notes
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

/// Left or right handed mode
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub enum Handedness {
    /// Right handed mode
    ///
    /// The right hand controls the note, the left hand controls the note
    RightHanded,
    /// Left handed mode
    ///
    /// The left hand controls the note, the right hand controls the volme
    LeftHanded,
}

impl Default for Handedness {
    fn default() -> Self {
        Self::RightHanded
    }
}

/// Scale with a name
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(deny_unknown_fields, default)]
pub struct NamedScale {
    /// Human readable name
    pub name: String,
    /// Corresponding scale
    pub scale: ScaleIntervals,
}

impl NamedScale {
    /// Creates a new [`NamedScale`].
    pub fn new(name: String, scale: ScaleIntervals) -> Self {
        Self { name, scale }
    }
}
