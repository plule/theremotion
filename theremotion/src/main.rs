//! Music instrument based on the Leap Motion and Faust

#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![cfg_attr(not(feature = "leap"), allow(dead_code))] // When building without leap support for tests, allow dead code
#![cfg_attr(not(feature = "leap"), allow(unused_variables))] // When building without leap support for tests, allow dead code

/// DSP controllable parameters
mod controls;

/// Thread transforming and dispatching the messages from the others
mod conductor_thread;

/// Thread computing the DSP and sending parameter updates
mod dsp_thread;

/// Thread reading the hand positions
#[cfg(feature = "leap")]
mod leap_thread;

/// Mod creating the main window and event loop
mod ui_thread;

/// Application settings
mod settings;

/// Music related types and algorithms
mod solfege;

/// Poor man's Step implementation
mod step_iter;

/// Newtypes for strongly typed exchanges
mod types;

pub use types::*;

use cpal::traits::StreamTrait;
use default_boxed::DefaultBoxed;
use faust_state::DspHandle;
use settings::Settings;
pub use step_iter::StepIter;
use theremotion_ui::*;

/// Theremotion version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    // Read application settings
    let settings = Settings::read();

    if settings.system.high_priority_process {
        set_high_priority();
    }

    // Init communication channels
    let (leap_tx, leap_rx) = crossbeam_channel::unbounded(); // Settings update to leap thread
    let (ui_tx, ui_rx) = crossbeam_channel::unbounded(); // UI update messages
    let (dsp_tx, dsp_rx) = crossbeam_channel::unbounded(); // DSP parameter update messages
    let (co_tx, co_rx) = crossbeam_channel::unbounded(); // Conductor messages

    // Init DSP
    let dsp = theremotion_dsp::Instrument::default_boxed();
    let (dsp, state) = DspHandle::<theremotion_dsp::Instrument>::from_dsp(dsp);

    // Init the controls struct
    let controls = controls::Controls::from(&state);

    // Queue the initialization messages
    leap_tx
        .send(leap_thread::Message::SettingsUpdate(settings.clone()))
        .unwrap();
    settings
        .current_preset
        .send_to_dsp(&controls, &dsp_tx)
        .unwrap();

    // Start the conductor thread
    let conductor = conductor_thread::run(
        settings.clone(),
        controls.clone(),
        co_rx,
        dsp_tx.clone(),
        ui_tx.clone(),
        leap_tx.clone(),
    );

    // Init sound output
    let stream = dsp_thread::run(dsp, state, dsp_rx);
    stream.play().expect("Failed to play stream");

    // Init leap thread
    #[cfg(feature = "leap")]
    let leap_worker = leap_thread::run(leap_rx, co_tx.clone());

    // Start UI
    let (window, _window_timer) = ui_thread::run(co_tx.clone(), ui_rx, settings);
    window.run().expect("Failed to start the UI");
    co_tx
        .send(conductor_thread::ConductorMessage::Exit)
        .expect("Error when trying to exit");

    #[cfg(feature = "leap")]
    leap_worker
        .join()
        .expect("Error when stopping the leap worker");

    conductor
        .join()
        .expect("Error when stopping the conductor thread");
}

#[cfg(target_os = "windows")]
fn set_high_priority() {
    unsafe {
        let process = windows::Win32::System::Threading::GetCurrentProcess();
        windows::Win32::System::Threading::SetPriorityClass(
            process,
            windows::Win32::System::Threading::REALTIME_PRIORITY_CLASS,
        )
        .expect("Failed to set high priority");
    }
}

#[cfg(not(target_os = "windows"))]
fn set_high_priority() {
    log::warn!("High priority process is not supported on this platform yet");
}
