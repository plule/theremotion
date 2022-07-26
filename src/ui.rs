use std::sync::{mpsc::Sender, Arc, Mutex};

use egui::plot::{HLine, Legend, Line, VLine, Value, Values};
use faust_state::StateHandle;

use crate::{dsp, settings::Settings};

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
        Self {
            dsp,
            settings_tx,
            controls: dsp::Controls::default(),
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
            ui.add(crate::ui_keyboard::Keyboard::new(controls.note));

            ui.horizontal_top(|ui| {
                ui.add(
                    egui::Slider::new(&mut controls.note, dsp::Controls::note_range())
                        .show_value(false)
                        .text("Pitch")
                        .vertical(),
                );
                egui::plot::Plot::new("lh_hond")
                    .allow_boxed_zoom(false)
                    .allow_drag(false)
                    .allow_scroll(false)
                    .allow_zoom(false)
                    .include_x(*dsp::Controls::detune_range().start())
                    .include_x(*dsp::Controls::detune_range().end())
                    .include_y(*dsp::Controls::supersaw_range().start())
                    .include_y(*dsp::Controls::supersaw_range().end())
                    .legend(Legend::default())
                    .show_axes([false, false])
                    .width(200.0)
                    .height(200.0)
                    .show(ui, |plot_ui| {
                        plot_ui.vline(VLine::new(controls.detune).name("Detune"));
                        plot_ui.hline(HLine::new(controls.supersaw).name("Supersaw"));
                    });
                egui::plot::Plot::new("rh_plot")
                    .allow_boxed_zoom(false)
                    .allow_drag(false)
                    .allow_scroll(false)
                    .allow_zoom(false)
                    .include_x(*dsp::Controls::cutoff_range().start())
                    .include_x(*dsp::Controls::cutoff_range().end())
                    .include_y(*dsp::Controls::resonance_range().start())
                    .include_y(*dsp::Controls::resonance_range().end())
                    .legend(Legend::default())
                    .show_axes([false, false])
                    .width(200.0)
                    .height(200.0)
                    .show(ui, |plot_ui| {
                        plot_ui.vline(VLine::new(controls.cutoff_note).name("Cutoff"));
                        plot_ui.hline(HLine::new(controls.resonance).name("Resonance"));
                    });
                ui.add(
                    egui::Slider::new(&mut controls.volume, dsp::Controls::volume_range())
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

            let smooths = (*dsp::Controls::note_range().start() as usize * 10
                ..*dsp::Controls::note_range().end() as usize * 10)
                .map(|i| {
                    let x = i as f32 * 0.1;
                    Value::new(x, crate::leap::smoothstairs(x, settings.autotune_strength))
                });
            let line = Line::new(Values::from_values_iter(smooths));

            egui::plot::Plot::new("autotune_plot")
                .allow_boxed_zoom(false)
                .allow_drag(false)
                .allow_scroll(false)
                .allow_zoom(false)
                .include_x(*dsp::Controls::note_range().start())
                .include_x(*dsp::Controls::note_range().end())
                .include_y(*dsp::Controls::note_range().start())
                .include_y(*dsp::Controls::note_range().end())
                .legend(Legend::default())
                .show_axes([false, false])
                .width(400.0)
                .height(400.0)
                .show(ui, |plot_ui| {
                    plot_ui.line(line);
                });

            egui::warn_if_debug_build(ui);
        });
        settings_tx.send(settings.clone()).unwrap();
        ctx.request_repaint();
    }
}
