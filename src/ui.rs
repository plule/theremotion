use std::{fmt::Display, ops::RangeInclusive};

use crossbeam_channel::{Receiver, Sender};

use egui::{
    plot::{
        uniform_grid_spacer, GridMark, HLine, Legend, Line, MarkerShape, PlotPoint, PlotPoints,
        Points, VLine,
    },
    text::LayoutJob,
    FontFamily, FontId, Key, RichText, Slider, TextFormat, TextStyle,
};
use staff::{midi::MidiNote, scale::ScaleIntervals, Pitch};

use crate::{
    controls::{self},
    scales::MoreScales,
    settings::Settings,
    ui_keyboard::KeyboardEditMode,
};

pub struct Theremotion {
    dsp_controls_rx: Receiver<controls::Controls>,
    controls: controls::Controls,
    settings: Settings,
    saved_settings: Settings,
    settings_tx: Sender<Settings>,
    main_tab: MainTab,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MainTab {
    Play,
    RootEdit,
    ScaleEdit,
    Mix,
    Instructions,
}

impl Display for MainTab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MainTab::Play => f.write_str("Play"),
            MainTab::RootEdit => f.write_str("Root Edit"),
            MainTab::ScaleEdit => f.write_str("Scale Edit"),
            MainTab::Mix => f.write_str("Mix"),
            MainTab::Instructions => f.write_str("Instructions"),
        }
    }
}

impl Theremotion {
    /// Called once before the first frame.
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        dsp_controls_rx: Receiver<controls::Controls>,
        settings_tx: Sender<Settings>,
    ) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        let mut style = (*cc.egui_ctx.style()).clone();
        style.text_styles = [
            (
                TextStyle::Small,
                FontId::new(15.0, FontFamily::Proportional),
            ),
            (TextStyle::Body, FontId::new(21.0, FontFamily::Proportional)),
            (
                TextStyle::Button,
                FontId::new(40.0, FontFamily::Proportional),
            ),
            (
                TextStyle::Heading,
                FontId::new(64.0, FontFamily::Proportional),
            ),
            (
                TextStyle::Monospace,
                FontId::new(21.0, FontFamily::Monospace),
            ),
        ]
        .into();
        cc.egui_ctx.set_style(style);

        let controls = dsp_controls_rx.recv().unwrap();
        let settings = Settings::read();
        settings_tx.send(settings.clone()).unwrap();
        Self {
            dsp_controls_rx,
            settings_tx,
            controls,
            saved_settings: settings.clone(),
            main_tab: MainTab::Play,
            settings,
        }
    }
}

impl eframe::App for Theremotion {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self {
            dsp_controls_rx,
            controls,
            settings,
            settings_tx,
            saved_settings,
            main_tab,
        } = self;

        if ctx.input().key_down(Key::Escape) {
            frame.close();
        }

        // Update the current control state from the DSP
        if let Some(new_controls) = dsp_controls_rx.try_iter().last() {
            *controls = new_controls;
        }

