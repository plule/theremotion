use std::{
    sync::{Arc, Mutex},
    thread,
};

use faust_state::StateHandle;
use leaprs::*;

fn convert_range(value: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    ((((value - in_min) * (out_max - out_min)) / (in_max - in_min)) + out_min)
        .clamp(out_min, out_max)
}

pub fn start_leap_worker(dsp: Arc<Mutex<StateHandle>>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut connection =
            Connection::create(ConnectionConfig::default()).expect("Failed to connect");
        connection.open().expect("Failed to open the connection");
        let mut controls = crate::dsp::Controls::default();
        loop {
            if let Ok(message) = connection.poll(1000) {
                if let Event::Tracking(e) = message.event() {
                    {
                        let mut dsp = dsp.lock().expect("DSP thread poisened");
                        controls.read(&mut dsp);
                    }

                    for hand in e.hands() {
                        match hand.hand_type() {
                            HandType::Left => {
                                let position = hand.palm().position();

                                controls.note =
                                    convert_range(position.y(), 100.0, 600.0, 34.0, 72.0);
                            }
                            HandType::Right => {
                                let position = hand.palm().position();

                                controls.cutoff_note =
                                    convert_range(position.x(), -100.0, 100.0, -20.0, 20.0);

                                controls.volume =
                                    convert_range(position.y(), 200.0, 300.0, -96.0, 0.0);
                                controls.resonance =
                                    convert_range(position.z(), 100.0, -100.0, 1.0, 30.0);
                            }
                        }
                    }

                    {
                        let mut dsp = dsp.lock().expect("DSP thread poisened");
                        controls.write(&mut dsp);
                    }
                }
            }
        }
    })
}
