use std::sync::{mpsc::Sender, Arc, Mutex};

use egui::plot::{HLine, Legend, Line, VLine, Value, Values};
use faust_state::StateHandle;

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
                settings.scale_notes(),
            ));

            ui.horizontal_top(|ui| {
                ui.add_enabled(
                    false,
                    egui::Slider::new(&mut controls.note.value, settings.note_range_f())
                        .show_value(false)
                        .text("Pitch")
                        .vertical(),
                );
                egui::plot::Plot::new("lh_hond")
                    .allow_boxed_zoom(false)
                    .allow_drag(false)
                    .allow_scroll(false)
                    .allow_zoom(false)
                    .include_x(*controls.detune.range.start())
                    .include_x(*controls.detune.range.end())
                    .include_y(*controls.supersaw.range.start())
                    .include_y(*controls.supersaw.range.end())
                    .legend(Legend::default())
                    .show_axes([false, false])
                    .width(200.0)
                    .height(200.0)
                    .show(ui, |plot_ui| {
                        plot_ui.vline(VLine::new(controls.detune.value).name("Detune"));
                        plot_ui.hline(HLine::new(controls.supersaw.value).name("Supersaw"));
                    });
                egui::plot::Plot::new("rh_plot")
                    .allow_boxed_zoom(false)
                    .allow_drag(false)
                    .allow_scroll(false)
                    .allow_zoom(false)
                    .include_x(*controls.cutoff_note.range.start())
                    .include_x(*controls.cutoff_note.range.end())
                    .include_y(*controls.resonance.range.start())
                    .include_y(*controls.resonance.range.end())
                    .legend(Legend::default())
                    .show_axes([false, false])
                    .width(200.0)
                    .height(200.0)
                    .show(ui, |plot_ui| {
                        plot_ui.vline(VLine::new(controls.cutoff_note.value).name("Cutoff"));
                        plot_ui.hline(HLine::new(controls.resonance.value).name("Resonance"));
                    });
                ui.add_enabled(
                    false,
                    egui::Slider::new(&mut controls.volume.value, controls.volume.range.to_owned())
                        .show_value(false)
                        .text("Volume")
                        .vertical(),
                );
            });

            ui.add(
                egui::Slider::new(&mut settings.autotune_strength, 0..=5)
                    .integer()
                    .text("Autotune Strength"),
            );

            ui.selectable_value(&mut settings.scale, ScaleType::Chromatic, "Chromatic");
            ui.selectable_value(&mut settings.scale, ScaleType::Major, "Major");
            ui.selectable_value(&mut settings.scale, ScaleType::Minor, "Minor");
            ui.selectable_value(&mut settings.scale, ScaleType::Blues, "Blues");

            let note_range = settings.note_range();
            let smooths =
                (*note_range.start() as usize * 10..*note_range.end() as usize * 10).map(|i| {
                    let x = i as f32 * 0.1;
                    Value::new(
                        x,
                        crate::dsp::smoothstairs(
                            x,
                            settings.autotune_strength,
                            settings.scale_notes(),
                        ),
                    )
                });
            let line = Line::new(Values::from_values_iter(smooths));

            egui::plot::Plot::new("autotune_plot")
                .allow_boxed_zoom(false)
                .allow_drag(false)
                .allow_scroll(false)
                .allow_zoom(false)
                .include_x(*note_range.start())
                .include_x(*note_range.end())
                .include_y(*note_range.start())
                .include_y(*note_range.end())
                .legend(Legend::default())
                .show_axes([false, false])
                .width(400.0)
                .height(400.0)
                .show(ui, |plot_ui| {
                    plot_ui.line(line);
                    plot_ui.vline(VLine::new(controls.note.raw_value));
                    plot_ui.hline(HLine::new(controls.note.value));
                });

            egui::warn_if_debug_build(ui);
        });
        settings_tx.send(settings.clone()).unwrap();
        ctx.request_repaint();
    }
}
