use std::{
    sync::{mpsc::Receiver, Arc, Mutex},
    thread,
};

use faust_state::StateHandle;
use leaprs::*;

use crate::{dsp::Controls, settings::Settings};

/// Start the leap motion thread
pub fn start_leap_worker(
    dsp: Arc<Mutex<StateHandle>>,
    settings_rx: Receiver<Settings>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut connection =
            Connection::create(ConnectionConfig::default()).expect("Failed to connect");
        connection.open().expect("Failed to open the connection");
        let mut controls: Controls = { dsp.lock().as_deref().unwrap().into() };
        let mut settings = Settings::default();
        loop {
            if let Ok(message) = connection.poll(1000) {
                if let Event::Tracking(e) = message.event() {
                    {
                        let mut dsp = dsp.lock().expect("DSP thread poisened");
                        controls.receive(&mut dsp);
                    }

                    if let Some(new_settings) = settings_rx.try_iter().last() {
                        settings = new_settings;
                    }

                    for hand in e.hands() {
                        match hand.hand_type() {
                            HandType::Left => {
                                let position = hand.palm().position();

                                controls.detune.set_scaled(position.x(), -200.0..=0.0);
                                controls
                                    .note
                                    .set_scaled(position.y(), 100.0..=600.0, &settings);
                                controls.supersaw.set_scaled(position.z(), 100.0..=-100.0);
                            }
                            HandType::Right => {
                                let position = hand.palm().position();

                                controls.cutoff_note.set_scaled(position.x(), 0.0..=200.0);
                                controls.volume.set_scaled(position.y(), 200.0..=300.0);
                                controls.resonance.set_scaled(position.z(), 100.0..=-100.0);
                                controls
                                    .sub_volume
                                    .set_scaled(hand.grab_angle(), 0.0..=std::f32::consts::PI);
                            }
                        }
                    }

                    {
                        let mut dsp = dsp.lock().expect("DSP thread poisened");
                        controls.send(&mut dsp);
                    }
                }
            }
        }
    })
}
