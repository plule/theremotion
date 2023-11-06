use std::{collections::HashSet, rc::Rc};

use itertools::Itertools;
use nalgebra::Vector2;
use slint::*;
use staff::midi::MidiNote;
use std::sync::mpsc::{Receiver, Sender};
use theremotion_ui::MainWindow;

use crate::{
    controls::Controls,
    settings::{Handedness, Settings},
    thread_conductor::{Msg as CM, TrackingStatus},
    {MidiNoteF, Volume},
};

/// Message to update externally the UI
#[derive(Debug)]
pub enum Msg {
    /// Close the UI
    Exit,
    /// Current application error status
    Status(TrackingStatus),
    /// Lead instrument volume (0-1)
    LeadVolume(f32),
    /// Lead notes, volume and raw horizontal coordinates
    Lead([(MidiNoteF, Volume); 4], Vector2<f32>),
    /// Floating number of chord notes (2.5 is 2 chord notes and the next half volume)
    ChordsNumber(f32),
    /// Floating number of drone notes
    DroneNumber(f32),
    /// Lead instrument note, without autotune
    RawNote(MidiNoteF),
    /// Filter cutoff and resonance.
    /// Filter: -1 to 1, resonance: 0 to 1
    Filter(f32, f32),
    /// Amount of autotune
    AutotuneAmount(usize),
    /// Visible hands
    HasHands(bool, bool),
    /// Volume hand is pinching and the guitar sound can be activated
    StrumReady(bool),
    /// Trumpet string strength (0-1)
    TrumpetStrength(f32),
    /// Settings update from leap
    Settings(Settings),
}

