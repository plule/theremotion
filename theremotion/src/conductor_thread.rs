use std::{cmp::Ordering, f32::consts::PI, thread};

use crossbeam_channel::{Receiver, Sender};
use itertools::Itertools;
use nalgebra::{Vector2, Vector3};
use staff::{midi::Octave, Interval, Pitch};

use crate::{
    controls, dsp_thread, leap_thread,
    settings::{Handedness, NamedScale, Preset, Settings},
    solfege::{IntervalF, Volume},
    ui_thread::{self, UiUpdate},
};

const HALF_PI: f32 = PI / 2.0;

pub struct HandMessage {
    pub x_factor: f32,
    pub position: Vector3<f32>,
    pub velocity: Vector3<f32>,
    pub rotation: Option<f32>,
    pub pinch: f32,
    pub grab: f32,
}

#[derive(Debug)]
pub enum LeapStatus {
    Error(String),
    Warning(String),
    Ok,
}

pub enum ConductorMessage {
    Exit,
    LeapStatus(LeapStatus),
    PitchHand(HandMessage),
    VolumeHand(HandMessage),
    VisibleHands { left: bool, right: bool },
    DroneClicked(i32),
    RootClicked(i32),
    ScaleClicked(i32),
    LeadOctave(i32),
    GuitarOctave(i32),
    DroneOctave(i32),
    FullscreenClicked,
    LHClicked,
    RHClicked,
    HighPriorityClicked,
    OnScreenKeyboardClicked,
    SelectScale(i32),
    DeleteScale(i32),
    SaveScale(String),
    SelectPreset(i32),
    DeletePreset(i32),
    SavePreset(String),
    LeadVolume(Volume),
    GuitarVolume(Volume),
    DroneVolume(Volume),
    MasterVolume(Volume),
    EchoAmount(Volume),
    EchoDuration(f32),
    EchoFeedback(f32),
    ReverbAmount(Volume),
    ReverbTime(f32),
    ReverbDamp(f32),
    ReverbSize(f32),
    DroneDetune(f32),
    GuitarDroneClicked,
}

pub fn run(
    settings: Settings,
    controls: controls::Controls,
    rx: Receiver<ConductorMessage>,
    dsp_tx: Sender<dsp_thread::ParameterUpdate>,
    ui_tx: Sender<ui_thread::UiUpdate>,
    leap_tx: Sender<leap_thread::Message>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut conductor = Conductor {
            settings,
            controls,
            dsp_tx,
            ui_tx,
            leap_tx,
            play_state: PlayState::default(),
        };
        conductor.run(rx).unwrap();
    })
}

/// The conductor interprets and transmits the messages between
/// the threads.
struct Conductor {
    /// Output: Sound parameter updates sent to the DSP
    pub dsp_tx: Sender<dsp_thread::ParameterUpdate>,

    /// Output: User interface updates
    pub ui_tx: Sender<UiUpdate>,

    /// Output: Setting updates sent to the leap thread
    pub leap_tx: Sender<leap_thread::Message>,

    /// Application settings current state
    pub settings: Settings,

    /// DSP control metadata
    pub controls: controls::Controls,

    /// Stateful playing state
    pub play_state: PlayState,
}

/// Stateful part of the playing interactions that are not part of the DSP
struct PlayState {
    pub guitar_gates: [bool; 4],
    pub drone_grab_state: Option<(f32, f32)>,
    pub drone_state: f32,
}

impl Default for PlayState {
    fn default() -> Self {
        Self {
            guitar_gates: [false, false, false, false],
            drone_grab_state: None,
            drone_state: 0.0,
        }
    }
}

impl Conductor {
    pub fn run(&mut self, rx: Receiver<ConductorMessage>) -> anyhow::Result<()> {
        for msg in rx.iter() {
            let exit = self.on_conductor_message(msg)?;
            if exit {
                return Ok(());
            }
        }
        Ok(())
    }

