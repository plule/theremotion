use std::{f32::consts::PI, thread};

use crossbeam_channel::{Receiver, Sender};
use itertools::Itertools;
use nalgebra::{Vector2, Vector3};
use staff::{midi::Octave, Interval, Pitch};

use crate::{
    controls, dsp_thread,
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

pub enum ConductorMessage {
    LeapError(Option<String>),
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
    leap_tx: Sender<Settings>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let _ = conductor(rx, settings, controls, dsp_tx, ui_tx, leap_tx);
    })
}

fn conductor(
    rx: Receiver<ConductorMessage>,
    mut current_settings: Settings,
    controls: controls::Controls,
    mut dsp_tx: Sender<dsp_thread::ParameterUpdate>,
    mut ui_tx: Sender<UiUpdate>,
    leap_tx: Sender<Settings>,
) -> anyhow::Result<()> {
    // Guitar gates
    let mut guitar_gates = [false, false, false, false];
    let mut drone_grab_state: Option<(Volume, f32)> = None;

    let controls = &controls;
    let dsp_tx: &mut Sender<dsp_thread::ParameterUpdate> = &mut dsp_tx;
    let ui_tx: &mut Sender<ui_thread::UiUpdate> = &mut ui_tx;

    for msg in rx.iter() {
        let mut settings = current_settings.clone();

        let preset = &mut settings.current_preset;

        match msg {
            ConductorMessage::LeapError(None) => {
                ui_tx.send(UiUpdate::ErrorReset)?;
            }
            ConductorMessage::LeapError(Some(error)) => {
                tracing::debug!("Leap error: {}", error);
                ui_tx.send(UiUpdate::Error(error))?;
            }
            ConductorMessage::PitchHand(h) => {
                on_pitch_hand(h, controls, preset, dsp_tx, ui_tx, &mut guitar_gates)?;
            }
            ConductorMessage::VolumeHand(h) => {
                on_volume_hand(
                    h,
                    controls,
                    preset,
                    dsp_tx,
                    ui_tx,
                    &guitar_gates,
                    &mut drone_grab_state,
                )?;
            }
            ConductorMessage::VisibleHands { left, right } => {
                ui_tx.send(UiUpdate::HasHands(left, right))?;
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

        if settings != current_settings {
            tracing::debug!("Settings were updated");
            ui_tx.send(UiUpdate::Settings(settings.clone()))?;
            leap_tx.send(settings.clone())?;
            settings.current_preset.send_to_dsp(controls, dsp_tx)?;
            current_settings = settings;
            current_settings.save()?;
        }
    }
    Ok(())
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

        dbg!(preset.scale.count());

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

        tracing::debug!(
            "Drone {} clicked, drone: {:?}",
            note_index,
            &drone_intervals
        );
    }
}

fn on_volume_hand(
    h: HandMessage,
    controls: &controls::Controls,
    preset: &mut Preset,
    dsp_tx: &mut Sender<dsp_thread::ParameterUpdate>,
    ui_tx: &mut Sender<UiUpdate>,
    guitar_gates: &[bool; 4],
    drone_grab_state: &mut Option<(Volume, f32)>,
) -> Result<(), anyhow::Error> {
    let strum_ready = h.pinch > 0.9;
    if let Some(rotation) = h.rotation {
        if strum_ready {
            for (i, string) in &mut controls.strum.iter().enumerate() {
                string.pluck.send(
                    dsp_tx,
                    rotation > HALF_PI + (i as f32) * 0.2 && guitar_gates[i],
                );
            }
            controls
                .strum_drone
                .pluck
                .send(dsp_tx, preset.drone.pluck_drone && rotation > HALF_PI + 0.3);
        }

        let pluck_mute = controls
            .pluck_mute
            .get_scaled(rotation, &(0.0..=(HALF_PI - 0.2)));
        controls.pluck_mute.send(dsp_tx, pluck_mute)?;
    }
    let cutoff_note_norm =
        controls::convert_range(h.position.x, &(50.0..=200.0), &(-1.0..=1.0)).clamp(-1.0, 1.0);
    let cutoff_note = controls
        .cutoff_note
        .get_scaled(cutoff_note_norm, &(-1.0..=1.0));
    let resonance_norm =
        controls::convert_range(h.position.z, &(100.0..=-100.0), &(0.0..=1.0)).clamp(0.0, 1.0);
    let resonance = controls.resonance.get_scaled(resonance_norm, &(0.0..=1.0));
    let lead_volume = controls
        .lead_volume
        .get_scaled(h.position.y, &(300.0..=400.0));
    if h.grab >= 1.0 {
        if let Some(drone_volume_angle) = h.rotation {
            let (init_drone_volume, init_drone_volume_angle) =
                *drone_grab_state.get_or_insert((preset.mix.drone, drone_volume_angle));
            let offset = drone_volume_angle - init_drone_volume_angle;
            let new_volume = (init_drone_volume + Volume(offset / 5.0)).clamped();
            preset.mix.drone = new_volume;
            controls.mix_drone_volume.send(dsp_tx, new_volume)?;
        }
    } else {
        *drone_grab_state = None;
    }
    controls.cutoff_note.send(dsp_tx, cutoff_note)?;
    controls.lead_volume.send(dsp_tx, lead_volume)?;
    controls.resonance.send(dsp_tx, resonance)?;
    ui_tx.send(UiUpdate::Filter(
        cutoff_note_norm * h.x_factor,
        resonance_norm,
    ))?;
    ui_tx.send(UiUpdate::LeadVolume(lead_volume))?;
    ui_tx.send(UiUpdate::StrumReady(strum_ready))?;
    Ok(())
}

fn on_pitch_hand(
    h: HandMessage,
    controls: &controls::Controls,
    preset: &mut Preset,
    dsp_tx: &mut Sender<dsp_thread::ParameterUpdate>,
    ui_tx: &mut Sender<UiUpdate>,
    guitar_gates: &mut [bool; 4],
) -> Result<(), anyhow::Error> {
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
    let note_number_height = controls::convert_range(h.position.y, &(350.0..=500.0), &(1.0..=4.0));
    let lead_volumes =
        [0.0, 1.0, 2.0, 3.0].map(|v| (note_number_height.clamp(1.0, 4.0) - v).clamp(0.0, 1.0));
    *guitar_gates = lead_volumes.map(|v| v > 0.0);
    let autotune = controls::convert_range(h.pinch, &(0.0..=1.0), &(0.0..=5.0)) as usize;
    let note = restricted_scale_window.autotune(raw_note, autotune);
    let chord = full_scale_window.autochord(note, &[0, 2, 4, 7]);
    let lead_offset = preset.lead_interval_f();
    let pluck_offset = preset.pluck_interval_f();
    let pitch_bend = controls
        .pitch_bend
        .get_scaled(h.velocity.x + h.velocity.z, &(-300.0..=300.0));
    let trumpet = controls
        .drone_trumpet
        .get_scaled(h.velocity.y.abs(), &(0.0..=250.0));
    for (control, value) in controls.lead.iter().zip(lead_volumes) {
        control.volume.send(dsp_tx, value)?;
    }
    for (i, note) in chord.iter().enumerate() {
        if let Some(note) = note {
            controls.lead[i].send_note(dsp_tx, &(*note + lead_offset))?;
            controls.strum[i].send_note(dsp_tx, &(*note + pluck_offset))?;
        }
    }
    controls.strum_drone.send_note(
        dsp_tx,
        &(preset.root_note_f() + pluck_offset + IntervalF(12.0)),
    )?;
    controls.pitch_bend.send(dsp_tx, pitch_bend)?;
    controls.drone_trumpet.send(dsp_tx, trumpet)?;
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
