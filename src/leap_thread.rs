use std::thread;

use anyhow::Result;
use crossbeam_channel::{Receiver, Sender};
use leaprs::*;
use nalgebra::{Vector2, Vector3};

use crate::{
    controls, dsp_thread,
    settings::{Handedness, Settings},
    ui::{self, UiUpdate},
};

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
        loop {
            if let Some(new_settings) = settings_rx.try_iter().last() {
                settings = new_settings;
            }
            if read_leap_and_update(&mut connection, &settings, &controls, &ui_tx, &dsp_tx).is_err()
            {
                // For the lack of better error handling, just assume
                // that the gui thread quit
                return;
            }
        }
    })
}

fn read_leap_and_update(
    connection: &mut Connection,
    settings: &Settings,
    controls: &controls::Controls,
    ui_tx: &Sender<UiUpdate>,
    dsp_tx: &Sender<dsp_thread::ParameterUpdate>,
) -> Result<()> {
    match connection.poll(100) {
        Ok(message) => {
            if let Event::Tracking(e) = message.event() {
                on_tracking_event(e, settings, controls, ui_tx, dsp_tx)?;
            }
            ui_tx.send(UiUpdate::ErrorReset)?;
        }
        Err(err) => {
            ui_tx.send(UiUpdate::Error(err.to_string()))?;
        }
    }
    Ok(())
}

fn on_tracking_event(
    e: TrackingEvent<'_>,
    settings: &Settings,
    controls: &controls::Controls,
    ui_tx: &Sender<UiUpdate>,
    dsp_tx: &Sender<dsp_thread::ParameterUpdate>,
) -> Result<()> {
    let preset = &settings.current_preset;
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

    let x_mirror = match handedness {
        Handedness::LeftHanded => -1.0,
        Handedness::RightHanded => 1.0,
    };

    // Guitar gates
    let mut guitar_gates = [false, false, false, false];
    if let Some(hand) = pitch_hand {
        let position = hand.palm().position();
        let velocity = hand.palm().velocity();

        let note_range = preset.note_range_f();

        // Position the virtual antenna and the pitch hand relative to it
        let antenna_coord = Vector2::new(x_mirror * 400.0, -200.0);
        let pitch_coord_mm = antenna_coord - Vector2::new(position.x(), position.z());
        // Hand position in semitones
        let pitch_coord_semitones = pitch_coord_mm / 15.0;
        let pitch_coord_semitones = Vector2::new(-pitch_coord_semitones.x, pitch_coord_semitones.y);
        // Hand distance from the antenna in semitones
        let pitch_distance_semitones = pitch_coord_semitones.norm();
        // Convert to note, based on the current note range
        let raw_note = (*note_range.end() - pitch_distance_semitones)
            .clamp(*note_range.start(), *note_range.end());

        // Floating number of played chords. 2.5 means 2 notes and one half volume.
        let note_number_height =
            controls::convert_range(position.y(), &(350.0..=500.0), &(1.0..=4.0));
        let lead_volumes =
            [0.0, 1.0, 2.0, 3.0].map(|v| (note_number_height.clamp(1.0, 4.0) - v).clamp(0.0, 1.0));

        guitar_gates = lead_volumes.map(|v| v >= 0.2);

        // Autotune amount
        let autotune =
            controls::convert_range(hand.pinch_strength(), &(0.0..=1.0), &(0.0..=5.0)) as usize;

        // Lead note, autotuned
        let note = restricted_scale_window.autotune(raw_note, autotune);

        // Autochord, from the autotuned note so that the chord itself is autotuned
        let chord = full_scale_window.autochord(note, &[0, 2, 4, 7]);

        let pluck_offset = 12.0 * (preset.guitar_octave - preset.octave) as f32;

        let pitch_bend = controls
            .pitch_bend
            .get_scaled(velocity.x() + velocity.y(), &(-300.0..=300.0));

        // Send to dsp
        for (control, value) in controls.lead.iter().zip(lead_volumes) {
            control.volume.send(dsp_tx, value)?;
        }

        for (i, note) in chord.iter().enumerate() {
            if let Some(note) = note {
                controls.lead[i].note.send(dsp_tx, *note)?;
                controls.strum[i].note.send(dsp_tx, *note + pluck_offset)?;
            }
        }

        controls.pitch_bend.send(dsp_tx, pitch_bend)?;

        // Send to UI
        ui_tx.send(UiUpdate::AutotuneAmount(autotune))?;
        ui_tx.send(UiUpdate::LeadChordNotes(
            chord.map(|c| c.unwrap_or_default()),
        ))?;
        ui_tx.send(UiUpdate::LeadChordVolumes(lead_volumes))?;
        ui_tx.send(UiUpdate::RawNote(raw_note))?;
        ui_tx.send(UiUpdate::PitchXY(
            pitch_coord_semitones.x,
            pitch_coord_semitones.y,
        ))?;
        ui_tx.send(UiUpdate::ChordsNumber(note_number_height))?;
    }
    if let Some(hand) = volume_hand {
        let position = hand.palm().position();

        let palm_normal = Vector3::from(hand.palm().normal().array());
        let palm_dot = palm_normal.dot(&Vector3::y());
        if hand.pinch_strength() > 0.9 {
            for (i, string) in &mut controls.strum.iter().enumerate() {
                string
                    .pluck
                    .send(dsp_tx, palm_dot > 0.0 + (i as f32) * 0.2 && guitar_gates[i]);
            }
        }
        let pluck_mute = controls.pluck_mute.get_scaled(palm_dot, &(-1.0..=0.0));
        let cutoff_note = controls
            .cutoff_note
            .get_scaled(-x_mirror * position.x(), &(50.0..=200.0));
        let lead_volume = controls
            .lead_volume
            .get_scaled(position.y(), &(300.0..=400.0));
        let resonance = controls
            .resonance
            .get_scaled(position.z(), &(100.0..=-100.0));

        // Send to dsp
        controls.pluck_mute.send(dsp_tx, pluck_mute)?;
        controls.cutoff_note.send(dsp_tx, cutoff_note)?;
        controls.lead_volume.send(dsp_tx, lead_volume)?;
        controls.resonance.send(dsp_tx, resonance)?;

        // Send to UI
        ui_tx.send(UiUpdate::Filter(cutoff_note, resonance))?;
        ui_tx.send(UiUpdate::LeadVolume(lead_volume))?;
    }

    ui_tx.send(UiUpdate::HasHands(
        hands.iter().any(|h| h.hand_type() == HandType::Left),
        hands.iter().any(|h| h.hand_type() == HandType::Right),
    ))?;

    Ok(())
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