    fn on_conductor_message(&mut self, msg: ConductorMessage) -> anyhow::Result<bool> {
        let mut settings = self.settings.clone();

        let preset = &mut settings.current_preset;

        match msg {
            ConductorMessage::Exit => {
                log::debug!("Conductor thread exiting");
                self.leap_tx.send(leap_thread::Message::Exit)?;
                return Ok(true);
            }
            ConductorMessage::LeapStatus(status) => {
                self.ui_tx.send(UiUpdate::Status(status))?;
            }
            ConductorMessage::PitchHand(h) => {
                self.on_pitch_hand(h, preset)?;
            }
            ConductorMessage::VolumeHand(h) => {
                self.on_volume_hand(h, preset)?;
            }
            ConductorMessage::VisibleHands { left, right } => {
                self.ui_tx.send(UiUpdate::HasHands(left, right))?;
            }
            ConductorMessage::DroneClicked(note_index) => {
                toggle_drone(preset, note_index);
            }
            ConductorMessage::RootClicked(p) => {
                let pitch = Pitch::from_byte((p % 12) as u8);
                preset.pitch = pitch;
                tracing::debug!("Root note {} clicked, pitch: {}", p, preset.pitch);
            }
            ConductorMessage::ScaleClicked(note_index) => {
                toggle_scale_note(preset, note_index);
            }
            ConductorMessage::FullscreenClicked => {
                settings.system.fullscreen = !settings.system.fullscreen;
            }
            ConductorMessage::LHClicked => {
                settings.system.handedness = Handedness::LeftHanded;
            }
            ConductorMessage::RHClicked => {
                settings.system.handedness = Handedness::RightHanded;
            }
            ConductorMessage::HighPriorityClicked => {
                settings.system.high_priority_process = !settings.system.high_priority_process;
            }
            ConductorMessage::OnScreenKeyboardClicked => {
                settings.system.force_touchscreen = !settings.system.force_touchscreen;
            }
            ConductorMessage::LeadOctave(o) => {
                preset.lead_octave = Octave::new_unchecked(o as i8);
            }
            ConductorMessage::GuitarOctave(o) => {
                preset.guitar_octave = Octave::new_unchecked(o as i8);
            }
            ConductorMessage::DroneOctave(o) => {
                preset.drone_octave = Octave::new_unchecked(o as i8);
            }
            ConductorMessage::SelectScale(id) => {
                if let Some((scale, _)) = settings
                    .system_and_user_scales()
                    .find(|(s, _)| s.id() == id)
                {
                    settings.current_preset.scale = scale.scale;
                }
            }
            ConductorMessage::DeleteScale(id) => {
                settings.scales.retain(|s| s.id() != id);
            }
            ConductorMessage::SaveScale(name) => {
                settings
                    .scales
                    .push(NamedScale::new(name, settings.current_preset.scale));
            }
            ConductorMessage::SelectPreset(id) => {
                let preset = settings
                    .system_and_user_presets()
                    .find(|(p, _)| p.id() == id)
                    .map(|(p, _)| p.clone());
                if let Some(preset) = preset {
                    settings.current_preset = preset;
                }
            }
            ConductorMessage::DeletePreset(id) => {
                settings.presets.retain(|p| p.id() != id);
            }
            ConductorMessage::SavePreset(name) => {
                let mut preset = settings.current_preset.clone();
                preset.name = name;
                settings.presets.push(preset);
            }
            ConductorMessage::LeadVolume(v) => preset.mix.lead = v,
            ConductorMessage::GuitarVolume(v) => preset.mix.guitar = v,
            ConductorMessage::DroneVolume(v) => preset.mix.drone = v,
            ConductorMessage::MasterVolume(v) => preset.mix.master = v,
            ConductorMessage::EchoAmount(v) => preset.fx.echo.mix = v,
            ConductorMessage::EchoDuration(v) => preset.fx.echo.duration = v,
            ConductorMessage::EchoFeedback(v) => preset.fx.echo.feedback = v,
            ConductorMessage::ReverbAmount(v) => preset.fx.reverb.mix = v,
            ConductorMessage::ReverbTime(v) => preset.fx.reverb.time = v,
            ConductorMessage::ReverbDamp(v) => preset.fx.reverb.damp = v,
            ConductorMessage::ReverbSize(v) => preset.fx.reverb.size = v,
            ConductorMessage::DroneDetune(v) => preset.drone.detune = v,
            ConductorMessage::GuitarDroneClicked => {
                preset.drone.pluck_drone = !preset.drone.pluck_drone
            }
        }

        if settings != self.settings {
            tracing::debug!("Settings were updated");
            self.ui_tx.send(UiUpdate::Settings(settings.clone()))?;
            self.leap_tx
                .send(leap_thread::Message::SettingsUpdate(settings.clone()))?;
            settings
                .current_preset
                .send_to_dsp(&self.controls, &self.dsp_tx)?;
            self.settings = settings;
            self.settings.save()?;
        }

        Ok(false)
    }

