#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod dsp;
mod leap;
mod settings;
mod ui;
mod ui_keyboard;

#[allow(clippy::all)]
mod faust {
    include!(concat!(env!("OUT_DIR"), "/dsp.rs"));
}

use cpal::traits::StreamTrait;
use faust_state::DspHandle;

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    // Init communication channels
    let (settings_tx, settings_rx) = crossbeam_channel::unbounded();
    let (dsp_controls_tx, dsp_controls_rx) = crossbeam_channel::unbounded();

    // Init DSP
    let (dsp, state) = DspHandle::<faust::Instrument>::new();

    // Init sound output
    let stream = dsp::run_dsp(dsp);
    stream.play().expect("Failed to play stream");

    // Init leap thread
    let _leap_worker = leap::start_leap_worker(state, settings_rx, dsp_controls_tx);

    // Start UI
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Leapotron",
        native_options,
        Box::new(move |cc| Box::new(ui::Leapotron::new(cc, dsp_controls_rx, settings_tx))),
    );
}
