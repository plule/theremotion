use std::{f32::consts::PI, thread};

use anyhow::Result;
use crossbeam_channel::{Receiver, Sender};
use leaprs::*;
use nalgebra::{UnitQuaternion, Vector2, Vector3};

use crate::{
    controls, dsp_thread,
    settings::{Handedness, Settings},
    solfege::{IntervalF, Volume},
    ui::{self, UiUpdate},
};

const HALF_PI: f32 = PI / 2.0;

/// Start the leap motion thread
pub fn run(
    controls: controls::Controls,
    settings_rx: Receiver<Settings>,
    ui_tx: Sender<ui::UiUpdate>,
    dsp_tx: Sender<dsp_thread::ParameterUpdate>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut connection =
            Connection::create(ConnectionConfig::default()).expect("Failed to connect");
        connection.open().expect("Failed to open the connection");
        let mut settings = Settings::default();
        let mut reader = LeapReader::new(&controls, &ui_tx, &dsp_tx);
        loop {
            if let Some(new_settings) = settings_rx.try_iter().last() {
                settings = new_settings;
            }
            if reader.read_and_update(&settings, &mut connection).is_err() {
                // For the lack of better error handling, just assume
                // that the gui thread quit
                return;
            }
        }
    })
}

struct LeapReader<'a> {
    controls: &'a controls::Controls,
    ui_tx: &'a Sender<UiUpdate>,
    dsp_tx: &'a Sender<dsp_thread::ParameterUpdate>,

    drone_grab_state: Option<(Volume, f32)>,
}

impl<'a> LeapReader<'a> {
    fn new(
        controls: &'a controls::Controls,
        ui_tx: &'a Sender<UiUpdate>,
        dsp_tx: &'a Sender<dsp_thread::ParameterUpdate>,
    ) -> Self {
        Self {
            controls,
            ui_tx,
            dsp_tx,
            drone_grab_state: None,
        }
    }
}

impl<'a> LeapReader<'a> {
    fn read_and_update(&mut self, settings: &Settings, connection: &mut Connection) -> Result<()> {
        match connection.poll(100) {
            Ok(message) => {
                if let Event::Tracking(e) = message.event() {
                    self.on_tracking_event(e, settings)?;
                }
                self.ui_tx.send(UiUpdate::ErrorReset)?;
            }
            Err(err) => {
                self.ui_tx.send(UiUpdate::Error(err.to_string()))?;
            }
        }
        Ok(())
    }