    fn on_pitch_hand(&mut self, h: HandMessage, preset: &Preset) -> anyhow::Result<()> {
        let dsp_tx = &mut self.dsp_tx;
        let ui_tx = &mut self.ui_tx;

        let full_scale_window = preset.full_scale_floating_window();
        let restricted_scale_window = preset.restricted_scale_floating_window();
        let note_range = preset.note_range_f();
        let antenna_coord = Vector2::new(400.0, -200.0);
        let pitch_coord_mm = antenna_coord - Vector2::new(h.position.x, h.position.z);
        let pitch_coord_semitones = pitch_coord_mm / 15.0;
        let pitch_coord_semitones = Vector2::new(-pitch_coord_semitones.x, pitch_coord_semitones.y);
        let pitch_distance_semitones = IntervalF(pitch_coord_semitones.norm());
        let raw_note = (*note_range.end() - pitch_distance_semitones)
            .clamp(*note_range.start(), *note_range.end());
        let note_number_height =
            controls::convert_range(h.position.y, &(350.0..=500.0), &(1.0..=4.0));
        let lead_volumes =
            [0.0, 1.0, 2.0, 3.0].map(|v| (note_number_height.clamp(1.0, 4.0) - v).clamp(0.0, 1.0));
        self.play_state.guitar_gates = lead_volumes.map(|v| v > 0.0);
        let autotune = controls::convert_range(h.pinch, &(0.0..=1.0), &(0.0..=5.0)) as usize;
        let note = restricted_scale_window.autotune(raw_note, autotune);
        let chord = full_scale_window.autochord(note, &[0, 2, 4, 7]);
        let lead_offset = preset.lead_interval_f();
        let pluck_offset = preset.pluck_interval_f();
        let pitch_bend = self
            .controls
            .pitch_bend
            .get_scaled(h.velocity.x + h.velocity.z, &(-300.0..=300.0));
        let trumpet = self
            .controls
            .drone_trumpet
            .get_scaled(h.velocity.y.abs(), &(0.0..=250.0));
        for (control, value) in self.controls.lead.iter().zip(lead_volumes) {
            control.volume.send(dsp_tx, value)?;
        }
        for (i, note) in chord.iter().enumerate() {
            if let Some(note) = note {
                self.controls.lead[i].send_note(dsp_tx, &(*note + lead_offset))?;
                self.controls.strum[i].send_note(dsp_tx, &(*note + pluck_offset))?;
            }
        }
        self.controls.strum_drone.send_note(
            dsp_tx,
            &(preset.root_note_f() + pluck_offset + IntervalF(12.0)),
        )?;
        self.controls.pitch_bend.send(dsp_tx, pitch_bend)?;
        self.controls.drone_trumpet.send(dsp_tx, trumpet)?;
        let lead_chord = chord
            .into_iter()
            .map(|c| c.unwrap_or_default())
            .zip(lead_volumes.into_iter().map(Volume))
            .collect_vec();
        let lead_chord = [lead_chord[0], lead_chord[1], lead_chord[2], lead_chord[3]];
        ui_tx.send(UiUpdate::AutotuneAmount(autotune))?;
        ui_tx.send(UiUpdate::Lead(
            lead_chord,
            Vector2::new(
                pitch_coord_semitones.x * h.x_factor,
                pitch_coord_semitones.y,
            ),
        ))?;
        ui_tx.send(UiUpdate::RawNote(raw_note))?;
        ui_tx.send(UiUpdate::ChordsNumber(note_number_height))?;
        ui_tx.send(UiUpdate::TrumpetStrength(trumpet))?;
        Ok(())
    }

