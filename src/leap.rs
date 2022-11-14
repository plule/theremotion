use std::thread;

use crossbeam_channel::{Receiver, Sender};
use faust_state::StateHandle;
use leaprs::*;
use nalgebra::Vector3;

use crate::{controls, controls::ControlTrait, settings::Settings};

/// Start the leap motion thread
pub fn start_leap_worker(
    mut dsp: StateHandle,
    settings_rx: Receiver<Settings>,
    dsp_controls_tx: Sender<controls::Controls>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut connection =
            Connection::create(ConnectionConfig::default()).expect("Failed to connect");
        connection.open().expect("Failed to open the connection");
        let mut controls: controls::Controls = (&dsp).into();
        let mut settings = Settings::default();
        dsp_controls_tx.send(controls.clone()).unwrap();
        loop {
            controls.warning = None;
            controls.error = None;
            if let Some(new_settings) = settings_rx.try_iter().last() {
                settings = new_settings;
            }
            let preset = &settings.current_preset;
            controls.update_from_preset(preset);
            match connection.poll(100) {
                Ok(message) => {
                    if let Event::Tracking(e) = message.event() {
                        let full_scale =
                            crate::music_theory::build_scale(preset.root_note(), preset.scale);

                        let hands = e.hands();

                        let left_hand = hands.iter().find(|h| h.hand_type() == HandType::Left);
                        let right_hand = hands.iter().find(|h| h.hand_type() == HandType::Right);
                        controls.has_hands = (left_hand.is_some(), right_hand.is_some());

                        let mut strums_enabled = [false, false, false, false];

                        if let Some(hand) = left_hand {
                            let position = hand.palm().position();
                            let velocity = hand.palm().velocity();

                            let note_input_range = 100.0..=600.0;

                            controls.raw_note = controls::convert_range(
                                position.y(),
                                note_input_range.to_owned(),
                                &preset.note_range_f(),
                            );

                            // Determine the played chord
                            let z = position.z();
                            controls.lead[0].volume.value = 1.0;
                            controls
                                .lead
                                .iter_mut()
                                .enumerate()
                                .skip(1)
                                .for_each(|(i, note)| {
                                    let from = 100.0 - 50.0 * i as f32;
                                    let to = 50.0 - 50.0 * i as f32;
                                    note.volume.set_scaled(z, from..=to);
                                });

                            strums_enabled = controls.lead.clone().map(|c| c.volume.value >= 0.5);

                            controls.autotune = controls::convert_range(
                                hand.pinch_strength(),
                                0.0..=1.0,
                                &(0.0..=5.0),
                            ) as usize;

                            // In any case, assign all the notes
                            let note = crate::music_theory::autotune(
                                controls.raw_note,
                                controls.autotune,
                                preset.scale_notes(),
                            );

                            let chord = [
                                Some(note),
                                crate::music_theory::auto_chord(note, &full_scale, 2),
                                crate::music_theory::auto_chord(note, &full_scale, 4),
                                crate::music_theory::auto_chord(note, &full_scale, 7),
                            ];

                            let pluck_offset = 12.0 * (preset.guitar_octave - preset.octave) as f32;

                            for (i, note) in chord.iter().enumerate() {
                                if let Some(note) = note {
                                    controls.lead[i].note.value = *note;
                                    controls.strum[i].note.value = *note + pluck_offset;
                                }
                            }

                            controls
                                .pitch_bend
                                .set_scaled(velocity.x() + velocity.z(), -300.0..=300.0);
                        }

                        if let Some(hand) = right_hand {
                            let position = hand.palm().position();

                            let palm_normal = Vector3::from(hand.palm().normal().array());
                            let palm_dot = palm_normal.dot(&Vector3::y());
                            if hand.pinch_strength() > 0.9 {
                                for (i, string) in &mut controls.strum.iter_mut().enumerate() {
                                    string.pluck.value =
                                        palm_dot > 0.0 + (i as f32) * 0.2 && strums_enabled[i];
                                }
                            }
                            controls.pluck_mute.set_scaled(palm_dot, -1.0..=0.0);
                            controls.cutoff_note.set_scaled(position.x(), 50.0..=200.0);
                            controls.lead_volume.set_scaled(position.y(), 300.0..=400.0);
                            controls.resonance.set_scaled(position.z(), 100.0..=-100.0);
                        }
                    }
                }
                Err(err) => {
                    controls.error = Some(err.to_string());
                }
            }
            controls.send(&mut dsp);
            let stopped = dsp_controls_tx.send(controls.clone()).is_err();
            if stopped {
                return;
            }
        }
    })
}
