use std::{
    ops::RangeInclusive,
    sync::{mpsc::Receiver, Arc, Mutex},
    thread,
};

use faust_state::StateHandle;
use leaprs::*;
use music_note::midi::MidiNote;

use crate::{dsp, settings::Settings};

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

/// Smooth step function loosely "sticking" the value to 0 or 1
/// Assumes that value is between 0 and 1
/// https://en.wikipedia.org/wiki/Smoothstep
fn smoothstep(interval: &RangeInclusive<f32>, x: f32) -> f32 {
    let x = (x - interval.start()) / (interval.end() - interval.start());
    x * x * (3.0 - 2.0 * x)
}

pub fn smoothstairs(value: f32, amount: usize, scale: Vec<MidiNote>) -> f32 {
    let scale: Vec<_> = scale
        .windows(2)
        .map(|w| (w[0].into_byte() as f32)..=(w[1].into_byte() as f32))
        .collect();

    if let Some(interval) = scale.iter().find(|interval| interval.contains(&value)) {
        let mut value = value;

        for _ in 0..amount {
            let smooth = smoothstep(interval, value);
            value = interval.start() + smooth * (interval.end() - interval.start());
        }
        return value;
    }
    value
}

/// Start the leap motion thread
pub fn start_leap_worker(
    dsp: Arc<Mutex<StateHandle>>,
    settings_rx: Receiver<Settings>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut connection =
            Connection::create(ConnectionConfig::default()).expect("Failed to connect");
        connection.open().expect("Failed to open the connection");
        let mut controls = crate::dsp::Controls::default();
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

                                controls.detune = convert_range(
                                    position.x(),
                                    -200.0..=0.0,
                                    dsp::Controls::detune_range(),
                                );

                                let raw_note = convert_range(
                                    position.y(),
                                    100.0..=600.0,
                                    dsp::Controls::note_range(),
                                );
                                controls.note = smoothstairs(
                                    raw_note,
                                    settings.autotune_strength,
                                    settings.scale_notes(),
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
                                    0.0..=200.0,
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
                        controls.send(&mut dsp);
                    }
                }
            }
        }
    })
}
