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
            controls.update_mix(&settings);
            match connection.poll(100) {
                Ok(message) => {
                    if let Event::Tracking(e) = message.event() {
                        let full_scale =
                            crate::music_theory::build_scale(settings.root_note(), settings.scale);

                        if let Some(drone) = settings.drone {
                            controls.drone_note.value = drone.into_byte() as f32;
                            controls.drone_volume.value = 0.2;
                        } else {
                            controls.drone_volume.value = 0.0;
                        }

                        let hands = e.hands();

                        if hands.is_empty() {
                            controls.warning = Some("No hand in view".to_string());
                        }

                        let left_hand = hands.iter().find(|h| h.hand_type() == HandType::Left);
                        let right_hand = hands.iter().find(|h| h.hand_type() == HandType::Right);

                        if let Some(hand) = left_hand {
                            let position = hand.palm().position();
                            let velocity = hand.palm().velocity();

                            let note_input_range = 100.0..=600.0;

                            controls.raw_note = controls::convert_range(
                                position.y(),
                                note_input_range.to_owned(),
                                &settings.note_range_f(),
                            );

                            // Depending on finger position,
                            // either assign autotuned chord
                            // or single voice
                            match hand.get_finger_positions() {
                                (thumb, index, middle, false, false) => {
                                    // Chord position
                                    controls.autotune = 4;
                                    controls.lead[1].volume.value = if thumb { 1.0 } else { 0.0 };
                                    controls.lead[2].volume.value = if index { 1.0 } else { 0.0 };
                                    controls.lead[3].volume.value = if middle { 1.0 } else { 0.0 };
                                }
                                _ => {
                                    // Monophonic position
                                    controls.autotune = controls::convert_range(
                                        hand.pinch_strength(),
                                        0.0..=1.0,
                                        &(0.0..=5.0),
                                    )
                                        as usize;
                                    controls.lead[0].volume.value = 1.0;
                                    for chord in controls.lead.iter_mut().skip(1) {
                                        chord.volume.value = 0.0;
                                    }
                                }
                            }

                            // In any case, assign all the notes
                            let note = crate::music_theory::autotune(
                                controls.raw_note,
                                controls.autotune,
                                settings.scale_notes(),
                            );

                            let chord = [
                                Some(note),
                                crate::music_theory::auto_chord(note, &full_scale, 2), // thumb
                                crate::music_theory::auto_chord(note, &full_scale, 6), // index
                                crate::music_theory::auto_chord(note, &full_scale, 3), // middle
                            ];

                            for (i, note) in chord.iter().enumerate() {
                                if let Some(note) = note {
                                    controls.lead[i].note.value = *note;
                                }
                            }

                            let pluck_octave_offset = settings.guitar_octave - settings.octave;
                            controls.pluck_note.value = note + 12.0 * pluck_octave_offset as f32;
                            controls
                                .pitch_bend
                                .set_scaled(velocity.x() + velocity.z(), -300.0..=300.0);
                        }

                        if let Some(hand) = right_hand {
                            let position = hand.palm().position();

                            let palm_normal = Vector3::from(hand.palm().normal().array());
                            let palm_dot = palm_normal.dot(&Vector3::y());
                            if hand.pinch_strength() > 0.9 {
                                controls.pluck.value = palm_dot > 0.0;
                            }
                            controls.pluck_mute.set_scaled(palm_dot, -1.0..=0.0);
                            controls.pluck_wah.set_scaled(palm_dot, 0.0..=1.0);
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
            dsp_controls_tx.send(controls.clone()).unwrap();
        }
    })
}

trait FingerPositions {
    fn get_finger_positions(&self) -> (bool, bool, bool, bool, bool);
}

impl FingerPositions for Hand<'_> {
    fn get_finger_positions(&self) -> (bool, bool, bool, bool, bool) {
        (
            self.thumb().is_extended(),
            self.index().is_extended(),
            self.middle().is_extended(),
            self.ring().is_extended(),
            self.pinky().is_extended(),
        )
    }
}