pub fn run(
    tx: Sender<CM>,
    mut ui_rx: Receiver<Msg>,
    controls: Controls,
    mut settings: Settings,
) -> (MainWindow, slint::Timer) {
    if settings.system.fullscreen {
        std::env::set_var("SLINT_FULLSCREEN", "1");
    }
    let window = theremotion_ui::MainWindow::new().expect("Failed to create the UI");
    let window_weak = window.as_weak();

    // Send the min/max set in the DSP
    set_ui_controls(&window.global::<theremotion_ui::UIState<'_>>(), controls);

    update_ui_from_settings(&window.global::<theremotion_ui::UIState<'_>>(), &settings);

    /// Helper to connect slint callbacks to events sent on a channel
    struct Connector(Sender<CM>);
    impl Connector {
        // Build a callback that sends the callback parameter as a message
        // Needs the callback parameter to be convertible to the message content
        fn send<Tin, Tout: From<Tin>>(
            &self,
            msg: impl Fn(Tout) -> CM + 'static,
        ) -> impl FnMut(Tin) + 'static {
            let tx = self.0.clone();
            move |v| tx.send(msg(Tout::from(v))).unwrap()
        }

        // Build a callback that sends a message without parameter
        fn send2(&self, msg: impl Fn() -> CM + 'static) -> impl FnMut() + 'static {
            let tx = self.0.clone();
            move || tx.send(msg()).unwrap()
        }
    }
    let c = Connector(tx.clone());

    window
        .global::<theremotion_ui::VirtualKeyboardHandler<'_>>()
        .on_key_pressed({
            let window_weak = window_weak.clone();
            move |key| {
                window_weak
                    .unwrap()
                    .window()
                    .dispatch_event(slint::platform::WindowEvent::KeyPressed { text: key.clone() });
                window_weak
                    .unwrap()
                    .window()
                    .dispatch_event(slint::platform::WindowEvent::KeyReleased { text: key });
            }
        });

    window.window().on_close_requested({
        let tx = tx.clone();
        move || {
            tx.send(CM::Exit).unwrap();
            slint::CloseRequestResponse::HideWindow
        }
    });

    let ui = window.global::<theremotion_ui::UIState<'_>>();

    // Common
    ui.on_close(c.send2(|| CM::Exit));

    // Play tab
    ui.on_drone_clicked(c.send(CM::DroneClicked));

    // Root tab
    ui.on_root_pitch_clicked(c.send(CM::RootClicked));
    ui.on_lead_octave_clicked(c.send(CM::LeadOctave));
    ui.on_guitar_octave_clicked(c.send(CM::GuitarOctave));
    ui.on_drone_octave_clicked(c.send(CM::DroneOctave));

    // Scale tab
    ui.on_scale_clicked(c.send(CM::ScaleClicked));
    ui.on_select_scale(c.send(CM::SelectScale));
    ui.on_delete_scale(c.send(CM::DeleteScale));
    ui.on_save_scale(c.send(CM::SaveScale));

    // Mix tab
    ui.on_mix_lead_changed(c.send(CM::LeadVolume));
    ui.on_mix_pluck_changed(c.send(CM::GuitarVolume));
    ui.on_mix_drone_changed(c.send(CM::DroneVolume));
    ui.on_mix_master_changed(c.send(CM::MasterVolume));
    ui.on_guitar_drone_clicked(c.send2(|| CM::GuitarDroneClicked));

    // Effects tab
    ui.on_echo_mix_changed(c.send(CM::EchoAmount));
    ui.on_echo_duration_changed(c.send(CM::EchoDuration));
    ui.on_echo_feedback_changed(c.send(CM::EchoFeedback));
    ui.on_reverb_mix_changed(c.send(CM::ReverbAmount));
    ui.on_reverb_time_changed(c.send(CM::ReverbTime));
    ui.on_reverb_damp_changed(c.send(CM::ReverbDamp));
    ui.on_reverb_size_changed(c.send(CM::ReverbSize));
    ui.on_drone_detune_changed(c.send(CM::DroneDetune));

    // Presets tab
    ui.on_select_preset(c.send(CM::SelectPreset));
    ui.on_delete_preset(c.send(CM::DeletePreset));
    ui.on_save_preset(c.send(CM::SavePreset));

    // Settings tab
    ui.on_fullscreen_clicked(c.send2(|| CM::FullscreenClicked));
    ui.on_fullscreen_clicked(c.send2(|| CM::FullscreenClicked));
    ui.on_on_screen_kbd_clicked(c.send2(|| CM::OnScreenKeyboardClicked));
    ui.on_lh_clicked(c.send2(|| CM::LHClicked));
    ui.on_rh_clicked(c.send2(|| CM::RHClicked));
    ui.on_high_priority_clicked(c.send2(|| CM::HighPriorityClicked));

    let window_timer = slint::Timer::default();

    window_timer.start(
        slint::TimerMode::Repeated,
        std::time::Duration::from_millis(10),
        {
            move || {
                let window = window_weak.unwrap();
                read_updates(&mut ui_rx, &mut settings, &window);
            }
        },
    );
    (window, window_timer)
}

fn read_updates(
    ui_rx: &mut Receiver<Msg>,
    settings: &mut Settings,
    window: &theremotion_ui::MainWindow,
) {
    for event in ui_rx.try_iter() {
        let restricted_scale_window = settings.current_preset.restricted_scale_floating_window();
        let ui = window.global::<theremotion_ui::UIState<'_>>();

        match event {
            Msg::Exit => {
                // When the last (only) window is hidden, it exits
                window.hide().expect("Failed to hide the window");
            }
            Msg::Status(TrackingStatus::Ok) => {
                ui.set_status(theremotion_ui::Status::Ok);
                ui.set_status_message("Ok".into());
            }
            Msg::Status(TrackingStatus::Warning(text)) => {
                ui.set_status(theremotion_ui::Status::Warning);
                ui.set_status_message(text.into());
            }
            Msg::Status(TrackingStatus::Error(text)) => {
                ui.set_status(theremotion_ui::Status::Error);
                ui.set_status_message(text.into());
            }
            Msg::LeadVolume(v) => ui.set_volume(v),
            Msg::Lead(notes, coords) => {
                let coords_direction = coords.normalize();
                let range_end = *settings.current_preset.note_range_f().end();
                ui.set_tuner_note_tuned(notes[0].0 .0);
                // Lead for dots
                ui.set_notes(
                    notes
                        .map(|(note, volume)| {
                            let coords = coords_direction * (range_end - note).semitones();
                            theremotion_ui::NotePoint {
                                volume: volume.0,
                                x: coords.x,
                                y: coords.y,
                            }
                        })
                        .into(),
                );
                ui.set_raw_note(theremotion_ui::NotePoint {
                    volume: 1.0,
                    x: coords.x,
                    y: coords.y,
                });
                // Lead for keyboard
                let mut leads = [0.0; 12 * 4];
                for (note, volume) in notes {
                    let index_1 = note.0.floor();
                    let index_1_factor = 1.0 - (note.0 - index_1);
                    let index_2 = index_1 + 1.0;
                    let index_2_factor = 1.0 - index_1_factor;

                    let index_1 = index_1 as usize;
                    let index_2 = index_2 as usize;

                    if index_1 < leads.len() {
                        leads[index_1] += volume.0 * index_1_factor;
                    }

                    if index_2 < leads.len() {
                        leads[index_2] += volume.0 * index_2_factor;
                    }
                }
                let kb_leads = ui.get_leads();
                for (index, lead) in leads.into_iter().enumerate() {
                    kb_leads.set_row_data(index, lead);
                }
            }
            Msg::ChordsNumber(c) => ui.set_chords_number(c),
            Msg::DroneNumber(d) => ui.set_drone_number(d),
            Msg::RawNote(n) => {
                ui.set_tuner_note_focus(restricted_scale_window.closest_in_scale(n).0);
                ui.set_tuner_note(n.0);
            }
            Msg::Filter(c, r) => {
                ui.set_filter_cutoff(c);
                ui.set_filter_resonance(r);
            }
            Msg::AutotuneAmount(a) => ui.set_autotune_amount(a.try_into().unwrap_or_default()),
            Msg::HasHands(l, r) => {
                ui.set_has_left_hand(l);
                ui.set_has_right_hand(r);
            }
            Msg::StrumReady(s) => ui.set_strum_ready(s),
            Msg::TrumpetStrength(_) => {} // todo?
            Msg::Settings(s) => {
                *settings = s;
                update_ui_from_settings(&ui, settings);
            }
        }
    }
}

fn set_ui_controls(ui: &theremotion_ui::UIState<'_>, controls: Controls) {
    ui.set_drone_detune_control(ui_control(&controls.drone_detune));
    ui.set_echo_mix_control(ui_control(&controls.echo_mix));
    ui.set_echo_duration_control(ui_control(&controls.echo_duration));
    ui.set_echo_feedback_control(ui_control(&controls.echo_feedback));
    ui.set_reverb_mix_control(ui_control(&controls.reverb_mix));
    ui.set_reverb_time_control(ui_control(&controls.reverb_time));
    ui.set_reverb_damp_control(ui_control(&controls.reverb_damp));
    ui.set_reverb_size_control(ui_control(&controls.reverb_size));
    ui.set_mix_master_control(ui_control(&controls.mix_master_volume));
    ui.set_mix_drone_control(ui_control(&controls.mix_drone_volume));
    ui.set_mix_lead_control(ui_control(&controls.mix_lead_volume));
    ui.set_mix_pluck_control(ui_control(&controls.mix_pluck_volume));
}

fn update_ui_from_settings(ui: &theremotion_ui::UIState<'_>, settings: &Settings) {
    ui.set_handedness(settings.system.handedness.into());
    ui.set_fullscreen(settings.system.fullscreen);
    ui.set_high_priority(settings.system.high_priority_process);
    ui.set_use_on_screen_keyboard(settings.system.force_touchscreen);

    let preset = &settings.current_preset;
    ui.set_lead_octave(preset.lead_octave.into_i8() as i32);
    ui.set_guitar_octave(preset.guitar_octave.into_i8() as i32);
    ui.set_drone_octave(preset.drone_octave.into_i8() as i32);
    ui.set_highest_note(preset.note_range().end().into_byte() as i32);

    ui.set_mix_lead(preset.mix.lead.0);
    ui.set_mix_pluck(preset.mix.guitar.0);
    ui.set_mix_drone(preset.mix.drone.0);
    ui.set_mix_master(preset.mix.master.0);

    ui.set_enable_guitar_drone(preset.drone.pluck_drone);

    ui.set_echo_mix(preset.fx.echo.mix.0);
    ui.set_echo_duration(preset.fx.echo.duration);
    ui.set_echo_feedback(preset.fx.echo.feedback);
    ui.set_reverb_mix(preset.fx.reverb.mix.0);
    ui.set_reverb_time(preset.fx.reverb.time);
    ui.set_reverb_damp(preset.fx.reverb.damp);
    ui.set_reverb_size(preset.fx.reverb.size);
    ui.set_drone_detune(preset.drone.detune);

    let root_pitch = settings.current_preset.pitch;
    ui.set_root_pitch(root_pitch.into_byte().into());
    let scales = VecModel::from(
        settings
            .system_and_user_scales()
            .map(|(scale, user)| theremotion_ui::Selectable {
                id: scale.id(),
                selected: scale.scale == preset.scale,
                name: scale.name.into(),
                removable: user,
            })
            .collect_vec(),
    );
    ui.set_scale_presets(ModelRc::from(Rc::new(scales)));

    let presets = VecModel::from(
        settings
            .system_and_user_presets()
            .map(|(preset, user)| theremotion_ui::Selectable {
                id: preset.id(),
                name: preset.name.clone().into(),
                removable: user,
                selected: preset == &settings.current_preset,
            })
            .collect_vec(),
    );
    ui.set_presets(ModelRc::from(Rc::new(presets)));

    let scale: HashSet<MidiNote> = preset.restricted_scale().into_iter().collect();
    let ui_scale = ui.get_scale_notes();
    for index in 0..(12 * 4) {
        ui_scale.set_row_data(index, scale.contains(&MidiNote::from_byte(index as u8)));
    }

    let drone_notes: HashSet<MidiNote> = settings
        .current_preset
        .drone_notes()
        .into_iter()
        .flatten()
        .collect();

    let ui_drones = ui.get_drones();

    for index in 0..=12 * 4 {
        let note = MidiNote::from(index);
        let index = index as usize;
        ui_drones.set_row_data(
            index,
            if drone_notes.contains(&note) {
                1.0
            } else {
                0.0
            },
        );
    }
}

impl From<theremotion_ui::Handedness> for Handedness {
    fn from(value: theremotion_ui::Handedness) -> Self {
        match value {
            theremotion_ui::Handedness::LeftHanded => Handedness::LeftHanded,
            theremotion_ui::Handedness::RightHanded => Handedness::RightHanded,
        }
    }
}

impl From<Handedness> for theremotion_ui::Handedness {
    fn from(value: Handedness) -> Self {
        match value {
            Handedness::RightHanded => theremotion_ui::Handedness::RightHanded,
            Handedness::LeftHanded => theremotion_ui::Handedness::LeftHanded,
        }
    }
}

fn ui_control(control: &crate::controls::Control) -> theremotion_ui::DspControl {
    theremotion_ui::DspControl {
        min: *control.input.range.start(),
        max: *control.input.range.end(),
    }
}
