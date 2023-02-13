mod v1;
use std::{ops::RangeInclusive, path::PathBuf};

use anyhow::{Context, Ok, Result};
use crossbeam_channel::Sender;
use serde::{Deserialize, Serialize};
use staff::midi::MidiNote;

use crate::{
    controls::Controls,
    dsp_thread::ParameterUpdate,
    solfege::{MidiNoteF, ScaleWindows},
};

use self::v1::{DroneSettings, EchoSettings, FxSettings, MixSettings, ReverbSettings};

pub type Settings = v1::Settings;
pub type Preset = v1::Preset;
pub type Handedness = v1::Handedness;
pub type NamedScale = v1::NamedScale;

const PRESETS_BYTES: &[u8] = include_bytes!("presets.yaml");

lazy_static::lazy_static! {
    static ref PRESETS: Vec<Preset> = serde_yaml::from_slice(PRESETS_BYTES).unwrap();
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
enum Version {
    V1(v1::Settings),
}

impl Default for Version {
    fn default() -> Self {
        Version::V1(v1::Settings::default())
    }
}

impl Settings {
    pub fn from_reader<R>(f: R) -> Result<Self>
    where
        R: std::io::Read,
    {
        let settings: Version = serde_yaml::from_reader(f)?;
        match settings {
            Version::V1(settings) => Ok(settings),
        }
    }

    fn path() -> Result<PathBuf> {
        let directories = directories::ProjectDirs::from("", "", "Theremotion")
            .context("No settings directory")?;
        let directory = directories.config_dir();
        Ok(directory.with_file_name("settings.yaml"))
    }

    pub fn try_read() -> Result<Self> {
        let path = Settings::path()?;
        log::debug!(
            "Loading settings from {}",
            path.to_str().unwrap_or_default()
        );
        let f = std::fs::File::open(path)?;
        Self::from_reader(f)
    }

    pub fn read() -> Self {
        Self::try_read().unwrap_or_default()
    }

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
        let settings = Version::V1(self.clone());
        serde_yaml::to_writer(f, &settings)?;
        Ok(())
    }
}

impl Preset {
    pub fn root_note(&self) -> MidiNote {
        MidiNote::new(self.pitch, self.octave)
    }

    pub fn note_range(&self) -> RangeInclusive<MidiNote> {
        self.root_note()..=(self.root_note() + self.octave_range.into())
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
        crate::solfege::build_scale_notes(self.root_note(), self.scale, range)
    }

    /// Send the relevant preset data to the DSP
    pub fn send_to_dsp(&self, controls: &Controls, tx: &Sender<ParameterUpdate>) -> Result<()> {
        self.drone.send_to_dsp(controls, tx)?;
        self.mix.send_to_dsp(controls, tx)?;
        self.fx.send_to_dsp(controls, tx)?;
        Ok(())
    }

    pub fn system_presets() -> &'static Vec<Self> {
        &PRESETS
    }
}

impl DroneSettings {
    pub fn send_to_dsp(&self, controls: &Controls, tx: &Sender<ParameterUpdate>) -> Result<()> {
        controls.drone_detune.send(tx, self.detune)?;
        for (control, drone) in controls.drone_notes.iter().zip(self.notes) {
            if let Some(drone) = drone {
                control.note.send(tx, drone.into_byte() as f32)?;
                control.volume.send(tx, 1.0)?;
            } else {
                control.volume.send(tx, 0.0)?;
            }
        }
        Ok(())
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

    use super::*;

    #[rstest]
    fn from_v1() {
        let f = std::fs::File::open("src/settings/v1.yaml").unwrap();
        let settings = Settings::from_reader(f).unwrap();
        assert_eq!("Current", settings.current_preset.name);
        assert_eq!(1, settings.presets.len());
        assert_eq!(Octave::TWO, settings.current_preset.octave);
        assert_eq!(Octave::THREE, settings.current_preset.guitar_octave);
        assert_eq!(
            Some(MidiNote::from_byte(50)),
            settings.current_preset.drone.notes[0]
        );
        assert_eq!(None, settings.current_preset.drone.notes[1]);
        assert_eq!(0.1, settings.current_preset.mix.drone);
        assert_eq!(0.9, settings.current_preset.fx.echo.mix);
        assert_eq!(0.8, settings.current_preset.fx.reverb.mix);
    }

    #[rstest]
    fn default() {
        // Dynamically deserialized at runtime...
        assert!(!Preset::system_presets().is_empty());
    }
}
