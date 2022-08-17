use std::thread;

use crossbeam_channel::{Receiver, Sender};
use faust_state::StateHandle;
use leaprs::*;
use nalgebra::Vector3;

use crate::{
    controls,
    controls::{smoothstairs, ControlTrait},
    settings::Settings,
};

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
            match connection.poll(1000) {
                Ok(message) => {
                    if let Event::Tracking(e) = message.event() {
                        if let Some(new_settings) = settings_rx.try_iter().last() {
                            settings = new_settings;
                        }

                        let hands = e.hands();

                        if hands.is_empty() {
                            controls.warning = Some("No hand in view".to_string());
                        }

                        for hand in hands {
                            match hand.hand_type() {
                                HandType::Left => {
                                    let position = hand.palm().position();

                                    match hand.get_finger_positions() {
                                        (false, false, false, false, false) => {}
                                        (false, true, false, false, true) => {
                                            controls
                                                .drone_note
                                                .set_scaled(position.y(), 100.0..=600.0);
                                            controls.drone_note.value = smoothstairs(
                                                controls.drone_note.value,
                                                5,
                                                settings.scale_notes(),
                                            );
                                        }
                                        _ => {
                                            controls
                                                .detune
                                                .set_scaled(position.x(), -200.0..=-50.0);
                                            let note_input_range = 100.0..=600.0;
                                            controls.note.set_scaled(
                                                position.y(),
                                                note_input_range.to_owned(),
                                                hand.pinch_strength(),
                                                0.0..=1.0,
                                                &settings,
                                            );
                                            controls
                                                .supersaw
                                                .set_scaled(position.z(), 100.0..=-100.0);
                                        }
                                    }
                                }
                                HandType::Right => {
                                    let position = hand.palm().position();

                                    match hand.get_finger_positions() {
                                        (false, false, false, false, false) => {}
                                        (false, true, false, false, true) => {
                                            controls
                                                .drone_volume
                                                .set_scaled(position.y(), 300.0..=400.0);
                                        }
                                        _ => {
                                            if hand.pinch_strength() > 0.9 {
                                                let palm_normal =
                                                    Vector3::from(hand.palm().normal().array());
                                                let palm_dot = palm_normal.dot(&Vector3::y());
                                                controls.pluck.value = palm_dot > 0.0;
                                            }
                                            controls
                                                .cutoff_note
                                                .set_scaled(position.x(), 50.0..=200.0);
                                            controls.volume.set_scaled(position.y(), 300.0..=400.0);
                                            controls
                                                .resonance
                                                .set_scaled(position.z(), 100.0..=-100.0);
                                            controls
                                                .pluck_position
                                                .set_scaled(position.x(), 50.0..=200.0);
                                        }
                                    }
                                }
                            }
                        }

                        controls.send(&mut dsp);
                    }
                }
                Err(err) => {
                    controls.error = Some(err.to_string());
                }
            }
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
