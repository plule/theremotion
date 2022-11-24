use std::thread;

use anyhow::Result;
use crossbeam_channel::{Receiver, Sender};
use leaprs::*;
use nalgebra::{Vector2, Vector3};

use crate::{
    controls, dsp_thread,
    settings::{Preset, Settings},
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
            let preset = &settings.current_preset;
            if read_leap_and_update(&mut connection, preset, &controls, &ui_tx, &dsp_tx).is_err() {
                // For the lack of better error handling, just assume
                // that the gui thread quit
                return;
            }
        }
    })
}

fn read_leap_and_update(
    connection: &mut Connection,
    preset: &Preset,
    controls: &controls::Controls,
    ui_tx: &Sender<UiUpdate>,
    dsp_tx: &Sender<dsp_thread::ParameterUpdate>,
) -> Result<()> {
    match connection.poll(100) {
        Ok(message) => {
            if let Event::Tracking(e) = message.event() {
                on_tracking_event(e, preset, controls, ui_tx, dsp_tx)?;
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
    preset: &Preset,
    controls: &controls::Controls,
    ui_tx: &Sender<UiUpdate>,
    dsp_tx: &Sender<dsp_thread::ParameterUpdate>,
) -> Result<()> {
    let full_scale_window = preset.full_scale_floating_window();
    let restricted_scale_window = preset.restricted_scale_floating_window();
    let hands = e.hands();
    let left_hand = hands.iter().find(|h| h.hand_type() == HandType::Left);
    let right_hand = hands.iter().find(|h| h.hand_type() == HandType::Right);
    ui_tx.send(UiUpdate::HasHands((
        left_hand.is_some(),
        right_hand.is_some(),
    )))?;
    let mut strums_enabled = [false, false, false, false];
    if let Some(hand) = left_hand {
        let position = hand.palm().position();
        let velocity = hand.palm().velocity();

        let antenna_coord = Vector2::new(-400.0, -200.0);
        let pitch_coord = Vector2::new(position.x(), position.z());
        let dist = (pitch_coord - antenna_coord).norm();
        let raw_note = controls::convert_range(dist, 500.0..=0.0, &preset.note_range_f());

        // Determine the played chord
        let y = position.y();
        let lead_volumes = [0, 1, 2, 3].map(|i| {
            if i == 0 {
                return 1.0;
            }
            let note = &controls.lead[i];
            let from = 300.0 + 50.0 * i as f32;
            let to = 350.0 + 50.0 * i as f32;
            note.volume.get_scaled(y, from..=to)
        });

        strums_enabled = lead_volumes.map(|v| v >= 0.5);

        let autotune =
            controls::convert_range(hand.pinch_strength(), 0.0..=1.0, &(0.0..=5.0)) as usize;

        // In any case, assign all the notes
        let note = restricted_scale_window.autotune(raw_note, autotune);

        let chord = full_scale_window.autochord(note, &[0, 2, 4, 7]);

        let pluck_offset = 12.0 * (preset.guitar_octave - preset.octave) as f32;

        let pitch_bend = controls
            .pitch_bend
            .get_scaled(velocity.x() + velocity.y(), -300.0..=300.0);

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
    }
    if let Some(hand) = right_hand {
        let position = hand.palm().position();

        let palm_normal = Vector3::from(hand.palm().normal().array());
        let palm_dot = palm_normal.dot(&Vector3::y());
        if hand.pinch_strength() > 0.9 {
            for (i, string) in &mut controls.strum.iter().enumerate() {
                string.pluck.send(
                    dsp_tx,
                    palm_dot > 0.0 + (i as f32) * 0.2 && strums_enabled[i],
                );
            }
        }
        let pluck_mute = controls.pluck_mute.get_scaled(palm_dot, -1.0..=0.0);
        let cutoff_note = controls.cutoff_note.get_scaled(position.x(), 50.0..=200.0);
        let lead_volume = controls.lead_volume.get_scaled(position.y(), 300.0..=400.0);
        let resonance = controls.resonance.get_scaled(position.z(), 100.0..=-100.0);

        // Send to dsp
        controls.pluck_mute.send(dsp_tx, pluck_mute)?;
        controls.cutoff_note.send(dsp_tx, cutoff_note)?;
        controls.lead_volume.send(dsp_tx, lead_volume)?;
        controls.resonance.send(dsp_tx, resonance)?;

        // Send to UI
        ui_tx.send(UiUpdate::Filter(cutoff_note, resonance))?;
        ui_tx.send(UiUpdate::LeadVolume(lead_volume))?;
    }
    Ok(())
}
