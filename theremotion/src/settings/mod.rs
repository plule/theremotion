mod v1;
mod v2;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    ops::RangeInclusive,
    path::PathBuf,
};

use anyhow::{Context, Ok, Result};
use crossbeam_channel::Sender;
use serde::{Deserialize, Serialize};
use staff::{
    midi::{MidiNote, Octave},
    scale::ScaleIntervals,
    Interval,
};

use crate::{
    controls::Controls,
    dsp_thread::ParameterUpdate,
    solfege::{MoreScales, ScaleWindows},
    {IntervalF, MidiNoteF, OctaveInterval},
};

pub use self::v1::{EchoSettings, FxSettings, Handedness, MixSettings, NamedScale, ReverbSettings};

pub use self::v2::{DroneSettings, Preset, Settings};

/// Default presets
const PRESETS_BYTES: &[u8] = include_bytes!("presets.yaml");

lazy_static::lazy_static! {
    /// Default presets
    static ref PRESETS: Vec<Preset> = serde_yaml::from_slice(PRESETS_BYTES).unwrap();
}

/// Versionned application settings.
///
/// This is the top-level serialized object
#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
enum Version {
    V1(v1::Settings),
    V2(v2::Settings),
}

impl Default for Version {
    fn default() -> Self {
        Version::V2(v2::Settings::default())
    }
}

impl Settings {
    /// Initialize from a stream
    pub fn from_reader<R>(f: R) -> Result<Self>
    where
        R: std::io::Read,
    {
        let settings: Version = serde_yaml::from_reader(f)?;
        match settings {
            Version::V1(settings) => Ok(settings.into()),
            Version::V2(settings) => Ok(settings),
        }
    }

    fn path() -> Result<PathBuf> {
        let directories = directories::ProjectDirs::from("", "", "Theremotion")
            .context("No settings directory")?;
        let directory = directories.config_dir();
        Ok(directory.with_file_name("settings.yaml"))
    }

    /// Try reading the settings
    fn try_read() -> Result<Self> {
        let path = Settings::path()?;
        log::debug!(
            "Loading settings from {}",
            path.to_str().unwrap_or_default()
        );
        let f = std::fs::File::open(path)?;
        Self::from_reader(f)
    }

    /// Read the settings or get the default
    pub fn read() -> Self {
        Self::try_read().unwrap_or_default()
    }

    /// Save the settings
    pub fn save(&self) -> Result<()> {
        let path = Settings::path()?;
        if let Some(parent) = path.as_path().parent() {
            std::fs::create_dir_all(parent)?;
        }

        log::debug!("Saving settings to {}", path.to_str().unwrap_or_default());

        let f = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)?;
        let settings = Version::V2(self.clone());
        serde_yaml::to_writer(f, &settings)?;
        Ok(())
    }

    pub fn system_and_user_scales(&self) -> impl Iterator<Item = (NamedScale, bool)> {
        let user_scales = self.scales.clone().into_iter().map(|s| (s, true));
        let system_scales = [
            ("Chromatic", ScaleIntervals::all()),
            ("Major", ScaleIntervals::major()),
            ("Melodic Minor", ScaleIntervals::melodic_minor()),
            ("Harmonic Minor", ScaleIntervals::harmonic_minor()),
            ("Natural Minor", ScaleIntervals::natural_minor()),
            ("Dorian", ScaleIntervals::dorian()),
            ("Blues", ScaleIntervals::blues()),
            ("Freygish", ScaleIntervals::freygish()),
            ("Altered Dorian", ScaleIntervals::altered_dorian()),
        ]
        .map(|(name, scale)| (NamedScale::new(name.to_string(), scale), false));
        user_scales.chain(system_scales)
    }

    pub fn system_and_user_presets(&self) -> impl Iterator<Item = (&Preset, bool)> {
        let user_presets = self.presets.iter().map(|p| (p, true));
        let system_presets = Preset::system_presets().iter().map(|p| (p, false));
        user_presets.chain(system_presets)
    }
}

impl Preset {
    pub fn octave_range() -> OctaveInterval {
        OctaveInterval(3)
    }
    /// Root note from the first octave
    pub fn root_note(&self) -> MidiNote {
        MidiNote::new(self.pitch, Octave::NEGATIVE_ONE)
    }

    pub fn root_note_f(&self) -> MidiNoteF {
        self.root_note().into()
    }

    pub fn lead_interval(&self) -> Interval {
        OctaveInterval::from_octaves(Octave::NEGATIVE_ONE, self.lead_octave).into()
    }

    pub fn lead_interval_f(&self) -> IntervalF {
        self.lead_interval().into()
    }

    pub fn pluck_interval(&self) -> Interval {
        OctaveInterval::from_octaves(Octave::NEGATIVE_ONE, self.guitar_octave).into()
    }

    pub fn pluck_interval_f(&self) -> IntervalF {
        self.pluck_interval().into()
    }

    pub fn drone_interval(&self) -> Interval {
        OctaveInterval::from_octaves(Octave::NEGATIVE_ONE, self.drone_octave).into()
    }

