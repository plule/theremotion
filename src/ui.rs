use crossbeam_channel::{Receiver, Sender};

use egui::{
    plot::{
        uniform_grid_spacer, HLine, Legend, Line, MarkerShape, PlotPoint, PlotPoints, Points, VLine,
    },
    FontFamily, FontId, RichText, TextStyle,
};
use staff::{midi::MidiNote, scale::ScaleIntervals};

use crate::{
    controls::{self, Controls},
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
    monitoring_rx: Receiver<Vec<f32>>,
    monitoring: Vec<f32>,
    main_tab: MainTab,
    keyboard_edit_mode: KeyboardEditMode,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MainTab {
    ScaleEdit,
    Play,
    Instructions,
}

impl Theremotion {
    /// Called once before the first frame.
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        dsp_controls_rx: Receiver<controls::Controls>,
        settings_tx: Sender<Settings>,
        monitoring_rx: Receiver<Vec<f32>>,
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
            monitoring_rx,
            monitoring: Vec::default(),
            saved_settings: settings.clone(),
            main_tab: MainTab::Play,
            keyboard_edit_mode: KeyboardEditMode::RootNote,
            settings,
        }
    }
}

impl eframe::App for Theremotion {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            dsp_controls_rx,
            controls,
            settings,
            settings_tx,
            monitoring,
            monitoring_rx,
            saved_settings,
            main_tab,
            keyboard_edit_mode,
        } = self;

        // Update the current control state from the DSP
        if let Some(new_controls) = dsp_controls_rx.try_iter().last() {
            *controls = new_controls;
        }

        if let Some(new_monitoring) = monitoring_rx.try_iter().last() {
            *monitoring = new_monitoring;
        }

        egui::SidePanel::right("right_panel")
            .default_width(32.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.selectable_value(main_tab, MainTab::Play, RichText::new("👐").heading());
                    ui.selectable_value(
                        main_tab,
                        MainTab::ScaleEdit,
                        RichText::new("🎼").heading(),
                    );
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
        egui::CentralPanel::default().show(ctx, |ui| match main_tab {
            MainTab::ScaleEdit => {
                edit_scale_tab(ui, settings, controls, keyboard_edit_mode);
            }
            MainTab::Play => {
                ui.add(crate::ui_keyboard::Keyboard::new(
                    controls.lead.iter().collect(),
                    settings,
                    KeyboardEditMode::Drone,
                ));

                ui.separator();

                ui.horizontal(|ui| {
                    autotune_plot(
                        ui,
                        320.0,
                        settings,
                        controls.autotune,
                        controls.raw_note,
                        controls.lead[0].note.value,
                    );

                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            xy_plot(
                                ui,
                                150.0,
                                "lh_hand",
                                &controls.detune,
                                &controls.supersaw,
                                "Detune",
                                "Supersaw",
                            );
                            ui.spacing();
                            xy_plot(
                                ui,
                                150.0,
                                "rh_hand",
                                &controls.cutoff_note,
                                &controls.resonance,
                                "Cutoff",
                                "Resonance",
                            );
                        });
                        monitoring_plot(ui, "monitoring", monitoring);
                    });
                });
            }
            MainTab::Instructions => {
                instructions_tab(ui);
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

fn edit_scale_tab(
    ui: &mut egui::Ui,
    settings: &mut Settings,
    controls: &Controls,
    keyboard_edit_mode: &mut KeyboardEditMode,
) {
    ui.add(crate::ui_keyboard::Keyboard::new(
        controls.lead.iter().collect(),
        settings,
        *keyboard_edit_mode,
    ));

    ui.separator();

    ui.horizontal(|ui| {
        ui.selectable_value(
            keyboard_edit_mode,
            KeyboardEditMode::RootNote,
            "✏ Root Note",
        );
        ui.selectable_value(keyboard_edit_mode, KeyboardEditMode::Scale, "✏ Scale");
    });

    ui.separator();

    match keyboard_edit_mode {
        KeyboardEditMode::RootNote => {
            ui.horizontal(|ui| {
                if ui.button(RichText::new("-").heading()).clicked() {
                    settings.root_note = MidiNote::from_byte(settings.root_note.into_byte() - 12);
                }

                ui.label(format!("Octave {}", settings.root_note.octave()));

                if ui.button(RichText::new("+").heading()).clicked() {
                    settings.root_note = MidiNote::from_byte(settings.root_note.into_byte() + 12);
                }
            });

            ui.horizontal(|ui| {
                if ui.button(RichText::new("-").heading()).clicked() {
                    settings.root_note = MidiNote::from_byte(settings.root_note.into_byte() - 1);
                }

                ui.label(format!("Root Note {}", settings.root_note.pitch()));

                if ui.button(RichText::new("+").heading()).clicked() {
                    settings.root_note = MidiNote::from_byte(settings.root_note.into_byte() + 1);
                }
            });
        }
        KeyboardEditMode::Scale => {
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
        _ => {}
    }
}

fn instructions_tab(ui: &mut egui::Ui) {
    ui.label("👐 Theremotion is a synthesizer controlled by your hands.");
    ui.label("👉 Move up and down your right hand to control the volume.");
    ui.label("👈 Move up and down your left hand to control the pitch.");
    ui.label("👋 Move your hands on the horizontal plane to adapt the timbre.");
    ui.label("👌 Pinch with your left hand to stick on a scale.");
    ui.label("🎸 Pinch with your right hand, and rotate it to play guitar.");
    ui.label("🎼 Left click on the keyboard to select a root note.");
    ui.label("🎹 Choose a predefined scale or right click on the keyboard to make a custom scale.");
    ui.label("♒ Middle click on the keyboard to enable a Drone.");
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
    let plot_id = format!(
        "{}{}",
        settings.root_note.pitch(),
        settings.root_note.octave()
    );
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
        .x_axis_formatter(|v, _| MidiNote::from_byte(v as u8).to_string())
        .y_axis_formatter(|v, _| MidiNote::from_byte(v as u8).to_string())
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

fn monitoring_plot(ui: &mut egui::Ui, plot_name: &str, monitoring: &[f32]) {
    let line = Line::new(PlotPoints::Owned(
        monitoring
            .iter()
            // Wait for zero-cross
            .skip_while(|s| **s <= 0.0)
            .skip_while(|s| **s >= 1.0)
            .step_by(10)
            .enumerate()
            .map(|(index, value)| PlotPoint::new(index as f64, *value))
            .collect(),
    ))
    .fill(0.0)
    .highlight(true);
    egui::plot::Plot::new(plot_name)
        .allow_boxed_zoom(false)
        .allow_drag(false)
        .allow_scroll(false)
        .allow_zoom(false)
        .include_x(0.0)
        .include_x(30.0)
        .include_y(-2.0)
        .include_y(2.0)
        .width(100.0)
        .height(50.0)
        .show(ui, |plot_ui| {
            plot_ui.line(line);
        });
}
