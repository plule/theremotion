use std::{collections::HashSet, rc::Rc};

use crossbeam_channel::Sender;
use itertools::Itertools;
use nalgebra::Vector2;
use slint::*;
use staff::midi::MidiNote;
use theremotion_ui::MainWindow;

use crate::{
    conductor_thread::{ConductorMessage as CM, LeapStatus},
    settings::{Handedness, Settings},
    {MidiNoteF, Volume},
};

/// Message to update externally the UI
#[derive(Debug)]
pub enum UiUpdate {
    ///Current application error status
    Status(LeapStatus),
    /// Lead instrument volume (0-1)
    LeadVolume(f32),
    /// Lead notes, volume and raw horizontal coordinates
    Lead([(MidiNoteF, Volume); 4], Vector2<f32>),
    /// Floating number of chord notes (2.5 is 2 chord notes and the next half volume)
    ChordsNumber(f32),
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
    mut ui_rx: crossbeam_channel::Receiver<UiUpdate>,
    mut settings: Settings,
) -> (MainWindow, slint::Timer) {
    if settings.system.fullscreen {
        std::env::set_var("SLINT_FULLSCREEN", "1");
    }
    let mut window = theremotion_ui::MainWindow::new().expect("Failed to create the UI");
    let window_weak = window.as_weak();

    update_ui_from_settings(&mut window, &settings);

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

    // Common
    window.on_close({
        let window_weak = window_weak.clone();
        move || {
            window_weak.unwrap().hide().unwrap();
        }
    });
    // Play tab
    window.on_drone_clicked(c.send(CM::DroneClicked));

    // Root tab
    window.on_root_pitch_clicked(c.send(CM::RootClicked));
    window.on_lead_octave_clicked(c.send(CM::LeadOctave));
    window.on_guitar_octave_clicked(c.send(CM::GuitarOctave));
    window.on_drone_octave_clicked(c.send(CM::DroneOctave));

    // Scale tab
    window.on_scale_clicked(c.send(CM::ScaleClicked));
    window.on_select_scale(c.send(CM::SelectScale));
    window.on_delete_scale(c.send(CM::DeleteScale));
    window.on_save_scale(c.send(CM::SaveScale));

    // Mix tab
    window.on_mix_lead_changed(c.send(CM::LeadVolume));
    window.on_mix_guitar_changed(c.send(CM::GuitarVolume));
    window.on_mix_drone_changed(c.send(CM::DroneVolume));
    window.on_mix_master_changed(c.send(CM::MasterVolume));
    window.on_guitar_drone_clicked(c.send2(|| CM::GuitarDroneClicked));

    // Effects tab
    window.on_echo_amount_changed(c.send(CM::EchoAmount));
    window.on_echo_duration_changed(c.send(CM::EchoDuration));
    window.on_echo_feedback_changed(c.send(CM::EchoFeedback));
    window.on_reverb_amount_changed(c.send(CM::ReverbAmount));
    window.on_reverb_time_changed(c.send(CM::ReverbTime));
    window.on_reverb_damp_changed(c.send(CM::ReverbDamp));
    window.on_reverb_size_changed(c.send(CM::ReverbSize));
    window.on_drone_detune_changed(c.send(CM::DroneDetune));

    // Presets tab
    window.on_select_preset(c.send(CM::SelectPreset));
    window.on_delete_preset(c.send(CM::DeletePreset));
    window.on_save_preset(c.send(CM::SavePreset));

    // Settings tab
    window.on_fullscreen_clicked(c.send2(|| CM::FullscreenClicked));
    window.on_fullscreen_clicked(c.send2(|| CM::FullscreenClicked));
    window.on_on_screen_kbd_clicked(c.send2(|| CM::OnScreenKeyboardClicked));
    window.on_lh_clicked(c.send2(|| CM::LHClicked));
    window.on_rh_clicked(c.send2(|| CM::RHClicked));
    window.on_high_priority_clicked(c.send2(|| CM::HighPriorityClicked));

    let window_timer = slint::Timer::default();

    window_timer.start(
        slint::TimerMode::Repeated,
        std::time::Duration::from_millis(10),
        {
            move || {
                read_updates(&mut ui_rx, &mut settings, &mut window_weak.unwrap());
            }
        },
    );
    (window, window_timer)
}

