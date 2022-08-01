use std::sync::{mpsc::Sender, Arc, Mutex};

use egui::plot::{
    uniform_grid_spacer, HLine, Legend, Line, MarkerShape, Points, VLine, Value, Values,
};
use faust_state::StateHandle;
use music_note::midi::MidiNote;

use crate::{
    dsp::{self, Controls},
    settings::{ScaleType, Settings},
};

pub struct Leapotron {
    dsp: Arc<Mutex<StateHandle>>,
    controls: dsp::Controls,
    settings: Settings,
    settings_tx: Sender<Settings>,
}

impl Leapotron {
    /// Called once before the first frame.
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        dsp: Arc<Mutex<StateHandle>>,
        settings_tx: Sender<Settings>,
    ) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        let controls: Controls = { dsp.lock().as_deref().unwrap().into() };
        Self {
            dsp,
            settings_tx,
            controls,
            settings: Settings::default(),
        }
    }
}

impl eframe::App for Leapotron {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self {
            dsp,
            controls,
            settings,
            settings_tx,
        } = self;

        // Update the current control state from the DSP
        {
            let mut dsp = dsp.lock().expect("DSP thread poisened");
            controls.receive(&mut dsp);
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().spacing.slider_width = 200.0;
            ui.add(crate::ui_keyboard::Keyboard::new(
                controls.note.value,
                settings,
            ));

            ui.horizontal(|ui| {
                ui.selectable_value(&mut settings.scale, ScaleType::Chromatic, "Chromatic");
                ui.selectable_value(&mut settings.scale, ScaleType::Major, "Major");
                ui.selectable_value(
                    &mut settings.scale,
                    ScaleType::MelodicMinor,
                    "Melodic Minor",
                );
                ui.selectable_value(
                    &mut settings.scale,
                    ScaleType::NaturalMinor,
                    "Natural Minor",
                );
                ui.selectable_value(
                    &mut settings.scale,
                    ScaleType::HarmonicMinor,
                    "Harmonic Minor",
                );
                ui.selectable_value(&mut settings.scale, ScaleType::Blues, "Blues");
                ui.separator();
                ui.add(
                    egui::Slider::new(&mut settings.autotune_strength, 0..=5)
                        .integer()
                        .text("Autotune Strength"),
                );
            });

            ui.separator();

            ui.horizontal_top(|ui| {
                autotune_plot(ui, settings, &controls.note);
                ui.add_space(10.0);
                ui.add_enabled(
                    false,
                    egui::Slider::new(&mut controls.note.value, settings.note_range_f())
                        .show_value(false)
                        .text("Pitch")
                        .vertical(),
                );
                xy_plot(
                    ui,
                    "lh_hand",
                    &controls.detune,
                    &controls.supersaw,
                    "Detune",
                    "Supersaw",
                );
                ui.spacing();
                xy_plot(
                    ui,
                    "rh_hand",
                    &controls.cutoff_note,
                    &controls.resonance,
                    "Cutoff",
                    "Resonance",
                );
                ui.add_enabled(
                    false,
                    egui::Slider::new(&mut controls.volume.value, controls.volume.range.to_owned())
                        .show_value(false)
                        .text("Volume")
                        .vertical(),
                );
            });
            ui.separator();

            egui::warn_if_debug_build(ui);
        });
        settings_tx.send(settings.clone()).unwrap();
        ctx.request_repaint();
    }
}

fn autotune_plot(ui: &mut egui::Ui, settings: &mut Settings, control: &dsp::NoteControl) {
    let note_range = settings.note_range();
    let smooths = (*note_range.start() as usize * 10..*note_range.end() as usize * 10).map(|i| {
        let x = i as f32 * 0.1;
        Value::new(
            x,
            crate::dsp::smoothstairs(x, settings.autotune_strength, settings.scale_notes()),
        )
    });
    let line = Line::new(Values::from_values_iter(smooths));
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
        .x_axis_formatter(|v, _| {
            let note = MidiNote::from_byte(v as u8);
            format!("{}{}", note.pitch(), note.octave())
        })
        .y_axis_formatter(|v, _| {
            let note = MidiNote::from_byte(v as u8);
            format!("{}{}", note.pitch(), note.octave())
        })
        .legend(Legend::default())
        .width(200.0)
        .height(200.0)
        .show(ui, |plot_ui| {
            plot_ui.line(line);
            plot_ui.points(
                Points::new(Values::from_values(vec![Value::new(
                    control.raw_value,
                    control.value,
                )]))
                .shape(MarkerShape::Plus)
                .radius(6.0),
            );
        });
}

fn xy_plot(
    ui: &mut egui::Ui,
    plot_name: &str,
    control_x: &dsp::Control,
    control_y: &dsp::Control,
    control_x_name: &str,
    control_y_name: &str,
) {
    egui::plot::Plot::new(plot_name)
        .allow_boxed_zoom(false)
        .allow_drag(false)
        .allow_scroll(false)
        .allow_zoom(false)
        .include_x(*control_x.range.start())
        .include_x(*control_x.range.end())
        .include_y(*control_y.range.start())
        .include_y(*control_y.range.end())
        .legend(Legend::default())
        .show_axes([false, false])
        .width(200.0)
        .height(200.0)
        .show(ui, |plot_ui| {
            plot_ui.vline(VLine::new(control_x.value).name(control_x_name));
            plot_ui.hline(HLine::new(control_y.value).name(control_y_name));
        });
}
