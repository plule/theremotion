use std::sync::{Arc, Mutex};

use faust_state::StateHandle;

pub struct Leapotron {
    dsp: Arc<Mutex<StateHandle>>,
}

impl Leapotron {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>, dsp: Arc<Mutex<StateHandle>>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self { dsp }
    }
}

impl eframe::App for Leapotron {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self { dsp } = self;

        let mut volume;
        let mut cutoff_note;
        let mut note;
        let mut res;

        {
            let dsp = dsp.lock().expect("DSP thread poisened");
            volume = *dsp.get_by_path("volume").unwrap();
            cutoff_note = *dsp.get_by_path("cutoff_note").unwrap();
            note = *dsp.get_by_path("note").unwrap();
            res = *dsp.get_by_path("res").unwrap();
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

            ui.add(egui::Slider::new(&mut volume, -96.0..=0.).text("Volume"));
            ui.add(egui::Slider::new(&mut note, 0.0..=127.0).text("Note"));
            ui.add(egui::Slider::new(&mut cutoff_note, -20.0..=20.0).text("Cutoff"));
            ui.add(egui::Slider::new(&mut res, 1.0..=20.0).text("Resonance"));
            egui::warn_if_debug_build(ui);
        });

        {
            let mut dsp = dsp.lock().expect("DSP thread poisened");
            dsp.set_by_path("volume", volume).unwrap();
            dsp.set_by_path("cutoff_note", cutoff_note).unwrap();
            dsp.set_by_path("note", note).unwrap();
            dsp.set_by_path("res", res).unwrap();
            dsp.send();
        }

        ctx.request_repaint();
    }
}
