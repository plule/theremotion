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
        loop {
            if let Ok(message) = connection.poll(1000) {
                if let Event::Tracking(e) = message.event() {
                    let mut volume;
                    let mut note;
                    let mut cutoff_note;
                    let mut res;
                    {
                        let dsp = dsp.lock().expect("DSP thread poisened");
                        volume = *dsp.get_by_path("volume").expect("Failed to read volume");
                        note = *dsp.get_by_path("note").expect("Failed to read freq");
                        cutoff_note = *dsp
                            .get_by_path("cutoff_note")
                            .expect("Failed to read cutoff");
                        res = *dsp.get_by_path("res").expect("Failed to read res");
                    }

                    for hand in e.hands() {
                        match hand.hand_type() {
                            HandType::Left => {
                                let position = hand.palm().position();

                                note = convert_range(position.y(), 100.0, 600.0, 34.0, 72.0);
                            }
                            HandType::Right => {
                                let position = hand.palm().position();

                                cutoff_note =
                                    convert_range(position.x(), -100.0, 100.0, -20.0, 20.0);

                                volume = convert_range(position.y(), 200.0, 300.0, -96.0, 0.0);
                                res = convert_range(position.z(), 100.0, -100.0, 1.0, 30.0);
                            }
                        }
                    }

                    {
                        let mut dsp = dsp.lock().expect("DSP thread poisened");
                        dsp.set_by_path("volume", volume)
                            .expect("Failed to set volume.");
                        dsp.set_by_path("note", note).expect("Failed to set freq.");
                        dsp.set_by_path("cutoff_note", cutoff_note)
                            .expect("Failed to set cutoff");
                        dsp.set_by_path("res", res)
                            .expect("Failed to set resonnance");
                        dsp.send();
                    }
                }
            }
        }
    })
}
