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

fn midi_to_freq(midi: f32) -> f32 {
    440.0 * 2.0_f32.powf((midi - 69.0) / 12.0)
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
                    let mut freq;
                    {
                        let dsp = dsp.lock().expect("DSP thread poisened");
                        volume = *dsp.get_by_path("volume").expect("Failed to read volume");
                        freq = *dsp.get_by_path("freq").expect("Failed to read volume");
                    }

                    for hand in e.hands() {
                        match hand.hand_type() {
                            HandType::Left => {
                                let y = hand.palm().position().y();

                                let midi_note = convert_range(y, 100.0, 500.0, 48.0, 72.0);
                                freq = midi_to_freq(midi_note);
                            }
                            HandType::Right => {
                                let y = hand.palm().position().y();

                                volume = convert_range(y, 200.0, 300.0, -96.0, 0.0);
                            }
                        }
                    }

                    {
                        let mut dsp = dsp.lock().expect("DSP thread poisened");
                        dsp.set_by_path("volume", volume)
                            .expect("Failed to set volume.");
                        dsp.set_by_path("freq", freq).expect("Failed to set freq.");
                        dsp.send();
                    }
                }
            }
        }
    })
}
