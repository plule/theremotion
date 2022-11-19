mod v1;
use std::{ops::RangeInclusive, path::PathBuf};

use anyhow::{Context, Ok, Result};
use serde::{Deserialize, Serialize};
use staff::{
    midi::{MidiNote, Octave},
    Interval,
};

pub type Settings = v1::Settings;
pub type Preset = v1::Preset;

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

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use staff::midi::MidiNote;

    use super::*;

    #[rstest]
    fn from_v1() {
        let f = std::fs::File::open("src/settings/v1.yaml").unwrap();
        let settings = Settings::from_reader(f).unwrap();
        assert_eq!("Current", settings.current_preset.name);
        assert_eq!(1, settings.presets.len());
        assert_eq!(
            Some(MidiNote::from_byte(50)),
            settings.current_preset.drone.notes[0]
        );
        assert_eq!(None, settings.current_preset.drone.notes[1]);
        assert_eq!(0.1, settings.current_preset.mix.drone);
        assert_eq!(0.9, settings.current_preset.fx.echo.mix);
        assert_eq!(0.8, settings.current_preset.fx.reverb.mix);
    }
}