    fn on_tracking_event(&mut self, e: TrackingEvent<'_>, settings: &Settings) -> Result<()> {
        let mut preset = settings.current_preset.clone();
        let handedness = &settings.system.handedness;

        // Retrieve the current scale 2 by 2 windows
        let full_scale_window = preset.full_scale_floating_window();
        let restricted_scale_window = preset.restricted_scale_floating_window();

        // List of visible hands
        let hands = e.hands();
        let pitch_hand = hands
            .iter()
            .find(|h| h.hand_type() == pitch_hand_type(handedness));
        let volume_hand = hands
            .iter()
            .find(|h| h.hand_type() == volume_hand_type(handedness));

        // Guitar gates
        let mut guitar_gates = [false, false, false, false];
        if let Some(hand) = pitch_hand {
            let position = hand.position_from_body();
            let velocity = hand.velocity_from_body();

            let note_range = preset.note_range_f();

            // Position the virtual antenna and the pitch hand relative to it
            let antenna_coord = Vector2::new(400.0, -200.0);
            let pitch_coord_mm = antenna_coord - Vector2::new(position.x, position.z);
            // Hand position in semitones
            let pitch_coord_semitones = pitch_coord_mm / 15.0;
            let pitch_coord_semitones =
                Vector2::new(-pitch_coord_semitones.x, pitch_coord_semitones.y);
            // Hand distance from the antenna in semitones
            let pitch_distance_semitones = IntervalF(pitch_coord_semitones.norm());
            // Convert to note, based on the current note range
            let raw_note = (*note_range.end() - pitch_distance_semitones)
                .clamp(*note_range.start(), *note_range.end());

            // Floating number of played chords. 2.5 means 2 notes and one half volume.
            let note_number_height =
                controls::convert_range(position.y, &(350.0..=500.0), &(1.0..=4.0));
            let lead_volumes = [0.0, 1.0, 2.0, 3.0]
                .map(|v| (note_number_height.clamp(1.0, 4.0) - v).clamp(0.0, 1.0));

            guitar_gates = lead_volumes.map(|v| v > 0.0);

            // Autotune amount
            let autotune =
                controls::convert_range(hand.pinch_strength(), &(0.0..=1.0), &(0.0..=5.0)) as usize;

            // Base note, autotuned
            let note = restricted_scale_window.autotune(raw_note, autotune);

            // Autochord, from the autotuned note so that the chord itself is autotuned
            let chord = full_scale_window.autochord(note, &[0, 2, 4, 7]);

            let lead_offset = preset.lead_interval_f();
            let pluck_offset = preset.pluck_interval_f();

            let pitch_bend = self
                .controls
                .pitch_bend
                .get_scaled(velocity.x + velocity.z, &(-300.0..=300.0));

            let trumpet = self
                .controls
                .drone_trumpet
                .get_scaled(velocity.y.abs(), &(0.0..=250.0));

            // Send to dsp
            for (control, value) in self.controls.lead.iter().zip(lead_volumes) {
                control.volume.send(self.dsp_tx, value)?;
            }

            for (i, note) in chord.iter().enumerate() {
                if let Some(note) = note {
                    self.controls.lead[i].send_note(self.dsp_tx, &(*note + lead_offset))?;
                    self.controls.strum[i].send_note(self.dsp_tx, &(*note + pluck_offset))?;
                }
            }
            self.controls.strum_drone.send_note(
                self.dsp_tx,
                &(preset.root_note_f() + pluck_offset + IntervalF(12.0)),
            )?;

            self.controls.pitch_bend.send(self.dsp_tx, pitch_bend)?;
            self.controls.drone_trumpet.send(self.dsp_tx, trumpet)?;

            // Send to UI
            self.ui_tx.send(UiUpdate::AutotuneAmount(autotune))?;
            self.ui_tx.send(UiUpdate::LeadChordNotes(
                chord.map(|c| c.unwrap_or_default()),
            ))?;
            self.ui_tx.send(UiUpdate::LeadChordVolumes(lead_volumes))?;
            self.ui_tx.send(UiUpdate::RawNote(raw_note))?;
            self.ui_tx.send(UiUpdate::PitchXY(
                pitch_coord_semitones.x * hand.x_factor(),
                pitch_coord_semitones.y,
            ))?;
            self.ui_tx
                .send(UiUpdate::ChordsNumber(note_number_height))?;
            self.ui_tx.send(UiUpdate::TrumpetStrength(trumpet))?;
        }
        if let Some(hand) = volume_hand {
            let position = hand.position_from_body();
            let rotation = hand.rotation_from_body();
            let strum_ready = hand.pinch_strength() > 0.9;
            if let Some(rotation) = rotation {
                if strum_ready {
                    for (i, string) in &mut self.controls.strum.iter().enumerate() {
                        string.pluck.send(
                            self.dsp_tx,
                            rotation > HALF_PI + (i as f32) * 0.2 && guitar_gates[i],
                        );
                    }
                    self.controls.strum_drone.pluck.send(
                        self.dsp_tx,
                        preset.drone.pluck_drone && rotation > HALF_PI + 0.3,
                    );
                }
            }

            let pluck_mute = self
                .controls
                .pluck_mute
                .get_scaled(rotation.unwrap_or_default(), &(0.0..=(HALF_PI - 0.2)));

            let cutoff_note_norm =
                controls::convert_range(position.x, &(50.0..=200.0), &(-1.0..=1.0))
                    .clamp(-1.0, 1.0);
            let cutoff_note = self
                .controls
                .cutoff_note
                .get_scaled(cutoff_note_norm, &(-1.0..=1.0));
            let resonance_norm =
                controls::convert_range(position.z, &(100.0..=-100.0), &(0.0..=1.0))
                    .clamp(0.0, 1.0);
            let resonance = self
                .controls
                .resonance
                .get_scaled(resonance_norm, &(0.0..=1.0));
            let lead_volume = self
                .controls
                .lead_volume
                .get_scaled(position.y, &(300.0..=400.0));

            // Send to dsp
            self.controls.pluck_mute.send(self.dsp_tx, pluck_mute)?;
            self.controls.cutoff_note.send(self.dsp_tx, cutoff_note)?;
            self.controls.lead_volume.send(self.dsp_tx, lead_volume)?;
            self.controls.resonance.send(self.dsp_tx, resonance)?;

            // Send to UI
            self.ui_tx.send(UiUpdate::Filter(
                cutoff_note_norm * hand.x_factor(),
                resonance_norm,
            ))?;
            self.ui_tx.send(UiUpdate::LeadVolume(lead_volume))?;
            self.ui_tx.send(UiUpdate::StrumReady(strum_ready))?;
        }

        if let (Some(pitch_hand), Some(volume_hand)) = (pitch_hand, volume_hand) {
            if pitch_hand.grab_strength() > 0.95 && volume_hand.grab_strength() > 0.95 {
                if let Some(drone_volume_angle) = volume_hand.rotation_from_body() {
                    let (init_drone_volume, init_drone_volume_angle) = *self
                        .drone_grab_state
                        .get_or_insert((preset.mix.drone, drone_volume_angle));
                    let offset = drone_volume_angle - init_drone_volume_angle;
                    let new_volume = (init_drone_volume + Volume(offset / 5.0)).clamped();
                    preset.mix.drone = new_volume;
                    self.controls
                        .mix_drone_volume
                        .send(self.dsp_tx, new_volume)?;
                }
            } else {
                self.drone_grab_state = None;
            }
        }

        self.ui_tx.send(UiUpdate::HasHands(
            hands.iter().any(|h| h.hand_type() == HandType::Left),
            hands.iter().any(|h| h.hand_type() == HandType::Right),
        ))?;

        if preset != settings.current_preset {
            self.ui_tx.send(UiUpdate::Settings(preset))?;
        }

        Ok(())
    }
}

