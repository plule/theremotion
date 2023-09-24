use super::v1;
use serde::{Deserialize, Serialize};
use staff::{
    midi::{MidiNote, Octave},
    scale::ScaleIntervals,
    Interval, Pitch,
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
    #[serde(with = "interval_list_serde")]
    pub intervals: [Option<Interval>; 4],
    /// Detune amount (in midi note) between the notes
    pub detune: f32,
    /// Enable the pluck drone
    pub pluck_drone: bool,
}

/// Proxy the interval (de)serialization to flatten it
mod interval_list_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use staff::Interval;

    #[derive(Serialize, Deserialize)]
    struct IntervalList([Option<u8>; 4]);

    impl From<&[Option<Interval>; 4]> for IntervalList {
        fn from(value: &[Option<Interval>; 4]) -> Self {
            Self(value.map(|v| v.map(|v| v.semitones())))
        }
    }

    impl From<IntervalList> for [Option<Interval>; 4] {
        fn from(value: IntervalList) -> Self {
            value.0.map(|i| i.map(Interval::new))
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<[Option<Interval>; 4], D::Error>
    where
        D: Deserializer<'de>,
    {
        IntervalList::deserialize(deserializer).map(|i| i.into())
    }

    pub fn serialize<S>(intervals: &[Option<Interval>; 4], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        IntervalList::from(intervals).serialize(serializer)
    }
}

/// Sound preset
#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields, default)]
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
    pub drone: DroneSettings,

    /// Volume settings
    pub mix: v1::MixSettings,

    /// Effects settings
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
            .map(|note| note.map(|note| (note - root_note)));
        Self {
            name: value.name,
            lead_octave: value.octave,
            guitar_octave: value.guitar_octave,
            drone_octave: value.octave,
            pitch: value.pitch,
            scale: value.scale,
            drone: DroneSettings {
                intervals: drone_intervals,
                pluck_drone: false,
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