fn read_updates(
    ui_rx: &mut crossbeam_channel::Receiver<UiUpdate>,
    settings: &mut Settings,
    window: &mut MainWindow,
) {
    for event in ui_rx.try_iter() {
        let restricted_scale_window = settings.current_preset.restricted_scale_floating_window();
        match event {
            UiUpdate::Status(LeapStatus::Ok) => {
                window.set_status(theremotion_ui::Status::Ok);
                window.set_status_message("Ok".into());
            }
            UiUpdate::Status(LeapStatus::Warning(text)) => {
                window.set_status(theremotion_ui::Status::Warning);
                window.set_status_message(text.into());
            }
            UiUpdate::Status(LeapStatus::Error(text)) => {
                window.set_status(theremotion_ui::Status::Error);
                window.set_status_message(text.into());
            }
            UiUpdate::LeadVolume(v) => window.set_lead_volume(v),
            UiUpdate::Lead(notes, coords) => {
                let coords_direction = coords.normalize();
                let range_end = *settings.current_preset.note_range_f().end();
                window.set_tuned_note(notes[0].0 .0);
                // Lead for dots
                window.set_lead_notes(
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
                window.set_lead_raw_note(theremotion_ui::NotePoint {
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
                let kb_leads = window.get_leads();
                for (index, lead) in leads.into_iter().enumerate() {
                    kb_leads.set_row_data(index, lead);
                }
            }
            UiUpdate::ChordsNumber(c) => window.set_chords_number(c),
            UiUpdate::RawNote(n) => {
                window.set_tuner_note_focus(restricted_scale_window.closest_in_scale(n).0);
                window.set_raw_note(n.0);
            }
            UiUpdate::Filter(c, r) => {
                window.set_filter_cutoff(c);
                window.set_filter_resonance(r);
            }
            UiUpdate::AutotuneAmount(a) => {
                window.set_autotune_amount(a.try_into().unwrap_or_default())
            }
            UiUpdate::HasHands(l, r) => {
                window.set_has_left_hand(l);
                window.set_has_right_hand(r);
            }
            UiUpdate::StrumReady(s) => window.set_strum_ready(s),
            UiUpdate::TrumpetStrength(s) => window.set_trumpet_strength(s),
            UiUpdate::Settings(s) => {
                *settings = s;
                update_ui_from_settings(window, settings);
            }
        }
    }
}

fn update_ui_from_settings(window: &mut theremotion_ui::MainWindow, settings: &Settings) {
    window.set_handedness(settings.system.handedness.into());
    window.set_fullscreen(settings.system.fullscreen);
    window.set_high_priority(settings.system.high_priority_process);
    window.set_use_on_screen_keyboard(settings.system.force_touchscreen);

    let preset = &settings.current_preset;
    window.set_lead_octave(preset.lead_octave.into_i8() as i32);
    window.set_guitar_octave(preset.guitar_octave.into_i8() as i32);
    window.set_drone_octave(preset.drone_octave.into_i8() as i32);
    window.set_highest_note(preset.note_range().end().into_byte() as i32);

    window.set_mix_lead(preset.mix.lead.0);
    window.set_mix_guitar(preset.mix.guitar.0);
    window.set_mix_drone(preset.mix.drone.0);
    window.set_mix_master(preset.mix.master.0);

    window.set_enable_guitar_drone(preset.drone.pluck_drone);

    window.set_echo_amount(preset.fx.echo.mix.0);
    window.set_echo_duration(preset.fx.echo.duration);
    window.set_echo_feedback(preset.fx.echo.feedback);
    window.set_reverb_amount(preset.fx.reverb.mix.0);
    window.set_reverb_time(preset.fx.reverb.time);
    window.set_reverb_damp(preset.fx.reverb.damp);
    window.set_reverb_size(preset.fx.reverb.size);
    window.set_drone_detune(preset.drone.detune);

    let root_pitch = settings.current_preset.pitch;
    window.set_root_pitch(root_pitch.into_byte().into());
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
    window.set_scale_presets(ModelRc::from(Rc::new(scales)));

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
    window.set_presets(ModelRc::from(Rc::new(presets)));

    let scale: HashSet<MidiNote> = preset.restricted_scale().into_iter().collect();
    let ui_scale = window.get_scale_notes();
    for index in 0..(12 * 4) {
        ui_scale.set_row_data(index, scale.contains(&MidiNote::from_byte(index as u8)));
    }

    let drone_notes: HashSet<MidiNote> = settings
        .current_preset
        .drone_notes()
        .into_iter()
        .flatten()
        .collect();

    let ui_drones = window.get_drones();

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