        egui::SidePanel::right("right_panel")
            .default_width(32.0)
            .show(ctx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.selectable_value(main_tab, MainTab::Play, RichText::new("👐").heading());
                    ui.selectable_value(main_tab, MainTab::RootEdit, RichText::new("🎵").heading());
                    ui.selectable_value(
                        main_tab,
                        MainTab::ScaleEdit,
                        RichText::new("🎼").heading(),
                    );
                    ui.selectable_value(main_tab, MainTab::Mix, RichText::new("🔊").heading());
                    ui.selectable_value(
                        main_tab,
                        MainTab::Instructions,
                        RichText::new("ℹ").heading(),
                    );
                });
            });
        egui::TopBottomPanel::bottom("bottom_panel")
            .default_height(32.0)
            .show(ctx, |ui| {
                if let Some(warning) = &controls.warning {
                    let warning = format!("⚠ Leap: {}", warning);
                    ui.label(RichText::new(warning).color(egui::Color32::YELLOW));
                }

                if let Some(error) = &controls.error {
                    let error = format!("⚠ Leap: {}", error);
                    ui.label(RichText::new(error).color(egui::Color32::RED));
                }

                egui::warn_if_debug_build(ui);
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                let (lh, rh) = controls.has_hands;
                let lh = if lh { "👈" } else { "⬜" };
                let rh = if rh { "👉" } else { "⬜" };
                ui.heading(format!("{lh} {main_tab} {rh}"));
            });
            ui.separator();
            match main_tab {
                MainTab::Play => {
                    play_tab(ui, controls, settings);
                }
                MainTab::RootEdit => {
                    root_edit_tab(ui, controls, settings);
                }
                MainTab::ScaleEdit => {
                    scale_edit_tab(ui, controls, settings);
                }
                MainTab::Mix => {
                    mix_tab(ui, settings);
                }
                MainTab::Instructions => {
                    instructions_tab(ui, controls, settings);
                }
            }
        });

        if saved_settings != settings {
            settings.save();
            *saved_settings = settings.clone();
            settings_tx.send(settings.clone()).unwrap();
        }
        ctx.request_repaint();
    }
}

fn mix_tab(ui: &mut egui::Ui, settings: &mut Settings) {
    ui.horizontal(|ui| {
        ui.style_mut().spacing.slider_width = 300.0;
        ui.style_mut().spacing.button_padding = egui::vec2(32.0, 32.0);
        ui.style_mut().spacing.interact_size = egui::vec2(64.0, 64.0);
        let space = 40.0;
        ui.group(|ui| {
            ui.add_space(space);
            mix_slider(ui, "Lead", &mut settings.lead_volume);
            ui.add_space(space);
            mix_slider(ui, "Guitar", &mut settings.guitar_volume);
            ui.add_space(space);
            mix_slider(ui, "Drone", &mut settings.drone_volume);
            ui.add_space(space);
        });
        ui.group(|ui| {
            ui.add_space(space);
            mix_slider(ui, "Master", &mut settings.master_volume);
            ui.add_space(space);
        });
    });
}

fn mix_slider(ui: &mut egui::Ui, name: &str, value: &mut f32) {
    ui.vertical(|ui| {
        let icon = match &value {
            value if **value <= 0.0 => "🔇",
            value if **value <= 0.33 => "🔈",
            value if **value <= 0.66 => "🔉",
            _ => "🔊",
        };
        ui.add(
            Slider::new(value, 0.0..=1.0)
                .vertical()
                .min_decimals(2)
                .max_decimals(2)
                .step_by(0.01)
                .show_value(false)
                .text(format!("{} {}", icon, name)),
        );
    });
}

fn play_tab(ui: &mut egui::Ui, controls: &mut controls::Controls, settings: &mut Settings) {
    ui.add(crate::ui_keyboard::Keyboard::new(
        controls.lead.iter().collect(),
        settings,
        KeyboardEditMode::Drone,
    ));
    ui.separator();
    ui.horizontal(|ui| {
        autotune_plot(
            ui,
            250.0,
            settings,
            controls.autotune,
            controls.raw_note,
            controls.lead[0].note.value,
        );

        ui.add_space(10.0);

        tuner(
            ui,
            100.0,
            250.0,
            settings,
            "tuner",
            controls.raw_note,
            controls.lead[0].note.value,
        );

        ui.add_space(10.0);

        xy_plot(
            ui,
            250.0,
            "rh_hand",
            &controls.cutoff_note,
            &controls.resonance,
            "Cutoff",
            "Resonance",
        );
    });
}

