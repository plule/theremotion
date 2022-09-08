#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod auto_chord;
mod controls;
mod dsp_thread;
mod leap;
mod scales;
mod settings;
mod ui;
mod ui_keyboard;

#[allow(clippy::all)]
#[rustfmt::skip]
mod dsp;

use cpal::traits::StreamTrait;
use faust_state::DspHandle;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    // Init communication channels
    let (settings_tx, settings_rx) = crossbeam_channel::unbounded(); // UI to Leap
    let (dsp_controls_tx, dsp_controls_rx) = crossbeam_channel::unbounded(); // Leap to UI
    let (monitoring_tx, monitoring_rx) = crossbeam_channel::unbounded(); // DSP out to UI

    // Init DSP
    let (dsp, state) = DspHandle::<dsp::Instrument>::new();

    // Init sound output
    let stream = dsp_thread::run(dsp, monitoring_tx);
    stream.play().expect("Failed to play stream");

    // Init leap thread
    let _leap_worker = leap::start_leap_worker(state, settings_rx, dsp_controls_tx);

    // Start UI
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        format!("Leapotron v{}", VERSION).as_str(),
        native_options,
        Box::new(move |cc| {
            Box::new(ui::Leapotron::new(
                cc,
                dsp_controls_rx,
                settings_tx,
                monitoring_rx,
            ))
        }),
    );
}