    fn on_volume_hand(&mut self, h: HandMessage, preset: &Preset) -> anyhow::Result<()> {
        let dsp_tx = &mut self.dsp_tx;
        let ui_tx = &mut self.ui_tx;

        let strum_ready = h.pinch > 0.9;
        if let Some(rotation) = h.rotation {
            if strum_ready {
                for (i, string) in &mut self.controls.strum.iter().enumerate() {
                    string.pluck.send(
                        dsp_tx,
                        rotation > HALF_PI + (i as f32) * 0.2 && self.play_state.guitar_gates[i],
                    );
                }
                self.controls
                    .strum_drone
                    .pluck
                    .send(dsp_tx, preset.drone.pluck_drone && rotation > HALF_PI + 0.3);
            }

            let pluck_mute = self
                .controls
                .pluck_mute
                .get_scaled(rotation, &(0.0..=(HALF_PI - 0.2)));
            self.controls.pluck_mute.send(dsp_tx, pluck_mute)?;
        }
        let cutoff_note_norm =
            controls::convert_range(h.position.x, &(50.0..=200.0), &(-1.0..=1.0)).clamp(-1.0, 1.0);
        let cutoff_note = self
            .controls
            .cutoff_note
            .get_scaled(cutoff_note_norm, &(-1.0..=1.0));
        let resonance_norm =
            controls::convert_range(h.position.z, &(100.0..=-100.0), &(0.0..=1.0)).clamp(0.0, 1.0);
        let resonance = self
            .controls
            .resonance
            .get_scaled(resonance_norm, &(0.0..=1.0));
        let lead_volume = self
            .controls
            .lead_volume
            .get_scaled(h.position.y, &(300.0..=400.0));
        if h.grab >= 1.0 {
            if let Some(drone_volume_angle) = h.rotation {
                let (init_drone_volume, init_drone_volume_angle) = *self
                    .play_state
                    .drone_grab_state
                    .get_or_insert((self.play_state.drone_state, drone_volume_angle));
                let offset = drone_volume_angle - init_drone_volume_angle;
                self.play_state.drone_state = (init_drone_volume + offset).clamp(0.0, 5.0);
                let drone_volumes = [0.0, 1.0, 2.0, 3.0]
                    .map(|v| (self.play_state.drone_state.clamp(0.0, 4.0) - v).clamp(0.0, 1.0));
                let drone_interval = preset.drone_interval();
                for ((control, drone), volume) in self
                    .controls
                    .drone_notes
                    .iter()
                    .zip(preset.drone_notes())
                    .zip(drone_volumes)
                {
                    if let Some(drone) = drone {
                        control
                            .note
                            .send(dsp_tx, ((drone + drone_interval).into_byte()) as f32)?;
                        control.volume.send(dsp_tx, volume)?;
                    } else {
                        control.volume.send(dsp_tx, 0.0)?;
                    }
                }
            }
        } else {
            self.play_state.drone_grab_state = None;
        }
        self.controls.cutoff_note.send(dsp_tx, cutoff_note)?;
        self.controls.lead_volume.send(dsp_tx, lead_volume)?;
        self.controls.resonance.send(dsp_tx, resonance)?;
        ui_tx.send(UiUpdate::Filter(
            cutoff_note_norm * h.x_factor,
            resonance_norm,
        ))?;
        ui_tx.send(UiUpdate::LeadVolume(lead_volume))?;
        ui_tx.send(UiUpdate::StrumReady(strum_ready))?;
        Ok(())
    }
}

fn toggle_scale_note(preset: &mut Preset, note_index: i32) {
    let root_index = preset.root_note().into_byte() as i32;
    let interval = note_index - root_index;
    if interval >= 0 {
        let interval = Interval::new((interval % 12) as u8);
        if preset.scale.contains(interval) {
            preset.scale.remove(interval);
        } else {
            preset.scale.push(interval);
        }

        tracing::debug!("Scale {} clicked", note_index);
    }
}

fn toggle_drone(preset: &mut Preset, note_index: i32) {
    let root_index = preset.root_note().into_byte() as i32;
    let interval = note_index - root_index;
    if (0..=(12 * 3)).contains(&interval) {
        let interval = Interval::new(interval as u8);
        let drone_intervals = &mut preset.drone.intervals;
        if let Some(existing_drone) = drone_intervals
            .iter_mut()
            .find(|n| n.iter().any(|n| *n == interval))
        {
            *existing_drone = None;
        } else if let Some(empty_slot) = drone_intervals.iter_mut().find(|n| n.is_none()) {
            *empty_slot = Some(interval);
        }

        drone_intervals.sort_unstable_by(|a, b| match (a, b) {
            (None, None) => Ordering::Equal,
            (None, Some(_)) => Ordering::Less,
            (Some(_), None) => Ordering::Greater,
            (Some(a), Some(b)) => {
                if a < b {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
        });

        tracing::debug!(
            "Drone {} clicked, drone: {:?}",
            note_index,
            &drone_intervals
        );
    }
}