fn instructions_tab(ui: &mut egui::Ui, controls: &mut controls::Controls, settings: &mut Settings) {
    ui.add(crate::ui_keyboard::Keyboard::new(
        controls.lead.iter().collect(),
        settings,
        KeyboardEditMode::None,
    ));
    ui.separator();
    ui.label("👐 Theremotion is a synthesizer controlled by your hands.");
    ui.label("👉 Move up and down your right hand to control the volume.");
    ui.label("👈 Move up and down your left hand to control the pitch.");
    ui.label("👋 Move your hands on the horizontal plane to adapt the timbre.");
    ui.label("👌 Pinch with your left hand to stick on a scale.");
    ui.label("🎸 Pinch with your right hand, and rotate it to play guitar.");
    ui.label("✌ ☝ Retract your pinky and ring fingers of your left hand, then play with the other fingers to play scales");
}

fn scale_edit_tab(ui: &mut egui::Ui, controls: &mut controls::Controls, settings: &mut Settings) {
    ui.add(crate::ui_keyboard::Keyboard::new(
        controls.lead.iter().collect(),
        settings,
        KeyboardEditMode::Scale,
    ));
    ui.separator();
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().button_padding.x = 10.0;
        ui.spacing_mut().button_padding.y = 10.0;
        ui.selectable_value(&mut settings.scale, ScaleIntervals::all(), "🎼 Chromatic");
        ui.selectable_value(&mut settings.scale, ScaleIntervals::major(), "🎼 Major");
        ui.selectable_value(
            &mut settings.scale,
            ScaleIntervals::melodic_minor(),
            "🎼 Melodic Minor",
        );
        ui.selectable_value(
            &mut settings.scale,
            ScaleIntervals::harmonic_minor(),
            "🎼 Harmonic Minor",
        );
        ui.selectable_value(
            &mut settings.scale,
            ScaleIntervals::natural_minor(),
            "🎼 Natural Minor",
        );
        ui.selectable_value(&mut settings.scale, ScaleIntervals::dorian(), "🎼 Dorian");
        ui.selectable_value(&mut settings.scale, ScaleIntervals::blues(), "🎼 Blues");
        ui.selectable_value(
            &mut settings.scale,
            ScaleIntervals::freygish(),
            "🎼 Freygish",
        );
        ui.selectable_value(
            &mut settings.scale,
            ScaleIntervals::altered_dorian(),
            "🎼 Altered Dorian",
        );
    });
}

fn root_edit_tab(ui: &mut egui::Ui, controls: &mut controls::Controls, settings: &mut Settings) {
    ui.add(crate::ui_keyboard::Keyboard::new(
        controls.lead.iter().collect(),
        settings,
        KeyboardEditMode::RootNote,
    ));
    ui.separator();
    octave_selector(ui, "Lead Octave", &mut settings.octave);
    octave_selector(ui, "Guitar Octave", &mut settings.guitar_octave);
    ui.separator();
    ui.vertical_centered_justified(|ui| {
        ui.label(RichText::new("Note").size(30.0));
    });
    ui.horizontal_wrapped(|ui| {
        for pitch in 0..=11 {
            let pitch = Pitch::from_byte(pitch);
            ui.selectable_value(
                &mut settings.pitch,
                pitch,
                RichText::new(format!("  {}  ", pitch)).size(40.0),
            );
        }
    });
}

fn octave_selector(ui: &mut egui::Ui, name: &str, octave_value: &mut i8) {
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new(name).size(40.0));
        for octave in 0..=4 {
            ui.selectable_value(
                octave_value,
                octave,
                RichText::new(format!("  {}  ", octave)).size(40.0),
            );
        }
    });
}

