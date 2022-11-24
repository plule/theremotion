#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![cfg_attr(not(feature = "leap"), allow(dead_code))] // When building without leap support for tests, allow dead code
#![cfg_attr(not(feature = "leap"), allow(unused_variables))] // When building without leap support for tests, allow dead code

mod controls;
mod dsp_thread;
#[cfg(feature = "leap")]
mod leap_thread;
mod scale_windows;
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
use settings::Settings;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
const ICON: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/icon"));

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

    // Read application settings
    let settings = Settings::read();

    // Init communication channels
    let (settings_tx, settings_rx) = crossbeam_channel::unbounded(); // Settings update to leap thread
    let (ui_tx, ui_rx) = crossbeam_channel::unbounded(); // UI update messages
    let (dsp_tx, dsp_rx) = crossbeam_channel::unbounded(); // DSP parameter update messages

    // Init DSP
    let dsp = dsp::Instrument::default_boxed();
    let (dsp, state) = DspHandle::<dsp::Instrument>::from_dsp(dsp);

    // Init the controls struct
    let controls = controls::Controls::from(&state);

    // Queue the initialization messages
    settings_tx.send(settings.clone()).unwrap();
    settings
        .current_preset
        .send_to_dsp(&controls, &dsp_tx)
        .unwrap();

    // Init sound output
    let stream = dsp_thread::run(dsp, state, dsp_rx);
    stream.play().expect("Failed to play stream");

    // Init leap thread
    #[cfg(feature = "leap")]
    let leap_worker = leap_thread::run(controls.clone(), settings_rx, ui_tx, dsp_tx.clone());

    // Start UI
    let native_options = eframe::NativeOptions {
        initial_window_pos: Some(egui::pos2(
            args.window_position_x as f32,
            args.window_position_y as f32,
        )),
        initial_window_size: Some(egui::vec2(800.0, 480.0)),
        maximized: args.fullscreen,
        decorated: !args.fullscreen,
        icon_data: Some(eframe::IconData {
            rgba: ICON.to_vec(),
            width: 128,
            height: 128,
        }),
        ..Default::default()
    };

    eframe::run_native(
        format!("Theremotion v{}", VERSION).as_str(),
        native_options,
        Box::new(move |cc| {
            Box::new(ui::App::new(
                cc,
                ui_rx,
                dsp_tx.clone(),
                settings_tx,
                controls.clone(),
                settings,
                args.tabtip,
            ))
        }),
    );

    #[cfg(feature = "leap")]
    leap_worker
        .join()
        .expect("Error when stopping the leap worker");
}
