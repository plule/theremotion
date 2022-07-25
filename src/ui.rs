use std::sync::{Arc, Mutex};

use egui::plot::{HLine, Legend, VLine};
use faust_state::StateHandle;

use crate::dsp;

pub struct Leapotron {
    dsp: Arc<Mutex<StateHandle>>,
    controls: dsp::Controls,
}

impl Leapotron {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>, dsp: Arc<Mutex<StateHandle>>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self {
            dsp,
            controls: dsp::Controls::default(),
        }
    }
}

impl eframe::App for Leapotron {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self { dsp, controls } = self;

        // Update the current control state from the DSP
        {
            let mut dsp = dsp.lock().expect("DSP thread poisened");
            controls.read(&mut dsp);
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
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

            egui::warn_if_debug_build(ui);
        });

        ctx.request_repaint();
    }
}