fn autotune_plot(
    ui: &mut egui::Ui,
    size: f32,
    settings: &mut Settings,
    autotune: usize,
    raw_value: f32,
    value: f32,
) {
    let note_range = settings.note_range();
    let smooths = (*note_range.start() as usize * 10..*note_range.end() as usize * 10).map(|i| {
        let x = i as f32 * 0.1;
        PlotPoint::new(
            x,
            crate::music_theory::autotune(x, autotune, settings.scale_notes()),
        )
    });
    let line = Line::new(PlotPoints::Owned(smooths.collect()));
    // hack: force the include_x/include_y to recenter on root note change
    let plot_id = format!("{}{}", settings.pitch, settings.octave);
    egui::plot::Plot::new(plot_id)
        .allow_boxed_zoom(false)
        .allow_drag(false)
        .allow_scroll(false)
        .allow_zoom(false)
        .include_x(*note_range.start())
        .include_x(*note_range.end())
        .include_y(*note_range.start())
        .include_y(*note_range.end())
        .x_grid_spacer(uniform_grid_spacer(|_| [12.0, 1.0, 1.0]))
        .y_grid_spacer(uniform_grid_spacer(|_| [12.0, 1.0, 1.0]))
        .x_axis_formatter(note_formatter)
        .y_axis_formatter(note_formatter)
        .legend(Legend::default())
        .width(size)
        .height(size)
        .show(ui, |plot_ui| {
            plot_ui.line(line);
            plot_ui.points(
                Points::new(PlotPoints::Owned(vec![PlotPoint::new(raw_value, value)]))
                    .shape(MarkerShape::Plus)
                    .radius(6.0),
            );
        });
}

fn note_formatter(note: f64, _range: &RangeInclusive<f64>) -> String {
    MidiNote::from_byte(note as u8).to_string()
}

fn xy_plot(
    ui: &mut egui::Ui,
    size: f32,
    plot_name: &str,
    control_x: &controls::Control,
    control_y: &controls::Control,
    control_x_name: &str,
    control_y_name: &str,
) {
    egui::plot::Plot::new(plot_name)
        .allow_boxed_zoom(false)
        .allow_drag(false)
        .allow_scroll(false)
        .allow_zoom(false)
        .include_x(*control_x.input.range.start())
        .include_x(*control_x.input.range.end())
        .include_y(*control_y.input.range.start())
        .include_y(*control_y.input.range.end())
        .legend(Legend::default())
        .show_axes([false, false])
        .width(size)
        .height(size)
        .show(ui, |plot_ui| {
            plot_ui.vline(VLine::new(control_x.value).name(control_x_name));
            plot_ui.hline(HLine::new(control_y.value).name(control_y_name));
        });
}

fn tuner(
    ui: &mut egui::Ui,
    width: f32,
    height: f32,
    settings: &Settings,
    plot_name: &str,
    note_raw: f32,
    note_tuned: f32,
) {
    let scale = settings.scale_notes();
    let closest = crate::music_theory::closest_in_scale(note_raw, &scale);
    let closest = scale
        .get(closest)
        .map(|closest| closest.into_byte() as f32)
        .unwrap_or_else(|| note_raw.round());

    // hack: force the include_x/include_y to recenter on root note change
    let plot_id = format!("{plot_name}{closest}");
    egui::plot::Plot::new(plot_id)
        .allow_boxed_zoom(false)
        .allow_drag(false)
        .allow_scroll(false)
        .allow_zoom(false)
        .include_y(closest - 2.0)
        .include_y(closest + 2.0)
        .include_x(-1.0)
        .include_x(1.0)
        .y_grid_spacer(move |input| {
            ((input.bounds.0.floor() as u8)..=(input.bounds.1.ceil() as u8))
                .into_iter()
                .map(|n| {
                    let note = MidiNote::from_byte(n);
                    let step_size = if scale.contains(&note) { 5.0 } else { 1.0 };
                    GridMark {
                        value: n as f64,
                        step_size,
                    }
                })
                .collect()
        })
        .show_axes([false, true])
        .y_axis_formatter(note_formatter)
        .width(width)
        .height(height)
        .show(ui, |plot_ui| {
            plot_ui.hline(HLine::new(note_raw).name("Note"));
            plot_ui.hline(HLine::new(note_tuned).name("Note (Tuned)"));
        });
}