    pub fn note_range(&self) -> RangeInclusive<MidiNote> {
        self.root_note()..=(self.root_note() + Self::octave_range().into())
    }

    pub fn note_range_f(&self) -> RangeInclusive<MidiNoteF> {
        let range = self.note_range();
        (MidiNoteF::from(*range.start()))..=(MidiNoteF::from(*range.end()))
    }

    /// List all the notes of the current scale for the selected number of octaves
    pub fn restricted_scale(&self) -> Vec<MidiNote> {
        self.scale_notes(self.note_range())
    }

    /// List all the notes of the current scale for the whole keyboard
    pub fn full_scale(&self) -> Vec<MidiNote> {
        self.scale_notes(MidiNote::from_byte(0)..=MidiNote::from_byte(127))
    }

    /// Build the scale two by two floating window for the full keyboard
    pub fn full_scale_floating_window(&self) -> ScaleWindows {
        ScaleWindows::from_notes(self.full_scale())
    }

    /// Build the scale two by two floating window for the selected number of octaves
    pub fn restricted_scale_floating_window(&self) -> ScaleWindows {
        ScaleWindows::from_notes(self.restricted_scale())
    }

    /// List all the notes in the current scale for the given range
    fn scale_notes(&self, range: RangeInclusive<MidiNote>) -> Vec<MidiNote> {
        crate::solfege::build_scale_notes(self.pitch, self.scale, range)
    }

    /// Send the relevant preset data to the DSP
    pub fn send_to_dsp(&self, controls: &Controls, tx: &Sender<ParameterUpdate>) -> Result<()> {
        controls.drone_detune.send(tx, self.drone.detune)?;
        let drone_interval = self.drone_interval();
        for (control, drone) in controls.drone_notes.iter().zip(self.drone_notes()) {
            if let Some(drone) = drone {
                control
                    .note
                    .send(tx, ((drone + drone_interval).into_byte()) as f32)?;
            }
        }

        self.mix.send_to_dsp(controls, tx)?;
        self.fx.send_to_dsp(controls, tx)?;
        Ok(())
    }

    pub fn drone_notes(&self) -> [Option<MidiNote>; 4] {
        let root_note = self.root_note();
        self.drone
            .intervals
            .map(|drone| drone.map(|drone| root_note + drone))
    }

    pub fn system_presets() -> &'static Vec<Self> {
        &PRESETS
    }

    pub fn id(&self) -> i32 {
        let mut hasher = DefaultHasher::default();
        self.name.hash(&mut hasher);
        hasher.finish() as i32
    }
}

impl MixSettings {
    pub fn send_to_dsp(&self, controls: &Controls, tx: &Sender<ParameterUpdate>) -> Result<()> {
        controls.mix_drone_volume.send(tx, self.drone)?;
        controls.mix_lead_volume.send(tx, self.lead)?;
        controls.mix_master_volume.send(tx, self.master)?;
        controls.mix_pluck_volume.send(tx, self.guitar)?;
        Ok(())
    }
}

impl EchoSettings {
    pub fn send_to_dsp(&self, controls: &Controls, tx: &Sender<ParameterUpdate>) -> Result<()> {
        controls.echo_duration.send(tx, self.duration)?;
        controls.echo_feedback.send(tx, self.feedback)?;
        controls.echo_mix.send(tx, self.mix)?;
        Ok(())
    }
}

impl ReverbSettings {
    pub fn send_to_dsp(&self, controls: &Controls, tx: &Sender<ParameterUpdate>) -> Result<()> {
        controls.reverb_damp.send(tx, self.damp)?;
        controls.reverb_mix.send(tx, self.mix)?;
        controls.reverb_size.send(tx, self.size)?;
        controls.reverb_time.send(tx, self.time)?;
        Ok(())
    }
}

impl FxSettings {
    pub fn send_to_dsp(&self, controls: &Controls, tx: &Sender<ParameterUpdate>) -> Result<()> {
        self.echo.send_to_dsp(controls, tx)?;
        self.reverb.send_to_dsp(controls, tx)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use staff::midi::{MidiNote, Octave};

    use crate::Volume;

    use super::*;

    #[rstest]
    fn from_v1() {
        let f = std::fs::File::open("src/settings/v1.yaml").unwrap();
        let settings = Settings::from_reader(f).unwrap();
        assert_eq!("Current", settings.current_preset.name);
        assert_eq!(1, settings.presets.len());
        assert_eq!(Octave::TWO, settings.current_preset.lead_octave);
        assert_eq!(Octave::THREE, settings.current_preset.guitar_octave);
        assert_eq!(
            MidiNote::from_byte(50),
            settings.current_preset.drone_notes()[0].unwrap()
                + settings.current_preset.drone_interval()
        );
        assert_eq!(None, settings.current_preset.drone.intervals[1]);
        assert_eq!(Volume(0.1), settings.current_preset.mix.drone);
        assert_eq!(Volume(0.9), settings.current_preset.fx.echo.mix);
        assert_eq!(Volume(0.8), settings.current_preset.fx.reverb.mix);
    }

    #[rstest]
    fn default() {
        // Dynamically deserialized at runtime...
        assert!(!Preset::system_presets().is_empty());
    }
}
