#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod controls;
mod dsp_thread;
mod leap;
mod music_theory;
mod scales;
mod settings;
mod ui;

#[allow(clippy::all)]
#[rustfmt::skip]
mod dsp;

use clap::Parser;
use cpal::traits::StreamTrait;
use default_boxed::DefaultBoxed;
use faust_state::DspHandle;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Command line arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Start Theremotion in full screen
    #[clap(
        long,
        value_parser,
        default_value_t = false,
        env = "THEREMOTION_FULLSCREEN"
    )]
    fullscreen: bool,

    /// Initial X position of the window
    #[clap(
        long,
        value_parser,
        default_value_t = 0,
        env = "THEREMOTION_WINDOWS_POSITION_X"
    )]
    window_position_x: i32,

    /// Initial Y position of the window
    #[clap(
        long,
        value_parser,
        default_value_t = 0,
        env = "THEREMOTION_WINDOWS_POSITION_Y"
    )]
    window_position_y: i32,

    /// Execute tabtip.exe on Windows to prompt the touchscreen keyboard
    #[clap(
        long,
        value_parser,
        default_value_t = false,
        env = "THEREMOTION_TABTIP"
    )]
    tabtip: bool,

    /// Set the process as high priority
    #[clap(
        long,
        value_parser,
        default_value_t = false,
        env = "THEREMOTION_HIGH_PRIORITY"
    )]
    high_priority: bool,
}

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    if args.high_priority {
        unsafe {
            let process = windows::Win32::System::Threading::GetCurrentProcess();
            windows::Win32::System::Threading::SetPriorityClass(
                process,
                windows::Win32::System::Threading::REALTIME_PRIORITY_CLASS,
            );
        }
    }

    // Init communication channels
    let (settings_tx, settings_rx) = crossbeam_channel::unbounded(); // UI to Leap
    let (dsp_controls_tx, dsp_controls_rx) = crossbeam_channel::unbounded(); // Leap to UI

    // Init DSP
    let dsp = dsp::Instrument::default_boxed();
    let (dsp, state) = DspHandle::<dsp::Instrument>::from_dsp(dsp);

    // Init sound output
    let stream = dsp_thread::run(dsp);
    stream.play().expect("Failed to play stream");

    // Init leap thread
    let _leap_worker = leap::start_leap_worker(state, settings_rx, dsp_controls_tx);

    // Start UI
    let native_options = eframe::NativeOptions {
        initial_window_pos: Some(egui::pos2(
            args.window_position_x as f32,
            args.window_position_y as f32,
        )),
        initial_window_size: Some(egui::vec2(800.0, 480.0)),
        maximized: args.fullscreen,
        decorated: !args.fullscreen,
        ..Default::default()
    };

    eframe::run_native(
        format!("Theremotion v{}", VERSION).as_str(),
        native_options,
        Box::new(move |cc| Box::new(ui::App::new(cc, dsp_controls_rx, settings_tx, args.tabtip))),
    );
}
