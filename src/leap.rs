use std::{
    ops::RangeInclusive,
    sync::{Arc, Mutex},
    thread,
};

use faust_state::StateHandle;
use leaprs::*;

use crate::dsp;

fn convert_range(
    value: f32,
    input_range: RangeInclusive<f32>,
    output_range: RangeInclusive<f32>,
) -> f32 {
    {
        let in_min = *input_range.start();
        let in_max = *input_range.end();
        let out_min = *output_range.start();
        let out_max = *output_range.end();
        ((((value - in_min) * (out_max - out_min)) / (in_max - in_min)) + out_min)
            .clamp(out_min, out_max)
    }
}

/// Start the leap motion thread
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

                                controls.detune = convert_range(
                                    position.x(),
                                    -100.0..=100.0,
                                    dsp::Controls::detune_range(),
                                );

                                controls.note = convert_range(
                                    position.y(),
                                    100.0..=600.0,
                                    dsp::Controls::note_range(),
                                );

                                controls.supersaw = convert_range(
                                    position.z(),
                                    100.0..=-100.0,
                                    dsp::Controls::supersaw_range(),
                                );
                            }
                            HandType::Right => {
                                let position = hand.palm().position();

                                controls.cutoff_note = convert_range(
                                    position.x(),
                                    -100.0..=100.0,
                                    dsp::Controls::cutoff_range(),
                                );

                                controls.volume = convert_range(
                                    position.y(),
                                    200.0..=300.0,
                                    dsp::Controls::volume_range(),
                                );
                                controls.resonance = convert_range(
                                    position.z(),
                                    100.0..=-100.0,
                                    dsp::Controls::resonance_range(),
                                );
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
