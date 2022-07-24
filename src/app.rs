use std::sync::{Arc, Mutex};

use faust_state::StateHandle;

pub struct Leapotron {
    dsp: Arc<Mutex<StateHandle>>,
    volume: f32,
    freq: f32,
}

impl Leapotron {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>, dsp: Arc<Mutex<StateHandle>>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self {
            dsp,
            volume: -25.,
            freq: 440.0,
        }
    }
}

impl eframe::App for Leapotron {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self { dsp, volume, freq } = self;

        {
            let dsp = dsp.lock().expect("DSP thread poisened");
            *volume = *dsp.get_by_path("volume").expect("Failed to read volume");
            *freq = *dsp.get_by_path("freq").expect("Failed to read volume");
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
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.add(egui::Slider::new(volume, -96.0..=0.).text("Volume"));
            ui.add(
                egui::Slider::new(freq, 27.50..=12543.85)
                    .logarithmic(true)
                    .text("Frequency"),
            );
            egui::warn_if_debug_build(ui);
        });

        {
            let mut dsp = dsp.lock().expect("DSP thread poisened");
            dsp.set_by_path("volume", *volume)
                .expect("Failed to set volume.");
            dsp.set_by_path("freq", *freq).expect("Failed to set freq.");
            dsp.send();
        }

        ctx.request_repaint();
    }
}