fn pitch_hand_type(handedness: &Handedness) -> leaprs::HandType {
    match handedness {
        Handedness::RightHanded => leaprs::HandType::Right,
        Handedness::LeftHanded => leaprs::HandType::Left,
    }
}

fn volume_hand_type(handedness: &Handedness) -> leaprs::HandType {
    match handedness {
        Handedness::RightHanded => leaprs::HandType::Left,
        Handedness::LeftHanded => leaprs::HandType::Right,
    }
}

/// Normalized body direction trait.
/// x direction is from the center of the body to the outside
trait DirectionFromBody {
    /// Factor applied to the x axis to normalize its direction
    fn x_factor(&self) -> f32;
    /// Palm position where the left/right position is normalized:
    /// positive x means arms more open.
    fn position_from_body(&self) -> Vector3<f32>;
    /// Palm velocity where the left/right position is normalized:
    /// positive x means arms more open.
    fn velocity_from_body(&self) -> Vector3<f32>;
    /// Hand twist angle
    fn rotation_from_body(&self) -> Option<f32>;
}

impl DirectionFromBody for Hand<'_> {
    fn x_factor(&self) -> f32 {
        match self.hand_type() {
            // The left hand goes away from the body in the negative x
            HandType::Left => -1.0,
            // The right hand goes away from the body in the positive x
            HandType::Right => 1.0,
        }
    }

    fn position_from_body(&self) -> Vector3<f32> {
        let position = self.palm().position();
        Vector3::new(self.x_factor() * position.x(), position.y(), position.z())
    }

    fn velocity_from_body(&self) -> Vector3<f32> {
        let velocity = self.palm().velocity();
        Vector3::new(self.x_factor() * velocity.x(), velocity.y(), velocity.z())
    }

    fn rotation_from_body(&self) -> Option<f32> {
        let rotation = self.arm().rotation();
        let rotation = UnitQuaternion::from_quaternion(nalgebra::Quaternion::new(
            rotation.w(),
            rotation.x(),
            rotation.y(),
            rotation.z(),
        ));
        let angle = -rotation.euler_angles().2 * self.x_factor();
        if angle < PI && angle > -HALF_PI {
            Some(angle)
        } else {
            None
        }
    }
}
