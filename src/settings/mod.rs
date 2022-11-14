mod v1;
use serde::{Deserialize, Serialize};

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
    pub fn from_reader<R>(f: R) -> Option<Self>
    where
        R: std::io::Read,
    {
        let settings: Version = serde_yaml::from_reader(f).ok()?;
        match settings {
            Version::V1(settings) => Some(settings),
        }
    }

    pub fn try_read() -> Option<Self> {
        let f = std::fs::File::open("settings.yaml").ok()?;
        Self::from_reader(f)
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
        let settings = Version::V1(self.clone());
        serde_yaml::to_writer(f, &settings).unwrap();
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
